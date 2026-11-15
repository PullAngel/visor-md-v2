//! Título provisional de un documento nuevo. Mientras no se guarda, la pestaña
//! no dice «sin título»: dice de qué trata, tomando lo primero en jerarquía del
//! texto. Al guardar, manda el nombre del archivo.

/// Marca de lo que no está guardado, en pestañas y cabeceras de panel.
pub(crate) const UNSAVED_MARK: &str = "•";

/// Nombre que se muestra cuando el texto no da para un título.
pub(crate) const UNTITLED_NAME: &str = "sin título";

/// Cuántas líneas se miran. Un título que aparece más abajo no es «lo
/// primero» y recorrer un documento enorme en cada cuadro sería un costo.
const SCAN_LINES: usize = 200;
const MAX_TITLE_CHARS: usize = 40;

/// Título provisional: el primer título de nivel 1; si no hay, el primero de
/// menor nivel; si no hay títulos, la primera línea con texto. `None` si el
/// documento no tiene nada que decir.
pub(crate) fn provisional_title<I, S>(lines: I) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut best_heading: Option<(usize, String)> = None;
    let mut first_text: Option<String> = None;
    let mut in_fence = false;
    let mut in_front_matter = false;
    for (index, line) in lines.into_iter().take(SCAN_LINES).enumerate() {
        let line = line.as_ref().trim();
        if index == 0 && line == "---" {
            in_front_matter = true;
            continue;
        }
        if in_front_matter {
            if line == "---" || line == "..." {
                in_front_matter = false;
            }
            continue;
        }
        if line.starts_with("```") || line.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence || line.is_empty() {
            continue;
        }
        if let Some((level, text)) = heading(line) {
            let text = clean(text);
            if text.is_empty() {
                continue;
            }
            if level == 1 {
                return Some(shorten(&text));
            }
            if best_heading
                .as_ref()
                .is_none_or(|(best_level, _)| level < *best_level)
            {
                best_heading = Some((level, text));
            }
        } else if first_text.is_none() {
            let text = clean(strip_block_marker(line));
            if !text.is_empty() {
                first_text = Some(text);
            }
        }
    }
    best_heading
        .map(|(_, text)| text)
        .or(first_text)
        .map(|text| shorten(&text))
}

/// Nivel y texto de una línea `# Título`, o `None` si no lo es.
fn heading(line: &str) -> Option<(usize, &str)> {
    let hashes = line.chars().take_while(|c| *c == '#').count();
    if !(1..=6).contains(&hashes) {
        return None;
    }
    let rest = &line[hashes..];
    // Una línea de solo `#` es un título vacío.
    if rest.is_empty() {
        return Some((hashes, ""));
    }
    // `#etiqueta` sin espacio no es un título.
    rest.starts_with([' ', '\t'])
        .then(|| (hashes, rest.trim().trim_end_matches('#').trim()))
}

/// Quita el marcador de cita, lista o tarea del comienzo de una línea.
fn strip_block_marker(line: &str) -> &str {
    let mut text = line.trim_start_matches('>').trim_start();
    for marker in ["- [ ] ", "- [x] ", "- [X] ", "- ", "* ", "+ "] {
        if let Some(rest) = text.strip_prefix(marker) {
            text = rest;
            break;
        }
    }
    text.trim()
}

/// Texto plano de un fragmento Markdown: sin énfasis, código ni enlaces.
fn clean(text: &str) -> String {
    let mut output = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '*' | '_' | '`' | '~' | '=' => {}
            '[' => {
                // [texto](destino) y [[nota]] conservan el texto.
            }
            ']' => {
                // Descarta «(destino)» que sigue a un enlace.
                if chars.peek() == Some(&'(') {
                    for next in chars.by_ref() {
                        if next == ')' {
                            break;
                        }
                    }
                }
            }
            other => output.push(other),
        }
    }
    output.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn shorten(text: &str) -> String {
    let mut chars = text.chars();
    let head: String = chars.by_ref().take(MAX_TITLE_CHARS).collect();
    if chars.next().is_some() {
        format!("{}…", head.trim_end())
    } else {
        head
    }
}

/// Nombre de archivo sugerido al guardar un documento nuevo: el título sin
/// los caracteres que Windows no admite en un nombre.
pub(crate) fn suggested_file_stem(title: Option<&str>) -> String {
    let cleaned: String = title
        .unwrap_or("")
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' | '…' => ' ',
            c if c.is_control() => ' ',
            c => c,
        })
        .collect();
    let cleaned = cleaned
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(['.', ' '])
        .to_string();
    if cleaned.is_empty() {
        "nota".to_string()
    } else {
        cleaned
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn titulo(texto: &str) -> Option<String> {
        provisional_title(texto.lines())
    }

    #[test]
    fn el_primer_titulo_de_nivel_uno_manda() {
        assert_eq!(
            titulo("intro\n\n## Parte\n\n# Apuntes de biología\n"),
            Some("Apuntes de biología".into())
        );
    }

    #[test]
    fn sin_nivel_uno_se_usa_el_titulo_de_menor_nivel_que_aparece_primero() {
        assert_eq!(
            titulo("### Detalle\n## Resumen\n## Otro\n"),
            Some("Resumen".into())
        );
    }

    #[test]
    fn sin_titulos_se_usa_la_primera_linea_con_texto() {
        assert_eq!(
            titulo("\n\n> **Comprar** pan y leche\nmás\n"),
            Some("Comprar pan y leche".into())
        );
        assert_eq!(titulo("- [ ] Llamar a Ana"), Some("Llamar a Ana".into()));
    }

    #[test]
    fn se_ignoran_front_matter_codigo_y_etiquetas() {
        assert_eq!(titulo("---\ntitle: x\n---\n# Real\n"), Some("Real".into()));
        assert_eq!(
            titulo("```\n# no es título\n```\nHola mundo\n"),
            Some("Hola mundo".into())
        );
        // `#etiqueta` sin espacio es texto, no un título.
        assert_eq!(titulo("#etiqueta\n"), Some("#etiqueta".into()));
    }

    #[test]
    fn limpia_enlaces_y_enfasis_y_acorta() {
        assert_eq!(
            titulo("# Ver [la guía](https://x.org) y `código`"),
            Some("Ver la guía y código".into())
        );
        assert_eq!(titulo("# [[Nota enlazada]]"), Some("Nota enlazada".into()));
        let largo = format!("# {}", "palabra ".repeat(20));
        let resultado = titulo(&largo).expect("hay título");
        assert!(resultado.chars().count() <= MAX_TITLE_CHARS + 1);
        assert!(resultado.ends_with('…'));
    }

    #[test]
    fn un_documento_vacio_o_en_blanco_no_tiene_titulo() {
        assert_eq!(titulo(""), None);
        assert_eq!(titulo("\n   \n\t\n"), None);
        assert_eq!(titulo("#\n##  \n"), None);
    }

    #[test]
    fn el_nombre_sugerido_no_lleva_caracteres_invalidos() {
        assert_eq!(
            suggested_file_stem(Some("¿Qué es esto? / A:B")),
            "¿Qué es esto A B"
        );
        assert_eq!(suggested_file_stem(Some("...")), "nota");
        assert_eq!(suggested_file_stem(None), "nota");
        assert_eq!(suggested_file_stem(Some("Resumen…")), "Resumen");
    }
}
