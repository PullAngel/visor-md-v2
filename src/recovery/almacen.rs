//! El almacén de copias en disco: carpetas por sesión, cerrojo de cada sesión,
//! listado de las que quedaron huérfanas, adopción y barrido.
//!
//! Una sesión es un proceso. Cada una mantiene abierto, sin compartirlo, su
//! archivo `.lock`; cuando el proceso muere, de la forma que sea, el sistema lo
//! libera y la sesión pasa a ser huérfana. Eso reemplaza cualquier heurística
//! con identificadores de proceso, que se reutilizan.

use super::formato::{self, DecodeError, EntryMeta, MAX_HEADER_BYTES};
use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub(crate) const ENTRY_EXTENSION: &str = "vmdr";
const SESSION_PREFIX: &str = "s-";
const LOCK_EXTENSION: &str = "lock";
const CLAIM_EXTENSION: &str = "claim";
const TEMP_DIR_PREFIX: &str = ".atomicwrite";

/// Cuánto vive una copia huérfana sin que nadie la resuelva.
pub(crate) const ORPHAN_RETENTION: Duration = Duration::from_secs(30 * 24 * 60 * 60);
/// Una copia ilegible se descarta pasado este tiempo.
const CORRUPT_RETENTION: Duration = Duration::from_secs(24 * 60 * 60);
/// Un temporal de `atomicwrites` más viejo que esto es de un proceso que murió
/// a mitad de una escritura.
const STALE_TEMP: Duration = Duration::from_secs(60 * 60);
/// Las copias del formato anterior (`session-*.md`) no se tocan si son tan
/// recientes que podrían ser de una ventana abierta con la versión vieja.
const LEGACY_QUIET: Duration = Duration::from_secs(60);
/// Tope de copias huérfanas y de su tamaño total.
pub(crate) const MAX_ORPHANS: usize = 50;
pub(crate) const MAX_ORPHAN_BYTES: u64 = 64 * 1024 * 1024;

pub(crate) struct Store {
    root: PathBuf,
}

/// La sesión de este proceso. Mientras exista, su cerrojo está tomado.
pub(crate) struct Session {
    pub(crate) dir: PathBuf,
    lock_path: PathBuf,
    lock: Option<File>,
}

impl Drop for Session {
    fn drop(&mut self) {
        // En Windows no se puede borrar un archivo abierto sin compartir.
        drop(self.lock.take());
        let _ = fs::remove_dir(&self.dir);
        let _ = fs::remove_file(&self.lock_path);
    }
}

/// Una copia que quedó de una sesión que ya no existe.
#[derive(Clone, Debug)]
pub(crate) struct Orphan {
    pub(crate) path: PathBuf,
    /// Identidad estable de la copia (el nombre del archivo, sin extensión).
    pub(crate) key: String,
    pub(crate) meta: EntryMeta,
    pub(crate) body_len: u64,
    pub(crate) modified: SystemTime,
    /// Viene del formato anterior: sin origen ni suma de verificación.
    pub(crate) legacy: bool,
}

