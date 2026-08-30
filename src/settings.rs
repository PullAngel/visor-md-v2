//! Preferencias locales pequeñas y versionadas.
//!
//! No contienen texto, rutas de documentos ni permisos concedidos por una
//! bóveda. Un archivo ausente, grande o inválido degrada a valores seguros.

use atomicwrites::{AllowOverwrite, AtomicFile};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const SETTINGS_VERSION: u32 = 2;
const MAX_SETTINGS_BYTES: u64 = 16 * 1024;
const MAX_DOCUMENT_MODES: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DocumentModePreference {
    Reading,
    Editing,
    Split,
}

impl DocumentModePreference {
    fn code(self) -> &'static str {
        match self {
            Self::Reading => "r",
            Self::Editing => "e",
            Self::Split => "s",
        }
    }

    fn parse(code: &str) -> Option<Self> {
        match code {
            "r" => Some(Self::Reading),
            "e" => Some(Self::Editing),
            "s" => Some(Self::Split),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Settings {
    pub(crate) recovery_enabled: bool,
    document_modes: Vec<(String, DocumentModePreference)>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            recovery_enabled: true,
            document_modes: Vec::new(),
        }
    }
}

impl Settings {
    pub(crate) fn load() -> Self {
        load_from(&settings_path()).unwrap_or_default()
    }

    pub(crate) fn store(&self) -> Result<(), std::io::Error> {
        let path = settings_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        store_to(&path, self)
    }

    pub(crate) fn document_mode(&self, path: &Path) -> Option<DocumentModePreference> {
        let key = document_key(path);
        self.document_modes
            .iter()
            .rev()
            .find_map(|(candidate, mode)| (candidate == &key).then_some(*mode))
    }

    pub(crate) fn remember_document_mode(&mut self, path: &Path, mode: DocumentModePreference) {
        let key = document_key(path);
        self.document_modes
            .retain(|(candidate, _)| candidate != &key);
        self.document_modes.push((key, mode));
        if self.document_modes.len() > MAX_DOCUMENT_MODES {
            self.document_modes
                .drain(..self.document_modes.len() - MAX_DOCUMENT_MODES);
        }
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
    let mut document_modes = Vec::new();
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
            "document_mode" => {
                let Some((key, code)) = value.trim().split_once(',') else {
                    continue;
                };
                if key.len() == 16
                    && key.bytes().all(|byte| byte.is_ascii_hexdigit())
                    && let Some(mode) = DocumentModePreference::parse(code)
                {
                    document_modes.retain(|(candidate, _)| candidate != key);
                    document_modes.push((key.to_ascii_lowercase(), mode));
                    if document_modes.len() > MAX_DOCUMENT_MODES {
                        document_modes.remove(0);
                    }
                }
            }
            _ => {}
        }
    }
    if !matches!(version, Some(1) | Some(SETTINGS_VERSION)) {
        return Ok(Settings::default());
    }
    Ok(Settings {
        recovery_enabled: recovery_enabled.unwrap_or(true),
        document_modes: if version == Some(SETTINGS_VERSION) {
            document_modes
        } else {
            Vec::new()
        },
    })
}

fn store_to(path: &Path, settings: &Settings) -> Result<(), std::io::Error> {
    let mut source = format!(
        "version={SETTINGS_VERSION}\nrecovery_enabled={}\n",
        settings.recovery_enabled
    );
    for (key, mode) in &settings.document_modes {
        source.push_str(&format!("document_mode={key},{}\n", mode.code()));
    }
    AtomicFile::new(path, AllowOverwrite)
        .write(|file| {
            file.write_all(source.as_bytes())?;
            file.sync_all()
        })
        .map_err(Into::into)
}

fn document_key(path: &Path) -> String {
    let canonical = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let mut normalized = canonical.to_string_lossy().replace('\\', "/");
    #[cfg(target_os = "windows")]
    {
        normalized = normalized.to_lowercase();
    }
    // FNV-1a es estable entre versiones y plataformas. No pretende ser un
    // secreto criptográfico: evita guardar la ruta en claro, no anonimizarla
    // frente a alguien que ya controla el perfil local.
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in normalized.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
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
        let settings = Settings {
            recovery_enabled: false,
            document_modes: Vec::new(),
        };
        store_to(&path, &settings).unwrap();

        assert_eq!(
            load_from(&path).unwrap(),
            Settings {
                recovery_enabled: false,
                document_modes: Vec::new(),
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

    #[test]
    fn recuerda_modo_sin_persistir_ruta_ni_nombre() {
        let settings_path = temp_file();
        let document = std::env::temp_dir()
            .join("apuntes privados")
            .join("redes.md");
        let mut settings = Settings::default();
        settings.remember_document_mode(&document, DocumentModePreference::Split);
        store_to(&settings_path, &settings).unwrap();

        let stored = fs::read_to_string(&settings_path).unwrap();
        assert!(!stored.contains("apuntes privados"));
        assert!(!stored.contains("redes.md"));
        let loaded = load_from(&settings_path).unwrap();
        assert_eq!(
            loaded.document_mode(&document),
            Some(DocumentModePreference::Split)
        );
        let _ = fs::remove_file(settings_path);
    }

    #[test]
    fn migra_version_uno_sin_perder_preferencia_de_recuperacion() {
        let path = temp_file();
        fs::write(&path, "version=1\nrecovery_enabled=false\n").unwrap();

        let loaded = load_from(&path).unwrap();
        assert!(!loaded.recovery_enabled);
        assert!(loaded.document_modes.is_empty());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn limita_el_historial_de_modos_sin_perder_los_mas_recientes() {
        let mut settings = Settings::default();
        for index in 0..(MAX_DOCUMENT_MODES + 8) {
            settings.remember_document_mode(
                Path::new(&format!("nota-{index}.md")),
                DocumentModePreference::Reading,
            );
        }

        assert_eq!(settings.document_modes.len(), MAX_DOCUMENT_MODES);
        assert_eq!(settings.document_mode(Path::new("nota-0.md")), None);
        assert_eq!(
            settings.document_mode(Path::new("nota-135.md")),
            Some(DocumentModePreference::Reading)
        );
    }
}
