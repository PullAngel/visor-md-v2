// Visor MD v2
//
// Núcleo nativo. Abre un .md, lo parsea con comrak, lo maqueta con parley y lo
// dibuja con tiny-skia sobre una ventana winit + softbuffer. Incluye edición
// source-first, guardado fiel, pestañas y workspace acotado. El chrome y la
// separación modular continúan evolucionando sobre esta base.
//
// Lo que se mide con esto va a docs/budget.md. El criterio de salida del
// Sprint 0 esta en docs/roadmap.md.

// Regla de docs/security.md: cero `unsafe` en codigo propio. La unica excepcion
// prevista es la capa de integracion con el sistema operativo, que todavia no
// existe y cuando exista se aisla en su propio modulo y se revisa a mano.
#![forbid(unsafe_code)]

pub mod editor;
mod files;
mod fonts;
mod limits;
mod recovery;
mod settings;
mod theme;
mod vfs;
mod workspace;

use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use arboard::Clipboard;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{Arena, Options, parse_document};
use parley::layout::{
    Affinity, Alignment, Cursor, GlyphRun, Layout, PositionedLayoutItem, Selection,
};
use parley::style::{
    FontFamily, FontFamilyName, FontStyle, FontWeight, GenericFamily, StyleProperty,
};
use parley::{AlignmentOptions, FontContext, LayoutContext, LineHeight};
use swash::FontRef;
use swash::scale::image::Content;
use swash::scale::{Render, ScaleContext, Source, StrikeWith};
use swash::zeno::{Format, Vector};
use tiny_skia::{Color, Paint, Pixmap, PremultipliedColorU8, Rect, Transform};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, Ime, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{CursorIcon, Theme, Window, WindowId};

use editor::{SourceEditor, TextBuffer};
use files::{
    DEFAULT_DOCUMENT_LIMIT_BYTES, FileIdentity, FileSaveError, LineEndings, TextMetadata,
    changed_on_disk, is_markdown_path, open_explicit_primary, save_explicit_primary,
    save_new_primary,
};
use fonts::{FONT_CODE, FONT_DOC, FONT_UI, register_embedded_fonts};
use limits::{Degradation, MAX_BLOCKS, MAX_INDENT_DEPTH, MAX_NEST, MAX_SAFE_LINE_BYTES};
use recovery::RecoverySession;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use settings::Settings;
use theme::{DAY, NIGHT, Palette, Role};
use vfs::WorkspaceRoot;
use workspace::{WikiResolution, WorkspaceIndex, WorkspaceLimits, index_workspace_cancellable};

const MARGIN: f32 = 48.0;
const MAX_MEASURE: f32 = 720.0;
const SELECTION_SCROLL_EDGE: f32 = 32.0;
const SELECTION_SCROLL_MAX_STEP: f32 = 18.0;
const CONTEXT_MENU_WIDTH: f32 = 224.0;
const CONTEXT_MENU_ROW_HEIGHT: f32 = 34.0;
const CONTEXT_MENU_PADDING: f32 = 8.0;
const CODE_COPY_WIDTH: f32 = 60.0;
const CODE_COPY_HEIGHT: f32 = 22.0;
const MAX_SEARCH_QUERY_CHARS: usize = 256;
const MAX_OVERLAY_LABEL_CHARS: usize = 180;
const STATUS_HEIGHT: f32 = 28.0;
const MAX_TAB_WIDTH: f32 = 220.0;
const PANEL_X: f32 = MARGIN;
const PANEL_Y: f32 = 80.0;
const PANEL_WIDTH: f32 = 420.0;
const PANEL_ROW_HEIGHT: f32 = 28.0;
const PANEL_CAPACITY: usize = 9;
const TOOLBAR_X: f32 = MARGIN;
const TOOLBAR_Y: f32 = 8.0;
const TOOLBAR_HEIGHT: f32 = 28.0;
const TOOLBAR_ITEM_WIDTH: f32 = 86.0;
const TOOLBAR_ACTIONS: [AppAction; 6] = [
    AppAction::NewDocument,
    AppAction::OpenDocument,
    AppAction::Save,
    AppAction::ToggleMode,
    AppAction::SearchDocument,
    AppAction::CommandPalette,
];
static NEXT_DOCUMENT_ID: AtomicU64 = AtomicU64::new(1);

fn tab_width(window_width: f32, tab_count: usize) -> f32 {
    if tab_count == 0 {
        return 0.0;
    }
    (window_width / tab_count as f32).min(MAX_TAB_WIDTH)
}

fn tab_index_at(
    x: f32,
    y: f32,
    window_width: f32,
    window_height: f32,
    count: usize,
) -> Option<usize> {
    if count == 0 || y < window_height - STATUS_HEIGHT || x < 0.0 {
        return None;
    }
    let width = tab_width(window_width, count);
    let index = (x / width) as usize;
    (index < count).then_some(index)
}

fn tab_close_index_at(
    x: f32,
    y: f32,
    window_width: f32,
    window_height: f32,
    count: usize,
) -> Option<usize> {
    let index = tab_index_at(x, y, window_width, window_height, count)?;
    let width = tab_width(window_width, count);
    let local_x = x - index as f32 * width;
    (local_x >= (width - 26.0).max(0.0)).then_some(index)
}

fn adjacent_tab_id(order: &[u64], current: u64, backwards: bool) -> Option<u64> {
    if order.len() < 2 {
        return None;
    }
    let position = order.iter().position(|id| *id == current)?;
    let target = if backwards {
        (position + order.len() - 1) % order.len()
    } else {
        (position + 1) % order.len()
    };
    order.get(target).copied()
}

fn panel_window(total: usize, selected: usize, capacity: usize) -> std::ops::Range<usize> {
    if total == 0 || capacity == 0 {
        return 0..0;
    }
    let selected = selected.min(total - 1);
    let start = selected
        .saturating_sub(capacity / 2)
        .min(total.saturating_sub(capacity));
    start..(start + capacity.min(total))
}

fn panel_item_at(x: f32, y: f32, total: usize, selected: usize) -> Option<usize> {
    if x < PANEL_X || x >= PANEL_X + PANEL_WIDTH || y < PANEL_Y + PANEL_ROW_HEIGHT {
        return None;
    }
    let range = panel_window(total, selected, PANEL_CAPACITY);
    let visible_row = ((y - PANEL_Y) / PANEL_ROW_HEIGHT) as usize;
    let item_offset = visible_row.checked_sub(1)?;
    (item_offset < range.len()).then_some(range.start + item_offset)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum WorkspaceTreeKind {
    Directory,
    Note,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WorkspaceTreeRow {
    relative_path: PathBuf,
    depth: usize,
    kind: WorkspaceTreeKind,
    collapsed: bool,
}

impl WorkspaceTreeRow {
    fn label(&self) -> String {
        let name = self
            .relative_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("?");
        let indent = "  ".repeat(self.depth.min(12));
        match self.kind {
            WorkspaceTreeKind::Directory => {
                format!("{indent}{} {name}/", if self.collapsed { "+" } else { "-" })
            }
            WorkspaceTreeKind::Note => format!("{indent}{name}"),
        }
    }

    fn note_path(&self) -> Option<&std::path::Path> {
        matches!(self.kind, WorkspaceTreeKind::Note).then_some(&self.relative_path)
    }
}

fn workspace_tree_rows(paths: &[PathBuf]) -> Vec<WorkspaceTreeRow> {
    let mut directories = BTreeSet::new();
    for path in paths {
        let mut parent = path.parent();
        while let Some(directory) = parent {
            if directory.as_os_str().is_empty() {
                break;
            }
            directories.insert(directory.to_path_buf());
            parent = directory.parent();
        }
    }
    let mut rows = directories
        .into_iter()
        .map(|relative_path| WorkspaceTreeRow {
            depth: relative_path.components().count().saturating_sub(1),
            relative_path,
            kind: WorkspaceTreeKind::Directory,
            collapsed: false,
        })
        .chain(paths.iter().cloned().map(|relative_path| WorkspaceTreeRow {
            depth: relative_path.components().count().saturating_sub(1),
            relative_path,
            kind: WorkspaceTreeKind::Note,
            collapsed: false,
        }))
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.relative_path
            .cmp(&right.relative_path)
            .then_with(|| match (&left.kind, &right.kind) {
                (WorkspaceTreeKind::Directory, WorkspaceTreeKind::Note) => std::cmp::Ordering::Less,
                (WorkspaceTreeKind::Note, WorkspaceTreeKind::Directory) => {
                    std::cmp::Ordering::Greater
                }
                _ => std::cmp::Ordering::Equal,
            })
    });
    rows
}

fn visible_workspace_tree_rows(
    paths: &[PathBuf],
    collapsed: &HashSet<PathBuf>,
) -> Vec<WorkspaceTreeRow> {
    workspace_tree_rows(paths)
        .into_iter()
        .filter(|row| {
            let mut parent = row.relative_path.parent();
            while let Some(directory) = parent {
                if directory.as_os_str().is_empty() {
                    break;
                }
                if collapsed.contains(directory) {
                    return false;
                }
                parent = directory.parent();
            }
            true
        })
        .map(|mut row| {
            row.collapsed = collapsed.contains(&row.relative_path);
            row
        })
        .collect()
}

fn toolbar_action_at(x: f32, y: f32, window_width: f32) -> Option<AppAction> {
    if x < TOOLBAR_X || y < TOOLBAR_Y || y >= TOOLBAR_Y + TOOLBAR_HEIGHT || x >= window_width {
        return None;
    }
    let index = ((x - TOOLBAR_X) / TOOLBAR_ITEM_WIDTH) as usize;
    TOOLBAR_ACTIONS.get(index).copied()
}

fn adjacent_toolbar_index(current: usize, backwards: bool) -> usize {
    if backwards {
        (current + TOOLBAR_ACTIONS.len() - 1) % TOOLBAR_ACTIONS.len()
    } else {
        (current + 1) % TOOLBAR_ACTIONS.len()
    }
}

fn tab_label(path: &str, dirty: bool, max_chars: usize) -> String {
    let path_buf = PathBuf::from(path);
    let name = path_buf
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(path);
    let max_chars = max_chars.max(4);
    let suffix = if dirty { " *" } else { "" };
    let available = max_chars.saturating_sub(suffix.chars().count());
    let shortened = if name.chars().count() > available {
        format!(
            "{}…",
            name.chars()
                .take(available.saturating_sub(1))
                .collect::<String>()
        )
    } else {
        name.to_string()
    };
    format!("{shortened}{suffix}")
}

/// Los controles efímeros no necesitan retener texto ilimitado. El límite se
/// aplica por caracteres para no cortar UTF-8 y solo reduce la entrada de la
/// interfaz, nunca el documento ni el índice de workspace.
fn append_limited_text(destination: &mut String, text: &str, limit: usize) {
    let remaining = limit.saturating_sub(destination.chars().count());
    destination.extend(text.chars().take(remaining));
}

fn abbreviated_label(text: &str, limit: usize) -> String {
    let mut chars = text.chars();
    let abbreviated: String = chars.by_ref().take(limit).collect();
    if chars.next().is_some() {
        format!("{abbreviated}…")
    } else {
        abbreviated
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ContextAction {
    Paste,
    CopyText,
    CopyMarkdown,
    Search,
    ToggleMode,
    ToggleSplit,
    Save,
    SaveAs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppAction {
    NewDocument,
    OpenDocument,
    Save,
    SaveAs,
    CloseDocument,
    ToggleMode,
    ToggleSplit,
    SearchDocument,
    ChooseWorkspace,
    RefreshWorkspace,
    SearchWorkspace,
    DocumentOutline,
    WorkspaceFiles,
    Backlinks,
    RestoreRecovery,
    ToggleRecovery,
    CommandPalette,
}

const APP_ACTIONS: [AppAction; 16] = [
    AppAction::NewDocument,
    AppAction::OpenDocument,
    AppAction::Save,
    AppAction::SaveAs,
    AppAction::CloseDocument,
    AppAction::ToggleMode,
    AppAction::ToggleSplit,
    AppAction::SearchDocument,
    AppAction::ChooseWorkspace,
    AppAction::RefreshWorkspace,
    AppAction::SearchWorkspace,
    AppAction::DocumentOutline,
    AppAction::WorkspaceFiles,
    AppAction::Backlinks,
    AppAction::RestoreRecovery,
    AppAction::ToggleRecovery,
];

impl AppAction {
    fn label(self) -> &'static str {
        match self {
            Self::NewDocument => "Nuevo documento · Ctrl+N",
            Self::OpenDocument => "Abrir documento · Ctrl+O",
            Self::Save => "Guardar · Ctrl+S",
            Self::SaveAs => "Guardar como · Ctrl+Shift+S",
            Self::CloseDocument => "Cerrar pestaña · Ctrl+W",
            Self::ToggleMode => "Alternar lectura y edición · F2",
            Self::ToggleSplit => "Comparar fuente y vista · F3",
            Self::SearchDocument => "Buscar en documento · Ctrl+F",
            Self::ChooseWorkspace => "Abrir carpeta · Ctrl+Shift+O",
            Self::RefreshWorkspace => "Actualizar índice de carpeta · Ctrl+Shift+I",
            Self::SearchWorkspace => "Buscar en carpeta · Ctrl+Shift+F",
            Self::DocumentOutline => "Índice del documento · Ctrl+Shift+L",
            Self::WorkspaceFiles => "Notas de la carpeta · Ctrl+Shift+T",
            Self::Backlinks => "Backlinks · Ctrl+Shift+B",
            Self::RestoreRecovery => "Abrir recuperación · Ctrl+Shift+R",
            Self::ToggleRecovery => "Activar o desactivar recuperación local",
            Self::CommandPalette => "Mostrar todas las acciones · Ctrl+Shift+P",
        }
    }

    fn short_label(self, mode: DocumentMode) -> &'static str {
        match self {
            Self::NewDocument => "Nuevo",
            Self::OpenDocument => "Abrir",
            Self::Save => "Guardar",
            Self::ToggleMode if mode == DocumentMode::Reading => "Editar",
            Self::ToggleMode => "Leer",
            Self::ToggleSplit if mode == DocumentMode::Split => "Cerrar comparación",
            Self::ToggleSplit => "Comparar",
            Self::SearchDocument => "Buscar",
            Self::CommandPalette => "Más",
            _ => self.label(),
        }
    }
}

fn filtered_actions(query: &str) -> Vec<AppAction> {
    let query = query.trim().to_lowercase();
    APP_ACTIONS
        .iter()
        .copied()
        .filter(|action| query.is_empty() || action.label().to_lowercase().contains(&query))
        .collect()
}

impl ContextAction {
    fn label(self) -> &'static str {
        match self {
            Self::Paste => "Pegar",
            Self::CopyText => "Copiar texto",
            Self::CopyMarkdown => "Copiar Markdown original",
            Self::Search => "Buscar en documento",
            Self::ToggleMode => "Alternar lectura y edición",
            Self::ToggleSplit => "Comparar fuente y vista",
            Self::Save => "Guardar",
            Self::SaveAs => "Guardar como",
        }
    }

    fn source_markdown(self) -> bool {
        matches!(self, Self::CopyMarkdown)
    }
}

fn context_actions(mode: DocumentMode) -> &'static [ContextAction] {
    match mode {
        DocumentMode::Reading => &[
            ContextAction::CopyText,
            ContextAction::CopyMarkdown,
            ContextAction::Search,
            ContextAction::ToggleMode,
            ContextAction::ToggleSplit,
            ContextAction::Save,
            ContextAction::SaveAs,
        ],
        DocumentMode::SourceEditing => &[
            ContextAction::Paste,
            ContextAction::CopyText,
            ContextAction::CopyMarkdown,
            ContextAction::Search,
            ContextAction::ToggleMode,
            ContextAction::ToggleSplit,
            ContextAction::Save,
            ContextAction::SaveAs,
        ],
        DocumentMode::Split => &[
            ContextAction::Paste,
            ContextAction::CopyText,
            ContextAction::CopyMarkdown,
            ContextAction::Search,
            ContextAction::ToggleMode,
            ContextAction::ToggleSplit,
            ContextAction::Save,
            ContextAction::SaveAs,
        ],
    }
}

fn context_action_at(
    menu: (f32, f32),
    pointer: (f32, f32),
    mode: DocumentMode,
) -> Option<ContextAction> {
    let (_, top) = menu;
    let (x, y) = pointer;
    if !(menu.0..=menu.0 + CONTEXT_MENU_WIDTH).contains(&x)
        || !(top..=top + CONTEXT_MENU_ROW_HEIGHT * context_actions(mode).len() as f32).contains(&y)
    {
        return None;
    }
    context_actions(mode)
        .get(((y - top) / CONTEXT_MENU_ROW_HEIGHT) as usize)
        .copied()
}

#[derive(Default)]
struct TraversalState {
    degradation: Option<Degradation>,
}

impl TraversalState {
    fn mark(&mut self, reason: Degradation) {
        if self.degradation.is_none() {
            self.degradation = Some(reason);
        }
    }
}

struct ParseOutcome {
    blocks: Vec<Block>,
    degradation: Option<Degradation>,
}

enum AppEvent {
    DocumentReady {
        document_id: u64,
        request: u64,
        path: PathBuf,
        source: String,
        metadata: TextMetadata,
        identity: FileIdentity,
        baseline_bytes: Vec<u8>,
        outcome: ParseOutcome,
        elapsed_ms: f64,
    },
    DocumentFailed {
        document_id: u64,
        request: u64,
        error: String,
    },
    ViewReady {
        document_id: u64,
        revision: u64,
        outcome: ParseOutcome,
        elapsed_ms: f64,
    },
    ViewRequested {
        document_id: u64,
        revision: u64,
    },
    ViewFailed {
        document_id: u64,
        revision: u64,
        error: String,
    },
    SaveReady {
        document_id: u64,
        revision: u64,
        identity: FileIdentity,
        baseline_bytes: Vec<u8>,
    },
    SaveFailed {
        document_id: u64,
        error: String,
        conflict: bool,
    },
    SaveAsReady {
        document_id: u64,
        path: PathBuf,
        revision: u64,
        identity: FileIdentity,
        baseline_bytes: Vec<u8>,
    },
    WorkspaceReady {
        request: u64,
        root: WorkspaceRoot,
        index: WorkspaceIndex,
    },
    WorkspaceFailed {
        request: u64,
        error: String,
    },
    WorkspaceChangeChecked {
        request: u64,
        changed: bool,
        truncated: bool,
    },
    ExternalChangeChecked {
        document_id: u64,
        result: Result<bool, String>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DocumentMode {
    Reading,
    SourceEditing,
    Split,
}

impl DocumentMode {
    fn is_editable(self) -> bool {
        matches!(self, Self::SourceEditing | Self::Split)
    }
}

fn is_current_view_result(
    result_document_id: u64,
    target_document_id: u64,
    result_revision: u64,
    active_revision: u64,
) -> bool {
    result_document_id == target_document_id && result_revision == active_revision
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Brush {
    foreground: (u8, u8, u8),
    background: Option<(u8, u8, u8)>,
    baseline_shift: i8,
}

impl Brush {
    fn text(foreground: (u8, u8, u8)) -> Self {
        Self {
            foreground,
            background: None,
            baseline_shift: 0,
        }
    }

    fn semantic(
        foreground: (u8, u8, u8),
        background: Option<(u8, u8, u8)>,
        baseline_shift: i8,
    ) -> Self {
        Self {
            foreground,
            background,
            baseline_shift,
        }
    }
}

/// Enfasis inline acumulado sobre un tramo de texto. Se acumula al bajar por
/// el arbol: un `**texto _asi_**` llega al fondo con `strong` y `emph` juntos.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Emphasis {
    strong: bool,
    emph: bool,
    code: bool,
    link: bool,
    strike: bool,
    kbd: bool,
    mark: bool,
    sub: bool,
    sup: bool,
}

impl Emphasis {
    fn is_plain(self) -> bool {
        self == Self::default()
    }
}

/// Un tramo del texto de un bloque con su enfasis. Los rangos son offsets de
/// bytes dentro de `Block::text`, que es lo que espera parley.
#[derive(Clone, Debug)]
struct Span {
    start: usize,
    end: usize,
    /// Rango equivalente en la fuente Markdown. A diferencia de `start` y
    /// `end`, no apunta al texto renderizado sino al archivo original.
    source: SourceRange,
    style: Emphasis,
}

/// Rango de bytes semiabierto dentro de la fuente original: `start..end`.
/// Comrak entrega linea y columna; `SourceIndex` lo convierte una sola vez a
/// offsets que luego sirven para seleccion, edicion y round-trip.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SourceRange {
    start: usize,
    end: usize,
}

impl SourceRange {
    fn is_valid_for(self, source: &str) -> bool {
        self.start <= self.end
            && self.end <= source.len()
            && source.is_char_boundary(self.start)
            && source.is_char_boundary(self.end)
    }

    fn is_valid_for_buffer(self, source: &TextBuffer) -> bool {
        self.start <= self.end
            && self.end <= source.len_bytes()
            && source.is_char_boundary(self.start)
            && source.is_char_boundary(self.end)
    }
}

/// Indice compacto de comienzos de linea. Las columnas de comrak son offsets
/// UTF-8 de base uno, de modo que se pueden convertir sin recorrer el archivo
/// por cada nodo.
struct SourceIndex {
    line_starts: Vec<usize>,
    len: usize,
}

impl SourceIndex {
    fn new(source: &str) -> Self {
        let mut line_starts = vec![0];
        for (i, byte) in source.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Self {
            line_starts,
            len: source.len(),
        }
    }

    fn range(&self, pos: comrak::nodes::Sourcepos) -> SourceRange {
        let start_line = self
            .line_starts
            .get(pos.start.line.saturating_sub(1))
            .copied()
            .unwrap_or(self.len);
        let end_line = self
            .line_starts
            .get(pos.end.line.saturating_sub(1))
            .copied()
            .unwrap_or(self.len);
        let start = start_line
            .saturating_add(pos.start.column.saturating_sub(1))
            .min(self.len);
        // `end.column` apunta al ultimo byte del nodo; el rango propio usa
        // fin exclusivo, por eso no se resta uno.
        let end = end_line.saturating_add(pos.end.column).min(self.len);
        SourceRange {
            start,
            end: end.max(start),
        }
    }