impl Orphan {
    #[cfg(test)]
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    /// Una copia descrita a mano, para pruebas de la interfaz.
    #[cfg(test)]
    pub(crate) fn for_tests(path: PathBuf, meta: EntryMeta, body_len: u64) -> Self {
        Self {
            key: path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("copia")
                .to_string(),
            path,
            meta,
            body_len,
            modified: SystemTime::now(),
            legacy: false,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct SweepReport {
    pub(crate) removed_entries: usize,
    pub(crate) removed_sessions: usize,
    pub(crate) removed_temps: usize,
    pub(crate) evicted_by_quota: usize,
}

/// 128 bits para un nombre. Salen del estado aleatorio del sistema que usa
/// `HashMap`; sirven para no repetir nombres, no como secreto.
pub(crate) fn random_hex() -> String {
    use std::hash::{BuildHasher, Hasher};
    let high = std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish();
    let low = std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish();
    format!("{high:016x}{low:016x}")
}

impl Store {
    pub(crate) fn at(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Crea la sesión de este proceso y toma su cerrojo.
    pub(crate) fn begin_session(&self) -> std::io::Result<Session> {
        fs::create_dir_all(&self.root)?;
        let id = random_hex();
        let lock_path = self
            .root
            .join(format!("{SESSION_PREFIX}{id}.{LOCK_EXTENSION}"));
        let lock = take_lock(&lock_path)?;
        let dir = self.root.join(format!("{SESSION_PREFIX}{id}"));
        fs::create_dir(&dir)?;
        Ok(Session {
            dir,
            lock_path,
            lock: Some(lock),
        })
    }

    /// Las sesiones (identificador, carpeta, cerrojo) cuyo dueño ya no vive.
    fn orphan_sessions(&self) -> Vec<(String, PathBuf, PathBuf)> {
        let Ok(entries) = fs::read_dir(&self.root) else {
            return Vec::new();
        };
        entries
            .flatten()
            .filter_map(|entry| {
                let file_type = entry.file_type().ok()?;
                if !file_type.is_dir() || file_type.is_symlink() {
                    return None;
                }
                let name = entry.file_name().into_string().ok()?;
                let id = name.strip_prefix(SESSION_PREFIX)?.to_string();
                if id.is_empty() || !id.chars().all(|c| c.is_ascii_hexdigit()) {
                    return None;
                }
                let lock = self
                    .root
                    .join(format!("{SESSION_PREFIX}{id}.{LOCK_EXTENSION}"));
                lock_is_free(&lock).then(|| (id, entry.path(), lock))
            })
            .collect()
    }

    /// Todas las copias huérfanas, leyendo solo sus cabeceras. Las más nuevas
    /// primero.
    pub(crate) fn orphans(&self) -> Vec<Orphan> {
        let mut found = Vec::new();
        for (_, dir, _) in self.orphan_sessions() {
            let Ok(entries) = fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                if let Some(orphan) = read_orphan(&entry.path()) {
                    found.push(orphan);
                }
            }
        }
        found.extend(self.legacy_orphans(SystemTime::now()));
        found.sort_by_key(|orphan| std::cmp::Reverse(orphan.modified));
        found
    }

    /// Copias del formato anterior: archivos sueltos `session-*.md`.
    fn legacy_orphans(&self, now: SystemTime) -> Vec<Orphan> {
        let Ok(entries) = fs::read_dir(&self.root) else {
            return Vec::new();
        };
        entries
            .flatten()
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if !(name.starts_with("session-") && name.ends_with(".md")) {
                    return None;
                }
                let file_type = entry.file_type().ok()?;
                if !file_type.is_file() || file_type.is_symlink() {
                    return None;
                }
                let metadata = entry.metadata().ok()?;
                let modified = metadata.modified().ok()?;
                if now
                    .duration_since(modified)
                    .is_ok_and(|age| age < LEGACY_QUIET)
                {
                    return None;
                }
                let mut text = String::new();
                File::open(entry.path())
                    .ok()?
                    .take(2048)
                    .read_to_string(&mut text)
                    .ok();
                let title = text
                    .lines()
                    .map(|line| line.trim_start_matches('#').trim())
                    .find(|line| !line.is_empty())
                    .unwrap_or("")
                    .chars()
                    .take(80)
                    .collect::<String>();
                Some(Orphan {
                    path: entry.path(),
                    key: name.trim_end_matches(".md").to_string(),
                    meta: EntryMeta {
                        title,
                        saved_at: unix_seconds(modified),
                        ..EntryMeta::default()
                    },
                    body_len: metadata.len(),
                    modified,
                    legacy: true,
                })
            })
            .collect()
    }

    /// Lee el contenido de una copia, verificando su integridad.
    pub(crate) fn read(&self, orphan: &Orphan, limit: u64) -> Result<String, DecodeError> {
        let bytes = fs::read(&orphan.path).map_err(|_| DecodeError::Truncated)?;
        if orphan.legacy {
            if bytes.len() as u64 > limit {
                return Err(DecodeError::TooLarge);
            }
            return String::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8);
        }
        formato::decode(&bytes, limit).map(|(_, text)| text)
    }

