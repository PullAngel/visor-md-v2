//! Preferencias locales pequeñas y versionadas.
//!
//! No contienen texto, rutas de documentos ni permisos concedidos por una
//! bóveda. Un archivo ausente, grande o inválido degrada a valores seguros.

use atomicwrites::{AllowOverwrite, AtomicFile};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const SETTINGS_VERSION: u32 = 1;
const MAX_SETTINGS_BYTES: u64 = 4 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Settings {
    pub(crate) recovery_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            recovery_enabled: true,
        }
    }
}

impl Settings {
    pub(crate) fn load() -> Self {
        load_from(&settings_path()).unwrap_or_default()
    }

    pub(crate) fn store(self) -> Result<(), std::io::Error> {
        let path = settings_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        store_to(&path, self)
    }
}

fn settings_path() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    base.join("VisorMD").join("settings-v1.conf")
}

fn load_from(path: &Path) -> Result<Settings, std::io::Error> {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Settings::default());
        }
        Err(error) => return Err(error),
    };
    if !metadata.is_file() || metadata.len() > MAX_SETTINGS_BYTES {
        return Ok(Settings::default());
    }
    let source = fs::read_to_string(path)?;
    let mut version = None;
    let mut recovery_enabled = None;
    for line in source.lines().map(str::trim) {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key.trim() {
            "version" => version = value.trim().parse::<u32>().ok(),
            "recovery_enabled" => {
                recovery_enabled = match value.trim() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                }
            }
            _ => {}
        }
    }
    if version != Some(SETTINGS_VERSION) {
        return Ok(Settings::default());
    }
    Ok(Settings {
        recovery_enabled: recovery_enabled.unwrap_or(true),
    })
}

fn store_to(path: &Path, settings: Settings) -> Result<(), std::io::Error> {
    let source = format!(
        "version={SETTINGS_VERSION}\nrecovery_enabled={}\n",
        settings.recovery_enabled
    );
    AtomicFile::new(path, AllowOverwrite)
        .write(|file| {
            file.write_all(source.as_bytes())?;
            file.sync_all()
        })
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file() -> PathBuf {
        std::env::temp_dir().join(format!(
            "visor-md-settings-{}-{}.conf",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn conserva_la_preferencia_sin_guardar_datos_del_documento() {
        let path = temp_file();
        store_to(
            &path,
            Settings {
                recovery_enabled: false,
            },
        )
        .unwrap();

        assert_eq!(
            load_from(&path).unwrap(),
            Settings {
                recovery_enabled: false
            }
        );
        let stored = fs::read_to_string(&path).unwrap();
        assert!(!stored.contains("path"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn una_version_desconocida_vuelve_al_valor_seguro() {
        let path = temp_file();
        fs::write(&path, "version=99\nrecovery_enabled=false\n").unwrap();

        assert_eq!(load_from(&path).unwrap(), Settings::default());
        let _ = fs::remove_file(path);
    }
}
