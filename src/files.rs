//! Frontera mínima de archivos para el documento principal.
//!
//! No es un disco virtual: concentra la política antes de que el parser vea
//! bytes. Recursos secundarios seguirán una política más estricta cuando haya
//! navegación de enlaces, imágenes o bóvedas.

use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Límite normal mientras no exista una preferencia avanzada persistente.
/// Supera holgadamente el corpus de 5 MiB medido, pero evita reservar memoria
/// sin cota al abrir por doble clic un archivo hostil.
pub(crate) const DEFAULT_DOCUMENT_LIMIT_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug)]
pub(crate) struct OpenedText {
    pub(crate) source: String,
}

#[derive(Debug)]
pub(crate) enum FileOpenError {
    Io {
        operation: &'static str,
        source: std::io::Error,
    },
    NotAFile,
    TooLarge {
        bytes: u64,
        limit: u64,
    },
    InvalidUtf8,
}

impl fmt::Display for FileOpenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { operation, source } => write!(formatter, "{operation}: {source}"),
            Self::NotAFile => formatter.write_str("la ruta no apunta a un archivo normal"),
            Self::TooLarge { bytes, limit } => write!(
                formatter,
                "el archivo ocupa {bytes} bytes y supera el límite actual de {limit} bytes"
            ),
            Self::InvalidUtf8 => {
                formatter.write_str("el archivo no está codificado como UTF-8 válido")
            }
        }
    }
}

/// Abre un documento elegido como entrada principal. UNC está permitido aquí:
/// abrirla fue una acción exterior explícita. El handle se abre antes de mirar
/// tamaño y se usa para la lectura, de modo que no se valida una ruta y luego
/// se lee un reemplazo distinto por esa misma ruta.
pub(crate) fn open_explicit_primary(
    path: impl AsRef<Path>,
    limit: u64,
) -> Result<OpenedText, FileOpenError> {
    let path = path.as_ref().to_path_buf();
    let mut file = File::open(&path).map_err(|source| FileOpenError::Io {
        operation: "no se pudo abrir el archivo",
        source,
    })?;
    let metadata = file.metadata().map_err(|source| FileOpenError::Io {
        operation: "no se pudo consultar el archivo abierto",
        source,
    })?;
    if !metadata.is_file() {
        return Err(FileOpenError::NotAFile);
    }
    let declared_len = metadata.len();
    if declared_len > limit {
        return Err(FileOpenError::TooLarge {
            bytes: declared_len,
            limit,
        });
    }

    let capacity = usize::try_from(declared_len).unwrap_or(usize::MAX);
    let mut bytes = Vec::with_capacity(capacity);
    // Un archivo puede crecer después de leer sus metadatos. Se lee como máximo
    // un byte adicional para comprobar el límite sin reservar de forma abierta.
    file.by_ref()
        .take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|source| FileOpenError::Io {
            operation: "no se pudo leer el archivo abierto",
            source,
        })?;
    let byte_len = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if byte_len > limit {
        return Err(FileOpenError::TooLarge {
            bytes: byte_len,
            limit,
        });
    }
    let source = String::from_utf8(bytes).map_err(|_| FileOpenError::InvalidUtf8)?;
    Ok(OpenedText { source })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_file(name: &str, bytes: &[u8]) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("el reloj del sistema es posterior a UNIX_EPOCH")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("visor-md-{name}-{nonce}.tmp"));
        fs::write(&path, bytes).expect("se puede preparar una fixture temporal");
        path
    }

    #[test]
    fn el_archivo_principal_se_lee_desde_un_handle_limitado() {
        let path = temporary_file("utf8", b"# nota\n");
        let opened = open_explicit_primary(&path, 64).expect("la fixture es válida");
        assert_eq!(opened.source, "# nota\n");
        assert_eq!(opened.source.len(), 7);
    }

    #[test]
    fn el_limite_de_apertura_se_aplica_antes_del_parser() {
        let path = temporary_file("large", b"12345");
        assert!(matches!(
            open_explicit_primary(path, 4),
            Err(FileOpenError::TooLarge { bytes: 5, limit: 4 })
        ));
    }

    #[test]
    fn utf8_invalido_no_llega_al_modelo() {
        let path = temporary_file("invalid", &[0xff, 0xfe]);
        assert!(matches!(
            open_explicit_primary(path, 64),
            Err(FileOpenError::InvalidUtf8)
        ));
    }
}