    fn range_of(&self, node: &AstNode<'_>) -> SourceRange {
        self.range(node.data.borrow().sourcepos)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InlineTargetKind {
    Link,
    WikiLink,
    Image,
}

impl InlineTargetKind {
    fn is_navigable(self) -> bool {
        matches!(self, Self::Link | Self::WikiLink)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LinkDestinationKind {
    Web,
    Mail,
    RelativeFile,
    Blocked,
}

fn classify_link_destination(destination: &str) -> LinkDestinationKind {
    let lowered = destination.trim().to_ascii_lowercase();
    if lowered.starts_with("https://") || lowered.starts_with("http://") {
        LinkDestinationKind::Web
    } else if lowered.starts_with("mailto:") {
        LinkDestinationKind::Mail
    } else if lowered.starts_with("file:")
        || lowered.starts_with("\\\\")
        || lowered.starts_with('/')
        || lowered.contains(':')
        || lowered.split(['/', '\\']).any(|part| part == "..")
    {
        LinkDestinationKind::Blocked
    } else {
        LinkDestinationKind::RelativeFile
    }
}

fn target_label(kind: InlineTargetKind, destination: &str) -> &'static str {
    match kind {
        InlineTargetKind::WikiLink => "enlace de bóveda",
        InlineTargetKind::Image => "imagen",
        InlineTargetKind::Link => match classify_link_destination(destination) {
            LinkDestinationKind::Web => "enlace web",
            LinkDestinationKind::Mail => "correo",
            LinkDestinationKind::RelativeFile => "archivo relativo",
            LinkDestinationKind::Blocked => "destino bloqueado",
        },
    }
}

/// El destino cambia la señal visual, pero no concede ninguna capacidad. El
/// subrayado sigue presente para que la distinción no dependa solo del color.
fn link_color(palette: Palette, destination: &str) -> (u8, u8, u8) {
    match classify_link_destination(destination) {
        LinkDestinationKind::Web | LinkDestinationKind::Mail => palette.external_link,
        LinkDestinationKind::RelativeFile => palette.accent,
        LinkDestinationKind::Blocked => palette.dim,
    }
}

fn external_destination(destination: &str) -> Option<&str> {
    let destination = destination.trim();
    matches!(
        classify_link_destination(destination),
        LinkDestinationKind::Web | LinkDestinationKind::Mail
    )
    .then_some(destination)
}

/// Delegar el enlace al sistema no pasa por un shell. La URL ya fue
/// clasificada antes de llegar aquí y esta función solo se invoca tras una
/// acción explícita de la persona usuaria.
fn open_external_destination(destination: &str) -> Result<(), String> {
    let destination = external_destination(destination)
        .ok_or_else(|| "el destino no está permitido para apertura externa".to_string())?;
    #[cfg(target_os = "windows")]
    let mut command = std::process::Command::new("explorer.exe");
    #[cfg(target_os = "macos")]
    let mut command = std::process::Command::new("open");
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = std::process::Command::new("xdg-open");
    command
        .arg(destination)
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("no se pudo delegar el enlace al sistema: {error}"))
}

/// Orden estable de los enlaces que puede recorrer el teclado. Las imágenes y
/// los destinos no interactivos permanecen fuera hasta que tengan una acción
/// segura y una UX propia.
fn link_targets_in_document_order(blocks: &[Block]) -> Vec<(usize, usize)> {
    blocks
        .iter()
        .enumerate()
        .flat_map(|(block_index, block)| {
            block
                .targets
                .iter()
                .enumerate()
                .filter_map(move |(target_index, target)| {
                    target
                        .kind
                        .is_navigable()
                        .then_some((block_index, target_index))
                })
        })
        .collect()
}

fn next_link_target(
    links: &[(usize, usize)],
    current: Option<(usize, usize)>,
    backwards: bool,
) -> Option<(usize, usize)> {
    let current_index = current.and_then(|target| links.iter().position(|link| *link == target));
    match (current_index, backwards) {
        (Some(index), false) => links.get((index + 1) % links.len()).copied(),
        (Some(0), true) => links.last().copied(),
        (Some(index), true) => links.get(index - 1).copied(),
        (None, false) => links.first().copied(),
        (None, true) => links.last().copied(),
    }
}

/// Semantica interactiva que no puede perderse al producir texto visible.
#[derive(Clone, Debug, PartialEq, Eq)]
struct InlineTarget {
    kind: InlineTargetKind,
    start: usize,
    end: usize,
    source: SourceRange,
    destination: String,
    title: String,
}

#[derive(Clone, Debug)]
struct TableCell {
    source: SourceRange,
    text: String,
    /// El estilo y los destinos son relativos al texto de esta celda. No se
    /// reconstruyen desde los separadores `|`, que no forman parte del valor.
    spans: Vec<Span>,
    targets: Vec<InlineTarget>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellAlignment {
    None,
    Left,
    Center,
    Right,
}

impl From<comrak::nodes::TableAlignment> for CellAlignment {
    fn from(value: comrak::nodes::TableAlignment) -> Self {
        match value {
            comrak::nodes::TableAlignment::None => Self::None,
            comrak::nodes::TableAlignment::Left => Self::Left,
            comrak::nodes::TableAlignment::Center => Self::Center,
            comrak::nodes::TableAlignment::Right => Self::Right,
        }
    }
}

/// Un bloque del documento ya aplanado: el arbol tipado de comrak reducido a
/// lo que el renderizador sabe dibujar.
#[derive(Clone, Copy, Debug)]
enum Kind {
    Heading(u8),
    Para,
    Item(u8),
    Code,
    TableRow {
        header: bool,
    },
    Quote,
    /// Callout de Obsidian reconocido dentro de una cita. Sigue siendo texto
    /// local: no crea comportamiento, estado ni recursos externos.
    Callout,
    /// Linea horizontal (`---`). No lleva texto.
    Rule,
}

#[derive(Clone)]
struct Block {
    text: String,
    /// Tramos con enfasis. Vacio en el caso comun (texto sin formato), que es
    /// la mayoria de las lineas de un documento real.
    spans: Vec<Span>,
    /// Posicion del bloque en el archivo original.
    source: SourceRange,
    /// Links e imagenes conservan su destino aunque el renderer actual solo
    /// pinte el texto visible.
    targets: Vec<InlineTarget>,
    /// Lenguaje o info string de un bloque de codigo.
    code_info: Option<String>,
    /// Celdas originales. El texto unido con `|` es solo una representacion
    /// temporal para el renderer del prototipo.
    table_cells: Vec<TableCell>,
    /// Alineación por columna, repetida en cada fila mientras el renderer use
    /// bloques aplanados. El modelo definitivo la conservará en la tabla.
    table_alignments: Vec<CellAlignment>,
    /// Profundidad semantica de cita, independiente de la sangria visual.
    quote_depth: u8,
    kind: Kind,
    /// Vineta, numero o casilla de un item de lista. Se dibuja en el margen,
    /// fuera del ancho de medida del texto.
    marker: Option<Marker>,
}

/// Punto temporal del lector. Los offsets viven en el texto renderizado del
/// bloque, igual que los layouts de Parley; el editor posterior lo relacionará
/// con rangos de fuente persistentes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BlockCursor {
    block: usize,
    offset: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DocumentSelection {
    anchor: BlockCursor,
    focus: BlockCursor,
}

impl DocumentSelection {
    fn collapsed(cursor: BlockCursor) -> Self {
        Self {
            anchor: cursor,
            focus: cursor,
        }
    }

    /// Devuelve el rango que corresponde pintar dentro de un bloque. Los
    /// bloques intermedios se seleccionan completos, sin inventar offsets.
    fn range_for(self, block: usize, text_len: usize) -> Option<(usize, usize)> {
        let (first, last) = if self.anchor <= self.focus {
            (self.anchor, self.focus)
        } else {
            (self.focus, self.anchor)
        };
        if !(first.block..=last.block).contains(&block) {
            return None;
        }
        let start = if block == first.block {
            first.offset.min(text_len)
        } else {
            0
        };
        let end = if block == last.block {
            last.offset.min(text_len)
        } else {
            text_len
        };
        Some((start, end))
    }

    /// Copia la representación legible de la selección. Cada bloque conserva
    /// su texto visible y los bloques distintos se separan con un salto de
    /// línea. Cuando un bloque entra completo, mantiene sus marcadores
    /// estructurales legibles; Ctrl+Shift+C sigue siendo la vía para la fuente
    /// Markdown exacta.
    fn rendered_text(self, blocks: &[Block]) -> Option<String> {
        let first = self.anchor.min(self.focus);
        let last = self.anchor.max(self.focus);
        let mut pieces = Vec::new();
        for index in first.block..=last.block {
            let block = blocks.get(index)?;
            let (start, end) = self.range_for(index, block.text.len())?;
            let text = block.text.get(start..end)?;
            if start == 0 && end == block.text.len() {
                pieces.push(format!("{}{}", rendered_block_prefix(block), text));
            } else {
                pieces.push(text.to_owned());
            }
        }
        let text = pieces.join("\n");
        (!text.is_empty()).then_some(text)
    }

    /// Copia el tramo original que cubre los bloques seleccionados. Esta
    /// operación es deliberadamente por bloque completo: los offsets de la
    /// vista renderizada no equivalen siempre a offsets del Markdown fuente.
    fn source_blocks(self, source: &str, blocks: &[Block]) -> Option<String> {
        let first = self.anchor.min(self.focus).block;
        let last = self.anchor.max(self.focus).block;
        let start = blocks.get(first)?.source.start;
        let end = blocks.get(last)?.source.end;
        let text = source.get(start..end)?;
        (!text.is_empty()).then_some(text.to_owned())
    }
}

/// Prefijo semántico de una copia de lectura. No intenta serializar Markdown:
/// solo evita que una lista, tarea o cita quede convertida en párrafos planos
/// al pegarla en otra aplicación.
fn rendered_block_prefix(block: &Block) -> String {
    match block.marker.as_ref() {
        Some(Marker::Text(marker)) => format!("{marker} "),
        Some(Marker::Task { done }) => {
            if *done {
                "[x] ".to_owned()
            } else {
                "[ ] ".to_owned()
            }
        }
        None if matches!(block.kind, Kind::Quote) => "> ".repeat(block.quote_depth.max(1) as usize),
        None => String::new(),
    }
}

impl Block {
    fn new(
        text: String,
        spans: Vec<Span>,
        kind: Kind,
        source: SourceRange,
        targets: Vec<InlineTarget>,
    ) -> Self {
        Self {
            text,
            spans,
            source,
            targets,
            code_info: None,
            table_cells: Vec::new(),
            table_alignments: Vec::new(),
            quote_depth: 0,
            kind,
            marker: None,
        }
    }

    /// Sangría efectiva del bloque. Las citas conservan la profundidad en el
    /// modelo y cada nivel adicional se desplaza sin dejar que una entrada
    /// patológica consuma todo el ancho de lectura.
    fn indent(&self) -> f32 {
        let nested_quote = if matches!(self.kind, Kind::Quote | Kind::Callout) {
            self.quote_depth.saturating_sub(1).min(MAX_INDENT_DEPTH) as f32 * 20.0
        } else {
            0.0
        };
        self.kind.indent() + nested_quote
    }
}

impl Kind {
    /// (tamano, peso, rol de color, monoespaciada)
    fn style(self) -> (f32, f32, Role, bool) {
        match self {
            // Escala tipografica de docs/design.md.
            Kind::Heading(1) => (31.0, 700.0, Role::Text, false),
            Kind::Heading(2) => (25.0, 700.0, Role::Text, false),
            Kind::Heading(3) => (20.0, 600.0, Role::Text, false),
            Kind::Heading(_) => (17.0, 600.0, Role::Text, false),
            Kind::Para | Kind::Item(_) => (16.0, 400.0, Role::Text, false),
            Kind::Code => (13.5, 400.0, Role::Accent, true),
            Kind::TableRow { header: true } => (15.0, 600.0, Role::Text, true),
            Kind::TableRow { header: false } => (15.0, 400.0, Role::Dim, true),
            Kind::Quote => (16.0, 400.0, Role::Dim, false),
            Kind::Callout => (16.0, 400.0, Role::Text, false),
            Kind::Rule => (16.0, 400.0, Role::Dim, false),
        }
    }

    fn line_height(self) -> f32 {
        match self {
            Kind::Heading(1) => 1.2,
            Kind::Heading(2) => 1.3,
            Kind::Heading(3) => 1.35,
            Kind::Heading(_) => 1.4,
            Kind::Code | Kind::TableRow { .. } => 1.7,
            _ => 1.65,
        }
    }

    /// Espacio antes del bloque.
    fn space_before(self) -> f32 {
        match self {
            Kind::Heading(1) => 34.0,
            Kind::Heading(2) => 30.0,
            Kind::Heading(_) => 24.0,
            Kind::Item(_) => 4.0,
            Kind::Code | Kind::TableRow { .. } => 2.0,
            Kind::Rule => 28.0,
            _ => 16.0,
        }
    }

    fn indent(self) -> f32 {
        match self {
            Kind::Item(d) => 22.0 * d.min(MAX_INDENT_DEPTH) as f32,
            Kind::Quote | Kind::Callout => 20.0,
            _ => 0.0,
        }
    }
}

// ---------------------------------------------------------------- parseo

#[derive(Default)]
struct InlineOutput {
    text: String,
    spans: Vec<Span>,
    targets: Vec<InlineTarget>,
}

/// La allowlist es deliberadamente pequeña. Estas etiquetas no crean un DOM,
/// no aceptan atributos y no activan recursos: solo se traducen a estilo
/// nativo de texto. Cualquier otra forma de HTML queda como fuente visible.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NativeHtml {
    Break,
    Open(HtmlSemantic),
    Close(HtmlSemantic),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HtmlSemantic {
    Kbd,
    Mark,
    Sub,
    Sup,
}

fn native_html(html: &str) -> Option<NativeHtml> {
    match html.trim().to_ascii_lowercase().as_str() {
        "<br>" | "<br/>" | "<br />" => Some(NativeHtml::Break),
        "<kbd>" => Some(NativeHtml::Open(HtmlSemantic::Kbd)),
        "</kbd>" => Some(NativeHtml::Close(HtmlSemantic::Kbd)),
        "<mark>" => Some(NativeHtml::Open(HtmlSemantic::Mark)),
        "</mark>" => Some(NativeHtml::Close(HtmlSemantic::Mark)),
        "<sub>" => Some(NativeHtml::Open(HtmlSemantic::Sub)),
        "</sub>" => Some(NativeHtml::Close(HtmlSemantic::Sub)),
        "<sup>" => Some(NativeHtml::Open(HtmlSemantic::Sup)),
        "</sup>" => Some(NativeHtml::Close(HtmlSemantic::Sup)),
        _ => None,
    }
}

impl HtmlSemantic {
    fn apply(self, style: &mut Emphasis) {
        match self {
            Self::Kbd => style.kbd = true,
            Self::Mark => style.mark = true,
            Self::Sub => style.sub = true,
            Self::Sup => style.sup = true,
        }
    }
}

struct HtmlScope {
    semantic: HtmlSemantic,
    raw_open: String,
    source: SourceRange,
    output: InlineOutput,
}

/// Los tags HTML no forman parte del árbol de comrak: llegan como hermanos de
/// sus textos. Esta pequeña pila retiene solo las etiquetas permitidas hasta
/// ver su cierre exacto. Así una apertura malformada no puede ocultarse ni
/// cambiar el estilo del resto del documento.
#[derive(Default)]
struct InlineCollector {
    output: InlineOutput,
    scopes: Vec<HtmlScope>,
}

impl InlineCollector {
    fn current_mut(&mut self) -> &mut InlineOutput {
        self.scopes
            .last_mut()
            .map(|scope| &mut scope.output)
            .unwrap_or(&mut self.output)
    }

    fn len(&self) -> usize {
        self.scopes
            .last()
            .map(|scope| scope.output.text.len())
            .unwrap_or(self.output.text.len())
    }

    fn literal(&mut self, text: &str, style: Emphasis, source: SourceRange) {
        if text.is_empty() {
            return;
        }
        // El texto dentro de una etiqueta pendiente conserva incluso su tramo
        // plano: al cerrar se le aplica la semántica nativa sin volver a
        // recorrer ni partir los offsets UTF-8.
        let retain_plain_span = !self.scopes.is_empty();
        let output = self.current_mut();
        let start = output.text.len();
        output.text.push_str(text);
        if retain_plain_span || !style.is_plain() {
            output.spans.push(Span {
                start,
                end: output.text.len(),
                source,
                style,
            });
        }
    }

    fn append_output(&mut self, mut child: InlineOutput) {
        let offset = self.len();
        for span in &mut child.spans {
            span.start += offset;
            span.end += offset;
        }
        for target in &mut child.targets {
            target.start += offset;
            target.end += offset;
        }
        let output = self.current_mut();
        output.text.push_str(&child.text);
        output.spans.extend(child.spans);
        output.targets.extend(child.targets);
    }

    fn open(&mut self, semantic: HtmlSemantic, raw_open: String, source: SourceRange) {
        self.scopes.push(HtmlScope {
            semantic,
            raw_open,
            source,
            output: InlineOutput::default(),
        });
    }

    fn close(&mut self, semantic: HtmlSemantic) -> bool {
        if !self
            .scopes
            .last()
            .is_some_and(|scope| scope.semantic == semantic)
        {
            return false;
        }
        let mut scope = self
            .scopes
            .pop()
            .expect("se verifico que la pila no esta vacia");
        for span in &mut scope.output.spans {
            semantic.apply(&mut span.style);
        }
        self.append_output(scope.output);
        true
    }

    fn finish(mut self) -> InlineOutput {
        // Una etiqueta permitida pero sin cierre no recibe semántica. Su tag
        // de apertura vuelve a la salida como código visible, antes de su
        // contenido, para no esconder Markdown defectuoso o hostil.
        while let Some(scope) = self.scopes.pop() {
            let inert = Emphasis {
                code: true,
                ..Emphasis::default()
            };
            self.literal(&scope.raw_open, inert, scope.source);
            self.append_output(scope.output);
        }
        self.output
    }
}

/// Extrae el destino y el texto visible de un wikilink de Obsidian. Esto no
/// abre nada ni consulta rutas: solo conserva una intención que la capa de
/// workspace resolverá contra una raíz concedida explícitamente.
fn wikilink_parts(raw: &str) -> Option<(String, String)> {
    let raw = raw.trim();
    let (target, alias) = raw.split_once('|').map_or((raw, None), |(target, alias)| {
        (target.trim(), Some(alias.trim()))
    });
    if target.is_empty() || target.contains(['\r', '\n']) {
        return None;
    }
    let label = alias.filter(|alias| !alias.is_empty()).unwrap_or(target);
    Some((target.to_owned(), label.to_owned()))
}

fn heading_key(text: &str) -> String {
    text.trim().trim_end_matches('#').trim().to_lowercase()
}

/// `comrak` trata `[[nota]]` como texto ordinario. Se reconoce únicamente en
/// texto Markdown normal: nunca dentro de código. Un formato defectuoso queda
/// visible de forma literal, en vez de inventar un destino o borrar contenido.
fn output_literal_with_wikilinks(
    output: &mut InlineCollector,
    text: &str,
    style: Emphasis,
    source: SourceRange,
) {
    let mut remaining = text;
    while let Some(open) = remaining.find("[[") {
        output.literal(&remaining[..open], style, source);
        let after_open = &remaining[open + 2..];
        let Some(close) = after_open.find("]]") else {
            output.literal(&remaining[open..], style, source);
            return;
        };
        let raw = &after_open[..close];
        if let Some((destination, label)) = wikilink_parts(raw) {
            let start = output.len();
            let mut linked = style;
            linked.link = true;
            output.literal(&label, linked, source);
            let end = output.len();
            output.current_mut().targets.push(InlineTarget {
                kind: InlineTargetKind::WikiLink,
                start,
                end,
                source,
                destination,
                title: String::new(),
            });
        } else {
            output.literal(&remaining[open..open + 2 + close + 2], style, source);
        }
        remaining = &after_open[close + 2..];
    }
    output.literal(remaining, style, source);
}

/// Recorre los hijos inline de un nodo acumulando texto y sus tramos con
/// enfasis. `state` baja por el arbol, asi un `**a _b_**` sale con los dos
/// enfasis puestos sobre `b`.
fn inline_into<'a>(
    node: &'a AstNode<'a>,
    state: Emphasis,
    nest: u16,
    source_index: &SourceIndex,
    traversal: &mut TraversalState,
    output: &mut InlineCollector,
) {
    // Tope de anidamiento: ver MAX_NEST.
    if nest > MAX_NEST {
        traversal.mark(Degradation::DepthLimit);
        return;
    }
    for child in node.children() {
        // El prestamo se suelta antes de recurrir: comrak usa RefCell y una
        // recursion con el prestamo vivo entra en panico.
        let value = child.data.borrow().value.clone();

        let child_source = source_index.range_of(child);

        match value {
            NodeValue::Text(t) => output_literal_with_wikilinks(output, &t, state, child_source),
            NodeValue::Code(c) => {
                let mut s = state;
                s.code = true;
                output.literal(&c.literal, s, child_source);
            }
            NodeValue::Emph => {
                let mut s = state;
                s.emph = true;
                inline_into(child, s, nest + 1, source_index, traversal, output);
            }
            NodeValue::Strong => {
                let mut s = state;
                s.strong = true;
                inline_into(child, s, nest + 1, source_index, traversal, output);
            }
            NodeValue::Strikethrough => {
                let mut s = state;
                s.strike = true;
                inline_into(child, s, nest + 1, source_index, traversal, output);
            }
            NodeValue::Highlight => {
                let mut s = state;
                s.mark = true;
                inline_into(child, s, nest + 1, source_index, traversal, output);
            }
            NodeValue::Link(link) => {
                let mut s = state;
                s.link = true;
                let start = output.len();
                inline_into(child, s, nest + 1, source_index, traversal, output);
                let end = output.len();
                if start < end {
                    output.current_mut().targets.push(InlineTarget {
                        kind: InlineTargetKind::Link,
                        start,
                        end,
                        source: child_source,
                        destination: link.url,
                        title: link.title,
                    });
                }
            }
            // Las imagenes son del Sprint 2. Por ahora se anuncia su texto
            // alternativo en vez de desaparecer en silencio.
            NodeValue::Image(link) => {
                let mut alt = InlineCollector::default();
                inline_into(
                    child,
                    Emphasis::default(),
                    nest + 1,
                    source_index,
                    traversal,
                    &mut alt,
                );
                let alt = alt.finish();
                let mut s = state;
                s.code = true;
                let etiqueta = if alt.text.trim().is_empty() {
                    "[imagen]".to_string()
                } else {
                    format!("[imagen: {}]", alt.text.trim())
                };
                let start = output.len();
                output.literal(&etiqueta, s, child_source);
                let end = output.len();
                output.current_mut().targets.push(InlineTarget {
                    kind: InlineTargetKind::Image,
                    start,
                    end,
                    source: child_source,
                    destination: link.url,
                    title: link.title,
                });
            }
            // La referencia se representa como texto compacto nativo. No se
            // genera HTML ni un ancla de navegador; el bloque de definición
            // conserva su propia fuente y se lee al final del documento.
            NodeValue::FootnoteReference(reference) => {
                output.literal(&format!("[{}]", reference.ix), state, child_source);
            }
            NodeValue::HtmlInline(html) => {
                match native_html(&html) {
                    Some(NativeHtml::Break) => output.literal("\n", state, child_source),
                    Some(NativeHtml::Open(semantic)) => output.open(semantic, html, child_source),
                    Some(NativeHtml::Close(semantic)) if output.close(semantic) => {}
                    Some(NativeHtml::Close(_)) | None => {
                        // Un cierre desparejo, atributos o HTML desconocido
                        // no se interpretan. Se ven como fuente monoespaciada.
                        // Nunca llegan al sistema, a una red o a un DOM.
                        let inert = Emphasis {
                            code: true,
                            ..state
                        };
                        output.literal(&html, inert, child_source);
                    }
                }
            }
            NodeValue::SoftBreak => output.literal(" ", state, child_source),
            NodeValue::LineBreak => output.literal("\n", state, child_source),
            _ => inline_into(child, state, nest + 1, source_index, traversal, output),
        }
    }
}

/// Texto y tramos de un nodo, empezando sin enfasis.
fn inline_of<'a>(
    node: &'a AstNode<'a>,
    source_index: &SourceIndex,
    traversal: &mut TraversalState,
) -> (String, Vec<Span>, Vec<InlineTarget>) {
    let mut output = InlineCollector::default();
    inline_into(
        node,
        Emphasis::default(),
        0,
        source_index,
        traversal,
        &mut output,
    );
    let output = output.finish();
    (output.text, output.spans, output.targets)
}

/// Como se marca un item de lista.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Marker {
    /// Vineta o numero, que se maqueta como texto.
    Text(String),
    /// Casilla de tarea, que **se dibuja**, no se escribe. Newsreader no
    /// tiene los glifos U+2610/2611 (ninguna de las tres familias los trae),
    /// asi que depender de un caracter dejaba la casilla invisible. Un
    /// rectangulo y dos lineas no dependen de la cobertura de la fuente.
    Task { done: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CalloutKind {
    Note,
    Info,
    Tip,
    Warning,
    Danger,
}

impl CalloutKind {
    fn label(self) -> &'static str {
        match self {
            Self::Note => "Nota",
            Self::Info => "Info",
            Self::Tip => "Consejo",
            Self::Warning => "Atención",
            Self::Danger => "Peligro",
        }
    }
}

/// Devuelve un callout conocido y cuántos bytes de la primera línea visible se
/// deben omitir. Variantes desconocidas siguen como cita y conservan su fuente
/// literalmente, que es una degradación visible y fiel.
fn callout_prefix(text: &str) -> Option<(CalloutKind, usize)> {
    let rest = text.strip_prefix("[!")?;
    let close = rest.find(']')?;
    let kind = match rest[..close].to_ascii_lowercase().as_str() {
        "note" => CalloutKind::Note,
        "info" => CalloutKind::Info,
        "tip" => CalloutKind::Tip,
        "warning" | "caution" => CalloutKind::Warning,
        "danger" | "important" => CalloutKind::Danger,
        _ => return None,
    };
    let mut prefix = 2 + close + 1;
    if matches!(text.as_bytes().get(prefix), Some(b'+' | b'-')) {
        prefix += 1;
    }
    while matches!(text.as_bytes().get(prefix), Some(b' ' | b'\t')) {
        prefix += 1;
    }
    Some((kind, prefix))
}

fn remove_rendered_prefix(block: &mut Block, prefix: usize) {
    if prefix == 0 || prefix > block.text.len() || !block.text.is_char_boundary(prefix) {
        return;
    }
    block.text.replace_range(..prefix, "");
    block.spans.retain_mut(|span| {
        if span.end <= prefix {
            return false;
        }
        span.start = span.start.saturating_sub(prefix);
        span.end -= prefix;
        span.start < span.end
    });
    block.targets.retain_mut(|target| {
        if target.end <= prefix {
            return false;
        }
        target.start = target.start.saturating_sub(prefix);
        target.end -= prefix;
        target.start < target.end
    });
}

/// Marcador de un item, resuelto contra la lista que lo contiene.
fn marker_for(list: &comrak::nodes::NodeList, index: usize, task: Option<char>) -> Marker {
    if let Some(mark) = task {
        return Marker::Task { done: mark != ' ' };
    }
    match list.list_type {
        comrak::nodes::ListType::Ordered => Marker::Text(format!("{}.", list.start + index)),
        comrak::nodes::ListType::Bullet => Marker::Text("•".into()),
    }
}

/// Dibuja la casilla de una tarea: cuadrado con borde y, si esta hecha,
/// relleno con su tilde.
fn draw_checkbox(pixmap: &mut Pixmap, x: f32, y: f32, size: f32, done: bool, palette: Palette) {
    let borde = palette.dim;
    let acento = palette.accent;
    let fondo = palette.bg;

    let mut paint = Paint {
        anti_alias: true,
        ..Default::default()
    };

    if done {
        // Relleno de acento, con la tilde calada en el color de fondo.
        paint.set_color(Color::from_rgba8(acento.0, acento.1, acento.2, 255));
        if let Some(r) = Rect::from_xywh(x, y, size, size) {
            pixmap.fill_rect(r, &paint, Transform::identity(), None);
        }
        let mut pb = tiny_skia::PathBuilder::new();
        pb.move_to(x + size * 0.24, y + size * 0.52);
        pb.line_to(x + size * 0.43, y + size * 0.71);
        pb.line_to(x + size * 0.78, y + size * 0.29);
        if let Some(path) = pb.finish() {
            let mut tilde = Paint {
                anti_alias: true,
                ..Default::default()
            };
            tilde.set_color(Color::from_rgba8(fondo.0, fondo.1, fondo.2, 255));
            let stroke = tiny_skia::Stroke {
                width: (size * 0.14).max(1.2),
                line_cap: tiny_skia::LineCap::Round,
                line_join: tiny_skia::LineJoin::Round,
                ..Default::default()
            };
            pixmap.stroke_path(&path, &tilde, &stroke, Transform::identity(), None);
        }
    } else {
        // Solo el contorno: cuatro filetes finos, sin relleno.
        paint.set_color(Color::from_rgba8(borde.0, borde.1, borde.2, 255));
        let g = (size * 0.10).max(1.0);
        for r in [
            Rect::from_xywh(x, y, size, g),
            Rect::from_xywh(x, y + size - g, size, g),
            Rect::from_xywh(x, y, g, size),
            Rect::from_xywh(x + size - g, y, g, size),
        ]
        .into_iter()
        .flatten()
        {
            pixmap.fill_rect(r, &paint, Transform::identity(), None);
        }
    }
}

fn flatten<'a>(
    node: &'a AstNode<'a>,
    depth: u8,
    nest: u16,
    source_index: &SourceIndex,
    traversal: &mut TraversalState,
    out: &mut Vec<Block>,
) {
    // Tope de anidamiento: ver MAX_NEST. Sin esto, `> `.repeat(5000) produce
    // 5000 niveles de recursion y el proceso muere por desbordar la pila.
    if nest > MAX_NEST {
        traversal.mark(Degradation::DepthLimit);
        return;
    }
    if out.len() >= MAX_BLOCKS {
        traversal.mark(Degradation::BlockLimit);
        return;
    }
    for child in node.children() {
        if out.len() >= MAX_BLOCKS {
            traversal.mark(Degradation::BlockLimit);
            break;
        }
        let value = child.data.borrow().value.clone();
        match value {
            NodeValue::Heading(h) => {
                let (text, spans, targets) = inline_of(child, source_index, traversal);
                push(
                    out,
                    text,
                    spans,
                    Kind::Heading(h.level),
                    None,
                    source_index.range_of(child),
                    targets,
                );
            }
            NodeValue::Paragraph => {
                let (text, spans, targets) = inline_of(child, source_index, traversal);
                // Un parrafo dentro de un item de lista se dibuja como item.
                let kind = if depth == 0 {
                    Kind::Para
                } else {
                    Kind::Item(depth)
                };
                push(
                    out,
                    text,
                    spans,
                    kind,
                    None,
                    source_index.range_of(child),
                    targets,
                );
            }
            // La lista reparte marcadores a sus items; el item solo aporta
            // profundidad. Hacerlo aca es lo que permite numerar bien las
            // ordenadas, porque el indice lo conoce la lista, no el item.
            NodeValue::List(list) => {
                for (index, item) in child.children().enumerate() {
                    let task = match &item.data.borrow().value {
                        NodeValue::TaskItem(t) => Some(t.symbol.unwrap_or(' ')),
                        _ => None,
                    };
                    let marker = marker_for(&list, index, task);
                    let antes = out.len();
                    flatten(
                        item,
                        depth.saturating_add(1),
                        nest + 1,
                        source_index,
                        traversal,
                        out,
                    );
                    // El marcador va al primer bloque que produjo el item.
                    if let Some(primero) = out.get_mut(antes) {
                        primero.marker = Some(marker);
                    }
                }
            }
            // La lista ya incremento `depth` al entrar al item. El nodo Item
            // es solo un contenedor; sumarle otra vez provocaba que una lista de
            // primer nivel se dibujara con sangria de segundo nivel.
            NodeValue::Item(_) | NodeValue::TaskItem(_) => {
                flatten(child, depth, nest + 1, source_index, traversal, out)
            }
            NodeValue::CodeBlock(cb) => {
                let empty_line = std::iter::once("").filter(|_| cb.literal.is_empty());
                for line in empty_line.chain(cb.literal.lines()) {
                    if out.len() >= MAX_BLOCKS {
                        traversal.mark(Degradation::BlockLimit);
                        break;
                    }
                    let mut block = Block::new(
                        line.to_string(),
                        Vec::new(),
                        Kind::Code,
                        source_index.range_of(child),
                        Vec::new(),
                    );
                    block.code_info = (!cb.info.is_empty()).then(|| cb.info.clone());
                    out.push(block);
                }
            }
            NodeValue::HtmlBlock(html) => {
                for line in html.literal.lines() {
                    if out.len() >= MAX_BLOCKS {
                        traversal.mark(Degradation::BlockLimit);
                        break;
                    }
                    out.push(Block::new(
                        line.to_string(),
                        Vec::new(),
                        Kind::Code,
                        source_index.range_of(child),
                        Vec::new(),
                    ));
                }
            }
            NodeValue::Table(table) => {
                let before = out.len();
                flatten(child, depth, nest + 1, source_index, traversal, out);
                let alignments: Vec<_> = table
                    .alignments
                    .iter()
                    .copied()
                    .map(CellAlignment::from)
                    .collect();
                for block in &mut out[before..] {
                    if matches!(block.kind, Kind::TableRow { .. }) {
                        block.table_alignments = alignments.clone();
                    }
                }
            }
            NodeValue::TableRow(header) => {
                let mut text = String::new();
                let mut spans = Vec::new();
                let mut targets = Vec::new();
                let mut cells = Vec::new();
                for cell in child.children() {
                    let (cell_text, mut cell_spans, mut cell_targets) =
                        inline_of(cell, source_index, traversal);
                    // La celda necesita sus rangos locales para ser maquetada
                    // sola; la fila conserva además copias desplazadas para
                    // selección/copia del modelo aplanado existente.
                    let local_spans = cell_spans.clone();
                    let local_targets = cell_targets.clone();
                    if !text.is_empty() {
                        text.push_str("  |  ");
                    }
                    let offset = text.len();
                    text.push_str(&cell_text);
                    for span in &mut cell_spans {
                        span.start += offset;
                        span.end += offset;
                    }
                    for target in &mut cell_targets {
                        target.start += offset;
                        target.end += offset;
                    }
                    cells.push(TableCell {
                        source: source_index.range_of(cell),
                        text: cell_text,
                        spans: local_spans,
                        targets: local_targets,
                    });
                    spans.extend(cell_spans);
                    targets.extend(cell_targets);
                }
                let mut block = Block::new(
                    text,
                    spans,
                    Kind::TableRow { header },
                    source_index.range_of(child),
                    targets,
                );
                block.table_cells = cells;
                out.push(block);
            }
            NodeValue::BlockQuote => {
                // Se recurre para conservar la estructura interna (varios
                // parrafos, listas dentro de la cita) en vez de aplastarla.
                let antes = out.len();
                flatten(child, depth, nest + 1, source_index, traversal, out);
                let callout = out.get(antes).and_then(|block| callout_prefix(&block.text));
                if let Some((kind, prefix)) = callout {
                    remove_rendered_prefix(&mut out[antes], prefix);
                    if out[antes].text.trim().is_empty() {
                        out.remove(antes);
                    }
                    if let Some(first) = out.get_mut(antes)
                        && first.marker.is_none()
                    {
                        first.marker = Some(Marker::Text(kind.label().to_owned()));
                    }
                    for block in &mut out[antes..] {
                        block.kind = Kind::Callout;
                    }
                }
                for block in &mut out[antes..] {
                    block.quote_depth = block.quote_depth.saturating_add(1);
                    if matches!(block.kind, Kind::Para) {
                        block.kind = Kind::Quote;
                    }
                }
            }
            NodeValue::ThematicBreak => out.push(Block::new(
                String::new(),
                Vec::new(),
                Kind::Rule,
                source_index.range_of(child),
                Vec::new(),
            )),
            _ => flatten(child, depth, nest + 1, source_index, traversal, out),
        }
    }
}

fn push(
    out: &mut Vec<Block>,
    text: String,
    spans: Vec<Span>,
    kind: Kind,
    marker: Option<Marker>,
    source: SourceRange,
    targets: Vec<InlineTarget>,
) {
    if !text.trim().is_empty() {
        let mut block = Block::new(text, spans, kind, source, targets);
        block.marker = marker;
        out.push(block);
    }
}

/// Comprueba invariantes del modelo antes de entregarlo a layout. Un rango
/// corrupto no debe llegar hasta parley ni quedar latente para el editor.
fn validate_model(source: &str, blocks: &[Block]) -> Result<(), &'static str> {
    for block in blocks {
        if !block.source.is_valid_for(source) {
            return Err("rango de fuente invalido en bloque");
        }
        for span in &block.spans {
            if span.start >= span.end
                || span.end > block.text.len()
                || !block.text.is_char_boundary(span.start)
                || !block.text.is_char_boundary(span.end)
            {
                return Err("rango renderizado invalido en tramo inline");
            }
            if !span.source.is_valid_for(source) {
                return Err("rango de fuente invalido en tramo inline");
            }
        }
        for target in &block.targets {
            if target.start >= target.end
                || target.end > block.text.len()
                || !block.text.is_char_boundary(target.start)
                || !block.text.is_char_boundary(target.end)
            {
                return Err("rango renderizado invalido en destino inline");
            }
            if !target.source.is_valid_for(source) {
                return Err("rango de fuente invalido en destino inline");
            }
        }
        if block
            .table_cells
            .iter()
            .any(|cell| !cell.source.is_valid_for(source))
        {
            return Err("rango de fuente invalido en celda");
        }
        if matches!(block.kind, Kind::TableRow { .. })
            && block.table_alignments.len() != block.table_cells.len()
        {
            return Err("cantidad de alineaciones invalida en tabla");
        }
    }
    Ok(())
}

fn markdown_options() -> Options<'static> {
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.highlight = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options
}

/// Convierte la fuente a bloques inertes para el modo seguro. Se conserva una
/// linea por bloque para que virtualizar siga siendo posible y no haya que
/// maquetar un archivo entero solo porque el render enriquecido se degrado.
fn safe_source_blocks(
    source: &str,
    source_index: &SourceIndex,
) -> Result<Vec<Block>, &'static str> {
    if source.is_empty() {
        return Ok(vec![Block::new(
            String::new(),
            Vec::new(),
            Kind::Code,
            SourceRange::default(),
            Vec::new(),
        )]);
    }

    let mut blocks = Vec::new();
    let mut start = 0;
    for raw in source.split_inclusive('\n') {
        let end = start + raw.len();
        let without_lf = raw.strip_suffix('\n').unwrap_or(raw);
        let text = without_lf.strip_suffix('\r').unwrap_or(without_lf);
        let mut chunk_start = 0;
        while chunk_start < text.len() || (text.is_empty() && chunk_start == 0) {
            if blocks.len() >= MAX_BLOCKS {
                return Err("demasiadas lineas para la vista segura");
            }
            let mut chunk_end = (chunk_start + MAX_SAFE_LINE_BYTES).min(text.len());
            while chunk_end > chunk_start && !text.is_char_boundary(chunk_end) {
                chunk_end -= 1;
            }
            // Un carácter Unicode siempre cabe dentro de este límite; esta rama
            // solo protege la invariante si el valor se modifica en el futuro.
            if chunk_end == chunk_start && chunk_start < text.len() {
                chunk_end = text[chunk_start..]
                    .char_indices()
                    .nth(1)
                    .map_or(text.len(), |(offset, _)| chunk_start + offset);
            }
            let is_last = chunk_end == text.len();
            let range = SourceRange {
                start: start + chunk_start,
                end: if is_last { end } else { start + chunk_end },
            };
            debug_assert!(range.is_valid_for(source));
            blocks.push(Block::new(
                text[chunk_start..chunk_end].to_string(),
                Vec::new(),
                Kind::Code,
                range,
                Vec::new(),
            ));
            if is_last {
                break;
            }
            chunk_start = chunk_end;
        }
        start = end;
    }

    // Evita que el parametro quede puramente documental: el indice y los
    // rangos deben coincidir también en la ultima linea.
    debug_assert_eq!(source_index.len, source.len());
    Ok(blocks)
}

fn safe_buffer_blocks(source: &TextBuffer) -> Result<Vec<Block>, &'static str> {
    if source.is_empty() {
        return Ok(vec![Block::new(
            String::new(),
            Vec::new(),
            Kind::Code,
            SourceRange::default(),
            Vec::new(),
        )]);
    }

    let mut blocks = Vec::new();
    let mut start = 0;
    for line in source.lines() {
        let raw = line.to_string();
        let end = start + raw.len();
        let without_lf = raw.strip_suffix('\n').unwrap_or(&raw);
        let text = without_lf.strip_suffix('\r').unwrap_or(without_lf);
        let mut chunk_start = 0;
        while chunk_start < text.len() || (text.is_empty() && chunk_start == 0) {
            if blocks.len() >= MAX_BLOCKS {
                return Err("demasiadas lineas para la vista segura");
            }
            let mut chunk_end = (chunk_start + MAX_SAFE_LINE_BYTES).min(text.len());
            while chunk_end > chunk_start && !text.is_char_boundary(chunk_end) {
                chunk_end -= 1;
            }
            if chunk_end == chunk_start && chunk_start < text.len() {
                chunk_end = text[chunk_start..]
                    .char_indices()
                    .nth(1)
                    .map_or(text.len(), |(offset, _)| chunk_start + offset);
            }
            let is_last = chunk_end == text.len();
            let range = SourceRange {
                start: start + chunk_start,
                end: if is_last { end } else { start + chunk_end },
            };
            debug_assert!(range.is_valid_for_buffer(source));
            blocks.push(Block::new(
                text[chunk_start..chunk_end].to_string(),
                Vec::new(),
                Kind::Code,
                range,
                Vec::new(),
            ));
            if is_last {
                break;
            }
            chunk_start = chunk_end;
        }
        start = end;
    }
    debug_assert_eq!(start, source.len_bytes());
    Ok(blocks)
}

/// Contenido mínimo de una sesión cuya apertura falló. No incorpora el error
/// del sistema: puede contener una ruta, un permiso o información que no hace
/// falta mostrar en el lienzo. El detalle queda en el registro de sesión.
fn opening_failure_blocks() -> (TextBuffer, Vec<Block>) {
    let source = "No se pudo abrir este documento. Revisa que exista, que sea legible y que no supere el límite de apertura.".to_string();
    let block = Block::new(
        source.clone(),
        Vec::new(),
        Kind::Para,
        SourceRange {
            start: 0,
            end: source.len(),
        },
        Vec::new(),
    );
    (TextBuffer::from_text(&source), vec![block])
}

fn parse_blocks(source: &str) -> Result<ParseOutcome, &'static str> {
    let source_index = SourceIndex::new(source);
    if source
        .split_inclusive('\n')
        .any(|line| line.len() > MAX_SAFE_LINE_BYTES)
    {
        return Ok(ParseOutcome {
            blocks: safe_source_blocks(source, &source_index)?,
            degradation: Some(Degradation::LineLimit),
        });
    }
    let arena = Arena::new();
    let options = markdown_options();
    let root = parse_document(&arena, source, &options);
    let mut traversal = TraversalState::default();
    let mut blocks = Vec::new();
    flatten(root, 0, 0, &source_index, &mut traversal, &mut blocks);

    let degradation = traversal.degradation;
    if degradation.is_some() {
        blocks = safe_source_blocks(source, &source_index)?;
    }
    validate_model(source, &blocks)?;

    Ok(ParseOutcome {
        blocks,
        degradation,
    })
}

// ---------------------------------------------------------------- maquetado

/// Posicion resuelta de un bloque. **No guarda el `Layout`**: mantener 43 mil
/// layouts de parley vivos costaba 393 MB en la primera medicion del Sprint 0.
/// Lo unico que hay que recordar por bloque es donde va y cuanto mide; el
/// layout se reconstruye para los pocos bloques visibles y se cachea.
struct Slot {
    y: f32,
    height: f32,
    x: f32,
    kind: Kind,
}

/// Un bloque visible puede ser un texto normal o una fila de tabla. Conservar
/// los layouts de las celdas evita convertir una tabla de vuelta en una línea
/// con separadores y permite ajustar cada columna dentro de su propio ancho.
enum CachedBlockLayout {
    Text(Box<Layout<Brush>>),
    Table(Vec<Layout<Brush>>),
}

const TABLE_CELL_PADDING: f32 = 8.0;

fn table_cell_advance(width: f32, scale: f32, columns: usize) -> f32 {
    let table_width = (width - MARGIN * scale * 2.0).min(MAX_MEASURE * scale);
    ((table_width / columns.max(1) as f32) - TABLE_CELL_PADDING * 2.0).max(1.0)
}

/// Busca el tramo visible sin recorrer todos los bloques en cada cuadro.
/// `slots` esta ordenado por `y`, por lo que dos busquedas binarias hacen que
/// el trabajo de scroll dependa de lo visible, no del largo del documento.
fn visible_range(slots: &[Slot], view_top: f32, view_bottom: f32) -> std::ops::Range<usize> {
    let start = slots.partition_point(|slot| slot.y + slot.height < view_top);
    let end = start + slots[start..].partition_point(|slot| slot.y <= view_bottom);
    start..end
}

fn max_scroll(doc_height: f32, viewport_height: f32) -> f32 {
    (doc_height - viewport_height).max(0.0)
}

fn layout_width_is_stale(laid_for_width: f32, viewport_width: f32) -> bool {
    (laid_for_width - viewport_width).abs() > 0.5
}

fn content_layout_width(viewport_width: f32, mode: DocumentMode) -> f32 {
    if mode == DocumentMode::Split {
        (viewport_width / 2.0).max(1.0)
    } else {
        viewport_width.max(1.0)
    }
}

fn selection_scroll_delta(pointer_y: f32, viewport_height: f32) -> f32 {
    if viewport_height <= SELECTION_SCROLL_EDGE * 2.0 {
        return 0.0;
    }
    if pointer_y < SELECTION_SCROLL_EDGE {
        let intensity = (SELECTION_SCROLL_EDGE - pointer_y).clamp(0.0, SELECTION_SCROLL_EDGE)
            / SELECTION_SCROLL_EDGE;
        return -SELECTION_SCROLL_MAX_STEP * intensity.max(0.2);
    }
    if pointer_y > viewport_height - SELECTION_SCROLL_EDGE {
        let intensity = (pointer_y - (viewport_height - SELECTION_SCROLL_EDGE))
            .clamp(0.0, SELECTION_SCROLL_EDGE)
            / SELECTION_SCROLL_EDGE;
        return SELECTION_SCROLL_MAX_STEP * intensity.max(0.2);
    }
    0.0
}

fn window_title(
    path: &str,
    safe_mode: Option<Degradation>,
    dirty: bool,
    notice: Option<&str>,
) -> String {
    let mode = match safe_mode {
        Some(Degradation::TextOnly) => " · texto inerte",
        Some(_) => " · modo seguro",
        None => "",
    };
    let notice = notice
        .map(|notice| format!(" · {notice}"))
        .unwrap_or_default();
    let dirty = if dirty { " *" } else { "" };
    format!("Visor MD v2 · {path}{dirty}{mode}{notice}")
}

fn safe_mode_label(reason: Degradation) -> String {
    format!(
        "Modo seguro: {}. Se muestra la fuente inerte.",
        reason.explanation()
    )
}

fn build_layout(
    block: &Block,
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<Brush>,
    width: f32,
    scale: f32,
    palette: Palette,
) -> Layout<Brush> {
    build_layout_with_advance(block, font_cx, layout_cx, width, scale, palette, None)
}

fn build_layout_with_advance(
    block: &Block,
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<Brush>,
    width: f32,
    scale: f32,
    palette: Palette,
    explicit_advance: Option<f32>,
) -> Layout<Brush> {
    let (size, weight, role, mono) = block.kind.style();
    let color = palette.resolve(role);
    let advance = explicit_advance.unwrap_or_else(|| {
        (width - MARGIN * scale * 2.0 - block.indent() * scale)
            .clamp(80.0 * scale, MAX_MEASURE * scale)
    });

    // Nombre embebido primero, generico del sistema como red de seguridad si
    // el registro fallara. Ver docs/design.md, "Contraste editorial".
    let stack: &[FontFamilyName] = if mono {
        &[
            FontFamilyName::Named(std::borrow::Cow::Borrowed(FONT_CODE)),
            FontFamilyName::Generic(GenericFamily::Monospace),
        ]
    } else {
        &[
            FontFamilyName::Named(std::borrow::Cow::Borrowed(FONT_DOC)),
            FontFamilyName::Generic(GenericFamily::SystemUi),
        ]
    };

    let mut builder = layout_cx.ranged_builder(font_cx, &block.text, 1.0, true);
    builder.push_default(StyleProperty::Brush(Brush::text(color)));
    builder.push_default(StyleProperty::FontFamily(FontFamily::List(
        std::borrow::Cow::Borrowed(stack),
    )));
    builder.push_default(StyleProperty::FontSize(size * scale));
    builder.push_default(StyleProperty::FontWeight(FontWeight::new(weight)));
    builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(
        block.kind.line_height(),
    )));

    // Enfasis inline. Cada tramo aplica solo lo suyo, encima del estilo base
    // del bloque, asi un `**negrita**` dentro de un encabezado sigue siendo
    // del tamano del encabezado.
    let mono_stack: &[FontFamilyName] = &[
        FontFamilyName::Named(std::borrow::Cow::Borrowed(FONT_CODE)),
        FontFamilyName::Generic(GenericFamily::Monospace),
    ];
    for span in &block.spans {
        let range = span.start..span.end;
        if span.style.strong {
            // Sobre un encabezado que ya es 700, sube a 800 para que la
            // negrita se distinga de su alrededor en vez de desaparecer.
            let peso = if weight >= 700.0 { 800.0 } else { 700.0 };
            builder.push(
                StyleProperty::FontWeight(FontWeight::new(peso)),
                range.clone(),
            );
        }
        if span.style.emph {
            builder.push(StyleProperty::FontStyle(FontStyle::Italic), range.clone());
        }
        if span.style.code {
            builder.push(
                StyleProperty::FontFamily(FontFamily::List(std::borrow::Cow::Borrowed(mono_stack))),
                range.clone(),
            );
            // El monoespaciado se ve mas grande al mismo cuerpo: se compensa.
            builder.push(StyleProperty::FontSize(size * 0.92 * scale), range.clone());
            let c = palette.accent;
            builder.push(StyleProperty::Brush(Brush::text(c)), range.clone());
        }
        if span.style.link {
            let c = palette.accent;
            builder.push(StyleProperty::Brush(Brush::text(c)), range.clone());
            builder.push(StyleProperty::Underline(true), range.clone());
        }
        if span.style.strike {
            builder.push(StyleProperty::Strikethrough(true), range.clone());
        }
        if span.style.kbd {
            builder.push(
                StyleProperty::FontFamily(FontFamily::List(std::borrow::Cow::Borrowed(mono_stack))),
                range.clone(),
            );
            builder.push(StyleProperty::FontSize(size * 0.86 * scale), range.clone());
        }
        if span.style.sub || span.style.sup {
            builder.push(StyleProperty::FontSize(size * 0.72 * scale), range.clone());
        }
        if span.style.kbd || span.style.mark || span.style.sub || span.style.sup {
            let foreground = if span.style.code || span.style.link {
                palette.accent
            } else {
                color
            };
            let background = if span.style.mark {
                Some(palette.mark)
            } else if span.style.kbd {
                Some(palette.kbd)
            } else {
                None
            };
            let shift = if span.style.sup {
                -1
            } else if span.style.sub {
                1
            } else {
                0
            };
            builder.push(
                StyleProperty::Brush(Brush::semantic(foreground, background, shift)),
                range,
            );
        }
    }

    // El parser retiene el destino de cada enlace separado de sus estilos.
    // Aplicarlo al final hace que su semántica visual gane sobre el verde
    // genérico de `span.style.link`, sin volver al documento una capacidad.
    for target in &block.targets {
        if target.kind.is_navigable() {
            builder.push(
                StyleProperty::Brush(Brush::text(link_color(palette, &target.destination))),
                target.start..target.end,
            );
        }
    }

    let mut layout: Layout<Brush> = builder.build(&block.text);
    layout.break_all_lines(Some(advance));
    layout.align(Alignment::Start, AlignmentOptions::default());
    layout
}

fn build_table_layouts(
    block: &Block,
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<Brush>,
    width: f32,
    scale: f32,
    palette: Palette,
) -> Vec<Layout<Brush>> {
    let columns = block.table_cells.len().max(1);
    let advance = table_cell_advance(width, scale, columns);
    block
        .table_cells
        .iter()
        .enumerate()
        .map(|(column, cell)| {
            let cell_block = Block::new(
                cell.text.clone(),
                cell.spans.clone(),
                block.kind,
                cell.source,
                cell.targets.clone(),
            );
            let mut layout = build_layout_with_advance(
                &cell_block,
                font_cx,
                layout_cx,
                width,
                scale,
                palette,
                Some(advance),
            );
            let alignment = block
                .table_alignments
                .get(column)
                .copied()
                .unwrap_or(CellAlignment::None);
            let alignment = match alignment {
                CellAlignment::None | CellAlignment::Left => Alignment::Start,
                CellAlignment::Center => Alignment::Center,
                CellAlignment::Right => Alignment::End,
            };
            layout.align(alignment, AlignmentOptions::default());
            layout
        })
        .collect()
}

/// Maqueta la vineta o el numero de un item, como pieza aparte del texto.
/// Va aparte a proposito: si el marcador viviera dentro del mismo texto, la
/// segunda linea de un item largo quedaria alineada bajo la vineta en vez de
/// bajo el texto, que es el defecto clasico de las listas mal hechas.
fn build_marker_layout(
    marker: &str,
    kind: Kind,
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<Brush>,
    scale: f32,
    palette: Palette,
) -> Layout<Brush> {
    let (size, _, _, _) = kind.style();
    let c = palette.dim;
    let stack: &[FontFamilyName] = &[
        FontFamilyName::Named(std::borrow::Cow::Borrowed(FONT_DOC)),
        FontFamilyName::Generic(GenericFamily::SystemUi),
    ];

    let mut builder = layout_cx.ranged_builder(font_cx, marker, 1.0, true);
    builder.push_default(StyleProperty::Brush(Brush::text(c)));
    builder.push_default(StyleProperty::FontFamily(FontFamily::List(
        std::borrow::Cow::Borrowed(stack),
    )));
    builder.push_default(StyleProperty::FontSize(size * scale));
    builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(
        kind.line_height(),
    )));

    let mut layout: Layout<Brush> = builder.build(marker);
    layout.break_all_lines(None);
    layout.align(Alignment::Start, AlignmentOptions::default());
    layout
}

