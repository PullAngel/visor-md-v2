//! Recuperación local de cambios no guardados.
//!
//! Las copias viven fuera del documento y no sustituyen Guardar. Contienen
//! texto sin cifrar bajo el perfil del usuario, por lo que solo se crean para
//! una edición activa y se eliminan después de un guardado confirmado.

use atomicwrites::{AllowOverwrite, AtomicFile};
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, SystemTime};

const RETENTION: Duration = Duration::from_secs(14 * 24 * 60 * 60);
/// Un temporal de `atomicwrites` que sigue ahí pasada esta edad es de un
/// proceso que murió a mitad de una escritura.
const STALE_TEMP: Duration = Duration::from_secs(60 * 60);
const TEMP_DIR_PREFIX: &str = ".atomicwrite";
const PRIVACY_NOTICE_FILE: &str = "privacy-notice-v1";

#[derive(Clone, Debug)]
pub(crate) struct RecoverySession {
    path: PathBuf,
    latest_request: Arc<AtomicU64>,
    write_lock: Arc<Mutex<()>>,
    /// Hash del contenido que ya está en disco; 0 si no hay copia. Evita
    /// reescribir lo que no cambió.
    last_hash: Arc<AtomicU64>,
    /// La última escritura falló y todavía no se avisó a la persona.
    failure_pending: Arc<AtomicBool>,
    /// La última escritura falló (se mantiene hasta que una tenga éxito).
    failed: Arc<AtomicBool>,
}

impl RecoverySession {
    pub(crate) fn start() -> Result<Self, std::io::Error> {
        let root = ensure_root()?;
        let _ = cleanup_stale(&root, SystemTime::now());
        let id = format!("session-{}-{}.md", std::process::id(), unique_suffix());
        Ok(Self::at_path(root.join(id)))
    }

