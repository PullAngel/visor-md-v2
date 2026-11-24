//! Geometría de las marcas de libro en el margen derecho.
//!
//! Cada marca se dibuja como una etiqueta a la derecha de la columna de texto,
//! con un largo que depende del espacio libre: con lugar muestra su texto
//! recortado; con menos, solo la cinta con su signo; y si ni eso cabe, una
//! muesca pegada al borde de la columna. Nunca se dibuja sobre el texto.
//! Este módulo es solo cuentas: la aplicación lo usa igual para dibujar y para
//! saber sobre qué marca está el puntero.

/// Alto de una etiqueta.
pub(crate) const TAG_HEIGHT: f32 = 22.0;
/// Separación entre la columna de texto y la etiqueta.
pub(crate) const TAG_GAP: f32 = 12.0;
/// Ancho mínimo para mostrar texto, y el máximo que ocupa una etiqueta.
const FULL_MIN: f32 = 112.0;
const FULL_MAX: f32 = 172.0;
/// Ancho de la cinta sin texto.
pub(crate) const RIBBON_WIDTH: f32 = 22.0;
/// Ancho de la muesca.
pub(crate) const NOTCH_WIDTH: f32 = 4.0;
/// Espacio que se deja libre al borde de la ventana (barra de posición).
pub(crate) const EDGE_RESERVE: f32 = 16.0;
/// Separación mínima entre dos etiquetas una debajo de la otra.
const STACK_GAP: f32 = 3.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum MarkDisplay {
    /// Etiqueta con texto, de este ancho.
    Full(f32),
    /// Solo la cinta con su signo.
    Ribbon,
    /// Una muesca junto a la columna: el espacio no alcanza para más.
    Notch,
}

/// Cómo se muestran las marcas según el espacio libre a la derecha de la
/// columna de texto (ya sin la separación ni la reserva del borde).
pub(crate) fn display_for(available: f32) -> MarkDisplay {
    if available >= FULL_MIN {
        MarkDisplay::Full(available.min(FULL_MAX))
    } else if available >= RIBBON_WIDTH {
        MarkDisplay::Ribbon
    } else {
        MarkDisplay::Notch
    }
}

/// Espacio libre a la derecha de la columna de texto. `column_right` es el
/// borde derecho del texto; `pane_right`, el borde derecho del panel, y
/// `reserved`, lo que ocupa algo que flota a la derecha (como el índice).
pub(crate) fn available_width(column_right: f32, pane_right: f32, reserved: f32) -> f32 {
    (pane_right - column_right - TAG_GAP - EDGE_RESERVE - reserved).max(0.0)
}

/// Rectángulo (x, y, ancho, alto) de una etiqueta.
pub(crate) fn tag_rect(display: MarkDisplay, column_right: f32, y: f32) -> (f32, f32, f32, f32) {
    match display {
        MarkDisplay::Full(width) => (column_right + TAG_GAP, y, width, TAG_HEIGHT),
        MarkDisplay::Ribbon => (column_right + TAG_GAP, y, RIBBON_WIDTH, TAG_HEIGHT),
        MarkDisplay::Notch => (column_right + 2.0, y, NOTCH_WIDTH, TAG_HEIGHT),
    }
}

/// Corre hacia abajo las etiquetas que se pisarían. `ys` va de arriba hacia
/// abajo y se modifica en sitio; devuelve cuánto se corrió cada una.
pub(crate) fn separate_vertically(ys: &mut [f32]) {
    for index in 1..ys.len() {
        let minimum = ys[index - 1] + TAG_HEIGHT + STACK_GAP;
        if ys[index] < minimum {
            ys[index] = minimum;
        }
    }
}

/// El texto de una etiqueta, recortado a `max_chars` con «…». Cuenta
/// caracteres, no bytes, y no corta una palabra si cabe un espacio cerca.
pub(crate) fn visible_label(text: &str, max_chars: usize) -> String {
    let flat: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= max_chars {
        return flat;
    }
    let head: String = flat.chars().take(max_chars.saturating_sub(1)).collect();
    // Si hay un espacio en el último tercio, se corta ahí.
    let cut = head
        .rfind(' ')
        .filter(|index| *index >= head.len() * 2 / 3)
        .map_or(head.as_str(), |index| &head[..index]);
    format!("{}…", cut.trim_end())
}

