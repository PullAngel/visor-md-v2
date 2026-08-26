/// Paleta Papel y tinta. El acento cambia entre temas porque el verde que
/// conserva contraste sobre negro pierde fuerza sobre el fondo claro.
#[derive(Clone, Copy)]
pub(crate) struct Palette {
    pub(crate) bg: (u8, u8, u8),
    pub(crate) surface: (u8, u8, u8),
    pub(crate) text: (u8, u8, u8),
    pub(crate) dim: (u8, u8, u8),
    pub(crate) accent: (u8, u8, u8),
}

pub(crate) const NIGHT: Palette = Palette {
    bg: (0x0C, 0x0F, 0x0D),
    surface: (0x12, 0x15, 0x13),
    text: (0xE9, 0xE9, 0xE4),
    dim: (0x8B, 0x91, 0x8C),
    accent: (0x5F, 0xD0, 0x8A),
};

pub(crate) const DAY: Palette = Palette {
    bg: (0xEB, 0xFA, 0xDC),
    surface: (0xF7, 0xFD, 0xEF),
    text: (0x13, 0x2A, 0x0A),
    dim: (0x5A, 0x6B, 0x4F),
    accent: (0x2E, 0x9E, 0x5B),
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Role {
    Text,
    Dim,
    Accent,
}

impl Palette {
    pub(crate) fn resolve(self, role: Role) -> (u8, u8, u8) {
        match role {
            Role::Text => self.text,
            Role::Dim => self.dim,
            Role::Accent => self.accent,
        }
    }
}
