//! Herramientas de estudio sobre el documento (fase 14): traducir entre lo que
//! se ve en Leer y la fuente Markdown, crear y quitar subrayados y calcular qué
//! tramos pintar. Sin interfaz: solo texto, rangos y el formato de
//! `anotaciones`.

use crate::anotaciones::{self, Ancla, Anotaciones, Color, Forma, Lectura, Registro, Tipo, Valor};

/// Tope de la cita de un subrayado (diseño, 5.6).
pub(crate) const MAX_CITA_BYTES: usize = 4096;

/// Posiciones del cuerpo normalizado a LF (en caracteres) frente a bytes del
/// texto real, que puede tener CRLF. Las anotaciones cuentan sobre el primero;
/// el documento y sus bloques, sobre el segundo.
pub(crate) struct LfIndex {
    /// El cuerpo con CRLF→LF.
    pub(crate) lf: String,
    /// Byte del texto real donde empieza cada carácter del cuerpo LF, y al
    /// final el largo total.
    bytes: Vec<usize>,
}

impl LfIndex {
    pub(crate) fn new(source: &str) -> Self {
        let mut lf = String::with_capacity(source.len());
        let mut bytes = Vec::with_capacity(source.len() + 1);
        let mut chars = source.char_indices().peekable();
        while let Some((index, character)) = chars.next() {
            if character == '\r' && matches!(chars.peek(), Some((_, '\n'))) {
                continue;
            }
            bytes.push(index);
            lf.push(character);
        }
        bytes.push(source.len());
        Self { lf, bytes }
    }

    /// Carácter del cuerpo LF que corresponde a un byte del texto real.
    pub(crate) fn char_at_byte(&self, byte: usize) -> usize {
        self.bytes.partition_point(|&start| start < byte)
    }

    /// Byte del texto real donde empieza el carácter `index` del cuerpo LF.
    pub(crate) fn byte_at_char(&self, index: usize) -> usize {
        self.bytes[index.min(self.bytes.len() - 1)]
    }
}

/// Para cada carácter del texto dibujado de un bloque, el byte de la fuente de
/// donde viene. El texto dibujado es casi siempre una subsecuencia de la
/// fuente (`**negrita**` → `negrita`, `[texto](url)` → `texto`): se recorre en
/// orden y cada carácter se busca hacia adelante. Si alguno no aparece (una
/// entidad, por ejemplo) se le asigna la posición actual sin avanzar.
pub(crate) fn rendered_source_map(text: &str, source: &str, base: usize) -> Vec<(usize, usize)> {
    let mut map = Vec::with_capacity(text.len() / 2 + 1);
    let mut cursor = 0;
    for (offset, character) in text.char_indices() {
        let found = source[cursor..]
            .char_indices()
            .take(4096)
            .find(|(_, candidate)| *candidate == character)
            .map(|(index, _)| cursor + index);
        match found {
            Some(at) => {
                map.push((offset, base + at));
                cursor = at + character.len_utf8();
            }
            None => map.push((offset, base + cursor)),
        }
    }
    // Centinela: el final del texto dibujado cae después del último carácter.
    map.push((text.len(), base + cursor));
    map
}

/// Byte de la fuente donde empieza lo dibujado desde `offset`.
pub(crate) fn source_start(map: &[(usize, usize)], offset: usize) -> usize {
    let index = map.partition_point(|&(rendered, _)| rendered < offset);
    map[index.min(map.len() - 1)].1
}

/// Byte de la fuente donde termina lo dibujado hasta `offset` (exclusivo).
pub(crate) fn source_end(map: &[(usize, usize)], text: &str, offset: usize) -> usize {
    if offset == 0 {
        return source_start(map, 0);
    }
    let previous = text[..offset.min(text.len())]
        .char_indices()
        .next_back()
        .map_or(0, |(index, _)| index);
    let index = map.partition_point(|&(rendered, _)| rendered < previous);
    let (_, source) = map[index.min(map.len() - 1)];
    let width = text[previous..].chars().next().map_or(0, char::len_utf8);
    source + width
}

/// Tramo del texto dibujado cuyos caracteres vienen de `[start, end)` de la
/// fuente, o `None` si ninguno.
pub(crate) fn rendered_range(
    map: &[(usize, usize)],
    start: usize,
    end: usize,
) -> Option<(usize, usize)> {
    let mut first = None;
    let mut last = None;
    for pair in map.windows(2) {
        let ((offset, source), (next_offset, _)) = (pair[0], pair[1]);
        if source >= start && source < end {
            first.get_or_insert(offset);
            last = Some(next_offset);
        }
    }
    Some((first?, last?))
}

/// Un tramo que se pinta: bytes del texto real, color (índice de la paleta) e
/// identificador de la anotación.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Highlight {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) color: usize,
    pub(crate) id: String,
}

/// Los colores elegidos con el selector libre se llevan como un número que
/// empieza en este valor y trae el RGB en sus 24 bits bajos. Los cinco de la
/// paleta son 0 a 4. En el archivo se guardan como `#rrggbb`.
pub(crate) const CUSTOM_BASE: usize = 1 << 24;

/// El número de un color libre.
pub(crate) const fn custom_color(rgb: (u8, u8, u8)) -> usize {
    CUSTOM_BASE | ((rgb.0 as usize) << 16) | ((rgb.1 as usize) << 8) | rgb.2 as usize
}

/// El RGB de un color libre, o `None` si es de la paleta.
pub(crate) fn custom_rgb(index: usize) -> Option<(u8, u8, u8)> {
    (index >= CUSTOM_BASE).then_some((
        ((index >> 16) & 0xFF) as u8,
        ((index >> 8) & 0xFF) as u8,
        (index & 0xFF) as u8,
    ))
}

/// Un número de color que se puede dibujar: los de la paleta no pasan de 4.
pub(crate) fn clamp_color(index: usize) -> usize {
    match custom_rgb(index) {
        Some(rgb) => custom_color(rgb),
        None => index.min(4),
    }
}

/// `#rrggbb` o `rrggbb` (mayúsculas o minúsculas) a RGB.
pub(crate) fn parse_hex_color(text: &str) -> Option<(u8, u8, u8)> {
    let digits = text.trim().strip_prefix('#').unwrap_or(text.trim());
    if digits.len() != 6 || !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    let value = u32::from_str_radix(digits, 16).ok()?;
    Some((
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,
    ))
}

/// `#rrggbb` en minúsculas.
pub(crate) fn hex_color(rgb: (u8, u8, u8)) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2)
}

/// El nombre de un color para los avisos.
pub(crate) fn color_name(index: usize) -> String {
    match custom_rgb(index) {
        Some(rgb) => format!("el color {}", hex_color(rgb).to_uppercase()),
        None => crate::theme::STUDY_COLOR_NAMES[index.min(4)].to_string(),
    }
}

