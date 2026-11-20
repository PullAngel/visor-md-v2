//! Herramientas de estudio sobre el documento (fase 14): traducir entre lo que
//! se ve en Leer y la fuente Markdown, crear y quitar subrayados y calcular qué
//! tramos pintar. Sin interfaz: solo texto, rangos y el formato de
//! `anotaciones`.

use crate::anotaciones::{self, Ancla, Anotaciones, Color, Forma, Lectura, Registro, Tipo};

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

/// Índice de un color en la paleta, o `None` si es desconocido.
pub(crate) fn color_index(color: &Color) -> Option<usize> {
    crate::theme::STUDY_COLOR_NAMES
        .iter()
        .position(|name| *name == color.texto())
}

pub(crate) fn color_for_index(index: usize) -> Color {
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
        return Err("Seleccioná el texto que querés subrayar");
    }
    let index = LfIndex::new(source);
    let (first, last) = (index.char_at_byte(start), index.char_at_byte(end));
    let chars: Vec<char> = index.lf.chars().collect();
    let cita: String = chars[first..last].iter().collect();
    if cita.trim().is_empty() {
        return Err("Seleccioná el texto que querés subrayar");
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
            })
        })
        .collect();
    marks.sort_by_key(|mark| mark.start);
    marks
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

/// La siguiente forma de marca: barra → cinta → punto → bandera → barra.
pub(crate) fn next_shape(shape: Option<&Forma>) -> Forma {
    match shape {
        Some(Forma::Barra) | None | Some(Forma::Otro(_)) => Forma::Cinta,
        Some(Forma::Cinta) => Forma::Punto,
        Some(Forma::Punto) => Forma::Bandera,
        Some(Forma::Bandera) => Forma::Barra,
    }
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
                format!("{}: {label}", crate::theme::STUDY_COLOR_NAMES[*color])
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
        sections[index]
            .marks
            .push((mark.color, mark.start, mark.label.clone()));
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
            registro.forma = Some(next_shape(registro.forma.as_ref()));
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
