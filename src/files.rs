//! Frontera mínima de archivos para el documento principal.
//!
//! No es un disco virtual: concentra la política antes de que el parser vea
//! bytes. Recursos secundarios seguirán una política más estricta cuando haya
//! navegación de enlaces, imágenes o bóvedas.

use atomicwrites::{AllowOverwrite, AtomicFile, DisallowOverwrite};
use std::fmt;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::time::SystemTime;

/// Límite normal mientras no exista una preferencia avanzada persistente.
/// Supera holgadamente el corpus de 5 MiB medido, pero evita reservar memoria
/// sin cota al abrir por doble clic un archivo hostil.
pub(crate) const DEFAULT_DOCUMENT_LIMIT_BYTES: u64 = 16 * 1024 * 1024;

/// Markdown se interpreta solo cuando su extensión lo declara. El resto de
/// archivos de texto se mantiene visible pero inerte, incluso si parece código.
pub(crate) fn is_markdown_path(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("md" | "markdown" | "mdown" | "mkdn")
    )
}

/// Metadatos de codificación que no pertenecen al contenido editable. El
/// editor los conserva para no cambiar un archivo solo por haberlo abierto.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct TextMetadata {
    pub(crate) has_utf8_bom: bool,
    pub(crate) line_endings: LineEndings,
}

/// Huella portátil de la versión abierta. No sustituye una identidad de handle
/// para reemplazo atómico, pero permite detectar el caso común de un archivo
/// que otra aplicación cambió antes de intentar guardarlo.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FileIdentity {
    pub(crate) byte_len: u64,
    modified: Option<SystemTime>,
}

impl FileIdentity {
    fn from_metadata(metadata: &std::fs::Metadata) -> Self {
        Self {
            byte_len: metadata.len(),
            modified: metadata.modified().ok(),
        }
    }

    fn still_matches(&self, path: impl AsRef<Path>) -> Result<bool, std::io::Error> {
        let metadata = std::fs::metadata(path)?;
        Ok(self == &Self::from_metadata(&metadata))
    }
}

/// Forma observada de los saltos de línea de la fuente. `Mixed` no es un error:
/// se informa para que una edición futura no normalice el documento en silencio.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum LineEndings {
    #[default]
    None,
    Lf,
    CrLf,
    Mixed,
}

impl TextMetadata {
    fn from_source(source: &str, has_utf8_bom: bool) -> Self {
        let mut saw_lf = false;
        let mut saw_crlf = false;
        let bytes = source.as_bytes();
        let mut index = 0;
        while index < bytes.len() {
            match bytes[index] {
                b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                    saw_crlf = true;
                    index += 2;
                }
                b'\n' => {
                    saw_lf = true;
                    index += 1;
                }
                _ => index += 1,
            }
        }
        let line_endings = match (saw_lf, saw_crlf) {
            (false, false) => LineEndings::None,
            (true, false) => LineEndings::Lf,
            (false, true) => LineEndings::CrLf,
            (true, true) => LineEndings::Mixed,
        };
        Self {
            has_utf8_bom,
            line_endings,
        }
    }

    /// Reconstruye exactamente la codificación UTF-8 que Visor MD admite.
    /// Los EOL pertenecen a `source`: el editor inserta el estilo observado,
    /// pero no normaliza las líneas ya existentes ni las mixtas.
    pub(crate) fn encode(self, source: &str) -> Vec<u8> {
        let prefix: &[u8] = if self.has_utf8_bom {
            &[0xef, 0xbb, 0xbf]
        } else {
            &[]
        };
        let mut bytes = Vec::with_capacity(prefix.len() + source.len());
        bytes.extend_from_slice(prefix);
        bytes.extend_from_slice(source.as_bytes());
        bytes
    }
}

#[derive(Debug)]
pub(crate) struct OpenedText {
    pub(crate) source: String,
    pub(crate) metadata: TextMetadata,
    pub(crate) identity: FileIdentity,
    /// Bytes exactos que se leyeron del archivo. Se conservan solo hasta el
    /// siguiente guardado para detectar una edición externa antes de reemplazar
    /// el destino; no se escriben como sidecar ni se envían a ningún servicio.
    pub(crate) baseline_bytes: Vec<u8>,
}