fn build_menu_layout(
    label: &str,
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<Brush>,
    palette: Palette,
) -> Layout<Brush> {
    let stack: &[FontFamilyName] = &[
        FontFamilyName::Named(std::borrow::Cow::Borrowed(FONT_UI)),
        FontFamilyName::Generic(GenericFamily::SystemUi),
    ];
    let mut builder = layout_cx.ranged_builder(font_cx, label, 1.0, true);
    builder.push_default(StyleProperty::Brush(Brush::text(palette.text)));
    builder.push_default(StyleProperty::FontFamily(FontFamily::List(
        std::borrow::Cow::Borrowed(stack),
    )));
    builder.push_default(StyleProperty::FontSize(13.0));
    let mut layout: Layout<Brush> = builder.build(label);
    layout.break_all_lines(None);
    layout
}

/// Representacion cacheada de un marcador visible. Los marcadores de texto
/// usan parley; las tareas conservan solo su estado y se dibujan con tiny-skia.
enum CachedMarker {
    Text(Box<Layout<Brush>>),
    Task { done: bool },
}

/// Alto aproximado de un bloque **sin maquetarlo**: cuenta caracteres y
/// estima cuantos entran por linea. No sirve para dibujar, solo para saber
/// donde cae cada bloque en la barra de scroll.
fn estimate_height(block: &Block, width: f32, scale: f32) -> f32 {
    // La linea horizontal no tiene texto: su alto es el del filete.
    if matches!(block.kind, Kind::Rule) {
        return scale;
    }
    let (size, _, _, mono) = block.kind.style();
    let advance = (width - MARGIN * scale * 2.0 - block.indent() * scale)
        .clamp(80.0 * scale, MAX_MEASURE * scale);
    // Ancho medio de caracter como fraccion del tamano de fuente. Aproximado
    // a proposito: el error se corrige al maquetar de verdad el bloque.
    let char_w = size * scale * if mono { 0.60 } else { 0.50 };
    let per_line = (advance / char_w).max(1.0);
    let lines = (block.text.chars().count() as f32 / per_line)
        .ceil()
        .max(1.0);
    lines * size * scale * block.kind.line_height()
}

/// Pasada de posicionamiento. Con `exact`, maqueta cada bloque para sacarle el
/// alto real y descarta el layout. Sin `exact`, lo estima sin maquetar: en un
/// documento de 43 mil bloques la diferencia es de segundos a milisegundos.
fn measure_all(
    blocks: &[Block],
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<Brush>,
    width: f32,
    scale: f32,
    exact: bool,
) -> (Vec<Slot>, f32) {
    let mut slots = Vec::with_capacity(blocks.len());
    let mut y = MARGIN * scale;

    for block in blocks {
        let height = if matches!(block.kind, Kind::Rule) {
            scale
        } else if matches!(block.kind, Kind::TableRow { .. }) {
            // Una fila se mide siempre por celda. Estimar desde el texto
            // aplanado permitiría que el contenido se pinte sobre la fila
            // siguiente al partirse una columna estrecha.
            build_table_layouts(block, font_cx, layout_cx, width, scale, NIGHT)
                .iter()
                .map(Layout::height)
                .fold(0.0_f32, f32::max)
                .max(1.0)
        } else if exact {
            // El color no afecta el alto: la paleta es irrelevante aca.
            build_layout(block, font_cx, layout_cx, width, scale, NIGHT).height()
        } else {
            estimate_height(block, width, scale)
        };
        y += block.kind.space_before() * scale;
        slots.push(Slot {
            y,
            height,
            x: MARGIN * scale + block.indent() * scale,
            kind: block.kind,
        });
        y += height;
    }

    (slots, y + MARGIN * scale)
}

// ---------------------------------------------------------------- dibujo

fn blend(px: &mut PremultipliedColorU8, color: (u8, u8, u8), alpha: u8) {
    if alpha == 0 {
        return;
    }
    let a = alpha as u32;
    let inv = 255 - a;
    let mix = |src: u8, dst: u8| ((src as u32 * a + dst as u32 * inv) / 255) as u8;
    let r = mix(color.0, px.red());
    let g = mix(color.1, px.green());
    let b = mix(color.2, px.blue());
    // El fondo es opaco, asi que el resultado tambien lo es: premultiplicar
    // con alpha 255 es la identidad y `from_rgba` no puede fallar.
    if let Some(out) = PremultipliedColorU8::from_rgba(r, g, b, 255) {
        *px = out;
    }
}

/// Cantidad de ejes de variación que se conservan en la clave de caché. Las
/// fuentes embebidas usan como máximo dos; una fuente ajena con más ejes se
/// dibuja sin caché antes que arriesgar reutilizar una forma equivocada.
const MAX_CACHED_VARIATION_COORDS: usize = 16;

/// Identidad de un glifo ya rasterizado. Sin posición subpixel: parley ya
/// entrega las posiciones alineadas a pixel (`quantize = true`). El tamaño no
/// basta para una fuente variable: peso, cursiva y otros ejes también cambian
/// la máscara del mismo identificador de glifo.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GlyphKey {
    blob: u64,
    index: u32,
    size: u32,
    glyph: u16,
    coord_len: u8,
    coords: [i16; MAX_CACHED_VARIATION_COORDS],
}

fn glyph_key(
    blob: u64,
    index: u32,
    size: f32,
    glyph: u16,
    normalized_coords: &[i16],
) -> Option<GlyphKey> {
    if normalized_coords.len() > MAX_CACHED_VARIATION_COORDS {
        return None;
    }
    let mut coords = [0; MAX_CACHED_VARIATION_COORDS];
    coords[..normalized_coords.len()].copy_from_slice(normalized_coords);
    Some(GlyphKey {
        blob,
        index,
        size: size.to_bits(),
        glyph,
        coord_len: normalized_coords.len() as u8,
        coords,
    })
}

/// Imagen de un glifo lista para mezclar. Las fuentes de texto normales dan
/// una máscara; los emoji pueden ser mapas RGBA y deben conservar sus colores.
struct CachedGlyph {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    content: Content,
    data: Vec<u8>,
}

type GlyphCache = HashMap<GlyphKey, Option<CachedGlyph>>;

/// Fondo discreto de `mark` y `kbd`. Se pinta por run antes de texto y
/// decoraciones, sin crear cajas interactivas ni interpretar atributos HTML.
fn draw_run_background(
    pixmap: &mut Pixmap,
    run: &GlyphRun<'_, Brush>,
    origin_x: f32,
    origin_y: f32,
) {
    let brush = run.style().brush;
    let Some(color) = brush.background else {
        return;
    };
    let metrics = run.run().metrics();
    let pad_x = 2.0;
    let pad_y = 1.0;
    let x = origin_x + run.offset() - pad_x;
    let y = origin_y + run.baseline() - metrics.ascent - pad_y;
    let width = run.advance() + pad_x * 2.0;
    let height = metrics.ascent + metrics.descent + pad_y * 2.0;
    let Some(rect) = Rect::from_xywh(x, y, width, height) else {
        return;
    };
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgba8(color.0, color.1, color.2, 255));
    pixmap.fill_rect(rect, &paint, Transform::identity(), None);
}

fn draw_glyph_run(
    pixmap: &mut Pixmap,
    scale_cx: &mut ScaleContext,
    cache: &mut GlyphCache,
    run: &GlyphRun<'_, Brush>,
    origin_x: f32,
    origin_y: f32,
) {
    let mut run_x = run.offset();
    let run_y = run.baseline();
    let brush = run.style().brush;
    let color = brush.foreground;

    let r = run.run();
    let font = r.font();
    let font_size = r.font_size();
    let blob = font.data.id();
    let index = font.index;

    let width = pixmap.width() as i32;
    let height = pixmap.height() as i32;

    // El scaler se construye solo si algun glifo del run falta en la cache.
    let mut scaler = None;

    for glyph in run.glyphs() {
        let gx = origin_x + run_x + glyph.x;
        let shift = brush.baseline_shift as f32 * font_size * 0.25;
        let gy = origin_y + run_y + glyph.y + shift;
        run_x += glyph.advance;

        // Fuera de la ventana no se rasteriza. Esta es la virtualizacion real:
        // lo que hace que un documento enorme no cueste de mas al dibujar.
        if gy < -80.0 || gy > height as f32 + 80.0 {
            continue;
        }

        let key = glyph_key(
            blob,
            index,
            font_size,
            glyph.id as u16,
            r.normalized_coords(),
        );
        let needs_raster = key.is_none_or(|key| !cache.contains_key(&key));
        let mut uncached = None;

        if needs_raster {
            if scaler.is_none() {
                let Some(font_ref) = FontRef::from_index(font.data.as_ref(), index as usize) else {
                    return;
                };
                scaler = Some(
                    scale_cx
                        .builder(font_ref)
                        .size(font_size)
                        .hint(true)
                        .normalized_coords(r.normalized_coords())
                        .build(),
                );
            }
            let Some(s) = scaler.as_mut() else {
                return;
            };

            let rendered = Render::new(&[
                Source::ColorOutline(0),
                Source::ColorBitmap(StrikeWith::BestFit),
                Source::Outline,
            ])
            .format(Format::Alpha)
            .offset(Vector::new(0.0, 0.0))
            .render(s, glyph.id as u16);

            let cached = match rendered {
                Some(image) if matches!(image.content, Content::Mask | Content::Color) => {
                    Some(CachedGlyph {
                        left: image.placement.left,
                        top: image.placement.top,
                        width: image.placement.width as i32,
                        height: image.placement.height as i32,
                        content: image.content,
                        data: image.data,
                    })
                }
                _ => None,
            };
            if let Some(key) = key {
                cache.insert(key, cached);
            } else {
                uncached = cached;
            }
        }

        let cached = match key {
            Some(key) => cache.get(&key).and_then(Option::as_ref),
            None => uncached.as_ref(),
        };
        let Some(g) = cached else {
            continue;
        };
        match g.content {
            Content::Mask => blit(pixmap, g, gx, gy, color, width, height),
            Content::Color => blit_color(pixmap, g, gx, gy, width, height),
            Content::SubpixelMask => {}
        }
    }
}

/// Subrayado y tachado. parley resuelve *si* van y *donde*, leyendo las
/// metricas de la fuente (cada tipografia dice a que altura corre su propio
/// subrayado); dibujarlos es cosa nuestra. Sin esto, un enlace se veia solo
/// coloreado y un `~~tachado~~` no se distinguia del texto normal.
fn draw_decorations(pixmap: &mut Pixmap, run: &GlyphRun<'_, Brush>, origin_x: f32, origin_y: f32) {
    let style = run.style();
    let metrics = run.run().metrics();
    let shift = style.brush.baseline_shift as f32 * run.run().font_size() * 0.25;

    let mut trazar = |offset: f32, grosor: f32, brush: Brush| {
        let y = origin_y + run.baseline() + shift - offset;
        let x = origin_x + run.offset();
        // Minimo de 1 px: con cuerpos chicos el grosor calculado puede
        // redondear a cero y la linea desaparece sin aviso.
        let alto = grosor.max(1.0);
        let Some(rect) = Rect::from_xywh(x, y, run.advance(), alto) else {
            return;
        };
        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba8(
            brush.foreground.0,
            brush.foreground.1,
            brush.foreground.2,
            255,
        ));
        paint.anti_alias = false;
        pixmap.fill_rect(rect, &paint, Transform::identity(), None);
    };

    if let Some(d) = &style.underline {
        let offset = d.offset.unwrap_or(metrics.underline_offset);
        let grosor = d.size.unwrap_or(metrics.underline_size);
        trazar(offset, grosor, d.brush);
    }
    if let Some(d) = &style.strikethrough {
        let offset = d.offset.unwrap_or(metrics.strikethrough_offset);
        let grosor = d.size.unwrap_or(metrics.strikethrough_size);
        trazar(offset, grosor, d.brush);
    }
}

fn blit(
    pixmap: &mut Pixmap,
    g: &CachedGlyph,
    gx: f32,
    gy: f32,
    color: (u8, u8, u8),
    width: i32,
    height: i32,
) {
    let x0 = gx.round() as i32 + g.left;
    let y0 = gy.round() as i32 - g.top;

    let pixels = pixmap.pixels_mut();
    for row in 0..g.height {
        let y = y0 + row;
        if y < 0 || y >= height {
            continue;
        }
        for col in 0..g.width {
            let x = x0 + col;
            if x < 0 || x >= width {
                continue;
            }
            let alpha = g.data[(row * g.width + col) as usize];
            blend(&mut pixels[(y * width + x) as usize], color, alpha);
        }
    }
}

/// Compone un emoji RGBA sobre el lienzo opaco. Swash entrega los bitmaps en
/// RGBA y `blend` mantiene la misma composición fuente-sobre-fondo que las
/// máscaras monocromas.
fn blit_color(pixmap: &mut Pixmap, g: &CachedGlyph, gx: f32, gy: f32, width: i32, height: i32) {
    let x0 = gx.round() as i32 + g.left;
    let y0 = gy.round() as i32 - g.top;
    let pixels = pixmap.pixels_mut();
    for row in 0..g.height {
        let y = y0 + row;
        if y < 0 || y >= height {
            continue;
        }
        for col in 0..g.width {
            let x = x0 + col;
            if x < 0 || x >= width {
                continue;
            }
            let offset = ((row * g.width + col) * 4) as usize;
            let Some(rgba) = g.data.get(offset..offset + 4) else {
                continue;
            };
            blend(
                &mut pixels[(y * width + x) as usize],
                (rgba[0], rgba[1], rgba[2]),
                rgba[3],
            );
        }
    }
}

// ---------------------------------------------------------------- app

struct DocumentState {
    id: u64,
    path: String,
    source: TextBuffer,
    source_metadata: TextMetadata,
    source_identity: Option<FileIdentity>,
    source_baseline_bytes: Option<Vec<u8>>,
    source_editor: SourceEditor,
    mode: DocumentMode,
    /// Último modelo Markdown correspondiente a la revisión aceptada. En
    /// edición se conserva separado de la representación inerte de la fuente
    /// para que la vista dividida nunca se convierta en autoridad de guardado.
    rendered_blocks: Vec<Block>,
    blocks: Vec<Block>,
    safe_mode: Option<Degradation>,
    /// Posición visual propia de la pestaña. Se sincroniza antes de cambiar
    /// de documento y se restaura al volver.
    scroll: f32,
    /// Ancla solicitada mientras este documento se prepara. Viaja con la
    /// pestaña para que un cambio de documento no aplique el scroll a otra.
    pending_heading: Option<String>,
    last_recovery: Instant,
}

impl DocumentState {
    fn untitled() -> Self {
        Self {
            id: NEXT_DOCUMENT_ID.fetch_add(1, Ordering::Relaxed),
            path: "sin título.md".to_string(),
            source: TextBuffer::new(),
            source_metadata: TextMetadata::default(),
            source_identity: None,
            source_baseline_bytes: None,
            source_editor: SourceEditor::new(),
            mode: DocumentMode::SourceEditing,
            rendered_blocks: Vec::new(),
            blocks: Vec::new(),
            safe_mode: None,
            scroll: 0.0,
            pending_heading: None,
            last_recovery: Instant::now(),
        }
    }

    fn is_dirty(&self) -> bool {
        self.source_editor.is_dirty()
    }
}

fn recovered_document(source: String) -> Result<DocumentState, &'static str> {
    let index = SourceIndex::new(&source);
    let blocks = safe_source_blocks(&source, &index)?;
    let mut recovered = DocumentState::untitled();
    recovered.path = "recuperación sin guardar.md".to_string();
    recovered.source = TextBuffer::from_text(&source);
    recovered.source_editor.mark_recovered();
    recovered.blocks = blocks;
    Ok(recovered)
}

struct InactiveDocument {
    document: DocumentState,
    recovery: Option<RecoverySession>,
}

fn dirty_document_count(active: &DocumentState, inactive: &[InactiveDocument]) -> usize {
    usize::from(active.is_dirty())
        + inactive
            .iter()
            .filter(|tab| tab.document.is_dirty())
            .count()
}

fn document_by_id_mut<'a>(
    active: &'a mut DocumentState,
    inactive: &'a mut [InactiveDocument],
    document_id: u64,
) -> Option<&'a mut DocumentState> {
    if active.id == document_id {
        return Some(active);
    }
    inactive
        .iter_mut()
        .find(|tab| tab.document.id == document_id)
        .map(|tab| &mut tab.document)
}

fn apply_save_result(
    document: &mut DocumentState,
    revision: u64,
    identity: FileIdentity,
    baseline_bytes: Vec<u8>,
) -> bool {
    document.source_identity = Some(identity);
    document.source_baseline_bytes = Some(baseline_bytes);
    let is_current = revision == document.source_editor.revision();
    if is_current {
        document.source_editor.mark_saved();
    }
    is_current
}

fn apply_view_outcome(document: &mut DocumentState, outcome: ParseOutcome) {
    document.rendered_blocks = outcome.blocks;
    if document.mode == DocumentMode::Reading {
        document.blocks = document.rendered_blocks.clone();
    }
    document.safe_mode = outcome.degradation;
}

