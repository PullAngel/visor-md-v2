//! Índice regenerable y acotado para una carpeta autorizada.
//!
//! No persiste contenido, no modifica la bóveda y no sigue rutas que escapen
//! de `WorkspaceRoot`. La interfaz podrá ejecutarlo fuera del hilo de ventana
//! y descartar su resultado al cambiar de workspace.

use crate::files::open_explicit_primary;
use crate::vfs::WorkspaceRoot;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const DEFAULT_MAX_WORKSPACE_FILES: usize = 10_000;
pub(crate) const DEFAULT_MAX_WORKSPACE_NOTE_BYTES: u64 = 512 * 1024;
pub(crate) const DEFAULT_MAX_WORKSPACE_SCAN_BYTES: u64 = 64 * 1024 * 1024;
/// Presupuesto global de texto retenido para búsqueda. El índice nunca se
/// persiste y debe poder descartarse sin dejar una copia completa de la bóveda.
pub(crate) const DEFAULT_MAX_INDEXED_CONTENT_BYTES: usize = 8 * 1024 * 1024;
const MAX_INDEXED_BYTES_PER_NOTE: usize = 8 * 1024;

#[derive(Clone, Copy, Debug)]
pub(crate) struct WorkspaceLimits {
    pub(crate) max_files: usize,
    pub(crate) max_note_bytes: u64,
    pub(crate) max_scanned_bytes: u64,
    pub(crate) max_indexed_content_bytes: usize,
}

