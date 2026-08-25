// Visor MD v2 - Sprint 0
//
// Prototipo de medicion, no de producto. Abre un .md, lo parsea con comrak,
// lo maqueta con parley y lo dibuja con tiny-skia sobre una ventana
// winit + softbuffer. Sin chrome, sin pestanas, sin Mermaid.
//
// Lo que se mide con esto va a docs/budget.md. El criterio de salida del
// Sprint 0 esta en docs/roadmap.md.

// Regla de docs/security.md: cero `unsafe` en codigo propio. La unica excepcion
// prevista es la capa de integracion con el sistema operativo, que todavia no
// existe y cuando exista se aisla en su propio modulo y se revisa a mano.
#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;

use comrak::nodes::{AstNode, NodeValue};
use comrak::{Arena, Options, parse_document};
use parley::layout::{Alignment, GlyphRun, Layout, PositionedLayoutItem};
use parley::style::{FontFamily, FontFamilyName, FontWeight, GenericFamily, StyleProperty};
use parley::{AlignmentOptions, FontContext, LayoutContext, LineHeight};
use swash::FontRef;
use swash::scale::image::Content;
use swash::scale::{Render, ScaleContext, Source, StrikeWith};
use swash::zeno::{Format, Vector};
use tiny_skia::{Color, Paint, Pixmap, PremultipliedColorU8, Rect, Transform};
use winit::application::ApplicationHandler;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

// Paleta Papel + Tinta, tema noche. Ver docs/design.md.
const BG: (u8, u8, u8) = (0x0C, 0x0F, 0x0D);
const SURFACE: (u8, u8, u8) = (0x12, 0x15, 0x13);
const TEXT: (u8, u8, u8) = (0xE9, 0xE9, 0xE4);
const DIM: (u8, u8, u8) = (0x8B, 0x91, 0x8C);
const ACCENT: (u8, u8, u8) = (0x5F, 0xD0, 0x8A);

const MARGIN: f32 = 48.0;
const MAX_MEASURE: f32 = 720.0;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct Brush(u8, u8, u8);

/// Un bloque del documento ya aplanado: el arbol tipado de comrak reducido a
/// lo que el Sprint 0 sabe dibujar.
#[derive(Clone, Copy, Debug)]
enum Kind {
    Heading(u8),
    Para,
    Item(u8),
    Code,
    TableRow { header: bool },
    Quote,
}

struct Block {
    text: String,
    kind: Kind,
}

impl Kind {
    /// (tamano, peso, color, monoespaciada)
    fn style(self) -> (f32, f32, (u8, u8, u8), bool) {
        match self {
            // Escala tipografica de docs/design.md.
            Kind::Heading(1) => (31.0, 700.0, TEXT, false),
            Kind::Heading(2) => (25.0, 700.0, TEXT, false),
            Kind::Heading(3) => (20.0, 600.0, TEXT, false),
            Kind::Heading(_) => (17.0, 600.0, TEXT, false),
            Kind::Para | Kind::Item(_) => (16.0, 400.0, TEXT, false),
            Kind::Code => (13.5, 400.0, ACCENT, true),
            Kind::TableRow { header: true } => (15.0, 600.0, TEXT, true),
            Kind::TableRow { header: false } => (15.0, 400.0, DIM, true),
            Kind::Quote => (16.0, 400.0, DIM, false),
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
            _ => 16.0,
        }
    }

    fn indent(self) -> f32 {
        match self {
            Kind::Item(d) => 20.0 * d as f32,
            Kind::Quote => 16.0,
            _ => 0.0,
        }
    }
}

// ---------------------------------------------------------------- parseo

fn inline_text<'a>(node: &'a AstNode<'a>, out: &mut String) {
    for child in node.children() {
        match &child.data.borrow().value {
            NodeValue::Text(t) => out.push_str(t),
            NodeValue::Code(c) => out.push_str(&c.literal),
            NodeValue::SoftBreak | NodeValue::LineBreak => out.push(' '),
            _ => inline_text(child, out),
        }
    }
}

