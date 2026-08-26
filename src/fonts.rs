use parley::FontContext;

/// Tipografia "Contraste editorial" de `docs/design.md`: Sora para la
/// interfaz, Newsreader para el documento y JetBrains Mono para codigo.
#[allow(dead_code)]
pub(crate) const FONT_UI: &str = "Sora";
pub(crate) const FONT_DOC: &str = "Newsreader";
pub(crate) const FONT_CODE: &str = "JetBrains Mono";

const SORA_TTF: &[u8] = include_bytes!("../assets/fonts/Sora.ttf");
const NEWSREADER_TTF: &[u8] = include_bytes!("../assets/fonts/Newsreader.ttf");
const NEWSREADER_ITALIC_TTF: &[u8] = include_bytes!("../assets/fonts/Newsreader-Italic.ttf");
const JETBRAINS_MONO_TTF: &[u8] = include_bytes!("../assets/fonts/JetBrainsMono.ttf");

/// Registra cuatro archivos de tres familias. Si un archivo no pudiera
/// reconocerse, Parley conserva el fallback configurado por el renderer.
pub(crate) fn register_embedded_fonts(font_cx: &mut FontContext) {
    for bytes in [
        SORA_TTF,
        NEWSREADER_TTF,
        NEWSREADER_ITALIC_TTF,
        JETBRAINS_MONO_TTF,
    ] {
        let blob = parley::fontique::Blob::new(std::sync::Arc::new(bytes.to_vec()));
        font_cx.collection.register_fonts(blob, None);
    }
}
