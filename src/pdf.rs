//! Escritor PDF mínimo y sin dependencias nuevas.
//!
//! Produce PDF 1.4 estático: páginas con trazados vectoriales, glifos
//! convertidos en formas reutilizables y una capa de texto invisible para
//! buscar y copiar. No escribe JavaScript, acciones, enlaces activos,
//! formularios, adjuntos ni recursos externos: el archivo resultante no tiene
//! comportamiento propio al abrirse.

use std::collections::HashMap;
use std::fmt::Write as _;

use swash::scale::outline::Outline;
use swash::zeno::Verb;

/// Medidas A4 en puntos tipográficos.
pub(crate) const A4_WIDTH: f32 = 595.28;
pub(crate) const A4_HEIGHT: f32 = 841.89;

/// Tamaño de referencia al convertir un glifo en forma reutilizable. La forma
/// se escala después con la matriz de cada aparición.
pub(crate) const GLYPH_UNITS: f32 = 1000.0;

pub(crate) struct PdfBuilder {
    width: f32,
    height: f32,
    pages: Vec<String>,
    glyph_forms: Vec<GlyphForm>,
    glyph_lookup: HashMap<GlyphFormKey, Option<usize>>,
}

struct GlyphForm {
    ops: String,
    bbox: [f32; 4],
}

/// Identidad de un glifo ya convertido: fuente, cara, variaciones y glifo.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct GlyphFormKey {
    pub(crate) font: u64,
    pub(crate) index: u32,
    pub(crate) coords: Vec<i16>,
    pub(crate) glyph: u32,
}

