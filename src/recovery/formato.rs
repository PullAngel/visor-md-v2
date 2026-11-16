//! Formato de una copia de recuperación (`.vmdr`).
//!
//! Un archivo, una cabecera de texto acotada y el cuerpo con su largo y su
//! suma de verificación. Así una copia truncada o dañada se reconoce antes de
//! restaurarla, y la lista de copias se arma leyendo solo las cabeceras. El
//! contenido es un dato no confiable: nunca se interpreta como código ni como
//! ruta de destino.

/// Primera línea de todo archivo. La versión va al final.
const MAGIC: &str = "VISORMD-RECOVERY";
const VERSION: u32 = 1;
/// La cabecera nunca supera esto; un archivo con una más larga se rechaza.
pub(crate) const MAX_HEADER_BYTES: usize = 4096;

const MAX_ORIGIN_CHARS: usize = 1024;
const MAX_TITLE_CHARS: usize = 80;
const MAX_BASE_CHARS: usize = 64;

/// Lo que se sabe de un documento sin guardar, además de su texto.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct EntryMeta {
    /// Archivo del que viene el documento; vacío si nació nuevo. Es solo
    /// informativo: nunca es un destino de escritura.
    pub(crate) origin_path: String,
    /// Huella del archivo de origen cuando se abrió o guardó (`largo:fecha`);
    /// vacía si no hay origen. Permite saber si cambió en disco.
    pub(crate) base: String,
    pub(crate) title: String,
    /// Segundos desde 1970 de la última copia.
    pub(crate) saved_at: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DecodeError {
    /// No empieza con la firma de Visor MD.
    NotARecovery,
    /// La versión es más nueva que la que entiende esta aplicación: se
    /// conserva sin tocarla.
    UnsupportedVersion(u32),
    BadHeader,
    /// El cuerpo no mide lo que dice la cabecera: el archivo está truncado.
    Truncated,
    BadChecksum,
    TooLarge,
    InvalidUtf8,
}

/// Los datos de la cabecera de un archivo ya leída.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Header {
    pub(crate) meta: EntryMeta,
    pub(crate) body_len: u64,
    pub(crate) body_crc: u32,
    /// Dónde empieza el cuerpo.
    pub(crate) body_start: usize,
}

/// CRC-32 (IEEE). Detecta cortes y daños; no es una defensa contra alguien
/// que modifique el archivo a propósito.
pub(crate) fn crc32(data: &[u8]) -> u32 {
    let mut crc = !0_u32;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xEDB8_8320 & 0_u32.wrapping_sub(crc & 1));
        }
    }
    !crc
}

/// Deja un valor en una sola línea y con el largo máximo: la cabecera no puede
/// partirse ni crecer por contenido del documento.
fn clean(value: &str, max_chars: usize) -> String {
    value
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .take(max_chars)
        .collect::<String>()
        .trim()
        .to_string()
}

pub(crate) fn encode(meta: &EntryMeta, body: &str) -> Vec<u8> {
    let mut bytes = format!(
        "{MAGIC} {VERSION}\norigin_path: {}\nbase: {}\ntitle: {}\nsaved_at: {}\nbody_len: {}\nbody_crc32: {:08x}\n\n",
        clean(&meta.origin_path, MAX_ORIGIN_CHARS),
        clean(&meta.base, MAX_BASE_CHARS),
        clean(&meta.title, MAX_TITLE_CHARS),
        meta.saved_at,
        body.len(),
        crc32(body.as_bytes()),
    )
    .into_bytes();
    bytes.extend_from_slice(body.as_bytes());
    bytes
}

/// Lee solo la cabecera. `bytes` puede ser el comienzo del archivo: alcanza
/// con los primeros `MAX_HEADER_BYTES + 2`.
pub(crate) fn decode_header(bytes: &[u8]) -> Result<Header, DecodeError> {
    let window = &bytes[..bytes.len().min(MAX_HEADER_BYTES + 2)];
    let end = window.windows(2).position(|pair| pair == b"\n\n").ok_or(
        if bytes.starts_with(MAGIC.as_bytes()) {
            DecodeError::BadHeader
        } else {
            DecodeError::NotARecovery
        },
    )?;
    let text = std::str::from_utf8(&window[..end]).map_err(|_| DecodeError::BadHeader)?;
    let mut lines = text.lines();
    let first = lines.next().ok_or(DecodeError::NotARecovery)?;
    let version = first
        .strip_prefix(MAGIC)
        .and_then(|rest| rest.strip_prefix(' '))
        .ok_or(DecodeError::NotARecovery)?
        .trim()
        .parse::<u32>()
        .map_err(|_| DecodeError::BadHeader)?;
    if version != VERSION {
        return Err(DecodeError::UnsupportedVersion(version));
    }
    let mut meta = EntryMeta::default();
    let (mut body_len, mut body_crc) = (None, None);
    for line in lines {
        // Una clave desconocida se ignora: una versión menor puede agregar.
        let Some((key, value)) = line.split_once(": ").or_else(|| line.split_once(':')) else {
            continue;
        };
        let value = value.trim();
        match key {
            "origin_path" => meta.origin_path = clean(value, MAX_ORIGIN_CHARS),
            "base" => meta.base = clean(value, MAX_BASE_CHARS),
            "title" => meta.title = clean(value, MAX_TITLE_CHARS),
            "saved_at" => meta.saved_at = value.parse().unwrap_or(0),
            "body_len" => body_len = value.parse::<u64>().ok(),
            "body_crc32" => body_crc = u32::from_str_radix(value, 16).ok(),
            _ => {}
        }
    }
    Ok(Header {
        meta,
        body_len: body_len.ok_or(DecodeError::BadHeader)?,
        body_crc: body_crc.ok_or(DecodeError::BadHeader)?,
        body_start: end + 2,
    })
}

