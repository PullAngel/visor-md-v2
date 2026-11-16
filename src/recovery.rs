//! Recuperación local de cambios no guardados.
//!
//! Las copias viven fuera del documento y no sustituyen Guardar. Cada documento
//! sin guardar tiene a lo sumo una copia, escrita de forma atómica en la
//! carpeta de la sesión de este proceso. Si el proceso muere, la sesión queda
//! huérfana y sus copias se ofrecen al abrir de nuevo. El diseño completo está
//! en `docs/arquitectura-recuperacion-2026-11-15.md`.
//!
//! Contienen texto sin cifrar bajo el perfil del usuario, por lo que solo se
//! crean para una edición activa y se eliminan después de un guardado
//! confirmado.

mod almacen;
mod formato;

use atomicwrites::{AllowOverwrite, AtomicFile};
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{ErrorKind, Write};
#[cfg(test)]
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, SystemTime};

pub(crate) use almacen::Orphan;
use almacen::{ENTRY_EXTENSION, Session, Store, random_hex};
pub(crate) use formato::EntryMeta;

/// Cuánto vive una copia del formato anterior sin que nadie la abra.
pub(super) const LEGACY_RETENTION: Duration = Duration::from_secs(14 * 24 * 60 * 60);
const PRIVACY_NOTICE_FILE: &str = "privacy-notice-v1";

/// El almacén y la sesión de este proceso: se crean una vez, al primer uso.
struct Process {
    store: Store,
    session: Session,
}

fn process() -> Option<&'static Process> {
    static PROCESS: OnceLock<Option<Process>> = OnceLock::new();
    PROCESS
        .get_or_init(|| {
            let root = ensure_root().ok()?;
            let store = Store::at(root);
            // Limpiar antes de crear la sesión propia: nunca toca una viva.
            let _ = store.sweep(SystemTime::now());
            let session = store.begin_session().ok()?;
            Some(Process { store, session })
        })
        .as_ref()
}

/// Una copia como la escribe el almacén, para pruebas de la interfaz.
#[cfg(test)]
pub(crate) fn encode_for_tests(meta: &EntryMeta, body: &str) -> Vec<u8> {
    formato::encode(meta, body)
}

/// Las copias que dejó una sesión que ya no existe, de la más nueva a la más
/// vieja.
pub(crate) fn orphans() -> Vec<Orphan> {
    process().map(|p| p.store.orphans()).unwrap_or_default()
}

/// Lee el texto de una copia huérfana, verificando su integridad.
pub(crate) fn read_orphan(orphan: &Orphan, limit: u64) -> Result<String, &'static str> {
    use formato::DecodeError;
    let process = process().ok_or("la recuperación no está disponible")?;
    process
        .store
        .read(orphan, limit)
        .map_err(|error| match error {
            DecodeError::TooLarge => "la copia es más grande de lo que se puede abrir",
            DecodeError::UnsupportedVersion(_) => "la copia es de una versión más nueva",
            _ => "la copia está dañada",
        })
}

