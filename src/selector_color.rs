//! Selector de color libre para los subrayados.
//!
//! Es el modelo y la geometría: conversiones de color, estado del selector y
//! dónde cae cada zona. No dibuja ni toca el documento; la aplicación lo
//! pinta con el mismo lienzo del resto de la interfaz. El color se elige en un
//! cuadro de saturación y brillo, una barra de tono y un campo hexadecimal.

/// Alto del cuadro de saturación y brillo.
pub(crate) const SQUARE_HEIGHT: f32 = 112.0;
/// Alto de la barra de tono.
pub(crate) const HUE_HEIGHT: f32 = 16.0;
/// Alto de una fila de controles (campo hexadecimal y botones).
pub(crate) const ROW_HEIGHT: f32 = 28.0;
/// Margen interior del selector.
pub(crate) const PADDING: f32 = 12.0;
/// Caracteres que se aceptan en el campo hexadecimal: `#` y seis dígitos.
const MAX_HEX_CHARS: usize = 7;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PickerDrag {
    Square,
    Hue,
}

/// Estado del selector libre. Tono de 0 a 360; saturación y brillo de 0 a 1.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CustomPicker {
    pub(crate) hue: f32,
    pub(crate) saturation: f32,
    pub(crate) value: f32,
    /// Lo que dice el campo hexadecimal, también mientras está incompleto.
    pub(crate) hex: String,
    pub(crate) drag: Option<PickerDrag>,
}

impl CustomPicker {
    pub(crate) fn from_rgb(rgb: (u8, u8, u8)) -> Self {
        let (hue, saturation, value) = rgb_to_hsv(rgb);
        Self {
            hue,
            saturation,
            value,
            hex: crate::estudio::hex_color(rgb).to_uppercase(),
            drag: None,
        }
    }

    pub(crate) fn rgb(&self) -> (u8, u8, u8) {
        hsv_to_rgb(self.hue, self.saturation, self.value)
    }

    fn sync_hex(&mut self) {
        self.hex = crate::estudio::hex_color(self.rgb()).to_uppercase();
    }

    /// Posición en el cuadro, de 0 a 1 en cada eje (arriba a la izquierda es 0, 0).
    pub(crate) fn set_square(&mut self, fx: f32, fy: f32) {
        self.saturation = fx.clamp(0.0, 1.0);
        self.value = 1.0 - fy.clamp(0.0, 1.0);
        self.sync_hex();
    }

    /// Posición en la barra de tono, de 0 a 1.
    pub(crate) fn set_hue(&mut self, fx: f32) {
        self.hue = fx.clamp(0.0, 1.0) * 360.0;
        self.sync_hex();
    }

    /// El color escrito en el campo, si está completo y es válido.
    pub(crate) fn parsed(&self) -> Option<(u8, u8, u8)> {
        crate::estudio::parse_hex_color(&self.hex)
    }

    /// Agrega texto tecleado al campo hexadecimal (solo `#` al comienzo y
    /// dígitos hexadecimales). Si el resultado es un color válido, mueve el
    /// cuadro y la barra.
    pub(crate) fn type_hex(&mut self, text: &str) {
        for character in text.chars() {
            let allowed =
                (character == '#' && self.hex.is_empty()) || character.is_ascii_hexdigit();
            if allowed && self.hex.chars().count() < MAX_HEX_CHARS {
                self.hex.push(character.to_ascii_uppercase());
            }
        }
        self.follow_hex();
    }

    pub(crate) fn backspace(&mut self) {
        self.hex.pop();
        self.follow_hex();
    }

    fn follow_hex(&mut self) {
        if let Some(rgb) = self.parsed() {
            let (hue, saturation, value) = rgb_to_hsv(rgb);
            // En un gris el tono no se puede deducir: se conserva el que había.
            if saturation > 0.0 && value > 0.0 {
                self.hue = hue;
            }
            self.saturation = saturation;
            self.value = value;
        }
    }
}

/// Tono (0 a 360), saturación y brillo (0 a 1) a RGB.
pub(crate) fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (u8, u8, u8) {
    let hue = hue.rem_euclid(360.0);
    let saturation = saturation.clamp(0.0, 1.0);
    let value = value.clamp(0.0, 1.0);
    let chroma = value * saturation;
    let sector = hue / 60.0;
    let second = chroma * (1.0 - (sector % 2.0 - 1.0).abs());
    let (red, green, blue) = match sector as u8 {
        0 => (chroma, second, 0.0),
        1 => (second, chroma, 0.0),
        2 => (0.0, chroma, second),
        3 => (0.0, second, chroma),
        4 => (second, 0.0, chroma),
        _ => (chroma, 0.0, second),
    };
    let offset = value - chroma;
    let channel = |part: f32| ((part + offset) * 255.0).round().clamp(0.0, 255.0) as u8;
    (channel(red), channel(green), channel(blue))
}