impl PdfBuilder {
    pub(crate) fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            pages: vec![String::new()],
            glyph_forms: Vec::new(),
            glyph_lookup: HashMap::new(),
        }
    }

    pub(crate) fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub(crate) fn new_page(&mut self) {
        self.pages.push(String::new());
    }

    fn content(&mut self) -> &mut String {
        if self.pages.is_empty() {
            self.pages.push(String::new());
        }
        let last = self.pages.len() - 1;
        &mut self.pages[last]
    }

    /// Rectángulo relleno. Coordenadas con origen arriba a la izquierda, como
    /// en el resto de la aplicación; la conversión a PDF ocurre aquí.
    pub(crate) fn fill_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: (u8, u8, u8),
    ) {
        if width <= 0.0 || height <= 0.0 {
            return;
        }
        let bottom = self.height - y - height;
        let content = self.content();
        let _ = writeln!(
            content,
            "{} rg {} {} {} {} re f",
            rgb(color),
            num(x),
            num(bottom),
            num(width),
            num(height)
        );
    }

    /// Contorno de rectángulo, usado para casillas de tareas y celdas.
    pub(crate) fn stroke_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        line: f32,
        color: (u8, u8, u8),
    ) {
        let bottom = self.height - y - height;
        let content = self.content();
        let _ = writeln!(
            content,
            "{} RG {} w {} {} {} {} re S",
            rgb(color),
            num(line),
            num(x),
            num(bottom),
            num(width),
            num(height)
        );
    }

    /// Tilde de una tarea hecha, como polilínea.
    pub(crate) fn check_mark(&mut self, x: f32, y: f32, size: f32, color: (u8, u8, u8)) {
        let page_height = self.height;
        let point = |px: f32, py: f32| {
            format!(
                "{} {}",
                num(x + px * size),
                num(page_height - (y + py * size))
            )
        };
        let ops = format!(
            "{} RG {} w 1 J 1 j {} m {} l {} l S",
            rgb(color),
            num(size * 0.14),
            point(0.22, 0.52),
            point(0.42, 0.72),
            point(0.78, 0.28)
        );
        let content = self.content();
        let _ = writeln!(content, "{ops}");
    }

    /// Registra la forma de un glifo una sola vez. Devuelve `None` si el glifo
    /// no tiene contorno (espacios o emojis en mapa de bits).
    pub(crate) fn glyph_form(
        &mut self,
        key: GlyphFormKey,
        outline: impl FnOnce() -> Option<Outline>,
    ) -> Option<usize> {
        if let Some(found) = self.glyph_lookup.get(&key) {
            return *found;
        }
        let form = outline().and_then(|outline| outline_to_form(&outline));
        let index = form.map(|form| {
            self.glyph_forms.push(form);
            self.glyph_forms.len() - 1
        });
        self.glyph_lookup.insert(key, index);
        index
    }

    /// Dibuja un glifo registrado con su línea base en `(x, baseline)`.
    pub(crate) fn draw_glyph(
        &mut self,
        form: usize,
        x: f32,
        baseline: f32,
        size: f32,
        color: (u8, u8, u8),
    ) {
        let scale = size / GLYPH_UNITS;
        let y = self.height - baseline;
        let content = self.content();
        let _ = writeln!(
            content,
            "q {} rg {} 0 0 {} {} {} cm /G{form} Do Q",
            rgb(color),
            num(scale),
            num(scale),
            num(x),
            num(y)
        );
    }

    /// Texto invisible (modo 3) colocado sobre los glifos dibujados. Permite
    /// buscar y copiar sin depender de incrustar fuentes variables.
    pub(crate) fn hidden_text(&mut self, x: f32, baseline: f32, size: f32, text: &str) {
        let Some(encoded) = encode_win_ansi(text) else {
            return;
        };
        if encoded.is_empty() {
            return;
        }
        let y = self.height - baseline;
        let content = self.content();
        let _ = writeln!(
            content,
            "BT 3 Tr /F1 {} Tf 1 0 0 1 {} {} Tm ({}) Tj ET",
            num(size.max(1.0)),
            num(x),
            num(y),
            escape_pdf_string(&encoded)
        );
    }

    /// Serializa el documento completo. Solo objetos estáticos.
    pub(crate) fn finish(self, title: &str) -> Vec<u8> {
        let mut objects: Vec<Vec<u8>> = Vec::new();
        // 1: catálogo, 2: árbol de páginas, 3: fuente oculta, 4: información.
        let page_count = self.pages.len();
        let first_form = 5;
        let first_page = first_form + self.glyph_forms.len();

        let mut xobjects = String::new();
        for index in 0..self.glyph_forms.len() {
            let _ = write!(xobjects, "/G{index} {} 0 R ", first_form + index);
        }
        let kids = (0..page_count)
            .map(|page| format!("{} 0 R", first_page + page * 2))
            .collect::<Vec<_>>()
            .join(" ");

        objects.push(b"<< /Type /Catalog /Pages 2 0 R >>".to_vec());
        objects.push(format!("<< /Type /Pages /Kids [{kids}] /Count {page_count} >>").into_bytes());
        objects.push(
            b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"
                .to_vec(),
        );
        let encoded_title = encode_win_ansi(title).unwrap_or_default();
        objects.push(
            format!(
                "<< /Title ({}) /Producer (Visor MD) >>",
                escape_pdf_string(&encoded_title)
            )
            .into_bytes(),
        );
        for form in &self.glyph_forms {
            objects.push(stream_object(
                &format!(
                    "/Type /XObject /Subtype /Form /BBox [{} {} {} {}]",
                    num(form.bbox[0]),
                    num(form.bbox[1]),
                    num(form.bbox[2]),
                    num(form.bbox[3])
                ),
                form.ops.as_bytes(),
            ));
        }
        for (page, content) in self.pages.iter().enumerate() {
            let content_id = first_page + page * 2 + 1;
            objects.push(
                format!(
                    "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font << /F1 3 0 R >> /XObject << {xobjects}>> >> /Contents {content_id} 0 R >>",
                    num(self.width),
                    num(self.height)
                )
                .into_bytes(),
            );
            objects.push(stream_object("", content.as_bytes()));
        }

        let mut output = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
        let mut offsets = Vec::with_capacity(objects.len());
        for (index, body) in objects.iter().enumerate() {
            offsets.push(output.len());
            output.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
            output.extend_from_slice(body);
            output.extend_from_slice(b"\nendobj\n");
        }
        let xref = output.len();
        output.extend_from_slice(
            format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes(),
        );
        for offset in offsets {
            output.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        output.extend_from_slice(
            format!(
                "trailer\n<< /Size {} /Root 1 0 R /Info 4 0 R >>\nstartxref\n{xref}\n%%EOF\n",
                objects.len() + 1
            )
            .as_bytes(),
        );
        output
    }
}