fn flatten<'a>(node: &'a AstNode<'a>, depth: u8, out: &mut Vec<Block>) {
    for child in node.children() {
        // El borrow se suelta antes de recurrir: comrak usa RefCell y una
        // recursion con el prestamo vivo entra en panico.
        let value = child.data.borrow().value.clone();
        match value {
            NodeValue::Heading(h) => {
                let mut text = String::new();
                inline_text(child, &mut text);
                push(out, text, Kind::Heading(h.level));
            }
            NodeValue::Paragraph => {
                let mut text = String::new();
                inline_text(child, &mut text);
                // Un parrafo dentro de un item de lista se dibuja como item.
                let kind = if depth == 0 {
                    Kind::Para
                } else {
                    Kind::Item(depth)
                };
                push(out, text, kind);
            }
            NodeValue::Item(_) | NodeValue::TaskItem(_) => flatten(child, depth + 1, out),
            NodeValue::CodeBlock(cb) => {
                for line in cb.literal.lines() {
                    out.push(Block {
                        text: line.to_string(),
                        kind: Kind::Code,
                    });
                }
            }
            NodeValue::TableRow(header) => {
                let mut cells = Vec::new();
                for cell in child.children() {
                    let mut text = String::new();
                    inline_text(cell, &mut text);
                    cells.push(text);
                }
                push(out, cells.join("  |  "), Kind::TableRow { header });
            }
            NodeValue::BlockQuote => {
                let mut text = String::new();
                inline_text(child, &mut text);
                push(out, text, Kind::Quote);
            }
            _ => flatten(child, depth, out),
        }
    }
}

fn push(out: &mut Vec<Block>, text: String, kind: Kind) {
    if !text.trim().is_empty() {
        out.push(Block { text, kind });
    }
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

fn build_layout(
    block: &Block,
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<Brush>,
    width: f32,
) -> Layout<Brush> {
    let (size, weight, color, mono) = block.kind.style();
    let advance = (width - MARGIN * 2.0 - block.kind.indent())
        .min(MAX_MEASURE)
        .max(80.0);

    let family = if mono {
        GenericFamily::Monospace
    } else {
        GenericFamily::SystemUi
    };

    let mut builder = layout_cx.ranged_builder(font_cx, &block.text, 1.0, true);
    builder.push_default(StyleProperty::Brush(Brush(color.0, color.1, color.2)));
    builder.push_default(StyleProperty::FontFamily(FontFamily::Single(
        FontFamilyName::Generic(family),
    )));
    builder.push_default(StyleProperty::FontSize(size));
    builder.push_default(StyleProperty::FontWeight(FontWeight::new(weight)));
    builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(
        block.kind.line_height(),
    )));

    let mut layout: Layout<Brush> = builder.build(&block.text);
    layout.break_all_lines(Some(advance));
    layout.align(Alignment::Start, AlignmentOptions::default());
    layout
}