fn paths_refer_to_same_file(left: &std::path::Path, right: &std::path::Path) -> bool {
    if left == right {
        return true;
    }
    let is_unc = |path: &std::path::Path| {
        let path = path.to_string_lossy();
        path.starts_with("\\\\") || path.starts_with("//")
    };
    // La deduplicación es una comodidad, no permiso para tocar una ubicación
    // de red. Una UNC solo coincide textualmente; nunca se canonicaliza aquí.
    if is_unc(left) || is_unc(right) {
        return false;
    }
    match (fs::canonicalize(left), fs::canonicalize(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

struct App {
    started: Instant,
    document: DocumentState,
    /// Documentos inactivos de la sesión. Cada uno conserva su fuente e
    /// historial; los caches de dibujo siguen perteneciendo a la ventana.
    inactive_documents: Vec<InactiveDocument>,
    /// Orden visible estable de las pestañas, independiente de cuál esté
    /// activa. Los identificadores existen solo durante la sesión.
    tab_order: Vec<u64>,
    external_checks_in_flight: HashSet<u64>,
    /// Identifica las pestañas con una escritura atómica pendiente. El resto
    /// de la sesión puede seguir usándose sin atribuir el resultado a otro
    /// documento.
    saves_in_flight: HashSet<u64>,
    recovery_enabled: bool,
    recovery: Option<RecoverySession>,
    /// Solo la primera sesión informa que la recuperación es texto local sin
    /// cifrar. La decisión no depende del documento abierto.
    recovery_privacy_notice_pending: bool,
    workspace: Option<(WorkspaceRoot, WorkspaceIndex)>,
    /// Cambiar de carpeta cancela cooperativamente el recorrido anterior. El
    /// contador descarta además cualquier resultado tardío del hilo anterior.
    workspace_request: u64,
    workspace_cancel: Option<Arc<AtomicBool>>,
    workspace_check_in_flight: Option<u64>,
    /// La raíz cambió desde el último índice conocido. No se usa un watcher:
    /// la persona decide cuándo reconstruir el índice con Ctrl+Shift+I.
    workspace_stale: bool,
    /// Versión de apertura solicitada. Una tarea terminada tarde no puede
    /// reemplazar el documento que la persona pidió después.
    document_request: u64,
    /// Solicitudes activas por pestaña. Un valor nuevo reemplaza lógicamente
    /// al anterior; los hilos viejos pueden terminar, pero no publicar estado.
    open_requests: HashMap<u64, u64>,
    view_requests: HashMap<u64, u64>,
    proxy: EventLoopProxy<AppEvent>,
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    font_cx: FontContext,
    layout_cx: LayoutContext<Brush>,
    scale_cx: ScaleContext,
    glyphs: GlyphCache,
    pixmap: Option<Pixmap>,
    slots: Vec<Slot>,
    /// Layouts de los bloques visibles, por indice. Se poda cada cuadro.
    live: HashMap<usize, (CachedBlockLayout, Option<CachedMarker>)>,
    doc_height: f32,
    laid_for_width: f32,
    /// Cache independiente de la mitad renderizada. Comparar ambas vistas no
    /// mezcla cursores ni rangos de la fuente con el modelo Markdown.
    preview_slots: Vec<Slot>,
    preview_live: HashMap<usize, (CachedBlockLayout, Option<CachedMarker>)>,
    preview_height: f32,
    preview_laid_for_width: f32,
    scale_factor: f32,
    scroll: f32,
    first_paint_done: bool,
    frames: u32,
    frame_time_total: f64,
    /// Modo medicion: recorre el documento solo y reporta, sin que nadie
    /// tenga que sentarse a arrastrar la rueda del mouse.
    bench: Option<u32>,
    /// Si el alto de cada bloque se saca maquetandolo (exacto y lento) o
    /// estimandolo (aproximado e instantaneo). Ver ADR-16.
    exact_measure: bool,
    /// Una edición puede cambiar mucho el ajuste de líneas. La primera vista
    /// posterior se mide completa para que un slot estimado no superponga
    /// bloques; después vuelve a regir la política normal de virtualización.
    exact_after_edit: bool,
    /// Las mediciones se acumulan y se imprimen al final. Escribir a stderr
    /// en el medio distorsiona lo que se esta midiendo: con la salida
    /// redirigida a un archivo, cada `eprintln` costaba mas que el trabajo
    /// que pretendia cronometrar.
    log: Vec<String>,
    /// Tema activo. Arranca siguiendo al sistema operativo (`Window::theme`);
    /// `T` lo alterna a mano. Ver docs/design.md.
    palette: Palette,
    /// Punto actual del cursor dentro de la ventana, en pixeles físicos.
    pointer: Option<(f32, f32)>,
    /// Mientras está activo, mover el mouse extiende la selección del bloque.
    selecting: bool,
    selection: Option<DocumentSelection>,
    modifiers: ModifiersState,
    text_cursor_hover: bool,
    /// Destino mostrado al pasar por encima de un enlace. No abre ni resuelve
    /// la ruta: solo hace visible lo que el documento ya declaró.
    hover_destination: Option<String>,
    /// Enlace recorrido con Tab. Se separa del hover para que el teclado no
    /// dependa de que el mouse permanezca inmóvil sobre el documento.
    focused_link: Option<(usize, usize)>,
    focus_destination: Option<String>,
    /// Origen del menú contextual propio. Solo contiene acciones de copia
    /// locales; no usa menús del sistema ni ejecuta destinos del documento.
    context_menu: Option<(f32, f32)>,
    /// Consulta efímera del documento abierto. No se persiste ni se comparte.
    search_query: Option<String>,
    search_match: usize,
    /// Consulta efímera sobre el índice local de la carpeta autorizada. Nunca
    /// toca el disco ni conserva la consulta fuera de esta ejecución.
    workspace_search_query: Option<String>,
    workspace_search_match: usize,
    /// Rutas relativas del índice ya autorizado para recorrer una carpeta sin
    /// repetir el recorrido de disco ni aceptar una ruta desde el documento.
    workspace_paths: Option<Vec<WorkspaceTreeRow>>,
    workspace_path_match: usize,
    collapsed_workspace_dirs: HashSet<PathBuf>,
    /// Backlinks del documento actual obtenidos del índice ya autorizado. No
    /// se persisten ni se resuelven contra el disco hasta pulsar Enter.
    backlink_paths: Option<Vec<PathBuf>>,
    backlink_match: usize,
    /// Índice efímero del documento abierto. Mantiene el número de bloque, no
    /// una ruta ni una copia de contenido, y se descarta al navegar.
    outline_headings: Option<Vec<(usize, String)>>,
    outline_match: usize,
    /// Selector efímero de acciones y su filtro local. No se persiste ni
    /// contiene datos del documento.
    command_palette: Option<usize>,
    command_palette_query: String,
    /// Foco visible de la barra superior. F6 lo activa; no sustituye los
    /// atajos directos ni simula semántica de lector de pantalla.
    toolbar_focus: Option<usize>,
    /// Se crea solamente ante una copia explícita. Mantenerlo vivo respeta el
    /// modelo de propiedad de X11/Wayland, donde el proceso sirve el texto
    /// hasta que otra aplicación lo solicita.
    clipboard: Option<Clipboard>,
    /// Confirmación discreta en el título de ventana; no contiene texto del
    /// documento ni rutas adicionales.
    notice: Option<String>,
    /// Conserva el resultado fatal para devolver un codigo de salida distinto
    /// de cero despues de cerrar ordenadamente el event loop.
    fatal_error: bool,
}

impl ApplicationHandler<AppEvent> for App {
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::ViewRequested {
                document_id,
                revision,
            } => {
                if self.view_requests.get(&document_id) != Some(&revision) {
                    return;
                }
                let Some(document) = document_by_id_mut(
                    &mut self.document,
                    &mut self.inactive_documents,
                    document_id,
                ) else {
                    self.view_requests.remove(&document_id);
                    return;
                };
                if document.source_editor.revision() != revision {
                    return;
                }
                let source = document.source.to_string();
                let proxy = self.proxy.clone();
                thread::spawn(move || {
                    let started = Instant::now();
                    let event = match parse_blocks(&source) {
                        Ok(outcome) => AppEvent::ViewReady {
                            document_id,
                            revision,
                            outcome,
                            elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
                        },
                        Err(error) => AppEvent::ViewFailed {
                            document_id,
                            revision,
                            error: format!("no se pudo actualizar la vista: {error}"),
                        },
                    };
                    let _ = proxy.send_event(event);
                });
            }
            AppEvent::DocumentReady {
                document_id,
                request,
                path,
                source,
                metadata,
                identity,
                baseline_bytes,
                outcome,
                elapsed_ms,
            } => {
                if self.open_requests.get(&document_id) != Some(&request) {
                    self.log
                        .push("[apertura] se descartó un documento desactualizado".to_string());
                    return;
                }
                self.open_requests.remove(&document_id);
                let active = document_id == self.document.id;
                let source_len = source.len();
                let block_len = outcome.blocks.len();
                let safe_mode = outcome.degradation;
                let byte_len = identity.byte_len;
                let Some(document) = document_by_id_mut(
                    &mut self.document,
                    &mut self.inactive_documents,
                    document_id,
                ) else {
                    self.log.push(
                        "[apertura] terminó una solicitud para una pestaña ya cerrada".to_string(),
                    );
                    return;
                };
                document.path = path.to_string_lossy().into_owned();
                document.source = TextBuffer::from_text(&source);
                document.source_metadata = metadata;
                document.source_identity = Some(identity);
                document.source_baseline_bytes = Some(baseline_bytes);
                document.source_editor = SourceEditor::new();
                document.mode = DocumentMode::Reading;
                document.rendered_blocks = outcome.blocks;
                document.blocks = document.rendered_blocks.clone();
                document.safe_mode = safe_mode;
                document.scroll = 0.0;
                self.log.push(format!(
                    "[medicion] preparar documento de {:.1} KB fuera de UI: {elapsed_ms:.0} ms  ({} bloques)",
                    source_len as f64 / 1024.0,
                    block_len
                ));
                self.log.push(format!(
                    "[fidelidad] UTF-8{}; EOL {:?}",
                    if metadata.has_utf8_bom {
                        " con BOM"
                    } else {
                        " sin BOM"
                    },
                    metadata.line_endings
                ));
                self.log
                    .push(format!("[fidelidad] huella inicial: {byte_len} bytes"));
                if let Some(reason) = safe_mode {
                    self.log.push(format!(
                        "[seguridad] {}; se muestra la fuente inerte",
                        reason.explanation()
                    ));
                }
                if active {
                    self.external_checks_in_flight.remove(&document_id);
                    self.reset_document_view();
                    self.notice = None;
                    self.refresh_title();
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                } else {
                    self.set_notice("otra pestaña terminó de abrirse");
                }
            }
            AppEvent::DocumentFailed {
                document_id,
                request,
                error,
            } => {
                if self.open_requests.get(&document_id) != Some(&request) {
                    self.log
                        .push("[apertura] se descartó un error desactualizado".to_string());
                    return;
                }
                self.open_requests.remove(&document_id);
                self.external_checks_in_flight.remove(&document_id);
                self.log.push(format!("[error] {error}"));
                if let Some(document) = document_by_id_mut(
                    &mut self.document,
                    &mut self.inactive_documents,
                    document_id,
                ) {
                    (document.source, document.blocks) = opening_failure_blocks();
                }
                if document_id == self.document.id {
                    self.reset_document_view();
                    self.set_notice("no se pudo abrir el documento");
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                } else {
                    self.set_notice("no se pudo abrir otra pestaña");
                }
            }
            AppEvent::ViewReady {
                document_id,
                revision,
                outcome,
                elapsed_ms,
            } => {
                if self.view_requests.get(&document_id) != Some(&revision) {
                    self.log
                        .push("[edición] se descartó una vista desactualizada".to_string());
                    return;
                }
                let Some(document) = document_by_id_mut(
                    &mut self.document,
                    &mut self.inactive_documents,
                    document_id,
                ) else {
                    self.view_requests.remove(&document_id);
                    return;
                };
                if !is_current_view_result(
                    document_id,
                    document.id,
                    revision,
                    document.source_editor.revision(),
                ) {
                    self.log
                        .push("[edición] se descartó una vista desactualizada".to_string());
                    return;
                }
                apply_view_outcome(document, outcome);
                self.view_requests.remove(&document_id);
                self.log.push(format!(
                    "[medicion] actualizar vista fuera de UI: {elapsed_ms:.0} ms"
                ));
                if document_id == self.document.id {
                    if self.document.mode == DocumentMode::Split {
                        self.invalidate_document_layout();
                        self.sync_source_selection();
                    } else {
                        self.reset_document_view();
                    }
                    self.set_notice(if self.document.mode == DocumentMode::Split {
                        "vista dividida actualizada"
                    } else {
                        "vista de lectura actualizada"
                    });
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                } else {
                    self.set_notice("otra pestaña actualizó su vista de lectura");
                }
            }
            AppEvent::ViewFailed {
                document_id,
                revision,
                error,
            } => {
                if self.view_requests.get(&document_id) != Some(&revision) {
                    self.log
                        .push("[edición] se descartó un error de vista desactualizado".to_string());
                    return;
                }
                self.view_requests.remove(&document_id);
                self.log.push(format!("[error] {error}"));
                self.set_notice(if document_id == self.document.id {
                    "no se pudo actualizar la vista; la fuente sigue intacta"
                } else {
                    "otra pestaña no pudo actualizar su vista; la fuente sigue intacta"
                });
            }
            AppEvent::SaveReady {
                document_id,
                revision,
                identity,
                baseline_bytes,
            } => {
                self.saves_in_flight.remove(&document_id);
                let active = document_id == self.document.id;
                let saved_current = document_by_id_mut(
                    &mut self.document,
                    &mut self.inactive_documents,
                    document_id,
                )
                .map(|document| apply_save_result(document, revision, identity, baseline_bytes));
                if saved_current == Some(true) {
                    if active {
                        if let Some(recovery) = &self.recovery {
                            let _ = recovery.clear();
                        }
                        self.set_notice("documento guardado de forma atómica");
                    } else {
                        if let Some(tab) = self
                            .inactive_documents
                            .iter()
                            .find(|tab| tab.document.id == document_id)
                            && let Some(recovery) = &tab.recovery
                        {
                            let _ = recovery.clear();
                        }
                        self.set_notice("otra pestaña terminó de guardarse");
                    }
                } else if saved_current == Some(false) {
                    self.set_notice(if active {
                        "se guardó una versión anterior; hay cambios nuevos"
                    } else {
                        "otra pestaña guardó una versión anterior"
                    });
                } else {
                    self.log.push(
                        "[guardado] terminó una escritura para una pestaña ya cerrada".to_string(),
                    );
                }
            }
            AppEvent::SaveFailed {
                document_id,
                error,
                conflict,
            } => {
                self.saves_in_flight.remove(&document_id);
                self.log.push(format!("[error] {error}"));
                if conflict && document_id == self.document.id {
                    self.resolve_save_conflict();
                } else if document_id != self.document.id {
                    self.set_notice(
                        "falló el guardado de otra pestaña; sus cambios siguen intactos",
                    );
                } else {
                    self.set_notice(&error);
                }
            }
            AppEvent::SaveAsReady {
                document_id,
                path,
                revision,
                identity,
                baseline_bytes,
            } => {
                self.saves_in_flight.remove(&document_id);
                let active = document_id == self.document.id;
                let saved_current = document_by_id_mut(
                    &mut self.document,
                    &mut self.inactive_documents,
                    document_id,
                )
                .map(|document| {
                    document.path = path.to_string_lossy().into_owned();
                    apply_save_result(document, revision, identity, baseline_bytes)
                });
                if saved_current == Some(true) {
                    if active {
                        if let Some(recovery) = &self.recovery {
                            let _ = recovery.clear();
                        }
                        self.set_notice("documento creado y guardado de forma atómica");
                    } else {
                        if let Some(tab) = self
                            .inactive_documents
                            .iter()
                            .find(|tab| tab.document.id == document_id)
                            && let Some(recovery) = &tab.recovery
                        {
                            let _ = recovery.clear();
                        }
                        self.set_notice("otra pestaña terminó de guardarse");
                    }
                } else if saved_current == Some(false) {
                    self.set_notice(if active {
                        "se creó una versión anterior; hay cambios nuevos"
                    } else {
                        "otra pestaña guardó una versión anterior"
                    });
                }
            }
            AppEvent::WorkspaceReady {
                request,
                root,
                index,
            } => {
                if request != self.workspace_request || index.cancelled {
                    self.log
                        .push("[workspace] se descartó un índice desactualizado".to_string());
                    return;
                }
                let note_count = index.notes.len();
                let skipped = index.skipped;
                let truncated = index.truncated;
                let content_truncated = index.content_truncated;
                let scan_truncated = index.scan_truncated;
                self.workspace = Some((root, index));
                self.workspace_paths = None;
                self.collapsed_workspace_dirs.clear();
                self.workspace_check_in_flight = None;
                self.workspace_stale = false;
                let suffix = match (truncated, content_truncated, scan_truncated) {
                    (_, _, true) => "; límite de lectura de carpeta alcanzado",
                    (true, true, false) => "; límite de notas y contenido alcanzado",
                    (true, false, false) => "; límite de notas alcanzado",
                    (false, true, false) => "; búsqueda limitada por presupuesto de memoria",
                    (false, false, false) => "",
                };
                self.set_notice(&format!(
                    "workspace indexado: {note_count} notas, {skipped} omitidas{suffix}"
                ));
            }
            AppEvent::WorkspaceFailed { request, error } => {
                if request != self.workspace_request {
                    self.log
                        .push("[workspace] se descartó un error desactualizado".to_string());
                    return;
                }
                self.log.push(format!("[workspace] {error}"));
                self.set_notice("no se pudo abrir la carpeta de trabajo");
            }
            AppEvent::WorkspaceChangeChecked {
                request,
                changed,
                truncated,
            } => {
                if self.workspace_check_in_flight != Some(request)
                    || request != self.workspace_request
                {
                    return;
                }
                self.workspace_check_in_flight = None;
                if changed && !self.workspace_stale {
                    self.workspace_stale = true;
                    self.set_notice(
                        "la carpeta de trabajo cambió; Ctrl+Shift+I actualiza el índice",
                    );
                } else if truncated {
                    self.log.push(
                        "[workspace] detección externa acotada a 1024 rutas indexadas".to_string(),
                    );
                }
            }
            AppEvent::ExternalChangeChecked {
                document_id,
                result,
            } => {
                self.external_checks_in_flight.remove(&document_id);
                if document_id != self.document.id {
                    return;
                }
                match result {
                    Ok(true) => self.resolve_external_change(),
                    Ok(false) => {}
                    Err(error) => {
                        self.log.push(format!("[archivos] {error}"));
                        self.set_notice(
                            "no se pudo comprobar si el archivo cambió fuera de Visor MD",
                        );
                    }
                }
            }
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        self.log.push(format!(
            "[medicion]   arranque del bucle hasta `resumed`: {:.0} ms",
            self.started.elapsed().as_secs_f64() * 1000.0
        ));

        let attrs = Window::default_attributes()
            .with_title(window_title(
                &self.document.path,
                self.document.safe_mode,
                self.document.source_editor.is_dirty(),
                self.notice.as_deref(),
            ))
            .with_inner_size(winit::dpi::LogicalSize::new(900.0, 760.0));
        let t = Instant::now();
        let window = match event_loop.create_window(attrs) {
            Ok(window) => Rc::new(window),
            Err(error) => {
                self.fail_and_exit(event_loop, format!("no se pudo crear la ventana: {error}"));
                return;
            }
        };
        self.log.push(format!(
            "[medicion]   create_window: {:.0} ms",
            t.elapsed().as_secs_f64() * 1000.0
        ));
        self.log.push(format!(
            "[medicion] ventana visible: {:.0} ms",
            self.started.elapsed().as_secs_f64() * 1000.0
        ));

        let t = Instant::now();
        let context = match softbuffer::Context::new(window.clone()) {
            Ok(context) => context,
            Err(error) => {
                self.fail_and_exit(
                    event_loop,
                    format!("no se pudo crear el contexto grafico: {error}"),
                );
                return;
            }
        };
        let surface = match softbuffer::Surface::new(&context, window.clone()) {
            Ok(surface) => surface,
            Err(error) => {
                self.fail_and_exit(
                    event_loop,
                    format!("no se pudo crear la superficie grafica: {error}"),
                );
                return;
            }
        };
        self.log.push(format!(
            "[medicion]   superficie softbuffer: {:.0} ms",
            t.elapsed().as_secs_f64() * 1000.0
        ));
        self.surface = Some(surface);
        // Sigue al tema del sistema si el sistema lo informa; si no, noche.
        self.palette = match window.theme() {
            Some(Theme::Light) => DAY,
            _ => NIGHT,
        };
        self.scale_factor = window.scale_factor() as f32;
        window.request_redraw();
        self.window = Some(window);
        if self.recovery_privacy_notice_pending {
            self.recovery_privacy_notice_pending = false;
            MessageDialog::new()
                .set_level(MessageLevel::Info)
                .set_title("Recuperación local activada")
                .set_description(
                    "Visor MD conserva temporalmente cambios sin guardar en tu perfil local para recuperarlos tras un cierre inesperado. Esa copia es texto sin cifrar, no un guardado definitivo y nunca se escribe dentro de la bóveda.",
                )
                .set_buttons(MessageButtons::Ok)
                .show();
            self.set_notice("recuperación local temporal activada");
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                if self.request_close_all() {
                    self.report();
                    event_loop.exit();
                }
            }
            WindowEvent::ThemeChanged(theme) => {
                self.palette = if matches!(theme, Theme::Light) {
                    DAY
                } else {
                    NIGHT
                };
                self.live.clear();
                self.preview_live.clear();
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            // La ventana usa coordenadas físicas para dibujar. Al cambiar el
            // viewport o el factor de escala, ningún layout anterior puede
            // reutilizarse con seguridad; se recalcula al próximo cuadro.
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(window) = &self.window {
                    self.scale_factor = window.scale_factor() as f32;
                }
                self.live.clear();
                self.preview_live.clear();
                self.laid_for_width = -1.0;
                self.preview_laid_for_width = -1.0;
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.pointer = Some((position.x as f32, position.y as f32));
                if self.context_menu.is_some()
                    && let Some(w) = &self.window
                {
                    w.request_redraw();
                }
                let target = self
                    .target_at(position.x as f32, position.y as f32)
                    .map(|target| {
                        let label = target_label(target.kind, &target.destination);
                        format!("{label}: {}", target.destination)
                    });
                let text_cursor_hover = target.is_none()
                    && self
                        .cursor_at(position.x as f32, position.y as f32)
                        .is_some();
                let target_changed = target != self.hover_destination;
                if target_changed {
                    self.hover_destination = target;
                    self.refresh_title();
                }
                if target_changed || text_cursor_hover != self.text_cursor_hover {
                    self.text_cursor_hover = text_cursor_hover;
                    if let Some(w) = &self.window {
                        w.set_cursor(if self.hover_destination.is_some() {
                            CursorIcon::Pointer
                        } else if text_cursor_hover {
                            CursorIcon::Text
                        } else {
                            CursorIcon::Default
                        });
                    }
                }
                if self.selecting {
                    self.extend_selection_to(position.x as f32, position.y as f32);
                    if let Some(w) = &self.window {
                        w.request_redraw();
                    }
                }
            }
            WindowEvent::CursorLeft { .. } => {
                self.pointer = None;
                self.selecting = false;
                self.text_cursor_hover = false;
                self.hover_destination = None;
                self.refresh_title();
                if let Some(w) = &self.window {
                    w.set_cursor(CursorIcon::Default);
                }
            }
            WindowEvent::Focused(false) => {
                self.selecting = false;
                self.modifiers = ModifiersState::empty();
            }
            WindowEvent::Focused(true) => {
                self.check_external_change();
                self.check_workspace_change();
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => match state {
                ElementState::Pressed => {
                    if let Some(menu) = self.context_menu.take() {
                        if let Some(action) = self.pointer.and_then(|pointer| {
                            context_action_at(menu, pointer, self.document.mode)
                        }) {
                            match action {
                                ContextAction::Paste => self.paste_into_source(),
                                ContextAction::CopyText | ContextAction::CopyMarkdown => {
                                    self.copy_selection(action.source_markdown());
                                }
                                ContextAction::Search => {
                                    self.perform_action(AppAction::SearchDocument)
                                }
                                ContextAction::ToggleMode => {
                                    self.perform_action(AppAction::ToggleMode)
                                }
                                ContextAction::ToggleSplit => {
                                    self.perform_action(AppAction::ToggleSplit)
                                }
                                ContextAction::Save => self.perform_action(AppAction::Save),
                                ContextAction::SaveAs => self.perform_action(AppAction::SaveAs),
                            }
                        }
                        if let Some(w) = &self.window {
                            w.request_redraw();
                        }
                        return;
                    }
                    if let Some((x, y)) = self.pointer
                        && self.activate_panel_at(x, y)
                    {
                        self.selecting = false;
                        return;
                    }
                    if let (Some((x, y)), Some(window)) = (self.pointer, &self.window) {
                        let size = window.inner_size();
                        if let Some(action) = toolbar_action_at(x, y, size.width as f32) {
                            self.toolbar_focus = None;
                            self.perform_action(action);
                            self.selecting = false;
                            return;
                        }
                        if let Some(index) = tab_close_index_at(
                            x,
                            y,
                            size.width as f32,
                            size.height as f32,
                            self.tab_order.len(),
                        ) && let Some(document_id) = self.tab_order.get(index).copied()
                        {
                            self.activate_document_tab(document_id);
                            if self.document.id == document_id {
                                self.close_active_document_tab();
                            }
                            self.selecting = false;
                            return;
                        }
                        if let Some(index) = tab_index_at(
                            x,
                            y,
                            size.width as f32,
                            size.height as f32,
                            self.tab_order.len(),
                        ) && let Some(document_id) = self.tab_order.get(index).copied()
                        {
                            self.activate_document_tab(document_id);
                            self.selecting = false;
                            return;
                        }
                    }
                    if self.document.mode == DocumentMode::Reading
                        && let Some((x, y)) = self.pointer
                        && let Some(block) = self.code_copy_at(x, y)
                    {
                        self.copy_code_block(block);
                        self.selecting = false;
                        return;
                    }
                    if self.document.mode == DocumentMode::Reading
                        && let Some((x, y)) = self.pointer
                        && let Some(block) = self.task_at(x, y)
                    {
                        self.toggle_task(block);
                        self.selecting = false;
                        return;
                    }
                    self.focused_link = None;
                    self.focus_destination = None;
                    self.refresh_title();
                    if self.document.mode.is_editable() {
                        if let Some(cursor) = self.pointer.and_then(|(x, y)| self.cursor_at(x, y)) {
                            self.set_source_cursor_from_block(cursor, false);
                        }
                    } else {
                        self.selection = self
                            .pointer
                            .and_then(|(x, y)| self.cursor_at(x, y))
                            .map(DocumentSelection::collapsed);
                    }
                    self.selecting = self.selection.is_some();
                    if let Some(w) = &self.window {
                        w.request_redraw();
                    }
                }
                ElementState::Released => self.selecting = false,
            },
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Right,
                ..
            } => {
                let Some((x, y)) = self.pointer else {
                    return;
                };
                if self.document.mode == DocumentMode::Split
                    && self
                        .window
                        .as_ref()
                        .is_some_and(|window| x >= window.inner_size().width as f32 / 2.0)
                {
                    self.set_notice(
                        "la vista derecha es de lectura; edita la fuente a la izquierda",
                    );
                    return;
                }
                let (x, y) = if let Some(window) = &self.window {
                    let size = window.inner_size();
                    (
                        x.min((size.width as f32 - CONTEXT_MENU_WIDTH).max(0.0)),
                        y.min(
                            (size.height as f32
                                - CONTEXT_MENU_ROW_HEIGHT
                                    * context_actions(self.document.mode).len() as f32)
                                .max(0.0),
                        ),
                    )
                } else {
                    (x, y)
                };
                self.context_menu = Some((x, y));
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::ModifiersChanged(modifiers) => self.modifiers = modifiers.state(),
            WindowEvent::Ime(Ime::Commit(text)) if self.workspace_search_query.is_some() => {
                self.append_workspace_search_text(&text);
            }
            WindowEvent::Ime(Ime::Commit(text)) if self.search_query.is_some() => {
                self.append_search_text(&text);
            }
            WindowEvent::Ime(Ime::Commit(text)) if self.command_palette.is_some() => {
                self.append_command_palette_text(&text);
            }
            WindowEvent::Ime(Ime::Commit(text)) if self.document.mode.is_editable() => {
                self.edit_source(|editor, source| editor.insert(source, text.as_str()));
            }
            WindowEvent::KeyboardInput { event, .. } if self.command_palette.is_some() => {
                self.handle_command_palette_key(&event);
            }
            WindowEvent::KeyboardInput { event, .. } if self.workspace_search_query.is_some() => {
                self.handle_workspace_search_key(&event);
            }
            WindowEvent::KeyboardInput { event, .. } if self.workspace_paths.is_some() => {
                self.handle_workspace_path_key(&event);
            }
            WindowEvent::KeyboardInput { event, .. } if self.backlink_paths.is_some() => {
                self.handle_backlink_key(&event);
            }
            WindowEvent::KeyboardInput { event, .. } if self.outline_headings.is_some() => {
                self.handle_outline_key(&event);
            }
            WindowEvent::KeyboardInput { event, .. } if self.search_query.is_some() => {
                self.handle_search_key(&event);
            }
            WindowEvent::KeyboardInput { event, .. } if self.toolbar_focus.is_some() => {
                self.handle_toolbar_key(&event);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::F6),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.toolbar_focus = Some(0);
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyP),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() && self.modifiers.shift_key() => {
                self.open_command_palette();
            }
            WindowEvent::KeyboardInput { event, .. } if self.document.mode.is_editable() => {
                self.handle_source_key(&event);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyF),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() && self.modifiers.shift_key() => {
                self.perform_action(AppAction::SearchWorkspace)
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyF),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() => self.perform_action(AppAction::SearchDocument),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyR),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() && self.modifiers.shift_key() => {
                self.perform_action(AppAction::RestoreRecovery);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyL),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() && self.modifiers.shift_key() => {
                self.perform_action(AppAction::DocumentOutline);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyI),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() && self.modifiers.shift_key() => {
                self.perform_action(AppAction::RefreshWorkspace)
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyO),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() && self.modifiers.shift_key() => {
                self.perform_action(AppAction::ChooseWorkspace);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyB),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() && self.modifiers.shift_key() => {
                self.perform_action(AppAction::Backlinks);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyT),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() && self.modifiers.shift_key() => {
                self.perform_action(AppAction::WorkspaceFiles);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyO),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() => {
                self.perform_action(AppAction::OpenDocument);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyN),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() => {
                self.perform_action(AppAction::NewDocument);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyS),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() && self.modifiers.shift_key() => {
                self.perform_action(AppAction::SaveAs);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyS),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() => {
                self.perform_action(AppAction::Save);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyZ),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.document.mode == DocumentMode::Reading
                && self.modifiers.control_key()
                && self.modifiers.shift_key() =>
            {
                self.edit_source(|editor, source| editor.redo(source));
                self.refresh_reading_async("rehaciendo cambio");
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyY),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.document.mode == DocumentMode::Reading && self.modifiers.control_key() => {
                self.edit_source(|editor, source| editor.redo(source));
                self.refresh_reading_async("rehaciendo cambio");
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyZ),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.document.mode == DocumentMode::Reading
                && self.modifiers.control_key()
                && !self.modifiers.shift_key() =>
            {
                self.edit_source(|editor, source| editor.undo(source));
                self.refresh_reading_async("deshaciendo cambio");
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyW),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() => self.perform_action(AppAction::CloseDocument),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::PageUp),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() => self.switch_document_tab(true),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::PageDown),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() => self.switch_document_tab(false),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::F2),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.perform_action(AppAction::ToggleMode);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::F3),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.perform_action(AppAction::ToggleSplit);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.selection = None;
                self.focused_link = None;
                self.focus_destination = None;
                self.refresh_title();
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::ArrowUp),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.move_selection_line(false, self.modifiers.shift_key());
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::ArrowDown),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.move_selection_line(true, self.modifiers.shift_key());
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::ArrowLeft),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.move_selection_visually(false, self.modifiers.shift_key());
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::ArrowRight),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.move_selection_visually(true, self.modifiers.shift_key());
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Home),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.move_selection_boundary(
                    false,
                    self.modifiers.control_key(),
                    self.modifiers.shift_key(),
                );
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::PageUp),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.scroll_page(false);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::PageDown),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.scroll_page(true);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::End),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.move_selection_boundary(
                    true,
                    self.modifiers.control_key(),
                    self.modifiers.shift_key(),
                );
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyA),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() => {
                self.select_document();
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyC),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } if self.modifiers.control_key() => {
                self.copy_selection(self.modifiers.shift_key());
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Enter),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.open_focused_link();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Tab),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.move_link_focus(self.modifiers.shift_key());
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyT),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                let is_night = self.palette.bg == NIGHT.bg;
                self.palette = if is_night { DAY } else { NIGHT };
                self.live.clear();
                self.preview_live.clear();
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y * 60.0,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                let viewport_height = self
                    .window
                    .as_ref()
                    .map(|window| window.inner_size().height as f32)
                    .unwrap_or(0.0);
                let max = max_scroll(self.doc_height.max(self.preview_height), viewport_height);
                self.scroll = (self.scroll - dy).clamp(0.0, max);
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(error) = self.redraw() {
                    self.fail_and_exit(event_loop, error);
                    return;
                }

                if !self.document_busy(self.document.id)
                    && let Some(total) = self.bench
                {
                    if self.frames >= total {
                        self.report();
                        event_loop.exit();
                    } else {
                        // Avanza un salto fijo por cuadro, dando la vuelta al
                        // documento entero para que la medicion no se quede
                        // midiendo siempre la misma pantalla.
                        let viewport_height = self
                            .window
                            .as_ref()
                            .map(|window| window.inner_size().height as f32)
                            .unwrap_or(0.0);
                        let max =
                            max_scroll(self.doc_height.max(self.preview_height), viewport_height)
                                .max(1.0);
                        self.scroll = (self.scroll + max / total as f32) % max;
                        if let Some(w) = &self.window {
                            w.request_redraw();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl App {
    fn perform_action(&mut self, action: AppAction) {
        match action {
            AppAction::NewDocument => self.create_new_document(),
            AppAction::OpenDocument => self.choose_document_to_open(),
            AppAction::Save => self.save_current_document(),
            AppAction::SaveAs => self.save_as_current_document(),
            AppAction::CloseDocument => self.close_active_document_tab(),
            AppAction::ToggleMode => match self.document.mode {
                DocumentMode::Reading => self.enter_source_mode(),
                DocumentMode::SourceEditing | DocumentMode::Split => {
                    self.refresh_reading_async("actualizando vista de lectura")
                }
            },
            AppAction::ToggleSplit => match self.document.mode {
                DocumentMode::Split => self.refresh_reading_async("cerrando vista dividida"),
                DocumentMode::Reading | DocumentMode::SourceEditing => self.enter_split_mode(),
            },
            AppAction::SearchDocument => self.open_document_search(),
            AppAction::ChooseWorkspace => self.choose_workspace(),
            AppAction::RefreshWorkspace => self.reindex_workspace(),
            AppAction::SearchWorkspace => self.open_workspace_search(),
            AppAction::DocumentOutline => self.show_document_outline(),
            AppAction::WorkspaceFiles => self.show_workspace_files(),
            AppAction::Backlinks => self.show_backlinks(),
            AppAction::RestoreRecovery => self.restore_latest_recovery(),
            AppAction::ToggleRecovery => self.toggle_recovery(),
            AppAction::CommandPalette => self.open_command_palette(),
        }
    }

    fn new_recovery_session(&self) -> Option<RecoverySession> {
        self.recovery_enabled
            .then(RecoverySession::start)
            .and_then(Result::ok)
    }

    fn toggle_recovery(&mut self) {
        if self.recovery_enabled {
            let dialog = MessageDialog::new()
                .set_level(MessageLevel::Warning)
                .set_title("Visor MD · desactivar recuperación")
                .set_description(
                    "La recuperación conserva copias temporales locales sin cifrar de documentos modificados. Si la desactivas, un cierre inesperado puede perder cambios.\n\nSí: mantenerla activa.\nNo: desactivarla y eliminar las copias de esta sesión.",
                )
                .set_buttons(MessageButtons::YesNo);
            let result = if let Some(window) = &self.window {
                dialog.set_parent(window.as_ref()).show()
            } else {
                dialog.show()
            };
            if !matches!(result, MessageDialogResult::No) {
                self.set_notice("la recuperación local sigue activa");
                return;
            }
        }
        self.recovery_enabled = !self.recovery_enabled;
        if self.recovery_enabled {
            self.recovery = RecoverySession::start().ok();
            for tab in &mut self.inactive_documents {
                tab.recovery = RecoverySession::start().ok();
            }
            self.recovery_privacy_notice_pending =
                RecoverySession::privacy_notice_needed().unwrap_or(false);
            self.set_notice("recuperación local activada");
        } else {
            if let Some(recovery) = self.recovery.take() {
                let _ = recovery.clear();
            }
            for tab in &mut self.inactive_documents {
                if let Some(recovery) = tab.recovery.take() {
                    let _ = recovery.clear();
                }
            }
            self.recovery_privacy_notice_pending = false;
            self.set_notice("recuperación local desactivada");
        }
        if (Settings {
            recovery_enabled: self.recovery_enabled,
        })
        .store()
        .is_err()
        {
            self.set_notice("la preferencia de recuperación no pudo guardarse");
        }
    }

    fn open_command_palette(&mut self) {
        self.context_menu = None;
        self.search_query = None;
        self.workspace_search_query = None;
        self.workspace_paths = None;
        self.backlink_paths = None;
        self.outline_headings = None;
        self.command_palette = Some(0);
        self.command_palette_query.clear();
        self.toolbar_focus = None;
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn handle_toolbar_key(&mut self, event: &KeyEvent) {
        if event.state != ElementState::Pressed || event.repeat {
            return;
        }
        let Some(index) = self.toolbar_focus else {
            return;
        };
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Escape | KeyCode::F6) => self.toolbar_focus = None,
            PhysicalKey::Code(KeyCode::ArrowLeft) => {
                self.toolbar_focus = Some(adjacent_toolbar_index(index, true));
            }
            PhysicalKey::Code(KeyCode::ArrowRight | KeyCode::Tab) => {
                self.toolbar_focus = Some(adjacent_toolbar_index(index, false));
            }
            PhysicalKey::Code(KeyCode::Enter | KeyCode::Space) => {
                self.toolbar_focus = None;
                self.perform_action(TOOLBAR_ACTIONS[index % TOOLBAR_ACTIONS.len()]);
            }
            _ => return,
        }
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn handle_command_palette_key(&mut self, event: &KeyEvent) {
        if event.state != ElementState::Pressed || event.repeat {
            return;
        }
        let Some(selected) = self.command_palette else {
            return;
        };
        let actions = self.command_palette_actions();
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                self.command_palette = None;
                self.command_palette_query.clear();
            }
            PhysicalKey::Code(KeyCode::Backspace) => {
                self.command_palette_query.pop();
                self.command_palette = Some(0);
            }
            PhysicalKey::Code(KeyCode::ArrowDown | KeyCode::Tab) if !actions.is_empty() => {
                self.command_palette = Some((selected + 1) % actions.len());
            }
            PhysicalKey::Code(KeyCode::ArrowUp) if !actions.is_empty() => {
                self.command_palette = Some((selected + actions.len() - 1) % actions.len());
            }
            PhysicalKey::Code(KeyCode::Enter) if !actions.is_empty() => {
                self.command_palette = None;
                self.command_palette_query.clear();
                self.perform_action(actions[selected % actions.len()]);
            }
            _ => {
                if !self.modifiers.control_key()
                    && !self.modifiers.alt_key()
                    && let Some(text) = event.text.as_deref()
                {
                    self.append_command_palette_text(text);
                } else {
                    return;
                }
            }
        }
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn append_command_palette_text(&mut self, text: &str) {
        if text.chars().any(char::is_control) {
            return;
        }
        append_limited_text(
            &mut self.command_palette_query,
            text,
            MAX_SEARCH_QUERY_CHARS,
        );
        self.command_palette = Some(0);
    }

    fn command_palette_actions(&self) -> Vec<AppAction> {
        filtered_actions(&self.command_palette_query)
    }

    fn activate_panel_at(&mut self, x: f32, y: f32) -> bool {
        if let Some(selected) = self.command_palette {
            let actions = self.command_palette_actions();
            if let Some(index) = panel_item_at(x, y, actions.len(), selected) {
                self.command_palette = None;
                self.command_palette_query.clear();
                self.perform_action(actions[index]);
                return true;
            }
        } else if self.workspace_search_query.is_some() {
            let count = self.workspace_search_matches().len();
            if let Some(index) = panel_item_at(x, y, count, self.workspace_search_match) {
                self.workspace_search_match = index;
                self.open_workspace_search_match();
                return true;
            }
        } else if let Some(paths) = &self.workspace_paths {
            if let Some(index) = panel_item_at(x, y, paths.len(), self.workspace_path_match) {
                self.workspace_path_match = index;
                self.open_workspace_path_match();
                return true;
            }
        } else if let Some(paths) = &self.backlink_paths {
            if let Some(index) = panel_item_at(x, y, paths.len(), self.backlink_match) {
                self.backlink_match = index;
                self.open_backlink_match();
                return true;
            }
        } else if let Some(headings) = &self.outline_headings
            && let Some(index) = panel_item_at(x, y, headings.len(), self.outline_match)
        {
            self.outline_match = index;
            self.open_outline_match();
            return true;
        }
        false
    }

    fn reset_document_view(&mut self) {
        self.invalidate_document_layout();
        self.scroll = self.document.scroll;
        self.selection = None;
        self.focused_link = None;
        self.focus_destination = None;
        self.context_menu = None;
        self.search_query = None;
        self.workspace_search_query = None;
        self.backlink_paths = None;
        self.outline_headings = None;
        self.command_palette = None;
        self.command_palette_query.clear();
        self.toolbar_focus = None;
    }

    fn invalidate_document_layout(&mut self) {
        self.slots.clear();
        self.live.clear();
        self.doc_height = 0.0;
        self.laid_for_width = -1.0;
        self.preview_slots.clear();
        self.preview_live.clear();
        self.preview_height = 0.0;
        self.preview_laid_for_width = -1.0;
        self.exact_after_edit = true;
    }

    fn document_busy(&self, document_id: u64) -> bool {
        self.open_requests.contains_key(&document_id)
            || self.view_requests.contains_key(&document_id)
            || self.saves_in_flight.contains(&document_id)
    }

    fn open_document_in_tab(&mut self, document: DocumentState) {
        let new_id = document.id;
        self.document.scroll = self.scroll;
        let current = std::mem::replace(&mut self.document, document);
        let next_recovery = self.new_recovery_session();
        let current_recovery = std::mem::replace(&mut self.recovery, next_recovery);
        self.inactive_documents.push(InactiveDocument {
            document: current,
            recovery: current_recovery,
        });
        self.tab_order.push(new_id);
        self.reset_document_view();
    }

    fn switch_document_tab(&mut self, backwards: bool) {
        if self.inactive_documents.is_empty() {
            self.set_notice("solo hay un documento abierto");
            return;
        }
        let Some(document_id) = adjacent_tab_id(&self.tab_order, self.document.id, backwards)
        else {
            self.set_notice("no se pudo identificar la pestaña activa");
            return;
        };
        self.activate_document_tab(document_id);
    }

    fn activate_document_tab(&mut self, document_id: u64) {
        if document_id == self.document.id {
            return;
        }
        let Some(index) = self
            .inactive_documents
            .iter()
            .position(|tab| tab.document.id == document_id)
        else {
            self.set_notice("la pestaña seleccionada ya no está disponible");
            return;
        };
        let next = self.inactive_documents.remove(index);
        self.document.scroll = self.scroll;
        let current = std::mem::replace(&mut self.document, next.document);
        let current_recovery = std::mem::replace(&mut self.recovery, next.recovery);
        self.inactive_documents.push(InactiveDocument {
            document: current,
            recovery: current_recovery,
        });
        self.reset_document_view();
        self.refresh_title();
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn close_active_document_tab(&mut self) {
        if self.document_busy(self.document.id) {
            self.set_notice("espera a que termine la operación de esta pestaña antes de cerrarla");
            return;
        }
        if !self.request_close_current() {
            return;
        }
        let active_position = self
            .tab_order
            .iter()
            .position(|id| *id == self.document.id)
            .unwrap_or(0);
        self.tab_order.retain(|id| *id != self.document.id);
        let next_id = self
            .tab_order
            .get(active_position.min(self.tab_order.len().saturating_sub(1)))
            .copied();
        let next = next_id.and_then(|id| {
            let index = self
                .inactive_documents
                .iter()
                .position(|tab| tab.document.id == id)?;
            Some(self.inactive_documents.remove(index))
        });
        if let Some(next) = next {
            self.document = next.document;
            self.recovery = next.recovery;
        } else {
            self.document = DocumentState::untitled();
            self.recovery = self.new_recovery_session();
            self.tab_order.push(self.document.id);
        }
        self.reset_document_view();
        self.set_notice("pestaña cerrada");
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
    fn enter_source_mode(&mut self) {
        match safe_buffer_blocks(&self.document.source) {
            Ok(blocks) => {
                self.document.mode = DocumentMode::SourceEditing;
                self.document.blocks = blocks;
                self.slots.clear();
                self.live.clear();
                self.laid_for_width = -1.0;
                self.selection = None;
                self.set_notice("edición de fuente · F2 para volver a lectura");
                if let Some(window) = &self.window {
                    window.set_cursor(CursorIcon::Text);
                    window.request_redraw();
                }
            }
            Err(error) => self.set_notice(&format!("no se pudo preparar la fuente: {error}")),
        }
    }

    fn refresh_source_blocks(&mut self) -> Result<(), &'static str> {
        self.document.blocks = safe_buffer_blocks(&self.document.source)?;
        self.slots.clear();
        self.live.clear();
        self.laid_for_width = -1.0;
        Ok(())
    }

    fn enter_split_mode(&mut self) {
        match safe_buffer_blocks(&self.document.source) {
            Ok(blocks) => {
                self.document.mode = DocumentMode::Split;
                self.document.blocks = blocks;
                self.reset_document_view();
                self.request_render_async();
                self.set_notice("vista dividida · fuente editable y lectura sincronizada");
                if let Some(window) = &self.window {
                    window.set_cursor(CursorIcon::Text);
                    window.request_redraw();
                }
            }
            Err(error) => self.set_notice(&format!("no se pudo preparar la fuente: {error}")),
        }
    }

    fn edit_source(
        &mut self,
        operation: impl FnOnce(&mut SourceEditor, &mut TextBuffer) -> Result<bool, editor::EditError>,
    ) {
        let document = &mut self.document;
        match operation(&mut document.source_editor, &mut document.source) {
            Ok(true) => match self.refresh_source_blocks() {
                Ok(()) => {
                    self.notice = None;
                    self.refresh_title();
                    self.schedule_recovery();
                    if self.document.mode == DocumentMode::Split {
                        self.request_render_async();
                    }
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
                Err(error) => self.set_notice(&format!("no se pudo actualizar la fuente: {error}")),
            },
            Ok(false) => {}
            Err(error) => self.set_notice(&format!("edición rechazada: {error:?}")),
        }
    }

    fn schedule_recovery(&mut self) {
        if self.document.last_recovery.elapsed().as_secs() < 3 {
            return;
        }
        let Some(recovery) = self.recovery.clone() else {
            return;
        };
        self.document.last_recovery = Instant::now();
        let source = self.document.source.to_string();
        let request = recovery.next_write_request();
        thread::spawn(move || {
            let _ = recovery.write_if_current(&source, request);
        });
    }

    fn direct_source_text<'a>(&self, event: &'a KeyEvent) -> Option<&'a str> {
        if self.modifiers.control_key()
            || self.modifiers.alt_key()
            || self.modifiers.super_key()
            || matches!(event.physical_key, PhysicalKey::Code(KeyCode::Enter))
        {
            return None;
        }
        event
            .text
            .as_deref()
            .filter(|text| !text.is_empty() && !text.chars().any(char::is_control))
    }

    fn refresh_reading_async(&mut self, notice: &str) {
        self.document.mode = DocumentMode::Reading;
        self.set_notice(notice);
        self.request_render_async();
    }

    fn request_render_async(&mut self) {
        let document_id = self.document.id;
        let revision = self.document.source_editor.revision();
        self.view_requests.insert(document_id, revision);
        let proxy = self.proxy.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(120));
            let _ = proxy.send_event(AppEvent::ViewRequested {
                document_id,
                revision,
            });
        });
    }

    fn handle_source_key(&mut self, event: &KeyEvent) {
        if event.state != ElementState::Pressed {
            return;
        }
        if !event.repeat
            && self.modifiers.control_key()
            && matches!(event.physical_key, PhysicalKey::Code(KeyCode::PageUp))
        {
            self.switch_document_tab(true);
            return;
        }
        if !event.repeat
            && self.modifiers.control_key()
            && matches!(event.physical_key, PhysicalKey::Code(KeyCode::KeyW))
        {
            self.perform_action(AppAction::CloseDocument);
            return;
        }
        if !event.repeat
            && self.modifiers.control_key()
            && matches!(event.physical_key, PhysicalKey::Code(KeyCode::PageDown))
        {
            self.switch_document_tab(false);
            return;
        }
        if !event.repeat
            && self.modifiers.control_key()
            && matches!(event.physical_key, PhysicalKey::Code(KeyCode::KeyF))
        {
            if self.modifiers.shift_key() {
                self.perform_action(AppAction::SearchWorkspace);
            } else {
                self.perform_action(AppAction::SearchDocument);
            }
            return;
        }
        if !event.repeat
            && self.modifiers.control_key()
            && self.modifiers.shift_key()
            && matches!(event.physical_key, PhysicalKey::Code(KeyCode::KeyL))
        {
            self.perform_action(AppAction::DocumentOutline);
            return;
        }
        if !event.repeat
            && self.modifiers.control_key()
            && self.modifiers.shift_key()
            && matches!(event.physical_key, PhysicalKey::Code(KeyCode::KeyB))
        {
            self.perform_action(AppAction::Backlinks);
            return;
        }
        if !event.repeat
            && self.modifiers.control_key()
            && self.modifiers.shift_key()
            && matches!(event.physical_key, PhysicalKey::Code(KeyCode::KeyT))
        {
            self.perform_action(AppAction::WorkspaceFiles);
            return;
        }

        // En Windows la escritura ordinaria llega en `KeyEvent::text`; IME
        // sigue entrando por `WindowEvent::Ime::Commit`. Las combinaciones de
        // control nunca se tratan como texto para no convertir Ctrl+S, Ctrl+V
        // ni atajos del sistema en contenido del documento.
        if let Some(text) = self.direct_source_text(event).map(str::to_owned) {
            self.edit_source(|editor, source| editor.insert(source, &text));
            self.sync_source_selection();
            if let Some(window) = &self.window {
                window.request_redraw();
            }
            return;
        }

        if event.repeat {
            return;
        }
        match event.physical_key {
            PhysicalKey::Code(KeyCode::F2) => {
                self.perform_action(AppAction::ToggleMode);
            }
            PhysicalKey::Code(KeyCode::F3) => {
                self.perform_action(AppAction::ToggleSplit);
            }
            PhysicalKey::Code(KeyCode::Backspace) => {
                self.edit_source(|editor, source| editor.backspace(source));
            }
            PhysicalKey::Code(KeyCode::Delete) => {
                self.edit_source(|editor, source| editor.delete(source));
            }
            PhysicalKey::Code(KeyCode::Enter) => {
                let eol = match self.document.source_metadata.line_endings {
                    LineEndings::CrLf => "\r\n",
                    LineEndings::None | LineEndings::Lf | LineEndings::Mixed => "\n",
                };
                let hard_break = self.modifiers.shift_key().then_some("\\");
                self.edit_source(|editor, source| {
                    if let Some(marker) = hard_break {
                        editor.insert(source, &format!("{marker}{eol}"))
                    } else {
                        editor.insert(source, eol)
                    }
                });
            }
            PhysicalKey::Code(KeyCode::ArrowLeft) => {
                let shift = self.modifiers.shift_key();
                let document = &mut self.document;
                let _ = document.source_editor.move_left(&document.source, shift);
            }
            PhysicalKey::Code(KeyCode::ArrowRight) => {
                let shift = self.modifiers.shift_key();
                let document = &mut self.document;
                let _ = document.source_editor.move_right(&document.source, shift);
            }
            PhysicalKey::Code(KeyCode::ArrowUp) => {
                let shift = self.modifiers.shift_key();
                let document = &mut self.document;
                let _ = document
                    .source_editor
                    .move_line(&document.source, false, shift);
            }
            PhysicalKey::Code(KeyCode::ArrowDown) => {
                let shift = self.modifiers.shift_key();
                let document = &mut self.document;
                let _ = document
                    .source_editor
                    .move_line(&document.source, true, shift);
            }
            PhysicalKey::Code(KeyCode::Home) => {
                let shift = self.modifiers.shift_key();
                let document = &mut self.document;
                let _ = document
                    .source_editor
                    .move_line_boundary(&document.source, false, shift);
            }
            PhysicalKey::Code(KeyCode::End) => {
                let shift = self.modifiers.shift_key();
                let document = &mut self.document;
                let _ = document
                    .source_editor
                    .move_line_boundary(&document.source, true, shift);
            }
            PhysicalKey::Code(KeyCode::KeyA) if self.modifiers.control_key() => {
                let document = &mut self.document;
                document.source_editor.select_all(&document.source);
            }
            PhysicalKey::Code(KeyCode::KeyC) if self.modifiers.control_key() => {
                self.copy_selection(false);
            }
            PhysicalKey::Code(KeyCode::KeyV) if self.modifiers.control_key() => {
                self.paste_into_source();
            }
            PhysicalKey::Code(KeyCode::KeyS)
                if self.modifiers.control_key() && self.modifiers.shift_key() =>
            {
                self.perform_action(AppAction::SaveAs);
            }
            PhysicalKey::Code(KeyCode::KeyS) if self.modifiers.control_key() => {
                self.perform_action(AppAction::Save);
            }
            PhysicalKey::Code(KeyCode::KeyN) if self.modifiers.control_key() => {
                self.perform_action(AppAction::NewDocument);
            }
            PhysicalKey::Code(KeyCode::KeyO) if self.modifiers.control_key() => {
                if self.modifiers.shift_key() {
                    self.perform_action(AppAction::ChooseWorkspace);
                } else {
                    self.perform_action(AppAction::OpenDocument);
                }
            }
            PhysicalKey::Code(KeyCode::KeyI)
                if self.modifiers.control_key() && self.modifiers.shift_key() =>
            {
                self.perform_action(AppAction::RefreshWorkspace);
            }
            PhysicalKey::Code(KeyCode::KeyR)
                if self.modifiers.control_key() && self.modifiers.shift_key() =>
            {
                self.perform_action(AppAction::RestoreRecovery);
            }
            PhysicalKey::Code(KeyCode::KeyZ) if self.modifiers.control_key() => {
                self.edit_source(|editor, source| editor.undo(source));
            }
            PhysicalKey::Code(KeyCode::KeyY) if self.modifiers.control_key() => {
                self.edit_source(|editor, source| editor.redo(source));
            }
            PhysicalKey::Code(KeyCode::Escape) => {
                let document = &mut self.document;
                let cursor = document.source_editor.cursor();
                let _ = document
                    .source_editor
                    .set_cursor(&document.source, cursor, false);
            }
            _ => {}
        }
        if self.document.mode.is_editable() {
            self.sync_source_selection();
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }

    fn paste_into_source(&mut self) {
        let clipboard = match self.clipboard.as_mut() {
            Some(clipboard) => clipboard,
            None => match Clipboard::new() {
                Ok(clipboard) => self.clipboard.insert(clipboard),
                Err(error) => {
                    self.log.push(format!(
                        "[portapapeles] no se pudo iniciar para pegar: {error}"
                    ));
                    self.set_notice("no se pudo acceder al portapapeles");
                    return;
                }
            },
        };
        match clipboard.get_text() {
            Ok(text) => self.edit_source(|editor, source| editor.insert(source, &text)),
            Err(error) => {
                self.log
                    .push(format!("[portapapeles] no se pudo pegar texto: {error}"));
                self.set_notice("el portapapeles no contiene texto utilizable");
            }
        }
    }

    fn save_current_document(&mut self) {
        let (Some(identity), Some(baseline_bytes)) = (
            self.document.source_identity.clone(),
            self.document.source_baseline_bytes.clone(),
        ) else {
            self.save_as_current_document();
            return;
        };
        if !self.document.source_editor.is_dirty() {
            self.set_notice("no hay cambios para guardar");
            return;
        }
        let path = self.document.path.clone();
        let source = self.document.source.to_string();
        let metadata = self.document.source_metadata;
        let revision = self.document.source_editor.revision();
        let proxy = self.proxy.clone();
        let document_id = self.document.id;
        if !self.saves_in_flight.insert(document_id) {
            self.set_notice("esta pestaña ya se está guardando");
            return;
        }
        self.set_notice("guardando de forma atómica");
        thread::spawn(move || {
            let event =
                match save_explicit_primary(&path, &source, metadata, &identity, &baseline_bytes) {
                    Ok(saved) => AppEvent::SaveReady {
                        document_id,
                        revision,
                        identity: saved.identity,
                        baseline_bytes: saved.baseline_bytes,
                    },
                    Err(error) => AppEvent::SaveFailed {
                        document_id,
                        conflict: matches!(&error, FileSaveError::Conflict),
                        error: format!("no se pudo guardar: {error}"),
                    },
                };
            let _ = proxy.send_event(event);
        });
    }

    fn save_as_current_document(&mut self) {
        let Some(path) = FileDialog::new()
            .add_filter("Markdown", &["md", "markdown"])
            .set_file_name("nota.md")
            .save_file()
        else {
            self.set_notice("guardar como cancelado");
            return;
        };
        let source = self.document.source.to_string();
        let metadata = self.document.source_metadata;
        let revision = self.document.source_editor.revision();
        let proxy = self.proxy.clone();
        let document_id = self.document.id;
        if !self.saves_in_flight.insert(document_id) {
            self.set_notice("esta pestaña ya se está guardando");
            return;
        }
        self.set_notice("creando documento de forma atómica");
        thread::spawn(move || {
            let event = match save_new_primary(&path, &source, metadata) {
                Ok(saved) => AppEvent::SaveAsReady {
                    document_id,
                    path,
                    revision,
                    identity: saved.identity,
                    baseline_bytes: saved.baseline_bytes,
                },
                Err(error) => AppEvent::SaveFailed {
                    document_id,
                    // Guardar como nunca reemplaza destinos existentes. Ese
                    // rechazo no es un conflicto de la fuente abierta y no
                    // debe ofrecer recargar el documento actual.
                    conflict: false,
                    error: format!(
                        "no se pudo guardar como: {error}. El destino existente no se modificó"
                    ),
                },
            };
            let _ = proxy.send_event(event);
        });
    }

    /// Revisa el archivo solo cuando la ventana vuelve a recibir foco. La
    /// lectura es acotada y ocurre fuera de UI; no se instala un watcher que
    /// observe carpetas o rutas que la persona no eligió.
    fn check_external_change(&mut self) {
        let document_id = self.document.id;
        if self.open_requests.contains_key(&document_id)
            || self.external_checks_in_flight.contains(&document_id)
        {
            return;
        }
        let Some(baseline_bytes) = self.document.source_baseline_bytes.clone() else {
            return;
        };
        let path = PathBuf::from(&self.document.path);
        let proxy = self.proxy.clone();
        self.external_checks_in_flight.insert(document_id);
        thread::spawn(move || {
            let result = changed_on_disk(&path, &baseline_bytes).map_err(|error| error.to_string());
            let _ = proxy.send_event(AppEvent::ExternalChangeChecked {
                document_id,
                result,
            });
        });
    }

    /// Compara fuera de la UI una fotografía acotada de rutas que el índice ya
    /// autorizó. No instala un watcher, no descubre rutas nuevas y una marca
    /// tardía no puede afectar otro workspace.
    fn check_workspace_change(&mut self) {
        if self.workspace_check_in_flight.is_some() {
            return;
        }
        let Some((root, index)) = self.workspace.as_ref() else {
            return;
        };
        let root = root.clone();
        let snapshot = index.change_snapshot();
        let request = self.workspace_request;
        let proxy = self.proxy.clone();
        self.workspace_check_in_flight = Some(request);
        thread::spawn(move || {
            let changed = snapshot.may_have_changed(&root);
            let _ = proxy.send_event(AppEvent::WorkspaceChangeChecked {
                request,
                changed,
                truncated: snapshot.truncated,
            });
        });
    }

    /// Detectar un cambio no modifica nada por sí mismo. Recargar conserva una
    /// recuperación si había cambios locales; guardar una copia nunca toca la
    /// versión externa; cancelar mantiene la vista actual.
    fn resolve_external_change(&mut self) {
        let description = if self.document.source_editor.is_dirty() {
            "El archivo cambió fuera de Visor MD mientras tenías cambios locales.\n\nSí: recargar la versión externa después de conservar una recuperación local.\nNo: Guardar una copia con otro nombre.\nCancelar: mantener esta edición abierta."
        } else {
            "El archivo cambió fuera de Visor MD.\n\nSí: recargar la versión externa.\nNo: Guardar una copia de la vista actual.\nCancelar: mantener la vista actual."
        };
        let dialog = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Visor MD · archivo modificado externamente")
            .set_description(description)
            .set_buttons(MessageButtons::YesNoCancel);
        let result = if let Some(window) = &self.window {
            dialog.set_parent(window.as_ref()).show()
        } else {
            dialog.show()
        };
        match result {
            MessageDialogResult::Yes => {
                if self.document.source_editor.is_dirty() {
                    let Some(recovery) = &self.recovery else {
                        self.set_notice(
                            "no se pudo recargar: la recuperación local no está disponible; guarda una copia",
                        );
                        return;
                    };
                    if let Err(error) = recovery.write(&self.document.source.to_string()) {
                        self.log.push(format!("[recuperación] {error}"));
                        self.set_notice(
                            "no se pudo conservar una recuperación; no se recargó el archivo externo",
                        );
                        return;
                    }
                }
                self.open_document_path(PathBuf::from(&self.document.path));
            }
            MessageDialogResult::No => self.save_as_current_document(),
            MessageDialogResult::Cancel
            | MessageDialogResult::Ok
            | MessageDialogResult::Custom(_) => {
                self.set_notice("cambio externo detectado: se conservó la vista actual")
            }
        }
    }

    /// Un conflicto externo nunca sobrescribe el destino. La única acción que
    /// descarta la versión local es recargar, y antes se guarda una
    /// recuperación local explícita para que un fallo posterior no la pierda.
    fn resolve_save_conflict(&mut self) {
        let dialog = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Visor MD · conflicto al guardar")
            .set_description(
                "El archivo cambió fuera de Visor MD y no se sobrescribió.\n\nSí: recargar la versión externa después de conservar una recuperación local.\nNo: Guardar una copia con otro nombre.\nCancelar: mantener esta edición abierta.",
            )
            .set_buttons(MessageButtons::YesNoCancel);
        let result = if let Some(window) = &self.window {
            dialog.set_parent(window.as_ref()).show()
        } else {
            dialog.show()
        };
        match result {
            MessageDialogResult::Yes => {
                let Some(recovery) = &self.recovery else {
                    self.set_notice(
                        "no se pudo recargar: la recuperación local no está disponible; guarda una copia",
                    );
                    return;
                };
                if let Err(error) = recovery.write(&self.document.source.to_string()) {
                    self.log.push(format!("[recuperación] {error}"));
                    self.set_notice(
                        "no se pudo conservar una recuperación; no se recargó el archivo externo",
                    );
                    return;
                }
                self.open_document_path(PathBuf::from(&self.document.path));
            }
            MessageDialogResult::No => self.save_as_current_document(),
            MessageDialogResult::Cancel
            | MessageDialogResult::Ok
            | MessageDialogResult::Custom(_) => {
                self.set_notice("conflicto conservado: el archivo externo no se sobrescribió")
            }
        }
    }

    /// El cierre no puede transformar una edición activa en pérdida silenciosa.
    /// La pestaña muestra el estado sucio y el diálogo nativo conserva una
    /// recuperación sin cifrar antes de permitir abandonarla.
    fn request_close_current(&mut self) -> bool {
        if !self.document.source_editor.is_dirty() {
            if let Some(recovery) = &self.recovery {
                let _ = recovery.clear();
            }
            return true;
        }
        let recovery_preserved = if self.recovery_enabled {
            let Some(recovery) = &self.recovery else {
                self.set_notice("no se cerró: no hay recuperación local disponible");
                return false;
            };
            if let Err(error) = recovery.write(&self.document.source.to_string()) {
                self.log.push(format!("[recuperación] {error}"));
                self.set_notice("no se cerró: no se pudo conservar la recuperación local");
                return false;
            }
            true
        } else {
            false
        };
        let description = if recovery_preserved {
            "Hay cambios sin guardar. Se conservó una recuperación local sin cifrar.\n\nSí: seguir editando.\nNo: cerrar y conservar la recuperación para restaurarla después."
        } else {
            "Hay cambios sin guardar y la recuperación local está desactivada. Si cierras, estos cambios se perderán.\n\nSí: seguir editando.\nNo: cerrar sin recuperación."
        };
        let dialog = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Visor MD · cambios sin guardar")
            .set_description(description)
            .set_buttons(MessageButtons::YesNo);
        let result = if let Some(window) = &self.window {
            dialog.set_parent(window.as_ref()).show()
        } else {
            dialog.show()
        };
        matches!(result, MessageDialogResult::No)
    }

    fn request_close_all(&mut self) -> bool {
        if !self.open_requests.is_empty()
            || !self.view_requests.is_empty()
            || !self.saves_in_flight.is_empty()
        {
            self.set_notice("espera a que termine la operación actual antes de cerrar");
            return false;
        }
        let dirty_inactive = self
            .inactive_documents
            .iter()
            .filter(|tab| tab.document.is_dirty())
            .collect::<Vec<_>>();
        let dirty_count = dirty_document_count(&self.document, &self.inactive_documents);
        if dirty_count == 0 {
            if let Some(recovery) = &self.recovery {
                let _ = recovery.clear();
            }
            for tab in &self.inactive_documents {
                if let Some(recovery) = &tab.recovery {
                    let _ = recovery.clear();
                }
            }
            return true;
        }

        let current_result = if self.recovery_enabled && self.document.is_dirty() {
            self.recovery
                .as_ref()
                .ok_or("recuperación no disponible")
                .and_then(|recovery| {
                    recovery
                        .write(&self.document.source.to_string())
                        .map_err(|_| "falló la recuperación")
                })
        } else {
            Ok(())
        };
        let inactive_result = if self.recovery_enabled {
            dirty_inactive.into_iter().try_for_each(|tab| {
                tab.recovery
                    .as_ref()
                    .ok_or("recuperación no disponible")
                    .and_then(|recovery| {
                        recovery
                            .write(&tab.document.source.to_string())
                            .map_err(|_| "falló la recuperación")
                    })
            })
        } else {
            Ok(())
        };
        if current_result.is_err() || inactive_result.is_err() {
            self.set_notice("no se cerró: no se pudieron conservar todas las recuperaciones");
            return false;
        }

        let description = if self.recovery_enabled {
            format!(
                "Hay {dirty_count} documentos con cambios sin guardar. Se conservaron recuperaciones locales sin cifrar.\n\nSí: seguir editando.\nNo: cerrar y conservar las recuperaciones."
            )
        } else {
            format!(
                "Hay {dirty_count} documentos con cambios sin guardar y la recuperación local está desactivada. Si cierras, esos cambios se perderán.\n\nSí: seguir editando.\nNo: cerrar sin recuperación."
            )
        };
        let dialog = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Visor MD · documentos sin guardar")
            .set_description(description)
            .set_buttons(MessageButtons::YesNo);
        let result = if let Some(window) = &self.window {
            dialog.set_parent(window.as_ref()).show()
        } else {
            dialog.show()
        };
        matches!(result, MessageDialogResult::No)
    }

    fn create_new_document(&mut self) {
        self.open_document_in_tab(DocumentState::untitled());
        self.set_notice("documento nuevo · Ctrl+Shift+S para elegir destino");
        if let Some(window) = &self.window {
            window.set_cursor(CursorIcon::Text);
            window.request_redraw();
        }
    }

    fn choose_document_to_open(&mut self) {
        let Some(path) = FileDialog::new()
            .add_filter(
                "Texto y Markdown",
                &[
                    "md", "markdown", "txt", "json", "yaml", "yml", "toml", "csv",
                ],
            )
            .set_title("Abrir documento")
            .pick_file()
        else {
            self.set_notice("abrir documento cancelado");
            return;
        };
        self.open_document_path(path);
    }

    fn choose_workspace(&mut self) {
        let Some(path) = FileDialog::new()
            .set_title("Elegir carpeta de trabajo")
            .pick_folder()
        else {
            self.set_notice("abrir carpeta cancelado");
            return;
        };
        self.start_workspace_index(path, "indexando carpeta de trabajo");
    }

    /// Volver a recorrer una carpeta ya concedida es una acción explícita. No
    /// instala vigilancia de directorios ni permite que contenido Markdown
    /// active lecturas nuevas fuera de la raíz elegida.
    fn reindex_workspace(&mut self) {
        let Some((root, _)) = &self.workspace else {
            self.set_notice("elige primero una carpeta de trabajo con Ctrl+Shift+O");
            return;
        };
        self.start_workspace_index(root.root().to_path_buf(), "actualizando carpeta de trabajo");
    }

    fn start_workspace_index(&mut self, path: PathBuf, notice: &str) {
        if let Some(cancel) = &self.workspace_cancel {
            cancel.store(true, Ordering::Relaxed);
        }
        self.workspace_request = self.workspace_request.saturating_add(1);
        self.workspace_check_in_flight = None;
        let request = self.workspace_request;
        let cancel = Arc::new(AtomicBool::new(false));
        self.workspace_cancel = Some(cancel.clone());
        let proxy = self.proxy.clone();
        self.set_notice(notice);
        thread::spawn(move || {
            let event = match WorkspaceRoot::open(path) {
                Ok(root) => {
                    let index =
                        index_workspace_cancellable(&root, WorkspaceLimits::default(), &cancel);
                    AppEvent::WorkspaceReady {
                        request,
                        root,
                        index,
                    }
                }
                Err(error) => AppEvent::WorkspaceFailed {
                    request,
                    error: error.to_string(),
                },
            };
            let _ = proxy.send_event(event);
        });
    }

    fn restore_latest_recovery(&mut self) {
        match RecoverySession::latest_pending(DEFAULT_DOCUMENT_LIMIT_BYTES) {
            Ok(Some(source)) => {
                let recovered = match recovered_document(source) {
                    Ok(recovered) => recovered,
                    Err(error) => {
                        self.set_notice(&format!("no se pudo mostrar la recuperación: {error}"));
                        return;
                    }
                };
                self.open_document_in_tab(recovered);
                self.set_notice("recuperación abierta como documento sin guardar");
            }
            Ok(None) => self.set_notice("no hay recuperaciones locales disponibles"),
            Err(error) => {
                self.log.push(format!("[recuperación] {error}"));
                self.set_notice("no se pudo leer la recuperación local");
            }
        }
    }

    fn open_document_path(&mut self, path: PathBuf) -> bool {
        if paths_refer_to_same_file(std::path::Path::new(&self.document.path), &path) {
            self.set_notice("el documento ya está abierto en la pestaña activa");
            return true;
        }
        if let Some(document_id) = self
            .inactive_documents
            .iter()
            .find(|tab| paths_refer_to_same_file(std::path::Path::new(&tab.document.path), &path))
            .map(|tab| tab.document.id)
        {
            self.activate_document_tab(document_id);
            self.set_notice("se activó la pestaña que ya contenía el documento");
            return true;
        }
        // Los resultados de navegación pertenecen al documento anterior. No
        // deben quedar flotando sobre una apertura nueva aunque su hilo tarde.
        let mut pending = DocumentState::untitled();
        pending.path = path.to_string_lossy().into_owned();
        pending.mode = DocumentMode::Reading;
        self.open_document_in_tab(pending);
        let document_id = self.document.id;
        self.document_request = self.document_request.wrapping_add(1);
        let request = self.document_request;
        self.open_requests.insert(document_id, request);
        self.set_notice("cargando documento");
        let proxy = self.proxy.clone();
        thread::spawn(move || {
            let event = match open_explicit_primary(&path, DEFAULT_DOCUMENT_LIMIT_BYTES) {
                Ok(opened) => match if is_markdown_path(&path) {
                    parse_blocks(&opened.source)
                } else {
                    let source_index = SourceIndex::new(&opened.source);
                    safe_source_blocks(&opened.source, &source_index).map(|blocks| ParseOutcome {
                        blocks,
                        degradation: Some(Degradation::TextOnly),
                    })
                } {
                    Ok(outcome) => AppEvent::DocumentReady {
                        document_id,
                        request,
                        path,
                        source: opened.source,
                        metadata: opened.metadata,
                        identity: opened.identity,
                        baseline_bytes: opened.baseline_bytes,
                        outcome,
                        elapsed_ms: 0.0,
                    },
                    Err(error) => AppEvent::DocumentFailed {
                        document_id,
                        request,
                        error: format!("el documento no se pudo preparar de forma segura: {error}"),
                    },
                },
                Err(error) => AppEvent::DocumentFailed {
                    document_id,
                    request,
                    error: format!("no se pudo abrir el documento: {error}"),
                },
            };
            let _ = proxy.send_event(event);
        });
        true
    }

    fn source_block_cursor(&self, source_offset: usize) -> Option<BlockCursor> {
        self.document
            .blocks
            .iter()
            .enumerate()
            .find_map(|(block, item)| {
                let text_end = item.source.start + item.text.len();
                (item.source.start <= source_offset && source_offset <= text_end).then_some(
                    BlockCursor {
                        block,
                        offset: source_offset - item.source.start,
                    },
                )
            })
    }

    fn sync_source_selection(&mut self) {
        let Some(anchor) = self.source_block_cursor(self.document.source_editor.anchor()) else {
            return;
        };
        let Some(focus) = self.source_block_cursor(self.document.source_editor.cursor()) else {
            return;
        };
        self.selection = Some(DocumentSelection { anchor, focus });
    }

    fn set_source_cursor_from_block(&mut self, cursor: BlockCursor, extend: bool) {
        let Some(block) = self.document.blocks.get(cursor.block) else {
            return;
        };
        let source_offset = block.source.start + cursor.offset.min(block.text.len());
        if self
            .document
            .source_editor
            .set_cursor(&self.document.source, source_offset, extend)
            .is_err()
        {
            return;
        }
        self.selection = Some(if extend {
            DocumentSelection {
                anchor: self.selection.map_or(cursor, |selection| selection.anchor),
                focus: cursor,
            }
        } else {
            DocumentSelection::collapsed(cursor)
        });
    }

    fn copy_selection(&mut self, source_markdown: bool) {
        let source_snapshot = source_markdown.then(|| self.document.source.to_string());
        let text = self.selection.and_then(|selection| {
            if source_markdown {
                selection.source_blocks(source_snapshot.as_deref()?, &self.document.blocks)
            } else {
                selection.rendered_text(&self.document.blocks)
            }
        });
        let Some(text) = text else {
            self.set_notice("sin texto seleccionado");
            return;
        };

        let kind = if source_markdown {
            "Markdown original copiado"
        } else {
            "texto copiado"
        };
        self.copy_text_to_clipboard(text, kind);
    }

    fn copy_text_to_clipboard(&mut self, text: String, kind: &str) {
        let clipboard = match self.clipboard.as_mut() {
            Some(clipboard) => clipboard,
            None => match Clipboard::new() {
                Ok(clipboard) => self.clipboard.insert(clipboard),
                Err(error) => {
                    self.log
                        .push(format!("[portapapeles] no se pudo iniciar: {error}"));
                    self.set_notice("no se pudo acceder al portapapeles");
                    return;
                }
            },
        };
        if let Err(error) = clipboard.set_text(text) {
            self.log
                .push(format!("[portapapeles] no se pudo copiar: {error}"));
            self.set_notice("no se pudo copiar");
            return;
        }
        self.log.push(format!("[portapapeles] {kind}"));
        self.set_notice(kind);
    }

    fn set_notice(&mut self, notice: &str) {
        self.notice = Some(notice.to_owned());
        self.refresh_title();
    }

    fn refresh_title(&self) {
        if let Some(window) = &self.window {
            window.set_title(&window_title(
                &self.document.path,
                self.document.safe_mode,
                self.document.source_editor.is_dirty(),
                self.hover_destination
                    .as_deref()
                    .or(self.focus_destination.as_deref())
                    .or(self.notice.as_deref()),
            ));
        }
    }

    fn move_link_focus(&mut self, backwards: bool) {
        let links = link_targets_in_document_order(&self.document.blocks);
        let Some((block_index, target_index)) =
            next_link_target(&links, self.focused_link, backwards)
        else {
            self.set_notice("este documento no tiene enlaces");
            return;
        };
        let Some((target_kind, target_start, target_destination)) = self
            .document
            .blocks
            .get(block_index)
            .and_then(|block| block.targets.get(target_index))
            .map(|target| (target.kind, target.start, target.destination.clone()))
        else {
            return;
        };
        self.focused_link = Some((block_index, target_index));
        self.focus_destination = Some(format!(
            "{}: {}",
            target_label(target_kind, &target_destination),
            target_destination
        ));
        self.notice = None;
        self.selection = Some(DocumentSelection::collapsed(BlockCursor {
            block: block_index,
            offset: target_start,
        }));
        if let (Some(slot), Some(window)) = (self.slots.get(block_index), &self.window) {
            let viewport = window.inner_size().height as f32;
            if slot.y < self.scroll {
                self.scroll = slot.y;
            } else if slot.y + slot.height > self.scroll + viewport {
                self.scroll = (slot.y + slot.height - viewport).max(0.0);
            }
            self.scroll = self.scroll.min(max_scroll(
                self.doc_height.max(self.preview_height),
                viewport,
            ));
            window.request_redraw();
        }
        self.refresh_title();
    }

    fn open_focused_link(&mut self) {
        let Some((block_index, target_index)) = self.focused_link else {
            self.set_notice("enfoca un enlace con Tab antes de abrirlo");
            return;
        };
        let Some(destination) = self
            .document
            .blocks
            .get(block_index)
            .and_then(|block| block.targets.get(target_index))
            .map(|target| (target.kind, target.destination.clone()))
        else {
            return;
        };
        if destination.0 == InlineTargetKind::WikiLink {
            self.open_workspace_wikilink(&destination.1);
            return;
        }
        match classify_link_destination(&destination.1) {
            LinkDestinationKind::RelativeFile => {
                let resolved = self
                    .workspace
                    .as_ref()
                    .map(|(root, _)| root.resolve_existing(std::path::Path::new(&destination.1)));
                match resolved {
                    Some(Ok(path)) => {
                        self.document.pending_heading = None;
                        self.open_document_path(path);
                    }
                    Some(Err(error)) => {
                        self.log.push(format!("[vfs] {error}"));
                        self.set_notice(
                            "el enlace local fue bloqueado por la política de archivos",
                        );
                    }
                    None => self.set_notice(
                        "abre una carpeta de trabajo para seguir enlaces locales de forma segura",
                    ),
                }
            }
            LinkDestinationKind::Blocked => {
                self.set_notice("destino bloqueado por la política de seguridad");
            }
            LinkDestinationKind::Web | LinkDestinationKind::Mail => {
                if let Err(error) = open_external_destination(&destination.1) {
                    self.log.push(format!("[enlace] {error}"));
                    self.set_notice("no se pudo abrir el enlace");
                }
            }
        }
    }

    /// Abre un wikilink solo después de resolverlo contra el índice de la
    /// carpeta elegida. Un nombre duplicado pide una ruta más precisa: nunca
    /// se abre la primera coincidencia por orden de recorrido.
    fn open_workspace_wikilink(&mut self, destination: &str) {
        let (note_target, heading) =
            destination
                .split_once('#')
                .map_or((destination, None), |(note, heading)| {
                    (
                        note,
                        (!heading.trim().is_empty()).then(|| heading.trim().to_owned()),
                    )
                });
        if note_target.trim().is_empty() {
            if let Some(heading) = heading {
                self.scroll_to_heading(&heading);
            } else {
                self.set_notice("el enlace de bóveda no declara una nota ni un encabezado");
            }
            return;
        }
        let Some((root, index)) = self.workspace.as_ref() else {
            self.set_notice("abre una carpeta de trabajo para seguir enlaces de bóveda");
            return;
        };
        let path = match index.resolve_wikilink(note_target) {
            WikiResolution::Found(note) => root.resolve_existing(&note.relative_path),
            WikiResolution::Missing => {
                self.set_notice(
                    "la nota indicada por el enlace no existe en la carpeta de trabajo",
                );
                return;
            }
            WikiResolution::Ambiguous => {
                self.set_notice("el enlace de bóveda es ambiguo; usa una ruta más precisa");
                return;
            }
        };
        match path {
            Ok(path) => {
                if self.open_document_path(path) {
                    self.document.pending_heading = heading;
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            Err(error) => {
                self.log.push(format!("[vfs] {error}"));
                self.set_notice("el enlace de bóveda fue bloqueado por la política de archivos");
            }
        }
    }

    /// Muestra backlinks derivados únicamente del índice de la carpeta
    /// concedida. La apertura posterior pasa otra vez por VFS; un documento no
    /// puede usar esta acción para salir de la bóveda.
    fn show_backlinks(&mut self) {
        let Some((root, index)) = self.workspace.as_ref() else {
            self.set_notice("abre una carpeta de trabajo para consultar backlinks");
            return;
        };
        let Ok(current) = fs::canonicalize(&self.document.path) else {
            self.set_notice("el documento abierto no se puede asociar al workspace");
            return;
        };
        let Ok(relative) = current.strip_prefix(root.root()) else {
            self.set_notice("el documento abierto no pertenece a la carpeta de trabajo");
            return;
        };
        let Some(note) = index.note_at_relative(relative) else {
            self.set_notice("el documento abierto no está indexado como una nota Markdown");
            return;
        };
        let paths = index
            .backlinks_to(note)
            .into_iter()
            .map(|backlink| backlink.relative_path.clone())
            .collect::<Vec<_>>();
        if paths.is_empty() {
            self.set_notice("ninguna nota de la carpeta apunta al documento actual");
            return;
        }
        self.search_query = None;
        self.workspace_search_query = None;
        self.workspace_paths = None;
        self.backlink_match = 0;
        self.backlink_paths = Some(paths);
        self.set_notice("backlinks · flechas eligen, Enter abre, Escape cierra");
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn open_backlink_match(&mut self) {
        let Some(paths) = &self.backlink_paths else {
            return;
        };
        let Some(relative_path) = paths.get(self.backlink_match % paths.len().max(1)).cloned()
        else {
            return;
        };
        let Some((root, _)) = &self.workspace else {
            self.set_notice("la carpeta de trabajo ya no está disponible");
            return;
        };
        match root.resolve_existing(&relative_path) {
            Ok(path) => {
                self.backlink_paths = None;
                self.open_document_path(path);
            }
            Err(error) => {
                self.log.push(format!("[vfs] {error}"));
                self.set_notice("el backlink fue bloqueado por la política de archivos");
            }
        }
    }

    fn handle_backlink_key(&mut self, event: &KeyEvent) {
        if event.state != ElementState::Pressed || event.repeat {
            return;
        }
        let count = self.backlink_paths.as_ref().map_or(0, Vec::len);
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                self.backlink_paths = None;
                self.set_notice("backlinks cerrados");
            }
            PhysicalKey::Code(KeyCode::ArrowDown) if count > 0 => {
                self.backlink_match = (self.backlink_match + 1) % count;
            }
            PhysicalKey::Code(KeyCode::ArrowUp) if count > 0 => {
                self.backlink_match = (self.backlink_match + count - 1) % count;
            }
            PhysicalKey::Code(KeyCode::Enter) => self.open_backlink_match(),
            _ => {}
        }
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn show_document_outline(&mut self) {
        let headings = document_outline(&self.document.blocks);
        if headings.is_empty() {
            self.set_notice("el documento no contiene encabezados");
            return;
        }
        self.search_query = None;
        self.workspace_search_query = None;
        self.workspace_paths = None;
        self.backlink_paths = None;
        self.outline_match = 0;
        self.outline_headings = Some(headings);
        self.set_notice("índice del documento · flechas eligen, Enter enfoca, Escape cierra");
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn handle_outline_key(&mut self, event: &KeyEvent) {
        if event.state != ElementState::Pressed || event.repeat {
            return;
        }
        let count = self.outline_headings.as_ref().map_or(0, Vec::len);
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                self.outline_headings = None;
                self.set_notice("índice del documento cerrado");
            }
            PhysicalKey::Code(KeyCode::ArrowDown) if count > 0 => {
                self.outline_match = (self.outline_match + 1) % count;
            }
            PhysicalKey::Code(KeyCode::ArrowUp) if count > 0 => {
                self.outline_match = (self.outline_match + count - 1) % count;
            }
            PhysicalKey::Code(KeyCode::Enter) if count > 0 => self.open_outline_match(),
            _ => {}
        }
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn open_outline_match(&mut self) {
        let heading = self
            .outline_headings
            .as_ref()
            .and_then(|headings| headings.get(self.outline_match % headings.len()))
            .map(|(block, _)| *block);
        if let Some(block) = heading
            && let (Some(slot), Some(window)) = (self.slots.get(block), &self.window)
        {
            self.scroll = slot.y.min(max_scroll(
                self.doc_height,
                window.inner_size().height as f32,
            ));
            self.selection = Some(DocumentSelection::collapsed(BlockCursor {
                block,
                offset: 0,
            }));
        }
        self.outline_headings = None;
        self.set_notice("encabezado enfocado");
    }

    fn scroll_to_heading(&mut self, heading: &str) {
        let requested = heading_key(heading);
        let Some((block_index, _)) = self.document.blocks.iter().enumerate().find(|(_, block)| {
            matches!(block.kind, Kind::Heading(_)) && heading_key(&block.text) == requested
        }) else {
            self.set_notice("el documento no contiene el encabezado solicitado");
            return;
        };
        let Some(slot) = self.slots.get(block_index) else {
            self.set_notice("el encabezado se enfocará al terminar el layout");
            self.document.pending_heading = Some(heading.to_owned());
            return;
        };
        let viewport = self
            .window
            .as_ref()
            .map_or(0.0, |window| window.inner_size().height as f32);
        self.scroll = slot.y.min(max_scroll(
            self.doc_height.max(self.preview_height),
            viewport,
        ));
        self.selection = Some(DocumentSelection::collapsed(BlockCursor {
            block: block_index,
            offset: 0,
        }));
        self.set_notice("encabezado enfocado");
    }

    fn open_document_search(&mut self) {
        self.workspace_search_query = None;
        self.workspace_paths = None;
        self.backlink_paths = None;
        self.outline_headings = None;
        self.search_query = Some(String::new());
        self.search_match = 0;
        self.set_notice("búsqueda local · escribe texto, Enter recorre resultados, Escape cierra");
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn open_workspace_search(&mut self) {
        if self.workspace.is_none() {
            self.set_notice("elige primero una carpeta de trabajo con Ctrl+Shift+O");
            return;
        }
        self.search_query = None;
        self.workspace_paths = None;
        self.backlink_paths = None;
        self.outline_headings = None;
        self.workspace_search_query = Some(String::new());
        self.workspace_search_match = 0;
        self.set_notice(
            "buscar en carpeta · escribe texto, flechas eligen, Enter abre, Escape cierra",
        );
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    /// Recorre las notas que ya entregó el índice en memoria. Esto no es un
    /// explorador de archivos: el único destino que puede abrirse se vuelve a
    /// resolver contra la raíz concedida al confirmar con Enter.
    fn show_workspace_files(&mut self) {
        let Some((_, index)) = self.workspace.as_ref() else {
            self.set_notice("elige primero una carpeta de trabajo con Ctrl+Shift+O");
            return;
        };
        let paths = index.note_paths();
        if paths.is_empty() {
            self.set_notice("la carpeta de trabajo no contiene notas Markdown indexadas");
            return;
        }
        self.search_query = None;
        self.workspace_search_query = None;
        self.backlink_paths = None;
        self.outline_headings = None;
        self.workspace_path_match = 0;
        self.workspace_paths = Some(visible_workspace_tree_rows(
            &paths,
            &self.collapsed_workspace_dirs,
        ));
        self.set_notice("árbol de notas · flechas eligen, Enter abre una nota, Escape cierra");
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn open_workspace_path_match(&mut self) {
        let Some(paths) = &self.workspace_paths else {
            return;
        };
        let Some(row) = paths
            .get(self.workspace_path_match % paths.len().max(1))
            .cloned()
        else {
            return;
        };
        let Some(relative_path) = row.note_path().map(std::path::Path::to_path_buf) else {
            let directory = row.relative_path.clone();
            if !self.collapsed_workspace_dirs.remove(&directory) {
                self.collapsed_workspace_dirs.insert(directory.clone());
            }
            let Some((_, index)) = self.workspace.as_ref() else {
                return;
            };
            let visible =
                visible_workspace_tree_rows(&index.note_paths(), &self.collapsed_workspace_dirs);
            self.workspace_path_match = visible
                .iter()
                .position(|candidate| candidate.relative_path == directory)
                .unwrap_or(0);
            self.workspace_paths = Some(visible);
            self.set_notice("carpeta del árbol actualizada");
            return;
        };
        let Some((root, _)) = &self.workspace else {
            self.set_notice("la carpeta de trabajo ya no está disponible");
            return;
        };
        match root.resolve_existing(&relative_path) {
            Ok(path) => {
                self.workspace_paths = None;
                self.open_document_path(path);
            }
            Err(error) => {
                self.log.push(format!("[vfs] {error}"));
                self.set_notice("la nota fue bloqueada por la política de archivos");
            }
        }
    }

    fn handle_workspace_path_key(&mut self, event: &KeyEvent) {
        if event.state != ElementState::Pressed || event.repeat {
            return;
        }
        let count = self.workspace_paths.as_ref().map_or(0, Vec::len);
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                self.workspace_paths = None;
                self.set_notice("lista de notas cerrada");
            }
            PhysicalKey::Code(KeyCode::ArrowDown) if count > 0 => {
                self.workspace_path_match = (self.workspace_path_match + 1) % count;
            }
            PhysicalKey::Code(KeyCode::ArrowUp) if count > 0 => {
                self.workspace_path_match = (self.workspace_path_match + count - 1) % count;
            }
            PhysicalKey::Code(KeyCode::Enter) => self.open_workspace_path_match(),
            _ => {}
        }
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn append_workspace_search_text(&mut self, text: &str) {
        if text.chars().any(char::is_control) {
            return;
        }
        if let Some(query) = &mut self.workspace_search_query {
            append_limited_text(query, text, MAX_SEARCH_QUERY_CHARS);
            self.workspace_search_match = 0;
        }
    }

    fn workspace_search_matches(&self) -> Vec<PathBuf> {
        let Some(query) = self
            .workspace_search_query
            .as_deref()
            .filter(|query| !query.is_empty())
        else {
            return Vec::new();
        };
        self.workspace.as_ref().map_or_else(Vec::new, |(_, index)| {
            index
                .search(query)
                .into_iter()
                .map(|note| note.relative_path.clone())
                .collect()
        })
    }

    fn open_workspace_search_match(&mut self) {
        let matches = self.workspace_search_matches();
        let Some(relative_path) = matches
            .get(self.workspace_search_match % matches.len().max(1))
            .cloned()
        else {
            self.set_notice("no hay notas que coincidan con la búsqueda");
            return;
        };
        let Some((root, _)) = &self.workspace else {
            return;
        };
        match root.resolve_existing(&relative_path) {
            Ok(path) => {
                self.workspace_search_query = None;
                self.open_document_path(path);
            }
            Err(error) => {
                self.log.push(format!("[vfs] {error}"));
                self.set_notice("la nota encontrada fue bloqueada por la política de archivos");
            }
        }
    }

    fn handle_workspace_search_key(&mut self, event: &KeyEvent) {
        if event.state != ElementState::Pressed || event.repeat {
            return;
        }
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                self.workspace_search_query = None;
                self.set_notice("búsqueda de carpeta cerrada");
            }
            PhysicalKey::Code(KeyCode::Backspace) => {
                if let Some(query) = &mut self.workspace_search_query {
                    query.pop();
                    self.workspace_search_match = 0;
                }
            }
            PhysicalKey::Code(KeyCode::ArrowDown) => {
                let count = self.workspace_search_matches().len();
                if count > 0 {
                    self.workspace_search_match = (self.workspace_search_match + 1) % count;
                }
            }
            PhysicalKey::Code(KeyCode::ArrowUp) => {
                let count = self.workspace_search_matches().len();
                if count > 0 {
                    self.workspace_search_match = (self.workspace_search_match + count - 1) % count;
                }
            }
            PhysicalKey::Code(KeyCode::Enter) => self.open_workspace_search_match(),
            _ => {}
        }
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn append_search_text(&mut self, text: &str) {
        if text.chars().any(char::is_control) {
            return;
        }
        if let Some(query) = &mut self.search_query {
            append_limited_text(query, text, MAX_SEARCH_QUERY_CHARS);
            self.search_match = 0;
            self.focus_search_match(0);
        }
    }

    fn search_matches(&self) -> Vec<usize> {
        let Some(query) = self
            .search_query
            .as_deref()
            .filter(|query| !query.is_empty())
        else {
            return Vec::new();
        };
        matching_block_indices(&self.document.blocks, query)
    }

    fn focus_search_match(&mut self, direction: i8) {
        let matches = self.search_matches();
        if matches.is_empty() {
            self.selection = None;
            return;
        }
        if direction > 0 {
            self.search_match = (self.search_match + 1) % matches.len();
        } else if direction < 0 {
            self.search_match = (self.search_match + matches.len() - 1) % matches.len();
        } else {
            self.search_match %= matches.len();
        }
        let block = matches[self.search_match];
        self.selection = Some(DocumentSelection::collapsed(BlockCursor {
            block,
            offset: 0,
        }));
        if let (Some(slot), Some(window)) = (self.slots.get(block), &self.window) {
            self.scroll = slot.y.min(max_scroll(
                self.doc_height,
                window.inner_size().height as f32,
            ));
            window.request_redraw();
        }
    }

    fn handle_search_key(&mut self, event: &KeyEvent) {
        if event.state != ElementState::Pressed || event.repeat {
            return;
        }
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                self.search_query = None;
                self.selection = None;
                self.set_notice("búsqueda cerrada");
            }
            PhysicalKey::Code(KeyCode::Backspace) => {
                if let Some(query) = &mut self.search_query {
                    query.pop();
                    self.search_match = 0;
                }
                self.focus_search_match(0);
            }
            PhysicalKey::Code(KeyCode::Enter) => {
                self.focus_search_match(if self.modifiers.shift_key() { -1 } else { 1 })
            }
            _ => {}
        }
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn cursor_at(&self, x: f32, y: f32) -> Option<BlockCursor> {
        if self.document.mode == DocumentMode::Split
            && self
                .window
                .as_ref()
                .is_some_and(|window| x >= window.inner_size().width as f32 / 2.0)
        {
            return None;
        }
        self.slots.iter().enumerate().find_map(|(block, slot)| {
            let top = slot.y - self.scroll;
            if y < top || y > top + slot.height {
                return None;
            }
            let (CachedBlockLayout::Text(layout), _) = self.live.get(&block)? else {
                // La selección por celda requiere mapear rangos de fuente y
                // geometría independientes. Hasta incorporarlo, no mentimos
                // devolviendo el offset del texto aplanado de una tabla.
                return None;
            };
            let cursor = Cursor::from_point(layout, x - slot.x, y - top);
            Some(BlockCursor {
                block,
                offset: cursor.index(),
            })
        })
    }

    fn task_at(&self, x: f32, y: f32) -> Option<usize> {
        self.slots.iter().enumerate().find_map(|(index, slot)| {
            let top = slot.y - self.scroll;
            let is_task = matches!(
                self.document.blocks[index].marker,
                Some(Marker::Task { .. })
            );
            let marker_left = slot.x - 28.0 * self.scale_factor;
            (is_task
                && (top..=top + slot.height).contains(&y)
                && (marker_left..=slot.x).contains(&x))
            .then_some(index)
        })
    }

    fn code_copy_at(&self, x: f32, y: f32) -> Option<usize> {
        let window = self.window.as_ref()?;
        let text_width = (window.inner_size().width as f32 - MARGIN * self.scale_factor * 2.0)
            .min(MAX_MEASURE * self.scale_factor);
        self.slots.iter().enumerate().find_map(|(index, slot)| {
            if !matches!(slot.kind, Kind::Code) {
                return None;
            }
            let (left, top, width, height) =
                code_copy_bounds(slot, text_width, self.scroll, self.scale_factor);
            ((left..=left + width).contains(&x) && (top..=top + height).contains(&y))
                .then_some(index)
        })
    }

    fn copy_code_block(&mut self, block: usize) {
        let Some(source) = self.document.blocks.get(block).and_then(|block| {
            self.document
                .source
                .slice_bytes(block.source.start..block.source.end)
                .ok()
        }) else {
            self.set_notice("no se pudo localizar el bloque de código en la fuente");
            return;
        };
        self.copy_text_to_clipboard(source, "bloque de código copiado");
    }

    fn toggle_task(&mut self, block: usize) {
        let Some(block) = self.document.blocks.get(block) else {
            return;
        };
        let source = block.source;
        let Ok(text) = self.document.source.slice_bytes(source.start..source.end) else {
            self.set_notice("no se pudo localizar la tarea en la fuente");
            return;
        };
        let Some((offset, replacement)) = task_marker_replacement(&text) else {
            self.set_notice("la tarea no conserva un marcador editable");
            return;
        };
        let start = source.start + offset;
        self.edit_source(|editor, source| {
            editor.set_cursor(source, start, false)?;
            editor.set_cursor(source, start + 1, true)?;
            editor.insert(source, replacement)
        });
        self.selection = None;
        self.set_notice("tarea actualizada · Ctrl+Z para deshacer");
    }

    fn target_at(&self, x: f32, y: f32) -> Option<&InlineTarget> {
        let cursor = self.cursor_at(x, y)?;
        self.document
            .blocks
            .get(cursor.block)?
            .targets
            .iter()
            .find(|target| {
                target.kind.is_navigable() && (target.start..target.end).contains(&cursor.offset)
            })
    }

    fn extend_selection_to(&mut self, x: f32, y: f32) {
        if self.document.mode.is_editable() {
            if let Some(cursor) = self.cursor_at(x, y) {
                self.set_source_cursor_from_block(cursor, true);
            }
            return;
        }
        let Some(selection) = self.selection else {
            return;
        };
        let Some(focus) = self.cursor_at(x, y) else {
            return;
        };
        self.selection = Some(DocumentSelection {
            anchor: selection.anchor,
            focus,
        });
    }

    fn move_selection_boundary(&mut self, end: bool, document: bool, extend: bool) {
        let Some(selection) = self.selection else {
            return;
        };
        let block = if document {
            if end {
                self.document.blocks.len().saturating_sub(1)
            } else {
                0
            }
        } else {
            selection.focus.block
        };
        let Some(block_text) = self.document.blocks.get(block).map(|block| &block.text) else {
            return;
        };
        let cursor = BlockCursor {
            block,
            offset: if end { block_text.len() } else { 0 },
        };
        self.selection = Some(if extend {
            DocumentSelection {
                anchor: selection.anchor,
                focus: cursor,
            }
        } else {
            DocumentSelection::collapsed(cursor)
        });
    }

    fn scroll_page(&mut self, down: bool) {
        let Some(window) = &self.window else {
            return;
        };
        let viewport = window.inner_size().height as f32;
        let step = (viewport * 0.88).max(1.0);
        let delta = if down { step } else { -step };
        self.scroll = (self.scroll + delta).clamp(
            0.0,
            max_scroll(self.doc_height.max(self.preview_height), viewport),
        );
        window.request_redraw();
    }

    fn move_selection_visually(&mut self, forward: bool, extend: bool) {
        let Some(selection) = self.selection else {
            return;
        };
        if !extend && selection.anchor.block != selection.focus.block {
            let boundary = if forward {
                selection.anchor.max(selection.focus)
            } else {
                selection.anchor.min(selection.focus)
            };
            self.selection = Some(DocumentSelection::collapsed(boundary));
            return;
        }
        let Some((CachedBlockLayout::Text(layout), _)) = self.live.get(&selection.focus.block)
        else {
            return;
        };
        let current = Selection::new(
            Cursor::from_byte_index(layout, selection.anchor.offset, Affinity::Downstream),
            Cursor::from_byte_index(layout, selection.focus.offset, Affinity::Downstream),
        );
        let next = if forward {
            current.next_visual(layout, extend)
        } else {
            current.previous_visual(layout, extend)
        };
        self.selection = Some(DocumentSelection {
            anchor: BlockCursor {
                block: selection.focus.block,
                offset: next.anchor().index(),
            },
            focus: BlockCursor {
                block: selection.focus.block,
                offset: next.focus().index(),
            },
        });
    }

    fn select_document(&mut self) {
        let Some(last) = self.document.blocks.len().checked_sub(1) else {
            return;
        };
        debug_assert!(
            self.document
                .blocks
                .iter()
                .all(|block| block.source.is_valid_for_buffer(&self.document.source)),
            "el modelo no conserva rangos validos para la fuente de la sesion"
        );
        self.selection = Some(DocumentSelection {
            anchor: BlockCursor {
                block: 0,
                offset: 0,
            },
            focus: BlockCursor {
                block: last,
                offset: self.document.blocks[last].text.len(),
            },
        });
    }

    fn move_selection_line(&mut self, down: bool, extend: bool) {
        let Some(selection) = self.selection else {
            return;
        };
        if !extend && selection.anchor.block != selection.focus.block {
            let boundary = if down {
                selection.anchor.max(selection.focus)
            } else {
                selection.anchor.min(selection.focus)
            };
            self.selection = Some(DocumentSelection::collapsed(boundary));
            return;
        }
        let Some((CachedBlockLayout::Text(layout), _)) = self.live.get(&selection.focus.block)
        else {
            return;
        };
        let focus = Cursor::from_byte_index(layout, selection.focus.offset, Affinity::Downstream);
        let current = if selection.anchor.block == selection.focus.block {
            Selection::new(
                Cursor::from_byte_index(layout, selection.anchor.offset, Affinity::Downstream),
                focus,
            )
        } else {
            Selection::new(focus, focus)
        };
        let next = if down {
            current.next_line(layout, extend)
        } else {
            current.previous_line(layout, extend)
        };
        self.selection = Some(DocumentSelection {
            anchor: if extend && selection.anchor.block != selection.focus.block {
                selection.anchor
            } else {
                BlockCursor {
                    block: selection.focus.block,
                    offset: next.anchor().index(),
                }
            },
            focus: BlockCursor {
                block: selection.focus.block,
                offset: next.focus().index(),
            },
        });
    }

    /// Desplaza mientras se arrastra cerca de un borde. El foco se recalcula
    /// contra el layout visible de este cuadro, nunca contra coordenadas de
    /// documento estimadas.
    fn autoscroll_selection(&mut self, viewport_height: f32) -> bool {
        if !self.selecting {
            return false;
        }
        let Some((x, y)) = self.pointer else {
            return false;
        };
        let delta = selection_scroll_delta(y, viewport_height);
        if delta == 0.0 {
            return false;
        }
        let next = (self.scroll + delta).clamp(
            0.0,
            max_scroll(self.doc_height.max(self.preview_height), viewport_height),
        );
        if (next - self.scroll).abs() < f32::EPSILON {
            return false;
        }
        self.scroll = next;
        self.extend_selection_to(x, y);
        true
    }

    fn redraw(&mut self) -> Result<(), String> {
        let Some(window) = self.window.clone() else {
            return Ok(());
        };
        let size = window.inner_size();
        let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else {
            return Ok(());
        };

        let frame_start = Instant::now();
        let split = self.document.mode == DocumentMode::Split;
        let layout_width = content_layout_width(size.width as f32, self.document.mode);

        // Re-medir solo si cambio el ancho.
        if self.exact_after_edit || layout_width_is_stale(self.laid_for_width, layout_width) {
            let t = Instant::now();
            let (slots, height) = measure_all(
                &self.document.blocks,
                &mut self.font_cx,
                &mut self.layout_cx,
                layout_width,
                self.scale_factor,
                self.exact_measure || self.exact_after_edit,
            );
            self.slots = slots;
            self.doc_height = height;
            self.laid_for_width = layout_width;
            self.exact_after_edit = false;
            self.scroll = self.scroll.min(max_scroll(
                self.doc_height.max(self.preview_height),
                size.height as f32,
            ));
            self.live.clear();
            self.log.push(format!(
                "[medicion] posicionar {} bloques ({}): {:.0} ms  (alto {:.0} px)",
                self.document.blocks.len(),
                if self.exact_measure {
                    "exacto"
                } else {
                    "estimado"
                },
                t.elapsed().as_secs_f64() * 1000.0,
                height
            ));
        }

        if split && layout_width_is_stale(self.preview_laid_for_width, layout_width) {
            let (slots, height) = measure_all(
                &self.document.rendered_blocks,
                &mut self.font_cx,
                &mut self.layout_cx,
                layout_width,
                self.scale_factor,
                false,
            );
            self.preview_slots = slots;
            self.preview_height = height;
            self.preview_laid_for_width = layout_width;
            self.preview_live.clear();
            self.scroll = self.scroll.min(max_scroll(
                self.doc_height.max(self.preview_height),
                size.height as f32,
            ));
        }

        if let Some(heading) = self.document.pending_heading.take() {
            self.scroll_to_heading(&heading);
        }

        // Que bloques caen en pantalla este cuadro.
        let view_top = self.scroll;
        let view_bottom = self.scroll + size.height as f32;
        let visible = visible_range(&self.slots, view_top, view_bottom);
        let preview_visible = if split {
            visible_range(&self.preview_slots, view_top, view_bottom)
        } else {
            0..0
        };

        // Solo los visibles conservan su layout vivo.
        self.live.retain(|i, _| visible.contains(i));
        for i in visible.clone() {
            if !self.live.contains_key(&i) {
                let block = &self.document.blocks[i];
                let layout = if matches!(block.kind, Kind::TableRow { .. }) {
                    CachedBlockLayout::Table(build_table_layouts(
                        block,
                        &mut self.font_cx,
                        &mut self.layout_cx,
                        layout_width,
                        self.scale_factor,
                        self.palette,
                    ))
                } else {
                    CachedBlockLayout::Text(Box::new(build_layout(
                        block,
                        &mut self.font_cx,
                        &mut self.layout_cx,
                        layout_width,
                        self.scale_factor,
                        self.palette,
                    )))
                };
                let marker = block.marker.as_ref().map(|m| match m {
                    Marker::Text(text) => CachedMarker::Text(Box::new(build_marker_layout(
                        text,
                        block.kind,
                        &mut self.font_cx,
                        &mut self.layout_cx,
                        self.scale_factor,
                        self.palette,
                    ))),
                    Marker::Task { done } => CachedMarker::Task { done: *done },
                });
                self.live.insert(i, (layout, marker));
            }
        }

        self.preview_live
            .retain(|index, _| preview_visible.contains(index));
        for index in preview_visible.clone() {
            if !self.preview_live.contains_key(&index) {
                let block = &self.document.rendered_blocks[index];
                let layout = if matches!(block.kind, Kind::TableRow { .. }) {
                    CachedBlockLayout::Table(build_table_layouts(
                        block,
                        &mut self.font_cx,
                        &mut self.layout_cx,
                        layout_width,
                        self.scale_factor,
                        self.palette,
                    ))
                } else {
                    CachedBlockLayout::Text(Box::new(build_layout(
                        block,
                        &mut self.font_cx,
                        &mut self.layout_cx,
                        layout_width,
                        self.scale_factor,
                        self.palette,
                    )))
                };
                let marker = block.marker.as_ref().map(|marker| match marker {
                    Marker::Text(text) => CachedMarker::Text(Box::new(build_marker_layout(
                        text,
                        block.kind,
                        &mut self.font_cx,
                        &mut self.layout_cx,
                        self.scale_factor,
                        self.palette,
                    ))),
                    Marker::Task { done } => CachedMarker::Task { done: *done },
                });
                self.preview_live.insert(index, (layout, marker));
            }
        }

        if self.autoscroll_selection(size.height as f32) {
            window.request_redraw();
        }

        let menu = self.context_menu;
        let menu_pointer = self.pointer;
        let safe_banner = self.document.safe_mode.map(|reason| {
            build_menu_layout(
                &safe_mode_label(reason),
                &mut self.font_cx,
                &mut self.layout_cx,
                self.palette,
            )
        });
        let search_query = self.search_query.clone();
        let search_overlay = search_query.as_ref().map(|query| {
            let matches = self.search_matches().len();
            let label = if query.is_empty() {
                "Buscar en documento…".to_string()
            } else {
                format!("Buscar: {query} · {matches} resultados")
            };
            let label = abbreviated_label(&label, MAX_OVERLAY_LABEL_CHARS);
            build_menu_layout(&label, &mut self.font_cx, &mut self.layout_cx, self.palette)
        });
        let workspace_search_query = self.workspace_search_query.clone();
        let workspace_search_results = workspace_search_query
            .as_ref()
            .map(|_| self.workspace_search_matches());
        let palette_actions = self.command_palette.map(|_| self.command_palette_actions());
        let navigation_panel_rows = if let (Some(selected), Some(actions)) =
            (self.command_palette, palette_actions.as_ref())
        {
            let selected = selected % actions.len().max(1);
            let range = panel_window(actions.len(), selected, PANEL_CAPACITY);
            let title = if self.command_palette_query.is_empty() {
                "Acciones · escribe para filtrar".to_string()
            } else {
                format!(
                    "Acciones: {} · {} resultados",
                    self.command_palette_query,
                    actions.len()
                )
            };
            let mut rows = vec![(false, abbreviated_label(&title, 48))];
            rows.extend(range.map(|index| {
                (
                    index == selected,
                    abbreviated_label(actions[index].label(), 48),
                )
            }));
            Some(rows)
        } else if let (Some(query), Some(paths)) = (
            workspace_search_query.as_ref(),
            workspace_search_results.as_ref(),
        ) {
            let selected = self.workspace_search_match % paths.len().max(1);
            let range = panel_window(paths.len(), selected, PANEL_CAPACITY);
            let title = if query.is_empty() {
                "Buscar en carpeta…".to_string()
            } else {
                format!("Buscar: {query} · {} resultados", paths.len())
            };
            let mut rows = vec![(false, abbreviated_label(&title, 48))];
            rows.extend(range.map(|index| {
                (
                    index == selected,
                    abbreviated_label(&paths[index].display().to_string(), 48),
                )
            }));
            Some(rows)
        } else if let Some(paths) = self.workspace_paths.as_ref() {
            let selected = self.workspace_path_match % paths.len().max(1);
            let range = panel_window(paths.len(), selected, PANEL_CAPACITY);
            let note_count = paths
                .iter()
                .filter(|row| matches!(row.kind, WorkspaceTreeKind::Note))
                .count();
            let mut rows = vec![(false, format!("Notas · {note_count} archivos"))];
            rows.extend(range.map(|index| {
                (
                    index == selected,
                    abbreviated_label(&paths[index].label(), 48),
                )
            }));
            Some(rows)
        } else if let Some(paths) = self.backlink_paths.as_ref() {
            let selected = self.backlink_match % paths.len().max(1);
            let range = panel_window(paths.len(), selected, PANEL_CAPACITY);
            let mut rows = vec![(false, format!("Backlinks · {} resultados", paths.len()))];
            rows.extend(range.map(|index| {
                (
                    index == selected,
                    abbreviated_label(&paths[index].display().to_string(), 48),
                )
            }));
            Some(rows)
        } else if let Some(headings) = self.outline_headings.as_ref() {
            let selected = self.outline_match % headings.len().max(1);
            let range = panel_window(headings.len(), selected, PANEL_CAPACITY);
            let mut rows = vec![(false, format!("Índice · {} encabezados", headings.len()))];
            rows.extend(
                range.map(|index| (index == selected, abbreviated_label(&headings[index].1, 48))),
            );
            Some(rows)
        } else {
            None
        };
        let navigation_panel_layouts = navigation_panel_rows.as_ref().map(|rows| {
            rows.iter()
                .map(|(_, label)| {
                    build_menu_layout(label, &mut self.font_cx, &mut self.layout_cx, self.palette)
                })
                .collect::<Vec<_>>()
        });
        let menu_layouts = menu.map(|_| {
            context_actions(self.document.mode)
                .iter()
                .map(|action| {
                    build_menu_layout(
                        action.label(),
                        &mut self.font_cx,
                        &mut self.layout_cx,
                        self.palette,
                    )
                })
                .collect::<Vec<_>>()
        });
        let code_copy_layout = build_menu_layout(
            "Copiar",
            &mut self.font_cx,
            &mut self.layout_cx,
            self.palette,
        );
        let toolbar_layouts = TOOLBAR_ACTIONS
            .iter()
            .map(|action| {
                build_menu_layout(
                    action.short_label(self.document.mode),
                    &mut self.font_cx,
                    &mut self.layout_cx,
                    self.palette,
                )
            })
            .collect::<Vec<_>>();
        let toolbar_focus = self.toolbar_focus;
        let document_mode_label = match self.document.mode {
            DocumentMode::Reading => "Lectura",
            DocumentMode::SourceEditing => "Edición",
            DocumentMode::Split => "Fuente + lectura",
        };
        let document_state_label = if self.open_requests.contains_key(&self.document.id) {
            "abriendo"
        } else if self.view_requests.contains_key(&self.document.id) {
            "actualizando vista"
        } else if self.saves_in_flight.contains(&self.document.id) {
            "guardando"
        } else if self.document.is_dirty() {
            "sin guardar"
        } else {
            "guardado"
        };
        let workspace_state_label = if self.workspace_stale {
            "carpeta por actualizar"
        } else if self.workspace.is_some() {
            "carpeta activa"
        } else {
            "sin carpeta"
        };
        let tab_width = tab_width(w.get() as f32, self.tab_order.len());
        let tab_label_chars = ((tab_width - 32.0).max(30.0) / 7.5).floor().max(4.0) as usize;
        let tab_descriptors = self
            .tab_order
            .iter()
            .filter_map(|id| {
                if *id == self.document.id {
                    Some((
                        true,
                        tab_label(
                            &self.document.path,
                            self.document.is_dirty(),
                            tab_label_chars,
                        ),
                    ))
                } else {
                    self.inactive_documents
                        .iter()
                        .find(|tab| tab.document.id == *id)
                        .map(|tab| {
                            (
                                false,
                                tab_label(
                                    &tab.document.path,
                                    tab.document.is_dirty(),
                                    tab_label_chars,
                                ),
                            )
                        })
                }
            })
            .collect::<Vec<_>>();
        let tab_layouts = tab_descriptors
            .iter()
            .map(|(_, label)| {
                build_menu_layout(label, &mut self.font_cx, &mut self.layout_cx, self.palette)
            })
            .collect::<Vec<_>>();
        let status_layout = build_menu_layout(
            &format!(
                "{document_mode_label} · {document_state_label} · {workspace_state_label} · Ctrl+Shift+P acciones"
            ),
            &mut self.font_cx,
            &mut self.layout_cx,
            self.palette,
        );

        // El pixmap se reusa entre cuadros: reservar 2,7 MB por cuadro y
        // ponerlos en cero es trabajo que no hace falta repetir.
        let needs_new = self
            .pixmap
            .as_ref()
            .is_none_or(|p| p.width() != w.get() || p.height() != h.get());
        if needs_new {
            self.pixmap = Some(
                Pixmap::new(w.get(), h.get())
                    .ok_or_else(|| "no hay memoria para el framebuffer".to_string())?,
            );
        }
        // Se desarma `self` en campos sueltos para que el prestamo del pixmap
        // no choque con el de la cache de glifos ni con el de los bloques.
        let context_mode = self.document.mode;
        let App {
            pixmap,
            slots,
            live,
            preview_slots,
            preview_live,
            scale_cx,
            glyphs,
            scroll,
            surface,
            palette,
            selection,
            focused_link,
            scale_factor,
            document,
            ..
        } = self;
        let blocks = &document.blocks;
        let pixmap = pixmap
            .as_mut()
            .ok_or_else(|| "el framebuffer no esta disponible".to_string())?;
        let bg = palette.bg;
        let surface_color = palette.surface;
        pixmap.fill(Color::from_rgba8(bg.0, bg.1, bg.2, 255));

        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba8(
            surface_color.0,
            surface_color.1,
            surface_color.2,
            255,
        ));

        let mut accent_paint = Paint::default();
        let ac = palette.accent;
        accent_paint.set_color(Color::from_rgba8(ac.0, ac.1, ac.2, 255));

        let mut dim_paint = Paint::default();
        let dc = palette.dim;
        dim_paint.set_color(Color::from_rgba8(dc.0, dc.1, dc.2, 160));

        let ancho_texto =
            (layout_width - MARGIN * *scale_factor * 2.0).min(MAX_MEASURE * *scale_factor);

        for i in visible {
            let slot = &slots[i];
            let Some((cached_layout, marker)) = live.get(&i) else {
                continue;
            };
            let top = slot.y - *scroll;

            if let CachedBlockLayout::Table(cells) = cached_layout {
                let columns = cells.len().max(1) as f32;
                let left = slot.x - 6.0;
                let table_width = ancho_texto + 12.0;
                let mut line = Paint::default();
                let dc = palette.dim;
                line.set_color(Color::from_rgba8(dc.0, dc.1, dc.2, 112));
                if matches!(slot.kind, Kind::TableRow { header: true }) {
                    let ac = palette.accent;
                    let mut header = Paint::default();
                    header.set_color(Color::from_rgba8(ac.0, ac.1, ac.2, 26));
                    if let Some(rect) =
                        Rect::from_xywh(left, top - 1.0, table_width, slot.height + 2.0)
                    {
                        pixmap.fill_rect(rect, &header, Transform::identity(), None);
                    }
                }
                for (x, y, width, height) in [
                    (left, top - 1.0, table_width, 1.0),
                    (left, top + slot.height, table_width, 1.0),
                    (left, top - 1.0, 1.0, slot.height + 2.0),
                    (left + table_width - 1.0, top - 1.0, 1.0, slot.height + 2.0),
                ] {
                    if let Some(rect) = Rect::from_xywh(x, y, width, height) {
                        pixmap.fill_rect(rect, &line, Transform::identity(), None);
                    }
                }
                for column in 1..columns as usize {
                    let x = left + table_width * column as f32 / columns;
                    if let Some(rect) = Rect::from_xywh(x, top - 1.0, 1.0, slot.height + 2.0) {
                        pixmap.fill_rect(rect, &line, Transform::identity(), None);
                    }
                }
                let column_width = ancho_texto / columns;
                for (column, layout) in cells.iter().enumerate() {
                    let x = slot.x + column as f32 * column_width + TABLE_CELL_PADDING;
                    let y = top + (slot.height - layout.height()).max(0.0) * 0.5;
                    for line in layout.lines() {
                        for entry in line.items() {
                            if let PositionedLayoutItem::GlyphRun(run) = entry {
                                draw_run_background(pixmap, &run, x, y);
                                draw_decorations(pixmap, &run, x, y);
                                draw_glyph_run(pixmap, scale_cx, glyphs, &run, x, y);
                            }
                        }
                    }
                }
                continue;
            }
            let CachedBlockLayout::Text(layout) = cached_layout else {
                continue;
            };

            match slot.kind {
                // Fondo de los bloques de codigo, dibujado con tiny-skia.
                Kind::Code => {
                    if let Some(rect) = Rect::from_xywh(
                        slot.x - 12.0,
                        top - 2.0,
                        ancho_texto + 24.0,
                        slot.height + 4.0,
                    ) {
                        pixmap.fill_rect(rect, &paint, Transform::identity(), None);
                    }
                    if context_mode == DocumentMode::Reading {
                        let (x, y, width, height) =
                            code_copy_bounds(slot, ancho_texto, *scroll, *scale_factor);
                        let ac = palette.accent;
                        let mut button = Paint::default();
                        button.set_color(Color::from_rgba8(ac.0, ac.1, ac.2, 42));
                        if let Some(rect) = Rect::from_xywh(x, y, width, height) {
                            pixmap.fill_rect(rect, &button, Transform::identity(), None);
                        }
                        for line in code_copy_layout.lines() {
                            for entry in line.items() {
                                if let PositionedLayoutItem::GlyphRun(run) = entry {
                                    draw_run_background(pixmap, &run, x + 8.0, y + 4.0);
                                    draw_glyph_run(
                                        pixmap,
                                        scale_cx,
                                        glyphs,
                                        &run,
                                        x + 8.0,
                                        y + 4.0,
                                    );
                                }
                            }
                        }
                    }
                }
                // Filete de acento a la izquierda de la cita, como las alertas.
                Kind::Quote => {
                    if let Some(rect) = Rect::from_xywh(slot.x - 20.0, top, 3.0, slot.height) {
                        pixmap.fill_rect(rect, &accent_paint, Transform::identity(), None);
                    }
                }
                // Un callout es una cita semántica con una superficie tenue y
                // rótulo nativo. No interpreta atributos de Obsidian ni crea
                // estado interactivo.
                Kind::Callout => {
                    if let Some(rect) = Rect::from_xywh(
                        slot.x - 20.0,
                        top - 4.0,
                        ancho_texto + 20.0,
                        slot.height + 8.0,
                    ) {
                        pixmap.fill_rect(rect, &paint, Transform::identity(), None);
                    }
                    if let Some(rect) =
                        Rect::from_xywh(slot.x - 20.0, top - 4.0, 3.0, slot.height + 8.0)
                    {
                        pixmap.fill_rect(rect, &accent_paint, Transform::identity(), None);
                    }
                }
                // Linea horizontal: un filete tenue, no un borde grueso.
                Kind::Rule => {
                    if let Some(rect) = Rect::from_xywh(slot.x, top, ancho_texto, 1.0) {
                        pixmap.fill_rect(rect, &dim_paint, Transform::identity(), None);
                    }
                    continue;
                }
                _ => {}
            }

            // La vineta va en el margen, a la izquierda del texto, para que
            // las lineas siguientes de un item largo queden bajo el texto.
            if let Some(marker) = marker {
                match marker {
                    CachedMarker::Text(marker) => {
                        let ancho_marca = marker.width();
                        for line in marker.lines() {
                            for entry in line.items() {
                                if let PositionedLayoutItem::GlyphRun(run) = entry {
                                    draw_run_background(
                                        pixmap,
                                        &run,
                                        slot.x - ancho_marca - 8.0,
                                        top,
                                    );
                                    draw_glyph_run(
                                        pixmap,
                                        scale_cx,
                                        glyphs,
                                        &run,
                                        slot.x - ancho_marca - 8.0,
                                        top,
                                    );
                                }
                            }
                        }
                    }
                    CachedMarker::Task { done } => {
                        let (font_size, _, _, _) = slot.kind.style();
                        let size = (font_size * *scale_factor * 0.82).max(12.0 * *scale_factor);
                        let line_box = font_size * *scale_factor * slot.kind.line_height();
                        draw_checkbox(
                            pixmap,
                            slot.x - size - 8.0,
                            top + (line_box - size).max(0.0) * 0.5,
                            size,
                            *done,
                            *palette,
                        );
                    }
                }
            }

            if let Some((start, end)) =
                selection.and_then(|selection| selection.range_for(i, blocks[i].text.len()))
            {
                let ac = palette.accent;
                let mut selection_paint = Paint::default();
                selection_paint.set_color(Color::from_rgba8(ac.0, ac.1, ac.2, 92));
                let geometry = if start == end {
                    let focus = selection
                        .filter(|selection| selection.focus.block == i)
                        .map(|selection| {
                            Cursor::from_byte_index(
                                layout,
                                selection.focus.offset,
                                Affinity::Downstream,
                            )
                        });
                    focus.map_or_else(Vec::new, |focus| vec![(focus.geometry(layout, 1.25), 0)])
                } else {
                    Selection::new(
                        Cursor::from_byte_index(layout, start, Affinity::Downstream),
                        Cursor::from_byte_index(layout, end, Affinity::Downstream),
                    )
                    .geometry(layout)
                };
                for (rect, _) in geometry {
                    let Some(rect) = Rect::from_xywh(
                        slot.x + rect.x0 as f32,
                        top + rect.y0 as f32,
                        rect.width() as f32,
                        rect.height() as f32,
                    ) else {
                        continue;
                    };
                    pixmap.fill_rect(rect, &selection_paint, Transform::identity(), None);
                }
            }

            if let Some((focus_block, focus_target)) = *focused_link
                && focus_block == i
                && let Some(target) = blocks[i].targets.get(focus_target)
            {
                let ac = palette.accent;
                let mut focus_paint = Paint::default();
                focus_paint.set_color(Color::from_rgba8(ac.0, ac.1, ac.2, 48));
                for (rect, _) in Selection::new(
                    Cursor::from_byte_index(layout, target.start, Affinity::Downstream),
                    Cursor::from_byte_index(layout, target.end, Affinity::Downstream),
                )
                .geometry(layout)
                {
                    let Some(rect) = Rect::from_xywh(
                        slot.x + rect.x0 as f32,
                        top + rect.y0 as f32,
                        rect.width() as f32,
                        rect.height() as f32,
                    ) else {
                        continue;
                    };
                    pixmap.fill_rect(rect, &focus_paint, Transform::identity(), None);
                }
            }

            for line in layout.lines() {
                for entry in line.items() {
                    if let PositionedLayoutItem::GlyphRun(run) = entry {
                        draw_run_background(pixmap, &run, slot.x, top);
                        draw_decorations(pixmap, &run, slot.x, top);
                        draw_glyph_run(pixmap, scale_cx, glyphs, &run, slot.x, top);
                    }
                }
            }
        }

        if split {
            let pane_x = layout_width;
            if let Some(rect) = Rect::from_xywh(pane_x - 1.0, 0.0, 1.0, h.get() as f32) {
                pixmap.fill_rect(rect, &dim_paint, Transform::identity(), None);
            }
            for index in preview_visible {
                let slot = &preview_slots[index];
                let Some((cached_layout, marker)) = preview_live.get(&index) else {
                    continue;
                };
                let top = slot.y - *scroll;
                if let CachedBlockLayout::Table(cells) = cached_layout {
                    let columns = cells.len().max(1) as f32;
                    let left = pane_x + slot.x - 6.0;
                    let table_width = ancho_texto + 12.0;
                    let mut line_paint = Paint::default();
                    let dim = palette.dim;
                    line_paint.set_color(Color::from_rgba8(dim.0, dim.1, dim.2, 112));
                    for (x, y, width, height) in [
                        (left, top - 1.0, table_width, 1.0),
                        (left, top + slot.height, table_width, 1.0),
                        (left, top - 1.0, 1.0, slot.height + 2.0),
                        (left + table_width - 1.0, top - 1.0, 1.0, slot.height + 2.0),
                    ] {
                        if let Some(rect) = Rect::from_xywh(x, y, width, height) {
                            pixmap.fill_rect(rect, &line_paint, Transform::identity(), None);
                        }
                    }
                    for column in 1..columns as usize {
                        let x = left + table_width * column as f32 / columns;
                        if let Some(rect) = Rect::from_xywh(x, top - 1.0, 1.0, slot.height + 2.0) {
                            pixmap.fill_rect(rect, &line_paint, Transform::identity(), None);
                        }
                    }
                    let column_width = ancho_texto / columns;
                    for (column, layout) in cells.iter().enumerate() {
                        let x = pane_x + slot.x + column as f32 * column_width + TABLE_CELL_PADDING;
                        let y = top + (slot.height - layout.height()).max(0.0) * 0.5;
                        for line in layout.lines() {
                            for entry in line.items() {
                                if let PositionedLayoutItem::GlyphRun(run) = entry {
                                    draw_run_background(pixmap, &run, x, y);
                                    draw_decorations(pixmap, &run, x, y);
                                    draw_glyph_run(pixmap, scale_cx, glyphs, &run, x, y);
                                }
                            }
                        }
                    }
                    continue;
                }
                let CachedBlockLayout::Text(layout) = cached_layout else {
                    continue;
                };
                let x = pane_x + slot.x;
                match slot.kind {
                    Kind::Code | Kind::Callout => {
                        if let Some(rect) = Rect::from_xywh(
                            x - 12.0,
                            top - 2.0,
                            ancho_texto + 24.0,
                            slot.height + 4.0,
                        ) {
                            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
                        }
                    }
                    Kind::Quote => {
                        if let Some(rect) = Rect::from_xywh(x - 20.0, top, 3.0, slot.height) {
                            pixmap.fill_rect(rect, &accent_paint, Transform::identity(), None);
                        }
                    }
                    Kind::Rule => {
                        if let Some(rect) = Rect::from_xywh(x, top, ancho_texto, 1.0) {
                            pixmap.fill_rect(rect, &dim_paint, Transform::identity(), None);
                        }
                        continue;
                    }
                    _ => {}
                }
                if let Some(marker) = marker {
                    match marker {
                        CachedMarker::Text(marker) => {
                            let marker_x = x - marker.width() - 8.0;
                            for line in marker.lines() {
                                for entry in line.items() {
                                    if let PositionedLayoutItem::GlyphRun(run) = entry {
                                        draw_glyph_run(
                                            pixmap, scale_cx, glyphs, &run, marker_x, top,
                                        );
                                    }
                                }
                            }
                        }
                        CachedMarker::Task { done } => {
                            let (font_size, _, _, _) = slot.kind.style();
                            let size = (font_size * *scale_factor * 0.82).max(12.0 * *scale_factor);
                            draw_checkbox(pixmap, x - size - 8.0, top, size, *done, *palette);
                        }
                    }
                }
                for line in layout.lines() {
                    for entry in line.items() {
                        if let PositionedLayoutItem::GlyphRun(run) = entry {
                            draw_run_background(pixmap, &run, x, top);
                            draw_decorations(pixmap, &run, x, top);
                            draw_glyph_run(pixmap, scale_cx, glyphs, &run, x, top);
                        }
                    }
                }
            }
        }

        for (index, layout) in toolbar_layouts.iter().enumerate() {
            let x = TOOLBAR_X + index as f32 * TOOLBAR_ITEM_WIDTH;
            let width = TOOLBAR_ITEM_WIDTH - 4.0;
            if x + width > w.get() as f32 - MARGIN {
                break;
            }
            let hovered = menu_pointer.is_some_and(|(pointer_x, pointer_y)| {
                pointer_x >= x
                    && pointer_x < x + width
                    && (TOOLBAR_Y..TOOLBAR_Y + TOOLBAR_HEIGHT).contains(&pointer_y)
            });
            let focused = toolbar_focus == Some(index);
            let active = TOOLBAR_ACTIONS[index] == AppAction::ToggleMode;
            if (hovered || focused || active)
                && let Some(rect) = Rect::from_xywh(x, TOOLBAR_Y, width, TOOLBAR_HEIGHT)
            {
                if active {
                    let ac = palette.accent;
                    let mut active_paint = Paint::default();
                    active_paint.set_color(Color::from_rgba8(ac.0, ac.1, ac.2, 36));
                    pixmap.fill_rect(rect, &active_paint, Transform::identity(), None);
                } else {
                    pixmap.fill_rect(rect, &paint, Transform::identity(), None);
                }
            }
            if (active || focused)
                && let Some(rect) = Rect::from_xywh(x, TOOLBAR_Y + TOOLBAR_HEIGHT - 2.0, width, 2.0)
            {
                pixmap.fill_rect(rect, &accent_paint, Transform::identity(), None);
            }
            for line in layout.lines() {
                for entry in line.items() {
                    if let PositionedLayoutItem::GlyphRun(run) = entry {
                        draw_run_background(pixmap, &run, x + 9.0, TOOLBAR_Y + 6.0);
                        draw_glyph_run(pixmap, scale_cx, glyphs, &run, x + 9.0, TOOLBAR_Y + 6.0);
                    }
                }
            }
        }

        if let Some(layout) = safe_banner {
            let banner_width = (layout.width() + 24.0).min(w.get() as f32 - MARGIN * 2.0);
            if let Some(rect) = Rect::from_xywh(MARGIN, 44.0, banner_width, 28.0) {
                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            }
            if let Some(rect) = Rect::from_xywh(MARGIN, 44.0, 3.0, 28.0) {
                pixmap.fill_rect(rect, &accent_paint, Transform::identity(), None);
            }
            for line in layout.lines() {
                for entry in line.items() {
                    if let PositionedLayoutItem::GlyphRun(run) = entry {
                        draw_run_background(pixmap, &run, MARGIN + 10.0, 49.0);
                        draw_glyph_run(pixmap, scale_cx, glyphs, &run, MARGIN + 10.0, 49.0);
                    }
                }
            }
        }

        let status_y = h.get() as f32 - STATUS_HEIGHT;
        if let Some(rect) = Rect::from_xywh(0.0, status_y, w.get() as f32, STATUS_HEIGHT) {
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
        }
        for (index, ((active, _), layout)) in
            tab_descriptors.iter().zip(tab_layouts.iter()).enumerate()
        {
            let x = index as f32 * tab_width;
            if *active && let Some(rect) = Rect::from_xywh(x, status_y, tab_width, 3.0) {
                pixmap.fill_rect(rect, &accent_paint, Transform::identity(), None);
            }
            if index > 0
                && let Some(rect) = Rect::from_xywh(x, status_y + 6.0, 1.0, 16.0)
            {
                pixmap.fill_rect(rect, &dim_paint, Transform::identity(), None);
            }
            for line in layout.lines() {
                for entry in line.items() {
                    if let PositionedLayoutItem::GlyphRun(run) = entry {
                        draw_run_background(pixmap, &run, x + 10.0, status_y + 7.0);
                        draw_glyph_run(pixmap, scale_cx, glyphs, &run, x + 10.0, status_y + 7.0);
                    }
                }
            }
            let close_x = x + tab_width - 14.0;
            let close_y = status_y + STATUS_HEIGHT / 2.0;
            let mut close_path = tiny_skia::PathBuilder::new();
            close_path.move_to(close_x - 4.0, close_y - 4.0);
            close_path.line_to(close_x + 4.0, close_y + 4.0);
            close_path.move_to(close_x + 4.0, close_y - 4.0);
            close_path.line_to(close_x - 4.0, close_y + 4.0);
            if let Some(close_path) = close_path.finish() {
                let stroke = tiny_skia::Stroke {
                    width: 1.4,
                    ..Default::default()
                };
                pixmap.stroke_path(
                    &close_path,
                    &dim_paint,
                    &stroke,
                    Transform::identity(),
                    None,
                );
            }
        }
        let status_x = w.get() as f32 - status_layout.width() - 12.0;
        let tabs_end = tab_width * tab_layouts.len() as f32;
        if status_x > tabs_end + 12.0 {
            for line in status_layout.lines() {
                for entry in line.items() {
                    if let PositionedLayoutItem::GlyphRun(run) = entry {
                        draw_run_background(pixmap, &run, status_x, status_y + 7.0);
                        draw_glyph_run(pixmap, scale_cx, glyphs, &run, status_x, status_y + 7.0);
                    }
                }
            }
        }

        if let Some(layout) = search_overlay {
            let overlay_width = (layout.width() + 24.0).min(w.get() as f32 - MARGIN * 2.0);
            if let Some(rect) = Rect::from_xywh(MARGIN, 80.0, overlay_width, 28.0) {
                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            }
            for line in layout.lines() {
                for entry in line.items() {
                    if let PositionedLayoutItem::GlyphRun(run) = entry {
                        draw_run_background(pixmap, &run, MARGIN + 10.0, 85.0);
                        draw_glyph_run(pixmap, scale_cx, glyphs, &run, MARGIN + 10.0, 85.0);
                    }
                }
            }
        }

        if let (Some(rows), Some(layouts)) = (
            navigation_panel_rows.as_ref(),
            navigation_panel_layouts.as_ref(),
        ) {
            let panel_x = PANEL_X;
            let panel_y = PANEL_Y;
            let panel_width = PANEL_WIDTH.min((w.get() as f32 - MARGIN * 2.0).max(120.0));
            let row_height = PANEL_ROW_HEIGHT;
            let panel_height = row_height * rows.len() as f32 + 8.0;
            if let Some(rect) = Rect::from_xywh(panel_x, panel_y, panel_width, panel_height) {
                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            }
            for (row, ((selected, _), layout)) in rows.iter().zip(layouts).enumerate() {
                let y = panel_y + 4.0 + row as f32 * row_height;
                if *selected {
                    let ac = palette.accent;
                    let mut selected_paint = Paint::default();
                    selected_paint.set_color(Color::from_rgba8(ac.0, ac.1, ac.2, 44));
                    if let Some(rect) =
                        Rect::from_xywh(panel_x + 4.0, y, panel_width - 8.0, row_height)
                    {
                        pixmap.fill_rect(rect, &selected_paint, Transform::identity(), None);
                    }
                }
                for line in layout.lines() {
                    for entry in line.items() {
                        if let PositionedLayoutItem::GlyphRun(run) = entry {
                            draw_run_background(pixmap, &run, panel_x + 12.0, y + 7.0);
                            draw_glyph_run(pixmap, scale_cx, glyphs, &run, panel_x + 12.0, y + 7.0);
                        }
                    }
                }
            }
        }

        if let (Some((x, y)), Some(layouts)) = (menu, menu_layouts) {
            if let Some(rect) = Rect::from_xywh(
                x,
                y,
                CONTEXT_MENU_WIDTH,
                CONTEXT_MENU_ROW_HEIGHT * context_actions(context_mode).len() as f32,
            ) {
                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            }
            for (row, (action, layout)) in context_actions(context_mode)
                .iter()
                .copied()
                .zip(layouts.iter())
                .enumerate()
            {
                if menu_pointer.and_then(|pointer| context_action_at((x, y), pointer, context_mode))
                    == Some(action)
                {
                    let ac = palette.accent;
                    let mut hover = Paint::default();
                    hover.set_color(Color::from_rgba8(ac.0, ac.1, ac.2, 38));
                    if let Some(rect) = Rect::from_xywh(
                        x,
                        y + row as f32 * CONTEXT_MENU_ROW_HEIGHT,
                        CONTEXT_MENU_WIDTH,
                        CONTEXT_MENU_ROW_HEIGHT,
                    ) {
                        pixmap.fill_rect(rect, &hover, Transform::identity(), None);
                    }
                }
                for line in layout.lines() {
                    for entry in line.items() {
                        if let PositionedLayoutItem::GlyphRun(run) = entry {
                            draw_run_background(
                                pixmap,
                                &run,
                                x + CONTEXT_MENU_PADDING,
                                y + row as f32 * CONTEXT_MENU_ROW_HEIGHT + 9.0,
                            );
                            draw_glyph_run(
                                pixmap,
                                scale_cx,
                                glyphs,
                                &run,
                                x + CONTEXT_MENU_PADDING,
                                y + row as f32 * CONTEXT_MENU_ROW_HEIGHT + 9.0,
                            );
                        }
                    }
                }
            }
        }

        // Volcado del pixmap a la ventana.
        let surface = surface
            .as_mut()
            .ok_or_else(|| "la superficie grafica no esta disponible".to_string())?;
        surface
            .resize(w, h)
            .map_err(|error| format!("no se pudo redimensionar la superficie: {error}"))?;
        let mut buffer = surface
            .buffer_mut()
            .map_err(|error| format!("no se pudo obtener el buffer grafico: {error}"))?;
        for (dst, src) in buffer.iter_mut().zip(pixmap.pixels()) {
            *dst = ((src.red() as u32) << 16) | ((src.green() as u32) << 8) | src.blue() as u32;
        }
        buffer
            .present()
            .map_err(|error| format!("no se pudo presentar el cuadro: {error}"))?;

        let ms = frame_start.elapsed().as_secs_f64() * 1000.0;
        if self.first_paint_done {
            self.frames += 1;
            self.frame_time_total += ms;
        } else {
            self.first_paint_done = true;
            self.log.push(format!(
                "[medicion] primer pintado: {:.0} ms desde el arranque",
                self.started.elapsed().as_secs_f64() * 1000.0
            ));
        }
        Ok(())
    }

    fn fail_and_exit(&mut self, event_loop: &ActiveEventLoop, error: String) {
        self.fatal_error = true;
        self.log.push(format!("[error] {error}"));
        self.report();
        event_loop.exit();
    }

    /// Se llama una sola vez, al salir: recien ahi se toca stderr.
    fn report(&self) {
        for line in &self.log {
            eprintln!("{line}");
        }
        if self.frames > 0 {
            let avg = self.frame_time_total / self.frames as f64;
            eprintln!(
                "[medicion] {} cuadros de scroll, promedio {:.1} ms ({:.0} fps equivalentes)",
                self.frames,
                avg,
                1000.0 / avg
            );
        }
    }
}

/// Devuelve únicamente el byte central de un marcador CommonMark reconocido.
/// Así el clic no puede alterar el texto de la tarea, sus espacios ni otras
/// ocurrencias de corchetes en el mismo ítem.
fn task_marker_replacement(text: &str) -> Option<(usize, &'static str)> {
    text.find("[ ]")
        .map(|offset| (offset + 1, "x"))
        .or_else(|| text.find("[x]").map(|offset| (offset + 1, " ")))
        .or_else(|| text.find("[X]").map(|offset| (offset + 1, " ")))
}

fn code_copy_bounds(slot: &Slot, text_width: f32, scroll: f32, scale: f32) -> (f32, f32, f32, f32) {
    let width = CODE_COPY_WIDTH * scale;
    let height = CODE_COPY_HEIGHT * scale;
    (
        slot.x + text_width - width - 4.0 * scale,
        slot.y - scroll + 4.0 * scale,
        width,
        height,
    )
}

fn matching_block_indices(blocks: &[Block], query: &str) -> Vec<usize> {
    if query.is_empty() {
        return Vec::new();
    }
    // La búsqueda es una interacción explícita y acotada al documento ya
    // cargado. Normalizar una vez la consulta evita que una diferencia de
    // mayúsculas vuelva inaccesible contenido Unicode al usuario.
    let folded_query = query.to_lowercase();
    blocks
        .iter()
        .enumerate()
        .filter_map(|(index, block)| {
            block
                .text
                .to_lowercase()
                .contains(&folded_query)
                .then_some(index)
        })
        .collect()
}

fn document_outline(blocks: &[Block]) -> Vec<(usize, String)> {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(index, block)| match block.kind {
            Kind::Heading(level) => Some((
                index,
                format!(
                    "{}{}",
                    "  ".repeat(level.saturating_sub(1) as usize),
                    block.text
                ),
            )),
            _ => None,
        })
        .collect()
}