/// Índice de un color en la paleta (o color libre), o `None` si es desconocido.
pub(crate) fn color_index(color: &Color) -> Option<usize> {
    if let Color::Otro(text) = color
        && text.starts_with('#')
    {
        return parse_hex_color(text).map(custom_color);
    }
    crate::theme::STUDY_COLOR_NAMES
        .iter()
        .position(|name| *name == color.texto())
}

pub(crate) fn color_for_index(index: usize) -> Color {
    if let Some(rgb) = custom_rgb(index) {
        return Color::Otro(hex_color(rgb));
    }
    match index {
        1 => Color::Verde,
        2 => Color::Azul,
        3 => Color::Rosa,
        4 => Color::Violeta,
        _ => Color::Amarillo,
    }
}

/// Las anotaciones del bloque, si se entiende y se puede modificar.
pub(crate) fn parse(block: Option<&str>) -> Result<Anotaciones, &'static str> {
    match block {
        None => Ok(Anotaciones::default()),
        Some(raw) => match anotaciones::leer(raw) {
            Lectura::Entendido(anotaciones) => Ok(anotaciones),
            Lectura::Inerte(anotaciones::Inerte::VersionNueva(_)) => Err(
                "Las anotaciones son de una versión más nueva de Visor MD: se conservan, pero no se pueden cambiar",
            ),
            Lectura::Inerte(anotaciones::Inerte::Limites) => Err(
                "El bloque de anotaciones es demasiado grande: se conserva, pero no se puede cambiar",
            ),
            Lectura::NoEsBloque => {
                Err("El bloque de anotaciones no se entiende: se conserva sin cambios")
            }
        },
    }
}

/// Los subrayados anclados en el texto, en bytes del texto real.
pub(crate) fn highlights(source: &str, anotaciones: &Anotaciones) -> Vec<Highlight> {
    let index = LfIndex::new(source);
    anotaciones
        .registros
        .iter()
        .filter(|registro| registro.tipo == Tipo::Subrayado)
        .filter_map(|registro| {
            let color = color_index(&registro.color)?;
            match anotaciones::anclar(&index.lf, registro) {
                Ancla::Anclada { inicio, fin } => Some(Highlight {
                    start: index.byte_at_char(inicio),
                    end: index.byte_at_char(fin),
                    color,
                    id: registro.id.clone(),
                }),
                _ => None,
            }
        })
        .collect()
}

/// Cita, contexto y posición de un tramo `[start, end)` del texto real.
fn quote(
    source: &str,
    start: usize,
    end: usize,
) -> Result<(u64, String, String, String), &'static str> {
    if start >= end || end > source.len() {
        return Err(
            "Seleccioná con el mouse el texto que querés subrayar y volvé a pulsar el marcador",
        );
    }
    let index = LfIndex::new(source);
    let (first, last) = (index.char_at_byte(start), index.char_at_byte(end));
    let chars: Vec<char> = index.lf.chars().collect();
    let cita: String = chars[first..last].iter().collect();
    if cita.trim().is_empty() {
        return Err(
            "Seleccioná con el mouse el texto que querés subrayar y volvé a pulsar el marcador",
        );
    }
    if cita.len() > MAX_CITA_BYTES {
        return Err("La selección es demasiado larga: subrayá en partes más cortas");
    }
    let antes: String = chars[first.saturating_sub(anotaciones::MAX_CONTEXTO)..first]
        .iter()
        .collect();
    let despues: String = chars[last..(last + anotaciones::MAX_CONTEXTO).min(chars.len())]
        .iter()
        .collect();
    Ok((first as u64, antes, cita, despues))
}

/// Subraya `[start, end)` con un color. Si el tramo cae sobre subrayados que ya
/// existen, les cambia el color en lugar de superponer otro: así «subrayar en
/// verde» sobre lo amarillo lo vuelve verde. Devuelve cuántos se cambiaron (0
/// si se creó uno nuevo).
pub(crate) fn highlight(
    anotaciones: &mut Anotaciones,
    source: &str,
    start: usize,
    end: usize,
    color: usize,
) -> Result<usize, &'static str> {
    let existing: Vec<String> = highlights(source, anotaciones)
        .into_iter()
        .filter(|item| item.start < end.max(start + 1) && start < item.end)
        .map(|item| item.id)
        .collect();
    if !existing.is_empty() {
        for registro in &mut anotaciones.registros {
            if existing.contains(&registro.id) {
                registro.color = color_for_index(color);
            }
        }
        return Ok(existing.len());
    }
    let (pos, antes, cita, despues) = quote(source, start, end)?;
    let id = anotaciones.nuevo_id();
    anotaciones.registros.push(Registro {
        id,
        tipo: Tipo::Subrayado,
        color: color_for_index(color),
        forma: None,
        pos: Some(pos),
        antes: Some(antes),
        cita: Some(cita),
        despues: Some(despues),
        nota: None,
        imagen: None,
        extras: Vec::new(),
    });
    Ok(0)
}

/// Quita los subrayados que tocan `[start, end)` (o que contienen el punto si
/// el tramo está vacío). Devuelve cuántos quitó.
pub(crate) fn remove_highlights(
    anotaciones: &mut Anotaciones,
    source: &str,
    start: usize,
    end: usize,
) -> usize {
    let touched: Vec<String> = highlights(source, anotaciones)
        .into_iter()
        .filter(|item| item.start < end.max(start + 1) && start < item.end)
        .map(|item| item.id)
        .collect();
    anotaciones
        .registros
        .retain(|registro| !touched.contains(&registro.id));
    touched.len()
}

/// La leyenda de un color, si la persona le dio un significado.
pub(crate) fn legend(anotaciones: &Anotaciones, color: usize) -> Option<&str> {
    anotaciones
        .registros
        .iter()
        .find(|registro| {
            registro.tipo == Tipo::Leyenda && color_index(&registro.color) == Some(color)
        })
        .and_then(|registro| registro.nota.as_deref())
        .filter(|nota| !nota.trim().is_empty())
}

/// Tope de la leyenda de un color.
pub(crate) const MAX_LEGEND_CHARS: usize = 80;