/// Alto aproximado de un bloque **sin maquetarlo**: cuenta caracteres y
/// estima cuantos entran por linea. No sirve para dibujar, solo para saber
/// donde cae cada bloque en la barra de scroll.
fn estimate_height(block: &Block, width: f32) -> f32 {
    let (size, _, _, mono) = block.kind.style();
    let advance = (width - MARGIN * 2.0 - block.kind.indent())
        .min(MAX_MEASURE)
        .max(80.0);
    // Ancho medio de caracter como fraccion del tamano de fuente. Aproximado
    // a proposito: el error se corrige al maquetar de verdad el bloque.
    let char_w = size * if mono { 0.60 } else { 0.50 };
    let per_line = (advance / char_w).max(1.0);
    let lines = (block.text.chars().count() as f32 / per_line).ceil().max(1.0);
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
        let height = if exact {
            build_layout(block, font_cx, layout_cx, width).height()
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
#[derive(PartialEq, Eq, Hash)]
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
    let color = (brush.0, brush.1, brush.2);

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
        let gy = origin_y + run_y + glyph.y;
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

        if !cache.contains_key(&key) {
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
            let s = scaler.as_mut().unwrap();

            let rendered = Render::new(&[
                Source::ColorOutline(0),
                Source::ColorBitmap(StrikeWith::BestFit),
                Source::Outline,
            ])
            .format(Format::Alpha)
            .offset(Vector::new(0.0, 0.0))
            .render(s, glyph.id as u16);

            let entry = match rendered {
                Some(image) if matches!(image.content, Content::Mask) => Some(CachedGlyph {
                    left: image.placement.left,
                    top: image.placement.top,
                    width: image.placement.width as i32,
                    height: image.placement.height as i32,
                    data: image.data,
                }),
                _ => None,
            };
            cache.insert(key, entry);

            // Se vuelve a buscar abajo con la clave recien insertada.
            let key = GlyphKey {
                blob,
                index,
                size: font_size.to_bits(),
                glyph: glyph.id as u16,
            };
            let Some(Some(g)) = cache.get(&key) else {
                continue;
            };
            blit(pixmap, g, gx, gy, color, width, height);
            continue;
        }

        let Some(Some(g)) = cache.get(&key) else {
            continue;
        };
        blit(pixmap, g, gx, gy, color, width, height);
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
    live: HashMap<usize, Layout<Brush>>,
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
            .with_title(format!("Visor MD v2 · Sprint 0 · {}", self.path))
            .with_inner_size(winit::dpi::LogicalSize::new(900.0, 760.0));
        let t = Instant::now();
        let window = Rc::new(event_loop.create_window(attrs).unwrap());
        self.log.push(format!(
            "[medicion]   create_window: {:.0} ms",
            t.elapsed().as_secs_f64() * 1000.0
        ));
        self.log.push(format!(
            "[medicion] ventana visible: {:.0} ms",
            self.started.elapsed().as_secs_f64() * 1000.0
        ));

        let t = Instant::now();
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        self.log.push(format!(
            "[medicion]   superficie softbuffer: {:.0} ms",
            t.elapsed().as_secs_f64() * 1000.0
        ));
        self.surface = Some(surface);
        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.report();
                event_loop.exit();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y * 60.0,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                let max = (self.doc_height - 200.0).max(0.0);
                self.scroll = (self.scroll - dy).clamp(0.0, max);
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                self.redraw();

                if let Some(total) = self.bench {
                    if self.frames >= total {
                        self.report();
                        event_loop.exit();
                    } else {
                        // Avanza un salto fijo por cuadro, dando la vuelta al
                        // documento entero para que la medicion no se quede
                        // midiendo siempre la misma pantalla.
                        let max = (self.doc_height - 200.0).max(1.0);
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
    fn redraw(&mut self) {
        let Some(window) = self.window.clone() else {
            return;
        };
        let size = window.inner_size();
        let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else {
            return;
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
            self.live.clear();
            self.log.push(format!(
                "[medicion] posicionar {} bloques ({}): {:.0} ms  (alto {:.0} px)",
                self.blocks.len(),
                if self.exact_measure { "exacto" } else { "estimado" },
                t.elapsed().as_secs_f64() * 1000.0,
                height
            ));
        }

        // Que bloques caen en pantalla este cuadro.
        let view_top = self.scroll;
        let view_bottom = self.scroll + size.height as f32;
        let visible: Vec<usize> = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, s)| s.y + s.height >= view_top && s.y <= view_bottom)
            .map(|(i, _)| i)
            .collect();

        // Solo los visibles conservan su layout vivo.
        self.live.retain(|i, _| visible.contains(i));
        for &i in &visible {
            if !self.live.contains_key(&i) {
                let layout = build_layout(
                    &self.blocks[i],
                    &mut self.font_cx,
                    &mut self.layout_cx,
                    size.width as f32,
                );
                self.live.insert(i, layout);
            }
        }

        // El pixmap se reusa entre cuadros: reservar 2,7 MB por cuadro y
        // ponerlos en cero es trabajo que no hace falta repetir.
        let needs_new = self
            .pixmap
            .as_ref()
            .is_none_or(|p| p.width() != w.get() || p.height() != h.get());
        if needs_new {
            self.pixmap = Some(Pixmap::new(w.get(), h.get()).unwrap());
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
            ..
        } = self;
        let pixmap = pixmap.as_mut().unwrap();
        pixmap.fill(Color::from_rgba8(BG.0, BG.1, BG.2, 255));

        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba8(SURFACE.0, SURFACE.1, SURFACE.2, 255));

        for &i in &visible {
            let slot = &slots[i];
            let Some(layout) = live.get(&i) else { continue };
            let top = slot.y - *scroll;

            // Fondo de los bloques de codigo, dibujado con tiny-skia.
            if matches!(slot.kind, Kind::Code) {
                let rect_w = (w.get() as f32 - MARGIN * 2.0 + 24.0).min(MAX_MEASURE + 24.0);
                if let Some(rect) =
                    Rect::from_xywh(slot.x - 12.0, top - 2.0, rect_w, slot.height + 4.0)
                {
                    pixmap.fill_rect(rect, &paint, Transform::identity(), None);
                }
            }

            for line in layout.lines() {
                for entry in line.items() {
                    if let PositionedLayoutItem::GlyphRun(run) = entry {
                        draw_glyph_run(pixmap, scale_cx, glyphs, &run, slot.x, top);
                    }
                }
            }
        }

        // Volcado del pixmap a la ventana.
        let surface = surface.as_mut().unwrap();
        surface.resize(w, h).unwrap();
        let mut buffer = surface.buffer_mut().unwrap();
        for (dst, src) in buffer.iter_mut().zip(pixmap.pixels()) {
            *dst = ((src.red() as u32) << 16) | ((src.green() as u32) << 8) | src.blue() as u32;
        }
        buffer.present().unwrap();

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
        Some(rest.strip_prefix('=').and_then(|n| n.parse().ok()).unwrap_or(240))
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
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    let root = parse_document(&arena, &source, &options);
    let mut blocks = Vec::new();
    flatten(root, 0, &mut blocks);
    let mut log = vec![format!(
        "[medicion] parseo de {:.1} KB: {:.0} ms  ({} bloques)",
        source.len() as f64 / 1024.0,
        t.elapsed().as_secs_f64() * 1000.0,
        blocks.len()
    )];

    let t = Instant::now();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    log.push(format!(
        "[medicion]   EventLoop::new: {:.0} ms",
        t.elapsed().as_secs_f64() * 1000.0
    ));

    let t = Instant::now();
    let font_cx = FontContext::new();
    log.push(format!(
        "[medicion]   FontContext::new (fuentes del sistema): {:.0} ms",
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
    };

    event_loop.run_app(&mut app).unwrap();
}

// ---------------------------------------------------------------- pruebas

#[cfg(test)]
mod pruebas {
    use super::*;

    fn aplanar(md: &str) -> Vec<Block> {
        let arena = Arena::new();
        let mut options = Options::default();
        options.extension.table = true;
        options.extension.strikethrough = true;
        options.extension.autolink = true;
        options.extension.tasklist = true;
        let root = parse_document(&arena, md, &options);
        let mut blocks = Vec::new();
        flatten(root, 0, &mut blocks);
        blocks
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
        let mut layout_cx = LayoutContext::new();
        let ancho = 900.0;

        let real: f32 = blocks
            .iter()
            .map(|b| build_layout(b, &mut font_cx, &mut layout_cx, ancho).height())
            .sum();
        let estimado: f32 = blocks.iter().map(|b| estimate_height(b, ancho)).sum();

        let error = (estimado - real).abs() / real;
        assert!(
            error < 0.35,
            "la estimacion se desvio {:.1}% (real {real:.0} px, estimado {estimado:.0} px)",
            error * 100.0
        );
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
}