impl Default for WorkspaceLimits {
    fn default() -> Self {
        Self {
            max_files: DEFAULT_MAX_WORKSPACE_FILES,
            max_note_bytes: DEFAULT_MAX_WORKSPACE_NOTE_BYTES,
            max_scanned_bytes: DEFAULT_MAX_WORKSPACE_SCAN_BYTES,
            max_indexed_content_bytes: DEFAULT_MAX_INDEXED_CONTENT_BYTES,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Heading {
    pub(crate) text: String,
    pub(crate) level: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WikiLink {
    /// Destino de nota sin alias ni ancla.
    pub(crate) note: String,
    pub(crate) alias: Option<String>,
    pub(crate) heading: Option<String>,
}

#[derive(Clone, Debug)]
// La UI usa este índice para búsqueda y backlinks por teclado. El árbol y los
// paneles plegables todavía son una capa posterior sobre el mismo modelo.
pub(crate) struct WorkspaceNote {
    pub(crate) relative_path: PathBuf,
    pub(crate) title: String,
    pub(crate) headings: Vec<Heading>,
    pub(crate) wikilinks: Vec<WikiLink>,
    /// Fragmento acotado y en memoria de la fuente. Sirve únicamente para
    /// búsqueda local; no se escribe, envía ni interpreta como código.
    search_text: String,
}

/// Resultado explícito de resolver un wikilink. No se elige una coincidencia
/// por nombre de archivo cuando la bóveda contiene más de una alternativa.
pub(crate) enum WikiResolution<'a> {
    Found(&'a WorkspaceNote),
    Missing,
    Ambiguous,
}

#[derive(Debug, Default)]
pub(crate) struct WorkspaceIndex {
    pub(crate) notes: Vec<WorkspaceNote>,
    pub(crate) skipped: usize,
    pub(crate) truncated: bool,
    pub(crate) indexed_content_bytes: usize,
    pub(crate) content_truncated: bool,
    pub(crate) scanned_bytes: u64,
    pub(crate) scan_truncated: bool,
    /// El recorrido se interrumpió por una acción posterior de la persona.
    /// Un resultado cancelado nunca debe sustituir el índice activo.
    pub(crate) cancelled: bool,
}

impl WorkspaceIndex {
    /// Devuelve únicamente rutas relativas que ya pasaron por el recorrido
    /// contenido. La interfaz debe resolverlas otra vez antes de abrirlas.
    pub(crate) fn note_paths(&self) -> Vec<PathBuf> {
        self.notes
            .iter()
            .map(|note| note.relative_path.clone())
            .collect()
    }

    /// Busca una nota por su ruta ya relativa a la raíz autorizada. La UI usa
    /// esto para asociar el documento abierto con los backlinks del índice sin
    /// volver a recorrer el disco ni aceptar rutas absolutas desde contenido.
    pub(crate) fn note_at_relative(&self, relative_path: &Path) -> Option<&WorkspaceNote> {
        self.notes
            .iter()
            .find(|note| note.relative_path == relative_path)
    }

    /// Resuelve la parte de nota de un wikilink sin tocar el filesystem. El
    /// resultado siempre proviene del índice ya contenido por la VFS.
    pub(crate) fn resolve_wikilink(&self, target: &str) -> WikiResolution<'_> {
        let target_declares_path =
            target.contains(['/', '\\']) || target.trim().to_ascii_lowercase().ends_with(".md");
        let target = normalized_note_key(target);
        if target.is_empty() {
            return WikiResolution::Missing;
        }
        if target_declares_path
            && let Some(note) = self.notes.iter().find(|note| {
                let relative = normalized_note_key(&note.relative_path.to_string_lossy());
                relative == target
            })
        {
            return WikiResolution::Found(note);
        }

        let mut by_stem = self.notes.iter().filter(|note| {
            note.relative_path
                .file_stem()
                .and_then(|name| name.to_str())
                .map(normalized_note_key)
                .as_deref()
                == Some(target.as_str())
        });
        match (by_stem.next(), by_stem.next()) {
            (Some(note), None) => WikiResolution::Found(note),
            (Some(_), Some(_)) => WikiResolution::Ambiguous,
            (None, _) => WikiResolution::Missing,
        }
    }

    /// Calcula backlinks hacia una nota concreta usando la misma resolución
    /// conservadora que la navegación. Un wikilink ambiguo no se atribuye a
    /// ninguna de sus posibles notas.
    pub(crate) fn backlinks_to(&self, target: &WorkspaceNote) -> Vec<&WorkspaceNote> {
        self.notes
            .iter()
            .filter(|note| {
                note.wikilinks.iter().any(|link| {
                    matches!(
                        self.resolve_wikilink(&link.note),
                        WikiResolution::Found(resolved)
                            if resolved.relative_path == target.relative_path
                    )
                })
            })
            .collect()
    }

    pub(crate) fn search(&self, query: &str) -> Vec<&WorkspaceNote> {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }
        self.notes
            .iter()
            .filter(|note| {
                note.title.to_lowercase().contains(&query)
                    || note
                        .relative_path
                        .to_string_lossy()
                        .to_lowercase()
                        .contains(&query)
                    || note
                        .headings
                        .iter()
                        .any(|heading| heading.text.to_lowercase().contains(&query))
                    || note.search_text.to_lowercase().contains(&query)
            })
            .collect()
    }
}

/// Recorre únicamente rutas ya contenidas. `.git` y `.obsidian` son metadatos
/// de otras herramientas: no se indexan para evitar ruido, secretos de plugin
/// y un falso efecto de compatibilidad al interpretar sus configuraciones.
#[cfg(test)]
pub(crate) fn index_workspace(root: &WorkspaceRoot, limits: WorkspaceLimits) -> WorkspaceIndex {
    index_workspace_cancellable(root, limits, &AtomicBool::new(false))
}

/// Variante cooperativa del indexado. La señal solo se comprueba entre
/// entradas, por lo que no hace falta matar hilos ni publicar estado parcial.
pub(crate) fn index_workspace_cancellable(
    root: &WorkspaceRoot,
    limits: WorkspaceLimits,
    cancelled: &AtomicBool,
) -> WorkspaceIndex {
    let mut index = WorkspaceIndex::default();
    let mut pending = vec![root.root().to_path_buf()];
    let mut visited_directories = HashSet::new();

    while let Some(directory) = pending.pop() {
        if cancelled.load(Ordering::Relaxed) {
            index.cancelled = true;
            return index;
        }
        if !visited_directories.insert(directory.clone()) {
            continue;
        }
        let Ok(entries) = fs::read_dir(&directory) else {
            index.skipped += 1;
            continue;
        };
        for entry in entries.flatten() {
            if cancelled.load(Ordering::Relaxed) {
                index.cancelled = true;
                return index;
            }
            let name = entry.file_name();
            if name == ".git" || name == ".obsidian" {
                continue;
            }
            let Ok(canonical) = fs::canonicalize(entry.path()) else {
                index.skipped += 1;
                continue;
            };
            if !canonical.starts_with(root.root()) {
                index.skipped += 1;
                continue;
            }
            let Ok(file_type) = entry.file_type() else {
                index.skipped += 1;
                continue;
            };
            if file_type.is_dir() {
                pending.push(canonical);
                continue;
            }
            if !file_type.is_file() || !is_markdown_path(&canonical) {
                continue;
            }
            if index.notes.len() >= limits.max_files {
                index.truncated = true;
                return index;
            }
            let Ok(metadata) = entry.metadata() else {
                index.skipped += 1;
                continue;
            };
            if metadata.len() > limits.max_note_bytes {
                index.skipped += 1;
                continue;
            }
            if index.scanned_bytes.saturating_add(metadata.len()) > limits.max_scanned_bytes {
                index.scan_truncated = true;
                return index;
            }
            index.scanned_bytes = index.scanned_bytes.saturating_add(metadata.len());
            match open_explicit_primary(&canonical, limits.max_note_bytes) {
                Ok(opened) => {
                    let relative_path = canonical
                        .strip_prefix(root.root())
                        .expect("la contención se comprobó antes")
                        .to_path_buf();
                    let available = limits
                        .max_indexed_content_bytes
                        .saturating_sub(index.indexed_content_bytes);
                    let (note, indexed_bytes, content_truncated) =
                        note_from_source(relative_path, &opened.source, available);
                    index.indexed_content_bytes =
                        index.indexed_content_bytes.saturating_add(indexed_bytes);
                    index.content_truncated |= content_truncated;
                    index.notes.push(note);
                }
                Err(_) => index.skipped += 1,
            }
        }
    }
    index
        .notes
        .sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    index
}

fn is_markdown_path(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("md" | "markdown" | "mdown" | "mkdn")
    )
}

fn note_from_source(
    relative_path: PathBuf,
    source: &str,
    available_content_bytes: usize,
) -> (WorkspaceNote, usize, bool) {
    let headings = headings_in(source);
    let title = headings
        .first()
        .map(|heading| heading.text.clone())
        .unwrap_or_else(|| {
            relative_path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("nota")
                .to_owned()
        });
    let keep = available_content_bytes
        .min(MAX_INDEXED_BYTES_PER_NOTE)
        .min(source.len());
    let mut end = keep;
    while end > 0 && !source.is_char_boundary(end) {
        end -= 1;
    }
    let search_text = source[..end].to_owned();
    let content_truncated = end < source.len();
    (
        WorkspaceNote {
            relative_path,
            title,
            headings,
            wikilinks: wikilinks_in(source),
            search_text,
        },
        end,
        content_truncated,
    )
}

fn headings_in(source: &str) -> Vec<Heading> {
    source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let level = trimmed.bytes().take_while(|byte| *byte == b'#').count();
            (1..=6)
                .contains(&level)
                .then_some(())
                .and_then(|()| trimmed.get(level..))
                .map(str::trim_start)
                .filter(|text| !text.is_empty())
                .map(|text| Heading {
                    text: text.trim_end_matches('#').trim().to_owned(),
                    level: level as u8,
                })
        })
        .collect()
}