/// Le da (o le quita, si el texto queda vacío) un significado a un color.
pub(crate) fn set_legend(anotaciones: &mut Anotaciones, color: usize, text: &str) {
    let text: String = text.trim().chars().take(MAX_LEGEND_CHARS).collect();
    let existing = anotaciones.registros.iter().position(|registro| {
        registro.tipo == Tipo::Leyenda && color_index(&registro.color) == Some(color)
    });
    match (existing, text.is_empty()) {
        (Some(index), true) => {
            anotaciones.registros.remove(index);
        }
        (Some(index), false) => anotaciones.registros[index].nota = Some(text),
        (None, true) => {}
        (None, false) => {
            let id = anotaciones.nuevo_id();
            anotaciones.registros.push(Registro {
                id,
                tipo: Tipo::Leyenda,
                color: color_for_index(color),
                forma: None,
                pos: None,
                antes: None,
                cita: None,
                despues: None,
                nota: Some(text),
                imagen: None,
                extras: Vec::new(),
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Marcas de libro (14.6).
// ---------------------------------------------------------------------------

/// Caracteres de la línea que cita una marca.
pub(crate) const MARK_QUOTE_CHARS: usize = 48;
/// Colores distintos que se ven como puntos en una fila del índice.
pub(crate) const MAX_INDEX_DOTS: usize = 3;
/// Marcas que lista la ayuda de una fila del índice antes de «y N más».
pub(crate) const MAX_TOOLTIP_MARKS: usize = 5;

/// Una marca anclada: byte del texto real donde está, color, forma y lo que
/// la describe (su nota o, si no tiene, su cita).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Bookmark {
    pub(crate) start: usize,
    pub(crate) color: usize,
    pub(crate) shape: Forma,
    pub(crate) label: String,
    pub(crate) id: String,
    /// El tipo de la marca («Duda», «Pendiente», uno propio…), si tiene.
    pub(crate) kind: Option<String>,
    /// El texto que la persona le escribió, sin la cita de la línea.
    pub(crate) note: Option<String>,
}

/// Las marcas ancladas, ordenadas por posición.
pub(crate) fn bookmarks(source: &str, anotaciones: &Anotaciones) -> Vec<Bookmark> {
    let index = LfIndex::new(source);
    let mut marks: Vec<Bookmark> = anotaciones
        .registros
        .iter()
        .filter(|registro| registro.tipo == Tipo::Marca)
        .filter_map(|registro| {
            let color = color_index(&registro.color)?;
            let Ancla::Anclada { inicio, .. } = anotaciones::anclar(&index.lf, registro) else {
                return None;
            };
            let label = registro
                .nota
                .clone()
                .filter(|nota| !nota.trim().is_empty())
                .or_else(|| registro.cita.clone())
                .unwrap_or_default();
            Some(Bookmark {
                start: index.byte_at_char(inicio),
                color,
                shape: registro.forma.clone().unwrap_or(Forma::Barra),
                label,
                id: registro.id.clone(),
                kind: mark_kind_of(registro).map(str::to_string),
                note: registro.nota.clone().filter(|nota| !nota.trim().is_empty()),
            })
        })
        .collect();
    marks.sort_by_key(|mark| mark.start);
    marks
}

// ---------------------------------------------------------------------------
// Tipos de marca: una etiqueta («Duda», «Pendiente»…) que trae su color y su
// forma por defecto, y que después se puede filtrar y recorrer.
// ---------------------------------------------------------------------------

/// Clave del tipo dentro del registro. El lector del formato ya conserva las
/// claves que no conoce, así que una versión anterior la respeta sin dibujarla.
const KIND_KEY: &str = "categoria";
/// Tope del nombre de un tipo.
pub(crate) const MAX_KIND_NAME_CHARS: usize = 24;

/// Un tipo de marca con el color y la forma que se ponen al elegirlo.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MarkKind {
    pub(crate) name: String,
    pub(crate) color: usize,
    pub(crate) shape: Forma,
}

/// Formas de marca que se pueden elegir. Las cuatro primeras son las de
/// siempre; las demás llevan un signo dentro.
pub(crate) fn shape_catalog() -> [Forma; 9] {
    [
        Forma::Barra,
        Forma::Cinta,
        Forma::Punto,
        Forma::Bandera,
        Forma::Otro("exclamacion".to_string()),
        Forma::Otro("interrogacion".to_string()),
        Forma::Otro("casilla".to_string()),
        Forma::Otro("repetir".to_string()),
        Forma::Otro("lapiz".to_string()),
    ]
}

/// Los tipos que trae la aplicación.
pub(crate) fn default_mark_kinds() -> Vec<MarkKind> {
    let kind = |name: &str, color: usize, shape: &str| MarkKind {
        name: name.to_string(),
        color,
        shape: Forma::Otro(shape.to_string()),
    };
    vec![
        kind(
            "Importante",
            custom_color((0xE5, 0x48, 0x4D)),
            "exclamacion",
        ),
        kind("Duda", custom_color((0xF0, 0x8C, 0x3A)), "interrogacion"),
        MarkKind {
            name: "Pendiente".to_string(),
            color: 0,
            shape: Forma::Otro("casilla".to_string()),
        },
        MarkKind {
            name: "Repasar".to_string(),
            color: 4,
            shape: Forma::Otro("repetir".to_string()),
        },
        MarkKind {
            name: "Corregir".to_string(),
            color: 3,
            shape: Forma::Otro("lapiz".to_string()),
        },
    ]
}

/// Nombre de un tipo, limpio: sin saltos ni `|`, recortado y de largo acotado.
pub(crate) fn clean_kind_name(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_control() && *c != '|')
        .collect::<String>()
        .trim()
        .chars()
        .take(MAX_KIND_NAME_CHARS)
        .collect()
}

/// El tipo de una marca, si tiene.
pub(crate) fn mark_kind_of(registro: &Registro) -> Option<&str> {
    registro
        .extras
        .iter()
        .find_map(|(key, value)| match (key.as_str(), value) {
            (KIND_KEY, Valor::Texto(name)) if !name.trim().is_empty() => Some(name.as_str()),
            _ => None,
        })
}

/// Pone o quita el tipo de una marca, sin tocar su color ni su forma.
pub(crate) fn set_mark_kind(registro: &mut Registro, kind: Option<&str>) {
    registro.extras.retain(|(key, _)| key != KIND_KEY);
    if let Some(name) = kind.map(clean_kind_name).filter(|name| !name.is_empty()) {
        registro
            .extras
            .push((KIND_KEY.to_string(), Valor::Texto(name)));
    }
}

/// Le da a una marca un tipo con su color y su forma por defecto.
pub(crate) fn apply_mark_kind(registro: &mut Registro, kind: &MarkKind) {
    set_mark_kind(registro, Some(&kind.name));
    registro.color = color_for_index(kind.color);
    registro.forma = Some(kind.shape.clone());
}

/// Cuántas marcas hay de cada tipo (en el orden de `kinds`) y cuántas no
/// tienen tipo. Los tipos que no están en `kinds` (de otra computadora) se
/// suman al final, con su nombre.
pub(crate) fn kind_counts(marks: &[Bookmark], kinds: &[MarkKind]) -> (Vec<(String, usize)>, usize) {
    let mut counts: Vec<(String, usize)> =
        kinds.iter().map(|kind| (kind.name.clone(), 0)).collect();
    let mut untyped = 0;
    for mark in marks {
        match &mark.kind {
            None => untyped += 1,
            Some(name) => match counts.iter_mut().find(|(known, _)| known == name) {
                Some((_, count)) => *count += 1,
                None => counts.push((name.clone(), 1)),
            },
        }
    }
    (counts, untyped)
}

/// Las marcas que pasan el filtro (sin filtro, todas), en orden de posición.
/// Un filtro vacío no deja ver ninguna; `None` en el nombre es «sin tipo».
pub(crate) fn filter_marks<'a>(
    marks: &'a [Bookmark],
    active: &[Option<String>],
) -> Vec<&'a Bookmark> {
    marks
        .iter()
        .filter(|mark| active.is_empty() || active.contains(&mark.kind))
        .collect()
}

