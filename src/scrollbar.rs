//! Barra de desplazamiento del documento (8.4).
//!
//! Solo geometría: dónde está la barra, qué parte del documento representa y
//! cuánto hay que desplazar al arrastrarla. Dibujo y clic usan las mismas
//! cuentas.

use std::time::Duration;

/// Ancho de la barra.
pub(crate) const WIDTH: f32 = 6.0;
/// Zona del borde derecho donde el puntero la hace aparecer.
pub(crate) const HOT_ZONE: f32 = 12.0;
/// Cuánto sigue visible después de desplazar.
pub(crate) const VISIBLE_AFTER_SCROLL: Duration = Duration::from_secs(1);
/// Opacidad del tirador en reposo: da al menos 3:1 contra el documento en los dos
/// temas (WCAG 1.4.11), lo que vigila la prueba de `theme.rs`.
pub(crate) const THUMB_ALPHA: u8 = 180;
const EDGE_GAP: f32 = 3.0;
const VERTICAL_GAP: f32 = 2.0;
const MIN_THUMB_HEIGHT: f32 = 28.0;

/// La barra de un panel: su recorrido y el tirador.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Thumb {
    pub(crate) x: f32,
    pub(crate) track_y: f32,
    pub(crate) track_height: f32,
    pub(crate) y: f32,
    pub(crate) height: f32,
    /// Borde derecho del panel: la zona activa se mide desde ahí.
    pub(crate) pane_right: f32,
}

/// Geometría del tirador de un panel (`pane_*` en píxeles de ventana). Es
/// `None` si el documento cabe entero.
pub(crate) fn thumb(
    pane_x: f32,
    pane_y: f32,
    pane_width: f32,
    pane_height: f32,
    scroll: f32,
    document_height: f32,
) -> Option<Thumb> {
    if document_height <= pane_height || pane_height <= 0.0 {
        return None;
    }
    let track_height = (pane_height - VERTICAL_GAP * 2.0).max(1.0);
    let height = (track_height * pane_height / document_height)
        .max(MIN_THUMB_HEIGHT)
        .min(track_height);
    let max_scroll = document_height - pane_height;
    let progress = (scroll / max_scroll).clamp(0.0, 1.0);
    let right = pane_x + pane_width;
    Some(Thumb {
        x: right - WIDTH - EDGE_GAP,
        track_y: pane_y + VERTICAL_GAP,
        track_height,
        y: pane_y + VERTICAL_GAP + (track_height - height) * progress,
        height,
        pane_right: right,
    })
}

impl Thumb {
    /// El punto está en la zona de 12 px del borde derecho, a la altura de
    /// la barra.
    pub(crate) fn hot_zone_contains(&self, x: f32, y: f32) -> bool {
        x >= self.pane_right - HOT_ZONE
            && x < self.pane_right
            && y >= self.track_y
            && y < self.track_y + self.track_height
    }

    /// El punto está sobre el tirador (con la zona activa como ancho, para que
    /// sea fácil de agarrar).
    pub(crate) fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.pane_right - HOT_ZONE
            && x < self.pane_right
            && y >= self.y
            && y < self.y + self.height
    }

    /// Desplazamiento del documento cuando el borde superior del tirador está
    /// en `thumb_y`.
    pub(crate) fn scroll_for(&self, thumb_y: f32, pane_height: f32, document_height: f32) -> f32 {
        let free = (self.track_height - self.height).max(1.0);
        let progress = ((thumb_y - self.track_y) / free).clamp(0.0, 1.0);
        progress * (document_height - pane_height).max(0.0)
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn un_documento_que_cabe_no_tiene_barra() {
        assert_eq!(thumb(0.0, 110.0, 800.0, 500.0, 0.0, 500.0), None);
        assert_eq!(thumb(0.0, 110.0, 800.0, 500.0, 0.0, 120.0), None);
    }

    #[test]
    fn el_tirador_representa_lo_que_se_ve_y_recorre_la_pista() {
        let arriba = thumb(0.0, 110.0, 800.0, 500.0, 0.0, 2000.0).unwrap();
        // Se ve un cuarto del documento: el tirador mide un cuarto del recorrido.
        assert!((arriba.height - arriba.track_height * 0.25).abs() < 1.0);
        assert_eq!(arriba.y, arriba.track_y);
        let abajo = thumb(0.0, 110.0, 800.0, 500.0, 1500.0, 2000.0).unwrap();
        assert!((abajo.y + abajo.height - (abajo.track_y + abajo.track_height)).abs() < 0.01);
        // Pegado al borde derecho, de 6 px.
        assert_eq!(arriba.x + WIDTH + EDGE_GAP, 800.0);
    }

    #[test]
    fn un_documento_enorme_no_reduce_el_tirador_por_debajo_del_minimo() {
        let t = thumb(0.0, 110.0, 800.0, 500.0, 0.0, 1_000_000.0).unwrap();
        assert_eq!(t.height, MIN_THUMB_HEIGHT);
    }

    #[test]
    fn arrastrar_el_tirador_devuelve_el_desplazamiento_y_es_inverso_de_thumb() {
        let (alto, documento) = (500.0, 2000.0);
        for scroll in [0.0, 300.0, 750.0, 1500.0] {
            let t = thumb(0.0, 110.0, 800.0, alto, scroll, documento).unwrap();
            let vuelta = t.scroll_for(t.y, alto, documento);
            assert!((vuelta - scroll).abs() < 1.0, "{scroll} → {vuelta}");
        }
        let t = thumb(0.0, 110.0, 800.0, alto, 0.0, documento).unwrap();
        // Más allá de los extremos se queda en el extremo.
        assert_eq!(t.scroll_for(-500.0, alto, documento), 0.0);
        assert_eq!(t.scroll_for(10_000.0, alto, documento), 1500.0);
    }

    #[test]
    fn la_zona_activa_son_los_12_px_del_borde() {
        let t = thumb(0.0, 110.0, 800.0, 500.0, 0.0, 2000.0).unwrap();
        assert!(t.hot_zone_contains(795.0, 300.0));
        assert!(!t.hot_zone_contains(700.0, 300.0));
        assert!(t.contains(795.0, t.y + 5.0));
        assert!(!t.contains(795.0, t.y + t.height + 20.0));
    }
}
