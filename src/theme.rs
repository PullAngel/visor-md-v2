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
    bg: (0x0C, 0x0F, 0x0D),
    surface: (0x12, 0x15, 0x13),
    elevated: (0x1A, 0x1F, 0x1C),
    floating: (0x23, 0x2A, 0x26),
    border: (0x1D, 0x23, 0x20),
    text: (0xE9, 0xE9, 0xE4),
    dim: (0x8B, 0x91, 0x8C),
    accent: (0x5F, 0xD0, 0x8A),
    external_link: (0x7D, 0xB3, 0xFF),
    mark: (0x2D, 0x53, 0x35),
    kbd: (0x21, 0x28, 0x23),
};

pub(crate) const DAY: Palette = Palette {
    bg: (0xEB, 0xFA, 0xDC),
    surface: (0xF7, 0xFD, 0xEF),
    elevated: (0xEE, 0xF6, 0xE3),
    floating: (0xE2, 0xED, 0xD5),
    border: (0xD6, 0xE5, 0xC6),
    text: (0x13, 0x2A, 0x0A),
    dim: (0x5A, 0x6B, 0x4F),
    accent: (0x2E, 0x9E, 0x5B),
    external_link: (0x1E, 0x68, 0xC4),
    mark: (0xC9, 0xEA, 0xAA),
    kbd: (0xDE, 0xE8, 0xD4),
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
}