/// La marca siguiente (o anterior) a `from` dentro de las filtradas, dando la
/// vuelta al llegar al final. `None` si no hay ninguna.
pub(crate) fn step_mark<'a>(
    filtered: &[&'a Bookmark],
    from: usize,
    backwards: bool,
) -> Option<&'a Bookmark> {
    if filtered.is_empty() {
        return None;
    }
    if backwards {
        filtered
            .iter()
            .rev()
            .find(|mark| mark.start < from)
            .or(filtered.last())
            .copied()
    } else {
        filtered
            .iter()
            .find(|mark| mark.start > from)
            .or(filtered.first())
            .copied()
    }
}

/// Comienzo de la línea del texto real que contiene `byte`.
fn line_start(source: &str, byte: usize) -> usize {
    source[..byte.min(source.len())]
        .rfind('\n')
        .map_or(0, |index| index + 1)
}

/// La marca anclada en la línea de `byte`, si hay.
fn bookmark_on_line(source: &str, anotaciones: &Anotaciones, byte: usize) -> Option<String> {
    let start = line_start(source, byte);
    let end = source[start..]
        .find('\n')
        .map_or(source.len(), |index| start + index);
    bookmarks(source, anotaciones)
        .into_iter()
        .find(|mark| (start..=end).contains(&mark.start))
        .map(|mark| mark.id)
}

/// Pone una marca en la línea de `byte` o, si ya tiene una, la quita.
/// Devuelve `true` si la puso.
pub(crate) fn toggle_bookmark(
    anotaciones: &mut Anotaciones,
    source: &str,
    byte: usize,
    color: usize,
    shape: Forma,
) -> Result<bool, &'static str> {
    if let Some(id) = bookmark_on_line(source, anotaciones, byte) {
        anotaciones.registros.retain(|registro| registro.id != id);
        return Ok(false);
    }
    let start = line_start(source, byte);
    let line_end = source[start..]
        .find('\n')
        .map_or(source.len(), |index| start + index);
    let line = source[start..line_end].trim_end_matches('\r');
    let head: String = line.chars().take(MARK_QUOTE_CHARS).collect();
    let head = head.trim_end();
    if head.trim().is_empty() {
        return Err("Poné la marca en una línea con texto");
    }
    let (pos, antes, cita, despues) = quote(source, start, start + head.len())?;
    let id = anotaciones.nuevo_id();
    anotaciones.registros.push(Registro {
        id,
        tipo: Tipo::Marca,
        color: color_for_index(color),
        forma: Some(shape),
        pos: Some(pos),
        antes: Some(antes),
        cita: Some(cita),
        despues: Some(despues),
        nota: None,
        imagen: None,
        extras: Vec::new(),
    });
    Ok(true)
}

/// El identificador de la marca de la línea de `byte`, si hay.
pub(crate) fn bookmark_id_on_line(
    source: &str,
    anotaciones: &Anotaciones,
    byte: usize,
) -> Option<String> {
    bookmark_on_line(source, anotaciones, byte)
}

/// Si la línea de `byte` ya tiene marca, no hace nada; si no, la pone.
pub(crate) fn ensure_bookmark(
    anotaciones: &mut Anotaciones,
    source: &str,
    byte: usize,
    color: usize,
    shape: Forma,
) -> Result<(), &'static str> {
    if bookmark_on_line(source, anotaciones, byte).is_some() {
        return Ok(());
    }
    toggle_bookmark(anotaciones, source, byte, color, shape).map(|_| ())
}

/// Quita una marca por su identificador. Devuelve `true` si existía.
pub(crate) fn remove_bookmark(anotaciones: &mut Anotaciones, id: &str) -> bool {
    let before = anotaciones.registros.len();
    anotaciones
        .registros
        .retain(|registro| !(registro.tipo == Tipo::Marca && registro.id == id));
    anotaciones.registros.len() != before
}

/// Aplica un cambio a la marca de la línea de `byte`. Error si no hay.
pub(crate) fn change_bookmark(
    anotaciones: &mut Anotaciones,
    source: &str,
    byte: usize,
    change: impl FnOnce(&mut Registro),
) -> Result<(), &'static str> {
    let id = bookmark_on_line(source, anotaciones, byte).ok_or("Esta línea no tiene marca")?;
    let registro = anotaciones
        .registros
        .iter_mut()
        .find(|registro| registro.id == id)
        .ok_or("Esta línea no tiene marca")?;
    change(registro);
    Ok(())
}

/// Lo que el índice muestra de las marcas de una sección.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SectionMarks {
    /// Colores distintos, en el orden de la paleta, como máximo tres.
    pub(crate) colors: Vec<usize>,
    /// Cuántos colores distintos más hay («+N»).
    pub(crate) more: usize,
    /// Todas las marcas de la sección: color, byte y descripción.
    pub(crate) marks: Vec<(usize, usize, String)>,
}

impl SectionMarks {
    pub(crate) fn is_empty(&self) -> bool {
        self.marks.is_empty()
    }

    /// La primera marca de un color en la sección.
    pub(crate) fn first_of_color(&self, color: usize) -> Option<usize> {
        self.marks
            .iter()
            .find(|(mark_color, _, _)| *mark_color == color)
            .map(|(_, start, _)| *start)
    }

    /// Texto de la ayuda: las primeras marcas con su descripción y «y N más».
    pub(crate) fn tooltip(&self) -> String {
        let mut lines: Vec<String> = self
            .marks
            .iter()
            .take(MAX_TOOLTIP_MARKS)
            .map(|(color, _, label)| {
                let label: String = label.chars().take(60).collect();
                // «Duda: …» si la marca tiene tipo (el rótulo ya lo trae);
                // si no, el nombre de su color.
                match label.split_once('\u{1f}') {
                    Some((kind, text)) => format!("{kind}: {text}"),
                    None => format!("{}: {label}", color_name(*color)),
                }
            })
            .collect();
        if self.marks.len() > MAX_TOOLTIP_MARKS {
            lines.push(format!("y {} más", self.marks.len() - MAX_TOOLTIP_MARKS));
        }
        lines.join("\n")
    }
}

/// Reparte las marcas entre las secciones del índice. `heading_starts` son
/// los bytes donde empieza cada título de la lista, en orden. Una marca va a
/// la sección del último título anterior a ella; las que están antes del
/// primer título van a la primera fila (el comienzo del índice).
pub(crate) fn sections(heading_starts: &[usize], marks: &[Bookmark]) -> Vec<SectionMarks> {
    let mut sections = vec![SectionMarks::default(); heading_starts.len()];
    if sections.is_empty() {
        return sections;
    }
    for mark in marks {
        let index = heading_starts
            .partition_point(|&start| start <= mark.start)
            .saturating_sub(1);
        // El tipo viaja delante del texto, separado por un carácter que no se
        // escribe, para que la ayuda lo muestre en lugar del color.
        let label = match &mark.kind {
            Some(kind) => format!("{kind}\u{1f}{}", mark.label),
            None => mark.label.clone(),
        };
        sections[index].marks.push((mark.color, mark.start, label));
    }
    for section in &mut sections {
        let mut colors: Vec<usize> = section.marks.iter().map(|(color, _, _)| *color).collect();
        colors.sort_unstable();
        colors.dedup();
        section.more = colors.len().saturating_sub(MAX_INDEX_DOTS);
        colors.truncate(MAX_INDEX_DOTS);
        section.colors = colors;
    }
    sections
}