    fn at_path(path: PathBuf) -> Self {
        Self {
            path,
            latest_request: Arc::new(AtomicU64::new(0)),
            write_lock: Arc::new(Mutex::new(())),
            last_hash: Arc::new(AtomicU64::new(0)),
            failure_pending: Arc::new(AtomicBool::new(false)),
            failed: Arc::new(AtomicBool::new(false)),
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
        let hash = content_hash(source);
        if self.last_hash.load(Ordering::Acquire) == hash && self.path.is_file() {
            // Lo que está en disco ya es esto: no se reescribe.
            return Ok(());
        }
        let result = AtomicFile::new(&self.path, AllowOverwrite)
            .write(|file| {
                file.write_all(source.as_bytes())?;
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

    /// Lee una recuperación anterior solo tras una acción explícita. Ignora
    /// archivos que superen el mismo límite de documento que la apertura normal.
    pub(crate) fn latest_pending(limit: u64) -> Result<Option<String>, std::io::Error> {
        let Some(root) = recovery_root() else {
            return Ok(None);
        };
        let Ok(entries) = fs::read_dir(root) else {
            return Ok(None);
        };
        let candidate = entries
            .flatten()
            .filter_map(|entry| {
                let metadata = entry.metadata().ok()?;
                let name = entry.file_name();
                let is_session = is_session_file_name(name.to_string_lossy().as_ref());
                (metadata.is_file() && is_session && metadata.len() <= limit)
                    .then_some((metadata.modified().ok(), entry.path()))
            })
            .max_by_key(|(modified, _)| *modified)
            .map(|(_, path)| path);
        candidate.map(fs::read_to_string).transpose()
    }

    /// Devuelve `true` solo en la primera sesión que pudo registrar el aviso.
    /// El marcador no contiene contenido del documento y se excluye de las
    /// recuperaciones restaurables.
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

fn content_hash(source: &str) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    source.hash(&mut hasher);
    // 0 significa «sin copia»: un contenido nunca puede valer eso.
    hasher.finish().max(1)
}

/// Carpeta de las copias dentro del perfil de la persona. Nunca cae a la
/// carpeta temporal del sistema: en Linux es `/tmp`, compartida y con nombres
/// predecibles. Sin perfil, la recuperación no está disponible.
fn recovery_root() -> Option<PathBuf> {
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

/// Elimina únicamente snapshots antiguos creados por Visor MD. No toca enlaces
/// simbólicos ni archivos ajenos aunque estén dentro del directorio de perfil.
fn cleanup_stale(root: &Path, now: SystemTime) -> Result<usize, std::io::Error> {
    let mut removed = 0;
    for entry in fs::read_dir(root)? {
        let Ok(entry) = entry else {
            continue;
        };
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let is_session = is_session_file_name(name.as_ref());
        let is_temp = name.starts_with(TEMP_DIR_PREFIX);
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_symlink() {
            continue;
        }
        // Una copia son archivos; un temporal huérfano de `atomicwrites` es
        // una carpeta con su archivo adentro.
        let (matches, limit) = if is_session && file_type.is_file() {
            (true, RETENTION)
        } else if is_temp && file_type.is_dir() {
            (true, STALE_TEMP)
        } else {
            (false, RETENTION)
        };
        if !matches {
            continue;
        }
        let Ok(modified) = entry.metadata().and_then(|metadata| metadata.modified()) else {
            continue;
        };
        if now.duration_since(modified).is_ok_and(|age| age > limit) {
            if file_type.is_dir() {
                fs::remove_dir_all(entry.path())?;
            } else {
                fs::remove_file(entry.path())?;
            }
            removed += 1;
        }
    }
    Ok(removed)
}

fn is_session_file_name(name: &str) -> bool {
    name.starts_with("session-") && name.ends_with(".md")
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn una_recuperacion_se_escribe_y_se_elimina_sin_tocar_el_documento() {
        let path = std::env::temp_dir().join(format!("visor-md-recovery-{}.md", unique_suffix()));
        let recovery = RecoverySession::at(&path);
        recovery.write("cambios sin guardar").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "cambios sin guardar");
        recovery.clear().unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn una_tarea_antigua_no_reemplaza_la_recuperacion_mas_nueva() {
        let path = std::env::temp_dir().join(format!("visor-md-recovery-{}.md", unique_suffix()));
        let recovery = RecoverySession::at(&path);
        let old = recovery.next_write_request();
        let current = recovery.next_write_request();

        assert!(!recovery.write_if_current("versión vieja", old).unwrap());
        assert!(
            recovery
                .write_if_current("versión vigente", current)
                .unwrap()
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), "versión vigente");
        recovery.clear().unwrap();
    }

    #[test]
    fn limpiar_invalida_una_escritura_pendiente() {
        let path = std::env::temp_dir().join(format!("visor-md-recovery-{}.md", unique_suffix()));
        let recovery = RecoverySession::at(&path);
        let pending = recovery.next_write_request();

        recovery.clear().unwrap();

        assert!(
            !recovery
                .write_if_current("no debe reaparecer", pending)
                .unwrap()
        );
        assert!(!path.exists());
    }

    #[test]
    fn la_limpieza_solo_elimina_sessions_antiguas_normales() {
        let root = std::env::temp_dir().join(format!("visor-md-recovery-root-{}", unique_suffix()));
        fs::create_dir_all(&root).unwrap();
        let old = root.join("session-old.md");
        let marker = root.join(PRIVACY_NOTICE_FILE);
        fs::write(&old, "viejo").unwrap();
        fs::write(&marker, "").unwrap();
        let removed = cleanup_stale(
            &root,
            SystemTime::now() + RETENTION + Duration::from_secs(1),
        )
        .unwrap();

        assert_eq!(removed, 1);
        assert!(!old.exists());
        assert!(marker.exists());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn la_limpieza_borra_temporales_huerfanos_viejos_y_respeta_los_recientes() {
        let root = std::env::temp_dir().join(format!("visor-md-recovery-tmp-{}", unique_suffix()));
        let viejo = root.join(format!("{TEMP_DIR_PREFIX}abc"));
        let ajeno = root.join("notas-del-usuario");
        fs::create_dir_all(&viejo).unwrap();
        fs::create_dir_all(&ajeno).unwrap();
        fs::write(viejo.join("tmpfile.tmp"), "a medias").unwrap();

        // Recién creado: no se toca (puede ser una escritura en curso).
        assert_eq!(cleanup_stale(&root, SystemTime::now()).unwrap(), 0);
        assert!(viejo.exists());

        let removed = cleanup_stale(
            &root,
            SystemTime::now() + STALE_TEMP + Duration::from_secs(1),
        )
        .unwrap();
        assert_eq!(removed, 1);
        assert!(!viejo.exists());
        assert!(ajeno.exists(), "lo que no es nuestro no se toca");
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn no_se_reescribe_un_contenido_que_no_cambio() {
        let path = std::env::temp_dir().join(format!("visor-md-recovery-{}.md", unique_suffix()));
        let recovery = RecoverySession::at(&path);
        recovery.write("uno").unwrap();
        let antes = fs::metadata(&path).unwrap().modified().unwrap();
        std::thread::sleep(Duration::from_millis(40));
        recovery.write("uno").unwrap();
        assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), antes);
        recovery.write("dos").unwrap();
        assert_ne!(fs::metadata(&path).unwrap().modified().unwrap(), antes);
        assert_eq!(fs::read_to_string(&path).unwrap(), "dos");
        // Si el archivo desapareció, se vuelve a escribir aunque el texto sea igual.
        fs::remove_file(&path).unwrap();
        recovery.write("dos").unwrap();
        assert!(path.exists());
        recovery.clear().unwrap();
    }

    #[test]
    fn un_fallo_de_escritura_se_avisa_una_vez_y_se_limpia_al_tener_exito() {
        let dir = std::env::temp_dir().join(format!("visor-md-recovery-fail-{}", unique_suffix()));
        // La carpeta de destino no existe: la escritura falla.
        let recovery = RecoverySession::at(dir.join("no-existe").join("copia.md"));
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
        let path = std::env::temp_dir().join(format!("visor-md-recovery-{}.md", unique_suffix()));
        let recovery = RecoverySession::at(&path);
        for version in 1..=50 {
            let request = recovery.next_write_request();
            recovery.submit(format!("versión {version}"), request);
        }
        // El hilo es asíncrono: se espera a que la última quede en disco.
        let fin = std::time::Instant::now() + Duration::from_secs(5);
        while fs::read_to_string(&path).ok().as_deref() != Some("versión 50") {
            assert!(
                std::time::Instant::now() < fin,
                "la última versión no llegó"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
        recovery.clear().unwrap();
    }

    #[test]
    fn el_marcador_de_privacidad_no_parece_una_recuperacion() {
        assert!(!is_session_file_name(PRIVACY_NOTICE_FILE));
        assert!(is_session_file_name("session-42-99.md"));
    }
}