    /// Pasa la copia a la sesión de este proceso con un renombrado atómico. Si
    /// otra ventana la adoptó antes, el renombrado falla y esta no la ve.
    pub(crate) fn adopt(&self, orphan: &Orphan, session: &Session) -> std::io::Result<PathBuf> {
        if orphan.legacy {
            // El formato anterior no tiene cabecera: se descarta tras leerla;
            // la pestaña restaurada escribe su propia copia.
            return Err(std::io::Error::new(
                ErrorKind::Unsupported,
                "una copia del formato anterior no se adopta",
            ));
        }
        let target = session
            .dir
            .join(format!("{}.{ENTRY_EXTENSION}", orphan.key));
        if target.exists() {
            return Err(std::io::Error::new(
                ErrorKind::AlreadyExists,
                "ya hay una copia con esa identidad en esta sesión",
            ));
        }
        // En Windows, renombrar por manija sigue al archivo aunque otro hilo ya
        // lo haya movido: dos ventanas podrían «ganar» a la vez. Un archivo de
        // reclamo creado con `create_new` decide un solo ganador.
        let claim = orphan.path.with_extension(CLAIM_EXTENSION);
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&claim)?;
        let moved = if orphan.path.is_file() {
            fs::rename(&orphan.path, &target)
        } else {
            Err(std::io::Error::new(
                ErrorKind::NotFound,
                "otra ventana ya tomó esta copia",
            ))
        };
        let _ = fs::remove_file(&claim);
        moved?;
        Ok(target)
    }

    pub(crate) fn discard(&self, orphan: &Orphan) -> std::io::Result<()> {
        match fs::remove_file(&orphan.path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Limpia lo que sobra, sin tocar nada que no sea de este módulo ni una
    /// sesión con dueño vivo.
    pub(crate) fn sweep(&self, now: SystemTime) -> SweepReport {
        let mut report = SweepReport::default();
        for (_, dir, lock) in self.orphan_sessions() {
            // Temporales abandonados dentro de la sesión.
            report.removed_temps += remove_stale_temps(&dir, now);
            let mut remaining = 0;
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let Ok(file_type) = entry.file_type() else {
                        continue;
                    };
                    if file_type.is_symlink() {
                        continue;
                    }
                    let age = entry
                        .metadata()
                        .ok()
                        .and_then(|metadata| metadata.modified().ok())
                        .and_then(|modified| now.duration_since(modified).ok());
                    let expired = match read_orphan(&path) {
                        // Una copia legible vence a los 30 días.
                        Some(_) => age.is_some_and(|age| age > ORPHAN_RETENTION),
                        // Una ilegible, al día. Lo que no es una copia de este
                        // módulo no se toca.
                        None if is_entry_name(&path) => {
                            age.is_some_and(|age| age > CORRUPT_RETENTION)
                        }
                        None => false,
                    };
                    if expired && fs::remove_file(&path).is_ok() {
                        report.removed_entries += 1;
                    } else {
                        remaining += 1;
                    }
                }
            }
            // Una sesión sin nada adentro no tiene motivo para existir.
            if remaining == 0 && fs::remove_dir(&dir).is_ok() {
                let _ = fs::remove_file(&lock);
                report.removed_sessions += 1;
            }
        }
        // Cerrojos sin carpeta (la sesión nunca llegó a escribir o ya se limpió).
        self.remove_dangling_locks();
        // Copias del formato anterior vencidas.
        for legacy in self.legacy_orphans(now) {
            let age = now.duration_since(legacy.modified).unwrap_or_default();
            if age > super::LEGACY_RETENTION && fs::remove_file(&legacy.path).is_ok() {
                report.removed_entries += 1;
            }
        }
        report.evicted_by_quota = self.enforce_quota();
        report
    }

    fn remove_dangling_locks(&self) {
        let Ok(entries) = fs::read_dir(&self.root) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let is_lock = path.extension().is_some_and(|ext| ext == LOCK_EXTENSION)
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(SESSION_PREFIX));
            if !is_lock || !lock_is_free(&path) {
                continue;
            }
            let dir = path.with_extension("");
            if !dir.exists() {
                let _ = fs::remove_file(&path);
            }
        }
    }

    /// Si hay demasiadas copias huérfanas o pesan demasiado, descarta las más
    /// viejas. Devuelve cuántas.
    fn enforce_quota(&self) -> usize {
        let mut orphans = self
            .orphans()
            .into_iter()
            .filter(|orphan| !orphan.legacy)
            .collect::<Vec<_>>();
        orphans.sort_by_key(|orphan| orphan.modified);
        let mut total: u64 = orphans.iter().map(|orphan| orphan.body_len).sum();
        let mut count = orphans.len();
        let mut evicted = 0;
        for orphan in orphans {
            if count <= MAX_ORPHANS && total <= MAX_ORPHAN_BYTES {
                break;
            }
            if self.discard(&orphan).is_ok() {
                count -= 1;
                total = total.saturating_sub(orphan.body_len);
                evicted += 1;
            }
        }
        evicted
    }
}

fn is_entry_name(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == ENTRY_EXTENSION)
}

