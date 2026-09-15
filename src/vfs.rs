//! Puerta de acceso a archivos secundarios dentro de un workspace explícito.
//!
//! El contenido Markdown nunca llama a `std::fs` directamente. Primero entrega
//! una ruta relativa a esta capa, que rechaza escapes léxicos, inspecciona
//! reparse points secundarios antes de resolver y comprueba la contención final.

use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug)]
pub(crate) struct WorkspaceRoot {
    canonical_root: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum VfsError {
    RootUnavailable,
    NotDirectory,
    EmptyPath,
    AbsoluteOrPrefixedPath,
    ParentTraversal,
    AlternateDataStream,
    Missing,
    OutsideRoot,
    ReparsePoint,
    Io(std::io::ErrorKind),
}

impl fmt::Display for VfsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::RootUnavailable => "no se pudo abrir la carpeta elegida",
            Self::NotDirectory => "la ruta elegida no es una carpeta",
            Self::EmptyPath => "la referencia no declara una ruta utilizable",
            Self::AbsoluteOrPrefixedPath => {
                "las referencias secundarias no pueden ser absolutas ni UNC"
            }
            Self::ParentTraversal => "la referencia intenta salir de la carpeta autorizada",
            Self::AlternateDataStream => {
                "la referencia contiene un stream alternativo no permitido"
            }
            Self::Missing => "el archivo referido no existe",
            Self::OutsideRoot => "la referencia termina fuera de la carpeta autorizada",
            Self::ReparsePoint => {
                "la referencia atraviesa un enlace simbólico o junction no permitido"
            }
            Self::Io(_) => "no se pudo resolver el archivo referido",
        };
        formatter.write_str(text)
    }
}

impl WorkspaceRoot {
    /// La carpeta es una capacidad concedida por una acción explícita de la
    /// persona. Puede ser UNC si fue elegida así, pero sus referencias internas
    /// continúan limitadas a esta misma raíz canonicalizada.
    pub(crate) fn open(path: impl AsRef<Path>) -> Result<Self, VfsError> {
        let canonical_root = fs::canonicalize(path).map_err(|_| VfsError::RootUnavailable)?;
        if !canonical_root.is_dir() {
            return Err(VfsError::NotDirectory);
        }
        Ok(Self { canonical_root })
    }

    pub(crate) fn root(&self) -> &Path {
        &self.canonical_root
    }

    /// Resuelve solo un archivo existente. Primero inspecciona cada componente
    /// secundario sin seguirlo; solo después canonicaliza y confirma contención.
    pub(crate) fn resolve_existing(
        &self,
        reference: impl AsRef<Path>,
    ) -> Result<PathBuf, VfsError> {
        let reference = reference.as_ref();
        validate_relative_reference(reference)?;
        let candidate = self.canonical_root.join(reference);
        self.reject_secondary_reparse_points(&candidate)?;
        let resolved = fs::canonicalize(candidate).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                VfsError::Missing
            } else {
                VfsError::Io(error.kind())
            }
        })?;
        if !resolved.starts_with(&self.canonical_root) {
            return Err(VfsError::OutsideRoot);
        }
        Ok(resolved)
    }

    /// Resuelve una referencia como lo hace Markdown: relativa a la carpeta
    /// del documento que la contiene. El documento base también debe estar
    /// dentro de la capacidad concedida; abrir otro archivo manualmente no le
    /// da acceso lateral a la bóveda seleccionada.
    pub(crate) fn resolve_existing_from(
        &self,
        base_file: impl AsRef<Path>,
        reference: impl AsRef<Path>,
    ) -> Result<PathBuf, VfsError> {
        let reference = reference.as_ref();
        validate_relative_reference(reference)?;
        let base = fs::canonicalize(base_file).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                VfsError::Missing
            } else {
                VfsError::Io(error.kind())
            }
        })?;
        if !base.starts_with(&self.canonical_root) {
            return Err(VfsError::OutsideRoot);
        }
        let parent = base.parent().ok_or(VfsError::OutsideRoot)?;
        let candidate = parent.join(reference);
        self.reject_secondary_reparse_points(&candidate)?;
        let resolved = fs::canonicalize(candidate).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                VfsError::Missing
            } else {
                VfsError::Io(error.kind())
            }
        })?;
        if !resolved.starts_with(&self.canonical_root) {
            return Err(VfsError::OutsideRoot);
        }
        Ok(resolved)
    }

    /// Una referencia secundaria declarada por un documento no puede hacer
    /// que el proceso siga symlinks o junctions, incluso si por casualidad
    /// terminan dentro de la raíz. Se inspecciona cada componente antes de
    /// canonicalizar, que en Windows podría consultar una ubicación de red.
    fn reject_secondary_reparse_points(&self, candidate: &Path) -> Result<(), VfsError> {
        let relative = candidate
            .strip_prefix(&self.canonical_root)
            .map_err(|_| VfsError::OutsideRoot)?;
        let mut current = self.canonical_root.clone();
        for component in relative.components() {
            let Component::Normal(name) = component else {
                return Err(VfsError::OutsideRoot);
            };
            current.push(name);
            let metadata = fs::symlink_metadata(&current).map_err(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    VfsError::Missing
                } else {
                    VfsError::Io(error.kind())
                }
            })?;
            if is_reparse_point(&metadata) {
                return Err(VfsError::ReparsePoint);
            }
        }
        Ok(())
    }
}