fn main() {
    let started = Instant::now();

    let args: Vec<String> = std::env::args().skip(1).collect();
    // `--bench` recorre el documento y sale. `--bench=0` pinta un solo cuadro
    // y sale: es la forma de cronometrar el arranque real, sin los cuadros de
    // scroll sumados al tiempo total del proceso.
    let bench = args.iter().find_map(|a| {
        let rest = a.strip_prefix("--bench")?;
        Some(
            rest.strip_prefix('=')
                .and_then(|n| n.parse().ok())
                .unwrap_or(240),
        )
    });
    let exact_measure = args.iter().any(|a| a == "--exacto");
    let opening_path = args.iter().find(|a| !a.starts_with("--")).cloned();
    let initial_document_request = u64::from(opening_path.is_some());

    let t = Instant::now();
    let event_loop = match EventLoop::<AppEvent>::with_user_event().build() {
        Ok(event_loop) => event_loop,
        Err(error) => {
            eprintln!("no se pudo iniciar la interfaz: {error}");
            std::process::exit(1);
        }
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    let proxy = event_loop.create_proxy();
    let mut log = vec![format!(
        "[medicion]   EventLoop::new: {:.0} ms",
        t.elapsed().as_secs_f64() * 1000.0
    )];

    let t = Instant::now();
    let mut font_cx = FontContext::new();
    log.push(format!(
        "[medicion]   FontContext::new (fuentes del sistema): {:.0} ms",
        t.elapsed().as_secs_f64() * 1000.0
    ));

    let t = Instant::now();
    register_embedded_fonts(&mut font_cx);
    log.push(format!(
        "[medicion]   registrar 4 archivos de 3 familias embebidas: {:.0} ms",
        t.elapsed().as_secs_f64() * 1000.0
    ));

    let settings = Settings::load();
    let recovery = settings
        .recovery_enabled
        .then(RecoverySession::start)
        .and_then(Result::ok);
    let recovery_privacy_notice_pending =
        recovery.is_some() && RecoverySession::privacy_notice_needed().unwrap_or(false);
    let initial_document_id = NEXT_DOCUMENT_ID.fetch_add(1, Ordering::Relaxed);
    let mut app = App {
        started,
        document: DocumentState {
            id: initial_document_id,
            path: opening_path
                .clone()
                .unwrap_or_else(|| "sin título.md".to_string()),
            source: TextBuffer::new(),
            source_metadata: TextMetadata::default(),
            source_identity: None,
            source_baseline_bytes: None,
            source_editor: SourceEditor::new(),
            mode: if opening_path.is_some() {
                DocumentMode::Reading
            } else {
                DocumentMode::SourceEditing
            },
            rendered_blocks: Vec::new(),
            blocks: Vec::new(),
            safe_mode: None,
            scroll: 0.0,
            pending_heading: None,
            last_recovery: Instant::now(),
        },
        inactive_documents: Vec::new(),
        tab_order: vec![initial_document_id],
        external_checks_in_flight: HashSet::new(),
        saves_in_flight: HashSet::new(),
        recovery_enabled: settings.recovery_enabled,
        recovery,
        recovery_privacy_notice_pending,
        workspace: None,
        workspace_request: 0,
        workspace_cancel: None,
        workspace_check_in_flight: None,
        workspace_stale: false,
        document_request: initial_document_request,
        open_requests: if opening_path.is_some() {
            HashMap::from([(initial_document_id, initial_document_request)])
        } else {
            HashMap::new()
        },
        view_requests: HashMap::new(),
        proxy: proxy.clone(),
        window: None,
        surface: None,
        font_cx,
        layout_cx: LayoutContext::new(),
        scale_cx: ScaleContext::new(),
        glyphs: GlyphCache::new(),
        pixmap: None,
        slots: Vec::new(),
        live: HashMap::new(),
        doc_height: 0.0,
        laid_for_width: -1.0,
        preview_slots: Vec::new(),
        preview_live: HashMap::new(),
        preview_height: 0.0,
        preview_laid_for_width: -1.0,
        scale_factor: 1.0,
        scroll: 0.0,
        first_paint_done: false,
        frames: 0,
        frame_time_total: 0.0,
        bench,
        exact_measure,
        exact_after_edit: false,
        log,
        palette: NIGHT,
        pointer: None,
        selecting: false,
        selection: None,
        modifiers: ModifiersState::empty(),
        text_cursor_hover: false,
        hover_destination: None,
        focused_link: None,
        focus_destination: None,
        context_menu: None,
        search_query: None,
        search_match: 0,
        workspace_search_query: None,
        workspace_search_match: 0,
        workspace_paths: None,
        workspace_path_match: 0,
        collapsed_workspace_dirs: HashSet::new(),
        backlink_paths: None,
        backlink_match: 0,
        outline_headings: None,
        outline_match: 0,
        command_palette: None,
        command_palette_query: String::new(),
        toolbar_focus: None,
        clipboard: None,
        notice: Some(if opening_path.is_some() {
            "cargando documento".to_string()
        } else {
            "documento nuevo · Ctrl+Shift+S para elegir destino".to_string()
        }),
        fatal_error: false,
    };

    if let Some(worker_path) = opening_path {
        thread::spawn(move || {
            let started = Instant::now();
            let event = match open_explicit_primary(&worker_path, DEFAULT_DOCUMENT_LIMIT_BYTES) {
                Ok(opened) => match if is_markdown_path(std::path::Path::new(&worker_path)) {
                    parse_blocks(&opened.source)
                } else {
                    let source_index = SourceIndex::new(&opened.source);
                    safe_source_blocks(&opened.source, &source_index).map(|blocks| ParseOutcome {
                        blocks,
                        degradation: Some(Degradation::TextOnly),
                    })
                } {
                    Ok(outcome) => AppEvent::DocumentReady {
                        document_id: initial_document_id,
                        request: initial_document_request,
                        path: PathBuf::from(&worker_path),
                        source: opened.source,
                        metadata: opened.metadata,
                        identity: opened.identity,
                        baseline_bytes: opened.baseline_bytes,
                        outcome,
                        elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
                    },
                    Err(error) => AppEvent::DocumentFailed {
                        document_id: initial_document_id,
                        request: initial_document_request,
                        error: format!("el documento no se pudo preparar de forma segura: {error}"),
                    },
                },
                Err(error) => AppEvent::DocumentFailed {
                    document_id: initial_document_id,
                    request: initial_document_request,
                    error: format!("no se pudo leer {worker_path}: {error}"),
                },
            };
            let _ = proxy.send_event(event);
        });
    }

    if let Err(error) = event_loop.run_app(&mut app) {
        eprintln!("la interfaz termino con un error: {error}");
        std::process::exit(1);
    }
    if app.fatal_error {
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------- pruebas

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn la_barra_de_pestanas_resuelve_cada_destino_sin_salir_de_su_area() {
        assert_eq!(tab_index_at(10.0, 580.0, 600.0, 600.0, 3), Some(0));
        assert_eq!(tab_index_at(210.0, 580.0, 600.0, 600.0, 3), Some(1));
        assert_eq!(tab_index_at(410.0, 580.0, 600.0, 600.0, 3), Some(2));
        assert_eq!(tab_index_at(10.0, 500.0, 600.0, 600.0, 3), None);
        assert_eq!(tab_index_at(590.0, 580.0, 600.0, 600.0, 2), None);
        assert_eq!(tab_close_index_at(190.0, 580.0, 600.0, 600.0, 3), Some(0));
        assert_eq!(tab_close_index_at(170.0, 580.0, 600.0, 600.0, 3), None);
    }

    #[test]
    fn recorrer_pestanas_respeta_el_orden_y_envuelve_los_extremos() {
        let order = [11, 22, 33];

        assert_eq!(adjacent_tab_id(&order, 22, false), Some(33));
        assert_eq!(adjacent_tab_id(&order, 33, false), Some(11));
        assert_eq!(adjacent_tab_id(&order, 11, true), Some(33));
        assert_eq!(adjacent_tab_id(&order, 99, false), None);
    }

    #[test]
    fn la_barra_superior_expone_solo_acciones_visibles() {
        assert_eq!(
            toolbar_action_at(50.0, 12.0, 900.0),
            Some(AppAction::NewDocument)
        );
        assert_eq!(
            toolbar_action_at(140.0, 12.0, 900.0),
            Some(AppAction::OpenDocument)
        );
        assert_eq!(
            toolbar_action_at(480.0, 12.0, 900.0),
            Some(AppAction::CommandPalette)
        );
        assert_eq!(toolbar_action_at(50.0, 50.0, 900.0), None);
        assert_eq!(toolbar_action_at(900.0, 12.0, 900.0), None);
        assert_eq!(adjacent_toolbar_index(0, true), TOOLBAR_ACTIONS.len() - 1);
        assert_eq!(adjacent_toolbar_index(TOOLBAR_ACTIONS.len() - 1, false), 0);
    }

    #[test]
    fn el_catalogo_de_acciones_es_pequeno_y_no_duplica_etiquetas() {
        let mut labels = APP_ACTIONS
            .iter()
            .map(|action| action.label())
            .collect::<Vec<_>>();
        let original_len = labels.len();
        labels.sort_unstable();
        labels.dedup();

        assert_eq!(original_len, 16);
        assert_eq!(labels.len(), original_len);
    }

    #[test]
    fn la_vista_dividida_reserva_la_mitad_sin_cambiar_otros_modos() {
        assert_eq!(content_layout_width(1200.0, DocumentMode::Split), 600.0);
        assert_eq!(content_layout_width(1200.0, DocumentMode::Reading), 1200.0);
        assert_eq!(
            content_layout_width(1200.0, DocumentMode::SourceEditing),
            1200.0
        );
        assert!(DocumentMode::Split.is_editable());
        assert!(!DocumentMode::Reading.is_editable());
    }

    #[test]
    fn la_paleta_filtra_acciones_sin_inventar_comandos() {
        assert_eq!(filtered_actions("").len(), APP_ACTIONS.len());
        assert_eq!(filtered_actions("backlinks"), vec![AppAction::Backlinks]);
        assert!(filtered_actions("GUARDAR").contains(&AppAction::Save));
        assert!(filtered_actions("GUARDAR").contains(&AppAction::SaveAs));
        assert_eq!(
            filtered_actions("actualizar índice"),
            vec![AppAction::RefreshWorkspace]
        );
        assert!(filtered_actions("acción inexistente").is_empty());
    }

    #[test]
    fn un_panel_largo_mantiene_visible_la_fila_elegida() {
        assert_eq!(panel_window(30, 0, 9), 0..9);
        assert_eq!(panel_window(30, 15, 9), 11..20);
        assert_eq!(panel_window(30, 29, 9), 21..30);
        assert_eq!(panel_window(3, 2, 9), 0..3);
        assert_eq!(panel_window(0, 0, 9), 0..0);
        assert_eq!(
            panel_item_at(60.0, PANEL_Y + PANEL_ROW_HEIGHT, 30, 15),
            Some(11)
        );
        assert_eq!(
            panel_item_at(60.0, PANEL_Y + PANEL_ROW_HEIGHT * 2.0, 30, 15),
            Some(12)
        );
        assert_eq!(
            panel_item_at(20.0, PANEL_Y + PANEL_ROW_HEIGHT, 30, 15),
            None
        );
        assert_eq!(panel_item_at(60.0, PANEL_Y, 30, 15), None);
    }

    #[test]
    fn el_arbol_del_workspace_muestra_jerarquia_sin_inventar_destinos() {
        let paths = vec![
            PathBuf::from("universidad/redes/uno.md"),
            PathBuf::from("universidad/seguridad.md"),
            PathBuf::from("raiz.md"),
        ];

        let rows = workspace_tree_rows(&paths);

        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0].note_path(), Some(std::path::Path::new("raiz.md")));
        assert_eq!(rows[1].label(), "- universidad/");
        assert!(rows[1].note_path().is_none());
        assert_eq!(rows[2].label(), "  - redes/");
        assert_eq!(rows[3].label(), "    uno.md");
        assert_eq!(
            rows[3].note_path(),
            Some(std::path::Path::new("universidad/redes/uno.md"))
        );
        assert_eq!(rows[4].label(), "  seguridad.md");

        let collapsed = HashSet::from([PathBuf::from("universidad/redes")]);
        let visible = visible_workspace_tree_rows(&paths, &collapsed);
        assert_eq!(visible.len(), 4);
        assert_eq!(visible[2].label(), "  + redes/");
        assert!(
            visible
                .iter()
                .all(|row| row.relative_path != std::path::Path::new("universidad/redes/uno.md"))
        );
    }

    #[test]
    fn la_etiqueta_de_pestana_conserva_unicode_y_marca_cambios() {
        let label = tab_label(r"C:\notas\日本語 y ciberseguridad.md", true, 18);

        assert!(label.ends_with('*'));
        assert!(label.chars().count() <= 18);
        assert!(label.is_char_boundary(label.len()));
    }

    #[test]
    fn el_cierre_global_cuenta_cambios_en_todas_las_pestanas() {
        let active = DocumentState::untitled();
        let mut dirty = DocumentState::untitled();
        dirty.source_editor.mark_recovered();
        let clean = DocumentState::untitled();

        let inactive = [
            InactiveDocument {
                document: dirty,
                recovery: None,
            },
            InactiveDocument {
                document: clean,
                recovery: None,
            },
        ];

        assert_eq!(dirty_document_count(&active, &inactive), 1);
    }

    #[test]
    fn un_resultado_asincrono_se_dirige_por_identidad_de_documento() {
        let mut active = DocumentState::untitled();
        active.path = "activa.md".to_string();
        let active_id = active.id;
        let mut inactive_document = DocumentState::untitled();
        inactive_document.path = "inactiva.md".to_string();
        let inactive_id = inactive_document.id;
        let mut inactive = [InactiveDocument {
            document: inactive_document,
            recovery: None,
        }];

        let selected = document_by_id_mut(&mut active, &mut inactive, inactive_id).unwrap();
        selected.path = "guardada.md".to_string();

        assert_eq!(active.id, active_id);
        assert_eq!(active.path, "activa.md");
        assert_eq!(inactive[0].document.path, "guardada.md");
        assert!(document_by_id_mut(&mut active, &mut inactive, u64::MAX).is_none());
    }

    #[test]
    fn el_resultado_renderizado_no_reemplaza_la_fuente_visible_en_split() {
        let source = "# Fuente\n";
        let mut document = DocumentState::untitled();
        document.source = TextBuffer::from_text(source);
        document.mode = DocumentMode::Split;
        document.blocks = safe_buffer_blocks(&document.source).unwrap();
        let source_text = document.blocks[0].text.clone();

        apply_view_outcome(&mut document, parse_blocks(source).unwrap());

        assert_eq!(document.blocks[0].text, source_text);
        assert_eq!(document.rendered_blocks[0].text, "Fuente");
        assert_eq!(document.mode, DocumentMode::Split);
    }

    #[test]
    fn el_resultado_renderizado_si_actualiza_el_modo_lectura() {
        let mut document = DocumentState::untitled();
        document.mode = DocumentMode::Reading;

        apply_view_outcome(&mut document, parse_blocks("**lectura**").unwrap());

        assert_eq!(document.blocks[0].text, "lectura");
        assert_eq!(document.rendered_blocks[0].text, "lectura");
    }

    #[test]
    fn una_recuperacion_se_prepara_como_documento_nuevo_y_modificado() {
        let recovered = recovered_document("# recuperado\n\ntexto".to_string()).unwrap();

        assert_eq!(recovered.path, "recuperación sin guardar.md");
        assert!(recovered.is_dirty());
        assert_eq!(recovered.mode, DocumentMode::SourceEditing);
        assert!(!recovered.blocks.is_empty());
        assert!(recovered.source_identity.is_none());
    }

    #[test]
    fn dos_rutas_equivalentes_no_crean_pestanas_duplicadas() {
        let root = std::env::temp_dir().join(format!(
            "visor-md-tabs-{}-{}",
            std::process::id(),
            NEXT_DOCUMENT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).unwrap();
        let path = root.join("nota.md");
        fs::write(&path, "# nota").unwrap();
        let alternate = root.join("sub").join("..").join("nota.md");
        fs::create_dir_all(root.join("sub")).unwrap();

        assert!(paths_refer_to_same_file(&path, &alternate));
        assert!(paths_refer_to_same_file(
            std::path::Path::new(r"\\servidor\notas\nota.md"),
            std::path::Path::new(r"\\servidor\notas\.\nota.md")
        ));
        let _ = fs::remove_dir_all(root);
    }

    fn aplanar(md: &str) -> Vec<Block> {
        parse_blocks(md)
            .expect("el documento de prueba debe ser valido")
            .blocks
    }

    /// El parser no puede caerse con entrada hostil. Es la propiedad, no la
    /// ausencia de crash por casualidad: estos son los casos que `security.md`
    /// nombra como bombas de recursos.
    #[test]
    fn entrada_patologica_no_entra_en_panico() {
        let casos = vec![
            String::new(),
            "\0\0\0".to_string(),
            "#".repeat(10_000),
            "> ".repeat(5_000) + "hola",
            "- ".repeat(5_000) + "item",
            "*".repeat(20_000),
            "|a|b|\n|-|-|\n".to_string() + &"|x|y|\n".repeat(5_000),
            "```\n".to_string() + &"x\n".repeat(10_000) + "```",
            "[a]: b\n".repeat(5_000),
            "\u{202E}\u{200B}texto invisible".to_string(),
            "😀🏳️‍🌈".repeat(1_000),
        ];
        for caso in casos {
            let _ = aplanar(&caso);
        }
    }

    /// Barrido reproducible de combinaciones raras. No sustituye una campaña
    /// de fuzzing: fija una muestra amplia y barata en cada ejecución normal.
    #[test]
    fn barrido_adversarial_determinista_no_entra_en_panico() {
        const FRAGMENTOS: &[&str] = &[
            "# título\n",
            "> cita ",
            "- [x] tarea\n",
            "**énfasis",
            "[enlace](https://example.com/a?b=c)",
            "<script>alert(1)</script>",
            "<mark atributo=\"inert\">",
            "</mark>",
            "`código`",
            "|a|b|\n|-|-|\n",
            "😀\u{202e}\u{200b}",
            "[",
            "](",
            "```rust\n",
            "\n",
        ];
        for seed in 0..128_u64 {
            let mut state = seed.wrapping_add(0x9e37_79b9);
            let mut input = String::new();
            for _ in 0..96 {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let fragment = FRAGMENTOS[(state as usize) % FRAGMENTOS.len()];
                input.push_str(fragment);
            }
            let parsed =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| parse_blocks(&input)));
            assert!(parsed.is_ok(), "el seed {seed} produjo un panic");
        }
    }

    #[test]
    fn un_documento_normal_no_entra_en_modo_seguro() {
        let outcome = parse_blocks("# Titulo\n\nTexto con **formato**.")
            .expect("el documento normal debe poder prepararse");
        assert_eq!(outcome.degradation, None);
        assert!(outcome.blocks.iter().any(|block| block.spans.len() == 1));
    }

    #[test]
    fn el_modo_seguro_conserva_la_fuente_completa() {
        let md = "> ".repeat(5_000) + "linea final sin salto";
        let outcome = parse_blocks(&md).expect("la vista segura debe ser representable");
        assert_eq!(outcome.degradation, Some(Degradation::DepthLimit));

        let reconstructed = outcome
            .blocks
            .iter()
            .map(|block| &md[block.source.start..block.source.end])
            .collect::<String>();
        assert_eq!(reconstructed, md, "el fallback perdio parte de la fuente");
        assert!(
            outcome
                .blocks
                .iter()
                .all(|block| matches!(block.kind, Kind::Code)),
            "la vista segura intento interpretar la fuente"
        );
    }

    #[test]
    fn una_linea_extensa_pasa_a_fuente_inerte_sin_romper_utf8() {
        let source = "á".repeat(MAX_SAFE_LINE_BYTES / 2 + 1);
        let outcome = parse_blocks(&source).expect("la línea extensa debe degradar");
        assert_eq!(outcome.degradation, Some(Degradation::LineLimit));
        assert!(outcome.blocks.len() > 1);
        assert!(
            outcome
                .blocks
                .iter()
                .all(|block| block.text.len() <= MAX_SAFE_LINE_BYTES)
        );
        assert!(
            outcome
                .blocks
                .iter()
                .all(|block| block.text.is_char_boundary(block.text.len()))
        );
        let reconstructed: String = outcome
            .blocks
            .iter()
            .map(|block| &source[block.source.start..block.source.end])
            .collect();
        assert_eq!(reconstructed, source);
    }

    #[test]
    fn la_busqueda_visible_no_recorre_ni_incluye_bloques_lejanos() {
        let slots: Vec<_> = (0..10)
            .map(|i| Slot {
                y: i as f32 * 100.0,
                height: 50.0,
                x: 0.0,
                kind: Kind::Para,
            })
            .collect();

        assert_eq!(visible_range(&slots, 175.0, 325.0), 2..4);
        assert_eq!(visible_range(&slots, 0.0, 50.0), 0..1);
        assert_eq!(visible_range(&slots, 951.0, 1_100.0), 10..10);
    }

    #[test]
    fn el_scroll_respeta_el_alto_real_de_la_ventana() {
        assert_eq!(max_scroll(1_000.0, 760.0), 240.0);
        assert_eq!(max_scroll(500.0, 760.0), 0.0);
    }

    #[test]
    fn un_ancho_nuevo_invalida_el_layout_anterior() {
        assert!(!layout_width_is_stale(900.0, 900.3));
        assert!(layout_width_is_stale(900.0, 900.6));
        assert!(layout_width_is_stale(-1.0, 900.0));
    }

    #[test]
    fn el_menu_contextual_solo_habilita_pegado_en_el_editor() {
        let menu = (100.0, 200.0);
        assert_eq!(
            context_action_at(menu, (110.0, 210.0), DocumentMode::Reading),
            Some(ContextAction::CopyText)
        );
        assert_eq!(
            context_action_at(menu, (110.0, 250.0), DocumentMode::Reading),
            Some(ContextAction::CopyMarkdown)
        );
        assert_eq!(
            context_action_at(menu, (110.0, 210.0), DocumentMode::SourceEditing),
            Some(ContextAction::Paste)
        );
        assert_eq!(
            context_action_at(menu, (50.0, 210.0), DocumentMode::SourceEditing),
            None
        );
        assert!(!ContextAction::CopyText.source_markdown());
        assert!(ContextAction::CopyMarkdown.source_markdown());
    }

    #[test]
    fn el_titulo_hace_visible_el_modo_seguro() {
        let normal = window_title("nota.md", None, false, None);
        let seguro = window_title("hostil.md", Some(Degradation::DepthLimit), false, None);
        assert!(!normal.contains("modo seguro"));
        assert!(seguro.contains("modo seguro"));
        assert!(seguro.contains("hostil.md"));
    }

    #[test]
    fn el_titulo_marca_cambios_sin_guardar() {
        let limpio = window_title("nota.md", None, false, None);
        let sucio = window_title("nota.md", None, true, None);
        assert!(!limpio.contains("nota.md *"));
        assert!(sucio.contains("nota.md *"));
    }

    #[test]
    fn el_aviso_de_modo_seguro_explica_la_degradacion_sin_ocultar_la_fuente() {
        let label = safe_mode_label(Degradation::LineLimit);
        assert!(label.contains("Modo seguro"));
        assert!(label.contains("longitud de línea"));
        assert!(label.contains("fuente inerte"));
    }

    #[test]
    fn el_texto_inerte_no_se_presenta_como_error_de_seguridad() {
        let title = window_title("datos.json", Some(Degradation::TextOnly), false, None);
        assert!(title.contains("texto inerte"));
        assert!(!title.contains("modo seguro"));
    }

    #[test]
    fn un_fallo_de_apertura_deja_un_mensaje_renderizable_sin_detalles() {
        let (source, blocks) = opening_failure_blocks();
        let source_text = source.to_string();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].text, source_text);
        assert!(blocks[0].source.is_valid_for_buffer(&source));
        assert!(blocks[0].targets.is_empty());
        assert!(!source_text.contains("C:\\"));
        assert!(!source_text.contains("permission"));
    }

    #[test]
    fn los_destinos_de_enlace_se_clasifican_sin_acceder_a_ellos() {
        assert_eq!(
            classify_link_destination("https://example.test"),
            LinkDestinationKind::Web
        );
        assert_eq!(
            classify_link_destination("mailto:alguien@example.test"),
            LinkDestinationKind::Mail
        );
        assert_eq!(
            classify_link_destination("notas/tema.md"),
            LinkDestinationKind::RelativeFile
        );
        for path in [
            "file:///C:/secreto",
            "\\\\servidor\\share",
            "C:/secreto",
            "../secreto.md",
        ] {
            assert_eq!(
                classify_link_destination(path),
                LinkDestinationKind::Blocked
            );
        }
    }

    #[test]
    fn los_enlaces_externos_e_internos_tienen_senales_distintas() {
        assert_eq!(link_color(DAY, "https://example.test"), DAY.external_link);
        assert_eq!(
            link_color(DAY, "mailto:alguien@example.test"),
            DAY.external_link
        );
        assert_eq!(link_color(DAY, "notas/tema.md"), DAY.accent);
        assert_eq!(link_color(DAY, "../secreto.md"), DAY.dim);
    }

    #[test]
    fn solo_web_y_correo_pueden_delegarse_al_sistema() {
        assert_eq!(
            external_destination("https://example.test"),
            Some("https://example.test")
        );
        assert_eq!(
            external_destination("  https://example.test  "),
            Some("https://example.test")
        );
        assert_eq!(
            external_destination("mailto:alguien@example.test"),
            Some("mailto:alguien@example.test")
        );
        assert_eq!(external_destination("notas/tema.md"), None);
        assert_eq!(external_destination("file:///C:/secreto"), None);
        assert_eq!(external_destination("\\\\servidor\\share"), None);
    }

    #[test]
    fn tab_recorrre_solo_los_enlaces_en_orden_del_documento() {
        let outcome = parse_blocks(
            "[primero](https://example.test)\n\n![imagen](img.png)\n\n[segundo](notas/dos.md)",
        )
        .unwrap();
        let links = link_targets_in_document_order(&outcome.blocks);
        assert_eq!(links, vec![(0, 0), (2, 0)]);
        assert_eq!(next_link_target(&links, None, false), Some((0, 0)));
        assert_eq!(next_link_target(&links, Some((0, 0)), false), Some((2, 0)));
        assert_eq!(next_link_target(&links, Some((0, 0)), true), Some((2, 0)));
        assert_eq!(next_link_target(&[], None, false), None);
    }

    #[test]
    fn las_casillas_producen_pixeles_y_estados_distintos() {
        let mut pendiente = Pixmap::new(32, 32).unwrap();
        pendiente.fill(Color::from_rgba8(NIGHT.bg.0, NIGHT.bg.1, NIGHT.bg.2, 255));
        let fondo = pendiente.data().to_vec();
        draw_checkbox(&mut pendiente, 8.0, 8.0, 16.0, false, NIGHT);
        assert_ne!(pendiente.data(), fondo, "la casilla pendiente no se dibujo");

        let mut terminada = Pixmap::new(32, 32).unwrap();
        terminada.fill(Color::from_rgba8(NIGHT.bg.0, NIGHT.bg.1, NIGHT.bg.2, 255));
        draw_checkbox(&mut terminada, 8.0, 8.0, 16.0, true, NIGHT);
        assert_ne!(terminada.data(), fondo, "la casilla terminada no se dibujo");
        assert_ne!(
            terminada.data(),
            pendiente.data(),
            "los dos estados de tarea producen la misma imagen"
        );
    }

    #[test]
    fn la_cache_de_glifos_separa_las_variaciones_de_fuente() {
        let regular = glyph_key(1, 0, 16.0, 42, &[0, 0]).unwrap();
        let bold = glyph_key(1, 0, 16.0, 42, &[0, 14_000]).unwrap();
        assert_ne!(regular, bold, "peso normal y negrita compartieron máscara");
        assert!(glyph_key(1, 0, 16.0, 42, &[0; 17]).is_none());
    }

    #[test]
    fn un_glifo_rgba_de_fallback_se_compone_en_el_lienzo() {
        let glyph = CachedGlyph {
            left: 0,
            top: 1,
            width: 1,
            height: 1,
            content: Content::Color,
            data: vec![255, 180, 0, 255],
        };
        let mut pixmap = Pixmap::new(4, 4).unwrap();
        pixmap.fill(Color::from_rgba8(NIGHT.bg.0, NIGHT.bg.1, NIGHT.bg.2, 255));
        let before = pixmap.data().to_vec();
        blit_color(&mut pixmap, &glyph, 1.0, 2.0, 4, 4);
        assert_ne!(pixmap.data(), before, "el bitmap RGBA no produjo píxeles");
    }

    /// Un bloque siempre ocupa algo. Un alto de cero haria que se superpongan
    /// y que la barra de scroll mienta hacia el otro lado.
    #[test]
    fn el_alto_estimado_siempre_es_positivo() {
        for md in ["# t", "texto", "- a", "```\nx\n```", "|a|\n|-|\n|b|"] {
            for block in aplanar(md) {
                for ancho in [200.0, 900.0, 4000.0] {
                    let h = estimate_height(&block, ancho, 1.0);
                    assert!(h > 0.0 && h.is_finite(), "alto invalido {h} en {md:?}");
                }
            }
        }
    }

    /// El ADR-16 acepta que la estimacion tenga error, pero acotado: si se
    /// desviara por un factor grande, la barra de scroll dejaria de servir.
    /// Se mide contra el alto real que da parley.
    #[test]
    fn el_alto_estimado_se_parece_al_real() {
        let md = "\
# Un titulo de prueba

Un parrafo comun y corriente, con la longitud que suele tener un parrafo real
en un documento de verdad, para que el ajuste de linea entre en juego y la
estimacion tenga algo que estimar.

- Un item de lista
- Otro item bastante mas largo que el anterior, para variar el largo

```
un bloque de codigo
con dos lineas
```
";
        let blocks = aplanar(md);
        assert!(!blocks.is_empty(), "el documento de prueba quedo vacio");

        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        let ancho = 900.0;

        let real: f32 = blocks
            .iter()
            .map(|b| build_layout(b, &mut font_cx, &mut layout_cx, ancho, 1.0, NIGHT).height())
            .sum();
        let estimado: f32 = blocks.iter().map(|b| estimate_height(b, ancho, 1.0)).sum();

        let error = (estimado - real).abs() / real;
        assert!(
            error < 0.35,
            "la estimacion se desvio {:.1}% (real {real:.0} px, estimado {estimado:.0} px)",
            error * 100.0
        );
    }

    #[test]
    fn la_escala_dpi_aumenta_la_tipografia_sin_cambiar_el_ancho_logico() {
        let block = &aplanar("Una línea de prueba para escala DPI.")[0];
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        let normal = build_layout(block, &mut font_cx, &mut layout_cx, 900.0, 1.0, NIGHT);
        let dpi_alto = build_layout(block, &mut font_cx, &mut layout_cx, 1800.0, 2.0, NIGHT);
        assert!(dpi_alto.height() > normal.height() * 1.9);
        assert_eq!(normal.lines().count(), dpi_alto.lines().count());
    }

    #[test]
    fn la_seleccion_tiene_geometria_dibujable() {
        let block = &aplanar("Selecciona estas palabras con el mouse.")[0];
        let start = block.text.find("estas").expect("texto de prueba");
        let end = start + "estas palabras".len();
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        let layout = build_layout(block, &mut font_cx, &mut layout_cx, 900.0, 1.0, NIGHT);
        let selection = Selection::new(
            Cursor::from_byte_index(&layout, start, Affinity::Downstream),
            Cursor::from_byte_index(&layout, end, Affinity::Downstream),
        );
        let geometry = selection.geometry(&layout);
        assert!(!geometry.is_empty(), "la seleccion no produjo rectangulos");
        assert!(geometry.iter().all(|(rect, _)| {
            rect.width().is_finite()
                && rect.height().is_finite()
                && rect.width() > 0.0
                && rect.height() > 0.0
        }));

        let caret =
            Cursor::from_byte_index(&layout, start, Affinity::Downstream).geometry(&layout, 1.25);
        assert!(caret.width().is_finite() && caret.height().is_finite());
        assert!(caret.width() > 0.0 && caret.height() > 0.0);
    }

    #[test]
    fn la_seleccion_entre_bloques_abarca_los_intermedios() {
        let forward = DocumentSelection {
            anchor: BlockCursor {
                block: 1,
                offset: 3,
            },
            focus: BlockCursor {
                block: 3,
                offset: 2,
            },
        };
        let backward = DocumentSelection {
            anchor: forward.focus,
            focus: forward.anchor,
        };
        for selection in [forward, backward] {
            assert_eq!(selection.range_for(0, 8), None);
            assert_eq!(selection.range_for(1, 10), Some((3, 10)));
            assert_eq!(selection.range_for(2, 6), Some((0, 6)));
            assert_eq!(selection.range_for(3, 7), Some((0, 2)));
            assert_eq!(selection.range_for(4, 8), None);
        }

        let collapsed = DocumentSelection::collapsed(BlockCursor {
            block: 2,
            offset: 4,
        });
        assert_eq!(collapsed.range_for(2, 9), Some((4, 4)));
    }

    #[test]
    fn el_autoscroll_de_seleccion_respeta_los_bordes() {
        assert_eq!(selection_scroll_delta(400.0, 800.0), 0.0);
        assert!(selection_scroll_delta(0.0, 800.0) < 0.0);
        assert!(selection_scroll_delta(799.0, 800.0) > 0.0);
        assert_eq!(selection_scroll_delta(10.0, 60.0), 0.0);
    }

    #[test]
    fn seleccionar_todo_abarca_desde_el_primer_hasta_el_ultimo_bloque() {
        let blocks = aplanar("primer bloque\n\nsegundo bloque");
        let selection = DocumentSelection {
            anchor: BlockCursor {
                block: 0,
                offset: 0,
            },
            focus: BlockCursor {
                block: blocks.len() - 1,
                offset: blocks.last().expect("hay bloques").text.len(),
            },
        };
        assert_eq!(selection.range_for(0, blocks[0].text.len()), Some((0, 13)));
        assert_eq!(selection.range_for(1, blocks[1].text.len()), Some((0, 14)));
    }

    #[test]
    fn copiar_vista_conserva_el_texto_de_una_seleccion_entre_bloques() {
        let blocks = aplanar("primero\n\nsegundo\n\ntercero");
        let selection = DocumentSelection {
            anchor: BlockCursor {
                block: 0,
                offset: 3,
            },
            focus: BlockCursor {
                block: 2,
                offset: 4,
            },
        };
        assert_eq!(
            selection.rendered_text(&blocks).as_deref(),
            Some("mero\nsegundo\nterc")
        );
    }

    #[test]
    fn copiar_vista_completa_conserva_la_estructura_legible() {
        let source = "- primero\n3. tercero\n- [x] terminado\n> una cita\n";
        let blocks = aplanar(source);
        let selection = DocumentSelection {
            anchor: BlockCursor {
                block: 0,
                offset: 0,
            },
            focus: BlockCursor {
                block: blocks.len() - 1,
                offset: blocks.last().unwrap().text.len(),
            },
        };
        assert_eq!(
            selection.rendered_text(&blocks).as_deref(),
            Some("• primero\n3. tercero\n[x] terminado\n> una cita")
        );
    }

    #[test]
    fn copiar_markdown_toma_bloques_fuente_completos() {
        let source = "# titulo\n\n**primero**\n\nsegundo\n";
        let blocks = aplanar(source);
        let selection = DocumentSelection {
            anchor: BlockCursor {
                block: 1,
                offset: 3,
            },
            focus: BlockCursor {
                block: 2,
                offset: 2,
            },
        };
        assert_eq!(
            selection.source_blocks(source, &blocks).as_deref(),
            Some("**primero**\n\nsegundo")
        );
    }

    #[test]
    fn una_seleccion_vacia_no_sobrescribe_el_portapapeles() {
        let blocks = aplanar("texto");
        let selection = DocumentSelection::collapsed(BlockCursor {
            block: 0,
            offset: 2,
        });
        assert_eq!(selection.rendered_text(&blocks), None);
        assert_eq!(
            selection.source_blocks("texto", &blocks).as_deref(),
            Some("texto")
        );
    }

    #[test]
    fn la_seleccion_vertical_usa_lineas_del_layout() {
        let block = &aplanar("uno dos tres cuatro cinco seis siete ocho nueve")[0];
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        let layout = build_layout(block, &mut font_cx, &mut layout_cx, 180.0, 1.0, NIGHT);
        assert!(layout.lines().count() > 1, "la fixture debe envolver");
        let cursor = Cursor::from_byte_index(&layout, 2, Affinity::Downstream);
        let next = Selection::new(cursor, cursor).next_line(&layout, false);
        assert_ne!(next.focus().index(), cursor.index());
    }

    /// Los encabezados y las filas de tabla tienen que sobrevivir el
    /// aplanado con su tipo puesto: es lo que decide como se dibujan.
    #[test]
    fn los_tipos_de_bloque_sobreviven_al_aplanado() {
        let blocks = aplanar("# uno\n\n## dos\n\ntexto\n\n- item\n\n|a|b|\n|-|-|\n|c|d|\n");
        let tipos: Vec<String> = blocks.iter().map(|b| format!("{:?}", b.kind)).collect();
        assert!(tipos.iter().any(|t| t.contains("Heading(1)")), "{tipos:?}");
        assert!(tipos.iter().any(|t| t.contains("Heading(2)")), "{tipos:?}");
        assert!(tipos.iter().any(|t| t.contains("Para")), "{tipos:?}");
        assert!(tipos.iter().any(|t| t.contains("Item")), "{tipos:?}");
        assert!(tipos.iter().any(|t| t.contains("TableRow")), "{tipos:?}");
    }

    /// Un bloque de codigo se parte en una linea por bloque para poder
    /// dibujarle el fondo por linea. Si eso cambia, el fondo se rompe.
    #[test]
    fn el_bloque_de_codigo_se_parte_por_lineas() {
        let blocks = aplanar("```\nuno\ndos\ntres\n```");
        let codigo: Vec<_> = blocks
            .iter()
            .filter(|b| matches!(b.kind, Kind::Code))
            .collect();
        assert_eq!(codigo.len(), 3, "se esperaban 3 lineas de codigo");
        assert_eq!(codigo[0].text, "uno");
        assert_eq!(codigo[2].text, "tres");
    }

    #[test]
    fn el_bloque_de_codigo_vacio_no_desaparece() {
        let blocks = aplanar("```\n```");
        assert_eq!(blocks.len(), 1);
        assert!(matches!(blocks[0].kind, Kind::Code));
        assert!(blocks[0].text.is_empty());
    }

    /// Corpus de integración para la semántica que el lector ya declara. No
    /// convierte cada ejemplo de CommonMark en una promesa de render completo:
    /// exige que lo soportado llegue como modelo válido hasta layout.
    #[test]
    fn corpus_commonmark_gfm_del_lector_llega_al_modelo() {
        let source = include_str!("../tests/fixtures/commonmark-gfm-reader.md");
        let outcome = parse_blocks(source).expect("el corpus debe parsearse");
        assert_eq!(outcome.degradation, None, "el corpus normal no se degrada");
        validate_model(source, &outcome.blocks).expect("el corpus conserva rangos validos");

        assert!(
            outcome
                .blocks
                .iter()
                .any(|block| matches!(block.kind, Kind::Heading(1)))
        );
        assert!(
            outcome
                .blocks
                .iter()
                .any(|block| matches!(block.kind, Kind::Heading(2)))
        );
        assert!(
            outcome
                .blocks
                .iter()
                .any(|block| matches!(block.kind, Kind::Quote) && block.quote_depth >= 2)
        );
        assert!(
            outcome
                .blocks
                .iter()
                .any(|block| matches!(block.kind, Kind::Rule))
        );
        assert!(outcome.blocks.iter().any(|block| {
            matches!(block.kind, Kind::Code) && block.code_info.as_deref() == Some("rust")
        }));
        assert!(outcome.blocks.iter().any(|block| {
            matches!(block.kind, Kind::TableRow { header: true })
                && block.table_alignments == vec![CellAlignment::Left, CellAlignment::Right]
        }));

        let markers: Vec<_> = outcome
            .blocks
            .iter()
            .filter_map(|block| block.marker.as_ref())
            .collect();
        assert!(
            markers
                .iter()
                .any(|marker| matches!(marker, Marker::Text(text) if text == "3."))
        );
        assert!(
            markers
                .iter()
                .any(|marker| matches!(marker, Marker::Task { done: false }))
        );
        assert!(
            markers
                .iter()
                .any(|marker| matches!(marker, Marker::Task { done: true }))
        );

        let targets: Vec<_> = outcome
            .blocks
            .iter()
            .flat_map(|block| &block.targets)
            .collect();
        assert!(targets.iter().any(|target| {
            target.kind == InlineTargetKind::Link
                && target.destination == "https://example.com/ruta"
        }));
        assert!(targets.iter().any(|target| {
            target.kind == InlineTargetKind::Image
                && target.destination == "https://example.com/diagram.png"
        }));
        assert!(targets.iter().any(|target| {
            target.kind == InlineTargetKind::Link
                && target.destination.starts_with("https://example.com?find=")
        }));

        let visible = outcome
            .blocks
            .iter()
            .map(|block| block.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(visible.contains("concepto clave"));
        assert!(visible.contains("<script"));
        assert!(visible.contains("referencia a pie[1]"));
        assert!(visible.contains("La definición no crea HTML"));

        // El contrato del lector termina en el layout, no en el AST ni en el
        // modelo intermedio. Cada bloque anunciado debe producir geometría
        // finita con las fuentes reales que usará la aplicación.
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        for block in &outcome.blocks {
            let layout = build_layout(block, &mut font_cx, &mut layout_cx, 680.0, 1.0, NIGHT);
            assert!(layout.height().is_finite() && layout.height() >= 0.0);
        }
    }

    #[test]
    fn las_notas_al_pie_se_muestran_sin_html_ni_perder_la_definicion() {
        let source = "Texto con nota[^uno].\n\n[^uno]: Definición con áéí.";
        let outcome = parse_blocks(source).expect("las notas al pie deben parsearse");
        validate_model(source, &outcome.blocks).expect("los rangos de nota son válidos");
        let rendered = outcome
            .blocks
            .iter()
            .map(|block| block.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("Texto con nota[1]."));
        assert!(rendered.contains("Definición con áéí."));
    }

    #[test]
    fn el_boton_de_copiar_codigo_queda_dentro_del_bloque_visible() {
        let slot = Slot {
            y: 120.0,
            height: 48.0,
            x: 48.0,
            kind: Kind::Code,
        };
        let (x, y, width, height) = code_copy_bounds(&slot, 640.0, 40.0, 1.0);

        assert!(x >= slot.x);
        assert!(y >= slot.y - 40.0);
        assert!(x + width <= slot.x + 640.0);
        assert!(y + height <= slot.y - 40.0 + slot.height);
    }

    #[test]
    fn la_busqueda_local_devuelve_solo_bloques_que_contienen_la_consulta() {
        let blocks = parse_blocks("uno\n\ndos con Clave\n\ntres con ñANDÚ")
            .expect("la fixture debe parsearse")
            .blocks;

        assert_eq!(matching_block_indices(&blocks, "clave"), vec![1]);
        assert_eq!(matching_block_indices(&blocks, "Ñandú"), vec![2]);
        assert!(matching_block_indices(&blocks, "ausente").is_empty());
        assert!(matching_block_indices(&blocks, "").is_empty());
    }

    #[test]
    fn una_vista_asincrona_solo_es_actual_para_su_documento_y_revision() {
        assert!(is_current_view_result(7, 7, 12, 12));
        assert!(!is_current_view_result(6, 7, 12, 12));
        assert!(!is_current_view_result(7, 7, 11, 12));
    }

    #[test]
    fn los_controles_efimeros_acotan_texto_sin_partir_unicode() {
        let mut query = "á".to_string();
        append_limited_text(&mut query, "éí", 2);
        assert_eq!(query, "áé");
        assert_eq!(abbreviated_label("áéí", 2), "áé…");
        assert_eq!(abbreviated_label("áé", 2), "áé");
    }

    #[test]
    fn el_indice_del_documento_conserva_orden_y_profundidad() {
        let blocks = parse_blocks("# Uno\n\nTexto\n\n### Tres")
            .expect("la fixture debe parsearse")
            .blocks;
        assert_eq!(
            document_outline(&blocks),
            vec![(0, "Uno".to_string()), (2, "    Tres".to_string())]
        );
    }

    /// Casos pequeños y trazables que complementan la fixture de lectura. No
    /// prometen conformidad CommonMark total: fijan construcciones que la UI
    /// ya anuncia y que deben conservar rangos válidos hasta el layout.
    #[test]
    fn casos_commonmark_gfm_anunciados_llegan_a_layout() {
        let cases = [
            ("salto_forzado", "primera línea\\\nsegunda línea"),
            ("autolink_escapado", "<https://example.com?find=\\*>"),
            ("enfasis_anidado", "**fuerte y _énfasis_**"),
            ("lista_anidada", "1. uno\n   1. dos\n      - tres"),
            ("tarea", "- [ ] pendiente\n- [x] hecha"),
            ("tabla", "| a | b |\n| :- | -: |\n| uno | dos |"),
            ("codigo", "```text\n<fuente inerte>\n```"),
            (
                "html_semantico",
                "<kbd>Ctrl</kbd> <mark>marca</mark> H<sub>2</sub>O",
            ),
            ("html_inerte", "<script>alert(1)</script>"),
        ];
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();

        for (name, source) in cases {
            let outcome = parse_blocks(source).unwrap_or_else(|error| {
                panic!("{name}: el parser rechazó una construcción anunciada: {error}")
            });
            assert_eq!(outcome.degradation, None, "{name}: no debe degradarse");
            validate_model(source, &outcome.blocks)
                .unwrap_or_else(|error| panic!("{name}: modelo inválido: {error}"));
            assert!(!outcome.blocks.is_empty(), "{name}: no produjo bloques");
            for block in &outcome.blocks {
                let layout = build_layout(block, &mut font_cx, &mut layout_cx, 360.0, 1.0, NIGHT);
                assert!(
                    layout.height().is_finite() && layout.height() >= 0.0,
                    "{name}: layout inválido"
                );
            }
        }
    }

    #[test]
    fn unicode_general_llega_al_layout_con_fallback() {
        let source = "العربية हिन्दी 日本語 한국어 🔒";
        let outcome = parse_blocks(source).expect("Unicode debe parsearse");
        assert_eq!(outcome.blocks[0].text, source);
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        let layout = build_layout(
            &outcome.blocks[0],
            &mut font_cx,
            &mut layout_cx,
            480.0,
            1.0,
            NIGHT,
        );
        let glyph_runs = layout
            .lines()
            .flat_map(|line| line.items())
            .filter(|entry| matches!(entry, PositionedLayoutItem::GlyphRun(_)))
            .count();
        assert!(
            glyph_runs > 0,
            "Unicode no produjo ninguna corrida de glifos"
        );
    }
}