// ---------------------------------------------------------------------------
// Edición y anotaciones sin lugar (14.7).
// ---------------------------------------------------------------------------

/// Anotaciones con cita que ya no se encuentran en el texto.
pub(crate) fn orphans<'a>(source: &str, anotaciones: &'a Anotaciones) -> Vec<&'a Registro> {
    let index = LfIndex::new(source);
    anotaciones
        .registros
        .iter()
        .filter(|registro| matches!(registro.tipo, Tipo::Subrayado | Tipo::Marca))
        .filter(|registro| anotaciones::anclar(&index.lf, registro) == Ancla::Huerfana)
        .collect()
}

/// Al guardar: cada anotación anclada recalcula su posición y su contexto con
/// el texto de ahora (la cita no cambia). Las que no tienen lugar se guardan
/// tal cual, con su última cita. Devuelve cuántas cambiaron.
pub(crate) fn refresh_anchors(anotaciones: &mut Anotaciones, source: &str) -> usize {
    let index = LfIndex::new(source);
    let chars: Vec<char> = index.lf.chars().collect();
    let mut changed = 0;
    for registro in &mut anotaciones.registros {
        if !matches!(registro.tipo, Tipo::Subrayado | Tipo::Marca) {
            continue;
        }
        let Ancla::Anclada { inicio, fin } = anotaciones::anclar(&index.lf, registro) else {
            continue;
        };
        let antes: String = chars[inicio.saturating_sub(anotaciones::MAX_CONTEXTO)..inicio]
            .iter()
            .collect();
        let despues: String = chars[fin..(fin + anotaciones::MAX_CONTEXTO).min(chars.len())]
            .iter()
            .collect();
        let cita: String = chars[inicio..fin].iter().collect();
        let nuevo = (Some(inicio as u64), Some(antes), Some(cita), Some(despues));
        let viejo = (
            registro.pos,
            registro.antes.clone(),
            registro.cita.clone(),
            registro.despues.clone(),
        );
        if nuevo != viejo {
            (
                registro.pos,
                registro.antes,
                registro.cita,
                registro.despues,
            ) = nuevo;
            changed += 1;
        }
    }
    changed
}

/// Quita las anotaciones sin lugar. Devuelve cuántas.
pub(crate) fn remove_orphans(anotaciones: &mut Anotaciones, source: &str) -> usize {
    let ids: Vec<String> = orphans(source, anotaciones)
        .into_iter()
        .map(|registro| registro.id.clone())
        .collect();
    anotaciones
        .registros
        .retain(|registro| !ids.contains(&registro.id));
    ids.len()
}