/// Resultado de una escritura explícita. El llamador reemplaza su baseline
/// recién después de recibir este valor, nunca al iniciar Guardar.
#[derive(Debug)]
pub(crate) struct SavedText {
    pub(crate) identity: FileIdentity,
    pub(crate) baseline_bytes: Vec<u8>,
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

#[derive(Debug)]
pub(crate) enum FileSaveError {
    Conflict,
    NotAFile,
    Io {
        operation: &'static str,
        source: std::io::Error,
    },
}

impl fmt::Display for FileSaveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conflict => formatter
                .write_str("el archivo cambió fuera de Visor MD; no se sobrescribió ningún dato"),
            Self::NotAFile => formatter.write_str("el destino no es un archivo normal"),
            Self::Io { operation, source } => write!(formatter, "{operation}: {source}"),
        }
    }
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
    let identity = FileIdentity::from_metadata(&metadata);
    let declared_len = identity.byte_len;
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
    Read::by_ref(&mut file)
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
    let has_utf8_bom = bytes.starts_with(&[0xef, 0xbb, 0xbf]);
    let source_bytes = if has_utf8_bom { &bytes[3..] } else { &bytes };
    let source =
        String::from_utf8(source_bytes.to_vec()).map_err(|_| FileOpenError::InvalidUtf8)?;
    let metadata = TextMetadata::from_source(&source, has_utf8_bom);
    Ok(OpenedText {
        source,
        metadata,
        identity,
        baseline_bytes: bytes,
    })
}

/// Guarda una fuente UTF-8 con un reemplazo atómico en el mismo filesystem.
///
/// Antes de escribir compara los bytes actuales con la versión que se abrió o
/// guardó por última vez. Esta comparación completa complementa la huella de
/// metadatos, que por sí sola puede no detectar dos cambios de igual tamaño y
/// fecha. Ante cualquier diferencia no se toca el destino.
pub(crate) fn save_explicit_primary(
    path: impl AsRef<Path>,
    source: &str,
    metadata: TextMetadata,
    expected_identity: &FileIdentity,
    baseline_bytes: &[u8],
) -> Result<SavedText, FileSaveError> {
    let path = path.as_ref();
    let existing_metadata = fs::metadata(path).map_err(|source| FileSaveError::Io {
        operation: "no se pudo consultar el destino antes de guardar",
        source,
    })?;
    if !existing_metadata.is_file() {
        return Err(FileSaveError::NotAFile);
    }
    let current_identity = FileIdentity::from_metadata(&existing_metadata);
    let current_bytes = fs::read(path).map_err(|source| FileSaveError::Io {
        operation: "no se pudo comprobar el contenido actual antes de guardar",
        source,
    })?;
    if &current_identity != expected_identity || current_bytes != baseline_bytes {
        return Err(FileSaveError::Conflict);
    }

    let output = metadata.encode(source);
    let atomic = AtomicFile::new(path, AllowOverwrite);
    atomic
        .write(|file| {
            file.write_all(&output)?;
            file.sync_all()
        })
        .map_err(|error| FileSaveError::Io {
            operation: "no se pudo reemplazar el archivo de forma atómica",
            source: error.into(),
        })?;

    let saved_metadata = fs::metadata(path).map_err(|source| FileSaveError::Io {
        operation: "el archivo se guardó pero no se pudo volver a identificar",
        source,
    })?;
    Ok(SavedText {
        identity: FileIdentity::from_metadata(&saved_metadata),
        baseline_bytes: output,
    })
}