// -------------------------------------------- pruebas del formato inline

#[cfg(test)]
mod pruebas_inline {
    use super::*;

    fn aplanar(md: &str) -> Vec<Block> {
        parse_blocks(md)
            .expect("el documento de prueba debe ser valido")
            .blocks
    }

    /// Devuelve el enfasis que cubre el primer caracter de `aguja`.
    fn enfasis_de(md: &str, aguja: &str) -> Emphasis {
        let blocks = aplanar(md);
        for b in &blocks {
            if let Some(pos) = b.text.find(aguja) {
                for s in &b.spans {
                    if s.start <= pos && pos < s.end {
                        return s.style;
                    }
                }
                return Emphasis::default();
            }
        }
        panic!(
            "no se encontro {aguja:?} en {:?}",
            blocks.iter().map(|b| &b.text).collect::<Vec<_>>()
        );
    }

    /// Las marcas de Markdown no se dibujan: se convierten en estilo. Si el
    /// texto plano todavia contiene los asteriscos, es que no se parsearon.
    #[test]
    fn las_marcas_no_quedan_en_el_texto() {
        let blocks = aplanar("Un **negrita** y un _cursiva_ y un `codigo`.");
        let texto = &blocks[0].text;
        assert!(!texto.contains('*'), "quedaron asteriscos: {texto:?}");
        assert!(!texto.contains('_'), "quedaron guiones bajos: {texto:?}");
        assert!(!texto.contains('`'), "quedaron backticks: {texto:?}");
        assert!(texto.contains("negrita") && texto.contains("cursiva"));
    }

