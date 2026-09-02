//! Índice regenerable y acotado para una carpeta autorizada.
//!
//! No persiste contenido, no modifica la bóveda y no sigue rutas que escapen
//! de `WorkspaceRoot`. La interfaz podrá ejecutarlo fuera del hilo de ventana
//! y descartar su resultado al cambiar de workspace.

use crate::files::open_explicit_primary;
use crate::vfs::WorkspaceRoot;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;

pub(crate) const DEFAULT_MAX_WORKSPACE_FILES: usize = 10_000;
pub(crate) const DEFAULT_MAX_WORKSPACE_NOTE_BYTES: u64 = 512 * 1024;
pub(crate) const DEFAULT_MAX_WORKSPACE_SCAN_BYTES: u64 = 64 * 1024 * 1024;
/// Presupuesto global de texto retenido para búsqueda. El índice nunca se
/// persiste y debe poder descartarse sin dejar una copia completa de la bóveda.
pub(crate) const DEFAULT_MAX_INDEXED_CONTENT_BYTES: usize = 8 * 1024 * 1024;
const MAX_INDEXED_BYTES_PER_NOTE: usize = 8 * 1024;
const MAX_CHANGE_MARKS: usize = 1_024;

#[derive(Clone, Debug)]
struct ChangeMark {
    relative_path: PathBuf,
    byte_len: u64,
    modified: Option<SystemTime>,
    directory: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct WorkspaceChangeSnapshot {
    marks: Vec<ChangeMark>,
    pub(crate) truncated: bool,
}

impl WorkspaceChangeSnapshot {
    pub(crate) fn may_have_changed(&self, root: &WorkspaceRoot) -> bool {
        self.marks.iter().any(|mark| {
            let resolved = if mark.relative_path == Path::new(".") {
                Ok(root.root().to_path_buf())
            } else {
                root.resolve_existing(&mark.relative_path)
            };
            let Ok(resolved) = resolved else {
                return true;
            };
            let Ok(metadata) = fs::metadata(resolved) else {
                return true;
            };
            metadata.is_dir() != mark.directory
                || metadata.len() != mark.byte_len
                || metadata.modified().ok() != mark.modified
        })
    }
}

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
// La UI deriva de este índice búsqueda, backlinks y un árbol efímero por
// teclado. El índice no necesita persistir estado visual ni contenido extra.
pub(crate) struct WorkspaceNote {
    pub(crate) relative_path: PathBuf,
    pub(crate) title: String,
    pub(crate) headings: Vec<Heading>,
    pub(crate) wikilinks: Vec<WikiLink>,
    /// Fragmento acotado y en memoria de la fuente. Sirve únicamente para
    /// búsqueda local; no se escribe, envía ni interpreta como código.
    search_text: String,
    /// Evidencia de una lectura ya contenida. Solo se usa para evitar releer
    /// notas intactas durante una actualización explícita.
    source_byte_len: u64,
    source_modified: Option<SystemTime>,
}

/// Resultado explícito de resolver un wikilink. No se elige una coincidencia
/// por nombre de archivo cuando la bóveda contiene más de una alternativa.
pub(crate) enum WikiResolution<'a> {
    Found(&'a WorkspaceNote),
    Missing,
    Ambiguous,
}

#[derive(Clone, Debug, Default)]
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
    change_marks: Vec<ChangeMark>,
    change_marks_truncated: bool,
}

impl WorkspaceIndex {
    pub(crate) fn change_snapshot(&self) -> WorkspaceChangeSnapshot {
        WorkspaceChangeSnapshot {
            marks: self.change_marks.clone(),
            truncated: self.change_marks_truncated,
        }
    }

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
    index_workspace_with_previous(root, None, limits, cancelled)
}