fn wikilinks_in(source: &str) -> Vec<WikiLink> {
    let mut links = Vec::new();
    let mut rest = source;
    while let Some(start) = rest.find("[[") {
        let after_open = &rest[start + 2..];
        let Some(end) = after_open.find("]]") else {
            break;
        };
        let raw = after_open[..end].trim();
        if !raw.is_empty() {
            let (target, alias) = raw.split_once('|').map_or((raw, None), |(target, alias)| {
                (target.trim(), Some(alias.trim().to_owned()))
            });
            let (note, heading) = target
                .split_once('#')
                .map_or((target.trim().to_owned(), None), |(note, heading)| {
                    (note.trim().to_owned(), Some(heading.trim().to_owned()))
                });
            if !note.is_empty() {
                links.push(WikiLink {
                    note,
                    alias: alias.filter(|alias| !alias.is_empty()),
                    heading: heading.filter(|heading| !heading.is_empty()),
                });
            }
        }
        rest = &after_open[end + 2..];
    }
    links
}

fn normalized_note_key(note: &str) -> String {
    let normalized = note
        .split_once('#')
        .map_or(note, |(note, _)| note)
        .trim()
        .replace('\\', "/")
        .to_lowercase();
    normalized.trim_end_matches(".md").to_owned()
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
        let root = std::env::temp_dir().join(format!("visor-md-workspace-{nonce}"));
        fs::create_dir_all(root.join("clases")).expect("se crea la fixture");
        fs::create_dir_all(root.join(".obsidian")).expect("se crea metadato ajeno");
        fs::write(
            root.join("clases").join("redes.md"),
            "# Redes\n\nVer [[seguridad|la guía]] y [[seguridad#Modelo]].\nClave: ciberseguridad interna.",
        )
        .expect("se crea la nota");
        fs::write(root.join("seguridad.md"), "# Seguridad\n## Modelo").expect("se crea la nota");
        fs::write(root.join(".obsidian").join("plugin.md"), "# No indexar")
            .expect("se crea configuración");
        root
    }

    #[test]
    fn indexa_notas_y_extrae_titulos_enlaces_y_backlinks() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        let index = index_workspace(&vfs, WorkspaceLimits::default());
        assert_eq!(index.notes.len(), 2);
        assert_eq!(index.notes[0].title, "Redes");
        assert_eq!(index.notes[0].wikilinks.len(), 2);
        assert_eq!(
            index.notes[0].wikilinks[0].alias.as_deref(),
            Some("la guía")
        );
        let seguridad = index
            .notes
            .iter()
            .find(|note| note.title == "Seguridad")
            .expect("la nota se indexó");
        assert_eq!(index.backlinks_to(seguridad).len(), 1);
        assert!(index.note_at_relative(Path::new("seguridad.md")).is_some());
        assert_eq!(index.search("modelo").len(), 2);
        assert_eq!(index.search("ciberseguridad interna").len(), 1);
        assert!(matches!(
            index.resolve_wikilink("seguridad#Modelo"),
            WikiResolution::Found(note) if note.title == "Seguridad"
        ));
        assert_eq!(
            index.note_paths(),
            vec![
                PathBuf::from("clases/redes.md"),
                PathBuf::from("seguridad.md")
            ]
        );
        assert!(matches!(
            index.resolve_wikilink("clases/redes"),
            WikiResolution::Found(note) if note.title == "Redes"
        ));
    }

    #[test]
    fn respeta_el_limite_y_no_interpreta_configuracion_de_obsidian() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        let index = index_workspace(
            &vfs,
            WorkspaceLimits {
                max_files: 1,
                ..WorkspaceLimits::default()
            },
        );
        assert_eq!(index.notes.len(), 1);
        assert!(index.truncated);
        assert!(
            index
                .notes
                .iter()
                .all(|note| !note.relative_path.starts_with(".obsidian"))
        );
    }

    #[test]
    fn cancelar_el_indice_no_publica_un_resultado_completo() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        let cancelled = AtomicBool::new(true);
        let index = index_workspace_cancellable(&vfs, WorkspaceLimits::default(), &cancelled);

        assert!(index.cancelled);
        assert!(index.notes.is_empty());
    }

    #[test]
    fn limita_el_texto_retenido_para_busqueda_sin_partir_unicode() {
        let (note, bytes, truncated) =
            note_from_source(PathBuf::from("nota.md"), "áéí contenido importante", 3);

        assert_eq!(note.search_text, "á");
        assert_eq!(bytes, "á".len());
        assert!(truncated);
    }

    #[test]
    fn limita_la_lectura_de_notas_y_de_la_carpeta() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        let small_note_limit = index_workspace(
            &vfs,
            WorkspaceLimits {
                max_note_bytes: 1,
                ..WorkspaceLimits::default()
            },
        );
        assert!(small_note_limit.notes.is_empty());
        assert!(small_note_limit.skipped >= 2);

        let small_scan_limit = index_workspace(
            &vfs,
            WorkspaceLimits {
                max_scanned_bytes: 1,
                ..WorkspaceLimits::default()
            },
        );
        assert!(small_scan_limit.scan_truncated);
        assert!(small_scan_limit.notes.is_empty());
    }

    #[test]
    fn wikilinks_defectuosos_no_crean_destinos_vacios() {
        assert_eq!(wikilinks_in("[[ ]] [[abierta"), Vec::<WikiLink>::new());
    }

    #[test]
    fn no_elige_un_wikilink_ambiguo_por_orden_del_indice() {
        let root = fixture_root();
        fs::create_dir_all(root.join("archivo")).expect("se crea la carpeta");
        fs::write(root.join("archivo").join("seguridad.md"), "# Archivo")
            .expect("se crea la nota duplicada");
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        let index = index_workspace(&vfs, WorkspaceLimits::default());

        assert!(matches!(
            index.resolve_wikilink("seguridad"),
            WikiResolution::Ambiguous
        ));
        assert!(matches!(
            index.resolve_wikilink("archivo/seguridad"),
            WikiResolution::Found(note) if note.title == "Archivo"
        ));
        let raiz = index
            .notes
            .iter()
            .find(|note| note.title == "Seguridad")
            .expect("la nota raíz se indexó");
        assert!(index.backlinks_to(raiz).is_empty());
    }
}
