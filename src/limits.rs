/// Tope duro al recorrer el arbol Markdown. Una entrada con miles de citas o
/// listas anidadas puede desbordar la pila antes de devolver un error normal.
pub(crate) const MAX_NEST: u16 = 64;

/// Máximo de bloques producidos por el modelo enriquecido o la vista segura.
pub(crate) const MAX_BLOCKS: usize = 100_000;

/// La sangría visual deja de crecer antes de consumir todo el ancho útil.
pub(crate) const MAX_INDENT_DEPTH: u8 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Degradation {
    DepthLimit,
    BlockLimit,
}

impl Degradation {
    pub(crate) fn explanation(self) -> &'static str {
        match self {
            Self::DepthLimit => "se excedio el limite de anidamiento",
            Self::BlockLimit => "se excedio el limite de bloques",
        }
    }
}
