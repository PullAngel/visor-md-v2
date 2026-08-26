// Visor MD v2
//
// Nucleo nativo en recuperacion. Abre un .md, lo parsea con comrak, lo maqueta
// con parley y lo dibuja con tiny-skia sobre una ventana winit + softbuffer.
// Todavía no tiene chrome, copia ni edición.
//
// Lo que se mide con esto va a docs/budget.md. El criterio de salida del
// Sprint 0 esta en docs/roadmap.md.

// Regla de docs/security.md: cero `unsafe` en codigo propio. La unica excepcion
// prevista es la capa de integracion con el sistema operativo, que todavia no
// existe y cuando exista se aisla en su propio modulo y se revisa a mano.
#![forbid(unsafe_code)]

mod fonts;
mod limits;
mod theme;

use std::collections::HashMap;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;

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
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Theme, Window, WindowId};

use fonts::{FONT_CODE, FONT_DOC, register_embedded_fonts};
use limits::{Degradation, MAX_BLOCKS, MAX_INDENT_DEPTH, MAX_NEST};
use theme::{DAY, NIGHT, Palette, Role};

const MARGIN: f32 = 48.0;
const MAX_MEASURE: f32 = 720.0;
const SELECTION_SCROLL_EDGE: f32 = 32.0;
const SELECTION_SCROLL_MAX_STEP: f32 = 18.0;

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
    Image,
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct TableCell {
    source: SourceRange,
    text: String,
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
    /// Linea horizontal (`---`). No lleva texto.
    Rule,
}

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
            Kind::Quote => 20.0,
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
            NodeValue::Text(t) => output.literal(&t, state, child_source),
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
                    spans.extend(cell_spans);
                    targets.extend(cell_targets);
                    cells.push(TableCell {
                        source: source_index.range_of(cell),
                        text: cell_text,
                    });
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
    options.extension.autolink = true;
    options.extension.tasklist = true;
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
        if blocks.len() >= MAX_BLOCKS {
            return Err("demasiadas lineas para la vista segura");
        }
        let end = start + raw.len();
        let without_lf = raw.strip_suffix('\n').unwrap_or(raw);
        let text = without_lf
            .strip_suffix('\r')
            .unwrap_or(without_lf)
            .to_string();
        let range = SourceRange { start, end };
        debug_assert!(range.is_valid_for(source));
        blocks.push(Block::new(text, Vec::new(), Kind::Code, range, Vec::new()));
        start = end;
    }

    // Evita que el parametro quede puramente documental: el indice y los
    // rangos deben coincidir también en la ultima linea.
    debug_assert_eq!(source_index.len, source.len());
    Ok(blocks)
}

fn parse_blocks(source: &str) -> Result<ParseOutcome, &'static str> {
    let arena = Arena::new();
    let options = markdown_options();
    let root = parse_document(&arena, source, &options);
    let source_index = SourceIndex::new(source);
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

fn window_title(path: &str, safe_mode: Option<Degradation>) -> String {
    let mode = if safe_mode.is_some() {
        " · modo seguro"
    } else {
        ""
    };
    format!("Visor MD v2 · {path}{mode}")
}

fn build_layout(
    block: &Block,
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<Brush>,
    width: f32,
    palette: Palette,
) -> Layout<Brush> {
    let (size, weight, role, mono) = block.kind.style();
    let color = palette.resolve(role);
    let advance = (width - MARGIN * 2.0 - block.kind.indent()).clamp(80.0, MAX_MEASURE);

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
    builder.push_default(StyleProperty::FontSize(size));
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
            builder.push(StyleProperty::FontSize(size * 0.92), range.clone());
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
            builder.push(StyleProperty::FontSize(size * 0.86), range.clone());
        }
        if span.style.sub || span.style.sup {
            builder.push(StyleProperty::FontSize(size * 0.72), range.clone());
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

    let mut layout: Layout<Brush> = builder.build(&block.text);
    layout.break_all_lines(Some(advance));
    layout.align(Alignment::Start, AlignmentOptions::default());
    layout
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
    builder.push_default(StyleProperty::FontSize(size));
    builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(
        kind.line_height(),
    )));

    let mut layout: Layout<Brush> = builder.build(marker);
    layout.break_all_lines(None);
    layout.align(Alignment::Start, AlignmentOptions::default());
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
fn estimate_height(block: &Block, width: f32) -> f32 {
    // La linea horizontal no tiene texto: su alto es el del filete.
    if matches!(block.kind, Kind::Rule) {
        return 1.0;
    }
    let (size, _, _, mono) = block.kind.style();
    let advance = (width - MARGIN * 2.0 - block.kind.indent()).clamp(80.0, MAX_MEASURE);
    // Ancho medio de caracter como fraccion del tamano de fuente. Aproximado
    // a proposito: el error se corrige al maquetar de verdad el bloque.
    let char_w = size * if mono { 0.60 } else { 0.50 };
    let per_line = (advance / char_w).max(1.0);
    let lines = (block.text.chars().count() as f32 / per_line)
        .ceil()
        .max(1.0);
    lines * size * block.kind.line_height()
}