/// Lee todo el archivo. `limit` es el largo máximo del cuerpo (el mismo que
/// rige para abrir un documento).
pub(crate) fn decode(bytes: &[u8], limit: u64) -> Result<(EntryMeta, String), DecodeError> {
    let header = decode_header(bytes)?;
    if header.body_len > limit {
        return Err(DecodeError::TooLarge);
    }
    let body = &bytes[header.body_start.min(bytes.len())..];
    if body.len() as u64 != header.body_len {
        return Err(DecodeError::Truncated);
    }
    if crc32(body) != header.body_crc {
        return Err(DecodeError::BadChecksum);
    }
    let text = String::from_utf8(body.to_vec()).map_err(|_| DecodeError::InvalidUtf8)?;
    Ok((header.meta, text))
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn meta() -> EntryMeta {
        EntryMeta {
            origin_path: r"C:\notas\biología.md".into(),
            base: "120:1794773108".into(),
            title: "Apuntes de biología".into(),
            saved_at: 1_794_773_108,
        }
    }

    #[test]
    fn el_crc32_coincide_con_el_valor_conocido() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(crc32(b""), 0);
    }

    #[test]
    fn una_copia_se_codifica_y_se_lee_de_vuelta() {
        let texto = "# Título\n\ncon acentos y 日本語\n";
        let bytes = encode(&meta(), texto);
        let (leida, cuerpo) = decode(&bytes, 1 << 20).expect("la copia es válida");
        assert_eq!(leida, meta());
        assert_eq!(cuerpo, texto);
        // La cabecera sola alcanza para la lista.
        let cabecera = decode_header(&bytes[..bytes.len().min(MAX_HEADER_BYTES + 2)]).unwrap();
        assert_eq!(cabecera.meta, meta());
        assert_eq!(cabecera.body_len, texto.len() as u64);
        // Un cuerpo vacío también es válido.
        assert_eq!(decode(&encode(&meta(), ""), 10).unwrap().1, "");
    }

    #[test]
    fn un_cuerpo_cortado_o_alterado_no_se_restaura() {
        let bytes = encode(&meta(), "contenido importante");
        assert_eq!(
            decode(&bytes[..bytes.len() - 3], 1 << 20).unwrap_err(),
            DecodeError::Truncated
        );
        let mut alterado = bytes.clone();
        let ultimo = alterado.len() - 1;
        alterado[ultimo] ^= 0x01;
        assert_eq!(
            decode(&alterado, 1 << 20).unwrap_err(),
            DecodeError::BadChecksum
        );
        // Agregar bytes de más tampoco pasa.
        let mut largo = bytes.clone();
        largo.extend_from_slice(b"x");
        assert_eq!(decode(&largo, 1 << 20).unwrap_err(), DecodeError::Truncated);
        // El límite del cuerpo se respeta antes de leerlo.
        assert_eq!(decode(&bytes, 5).unwrap_err(), DecodeError::TooLarge);
    }

    #[test]
    fn lo_que_no_es_una_copia_o_es_de_otra_version_se_distingue() {
        assert_eq!(
            decode_header(b"cualquier otra cosa\n\nmas").unwrap_err(),
            DecodeError::NotARecovery
        );
        assert_eq!(decode_header(b"").unwrap_err(), DecodeError::NotARecovery);
        let futura = b"VISORMD-RECOVERY 7\nbody_len: 0\nbody_crc32: 0\n\n";
        assert_eq!(
            decode_header(futura).unwrap_err(),
            DecodeError::UnsupportedVersion(7)
        );
        // Sin el fin de cabecera, o con una cabecera sin largo, es dañada.
        assert_eq!(
            decode_header(b"VISORMD-RECOVERY 1\ntitle: x").unwrap_err(),
            DecodeError::BadHeader
        );
        assert_eq!(
            decode_header(b"VISORMD-RECOVERY 1\ntitle: x\n\n").unwrap_err(),
            DecodeError::BadHeader
        );
    }

    #[test]
    fn una_cabecera_enorme_se_rechaza_y_los_valores_se_acotan() {
        let mut enorme = b"VISORMD-RECOVERY 1\n".to_vec();
        enorme.extend(std::iter::repeat_n(b'a', MAX_HEADER_BYTES * 2));
        assert!(decode_header(&enorme).is_err());

        let hostil = EntryMeta {
            title: format!("uno\ndos\r\ntres{}", "x".repeat(500)),
            origin_path: "a\nb".into(),
            ..EntryMeta::default()
        };
        let bytes = encode(&hostil, "x");
        let (leida, _) = decode(&bytes, 10).expect("sigue siendo válida");
        assert!(!leida.title.contains(['\n', '\r']));
        assert!(leida.title.chars().count() <= MAX_TITLE_CHARS);
        assert_eq!(leida.origin_path, "a b");
    }

    #[test]
    fn una_clave_desconocida_se_ignora() {
        let mut bytes = encode(&meta(), "hola");
        let extra = b"futuro: valor\n";
        let pos = bytes.windows(2).position(|p| p == b"\n\n").unwrap() + 1;
        bytes.splice(pos..pos, extra.iter().copied());
        assert_eq!(decode(&bytes, 100).unwrap().1, "hola");
    }
}
