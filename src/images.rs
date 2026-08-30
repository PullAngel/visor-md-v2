//! Lectura acotada de PNG locales ya resueltos por la VFS.
//!
//! Esta capa no decide permisos ni rutas. Recibe un archivo contenido después
//! de una acción explícita, valida el encabezado antes de pedir memoria al
//! decodificador y devuelve píxeles premultiplicados para el renderer.

use std::fmt;
use std::fs::File;
use std::io::{Read, Take};
use std::path::Path;
use tiny_skia::Pixmap;

pub(crate) const MAX_PNG_BYTES: u64 = 8 * 1024 * 1024;
pub(crate) const MAX_PNG_DIMENSION: u32 = 8_192;
pub(crate) const MAX_PNG_PIXELS: u64 = 16 * 1024 * 1024;

const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ImageError {
    NotFile,
    TooLarge,
    NotPng,
    InvalidHeader,
    DimensionsExceeded,
    Io(std::io::ErrorKind),
    Decode,
}

impl fmt::Display for ImageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::NotFile => "el recurso no es un archivo normal",
            Self::TooLarge => "la imagen supera el límite de 8 MiB comprimidos",
            Self::NotPng => "solo se permiten imágenes PNG locales",
            Self::InvalidHeader => "el encabezado PNG es inválido",
            Self::DimensionsExceeded => {
                "la imagen supera el límite de dimensiones o memoria descomprimida"
            }
            Self::Io(_) => "no se pudo leer la imagen local",
            Self::Decode => "el PNG no pudo decodificarse de forma segura",
        };
        formatter.write_str(message)
    }
}

pub(crate) fn load_local_png(path: &Path) -> Result<Pixmap, ImageError> {
    let file = File::open(path).map_err(|error| ImageError::Io(error.kind()))?;
    let metadata = file
        .metadata()
        .map_err(|error| ImageError::Io(error.kind()))?;
    if !metadata.is_file() {
        return Err(ImageError::NotFile);
    }
    if metadata.len() > MAX_PNG_BYTES {
        return Err(ImageError::TooLarge);
    }
    let bytes = read_bounded(file.take(MAX_PNG_BYTES + 1))?;
    if bytes.len() as u64 > MAX_PNG_BYTES {
        return Err(ImageError::TooLarge);
    }
    validate_png_header(&bytes)?;
    Pixmap::decode_png(&bytes).map_err(|_| ImageError::Decode)
}

fn read_bounded(mut source: Take<File>) -> Result<Vec<u8>, ImageError> {
    let mut bytes = Vec::new();
    source
        .read_to_end(&mut bytes)
        .map_err(|error| ImageError::Io(error.kind()))?;
    Ok(bytes)
}

fn validate_png_header(bytes: &[u8]) -> Result<(u32, u32), ImageError> {
    if bytes.get(..8) != Some(PNG_SIGNATURE) {
        return Err(ImageError::NotPng);
    }
    if bytes.len() < 24 || &bytes[12..16] != b"IHDR" {
        return Err(ImageError::InvalidHeader);
    }
    let header_len = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
    if header_len != 13 {
        return Err(ImageError::InvalidHeader);
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    let pixels = u64::from(width).checked_mul(u64::from(height));
    if width == 0
        || height == 0
        || width > MAX_PNG_DIMENSION
        || height > MAX_PNG_DIMENSION
        || pixels.is_none_or(|pixels| pixels > MAX_PNG_PIXELS)
    {
        return Err(ImageError::DimensionsExceeded);
    }
    Ok((width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png_header(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = Vec::from(PNG_SIGNATURE.as_slice());
        bytes.extend_from_slice(&13_u32.to_be_bytes());
        bytes.extend_from_slice(b"IHDR");
        bytes.extend_from_slice(&width.to_be_bytes());
        bytes.extend_from_slice(&height.to_be_bytes());
        bytes
    }

    #[test]
    fn rechaza_dimensiones_que_agotarian_memoria_antes_de_decodificar() {
        assert_eq!(
            validate_png_header(&png_header(8_192, 8_192)),
            Err(ImageError::DimensionsExceeded)
        );
        assert_eq!(
            validate_png_header(&png_header(16_384, 1)),
            Err(ImageError::DimensionsExceeded)
        );
    }

    #[test]
    fn distingue_un_archivo_que_solo_finge_ser_png() {
        assert_eq!(
            validate_png_header(b"no es una imagen"),
            Err(ImageError::NotPng)
        );
    }

    #[test]
    fn acepta_un_encabezado_con_presupuesto_valido() {
        assert_eq!(validate_png_header(&png_header(640, 480)), Ok((640, 480)));
    }

    #[test]
    fn decodifica_un_png_pequeno_dentro_del_presupuesto() {
        let source = Pixmap::new(3, 2).unwrap();
        let encoded = source.encode_png().unwrap();

        assert_eq!(validate_png_header(&encoded), Ok((3, 2)));
        let decoded = Pixmap::decode_png(&encoded).unwrap();
        assert_eq!((decoded.width(), decoded.height()), (3, 2));
    }

    #[test]
    fn lee_un_archivo_png_acotado_sin_confiar_en_la_extension() {
        let path = std::env::temp_dir().join(format!(
            "visor-md-image-{}-{}.bin",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let encoded = Pixmap::new(2, 4).unwrap().encode_png().unwrap();
        std::fs::write(&path, encoded).unwrap();

        let decoded = load_local_png(&path).unwrap();
        assert_eq!((decoded.width(), decoded.height()), (2, 4));
        let _ = std::fs::remove_file(path);
    }
}