/// RGB a tono (0 a 360), saturación y brillo (0 a 1).
pub(crate) fn rgb_to_hsv(rgb: (u8, u8, u8)) -> (f32, f32, f32) {
    let (red, green, blue) = (
        f32::from(rgb.0) / 255.0,
        f32::from(rgb.1) / 255.0,
        f32::from(rgb.2) / 255.0,
    );
    let maximum = red.max(green).max(blue);
    let minimum = red.min(green).min(blue);
    let delta = maximum - minimum;
    let hue = if delta == 0.0 {
        0.0
    } else if maximum == red {
        60.0 * ((green - blue) / delta).rem_euclid(6.0)
    } else if maximum == green {
        60.0 * ((blue - red) / delta + 2.0)
    } else {
        60.0 * ((red - green) / delta + 4.0)
    };
    let saturation = if maximum == 0.0 { 0.0 } else { delta / maximum };
    (hue, saturation, maximum)
}

type Rect = (f32, f32, f32, f32);

fn contains(rect: Rect, x: f32, y: f32) -> bool {
    x >= rect.0 && x < rect.0 + rect.2 && y >= rect.1 && y < rect.1 + rect.3
}

/// Zonas del selector libre, en coordenadas de ventana.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PickerGeometry {
    pub(crate) rect: Rect,
    pub(crate) square: Rect,
    pub(crate) hue: Rect,
    pub(crate) preview: Rect,
    pub(crate) hex: Rect,
    pub(crate) back: Rect,
    pub(crate) apply: Rect,
}

/// Dónde cae el selector: bajo `top`, alineado con `anchor_x` y sin salirse de
/// la ventana. `width` es el ancho del selector.
pub(crate) fn picker_geometry(
    anchor_x: f32,
    window_width: f32,
    top: f32,
    width: f32,
) -> PickerGeometry {
    let x = anchor_x.min(window_width - width - 8.0).max(8.0);
    let inner = width - PADDING * 2.0;
    let square = (x + PADDING, top + PADDING, inner, SQUARE_HEIGHT);
    let hue = (
        x + PADDING,
        square.1 + SQUARE_HEIGHT + 10.0,
        inner,
        HUE_HEIGHT,
    );
    let row_top = hue.1 + HUE_HEIGHT + 12.0;
    let preview = (x + PADDING, row_top, ROW_HEIGHT, ROW_HEIGHT);
    let hex = (
        x + PADDING + ROW_HEIGHT + 8.0,
        row_top,
        inner - ROW_HEIGHT - 8.0,
        ROW_HEIGHT,
    );
    let buttons_top = row_top + ROW_HEIGHT + 10.0;
    let button_width = (inner - 8.0) / 2.0;
    let back = (x + PADDING, buttons_top, button_width, ROW_HEIGHT);
    let apply = (
        x + PADDING + button_width + 8.0,
        buttons_top,
        button_width,
        ROW_HEIGHT,
    );
    let height = buttons_top + ROW_HEIGHT + PADDING - top;
    PickerGeometry {
        rect: (x, top, width, height),
        square,
        hue,
        preview,
        hex,
        back,
        apply,
    }
}

/// Lo que hay bajo un punto del selector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum PickerHit {
    /// Dentro del cuadro, con la posición de 0 a 1 en cada eje.
    Square(f32, f32),
    /// Sobre la barra de tono, de 0 a 1.
    Hue(f32),
    Hex,
    Back,
    Apply,
    /// Dentro del selector, en un lugar sin control.
    Panel,
    Outside,
}