fn stream_object(dictionary: &str, data: &[u8]) -> Vec<u8> {
    let mut object = format!("<< {dictionary} /Length {} >>\nstream\n", data.len()).into_bytes();
    object.extend_from_slice(data);
    object.extend_from_slice(b"\nendstream");
    object
}

/// Convierte un contorno de swash (y hacia arriba, unidades del tamaño de
/// referencia) en operadores de trazado PDF rellenos con la regla no nula.
fn outline_to_form(outline: &Outline) -> Option<GlyphForm> {
    let points = outline.points();
    let verbs = outline.verbs();
    if verbs.is_empty() || points.is_empty() {
        return None;
    }
    let mut ops = String::new();
    let mut cursor = 0usize;
    let mut last = (0.0f32, 0.0f32);
    let mut min = (f32::MAX, f32::MAX);
    let mut max = (f32::MIN, f32::MIN);
    let mut take = |count: usize, cursor: &mut usize| -> Option<Vec<(f32, f32)>> {
        let slice = points.get(*cursor..*cursor + count)?;
        *cursor += count;
        Some(
            slice
                .iter()
                .map(|point| {
                    min = (min.0.min(point.x), min.1.min(point.y));
                    max = (max.0.max(point.x), max.1.max(point.y));
                    (point.x, point.y)
                })
                .collect(),
        )
    };
    for verb in verbs {
        match verb {
            Verb::MoveTo => {
                let p = take(1, &mut cursor)?;
                let _ = write!(ops, "{} {} m ", num(p[0].0), num(p[0].1));
                last = p[0];
            }
            Verb::LineTo => {
                let p = take(1, &mut cursor)?;
                let _ = write!(ops, "{} {} l ", num(p[0].0), num(p[0].1));
                last = p[0];
            }
            Verb::QuadTo => {
                let p = take(2, &mut cursor)?;
                // Una cuadrática se expresa exactamente como cúbica.
                let c1 = (
                    last.0 + (p[0].0 - last.0) * 2.0 / 3.0,
                    last.1 + (p[0].1 - last.1) * 2.0 / 3.0,
                );
                let c2 = (
                    p[1].0 + (p[0].0 - p[1].0) * 2.0 / 3.0,
                    p[1].1 + (p[0].1 - p[1].1) * 2.0 / 3.0,
                );
                let _ = write!(
                    ops,
                    "{} {} {} {} {} {} c ",
                    num(c1.0),
                    num(c1.1),
                    num(c2.0),
                    num(c2.1),
                    num(p[1].0),
                    num(p[1].1)
                );
                last = p[1];
            }
            Verb::CurveTo => {
                let p = take(3, &mut cursor)?;
                let _ = write!(
                    ops,
                    "{} {} {} {} {} {} c ",
                    num(p[0].0),
                    num(p[0].1),
                    num(p[1].0),
                    num(p[1].1),
                    num(p[2].0),
                    num(p[2].1)
                );
                last = p[2];
            }
            Verb::Close => ops.push_str("h "),
        }
    }
    ops.push('f');
    Some(GlyphForm {
        ops,
        bbox: [min.0 - 1.0, min.1 - 1.0, max.0 + 1.0, max.1 + 1.0],
    })
}