/// Las citas de las anotaciones sin lugar, una por línea, para copiarlas.
pub(crate) fn orphan_quotes(source: &str, anotaciones: &Anotaciones) -> String {
    orphans(source, anotaciones)
        .into_iter()
        .map(|registro| {
            let cita = registro.cita.as_deref().unwrap_or("").replace('\n', " ");
            match registro
                .nota
                .as_deref()
                .filter(|nota| !nota.trim().is_empty())
            {
                Some(nota) => format!("{cita} — {nota}"),
                None => cita,
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn el_indice_lf_traduce_crlf_y_caracteres_anchos() {
        let source = "á\r\nb😀c\r\n";
        let index = LfIndex::new(source);
        assert_eq!(index.lf, "á\nb😀c\n");
        // «b» está en el byte 4 del texto real y es el carácter 2 del cuerpo LF.
        assert_eq!(index.char_at_byte(4), 2);
        assert_eq!(index.byte_at_char(2), 4);
        assert_eq!(index.byte_at_char(4), 9);
        assert_eq!(index.byte_at_char(99), source.len());
    }

    #[test]
    fn el_mapa_de_dibujo_salta_la_sintaxis_markdown() {
        let source = "Es **muy** [claro](http://x.y) hoy";
        let text = "Es muy claro hoy";
        let map = rendered_source_map(text, source, 0);
        // «muy» empieza en el byte 3 del dibujo y en el 5 de la fuente.
        assert_eq!(source_start(&map, 3), 5);
        assert_eq!(source_end(&map, text, 6), 8);
        assert_eq!(
            &source[source_start(&map, 3)..source_end(&map, text, 6)],
            "muy"
        );
        // Y de vuelta: la fuente de «claro» pinta «claro».
        let start = source.find("claro").unwrap();
        assert_eq!(rendered_range(&map, start, start + 5), Some((7, 12)));
    }

    #[test]
    fn subrayar_crea_la_cita_con_contexto_y_posicion_lf() {
        let source = "# Tema\r\n\r\nLa mitocondria produce ATP.\r\n";
        let start = source.find("mitocondria").unwrap();
        let mut anotaciones = Anotaciones::default();
        assert_eq!(
            highlight(&mut anotaciones, source, start, start + 11, 2),
            Ok(0)
        );
        let registro = &anotaciones.registros[0];
        assert_eq!(registro.cita.as_deref(), Some("mitocondria"));
        assert_eq!(registro.antes.as_deref(), Some("# Tema\n\nLa "));
        assert_eq!(registro.pos, Some(11));
        assert_eq!(registro.color, Color::Azul);
        let pintados = highlights(source, &anotaciones);
        assert_eq!(pintados.len(), 1);
        assert_eq!(&source[pintados[0].start..pintados[0].end], "mitocondria");
    }

    #[test]
    fn cambiar_el_color_no_mueve_el_ancla() {
        let source = "uno dos tres";
        let mut anotaciones = Anotaciones::default();
        highlight(&mut anotaciones, source, 4, 7, 0).unwrap();
        let antes = anotaciones.registros[0].clone();
        // Subrayar en rosa con el cursor dentro cambia el color del existente.
        assert_eq!(highlight(&mut anotaciones, source, 5, 5, 3), Ok(1));
        assert_eq!(anotaciones.registros.len(), 1);
        let despues = &anotaciones.registros[0];
        assert_eq!(despues.color, Color::Rosa);
        assert_eq!((despues.pos, &despues.cita), (antes.pos, &antes.cita));
        assert_eq!(remove_highlights(&mut anotaciones, source, 6, 6), 1);
        assert!(anotaciones.registros.is_empty());
    }

    #[test]
    fn la_seleccion_vacia_o_enorme_no_subraya() {
        let mut anotaciones = Anotaciones::default();
        assert!(highlight(&mut anotaciones, "abc", 1, 1, 0).is_err());
        assert!(highlight(&mut anotaciones, "a   b", 1, 4, 0).is_err());
        let largo = "x".repeat(MAX_CITA_BYTES + 1);
        assert!(highlight(&mut anotaciones, &largo, 0, largo.len(), 0).is_err());
        assert!(anotaciones.registros.is_empty());
    }

    #[test]
    fn la_leyenda_se_crea_cambia_y_se_quita() {
        let mut anotaciones = Anotaciones::default();
        set_legend(&mut anotaciones, 1, "  definiciones  ");
        assert_eq!(legend(&anotaciones, 1), Some("definiciones"));
        assert_eq!(legend(&anotaciones, 0), None);
        set_legend(&mut anotaciones, 1, "fórmulas");
        assert_eq!(legend(&anotaciones, 1), Some("fórmulas"));
        assert_eq!(anotaciones.registros.len(), 1);
        set_legend(&mut anotaciones, 1, "   ");
        assert!(anotaciones.registros.is_empty());
    }

    #[test]
    fn un_bloque_de_version_nueva_no_se_puede_cambiar() {
        assert!(parse(Some("<!-- visor-md:anotaciones v2\n-->")).is_err());
        assert_eq!(parse(None), Ok(Anotaciones::default()));
    }

    const GUIA: &str = "Intro suelta\n\n# Uno\n\ntexto uno\n\notra línea\n\n# Dos\n\ntexto dos\n";

    fn titulos() -> Vec<usize> {
        vec![GUIA.find("# Uno").unwrap(), GUIA.find("# Dos").unwrap()]
    }

    fn marcar(anotaciones: &mut Anotaciones, cita: &str, color: usize) {
        let byte = GUIA.find(cita).unwrap();
        assert_eq!(
            toggle_bookmark(anotaciones, GUIA, byte, color, Forma::Barra),
            Ok(true)
        );
    }

    #[test]
    fn marca_suma_un_punto_al_titulo_de_su_seccion() {
        let mut anotaciones = Anotaciones::default();
        marcar(&mut anotaciones, "texto dos", 2);
        let secciones = sections(&titulos(), &bookmarks(GUIA, &anotaciones));
        assert!(secciones[0].is_empty());
        assert_eq!(secciones[1].colors, vec![2]);
        assert_eq!(secciones[1].more, 0);
        // La marca cita el comienzo de su línea.
        assert_eq!(anotaciones.registros[0].cita.as_deref(), Some("texto dos"));
        // Poner otra vez en la misma línea la quita.
        let byte = GUIA.find("dos\n").unwrap();
        assert_eq!(
            toggle_bookmark(&mut anotaciones, GUIA, byte, 0, Forma::Barra),
            Ok(false)
        );
        assert!(anotaciones.registros.is_empty());
    }

    #[test]
    fn muchas_marcas_no_cambian_la_fila_del_indice() {
        // Decenas de marcas del mismo color en una sección: un solo punto.
        let mut texto = String::from("# Tema\n\n");
        for numero in 0..40 {
            texto.push_str(&format!("línea {numero}\n"));
        }
        let mut anotaciones = Anotaciones::default();
        for numero in 0..40 {
            let byte = texto.find(&format!("línea {numero}\n")).unwrap();
            toggle_bookmark(&mut anotaciones, &texto, byte, 1, Forma::Punto).unwrap();
        }
        let secciones = sections(&[0], &bookmarks(&texto, &anotaciones));
        assert_eq!(secciones[0].marks.len(), 40);
        assert_eq!(secciones[0].colors, vec![1]);
        assert_eq!(secciones[0].more, 0);
    }

    #[test]
    fn mas_de_tres_colores_muestran_mas_n() {
        let mut texto = String::from("# Tema\n\n");
        for color in 0..5 {
            texto.push_str(&format!("color {color}\n"));
        }
        let mut anotaciones = Anotaciones::default();
        for color in (0..5).rev() {
            let byte = texto.find(&format!("color {color}\n")).unwrap();
            toggle_bookmark(&mut anotaciones, &texto, byte, color, Forma::Barra).unwrap();
        }
        let secciones = sections(&[0], &bookmarks(&texto, &anotaciones));
        // En el orden de la paleta, aunque se marcaron al revés.
        assert_eq!(secciones[0].colors, vec![0, 1, 2]);
        assert_eq!(secciones[0].more, 2);
        assert_eq!(secciones[0].first_of_color(3), texto.find("color 3"));
    }

    #[test]
    fn la_ayuda_lista_las_marcas_de_la_seccion() {
        let mut texto = String::from("# Tema\n\n");
        for numero in 0..7 {
            texto.push_str(&format!("punto {numero}\n"));
        }
        let mut anotaciones = Anotaciones::default();
        for numero in 0..7 {
            let byte = texto.find(&format!("punto {numero}\n")).unwrap();
            toggle_bookmark(&mut anotaciones, &texto, byte, 0, Forma::Barra).unwrap();
        }
        // Una nota reemplaza a la cita en la ayuda.
        let byte = texto.find("punto 0").unwrap();
        change_bookmark(&mut anotaciones, &texto, byte, |registro| {
            registro.nota = Some("repasar".to_string());
        })
        .unwrap();
        let ayuda = sections(&[0], &bookmarks(&texto, &anotaciones))[0].tooltip();
        let lineas: Vec<&str> = ayuda.lines().collect();
        assert_eq!(lineas[0], "amarillo: repasar");
        assert_eq!(lineas[1], "amarillo: punto 1");
        assert_eq!(lineas.len(), MAX_TOOLTIP_MARKS + 1);
        assert_eq!(lineas[MAX_TOOLTIP_MARKS], "y 2 más");
    }

    #[test]
    fn marca_antes_del_primer_titulo_va_al_comienzo_del_indice() {
        let mut anotaciones = Anotaciones::default();
        marcar(&mut anotaciones, "Intro suelta", 4);
        let secciones = sections(&titulos(), &bookmarks(GUIA, &anotaciones));
        assert_eq!(secciones[0].colors, vec![4]);
        assert!(secciones[1].is_empty());
        assert!(sections(&[], &bookmarks(GUIA, &anotaciones)).is_empty());
    }

    #[test]
    fn la_marca_cambia_de_forma_y_no_se_pone_en_una_linea_vacia() {
        let mut anotaciones = Anotaciones::default();
        assert!(
            toggle_bookmark(
                &mut anotaciones,
                GUIA,
                GUIA.find("\n\n# Uno").unwrap() + 1,
                0,
                Forma::Barra
            )
            .is_err()
        );
        marcar(&mut anotaciones, "texto uno", 0);
        let byte = GUIA.find("texto uno").unwrap();
        change_bookmark(&mut anotaciones, GUIA, byte, |registro| {
            registro.forma = Some(Forma::Cinta);
        })
        .unwrap();
        assert_eq!(bookmarks(GUIA, &anotaciones)[0].shape, Forma::Cinta);
        assert!(change_bookmark(&mut anotaciones, GUIA, 0, |_| {}).is_err());
    }

    #[test]
    fn editar_antes_del_subrayado_lo_desplaza() {
        let original = "uno dos tres\ncuatro";
        let mut anotaciones = Anotaciones::default();
        highlight(&mut anotaciones, original, 4, 7, 0).unwrap();
        let editado = "INSERTADO uno dos tres\ncuatro";
        let pintados = highlights(editado, &anotaciones);
        assert_eq!(&editado[pintados[0].start..pintados[0].end], "dos");
        // Al guardar, la posición y el contexto se actualizan; la cita no.
        assert_eq!(refresh_anchors(&mut anotaciones, editado), 1);
        let registro = &anotaciones.registros[0];
        assert_eq!(registro.pos, Some(14));
        assert_eq!(registro.antes.as_deref(), Some("INSERTADO uno "));
        assert_eq!(registro.cita.as_deref(), Some("dos"));
        assert_eq!(refresh_anchors(&mut anotaciones, editado), 0);
    }

    #[test]
    fn borrar_el_texto_anclado_lo_deja_huerfano() {
        let original = "uno dos tres";
        let mut anotaciones = Anotaciones::default();
        highlight(&mut anotaciones, original, 4, 7, 2).unwrap();
        let sin_texto = "uno tres";
        assert!(highlights(sin_texto, &anotaciones).is_empty());
        assert_eq!(orphans(sin_texto, &anotaciones).len(), 1);
        // Guardar no la borra ni la cambia.
        let antes = anotaciones.clone();
        assert_eq!(refresh_anchors(&mut anotaciones, sin_texto), 0);
        assert_eq!(anotaciones, antes);
        assert_eq!(orphan_quotes(sin_texto, &anotaciones), "dos");
        assert_eq!(remove_orphans(&mut anotaciones, sin_texto), 1);
        assert!(anotaciones.registros.is_empty());
    }

    #[test]
    fn marcar_una_casilla_dentro_del_subrayado_no_lo_pierde() {
        let original = "# Tareas

- [ ] uno
- [ ] dos
- [ ] tres

Fin del texto
";
        let inicio = original.find("uno").unwrap();
        let fin = original.find("tres").unwrap() + "tres".len();
        let mut anotaciones = Anotaciones::default();
        highlight(&mut anotaciones, original, inicio, fin, 0).unwrap();
        let marcada = original.replace("- [ ] dos", "- [x] dos");
        let pintados = highlights(&marcada, &anotaciones);
        assert_eq!(pintados.len(), 1, "el subrayado se perdió");
        assert_eq!(
            &marcada[pintados[0].start..pintados[0].end],
            "uno
- [x] dos
- [ ] tres"
        );
        assert!(orphans(&marcada, &anotaciones).is_empty());
        // Al desmarcar, vuelve a quedar como al principio.
        assert_eq!(refresh_anchors(&mut anotaciones, &marcada), 1);
        let desmarcada = marcada.replace("- [x] dos", "- [ ] dos");
        let pintados = highlights(&desmarcada, &anotaciones);
        assert_eq!(
            &desmarcada[pintados[0].start..pintados[0].end],
            &original[inicio..fin]
        );
    }

    #[test]
    fn escribir_dentro_del_subrayado_lo_ensancha_sin_perderlo() {
        let original = "Primera frase importante aquí. Segunda frase.";
        let inicio = original.find("frase importante").unwrap();
        let fin = inicio + "frase importante".len();
        let mut anotaciones = Anotaciones::default();
        highlight(&mut anotaciones, original, inicio, fin, 1).unwrap();
        let editado = "Primera frase muy importante aquí. Segunda frase.";
        let pintados = highlights(editado, &anotaciones);
        assert_eq!(pintados.len(), 1);
        assert_eq!(
            &editado[pintados[0].start..pintados[0].end],
            "frase muy importante"
        );
    }

    #[test]
    fn un_contexto_corto_no_inventa_anclas() {
        let original = "a b c";
        let mut anotaciones = Anotaciones::default();
        highlight(&mut anotaciones, original, 2, 3, 0).unwrap();
        assert!(highlights("a x c", &anotaciones).is_empty());
    }

    fn marcar_con_tipo(anotaciones: &mut Anotaciones, texto: &str, linea: &str, tipo: usize) {
        let byte = texto.find(linea).unwrap();
        toggle_bookmark(anotaciones, texto, byte, 0, Forma::Barra).unwrap();
        let tipos = default_mark_kinds();
        change_bookmark(anotaciones, texto, byte, |registro| {
            apply_mark_kind(registro, &tipos[tipo]);
        })
        .unwrap();
    }

    #[test]
    fn la_ayuda_del_indice_nombra_el_tipo_o_el_color_y_no_falla_con_colores_libres() {
        let texto = "# Uno\n\nprimera linea\n\nsegunda linea\n\ntercera linea\n";
        let mut anotaciones = Anotaciones::default();
        marcar_con_tipo(&mut anotaciones, texto, "primera", 1);
        let byte = texto.find("segunda").unwrap();
        toggle_bookmark(
            &mut anotaciones,
            texto,
            byte,
            custom_color((0x12, 0x34, 0x56)),
            Forma::Barra,
        )
        .unwrap();
        let byte = texto.find("tercera").unwrap();
        toggle_bookmark(&mut anotaciones, texto, byte, 2, Forma::Barra).unwrap();
        let marcas = bookmarks(texto, &anotaciones);
        let secciones = sections(&[0], &marcas);
        let ayuda = secciones[0].tooltip();
        assert!(ayuda.contains("Duda: primera linea"), "{ayuda}");
        assert!(ayuda.contains("el color #123456: segunda linea"), "{ayuda}");
        assert!(ayuda.contains("azul: tercera linea"), "{ayuda}");
        assert!(!ayuda.contains('\u{1f}'));
        // Tres colores distintos (uno libre) caben en los puntos del índice.
        assert_eq!(secciones[0].colors.len(), 3);
    }

    #[test]
    fn el_tipo_de_una_marca_trae_color_y_forma_y_se_guarda_en_el_archivo() {
        let texto = "uno\ndos\ntres\n";
        let mut anotaciones = Anotaciones::default();
        marcar_con_tipo(&mut anotaciones, texto, "dos", 1);
        let tipos = default_mark_kinds();
        let registro = &anotaciones.registros[0];
        assert_eq!(mark_kind_of(registro), Some("Duda"));
        assert_eq!(color_index(&registro.color), Some(tipos[1].color));
        assert_eq!(registro.forma, Some(tipos[1].shape.clone()));
        // Sale en el archivo y vuelve igual.
        let bloque = anotaciones::escribir(&anotaciones, "\n").unwrap();
        assert!(bloque.contains("\"categoria\":\"Duda\""), "{bloque}");
        let Lectura::Entendido(leidas) = anotaciones::leer(&bloque) else {
            panic!("el bloque no se entendió");
        };
        assert_eq!(mark_kind_of(&leidas.registros[0]), Some("Duda"));
        assert_eq!(anotaciones::escribir(&leidas, "\n").unwrap(), bloque);
        // El marcador del documento lleva el tipo.
        let marcas = bookmarks(texto, &anotaciones);
        assert_eq!(marcas[0].kind.as_deref(), Some("Duda"));
        // Quitarle el tipo no cambia color ni forma.
        let mut sin = anotaciones.clone();
        set_mark_kind(&mut sin.registros[0], None);
        assert_eq!(mark_kind_of(&sin.registros[0]), None);
        assert_eq!(sin.registros[0].color, anotaciones.registros[0].color);
    }

    #[test]
    fn los_tipos_por_defecto_son_distintos_y_el_nombre_se_limpia() {
        let tipos = default_mark_kinds();
        let nombres: Vec<_> = tipos.iter().map(|tipo| tipo.name.as_str()).collect();
        assert_eq!(
            nombres,
            ["Importante", "Duda", "Pendiente", "Repasar", "Corregir"]
        );
        for (i, a) in tipos.iter().enumerate() {
            for b in &tipos[i + 1..] {
                assert_ne!(a.color, b.color, "{} y {} comparten color", a.name, b.name);
                assert_ne!(a.shape, b.shape, "{} y {} comparten forma", a.name, b.name);
            }
            assert!(shape_catalog().contains(&a.shape));
        }
        assert_eq!(
            clean_kind_name("  Para | revisar\n hoy  "),
            "Para  revisar hoy"
        );
        assert_eq!(
            clean_kind_name(&"x".repeat(80)).chars().count(),
            MAX_KIND_NAME_CHARS
        );
        assert_eq!(clean_kind_name("|\n"), "");
    }

    #[test]
    fn contar_filtrar_y_recorrer_marcas_por_tipo() {
        let texto = "a\nb\nc\nd\ne\n";
        let mut anotaciones = Anotaciones::default();
        marcar_con_tipo(&mut anotaciones, texto, "a", 1);
        marcar_con_tipo(&mut anotaciones, texto, "c", 1);
        marcar_con_tipo(&mut anotaciones, texto, "d", 2);
        toggle_bookmark(
            &mut anotaciones,
            texto,
            texto.find('e').unwrap(),
            0,
            Forma::Barra,
        )
        .unwrap();
        // Un tipo de otra computadora se conserva con su nombre.
        marcar_con_tipo(&mut anotaciones, texto, "b", 0);
        set_mark_kind(&mut anotaciones.registros[4], Some("Del trabajo"));
        let marcas = bookmarks(texto, &anotaciones);
        let tipos = default_mark_kinds();
        let (cuentas, sin_tipo) = kind_counts(&marcas, &tipos);
        assert_eq!(sin_tipo, 1);
        let por_nombre = |nombre: &str| cuentas.iter().find(|(n, _)| n == nombre).map(|(_, c)| *c);
        assert_eq!(por_nombre("Duda"), Some(2));
        assert_eq!(por_nombre("Pendiente"), Some(1));
        assert_eq!(por_nombre("Importante"), Some(0));
        assert_eq!(por_nombre("Del trabajo"), Some(1));
        // Filtrar: sin filtro, todas; con uno, solo ese; «sin tipo» es None.
        assert_eq!(filter_marks(&marcas, &[]).len(), 5);
        let duda = [Some("Duda".to_string())];
        let solo_duda = filter_marks(&marcas, &duda);
        assert_eq!(solo_duda.len(), 2);
        let sin = [None];
        assert_eq!(filter_marks(&marcas, &sin).len(), 1);
        let dos = [Some("Duda".to_string()), None];
        assert_eq!(filter_marks(&marcas, &dos).len(), 3);
        // Recorrer: siguiente y anterior, con vuelta.
        let primera = solo_duda[0].start;
        let segunda = solo_duda[1].start;
        assert_eq!(
            step_mark(&solo_duda, primera, false).unwrap().start,
            segunda
        );
        assert_eq!(
            step_mark(&solo_duda, segunda, false).unwrap().start,
            primera
        );
        assert_eq!(step_mark(&solo_duda, segunda, true).unwrap().start, primera);
        assert_eq!(step_mark(&solo_duda, primera, true).unwrap().start, segunda);
        assert!(step_mark(&[], 0, false).is_none());
    }

    #[test]
    fn un_color_libre_se_guarda_como_hex_y_vuelve_igual() {
        let rojizo = custom_color((0xE0, 0x30, 0x20));
        assert!(rojizo >= CUSTOM_BASE);
        assert_eq!(custom_rgb(rojizo), Some((0xE0, 0x30, 0x20)));
        assert_eq!(custom_rgb(3), None);
        let color = color_for_index(rojizo);
        assert_eq!(color, Color::Otro("#e03020".to_string()));
        assert_eq!(color_index(&color), Some(rojizo));
        // Los de la paleta no cambian.
        for indice in 0..5 {
            assert_eq!(color_index(&color_for_index(indice)), Some(indice));
        }
        // Un color mal escrito no se dibuja, pero tampoco rompe nada.
        assert_eq!(color_index(&Color::Otro("#12345".to_string())), None);
        assert_eq!(color_index(&Color::Otro("#zzzzzz".to_string())), None);
        assert_eq!(parse_hex_color(" #FFCC00 "), Some((255, 204, 0)));
        assert_eq!(parse_hex_color("ffcc0"), None);
        assert_eq!(clamp_color(99), 4);
        assert_eq!(clamp_color(rojizo), rojizo);
        assert_eq!(color_name(1), "verde");
        assert_eq!(color_name(rojizo), "el color #E03020");
    }

    #[test]
    fn subrayar_con_un_color_libre_lo_guarda_y_lo_dibuja_en_su_tramo() {
        let texto = "uno dos tres";
        let mut anotaciones = Anotaciones::default();
        let verde_lima = custom_color((0x99, 0xEE, 0x44));
        highlight(&mut anotaciones, texto, 4, 7, verde_lima).unwrap();
        assert_eq!(
            anotaciones.registros[0].color,
            Color::Otro("#99ee44".to_string())
        );
        let pintados = highlights(texto, &anotaciones);
        assert_eq!(pintados[0].color, verde_lima);
        // Cambiarlo a un color de la paleta reemplaza el libre.
        highlight(&mut anotaciones, texto, 4, 7, 2).unwrap();
        assert_eq!(anotaciones.registros[0].color, Color::Azul);
    }

    #[test]
    fn deshacer_el_borrado_recupera_el_subrayado() {
        let original = "uno dos tres";
        let mut anotaciones = Anotaciones::default();
        highlight(&mut anotaciones, original, 4, 7, 1).unwrap();
        assert!(highlights("uno tres", &anotaciones).is_empty());
        // Volver al texto de antes (deshacer) lo ancla otra vez.
        let vuelto = highlights(original, &anotaciones);
        assert_eq!(vuelto.len(), 1);
        assert_eq!(&original[vuelto[0].start..vuelto[0].end], "dos");
    }
}
