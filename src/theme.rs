/// Paleta Papel y tinta. El acento cambia entre temas porque el verde que
/// conserva contraste sobre negro pierde fuerza sobre el fondo claro.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Palette {
    pub(crate) bg: (u8, u8, u8),
    /// Base del documento. Se separa del fondo de ventana sin usar sombras.
    pub(crate) surface: (u8, u8, u8),
    /// Bloques y chrome que necesitan presencia sin competir con la lectura.
    pub(crate) elevated: (u8, u8, u8),
    /// Menús, paneles y avisos que flotan sobre el documento.
    pub(crate) floating: (u8, u8, u8),
    pub(crate) border: (u8, u8, u8),
    pub(crate) text: (u8, u8, u8),
    pub(crate) dim: (u8, u8, u8),
    pub(crate) accent: (u8, u8, u8),
    /// Azul convencional para destinos que salen de la aplicación, separado
    /// del verde reservado para la navegación interna.
    pub(crate) external_link: (u8, u8, u8),
    pub(crate) mark: (u8, u8, u8),
    pub(crate) kbd: (u8, u8, u8),
    /// Pestaña activa: un verde propio, algo más saturado que el documento,
    /// para reconocerla de un vistazo sin competir con el acento.
    pub(crate) tab: (u8, u8, u8),
    /// Fondo de una pestaña inactiva al pasar el puntero.
    pub(crate) tab_idle: (u8, u8, u8),
    /// Colores de estudio (subrayados y marcas de libro), en el orden de la
    /// paleta: amarillo, verde, azul, rosa y violeta.
    pub(crate) study: [StudyColor; 5],
}

/// Un color de estudio en un tema: el fondo que va detrás del texto subrayado
/// (el texto sigue con su color y contraste) y un trazo sólido para la barra
/// de una marca, los puntos del índice y el selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StudyColor {
    pub(crate) fill: (u8, u8, u8),
    pub(crate) stroke: (u8, u8, u8),
}

/// Nombres de los colores de estudio, en el orden de `Palette::study`.
pub(crate) const STUDY_COLOR_NAMES: [&str; 5] = ["amarillo", "verde", "azul", "rosa", "violeta"];

const fn study(fill: (u8, u8, u8), stroke: (u8, u8, u8)) -> StudyColor {
    StudyColor { fill, stroke }
}

pub(crate) const NIGHT: Palette = Palette {
    bg: (0x08, 0x10, 0x0C),
    surface: (0x0D, 0x17, 0x12),
    elevated: (0x14, 0x21, 0x19),
    floating: (0x1B, 0x2B, 0x21),
    border: (0x24, 0x37, 0x2C),
    text: (0xEC, 0xE8, 0xDE),
    dim: (0x9B, 0xA3, 0x99),
    accent: (0x62, 0xC9, 0x8A),
    external_link: (0x7D, 0xB3, 0xFF),
    mark: (0x2C, 0x4B, 0x35),
    kbd: (0x18, 0x27, 0x1E),
    tab: (0x17, 0x30, 0x1F),
    tab_idle: (0x11, 0x22, 0x18),
    study: [
        study((0x55, 0x44, 0x10), (0xE8, 0xC5, 0x4A)),
        study((0x30, 0x50, 0x10), (0x9F, 0xD3, 0x5A)),
        study((0x1C, 0x34, 0x5C), (0x6F, 0xA8, 0xF0)),
        study((0x55, 0x22, 0x38), (0xF0, 0x86, 0xB0)),
        study((0x40, 0x26, 0x5E), (0xB4, 0x96, 0xF2)),
    ],
};