/// Recorre de nuevo una raíz ya concedida para descubrir altas y bajas, pero
/// reutiliza la semántica de notas cuya evidencia de disco sigue igual. Abrir
/// una nota continúa resolviéndola con VFS: el índice no concede acceso.
pub(crate) fn reindex_workspace_cancellable(
    root: &WorkspaceRoot,
    previous: &WorkspaceIndex,
    limits: WorkspaceLimits,
    cancelled: &AtomicBool,
) -> WorkspaceIndex {
    index_workspace_with_previous(root, Some(previous), limits, cancelled)
}

fn index_workspace_with_previous(
    root: &WorkspaceRoot,
    previous: Option<&WorkspaceIndex>,
    limits: WorkspaceLimits,
    cancelled: &AtomicBool,
) -> WorkspaceIndex {
    let mut index = WorkspaceIndex::default();
    let mut pending = vec![root.root().to_path_buf()];
    let mut visited_directories = HashSet::new();
    let previous_notes = previous.map_or_else(HashMap::new, |previous| {
        previous
            .notes
            .iter()
            .map(|note| (note.relative_path.clone(), note))
            .collect::<HashMap<_, _>>()
    });

    while let Some(directory) = pending.pop() {
        if cancelled.load(Ordering::Relaxed) {
            index.cancelled = true;
            return index;
        }
        if !visited_directories.insert(directory.clone()) {
            continue;
        }
        record_change_mark(&mut index, root, &directory, true);
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
            let relative_path = canonical
                .strip_prefix(root.root())
                .expect("la contención se comprobó antes")
                .to_path_buf();
            if let Some(previous) = previous_notes.get(&relative_path)
                && previous.source_byte_len == metadata.len()
                && previous.source_modified == metadata.modified().ok()
            {
                let available = limits
                    .max_indexed_content_bytes
                    .saturating_sub(index.indexed_content_bytes);
                let (note, indexed_bytes, content_truncated) =
                    reused_note_with_content_limit(previous, available);
                index.indexed_content_bytes =
                    index.indexed_content_bytes.saturating_add(indexed_bytes);
                index.content_truncated |= content_truncated;
                index.notes.push(note);
                record_change_mark(&mut index, root, &canonical, false);
                continue;
            }
            match open_explicit_primary(&canonical, limits.max_note_bytes) {
                Ok(opened) => {
                    let available = limits
                        .max_indexed_content_bytes
                        .saturating_sub(index.indexed_content_bytes);
                    let (note, indexed_bytes, content_truncated) = note_from_source(
                        relative_path,
                        &opened.source,
                        metadata.len(),
                        metadata.modified().ok(),
                        available,
                    );
                    index.indexed_content_bytes =
                        index.indexed_content_bytes.saturating_add(indexed_bytes);
                    index.content_truncated |= content_truncated;
                    index.notes.push(note);
                    record_change_mark(&mut index, root, &canonical, false);
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

fn record_change_mark(
    index: &mut WorkspaceIndex,
    root: &WorkspaceRoot,
    path: &Path,
    directory: bool,
) {
    if index.change_marks.len() >= MAX_CHANGE_MARKS {
        index.change_marks_truncated = true;
        return;
    }
    let relative_path = path
        .strip_prefix(root.root())
        .ok()
        .filter(|path| !path.as_os_str().is_empty())
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
    let Ok(metadata) = fs::metadata(path) else {
        return;
    };
    index.change_marks.push(ChangeMark {
        relative_path,
        byte_len: metadata.len(),
        modified: metadata.modified().ok(),
        directory,
    });
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
    source_byte_len: u64,
    source_modified: Option<SystemTime>,
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
            source_byte_len,
            source_modified,
        },
        end,
        content_truncated,
    )
}

fn reused_note_with_content_limit(
    previous: &WorkspaceNote,
    available_content_bytes: usize,
) -> (WorkspaceNote, usize, bool) {
    let mut note = previous.clone();
    let keep = available_content_bytes.min(note.search_text.len());
    let mut end = keep;
    while end > 0 && !note.search_text.is_char_boundary(end) {
        end -= 1;
    }
    note.search_text.truncate(end);
    let source_len = usize::try_from(previous.source_byte_len).unwrap_or(usize::MAX);
    (note, end, end < source_len)
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

    fn obsidian_fixture_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/obsidian-vault")
    }

    #[test]
    fn la_boveda_fixture_conserva_aliases_callouts_y_bloquea_ambiguedades() {
        let root = obsidian_fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la fixture está contenida");
        let index = index_workspace(&vfs, WorkspaceLimits::default());

        assert_eq!(index.notes.len(), 4);
        assert!(
            index
                .notes
                .iter()
                .all(|note| !note.relative_path.starts_with(".obsidian"))
        );
        assert!(matches!(
            index.resolve_wikilink("seguridad"),
            WikiResolution::Ambiguous
        ));
        assert!(matches!(
            index.resolve_wikilink("archivo/seguridad"),
            WikiResolution::Found(note) if note.title == "Seguridad archivada"
        ));
        let seguridad = index
            .note_at_relative(Path::new("seguridad.md"))
            .expect("la nota existe");
        assert_eq!(index.backlinks_to(seguridad).len(), 1);
        assert!(matches!(
            index.resolve_wikilink("../secreto"),
            WikiResolution::Missing
        ));
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
    fn la_fotografia_detecta_cambios_dentro_de_subcarpetas() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        let index = index_workspace(&vfs, WorkspaceLimits::default());
        let snapshot = index.change_snapshot();

        assert!(!snapshot.may_have_changed(&vfs));
        fs::write(
            root.join("clases").join("redes.md"),
            "# Redes modificadas con más contenido",
        )
        .expect("se modifica una nota interna");
        assert!(snapshot.may_have_changed(&vfs));
    }

    #[test]
    fn la_fotografia_detecta_una_nota_eliminada_sin_buscar_fuera_de_la_raiz() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        let index = index_workspace(&vfs, WorkspaceLimits::default());
        let snapshot = index.change_snapshot();

        fs::remove_file(root.join("seguridad.md")).expect("se elimina la nota de la fixture");
        assert!(snapshot.may_have_changed(&vfs));
    }

    #[test]
    fn la_actualizacion_reutiliza_lo_intacto_y_relee_las_notas_modificadas() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        let initial = index_workspace(&vfs, WorkspaceLimits::default());

        fs::write(
            root.join("seguridad.md"),
            "# Seguridad actualizada\n\n[[redes]]\n\ncontenido nuevo",
        )
        .expect("se modifica la nota");
        let refreshed = reindex_workspace_cancellable(
            &vfs,
            &initial,
            WorkspaceLimits::default(),
            &AtomicBool::new(false),
        );

        let seguridad = refreshed
            .note_at_relative(Path::new("seguridad.md"))
            .expect("la nota actualizada se conserva");
        assert_eq!(seguridad.title, "Seguridad actualizada");
        assert_eq!(seguridad.wikilinks.len(), 1);
        assert_eq!(refreshed.search("ciberseguridad interna").len(), 1);
    }

    #[test]
    fn la_actualizacion_descubre_altas_y_bajas_sin_retener_notas_eliminadas() {
        let root = fixture_root();
        let vfs = WorkspaceRoot::open(&root).expect("la raíz es válida");
        let initial = index_workspace(&vfs, WorkspaceLimits::default());
        fs::remove_file(root.join("seguridad.md")).expect("se elimina la nota");
        fs::write(root.join("nueva.md"), "# Nueva nota").expect("se agrega la nota");

        let refreshed = reindex_workspace_cancellable(
            &vfs,
            &initial,
            WorkspaceLimits::default(),
            &AtomicBool::new(false),
        );

        assert!(
            refreshed
                .note_at_relative(Path::new("seguridad.md"))
                .is_none()
        );
        assert!(refreshed.note_at_relative(Path::new("nueva.md")).is_some());
    }

    #[test]
    fn limita_el_texto_retenido_para_busqueda_sin_partir_unicode() {
        let (note, bytes, truncated) = note_from_source(
            PathBuf::from("nota.md"),
            "áéí contenido importante",
            "áéí contenido importante".len() as u64,
            None,
            3,
        );

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
