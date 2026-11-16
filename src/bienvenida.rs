//! Pantalla de bienvenida (D-27).
//!
//! Se muestra sobre la pestaña inicial mientras no hay archivo de argumento y
//! el documento sigue vacío: tres botones grandes y dos líneas sobre los
//! modos. Es geometría pura, para que dibujo y clic coincidan.

use crate::dialog::DialogRect;

pub(crate) const TITLE: &str = "Bienvenida a Visor MD";
pub(crate) const BUTTON_LABELS: [&str; 3] =
    ["Abrir carpeta de apuntes", "Abrir archivo", "Nuevo apunte"];
/// Lo que se dice a la derecha de cada botón, tenue.
pub(crate) const BUTTON_SHORTCUTS: [&str; 3] = ["Ctrl+Shift+O", "Ctrl+O", "Ctrl+N"];
pub(crate) const HINTS: &str = "Leer muestra el documento; Editar abre su Markdown.\nComparar muestra las dos cosas lado a lado.";

pub(crate) const BUTTON_HEIGHT: f32 = 48.0;
const BUTTON_GAP: f32 = 10.0;
const BUTTON_MAX_WIDTH: f32 = 400.0;
const SIDE_MARGIN: f32 = 24.0;
const TITLE_HEIGHT: f32 = 34.0;
const TITLE_GAP: f32 = 22.0;
const HINTS_GAP: f32 = 24.0;
const HINTS_HEIGHT: f32 = 44.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct WelcomeGeometry {
    pub(crate) title_x: f32,
    pub(crate) title_y: f32,
    pub(crate) buttons: [DialogRect; 3],
    pub(crate) hints_x: f32,
    pub(crate) hints_y: f32,
    /// Ancho disponible para el título y las dos líneas.
    pub(crate) width: f32,
}

/// Acomoda el bloque en el centro del área del documento (`x`, `y`, `width`,
/// `height` en píxeles de ventana). Con poco alto se pega arriba, nunca se sale.
pub(crate) fn geometry(x: f32, y: f32, width: f32, height: f32) -> WelcomeGeometry {
    let block_width = (width - SIDE_MARGIN * 2.0).clamp(120.0, BUTTON_MAX_WIDTH);
    let left = (x + (width - block_width) * 0.5).round();
    let total = TITLE_HEIGHT
        + TITLE_GAP
        + BUTTON_HEIGHT * 3.0
        + BUTTON_GAP * 2.0
        + HINTS_GAP
        + HINTS_HEIGHT;
    let top = (y + ((height - total) * 0.5 - 10.0).max(8.0)).round();
    let buttons_top = top + TITLE_HEIGHT + TITLE_GAP;
    let button = |index: usize| DialogRect {
        x: left,
        y: (buttons_top + index as f32 * (BUTTON_HEIGHT + BUTTON_GAP)).round(),
        width: block_width,
        height: BUTTON_HEIGHT,
    };
    WelcomeGeometry {
        title_x: left,
        title_y: top,
        buttons: [button(0), button(1), button(2)],
        hints_x: left,
        hints_y: (buttons_top + BUTTON_HEIGHT * 3.0 + BUTTON_GAP * 2.0 + HINTS_GAP).round(),
        width: block_width,
    }
}

/// Qué botón hay bajo el punto.
pub(crate) fn button_at(geometry: &WelcomeGeometry, x: f32, y: f32) -> Option<usize> {
    geometry.buttons.iter().position(|rect| rect.contains(x, y))
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn los_tres_botones_caben_en_el_area_a_640_y_a_1100() {
        for (ancho, alto) in [(640.0, 344.0), (1100.0, 584.0), (1366.0, 700.0)] {
            let g = geometry(0.0, 110.0, ancho, alto);
            for rect in g.buttons {
                assert!(rect.x >= 0.0 && rect.x + rect.width <= ancho, "{ancho}");
                assert!(
                    rect.y >= 110.0 && rect.y + rect.height <= 110.0 + alto,
                    "{ancho}"
                );
            }
            assert!(
                g.hints_y + HINTS_HEIGHT <= 110.0 + alto,
                "las líneas caben a {ancho}"
            );
            // Los botones no se pisan.
            for par in g.buttons.windows(2) {
                assert!(par[0].y + par[0].height <= par[1].y);
            }
        }
    }

    #[test]
    fn cada_boton_responde_en_su_centro_y_no_fuera() {
        let g = geometry(0.0, 110.0, 1100.0, 584.0);
        for (indice, rect) in g.buttons.iter().enumerate() {
            let centro = (rect.x + rect.width * 0.5, rect.y + rect.height * 0.5);
            assert_eq!(button_at(&g, centro.0, centro.1), Some(indice));
        }
        assert_eq!(button_at(&g, 2.0, 2.0), None);
        // El hueco entre botones no activa ninguno.
        let hueco = g.buttons[0].y + BUTTON_HEIGHT + BUTTON_GAP * 0.5;
        assert_eq!(button_at(&g, g.buttons[0].x + 10.0, hueco), None);
    }

    #[test]
    fn con_una_columna_abierta_el_bloque_se_centra_en_el_resto() {
        let g = geometry(280.0, 110.0, 360.0, 344.0);
        assert!(g.buttons[0].x >= 280.0);
        assert!(g.buttons[0].x + g.buttons[0].width <= 640.0);
    }
}