/// `symlink_metadata` no sigue el último componente. En Windows, una junction
/// no siempre se anuncia como symlink, por lo que también se consulta el bit
/// de reparse point. La raíz elegida explícitamente queda fuera de esta regla;
/// solo se aplica a componentes que proceden de una referencia secundaria.
pub(crate) fn is_reparse_point(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    {
        false
    }
}

fn validate_relative_reference(reference: &Path) -> Result<(), VfsError> {
    if reference.as_os_str().is_empty() {
        return Err(VfsError::EmptyPath);
    }
    for component in reference.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                return Err(VfsError::AbsoluteOrPrefixedPath);
            }
            Component::ParentDir => return Err(VfsError::ParentTraversal),
            Component::Normal(segment) if segment.to_string_lossy().contains(':') => {
                return Err(VfsError::AlternateDataStream);
            }
            Component::CurDir | Component::Normal(_) => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fixture_root() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("el reloj es válido")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("visor-md-vfs-{nonce}"));
        fs::create_dir_all(root.join("apuntes")).expect("se crea la fixture");
        fs::write(root.join("apuntes").join("redes.md"), "# redes").expect("se crea el documento");
        root
    }

    #[test]
    fn resuelve_un_archivo_existente_dentro_de_la_raiz() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        let resolved = vfs
            .resolve_existing(Path::new("apuntes/redes.md"))
            .expect("la referencia está contenida");
        assert!(resolved.starts_with(vfs.root()));
        assert!(resolved.ends_with("redes.md"));
    }

    #[test]
    fn rechaza_escapes_antes_de_tocar_el_filesystem() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        assert_eq!(
            vfs.resolve_existing(Path::new("../secreto.md")),
            Err(VfsError::ParentTraversal)
        );
        assert!(matches!(
            vfs.resolve_existing(Path::new(r"C:\secreto.md")),
            Err(VfsError::AbsoluteOrPrefixedPath)
        ));
        assert_eq!(
            vfs.resolve_existing(Path::new("apuntes/redes.md:oculto")),
            Err(VfsError::AlternateDataStream)
        );
    }

    #[test]
    fn informa_referencia_ausente_sin_probar_otra_ruta() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        assert_eq!(
            vfs.resolve_existing(Path::new("apuntes/ausente.md")),
            Err(VfsError::Missing)
        );
    }

    #[test]
    fn resuelve_desde_la_carpeta_del_documento_no_desde_la_raiz() {
        let root = fixture_root();
        fs::create_dir_all(root.join("apuntes").join("media")).unwrap();
        fs::write(
            root.join("apuntes").join("media").join("diagrama.png"),
            b"png",
        )
        .unwrap();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");

        let resolved = vfs
            .resolve_existing_from(
                root.join("apuntes").join("redes.md"),
                Path::new("media/diagrama.png"),
            )
            .expect("la referencia usa la carpeta de la nota");

        assert_eq!(
            resolved,
            fs::canonicalize(root.join("apuntes").join("media").join("diagrama.png")).unwrap()
        );
    }

    #[cfg(unix)]
    #[test]
    fn una_referencia_secundaria_no_sigue_symlinks_antes_de_canonicalizar() {
        use std::os::unix::fs::symlink;

        let root = fixture_root();
        let outside = std::env::temp_dir().join(format!(
            "visor-md-vfs-outside-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("el reloj es válido")
                .as_nanos()
        ));
        fs::create_dir_all(&outside).expect("se crea destino externo");
        fs::write(outside.join("secreto.md"), "# secreto").expect("se crea archivo externo");
        symlink(&outside, root.join("apuntes").join("atajo")).expect("se crea symlink de fixture");
        symlink(
            outside.join("secreto.md"),
            root.join("apuntes").join("atajo-final.md"),
        )
        .expect("se crea symlink final de fixture");
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");

        assert_eq!(
            vfs.resolve_existing(Path::new("apuntes/atajo/secreto.md")),
            Err(VfsError::ReparsePoint)
        );
        assert_eq!(
            vfs.resolve_existing(Path::new("apuntes/atajo-final.md")),
            Err(VfsError::ReparsePoint)
        );
        assert_eq!(
            vfs.resolve_existing_from(
                root.join("apuntes").join("redes.md"),
                Path::new("atajo/secreto.md"),
            ),
            Err(VfsError::ReparsePoint)
        );
        let _ = fs::remove_file(root.join("apuntes").join("atajo"));
        let _ = fs::remove_file(root.join("apuntes").join("atajo-final.md"));
        let _ = fs::remove_dir_all(outside);
    }

    #[test]
    fn un_documento_fuera_de_la_raiz_no_obtiene_acceso_a_la_boveda() {
        let root = fixture_root();
        let outside = std::env::temp_dir().join(format!(
            "visor-md-outside-{}-{}.md",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::write(&outside, "# fuera").unwrap();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");

        assert_eq!(
            vfs.resolve_existing_from(&outside, Path::new("apuntes/redes.md")),
            Err(VfsError::OutsideRoot)
        );
        let _ = fs::remove_file(outside);
    }
}