fn unix_seconds(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

/// Lee la cabecera de una copia y confirma que el archivo mide lo que dice.
fn read_orphan(path: &Path) -> Option<Orphan> {
    if !is_entry_name(path) {
        return None;
    }
    let metadata = fs::symlink_metadata(path).ok()?;
    if !metadata.is_file() {
        return None;
    }
    let mut head = Vec::new();
    File::open(path)
        .ok()?
        .take(MAX_HEADER_BYTES as u64 + 2)
        .read_to_end(&mut head)
        .ok()?;
    let header = formato::decode_header(&head).ok()?;
    if header.body_start as u64 + header.body_len != metadata.len() {
        return None;
    }
    Some(Orphan {
        path: path.to_path_buf(),
        key: path.file_stem()?.to_str()?.to_string(),
        meta: header.meta,
        body_len: header.body_len,
        modified: metadata.modified().ok()?,
        legacy: false,
    })
}

fn remove_stale_temps(dir: &Path, now: SystemTime) -> usize {
    let Ok(entries) = fs::read_dir(dir) else {
        return 0;
    };
    let mut removed = 0;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        // Un reclamo de adopción que quedó a medias también sobra con el tiempo.
        let is_claim = file_type.is_file()
            && entry
                .path()
                .extension()
                .is_some_and(|ext| ext == CLAIM_EXTENSION);
        if file_type.is_symlink()
            || !(is_claim
                || (file_type.is_dir() && name.to_string_lossy().starts_with(TEMP_DIR_PREFIX)))
        {
            continue;
        }
        let old = entry
            .metadata()
            .ok()
            .and_then(|metadata| metadata.modified().ok())
            .and_then(|modified| now.duration_since(modified).ok())
            .is_some_and(|age| age > STALE_TEMP);
        if !old {
            continue;
        }
        let gone = if is_claim {
            fs::remove_file(entry.path()).is_ok()
        } else {
            fs::remove_dir_all(entry.path()).is_ok()
        };
        if gone {
            removed += 1;
        }
    }
    removed
}

/// Crea el cerrojo de una sesión y lo mantiene tomado.
#[cfg(windows)]
fn take_lock(path: &Path) -> std::io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt;
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .share_mode(0)
        .open(path)
}

#[cfg(not(windows))]
fn take_lock(path: &Path) -> std::io::Result<File> {
    let file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.try_lock().map_err(|error| match error {
        std::fs::TryLockError::Error(error) => error,
        std::fs::TryLockError::WouldBlock => {
            std::io::Error::new(ErrorKind::WouldBlock, "el cerrojo ya estaba tomado")
        }
    })?;
    Ok(file)
}

/// `true` si nadie mantiene tomado ese cerrojo: su dueño murió o no existe.
#[cfg(windows)]
fn lock_is_free(path: &Path) -> bool {
    use std::os::windows::fs::OpenOptionsExt;
    match OpenOptions::new()
        .read(true)
        .write(true)
        .share_mode(0)
        .open(path)
    {
        Ok(_) => true,
        Err(error) => error.kind() == ErrorKind::NotFound,
    }
}

