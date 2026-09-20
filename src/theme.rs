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
};

pub(crate) const DAY: Palette = Palette {
    bg: (0xE8, 0xDD, 0xC7),
    surface: (0xF5, 0xEE, 0xDC),
    elevated: (0xED, 0xE2, 0xCC),
    floating: (0xFA, 0xF4, 0xE7),
    border: (0xD3, 0xC4, 0xA7),
    text: (0x2B, 0x25, 0x1C),
    dim: (0x6C, 0x61, 0x51),
    accent: (0x2F, 0x6B, 0x45),
    external_link: (0x1E, 0x68, 0xC4),
    mark: (0xD9, 0xE4, 0xC9),
    kbd: (0xE5, 0xDA, 0xC3),
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Role {
    Text,
    Dim,
}

impl Palette {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DAY, NIGHT};

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
        }
    }
}