    #[test]
    fn el_salto_forzado_no_se_aplana_como_espacio() {
        let hard = aplanar("uno  \ndos");
        let soft = aplanar("uno\ndos");
        assert_eq!(hard[0].text, "uno\ndos");
        assert_eq!(soft[0].text, "uno dos");
    }

    #[test]
    fn escapes_y_entidades_llegan_como_texto_visible() {
        let blocks = aplanar(r"\*literal\* &amp; &#x1F600;");
        assert_eq!(blocks[0].text, "*literal* & 😀");
        assert!(blocks[0].spans.is_empty());
    }

    #[test]
    fn encabezado_setext_conserva_nivel_y_origen() {
        let md = "Titulo\n======";
        let blocks = aplanar(md);
        assert_eq!(blocks.len(), 1);
        assert!(matches!(blocks[0].kind, Kind::Heading(1)));
        assert_eq!(blocks[0].text, "Titulo");
        assert_eq!(&md[blocks[0].source.start..blocks[0].source.end], md);
    }

    #[test]
    fn cada_enfasis_llega_a_su_tramo() {
        assert!(enfasis_de("un **fuerte** aca", "fuerte").strong);
        assert!(enfasis_de("un _suave_ aca", "suave").emph);
        assert!(enfasis_de("un `mono` aca", "mono").code);
        assert!(enfasis_de("un ~~tachado~~ aca", "tachado").strike);
        assert!(enfasis_de("un ==importante== aca", "importante").mark);
        assert!(enfasis_de("un [enlace](http://x) aca", "enlace").link);
    }

