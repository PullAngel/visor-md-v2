//! Herramientas de estudio sobre el documento (fase 14): traducir entre lo que
//! se ve en Leer y la fuente Markdown, crear y quitar subrayados y calcular qué
//! tramos pintar. Sin interfaz: solo texto, rangos y el formato de
//! `anotaciones`.

use crate::anotaciones::{self, Ancla, Anotaciones, Color, Lectura, Registro, Tipo};

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
}