#[cfg(not(windows))]
fn lock_is_free(path: &Path) -> bool {
    match OpenOptions::new().read(true).write(true).open(path) {
        Ok(file) => file.try_lock().is_ok(),
        Err(error) => error.kind() == ErrorKind::NotFound,
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT: AtomicU64 = AtomicU64::new(0);

    fn raiz() -> PathBuf {
        let raiz = std::env::temp_dir().join(format!(
            "visor-md-almacen-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&raiz).unwrap();
        raiz
    }

    /// Escribe una copia válida en la carpeta de una sesión.
    fn copia(session_dir: &Path, key: &str, titulo: &str, texto: &str) -> PathBuf {
        let meta = EntryMeta {
            title: titulo.into(),
            saved_at: 1,
            ..EntryMeta::default()
        };
        let path = session_dir.join(format!("{key}.{ENTRY_EXTENSION}"));
        fs::write(&path, formato::encode(&meta, texto)).unwrap();
        path
    }

    /// Una sesión que murió: su carpeta con copias y un cerrojo que nadie
    /// mantiene tomado.
    fn sesion_muerta(raiz: &Path, id: &str) -> PathBuf {
        let dir = raiz.join(format!("{SESSION_PREFIX}{id}"));
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            raiz.join(format!("{SESSION_PREFIX}{id}.{LOCK_EXTENSION}")),
            "",
        )
        .unwrap();
        dir
    }

    #[test]
    fn una_sesion_viva_no_es_huerfana_y_una_muerta_si() {
        let raiz = raiz();
        let almacen = Store::at(&raiz);
        let viva = almacen.begin_session().expect("se crea la sesión");
        copia(&viva.dir, "aaaa", "viva", "texto de la sesión viva");
        let muerta = sesion_muerta(&raiz, "00ff");
        copia(&muerta, "bbbb", "muerta", "texto de la sesión muerta");

        let huerfanas = almacen.orphans();
        assert_eq!(huerfanas.len(), 1);
        assert_eq!(huerfanas[0].meta.title, "muerta");
        assert_eq!(huerfanas[0].key, "bbbb");
        assert_eq!(
            almacen.read(&huerfanas[0], 1 << 20).unwrap(),
            "texto de la sesión muerta"
        );
        drop(viva);
        let _ = fs::remove_dir_all(raiz);
    }

    #[test]
    fn un_cerrojo_tomado_no_esta_libre_y_al_soltarlo_si() {
        let raiz = raiz();
        let ruta = raiz.join("s-cerrojo.lock");
        let cerrojo = take_lock(&ruta).expect("se toma");
        assert!(!lock_is_free(&ruta), "mientras el proceso vive");
        assert!(take_lock(&ruta).is_err(), "nadie más lo toma");
        // Al morir el proceso, el sistema suelta el archivo.
        drop(cerrojo);
        assert!(lock_is_free(&ruta));
        assert!(lock_is_free(&raiz.join("no-existe.lock")));
        let _ = fs::remove_dir_all(raiz);
    }

    #[test]
    fn adoptar_mueve_la_copia_a_la_sesion_propia_y_solo_gana_uno() {
        let raiz = raiz();
        let almacen = Store::at(&raiz);
        let mia = almacen.begin_session().unwrap();
        let muerta = sesion_muerta(&raiz, "00aa");
        copia(&muerta, "dddd", "para adoptar", "contenido");
        let huerfana = almacen.orphans().remove(0);

        let nueva = almacen.adopt(&huerfana, &mia).expect("la primera gana");
        assert!(nueva.starts_with(&mia.dir));
        assert!(!huerfana.path.exists());
        // La segunda ventana que intente lo mismo no encuentra el archivo.
        assert!(almacen.adopt(&huerfana, &mia).is_err());
        assert!(almacen.orphans().is_empty(), "ya no es huérfana");
        drop(mia);
        let _ = fs::remove_dir_all(raiz);
    }

    #[test]
    fn dos_hilos_que_adoptan_la_misma_copia_dejan_un_solo_ganador() {
        let raiz = raiz();
        let almacen = std::sync::Arc::new(Store::at(&raiz));
        let muerta = sesion_muerta(&raiz, "00bb");
        copia(&muerta, "eeee", "carrera", "contenido");
        let huerfana = almacen.orphans().remove(0);
        let sesiones = [
            std::sync::Arc::new(almacen.begin_session().unwrap()),
            std::sync::Arc::new(almacen.begin_session().unwrap()),
        ];
        let hilos = sesiones
            .iter()
            .map(|sesion| {
                let (almacen, sesion, huerfana) =
                    (almacen.clone(), sesion.clone(), huerfana.clone());
                std::thread::spawn(move || almacen.adopt(&huerfana, &sesion).is_ok())
            })
            .collect::<Vec<_>>();
        let ganadores = hilos
            .into_iter()
            .map(|hilo| hilo.join().unwrap())
            .filter(|gano| *gano)
            .count();
        assert_eq!(ganadores, 1);
        drop(sesiones);
        let _ = fs::remove_dir_all(raiz);
    }

    #[test]
    fn el_barrido_quita_sesiones_vacias_temporales_viejos_y_copias_vencidas() {
        let raiz = raiz();
        let almacen = Store::at(&raiz);
        // Sesión muerta y vacía: se va con su cerrojo.
        let vacia = sesion_muerta(&raiz, "0001");
        // Sesión muerta con una copia reciente y un temporal viejo.
        let con_copia = sesion_muerta(&raiz, "0002");
        copia(&con_copia, "ffff", "reciente", "texto");
        fs::create_dir_all(con_copia.join(format!("{TEMP_DIR_PREFIX}xyz"))).unwrap();
        // Algo que no es nuestro dentro de una sesión muerta no se toca.
        fs::write(con_copia.join("notas-del-usuario.txt"), "mío").unwrap();

        let ahora = SystemTime::now();
        // Recién creado: no se toca nada con contenido.
        let informe = almacen.sweep(ahora);
        assert_eq!(informe.removed_sessions, 1);
        assert!(!vacia.exists());
        assert!(con_copia.join("ffff.vmdr").exists());
        assert_eq!(informe.removed_temps, 0);

        // Una hora y un segundo después, el temporal sobra; la copia sigue.
        let informe = almacen.sweep(ahora + STALE_TEMP + Duration::from_secs(1));
        assert_eq!(informe.removed_temps, 1);
        assert!(con_copia.join("ffff.vmdr").exists());
        assert!(con_copia.join("notas-del-usuario.txt").exists());

        // Treinta días después la copia vence; lo ajeno impide borrar la sesión.
        let informe = almacen.sweep(ahora + ORPHAN_RETENTION + Duration::from_secs(1));
        assert_eq!(informe.removed_entries, 1);
        assert!(!con_copia.join("ffff.vmdr").exists());
        assert!(con_copia.join("notas-del-usuario.txt").exists());
        assert!(con_copia.exists());
        let _ = fs::remove_dir_all(raiz);
    }

    #[test]
    fn una_copia_cortada_no_se_lista_y_se_descarta_al_dia() {
        let raiz = raiz();
        let almacen = Store::at(&raiz);
        let muerta = sesion_muerta(&raiz, "0003");
        let buena = copia(&muerta, "1111", "buena", "texto completo");
        let mala = copia(&muerta, "2222", "mala", "texto que se corta");
        let bytes = fs::read(&mala).unwrap();
        fs::write(&mala, &bytes[..bytes.len() - 4]).unwrap();

        let listadas = almacen.orphans();
        assert_eq!(listadas.len(), 1);
        assert_eq!(listadas[0].key, "1111");
        let ahora = SystemTime::now();
        almacen.sweep(ahora);
        assert!(mala.exists(), "recién dañada: todavía no se descarta");
        let informe = almacen.sweep(ahora + CORRUPT_RETENTION + Duration::from_secs(1));
        assert_eq!(informe.removed_entries, 1);
        assert!(!mala.exists());
        assert!(buena.exists());
        let _ = fs::remove_dir_all(raiz);
    }

    #[test]
    fn la_cuota_descarta_las_mas_viejas() {
        let raiz = raiz();
        let almacen = Store::at(&raiz);
        let muerta = sesion_muerta(&raiz, "0004");
        for numero in 0..(MAX_ORPHANS + 3) {
            copia(&muerta, &format!("{numero:032x}"), "n", "x");
        }
        assert_eq!(almacen.orphans().len(), MAX_ORPHANS + 3);
        let informe = almacen.sweep(SystemTime::now());
        assert_eq!(informe.evicted_by_quota, 3);
        assert_eq!(almacen.orphans().len(), MAX_ORPHANS);
        let _ = fs::remove_dir_all(raiz);
    }

    #[test]
    fn las_copias_del_formato_anterior_se_listan_pero_no_se_adoptan() {
        let raiz = raiz();
        let almacen = Store::at(&raiz);
        let vieja = raiz.join("session-1234-99.md");
        fs::write(&vieja, "# Mi nota vieja\n\ncuerpo").unwrap();
        // Recién escrita: podría ser de una ventana con la versión anterior.
        assert!(almacen.orphans().is_empty());
        let antes = SystemTime::now() - LEGACY_QUIET - Duration::from_secs(5);
        let archivo = File::options().write(true).open(&vieja).unwrap();
        archivo.set_modified(antes).unwrap();
        drop(archivo);
        let listadas = almacen.orphans();
        assert_eq!(listadas.len(), 1);
        assert!(listadas[0].legacy);
        assert_eq!(listadas[0].meta.title, "Mi nota vieja");
        assert_eq!(
            almacen.read(&listadas[0], 1 << 20).unwrap(),
            "# Mi nota vieja\n\ncuerpo"
        );
        let sesion = almacen.begin_session().unwrap();
        assert!(almacen.adopt(&listadas[0], &sesion).is_err());
        drop(sesion);
        let _ = fs::remove_dir_all(raiz);
    }

    #[test]
    fn los_nombres_aleatorios_no_se_repiten_y_son_hex() {
        let a = random_hex();
        let b = random_hex();
        assert_ne!(a, b);
        assert_eq!(a.len(), 32);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