/// Pasada de posicionamiento. Con `exact`, maqueta cada bloque para sacarle el
/// alto real y descarta el layout. Sin `exact`, lo estima sin maquetar: en un
/// documento de 43 mil bloques la diferencia es de segundos a milisegundos.
fn measure_all(
    blocks: &[Block],
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<Brush>,
    width: f32,
    exact: bool,
) -> (Vec<Slot>, f32) {
    let mut slots = Vec::with_capacity(blocks.len());
    let mut y = MARGIN;

    for block in blocks {
        let height = if matches!(block.kind, Kind::Rule) {
            1.0
        } else if exact {
            // El color no afecta el alto: la paleta es irrelevante aca.
            build_layout(block, font_cx, layout_cx, width, NIGHT).height()
        } else {
            estimate_height(block, width)
        };
        y += block.kind.space_before();
        slots.push(Slot {
            y,
            height,
            x: MARGIN + block.kind.indent(),
            kind: block.kind,
        });
        y += height;
    }

    (slots, y + MARGIN)
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

/// Identidad de un glifo ya rasterizado. Sin posicion subpixel: parley ya
/// entrega las posiciones alineadas a pixel (`quantize = true`), asi que una
/// sola mascara por glifo y tamano alcanza.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GlyphKey {
    blob: u64,
    index: u32,
    size: u32,
    glyph: u16,
}

/// Mascara alpha de un glifo, lista para mezclar.
struct CachedGlyph {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
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

        let key = GlyphKey {
            blob,
            index,
            size: font_size.to_bits(),
            glyph: glyph.id as u16,
        };

        if let std::collections::hash_map::Entry::Vacant(vacant) = cache.entry(key) {
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
                Some(image) if matches!(image.content, Content::Mask) => Some(CachedGlyph {
                    left: image.placement.left,
                    top: image.placement.top,
                    width: image.placement.width as i32,
                    height: image.placement.height as i32,
                    data: image.data,
                }),
                _ => None,
            };
            vacant.insert(cached);
        }

        let Some(Some(g)) = cache.get(&key) else {
            continue;
        };
        blit(pixmap, g, gx, gy, color, width, height);
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

// ---------------------------------------------------------------- app

struct App {
    started: Instant,
    path: String,
    blocks: Vec<Block>,
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    font_cx: FontContext,
    layout_cx: LayoutContext<Brush>,
    scale_cx: ScaleContext,
    glyphs: GlyphCache,
    pixmap: Option<Pixmap>,
    slots: Vec<Slot>,
    /// Layouts de los bloques visibles, por indice. Se poda cada cuadro.
    live: HashMap<usize, (Layout<Brush>, Option<CachedMarker>)>,
    doc_height: f32,
    laid_for_width: f32,
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
    /// Las mediciones se acumulan y se imprimen al final. Escribir a stderr
    /// en el medio distorsiona lo que se esta midiendo: con la salida
    /// redirigida a un archivo, cada `eprintln` costaba mas que el trabajo
    /// que pretendia cronometrar.
    log: Vec<String>,
    /// Tema activo. Arranca siguiendo al sistema operativo (`Window::theme`);
    /// `T` lo alterna a mano. Ver docs/design.md.
    palette: Palette,
    /// Indica que el render enriquecido excedio un limite defensivo. En ese
    /// caso `blocks` contiene la fuente inerte, no un arbol truncado.
    safe_mode: Option<Degradation>,
    /// Punto actual del cursor dentro de la ventana, en pixeles físicos.
    pointer: Option<(f32, f32)>,
    /// Mientras está activo, mover el mouse extiende la selección del bloque.
    selecting: bool,
    selection: Option<DocumentSelection>,
    modifiers: ModifiersState,
    /// Conserva el resultado fatal para devolver un codigo de salida distinto
    /// de cero despues de cerrar ordenadamente el event loop.
    fatal_error: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        self.log.push(format!(
            "[medicion]   arranque del bucle hasta `resumed`: {:.0} ms",
            self.started.elapsed().as_secs_f64() * 1000.0
        ));