pub(crate) fn discard_orphan(orphan: &Orphan) -> Result<(), std::io::Error> {
    match process() {
        Some(process) => process.store.discard(orphan),
        None => Ok(()),
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RecoverySession {
    path: PathBuf,
    latest_request: Arc<AtomicU64>,
    write_lock: Arc<Mutex<()>>,
    /// Hash de lo que ya está en disco; 0 si no hay copia. Evita reescribir
    /// lo que no cambió.
    last_hash: Arc<AtomicU64>,
    /// La última escritura falló y todavía no se avisó a la persona.
    failure_pending: Arc<AtomicBool>,
    /// La última escritura falló (se mantiene hasta que una tenga éxito).
    failed: Arc<AtomicBool>,
    /// Lo que se sabe del documento (título, origen), que viaja en la cabecera.
    meta: Arc<Mutex<EntryMeta>>,
}

impl RecoverySession {
    /// Una copia nueva en la sesión de este proceso. Su archivo no existe
    /// hasta la primera escritura.
    pub(crate) fn start() -> Result<Self, std::io::Error> {
        let process = process().ok_or_else(|| {
            std::io::Error::new(
                ErrorKind::NotFound,
                "no hay una carpeta de perfil para la recuperación",
            )
        })?;
        let name = format!("{}.{ENTRY_EXTENSION}", random_hex());
        Ok(Self::at_path(process.session.dir.join(name)))
    }

    /// Toma una copia huérfana como propia: sigue siendo la misma, ahora de
    /// una pestaña de esta sesión.
    pub(crate) fn adopt(orphan: &Orphan) -> Result<Self, std::io::Error> {
        let process = process().ok_or_else(|| {
            std::io::Error::new(ErrorKind::NotFound, "la recuperación no está disponible")
        })?;
        let path = process.store.adopt(orphan, &process.session)?;
        let session = Self::at_path(path);
        if let Ok(mut meta) = session.meta.lock() {
            *meta = orphan.meta.clone();
        }
        Ok(session)
    }

    fn at_path(path: PathBuf) -> Self {
        Self {
            path,
            latest_request: Arc::new(AtomicU64::new(0)),
            write_lock: Arc::new(Mutex::new(())),
            last_hash: Arc::new(AtomicU64::new(0)),
            failure_pending: Arc::new(AtomicBool::new(false)),
            failed: Arc::new(AtomicBool::new(false)),
            meta: Arc::new(Mutex::new(EntryMeta::default())),
        }
    }

    /// Actualiza lo que viaja en la cabecera (título, origen, huella). Se
    /// guarda en la próxima escritura.
    pub(crate) fn set_meta(&self, meta: EntryMeta) {
        if let Ok(mut current) = self.meta.lock() {
            *current = meta;
        }
    }

    /// Entrega la escritura al hilo escritor, que conserva solo la última
    /// versión pendiente de cada copia. Nunca bloquea al llamador.
    pub(crate) fn submit(&self, source: String, request: u64) {
        writer().submit(self.clone(), source, request);
    }

    /// `true` una sola vez por racha de fallos: la interfaz avisa y no repite.
    pub(crate) fn take_new_failure(&self) -> bool {
        self.failure_pending.swap(false, Ordering::AcqRel)
    }

    /// La última escritura falló; la copia en disco puede estar desactualizada.
    #[allow(dead_code)]
    pub(crate) fn is_failing(&self) -> bool {
        self.failed.load(Ordering::Acquire)
    }

    pub(crate) fn write(&self, source: &str) -> Result<(), std::io::Error> {
        self.latest_request.fetch_add(1, Ordering::AcqRel);
        let _guard = self.lock_writes()?;
        self.write_unlocked(source)
    }

    pub(crate) fn next_write_request(&self) -> u64 {
        self.latest_request.fetch_add(1, Ordering::AcqRel) + 1
    }

    /// Una tarea antigua nunca puede reemplazar una recuperación solicitada
    /// después. El lock serializa el reemplazo; el contador decide cuál es la
    /// versión vigente sin retener el contenido en memoria compartida.
    pub(crate) fn write_if_current(
        &self,
        source: &str,
        request: u64,
    ) -> Result<bool, std::io::Error> {
        let _guard = self.lock_writes()?;
        if self.latest_request.load(Ordering::Acquire) != request {
            return Ok(false);
        }
        self.write_unlocked(source)?;
        Ok(true)
    }

    fn write_unlocked(&self, source: &str) -> Result<(), std::io::Error> {
        let mut meta = self
            .meta
            .lock()
            .map(|meta| meta.clone())
            .unwrap_or_default();
        let hash = content_hash(source, &meta);
        if self.last_hash.load(Ordering::Acquire) == hash && self.path.is_file() {
            // Lo que está en disco ya es esto: no se reescribe.
            return Ok(());
        }
        meta.saved_at = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_secs());
        let bytes = formato::encode(&meta, source);
        let result = AtomicFile::new(&self.path, AllowOverwrite)
            .write(|file| {
                file.write_all(&bytes)?;
                file.sync_all()
            })
            .map_err(std::io::Error::from);
        match &result {
            Ok(()) => {
                self.last_hash.store(hash, Ordering::Release);
                self.failed.store(false, Ordering::Release);
            }
            Err(_) => {
                self.last_hash.store(0, Ordering::Release);
                if !self.failed.swap(true, Ordering::AcqRel) {
                    self.failure_pending.store(true, Ordering::Release);
                }
            }
        }
        result
    }

    pub(crate) fn clear(&self) -> Result<(), std::io::Error> {
        self.latest_request.fetch_add(1, Ordering::AcqRel);
        let _guard = self.lock_writes()?;
        self.last_hash.store(0, Ordering::Release);
        match fs::remove_file(&self.path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn lock_writes(&self) -> Result<std::sync::MutexGuard<'_, ()>, std::io::Error> {
        self.write_lock
            .lock()
            .map_err(|_| std::io::Error::other("el lock de recuperación quedó invalidado"))
    }

    /// Devuelve `true` solo en la primera sesión que pudo registrar el aviso.
    /// El marcador no contiene contenido del documento.
    pub(crate) fn privacy_notice_needed() -> Result<bool, std::io::Error> {
        let root = ensure_root()?;
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join(PRIVACY_NOTICE_FILE))
        {
            Ok(_) => Ok(true),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => Ok(false),
            Err(error) => Err(error),
        }
    }

    #[cfg(test)]
    fn at(path: impl AsRef<Path>) -> Self {
        Self::at_path(path.as_ref().to_path_buf())
    }
}

/// Hash del texto y de lo que va en la cabecera: un cambio de título también
/// pide reescribir.
fn content_hash(source: &str, meta: &EntryMeta) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    source.hash(&mut hasher);
    meta.title.hash(&mut hasher);
    meta.origin_path.hash(&mut hasher);
    meta.base.hash(&mut hasher);
    // 0 significa «sin copia»: un contenido nunca puede valer eso.
    hasher.finish().max(1)
}