    #[test]
    fn autolinks_conservan_texto_y_destino() {
        let blocks = aplanar("Visita https://example.com/ruta y <correo@example.com>.");
        let block = &blocks[0];
        assert_eq!(block.targets.len(), 2);
        assert_eq!(block.targets[0].kind, InlineTargetKind::Link);
        assert_eq!(block.targets[0].destination, "https://example.com/ruta");
        assert_eq!(block.targets[1].destination, "mailto:correo@example.com");
        for target in &block.targets {
            assert!(block.text.is_char_boundary(target.start));
            assert!(block.text.is_char_boundary(target.end));
        }
    }

    #[test]
    fn wikilinks_de_obsidian_son_enlaces_nativos_sin_interpretar_codigo() {
        let blocks = aplanar(
            "Ver [[clases/redes#Modelo|la guía]], [[seguridad]] y [[#Inicio]].\n\n`[[literal]]`",
        );
        assert_eq!(blocks[0].text, "Ver la guía, seguridad y #Inicio.");
        let targets: Vec<_> = blocks[0]
            .targets
            .iter()
            .filter(|target| target.kind == InlineTargetKind::WikiLink)
            .collect();
        assert_eq!(targets.len(), 3);
        assert_eq!(targets[0].destination, "clases/redes#Modelo");
        assert_eq!(targets[1].destination, "seguridad");
        assert_eq!(targets[2].destination, "#Inicio");
        assert!(blocks[0].spans.iter().any(|span| span.style.link));
        assert_eq!(blocks[1].text, "[[literal]]");
        assert!(blocks[1].targets.is_empty());
    }

    #[test]
    fn wikilinks_defectuosos_se_conservan_como_texto() {
        let block = &aplanar("[[ ]] y [[sin cierre")[0];
        assert_eq!(block.text, "[[ ]] y [[sin cierre");
        assert!(block.targets.is_empty());
    }

    #[test]
    fn subrayado_y_tachado_llegan_al_layout() {
        let blocks = aplanar("un [enlace](https://example.com) y ~~tachado~~");
        let block = &blocks[0];
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        let layout = build_layout(block, &mut font_cx, &mut layout_cx, 900.0, 1.0, NIGHT);

        let mut enlace_subrayado = false;
        let mut texto_tachado = false;
        for line in layout.lines() {
            for item in line.items() {
                if let PositionedLayoutItem::GlyphRun(run) = item {
                    enlace_subrayado |= run.style().underline.is_some();
                    texto_tachado |= run.style().strikethrough.is_some();
                }
            }
        }

        assert!(enlace_subrayado, "el enlace perdio el subrayado en layout");
        assert!(texto_tachado, "el tachado no llego al layout");
    }

    /// El caso que justifica acumular el estilo al bajar por el arbol en vez
    /// de pisarlo: un enfasis dentro de otro tiene que llegar con los dos.
    #[test]
    fn el_enfasis_anidado_se_acumula() {
        let e = enfasis_de("**fuerte con _suave_ adentro**", "suave");
        assert!(e.strong && e.emph, "se perdio un enfasis al anidar: {e:?}");
    }

    /// El texto sin formato no debe generar tramos: es el caso comun y
    /// llenar un `Vec` por cada linea normal seria peso al pedo.
    #[test]
    fn el_texto_plano_no_genera_tramos() {
        let blocks = aplanar("Una linea comun y corriente, sin nada de formato.");
        assert!(
            blocks[0].spans.is_empty(),
            "tramos de mas: {:?}",
            blocks[0].spans
        );
    }

    /// Los rangos son offsets de bytes y se los pasamos a parley tal cual.
    /// Si cayeran fuera del texto o en mitad de un caracter multibyte,
    /// parley entra en panico. Con acentos y emoji es facil equivocarse.
    #[test]
    fn los_rangos_caen_en_limites_de_caracter() {
        for md in [
            "**ñandú** y **camión**",
            "un `código` acentuado",
            "**😀 emoji** y _más_",
            "**a_b_c** mezclado",
        ] {
            for b in aplanar(md) {
                for s in &b.spans {
                    assert!(s.end <= b.text.len(), "rango fuera de texto en {md:?}");
                    assert!(s.start < s.end, "rango vacio o invertido en {md:?}");
                    assert!(
                        b.text.is_char_boundary(s.start) && b.text.is_char_boundary(s.end),
                        "rango parte un caracter en {md:?}: {s:?} sobre {:?}",
                        b.text
                    );
                }
            }
        }
    }

    /// Las listas ordenadas numeran desde donde diga el documento, no
    /// siempre desde 1, y las viñetas no llevan numero.
    #[test]
    fn los_marcadores_de_lista_se_numeran_bien() {
        let ordenada = aplanar("3. tres\n4. cuatro\n5. cinco");
        let marcas: Vec<_> = ordenada.iter().filter_map(|b| b.marker.clone()).collect();
        assert_eq!(
            marcas,
            vec![
                Marker::Text("3.".into()),
                Marker::Text("4.".into()),
                Marker::Text("5.".into())
            ],
            "numeracion mal"
        );

        let vinetas = aplanar("- uno\n- dos");
        let marcas: Vec<_> = vinetas.iter().filter_map(|b| b.marker.clone()).collect();
        assert_eq!(
            marcas,
            vec![Marker::Text("•".into()), Marker::Text("•".into())]
        );
    }

    #[test]
    fn cada_lista_incrementa_una_sola_vez_la_sangria() {
        let blocks = aplanar("- primero\n  - segundo");
        let depths: Vec<_> = blocks
            .iter()
            .filter_map(|block| match block.kind {
                Kind::Item(depth) => Some(depth),
                _ => None,
            })
            .collect();
        assert_eq!(depths, vec![1, 2]);
    }

    #[test]
    fn las_tareas_marcan_su_casilla() {
        let blocks = aplanar("- [ ] pendiente\n- [x] hecha");
        let marcas: Vec<_> = blocks.iter().filter_map(|b| b.marker.clone()).collect();
        assert_eq!(
            marcas,
            vec![Marker::Task { done: false }, Marker::Task { done: true }]
        );
    }

    /// Una cita con varios parrafos no se aplasta en uno solo, y todos
    /// quedan marcados como cita para que les toque el filete de acento.
    #[test]
    fn la_cita_conserva_su_estructura() {
        let blocks = aplanar("> primero\n>\n> segundo");
        let citas: Vec<_> = blocks
            .iter()
            .filter(|b| matches!(b.kind, Kind::Quote))
            .collect();
        assert_eq!(citas.len(), 2, "la cita se aplasto: {blocks:?}",);
    }

    #[test]
    fn callout_conocido_se_renderiza_sin_reescribir_su_fuente() {
        let source = "> [!WARNING] Revisa la red\n>\n> Texto de apoyo.";
        let blocks = aplanar(source);
        assert!(
            blocks
                .iter()
                .all(|block| matches!(block.kind, Kind::Callout))
        );
        assert_eq!(blocks[0].text, "Revisa la red");
        assert!(matches!(
            blocks[0].marker,
            Some(Marker::Text(ref label)) if label == "Atención"
        ));
        assert!(source.contains("[!WARNING]"));
    }

    #[test]
    fn callout_desconocido_permanece_como_cita_visible() {
        let blocks = aplanar("> [!CUSTOM] Conserva el marcador");
        assert!(matches!(blocks[0].kind, Kind::Quote));
        assert_eq!(blocks[0].text, "[!CUSTOM] Conserva el marcador");
    }

    #[test]
    fn la_linea_horizontal_produce_un_bloque() {
        let blocks = aplanar("antes\n\n---\n\ndespues");
        assert!(
            blocks.iter().any(|b| matches!(b.kind, Kind::Rule)),
            "no se genero la linea horizontal"
        );
    }

    /// Las imagenes son del Sprint 2, pero no pueden desaparecer en silencio:
    /// se anuncian con su texto alternativo.
    /// Regresion concreta: al pasar el aplanado a recorrer las citas en vez
    /// de aplastarlas, 5000 citas anidadas desbordaron la pila y mataron el
    /// proceso. El tope de MAX_NEST existe por esto. La prueba fija el
    /// comportamiento: se corta, no se cae.
    #[test]
    fn el_anidamiento_profundo_se_corta_en_vez_de_desbordar() {
        for md in [
            "> ".repeat(5_000) + "hola",
            "- ".repeat(5_000) + "item",
            "*".repeat(4_000) + "x" + &"*".repeat(4_000),
            "> - > - ".repeat(1_000) + "mezclado",
        ] {
            let outcome = parse_blocks(&md).expect("debe degradar sin caerse");
            assert_eq!(
                outcome.degradation,
                Some(Degradation::DepthLimit),
                "la entrada profunda no activo el modo seguro"
            );
            assert!(outcome.blocks.len() < MAX_BLOCKS, "explosion de bloques");
        }
    }

    /// La sangria no puede crecer sin limite: a cierta profundidad se comeria
    /// el ancho util y dejaria el texto en una columna de un caracter.
    #[test]
    fn la_sangria_tiene_tope() {
        let profundo = Kind::Item(200).indent();
        let tope = Kind::Item(MAX_INDENT_DEPTH).indent();
        assert_eq!(profundo, tope, "la sangria no se topo");
        assert!(
            tope < MAX_MEASURE / 2.0,
            "el tope de sangria es demasiado grande"
        );
    }

    #[test]
    fn la_imagen_se_anuncia_en_vez_de_desaparecer() {
        let blocks = aplanar("mira ![un gato](gato.png) aca");
        assert!(
            blocks[0].text.contains("un gato"),
            "se perdio la imagen: {:?}",
            blocks[0].text
        );
    }

    #[test]
    fn el_html_desconocido_permanece_visible_e_inerte() {
        let inline = aplanar("antes <script src=\"https://evil.test/x.js\"> despues");
        assert!(inline[0].text.contains("<script"));
        assert!(inline[0].targets.is_empty(), "HTML creo un destino activo");

        let block = aplanar("<iframe src=\"file:///secreto\">\ncontenido\n</iframe>");
        let visible = block
            .iter()
            .map(|item| item.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(visible.contains("<iframe"));
        assert!(visible.contains("file:///secreto"));
        assert!(block.iter().all(|item| item.targets.is_empty()));
    }

    /// Regresión portada del corpus de seguridad de v1. En la arquitectura
    /// nativa no hay DOM que pueda ejecutar estos nodos, pero además deben
    /// seguir visibles para que un payload no se esconda del lector.
    #[test]
    fn html_hostil_historico_sigue_visible_y_sin_destinos() {
        let md = r#"<script>window.__XSS = true</script>
<img src=x onerror="alert(1)">
<form action="https://atacante.example"><input autofocus></form>
<style>body { display: none }</style>
<svg onload="alert(2)"><foreignObject>texto</foreignObject></svg>
<div style="position:fixed;inset:0">barra falsa</div>"#;
        let blocks = aplanar(md);
        let visible = blocks
            .iter()
            .map(|block| block.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        for evidence in [
            "<script>",
            "onerror",
            "<form",
            "<style>",
            "<svg",
            "barra falsa",
        ] {
            assert!(visible.contains(evidence), "se oculto {evidence:?}");
        }
        assert!(blocks.iter().all(|block| block.targets.is_empty()));
    }

    /// Las conversiones PDF a Markdown suelen dejar tablas irregulares y
    /// vallas sin cerrar. El documento puede degradar visualmente, pero no
    /// puede perder su final ni abortar el aplanado.
    #[test]
    fn markdown_defectuoso_historico_llega_hasta_el_final() {
        let md = r#"# Documento convertido

| cabecera | con | columnas |
| --- | --- |
| fila corta |

![](imagen_que_no_existe.png)

```python
def ejemplo():
    return "falta la valla de cierre"

Pagina 14 de 14"#;
        let blocks = aplanar(md);
        let visible = blocks
            .iter()
            .map(|block| block.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(visible.contains("Documento convertido"));
        assert!(visible.contains("Pagina 14 de 14"));
        validate_model(md, &blocks).expect("el documento defectuoso conserva un modelo valido");
    }

    #[test]
    fn br_es_nativo_pero_sus_atributos_no_se_interpretan() {
        for tag in ["<br>", "<BR/>", "<br />"] {
            let blocks = aplanar(&format!("antes{tag}despues"));
            assert_eq!(blocks[0].text, "antes\ndespues");
        }

        let blocks = aplanar("antes<br onclick=\"alert(1)\">despues");
        assert!(blocks[0].text.contains("onclick"));
        assert!(!blocks[0].text.contains('\n'));
        assert!(blocks[0].targets.is_empty());
    }

    #[test]
    fn la_allowlist_html_se_vuelve_estilo_nativo_sin_atributos() {
        let md = "Usa <kbd>Ctrl</kbd>, <mark>importante</mark>, H<sub>2</sub>O y x<sup>2</sup>.";
        let block = &aplanar(md)[0];
        for tag in [
            "<kbd>", "</kbd>", "<mark>", "</mark>", "<sub>", "</sub>", "<sup>", "</sup>",
        ] {
            assert!(
                !block.text.contains(tag),
                "la etiqueta permitida quedo visible: {tag}"
            );
        }
        assert!(enfasis_de(md, "Ctrl").kbd);
        assert!(enfasis_de(md, "importante").mark);
        let sub_pos = block.text.find("H2O").expect("formula visible") + 1;
        let sup_pos = block.text.find("x2").expect("potencia visible") + 1;
        assert!(
            block
                .spans
                .iter()
                .any(|span| span.start <= sub_pos && sub_pos < span.end && span.style.sub)
        );
        assert!(
            block
                .spans
                .iter()
                .any(|span| span.start <= sup_pos && sup_pos < span.end && span.style.sup)
        );

        let con_atributos = aplanar("<mark onclick=alert(1)>no seguro</mark>");
        assert!(con_atributos[0].text.contains("onclick"));
        assert!(con_atributos[0].targets.is_empty());
    }

    #[test]
    fn html_permitido_sin_cierre_no_oculta_su_marca() {
        let block = &aplanar("antes <mark>despues")[0];
        assert_eq!(block.text, "antes <mark>despues");
        assert!(!enfasis_de("antes <mark>despues", "despues").mark);
    }

    #[test]
    fn semantica_html_llega_al_layout_y_al_dibujo() {
        let block = &aplanar("<kbd>Ctrl</kbd> <mark>clave</mark> H<sub>2</sub>O x<sup>2</sup>")[0];
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        let layout = build_layout(block, &mut font_cx, &mut layout_cx, 900.0, 1.0, NIGHT);
        let mut kbd = false;
        let mut mark = false;
        let mut sub = false;
        let mut sup = false;
        for line in layout.lines() {
            for item in line.items() {
                if let PositionedLayoutItem::GlyphRun(run) = item {
                    let brush = run.style().brush;
                    kbd |= brush.background == Some(NIGHT.kbd);
                    mark |= brush.background == Some(NIGHT.mark);
                    sub |= brush.baseline_shift > 0;
                    sup |= brush.baseline_shift < 0;
                }
            }
        }
        assert!(
            kbd && mark && sub && sup,
            "una semantica HTML no llego al layout"
        );
    }

    #[test]
    fn los_rangos_de_fuente_son_validos_y_apuntan_al_original() {
        let md = "# Titulo\n\nUn **ñandú** y un [enlace](destino.md).";
        let blocks = aplanar(md);
        validate_model(md, &blocks).expect("el modelo debe tener rangos validos");

        let span = blocks
            .iter()
            .flat_map(|block| &block.spans)
            .find(|span| md[span.source.start..span.source.end].contains("ñandú"))
            .expect("el texto enfatizado conserva su origen");
        assert_eq!(&md[span.source.start..span.source.end], "ñandú");

        let heading = blocks
            .iter()
            .find(|block| matches!(block.kind, Kind::Heading(1)))
            .unwrap();
        assert_eq!(&md[heading.source.start..heading.source.end], "# Titulo");
    }

    #[test]
    fn los_enlaces_e_imagenes_conservan_su_destino() {
        let blocks =
            aplanar("Un [documento](docs/uno.md \"nota\") y ![captura](img/a.png \"imagen\").");
        let targets: Vec<_> = blocks.iter().flat_map(|block| &block.targets).collect();
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[0].kind, InlineTargetKind::Link);
        assert_eq!(targets[0].destination, "docs/uno.md");
        assert_eq!(targets[0].title, "nota");
        assert_eq!(targets[1].kind, InlineTargetKind::Image);
        assert_eq!(targets[1].destination, "img/a.png");
        assert_eq!(targets[1].title, "imagen");
    }

    #[test]
    fn el_codigo_conserva_el_lenguaje() {
        let blocks = aplanar("```rust\nfn main() {}\n```");
        let code = blocks
            .iter()
            .find(|block| matches!(block.kind, Kind::Code))
            .unwrap();
        assert_eq!(code.code_info.as_deref(), Some("rust"));
    }

    #[test]
    fn las_tablas_conservan_sus_celdas() {
        let blocks = aplanar("| nombre | valor |\n| --- | --- |\n| uno | **dos** |");
        let row = blocks
            .iter()
            .find(|block| {
                matches!(block.kind, Kind::TableRow { header: false })
                    && block
                        .table_cells
                        .first()
                        .is_some_and(|cell| cell.text == "uno")
            })
            .expect("fila de datos");
        assert_eq!(row.table_cells.len(), 2);
        assert_eq!(row.table_cells[1].text, "dos");
        assert!(row.spans.iter().any(|span| span.style.strong));
    }

    #[test]
    fn las_celdas_guardan_estilos_con_rangos_locales() {
        let blocks = aplanar("| nombre | valor |\n| --- | --- |\n| uno | **dos** |");
        let row = blocks
            .iter()
            .find(|block| matches!(block.kind, Kind::TableRow { header: false }))
            .expect("fila de datos");
        let cell = &row.table_cells[1];
        assert_eq!(cell.text, "dos");
        assert!(
            cell.spans
                .iter()
                .any(|span| span.style.strong && span.end <= cell.text.len())
        );
    }

    #[test]
    fn una_fila_de_tabla_se_maqueta_por_celdas_sin_separadores() {
        let blocks = aplanar("| izquierda | centro |\n| :--- | :---: |\n| texto | **valor** |");
        let row = blocks
            .iter()
            .find(|block| matches!(block.kind, Kind::TableRow { header: false }))
            .expect("fila de datos");
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        let cells = build_table_layouts(row, &mut font_cx, &mut layout_cx, 720.0, 1.0, NIGHT);
        assert_eq!(cells.len(), 2);
        assert!(cells.iter().all(|layout| layout.width() > 0.0));
        assert!(
            cells.iter().all(|layout| layout.width() < 360.0),
            "una celda no debe ocupar toda la fila"
        );
    }

    #[test]
    fn las_tablas_conservan_alineacion_por_columna() {
        let blocks =
            aplanar("| izquierda | centro | derecha |\n| :--- | :---: | ---: |\n| a | b | c |");
        let row = blocks
            .iter()
            .find(|block| matches!(block.kind, Kind::TableRow { header: false }))
            .unwrap();
        assert_eq!(
            row.table_alignments,
            vec![
                CellAlignment::Left,
                CellAlignment::Center,
                CellAlignment::Right
            ]
        );
    }

    #[test]
    fn el_cambio_de_tarea_toca_solo_el_marcador() {
        assert_eq!(task_marker_replacement("- [ ] pendiente"), Some((3, "x")));
        assert_eq!(task_marker_replacement("- [x] terminada"), Some((3, " ")));
        assert_eq!(task_marker_replacement("- [X] terminada"), Some((3, " ")));
        assert_eq!(task_marker_replacement("- [nota] sin tarea"), None);
    }

    #[test]
    fn las_citas_anidadas_conservan_profundidad() {
        let blocks = aplanar("> exterior\n>\n> > interior");
        let depths: Vec<_> = blocks.iter().map(|block| block.quote_depth).collect();
        assert!(depths.contains(&1), "falta cita exterior: {depths:?}");
        assert!(depths.contains(&2), "falta cita interior: {depths:?}");
    }

    #[test]
    fn las_citas_anidadas_tambien_aumentan_la_sangria_visual() {
        let blocks = aplanar("> exterior\n>\n> > interior");
        let outer = blocks
            .iter()
            .find(|block| block.quote_depth == 1)
            .expect("cita exterior");
        let inner = blocks
            .iter()
            .find(|block| block.quote_depth == 2)
            .expect("cita interior");
        assert!(inner.indent() > outer.indent());
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.kind, self.text)
    }
}