/// Blanco o negro, el que más contraste da sobre `rgb`.
pub(crate) fn contrast_on(rgb: (u8, u8, u8)) -> (u8, u8, u8) {
    let luminance = 0.299 * f32::from(rgb.0) + 0.587 * f32::from(rgb.1) + 0.114 * f32::from(rgb.2);
    if luminance > 150.0 {
        (0x1A, 0x1A, 0x1A)
    } else {
        (0xFF, 0xFF, 0xFF)
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn el_espacio_decide_si_hay_texto_cinta_o_muesca() {
        assert_eq!(display_for(300.0), MarkDisplay::Full(FULL_MAX));
        assert_eq!(display_for(130.0), MarkDisplay::Full(130.0));
        assert_eq!(display_for(111.0), MarkDisplay::Ribbon);
        assert_eq!(display_for(RIBBON_WIDTH), MarkDisplay::Ribbon);
        assert_eq!(display_for(RIBBON_WIDTH - 1.0), MarkDisplay::Notch);
        assert_eq!(display_for(0.0), MarkDisplay::Notch);
    }

    #[test]
    fn el_espacio_libre_nunca_es_negativo_y_respeta_lo_que_flota() {
        // Ventana ancha: sobra margen.
        assert!(available_width(700.0, 1200.0, 0.0) > FULL_MIN);
        // Con el índice flotante abierto no alcanza.
        assert!(available_width(900.0, 1200.0, 260.0) < FULL_MIN);
        // Ventana angosta: no hay margen, pero tampoco un valor negativo.
        assert_eq!(available_width(640.0, 600.0, 0.0), 0.0);
    }

    #[test]
    fn la_etiqueta_nunca_toca_la_columna_ni_se_sale_de_su_espacio() {
        let columna = 700.0;
        for disponible in [0.0, 10.0, 30.0, 120.0, 400.0] {
            let display = display_for(disponible);
            let (x, _, ancho, alto) = tag_rect(display, columna, 100.0);
            assert!(x > columna, "toca el texto con {disponible} libres");
            assert_eq!(alto, TAG_HEIGHT);
            if !matches!(display, MarkDisplay::Notch) {
                assert!(x - columna >= TAG_GAP);
                assert!(ancho <= disponible.max(RIBBON_WIDTH) + 0.01, "{disponible}");
            }
        }
    }

    #[test]
    fn dos_etiquetas_juntas_se_separan_hacia_abajo() {
        let mut ys = [100.0, 104.0, 105.0, 300.0];
        separate_vertically(&mut ys);
        assert_eq!(ys[0], 100.0);
        assert!(ys[1] >= 100.0 + TAG_HEIGHT);
        assert!(ys[2] >= ys[1] + TAG_HEIGHT);
        assert_eq!(ys[3], 300.0, "la que ya estaba lejos no se mueve");
        for par in ys.windows(2) {
            assert!(par[1] - par[0] >= TAG_HEIGHT);
        }
    }

    #[test]
    fn el_texto_visible_se_recorta_con_puntos_suspensivos() {
        assert_eq!(
            visible_label("  Repasar   esto \n hoy ", 40),
            "Repasar esto hoy"
        );
        let largo = visible_label("Revisar la definición de entropía antes del examen", 24);
        assert!(largo.chars().count() <= 24, "{largo}");
        assert!(largo.ends_with('…'));
        assert!(!largo.contains("  "));
        // Con acentos y ñ no se corta un carácter por la mitad.
        let acentos = visible_label("ñandú árbol canción única", 8);
        assert!(
            acentos.ends_with('…') && acentos.chars().count() <= 8,
            "{acentos}"
        );
        assert_eq!(visible_label("", 5), "");
    }

    #[test]
    fn el_texto_de_la_etiqueta_contrasta_con_su_color() {
        assert_eq!(contrast_on((0xF0, 0xE0, 0x40)), (0x1A, 0x1A, 0x1A));
        assert_eq!(contrast_on((0x30, 0x20, 0x90)), (0xFF, 0xFF, 0xFF));
        assert_eq!(contrast_on((0xE5, 0x48, 0x4D)), (0xFF, 0xFF, 0xFF));
    }
}