pub(crate) const DAY: Palette = Palette {
    bg: (0xE8, 0xDD, 0xC7),
    surface: (0xF5, 0xEE, 0xDC),
    elevated: (0xED, 0xE2, 0xCC),
    floating: (0xFA, 0xF4, 0xE7),
    border: (0xD3, 0xC4, 0xA7),
    text: (0x2B, 0x25, 0x1C),
    dim: (0x5A, 0x50, 0x43),
    accent: (0x2F, 0x6B, 0x45),
    external_link: (0x1A, 0x5A, 0xAE),
    mark: (0xD9, 0xE4, 0xC9),
    kbd: (0xE5, 0xDA, 0xC3),
    tab: (0xDC, 0xE6, 0xCF),
    tab_idle: (0xED, 0xE4, 0xCF),
    study: [
        study((0xF7, 0xE0, 0x7A), (0x8C, 0x6A, 0x00)),
        study((0xC4, 0xE6, 0x9C), (0x4A, 0x7A, 0x1A)),
        study((0xC6, 0xDC, 0xF5), (0x2A, 0x62, 0xB0)),
        study((0xF4, 0xC0, 0xD4), (0xB0, 0x3A, 0x6E)),
        study((0xE2, 0xCB, 0xF2), (0x6E, 0x4C, 0xB8)),
    ],
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Role {
    Text,
    Dim,
}

impl Palette {
    /// El color de estudio por su nombre (`amarillo`, `verde`…). `None` si el
    /// nombre no es de la paleta: esa anotación se conserva pero no se dibuja.
    #[cfg_attr(not(test), allow(dead_code))] // Se usa desde la 14.5.
    pub(crate) fn study_color(&self, name: &str) -> Option<StudyColor> {
        STUDY_COLOR_NAMES
            .iter()
            .position(|known| *known == name)
            .map(|index| self.study[index])
    }

    pub(crate) fn resolve(self, role: Role) -> (u8, u8, u8) {
        match role {
            Role::Text => self.text,
            Role::Dim => self.dim,
        }
    }

    /// Interpolación corta para una transición visual. El modelo documental no
    /// participa: solo se transforma la paleta de dibujo entre dos estados ya
    /// válidos.
    pub(crate) fn interpolate(self, target: Self, progress: f32) -> Self {
        let progress = progress.clamp(0.0, 1.0);
        let mix = |from: (u8, u8, u8), to: (u8, u8, u8)| {
            (
                (from.0 as f32 + (to.0 as f32 - from.0 as f32) * progress).round() as u8,
                (from.1 as f32 + (to.1 as f32 - from.1 as f32) * progress).round() as u8,
                (from.2 as f32 + (to.2 as f32 - from.2 as f32) * progress).round() as u8,
            )
        };
        Self {
            bg: mix(self.bg, target.bg),
            surface: mix(self.surface, target.surface),
            elevated: mix(self.elevated, target.elevated),
            floating: mix(self.floating, target.floating),
            border: mix(self.border, target.border),
            text: mix(self.text, target.text),
            dim: mix(self.dim, target.dim),
            accent: mix(self.accent, target.accent),
            external_link: mix(self.external_link, target.external_link),
            mark: mix(self.mark, target.mark),
            kbd: mix(self.kbd, target.kbd),
            tab: mix(self.tab, target.tab),
            tab_idle: mix(self.tab_idle, target.tab_idle),
            study: std::array::from_fn(|index| StudyColor {
                fill: mix(self.study[index].fill, target.study[index].fill),
                stroke: mix(self.study[index].stroke, target.study[index].stroke),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DAY, NIGHT, STUDY_COLOR_NAMES};

    fn relative_luminance((red, green, blue): (u8, u8, u8)) -> f64 {
        let channel = |value: u8| {
            let value = f64::from(value) / 255.0;
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * channel(red) + 0.7152 * channel(green) + 0.0722 * channel(blue)
    }

    fn contrast(left: (u8, u8, u8), right: (u8, u8, u8)) -> f64 {
        let left = relative_luminance(left);
        let right = relative_luminance(right);
        (left.max(right) + 0.05) / (left.min(right) + 0.05)
    }

    #[test]
    fn la_interpolacion_conserva_los_extremos_exactos() {
        assert_eq!(NIGHT.interpolate(DAY, -1.0), NIGHT);
        assert_eq!(NIGHT.interpolate(DAY, 0.0), NIGHT);
        assert_eq!(NIGHT.interpolate(DAY, 1.0), DAY);
        assert_eq!(NIGHT.interpolate(DAY, 2.0), DAY);
    }

    #[test]
    fn la_interpolacion_mezcla_todos_los_roles_de_color() {
        let middle = NIGHT.interpolate(DAY, 0.5);
        assert_ne!(middle.bg, NIGHT.bg);
        assert_ne!(middle.bg, DAY.bg);
        assert_ne!(middle.text, NIGHT.text);
        assert_ne!(middle.accent, DAY.accent);
        assert_ne!(middle.external_link, NIGHT.external_link);
    }

    #[test]
    fn ambos_temas_conservan_contraste_para_lectura_y_controles() {
        for palette in [DAY, NIGHT] {
            assert!(contrast(palette.text, palette.surface) >= 7.0);
            assert!(contrast(palette.dim, palette.surface) >= 4.5);
            assert!(contrast(palette.accent, palette.surface) >= 4.5);
            // Las pestañas: texto legible sobre la activa y sobre el hover.
            assert!(contrast(palette.text, palette.tab) >= 7.0);
            assert!(contrast(palette.dim, palette.tab) >= 4.5);
            assert!(contrast(palette.dim, palette.tab_idle) >= 4.5);
            assert!(contrast(palette.accent, palette.tab) >= 4.5);
        }
    }

    /// WCAG 1.4.11: el tirador de la barra de desplazamiento, mezclado con su
    /// opacidad sobre el documento, se distingue con al menos 3:1.
    #[test]
    fn el_tirador_de_la_barra_se_distingue_del_documento() {
        for palette in [DAY, NIGHT] {
            let alpha = f64::from(crate::scrollbar::THUMB_ALPHA) / 255.0;
            let mezcla =
                |a: u8, b: u8| (f64::from(a) * alpha + f64::from(b) * (1.0 - alpha)).round() as u8;
            let tirador = (
                mezcla(palette.dim.0, palette.surface.0),
                mezcla(palette.dim.1, palette.surface.1),
                mezcla(palette.dim.2, palette.surface.2),
            );
            assert!(contrast(tirador, palette.surface) >= 3.0);
        }
    }

    /// D-24: el tema de día sube sus mínimos para texto tenue y enlaces
    /// externos, que eran lo más justo.
    #[test]
    fn el_tema_de_dia_tiene_margen_en_texto_tenue_y_enlaces_externos() {
        assert!(contrast(DAY.dim, DAY.surface) >= 5.5);
        assert!(contrast(DAY.dim, DAY.bg) >= 5.5);
        assert!(contrast(DAY.dim, DAY.elevated) >= 5.5);
        assert!(contrast(DAY.dim, DAY.tab) >= 5.5);
        assert!(contrast(DAY.external_link, DAY.surface) >= 5.5);
    }

    #[test]
    fn la_pestana_activa_se_distingue_del_documento_y_del_fondo() {
        for palette in [DAY, NIGHT] {
            assert_ne!(palette.tab, palette.surface);
            assert_ne!(palette.tab, palette.bg);
            assert_ne!(palette.tab, palette.tab_idle);
        }
        // En noche el verde de la pestaña es más claro que el fondo de la
        // barra, no solo distinto.
        assert!(
            relative_luminance(NIGHT.tab) > relative_luminance(NIGHT.bg),
            "la pestaña activa debe leerse sobre el fondo de la barra"
        );
    }

    /// El texto subrayado se sigue leyendo como el resto (7:1, AAA) y el trazo
    /// de una marca o un punto del índice se distingue del documento (3:1,
    /// WCAG 1.4.11), en los dos temas.
    #[test]
    fn paleta_de_estudio_conserva_contraste_en_ambos_temas() {
        for palette in [DAY, NIGHT] {
            for (index, color) in palette.study.iter().enumerate() {
                let nombre = STUDY_COLOR_NAMES[index];
                assert!(
                    contrast(palette.text, color.fill) >= 7.0,
                    "{nombre}: texto sobre el subrayado {:.2}",
                    contrast(palette.text, color.fill)
                );
                assert!(
                    contrast(color.stroke, palette.surface) >= 3.0,
                    "{nombre}: trazo sobre el documento {:.2}",
                    contrast(color.stroke, palette.surface)
                );
                assert!(contrast(color.stroke, palette.floating) >= 3.0, "{nombre}");
            }
        }
    }

    fn distancia((r1, g1, b1): (u8, u8, u8), (r2, g2, b2): (u8, u8, u8)) -> u32 {
        u32::from(r1.abs_diff(r2)) + u32::from(g1.abs_diff(g2)) + u32::from(b1.abs_diff(b2))
    }

    #[test]
    fn colores_de_estudio_se_distinguen_entre_si_y_del_resaltado() {
        for palette in [DAY, NIGHT] {
            for (i, a) in palette.study.iter().enumerate() {
                // Ni con el resaltado de búsqueda ni con el fondo del documento.
                assert!(
                    distancia(a.fill, palette.mark) >= 45,
                    "{}",
                    STUDY_COLOR_NAMES[i]
                );
                assert!(
                    distancia(a.fill, palette.surface) >= 45,
                    "{}",
                    STUDY_COLOR_NAMES[i]
                );
                for (j, b) in palette.study.iter().enumerate().skip(i + 1) {
                    let (x, y) = (STUDY_COLOR_NAMES[i], STUDY_COLOR_NAMES[j]);
                    assert!(distancia(a.fill, b.fill) >= 45, "{x} y {y}: fondos");
                    assert!(distancia(a.stroke, b.stroke) >= 60, "{x} y {y}: trazos");
                }
            }
            assert_eq!(palette.study_color("azul"), Some(palette.study[2]));
            assert_eq!(palette.study_color("turquesa"), None);
        }
    }
}
