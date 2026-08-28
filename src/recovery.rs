//! Recuperación local de cambios no guardados.
//!
//! Las copias viven fuera del documento y no sustituyen Guardar. Contienen
//! texto sin cifrar bajo el perfil del usuario, por lo que solo se crean para
//! una edición activa y se eliminan después de un guardado confirmado.

use atomicwrites::{AllowOverwrite, AtomicFile};
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

const RETENTION: Duration = Duration::from_secs(14 * 24 * 60 * 60);
const PRIVACY_NOTICE_FILE: &str = "privacy-notice-v1";

#[derive(Clone, Debug)]
pub(crate) struct RecoverySession {
    path: PathBuf,
}

impl RecoverySession {
    pub(crate) fn start() -> Result<Self, std::io::Error> {
        let root = recovery_root();
        fs::create_dir_all(&root)?;
        let _ = cleanup_stale(&root, SystemTime::now());
        let id = format!("session-{}-{}.md", std::process::id(), unique_suffix());
        Ok(Self {
            path: root.join(id),
        })
    }

    pub(crate) fn write(&self, source: &str) -> Result<(), std::io::Error> {
        AtomicFile::new(&self.path, AllowOverwrite)
            .write(|file| {
                file.write_all(source.as_bytes())?;
                file.sync_all()
            })
            .map_err(Into::into)
    }

    pub(crate) fn clear(&self) -> Result<(), std::io::Error> {
        match fs::remove_file(&self.path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Lee una recuperación anterior solo tras una acción explícita. Ignora
    /// archivos que superen el mismo límite de documento que la apertura normal.
    pub(crate) fn latest_pending(limit: u64) -> Result<Option<String>, std::io::Error> {
        let root = recovery_root();
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
        let root = recovery_root();
        fs::create_dir_all(&root)?;
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
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

fn recovery_root() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    base.join("VisorMD").join("recovery")
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
        let is_session = is_session_file_name(name.to_string_lossy().as_ref());
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !is_session || !file_type.is_file() || file_type.is_symlink() {
            continue;
        }
        let Ok(modified) = entry.metadata().and_then(|metadata| metadata.modified()) else {
            continue;
        };
        if now
            .duration_since(modified)
            .is_ok_and(|age| age > RETENTION)
        {
            fs::remove_file(entry.path())?;
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
    fn el_marcador_de_privacidad_no_parece_una_recuperacion() {
        assert!(!is_session_file_name(PRIVACY_NOTICE_FILE));
        assert!(is_session_file_name("session-42-99.md"));
    }
}