/// Carpeta de las copias dentro del perfil de la persona. Nunca cae a la
/// carpeta temporal del sistema: en Linux es `/tmp`, compartida y con nombres
/// predecibles. Sin perfil, la recuperación no está disponible.
fn recovery_root() -> Option<PathBuf> {
    // Las pruebas nunca tocan la carpeta real de la persona.
    #[cfg(test)]
    return Some(
        std::env::var_os("VISOR_TEST_RECOVERY_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                std::env::temp_dir().join(format!(
                    "visor-md-recuperacion-prueba-{}",
                    std::process::id()
                ))
            }),
    );
    #[cfg(not(test))]
    {
        let base = std::env::var_os("LOCALAPPDATA")
            .or_else(|| std::env::var_os("XDG_STATE_HOME"))
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .or_else(|| {
                std::env::var_os("HOME")
                    .map(PathBuf::from)
                    .filter(|path| path.is_absolute())
                    .map(|home| home.join(".local").join("state"))
            })?;
        Some(base.join("VisorMD").join("recovery"))
    }
}

/// La carpeta, creada y, en sistemas Unix, visible solo para su dueño.
fn ensure_root() -> Result<PathBuf, std::io::Error> {
    let root = recovery_root().ok_or_else(|| {
        std::io::Error::new(
            ErrorKind::NotFound,
            "no hay una carpeta de perfil para la recuperación",
        )
    })?;
    fs::create_dir_all(&root)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
    }
    Ok(root)
}

/// Un solo hilo escribe todas las copias. Cada copia tiene a lo sumo una
/// versión pendiente: una ráfaga de cambios produce una escritura, no cien.
struct Writer {
    pending: Mutex<HashMap<PathBuf, Job>>,
    wake: Condvar,
}

struct Job {
    session: RecoverySession,
    source: String,
    request: u64,
}

fn writer() -> &'static Writer {
    static WRITER: OnceLock<&'static Writer> = OnceLock::new();
    WRITER.get_or_init(|| {
        let writer: &'static Writer = Box::leak(Box::new(Writer {
            pending: Mutex::new(HashMap::new()),
            wake: Condvar::new(),
        }));
        std::thread::Builder::new()
            .name("recuperacion".into())
            .spawn(move || writer.run())
            .expect("el sistema permite crear el hilo de recuperación");
        writer
    })
}

impl Writer {
    fn submit(&self, session: RecoverySession, source: String, request: u64) {
        let Ok(mut pending) = self.pending.lock() else {
            return;
        };
        // El último gana: reemplaza la versión pendiente de la misma copia.
        pending.insert(
            session.path.clone(),
            Job {
                session,
                source,
                request,
            },
        );
        self.wake.notify_one();
    }