/// Texto para la capa invisible en WinAnsi (cp1252). Cubre español y la
/// mayoría de idiomas de Europa occidental; lo que no puede codificarse se
/// omite de la capa de búsqueda, nunca del dibujo.
pub(crate) fn encode_win_ansi(text: &str) -> Option<Vec<u8>> {
    let mut bytes = Vec::with_capacity(text.len());
    for character in text.chars() {
        let code = character as u32;
        let byte = match code {
            0x20..=0x7E | 0xA0..=0xFF => code as u8,
            0x2014 => 0x97,
            0x2013 => 0x96,
            0x2018 => 0x91,
            0x2019 => 0x92,
            0x201C => 0x93,
            0x201D => 0x94,
            0x2022 => 0x95,
            0x2026 => 0x85,
            0x20AC => 0x80,
            0x2122 => 0x99,
            _ => continue,
        };
        bytes.push(byte);
    }
    Some(bytes)
}

fn escape_pdf_string(bytes: &[u8]) -> String {
    let mut escaped = String::with_capacity(bytes.len() + 8);
    for byte in bytes {
        match byte {
            b'(' => escaped.push_str("\\("),
            b')' => escaped.push_str("\\)"),
            b'\\' => escaped.push_str("\\\\"),
            0x20..=0x7E => escaped.push(*byte as char),
            other => {
                let _ = write!(escaped, "\\{other:03o}");
            }
        }
    }
    escaped
}

fn rgb(color: (u8, u8, u8)) -> String {
    format!(
        "{} {} {}",
        num(f32::from(color.0) / 255.0),
        num(f32::from(color.1) / 255.0),
        num(f32::from(color.2) / 255.0)
    )
}

/// Número compacto con hasta tres decimales, sin notación científica.
fn num(value: f32) -> String {
    if !value.is_finite() {
        return "0".to_string();
    }
    let text = format!("{value:.3}");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() || trimmed == "-0" {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn el_pdf_vacio_es_estructuralmente_valido_y_sin_acciones() {
        let mut pdf = PdfBuilder::new(A4_WIDTH, A4_HEIGHT);
        pdf.fill_rect(10.0, 10.0, 50.0, 20.0, (200, 200, 200));
        pdf.hidden_text(10.0, 40.0, 12.0, "Título (con paréntesis) \\ y acentos");
        pdf.new_page();
        let bytes = pdf.finish("Prueba");
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.starts_with("%PDF-1.4"));
        assert!(text.trim_end().ends_with("%%EOF"));
        assert!(text.contains("/Count 2"));
        for forbidden in [
            "/JavaScript",
            "/JS",
            "/OpenAction",
            "/AA",
            "/URI",
            "/Launch",
            "/EmbeddedFile",
        ] {
            assert!(!text.contains(forbidden), "{forbidden}");
        }
        assert!(text.contains("\\(con par"));
        // El xref apunta exactamente al inicio de cada objeto.
        let xref_at = text.rfind("startxref\n").unwrap() + "startxref\n".len();
        let xref: usize = text[xref_at..].lines().next().unwrap().parse().unwrap();
        assert!(bytes[xref..].starts_with(b"xref"));
        let entries = text[xref..].lines().skip(3);
        for (index, entry) in entries.take_while(|line| line.ends_with(" n ")).enumerate() {
            let offset: usize = entry[..10].parse().unwrap();
            assert!(bytes[offset..].starts_with(format!("{} 0 obj", index + 1).as_bytes()));
        }
    }

    #[test]
    fn los_numeros_y_el_texto_se_escriben_de_forma_segura() {
        assert_eq!(num(1.5), "1.5");
        assert_eq!(num(2.0), "2");
        assert_eq!(num(-0.0001), "0");
        assert_eq!(num(f32::NAN), "0");
        assert_eq!(
            encode_win_ansi("ñandú — €").unwrap(),
            vec![0xF1, b'a', b'n', b'd', 0xFA, b' ', 0x97, b' ', 0x80]
        );
        assert_eq!(encode_win_ansi("漢字✓").unwrap(), Vec::<u8>::new());
        assert_eq!(escape_pdf_string(b"a(b)c\\"), "a\\(b\\)c\\\\");
    }
}