pub(crate) fn picker_hit(geometry: &PickerGeometry, x: f32, y: f32) -> PickerHit {
    if !contains(geometry.rect, x, y) {
        return PickerHit::Outside;
    }
    if contains(geometry.square, x, y) {
        return PickerHit::Square(
            (x - geometry.square.0) / geometry.square.2,
            (y - geometry.square.1) / geometry.square.3,
        );
    }
    if contains(geometry.hue, x, y) {
        return PickerHit::Hue((x - geometry.hue.0) / geometry.hue.2);
    }
    if contains(geometry.hex, x, y) {
        return PickerHit::Hex;
    }
    if contains(geometry.back, x, y) {
        return PickerHit::Back;
    }
    if contains(geometry.apply, x, y) {
        return PickerHit::Apply;
    }
    PickerHit::Panel
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn rgb_y_hsv_se_convierten_de_ida_y_vuelta() {
        for rgb in [
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (255, 255, 0),
            (0, 0, 0),
            (255, 255, 255),
            (128, 128, 128),
            (0xE0, 0x30, 0x20),
            (0x12, 0x9A, 0xFE),
        ] {
            let (hue, saturation, value) = rgb_to_hsv(rgb);
            assert!((0.0..360.0).contains(&hue), "{rgb:?}: tono {hue}");
            assert_eq!(hsv_to_rgb(hue, saturation, value), rgb, "{rgb:?}");
        }
        assert_eq!(hsv_to_rgb(0.0, 1.0, 1.0), (255, 0, 0));
        assert_eq!(hsv_to_rgb(120.0, 1.0, 1.0), (0, 255, 0));
        assert_eq!(hsv_to_rgb(240.0, 1.0, 1.0), (0, 0, 255));
        assert_eq!(hsv_to_rgb(360.0, 1.0, 1.0), (255, 0, 0));
        assert_eq!(hsv_to_rgb(0.0, 0.0, 0.5), (128, 128, 128));
    }

    #[test]
    fn mover_el_cuadro_y_la_barra_cambia_el_color_y_el_campo() {
        let mut selector = CustomPicker::from_rgb((255, 0, 0));
        assert_eq!(selector.hex, "#FF0000");
        // Arriba a la derecha: saturación y brillo completos.
        selector.set_square(1.0, 0.0);
        assert_eq!(selector.rgb(), (255, 0, 0));
        // Abajo, sea cual sea la x, es negro.
        selector.set_square(0.4, 1.0);
        assert_eq!(selector.rgb(), (0, 0, 0));
        assert_eq!(selector.hex, "#000000");
        // Arriba a la izquierda es blanco.
        selector.set_square(0.0, 0.0);
        assert_eq!(selector.rgb(), (255, 255, 255));
        // Fuera del cuadro se recorta, no se desborda.
        selector.set_square(5.0, -3.0);
        assert_eq!((selector.saturation, selector.value), (1.0, 1.0));
        selector.set_hue(1.0 / 3.0);
        assert_eq!(selector.rgb(), (0, 255, 0));
        selector.set_hue(7.0);
        assert_eq!(selector.hue, 360.0);
    }

    #[test]
    fn el_campo_hexadecimal_acepta_solo_hex_y_mueve_el_selector() {
        let mut selector = CustomPicker::from_rgb((0, 0, 0));
        selector.hex.clear();
        selector.type_hex("#ff");
        assert_eq!(selector.hex, "#FF");
        assert_eq!(selector.parsed(), None);
        // Los caracteres que no son hexadecimales se ignoran; el # solo al principio.
        selector.type_hex("g#zc");
        assert_eq!(selector.hex, "#FFC");
        selector.type_hex("c00000");
        assert_eq!(selector.hex, "#FFCC00", "no pasa de siete caracteres");
        assert_eq!(selector.parsed(), Some((255, 204, 0)));
        assert_eq!(selector.rgb(), (255, 204, 0));
        selector.backspace();
        assert_eq!(selector.hex, "#FFCC0");
        assert_eq!(selector.parsed(), None);
        // También sin el #.
        let mut sin = CustomPicker::from_rgb((0, 0, 0));
        sin.hex.clear();
        sin.type_hex("3366ff");
        assert_eq!(sin.parsed(), Some((0x33, 0x66, 0xFF)));
    }

    #[test]
    fn un_gris_conserva_el_tono_que_habia() {
        let mut selector = CustomPicker::from_rgb((0, 0, 255));
        let tono = selector.hue;
        selector.hex.clear();
        selector.type_hex("808080");
        assert_eq!(selector.rgb(), (128, 128, 128));
        assert_eq!(selector.hue, tono);
    }

    #[test]
    fn las_zonas_del_selector_no_se_pisan_y_caben_en_la_ventana() {
        let geometria = picker_geometry(900.0, 640.0, 100.0, 250.0);
        assert!(geometria.rect.0 + geometria.rect.2 <= 640.0 - 8.0);
        for zona in [
            geometria.square,
            geometria.hue,
            geometria.preview,
            geometria.hex,
            geometria.back,
            geometria.apply,
        ] {
            assert!(zona.0 >= geometria.rect.0 && zona.1 >= geometria.rect.1);
            assert!(zona.0 + zona.2 <= geometria.rect.0 + geometria.rect.2);
            assert!(zona.1 + zona.3 <= geometria.rect.1 + geometria.rect.3);
        }
        // Cada centro cae en su propia zona.
        let centro = |z: Rect| (z.0 + z.2 / 2.0, z.1 + z.3 / 2.0);
        let (x, y) = centro(geometria.square);
        assert!(
            matches!(picker_hit(&geometria, x, y), PickerHit::Square(fx, fy)
            if (fx - 0.5).abs() < 0.01 && (fy - 0.5).abs() < 0.01)
        );
        let (x, y) = centro(geometria.hue);
        assert!(matches!(picker_hit(&geometria, x, y), PickerHit::Hue(_)));
        let (x, y) = centro(geometria.hex);
        assert_eq!(picker_hit(&geometria, x, y), PickerHit::Hex);
        let (x, y) = centro(geometria.back);
        assert_eq!(picker_hit(&geometria, x, y), PickerHit::Back);
        let (x, y) = centro(geometria.apply);
        assert_eq!(picker_hit(&geometria, x, y), PickerHit::Apply);
        let (x, y) = centro(geometria.preview);
        assert_eq!(picker_hit(&geometria, x, y), PickerHit::Panel);
        assert_eq!(picker_hit(&geometria, 1.0, 1.0), PickerHit::Outside);
    }
}
