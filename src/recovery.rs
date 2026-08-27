//! Recuperación local de cambios no guardados.
//!
//! Las copias viven fuera del documento y no sustituyen Guardar. Contienen
//! texto sin cifrar bajo el perfil del usuario, por lo que solo se crean para
//! una edición activa y se eliminan después de un guardado confirmado.

use atomicwrites::{AllowOverwrite, AtomicFile};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub(crate) struct RecoverySession {
    path: PathBuf,
}

impl RecoverySession {
    pub(crate) fn start() -> Result<Self, std::io::Error> {
        let root = recovery_root();
        fs::create_dir_all(&root)?;
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
                (metadata.is_file() && metadata.len() <= limit)
                    .then_some((metadata.modified().ok(), entry.path()))
            })
            .max_by_key(|(modified, _)| *modified)
            .map(|(_, path)| path);
        candidate.map(fs::read_to_string).transpose()
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
}