/// Crea un documento en una ruta que la persona eligió explícitamente. No
/// reemplaza un archivo existente: Guardar como no puede convertirse en una
/// vía silenciosa para destruir un destino que todavía no fue abierto.
pub(crate) fn save_new_primary(
    path: impl AsRef<Path>,
    source: &str,
    metadata: TextMetadata,
) -> Result<SavedText, FileSaveError> {
    let path = path.as_ref();
    if fs::symlink_metadata(path).is_ok() {
        return Err(FileSaveError::Conflict);
    }
    let output = metadata.encode(source);
    AtomicFile::new(path, DisallowOverwrite)
        .write(|file| {
            file.write_all(&output)?;
            file.sync_all()
        })
        .map_err(|error| FileSaveError::Io {
            operation: "no se pudo crear el archivo de forma atómica",
            source: error.into(),
        })?;
    let saved_metadata = fs::metadata(path).map_err(|source| FileSaveError::Io {
        operation: "el archivo se creó pero no se pudo volver a identificar",
        source,
    })?;
    Ok(SavedText {
        identity: FileIdentity::from_metadata(&saved_metadata),
        baseline_bytes: output,
    })
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
        assert_eq!(opened.metadata.line_endings, LineEndings::Lf);
        assert!(!opened.metadata.has_utf8_bom);
        assert!(opened.identity.still_matches(&path).unwrap());
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

    #[test]
    fn solo_las_extensiones_markdown_habilitan_el_parser() {
        assert!(is_markdown_path(Path::new("nota.MD")));
        assert!(is_markdown_path(Path::new("nota.markdown")));
        for path in ["datos.json", "config.toml", "script.rs", "sin-extension"] {
            assert!(!is_markdown_path(Path::new(path)), "{path} debe ser inerte");
        }
    }

    #[test]
    fn bom_utf8_y_crlf_se_conservan_sin_aparecer_como_contenido() {
        let original = b"\xef\xbb\xbf# nota\r\nsegunda linea\r\n";
        let path = temporary_file("bom-crlf", original);
        let opened = open_explicit_primary(&path, 128).expect("la fixture es válida");

        assert_eq!(opened.source, "# nota\r\nsegunda linea\r\n");
        assert_eq!(opened.metadata.line_endings, LineEndings::CrLf);
        assert!(opened.metadata.has_utf8_bom);
        assert_eq!(opened.metadata.encode(&opened.source), original);
    }

    #[test]
    fn eol_mixtos_se_informan_y_sus_bytes_no_se_normalizan() {
        let original = b"uno\ndos\r\ntres\n";
        let path = temporary_file("mixed-eol", original);
        let opened = open_explicit_primary(&path, 128).expect("la fixture es válida");

        assert_eq!(opened.metadata.line_endings, LineEndings::Mixed);
        assert_eq!(opened.metadata.encode(&opened.source), original);
    }

    #[test]
    fn la_huella_detecta_cambio_externo_de_tamano() {
        let path = temporary_file("identity", b"uno");
        let opened = open_explicit_primary(&path, 64).expect("la fixture es válida");
        fs::write(&path, b"uno cambiado").expect("la fixture se puede modificar");

        assert!(!opened.identity.still_matches(&path).unwrap());
    }

    #[test]
    fn guardar_preserva_bom_crlf_y_actualiza_la_version_base() {
        let path = temporary_file("save-bom-crlf", b"\xef\xbb\xbfuno\r\ndos\r\n");
        let opened = open_explicit_primary(&path, 128).expect("la fixture es válida");
        let saved = save_explicit_primary(
            &path,
            "uno\r\ntres\r\n",
            opened.metadata,
            &opened.identity,
            &opened.baseline_bytes,
        )
        .expect("el guardado atómico funciona");

        assert_eq!(fs::read(&path).unwrap(), b"\xef\xbb\xbfuno\r\ntres\r\n");
        assert_eq!(saved.baseline_bytes, b"\xef\xbb\xbfuno\r\ntres\r\n");
        assert!(saved.identity.still_matches(&path).unwrap());
    }

    #[test]
    fn un_cambio_externo_no_se_sobrescribe_aunque_mantenga_el_tamano() {
        let path = temporary_file("save-conflict", b"uno");
        let opened = open_explicit_primary(&path, 64).expect("la fixture es válida");
        fs::write(&path, b"dos").expect("la fixture se puede modificar");

        assert!(matches!(
            save_explicit_primary(
                &path,
                "tres",
                opened.metadata,
                &opened.identity,
                &opened.baseline_bytes,
            ),
            Err(FileSaveError::Conflict)
        ));
        assert_eq!(fs::read(&path).unwrap(), b"dos");
    }

    #[test]
    fn guardar_como_crea_un_destino_nuevo_sin_reemplazar_existentes() {
        let path = temporary_file("save-new", b"existente");
        assert!(matches!(
            save_new_primary(&path, "nuevo", TextMetadata::default()),
            Err(FileSaveError::Conflict)
        ));
        assert_eq!(fs::read(&path).unwrap(), b"existente");
    }
}