    fn run(&self) {
        loop {
            let jobs: Vec<Job> = {
                let Ok(mut pending) = self.pending.lock() else {
                    return;
                };
                while pending.is_empty() {
                    pending = match self.wake.wait(pending) {
                        Ok(guard) => guard,
                        Err(_) => return,
                    };
                }
                pending.drain().map(|(_, job)| job).collect()
            };
            for job in jobs {
                let _ = job.session.write_if_current(&job.source, job.request);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temporal() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "visor-md-recovery-{}-{}",
            std::process::id(),
            random_hex()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn texto_de(path: &Path) -> String {
        formato::decode(&fs::read(path).unwrap(), 1 << 20)
            .expect("la copia es válida")
            .1
    }

    #[test]
    fn una_recuperacion_se_escribe_con_cabecera_y_se_elimina_sin_tocar_el_documento() {
        let dir = temporal();
        let path = dir.join("copia.vmdr");
        let recovery = RecoverySession::at(&path);
        recovery.set_meta(EntryMeta {
            title: "Mi nota".into(),
            origin_path: "C:/notas/mi-nota.md".into(),
            ..EntryMeta::default()
        });
        recovery.write("cambios sin guardar").unwrap();
        let (meta, texto) = formato::decode(&fs::read(&path).unwrap(), 1 << 20).unwrap();
        assert_eq!(texto, "cambios sin guardar");
        assert_eq!(meta.title, "Mi nota");
        assert_eq!(meta.origin_path, "C:/notas/mi-nota.md");
        assert!(meta.saved_at > 0);
        recovery.clear().unwrap();
        assert!(!path.exists());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn una_tarea_antigua_no_reemplaza_la_recuperacion_mas_nueva() {
        let dir = temporal();
        let path = dir.join("copia.vmdr");
        let recovery = RecoverySession::at(&path);
        let old = recovery.next_write_request();
        let current = recovery.next_write_request();

        assert!(!recovery.write_if_current("versión vieja", old).unwrap());
        assert!(
            recovery
                .write_if_current("versión vigente", current)
                .unwrap()
        );
        assert_eq!(texto_de(&path), "versión vigente");
        recovery.clear().unwrap();
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn limpiar_invalida_una_escritura_pendiente() {
        let dir = temporal();
        let path = dir.join("copia.vmdr");
        let recovery = RecoverySession::at(&path);
        let pending = recovery.next_write_request();

        recovery.clear().unwrap();

        assert!(
            !recovery
                .write_if_current("no debe reaparecer", pending)
                .unwrap()
        );
        assert!(!path.exists());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn no_se_reescribe_un_contenido_que_no_cambio_pero_si_un_titulo_nuevo() {
        let dir = temporal();
        let path = dir.join("copia.vmdr");
        let recovery = RecoverySession::at(&path);
        recovery.write("uno").unwrap();
        let antes = fs::metadata(&path).unwrap().modified().unwrap();
        std::thread::sleep(Duration::from_millis(40));
        recovery.write("uno").unwrap();
        assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), antes);
        recovery.write("dos").unwrap();
        assert_ne!(fs::metadata(&path).unwrap().modified().unwrap(), antes);
        assert_eq!(texto_de(&path), "dos");
        // Un título nuevo con el mismo texto sí reescribe la cabecera.
        recovery.set_meta(EntryMeta {
            title: "Nuevo título".into(),
            ..EntryMeta::default()
        });
        recovery.write("dos").unwrap();
        let (meta, _) = formato::decode(&fs::read(&path).unwrap(), 1 << 20).unwrap();
        assert_eq!(meta.title, "Nuevo título");
        // Si el archivo desapareció, se vuelve a escribir aunque el texto sea igual.
        fs::remove_file(&path).unwrap();
        recovery.write("dos").unwrap();
        assert!(path.exists());
        recovery.clear().unwrap();
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn un_fallo_de_escritura_se_avisa_una_vez_y_se_limpia_al_tener_exito() {
        let dir = temporal();
        // La carpeta de destino no existe: la escritura falla.
        let recovery = RecoverySession::at(dir.join("no-existe").join("copia.vmdr"));
        assert!(recovery.write("texto").is_err());
        assert!(recovery.is_failing());
        assert!(recovery.take_new_failure());
        assert!(
            !recovery.take_new_failure(),
            "un fallo se avisa una sola vez"
        );
        assert!(recovery.write("otra vez").is_err());
        assert!(
            !recovery.take_new_failure(),
            "una racha de fallos no repite el aviso"
        );

        fs::create_dir_all(dir.join("no-existe")).unwrap();
        recovery.write("ya").unwrap();
        assert!(!recovery.is_failing());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn el_hilo_escritor_conserva_solo_la_ultima_version_pendiente() {
        let dir = temporal();
        let path = dir.join("copia.vmdr");
        let recovery = RecoverySession::at(&path);
        for version in 1..=50 {
            let request = recovery.next_write_request();
            recovery.submit(format!("versión {version}"), request);
        }
        // El hilo es asíncrono: se espera a que la última quede en disco.
        let fin = std::time::Instant::now() + Duration::from_secs(5);
        loop {
            let listo = fs::read(&path)
                .ok()
                .and_then(|bytes| formato::decode(&bytes, 1 << 20).ok())
                .is_some_and(|(_, texto)| texto == "versión 50");
            if listo {
                break;
            }
            assert!(
                std::time::Instant::now() < fin,
                "la última versión no llegó"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
        recovery.clear().unwrap();
        let _ = fs::remove_dir_all(dir);
    }

    /// El proceso hijo de la prueba de muerte real: abre una sesión, escribe una
    /// copia y espera sin cerrar nada. Solo corre cuando lo lanza el padre.
    #[test]
    #[ignore = "lo lanza morir_el_proceso_deja_la_copia_recuperable"]
    fn hijo_que_escribe_y_se_cuelga() {
        let recovery = RecoverySession::start().expect("el hijo tiene almacén");
        recovery.set_meta(EntryMeta {
            title: "Escrito por el hijo".into(),
            origin_path: "C:/notas/hijo.md".into(),
            ..EntryMeta::default()
        });
        recovery.write("texto que no se guardó").expect("escribe");
        println!("LISTO");
        loop {
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    #[test]
    fn morir_el_proceso_deja_la_copia_recuperable_y_la_sesion_huerfana() {
        let raiz = std::env::temp_dir().join(format!(
            "visor-md-muerte-{}-{}",
            std::process::id(),
            random_hex()
        ));
        fs::create_dir_all(&raiz).unwrap();
        let mut hijo = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "recovery::tests::hijo_que_escribe_y_se_cuelga",
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ])
            .env("VISOR_TEST_RECOVERY_ROOT", &raiz)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("se lanza el proceso hijo");

        // Espera a que el hijo tenga su copia en disco.
        let almacen = Store::at(&raiz);
        let fin = std::time::Instant::now() + Duration::from_secs(60);
        let sesion_viva = loop {
            let archivos = fs::read_dir(&raiz)
                .into_iter()
                .flatten()
                .flatten()
                .filter(|entrada| entrada.path().is_dir())
                .flat_map(|dir| fs::read_dir(dir.path()).into_iter().flatten().flatten())
                .filter(|entrada| {
                    entrada
                        .path()
                        .extension()
                        .is_some_and(|ext| ext == ENTRY_EXTENSION)
                })
                .count();
            if archivos > 0 {
                break true;
            }
            assert!(
                std::time::Instant::now() < fin,
                "el hijo no escribió su copia"
            );
            std::thread::sleep(Duration::from_millis(50));
        };
        assert!(sesion_viva);
        // Mientras el hijo vive, su sesión no es huérfana.
        assert!(almacen.orphans().is_empty(), "una sesión viva no se ofrece");

        // Matar el proceso equivale a `taskkill /F`: no corre ningún cierre.
        hijo.kill().expect("se mata al hijo");
        hijo.wait().expect("se espera su fin");

        let huerfanas = almacen.orphans();
        assert_eq!(huerfanas.len(), 1, "la copia quedó como huérfana");
        assert_eq!(huerfanas[0].meta.title, "Escrito por el hijo");
        assert_eq!(huerfanas[0].meta.origin_path, "C:/notas/hijo.md");
        assert_eq!(
            almacen.read(&huerfanas[0], 1 << 20).unwrap(),
            "texto que no se guardó"
        );
        let _ = fs::remove_dir_all(raiz);
    }

    #[test]
    fn adoptar_una_huerfana_la_hace_propia_y_conserva_su_cabecera() {
        // Una sesión muerta en la carpeta de las pruebas, con su copia.
        let raiz = ensure_root().unwrap();
        let id = random_hex();
        let dir = raiz.join(format!("s-{id}"));
        fs::create_dir_all(&dir).unwrap();
        fs::write(raiz.join(format!("s-{id}.lock")), "").unwrap();
        let meta = EntryMeta {
            title: "Para adoptar".into(),
            origin_path: "C:/notas/x.md".into(),
            saved_at: 5,
            ..EntryMeta::default()
        };
        let clave = random_hex();
        fs::write(
            dir.join(format!("{clave}.vmdr")),
            formato::encode(&meta, "cuerpo"),
        )
        .unwrap();

        let huerfana = orphans()
            .into_iter()
            .find(|copia| copia.key == clave)
            .expect("la copia se ofrece");
        assert_eq!(read_orphan(&huerfana, 1 << 20).unwrap(), "cuerpo");
        let sesion = RecoverySession::adopt(&huerfana).expect("se adopta");
        assert!(!huerfana.path.exists());
        assert!(orphans().iter().all(|copia| copia.key != clave));
        // Al escribir, la cabecera conserva el origen heredado.
        sesion.write("cuerpo nuevo").unwrap();
        let (leida, texto) = formato::decode(&fs::read(&sesion.path).unwrap(), 1 << 20).unwrap();
        assert_eq!(texto, "cuerpo nuevo");
        assert_eq!(leida.origin_path, "C:/notas/x.md");
        sesion.clear().unwrap();
    }

    #[test]
    fn el_marcador_de_privacidad_no_es_una_copia() {
        // No tiene la extensión de una copia, así que ningún listado lo toma.
        assert!(!PRIVACY_NOTICE_FILE.ends_with(ENTRY_EXTENSION));
    }
}