        let attrs = Window::default_attributes()
            .with_title(window_title(&self.path, self.safe_mode))
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
        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.report();
                event_loop.exit();
            }
            WindowEvent::ThemeChanged(theme) => {
                self.palette = if matches!(theme, Theme::Light) {
                    DAY
                } else {
                    NIGHT
                };
                self.live.clear();
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.pointer = Some((position.x as f32, position.y as f32));
                if self.selecting {
                    self.extend_selection_to(position.x as f32, position.y as f32);
                    if let Some(w) = &self.window {
                        w.request_redraw();
                    }
                }
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => match state {
                ElementState::Pressed => {
                    self.selection = self
                        .pointer
                        .and_then(|(x, y)| self.cursor_at(x, y))
                        .map(DocumentSelection::collapsed);
                    self.selecting = self.selection.is_some();
                    if let Some(w) = &self.window {
                        w.request_redraw();
                    }
                }
                ElementState::Released => self.selecting = false,
            },
            WindowEvent::ModifiersChanged(modifiers) => self.modifiers = modifiers.state(),
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
                let max = max_scroll(self.doc_height, viewport_height);
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

                if let Some(total) = self.bench {
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
                        let max = max_scroll(self.doc_height, viewport_height).max(1.0);
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
    fn cursor_at(&self, x: f32, y: f32) -> Option<BlockCursor> {
        self.slots.iter().enumerate().find_map(|(block, slot)| {
            let top = slot.y - self.scroll;
            if y < top || y > top + slot.height {
                return None;
            }
            let (layout, _) = self.live.get(&block)?;
            let cursor = Cursor::from_point(layout, x - slot.x, y - top);
            Some(BlockCursor {
                block,
                offset: cursor.index(),
            })
        })
    }

    fn extend_selection_to(&mut self, x: f32, y: f32) {
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
        let Some((layout, _)) = self.live.get(&selection.focus.block) else {
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
        let next = (self.scroll + delta).clamp(0.0, max_scroll(self.doc_height, viewport_height));
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

        // Re-medir solo si cambio el ancho.
        if (self.laid_for_width - size.width as f32).abs() > 0.5 {
            let t = Instant::now();
            let (slots, height) = measure_all(
                &self.blocks,
                &mut self.font_cx,
                &mut self.layout_cx,
                size.width as f32,
                self.exact_measure,
            );
            self.slots = slots;
            self.doc_height = height;
            self.laid_for_width = size.width as f32;
            self.scroll = self
                .scroll
                .min(max_scroll(self.doc_height, size.height as f32));
            self.live.clear();
            self.log.push(format!(
                "[medicion] posicionar {} bloques ({}): {:.0} ms  (alto {:.0} px)",
                self.blocks.len(),
                if self.exact_measure {
                    "exacto"
                } else {
                    "estimado"
                },
                t.elapsed().as_secs_f64() * 1000.0,
                height
            ));
        }

        // Que bloques caen en pantalla este cuadro.
        let view_top = self.scroll;
        let view_bottom = self.scroll + size.height as f32;
        let visible = visible_range(&self.slots, view_top, view_bottom);

        // Solo los visibles conservan su layout vivo.
        self.live.retain(|i, _| visible.contains(i));
        for i in visible.clone() {
            if !self.live.contains_key(&i) {
                let block = &self.blocks[i];
                let layout = build_layout(
                    block,
                    &mut self.font_cx,
                    &mut self.layout_cx,
                    size.width as f32,
                    self.palette,
                );
                let marker = block.marker.as_ref().map(|m| match m {
                    Marker::Text(text) => CachedMarker::Text(Box::new(build_marker_layout(
                        text,
                        block.kind,
                        &mut self.font_cx,
                        &mut self.layout_cx,
                        self.palette,
                    ))),
                    Marker::Task { done } => CachedMarker::Task { done: *done },
                });
                self.live.insert(i, (layout, marker));
            }
        }

        if self.autoscroll_selection(size.height as f32) {
            window.request_redraw();
        }

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
        let App {
            pixmap,
            slots,
            live,
            scale_cx,
            glyphs,
            scroll,
            surface,
            palette,
            selection,
            blocks,
            ..
        } = self;
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

        let ancho_texto = (w.get() as f32 - MARGIN * 2.0).min(MAX_MEASURE);

        for i in visible {
            let slot = &slots[i];
            let Some((layout, marker)) = live.get(&i) else {
                continue;
            };
            let top = slot.y - *scroll;

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
                }
                // Filete de acento a la izquierda de la cita, como las alertas.
                Kind::Quote => {
                    if let Some(rect) = Rect::from_xywh(slot.x - 20.0, top, 3.0, slot.height) {
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
                        let size = (font_size * 0.82).max(12.0);
                        let line_box = font_size * slot.kind.line_height();
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
    let Some(path) = args.iter().find(|a| !a.starts_with("--")).cloned() else {
        eprintln!("uso: visor-md <archivo.md> [--bench[=N]] [--exacto]");
        std::process::exit(2);
    };

    let source = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("no se pudo leer {path}: {e}");
            std::process::exit(1);
        }
    };

    let t = Instant::now();
    let outcome = match parse_blocks(&source) {
        Ok(outcome) => outcome,
        Err(error) => {
            eprintln!("el documento no se pudo preparar de forma segura: {error}");
            std::process::exit(1);
        }
    };
    let blocks = outcome.blocks;
    let degradation = outcome.degradation;
    let mut log = vec![format!(
        "[medicion] parseo de {:.1} KB: {:.0} ms  ({} bloques)",
        source.len() as f64 / 1024.0,
        t.elapsed().as_secs_f64() * 1000.0,
        blocks.len()
    )];
    if let Some(reason) = degradation {
        log.push(format!(
            "[seguridad] {}; se muestra la fuente inerte",
            reason.explanation()
        ));
    }

    let t = Instant::now();
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(error) => {
            eprintln!("no se pudo iniciar la interfaz: {error}");
            std::process::exit(1);
        }
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    log.push(format!(
        "[medicion]   EventLoop::new: {:.0} ms",
        t.elapsed().as_secs_f64() * 1000.0
    ));

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

    let mut app = App {
        started,
        path,
        blocks,
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
        scroll: 0.0,
        first_paint_done: false,
        frames: 0,
        frame_time_total: 0.0,
        bench,
        exact_measure,
        log,
        palette: NIGHT,
        safe_mode: degradation,
        pointer: None,
        selecting: false,
        selection: None,
        modifiers: ModifiersState::empty(),
        fatal_error: false,
    };

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
    fn el_titulo_hace_visible_el_modo_seguro() {
        let normal = window_title("nota.md", None);
        let seguro = window_title("hostil.md", Some(Degradation::DepthLimit));
        assert!(!normal.contains("modo seguro"));
        assert!(seguro.contains("modo seguro"));
        assert!(seguro.contains("hostil.md"));
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

    /// Un bloque siempre ocupa algo. Un alto de cero haria que se superpongan
    /// y que la barra de scroll mienta hacia el otro lado.
    #[test]
    fn el_alto_estimado_siempre_es_positivo() {
        for md in ["# t", "texto", "- a", "```\nx\n```", "|a|\n|-|\n|b|"] {
            for block in aplanar(md) {
                for ancho in [200.0, 900.0, 4000.0] {
                    let h = estimate_height(&block, ancho);
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
            .map(|b| build_layout(b, &mut font_cx, &mut layout_cx, ancho, NIGHT).height())
            .sum();
        let estimado: f32 = blocks.iter().map(|b| estimate_height(b, ancho)).sum();

        let error = (estimado - real).abs() / real;
        assert!(
            error < 0.35,
            "la estimacion se desvio {:.1}% (real {real:.0} px, estimado {estimado:.0} px)",
            error * 100.0
        );
    }

    #[test]
    fn la_seleccion_tiene_geometria_dibujable() {
        let block = &aplanar("Selecciona estas palabras con el mouse.")[0];
        let start = block.text.find("estas").expect("texto de prueba");
        let end = start + "estas palabras".len();
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        let layout = build_layout(block, &mut font_cx, &mut layout_cx, 900.0, NIGHT);
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

        let visible = outcome
            .blocks
            .iter()
            .map(|block| block.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(visible.contains("concepto clave"));
        assert!(visible.contains("<script"));
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
    fn subrayado_y_tachado_llegan_al_layout() {
        let blocks = aplanar("un [enlace](https://example.com) y ~~tachado~~");
        let block = &blocks[0];
        let mut font_cx = FontContext::new();
        register_embedded_fonts(&mut font_cx);
        let mut layout_cx = LayoutContext::new();
        let layout = build_layout(block, &mut font_cx, &mut layout_cx, 900.0, NIGHT);

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
        let layout = build_layout(block, &mut font_cx, &mut layout_cx, 900.0, NIGHT);
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
    fn las_citas_anidadas_conservan_profundidad() {
        let blocks = aplanar("> exterior\n>\n> > interior");
        let depths: Vec<_> = blocks.iter().map(|block| block.quote_depth).collect();
        assert!(depths.contains(&1), "falta cita exterior: {depths:?}");
        assert!(depths.contains(&2), "falta cita interior: {depths:?}");
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.kind, self.text)
    }
}
