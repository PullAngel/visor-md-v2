//! Prueba de lenguaje (D-25): los textos de la interfaz usan el glosario del
//! proyecto y el voseo. Mira las etiquetas de las acciones, los textos de los
//! diálogos y todo lo que pasa por `set_notice`; los registros (`self.log`) no
//! cuentan.

use super::{APP_ACTIONS, ContextAction};

/// Términos retirados y lo que se dice en su lugar.
const PROHIBIDOS: &[(&str, &str)] = &[
    ("workspace", "carpeta de apuntes"),
    ("espacio de trabajo", "carpeta de apuntes"),
    ("bóveda", "carpeta de apuntes / enlace a otra nota"),
    ("carpeta de trabajo", "carpeta de apuntes"),
    ("backlink", "notas que enlazan aquí"),
    ("callout", "recuadro"),
    ("wikilink", "enlace a otra nota"),
    ("de forma atómica", "guardado"),
    ("guardado atómico", "guardado"),
    ("tsv", "copiar para hoja de cálculo"),
    ("encabezado", "título / subtítulo"),
    ("recuperación local", "recuperación"),
    ("recuperaciones locales", "recuperaciones"),
];

/// Imperativos de tuteo. Solo cuentan al empezar una frase o después de un
/// signo, que es donde un imperativo aparece en un aviso o un botón.
const TUTEO: &[&str] = &[
    "escribe",
    "elige",
    "pulsa",
    "selecciona",
    "presiona",
    "puedes",
    "tienes",
    "edita",
    "ingresa",
    "introduce",
    "haz",
    "abre",
    "usa",
    "espera",
    "activa",
    "desactiva",
    "enfoca",
    "cierra",
    "vuelve",
    "intenta",
    "prueba",
    "revisa",
    "quita",
    "pega",
    "guarda",
];

fn literales(codigo: &str) -> Vec<String> {
    let mut salida = Vec::new();
    let bytes: Vec<char> = codigo.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        // Comentarios de línea: no son texto de la interfaz.
        if bytes[i] == '/' && bytes.get(i + 1) == Some(&'/') {
            while i < bytes.len() && bytes[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if bytes[i] == '"' {
            let mut texto = String::new();
            i += 1;
            while i < bytes.len() && bytes[i] != '"' {
                if bytes[i] == '\\' {
                    i += 1;
                }
                if let Some(c) = bytes.get(i) {
                    texto.push(*c);
                }
                i += 1;
            }
            salida.push(texto);
        }
        i += 1;
    }
    salida
}

/// Los textos que se muestran al usuario en `bloque`.
fn avisos(codigo: &str) -> Vec<String> {
    let mut salida = Vec::new();
    for (inicio, _) in codigo.match_indices("set_notice(") {
        let mut profundidad = 1;
        let mut fin = inicio + "set_notice(".len();
        let resto = &codigo[fin..];
        for (indice, c) in resto.char_indices() {
            match c {
                '(' => profundidad += 1,
                ')' => {
                    profundidad -= 1;
                    if profundidad == 0 {
                        fin += indice;
                        break;
                    }
                }
                _ => {}
            }
        }
        salida.extend(literales(&codigo[inicio + "set_notice(".len()..fin]));
    }
    salida
}

/// Cada frase del texto, para mirar el principio de cada una.
fn frases(texto: &str) -> impl Iterator<Item = &str> {
    texto
        .split(['.', ':', ';', ',', '·', '\n'])
        .map(str::trim)
        .filter(|frase| !frase.is_empty())
}

fn problemas(origen: &str, texto: &str) -> Vec<String> {
    let mut encontrados = Vec::new();
    let minusculas = texto.to_lowercase();
    for (prohibido, en_su_lugar) in PROHIBIDOS {
        // `tsv` solo cuenta como palabra suelta.
        let aparece = if *prohibido == "tsv" {
            minusculas
                .split(|c: char| !c.is_alphanumeric())
                .any(|palabra| palabra == "tsv")
        } else {
            minusculas.contains(prohibido)
        };
        if aparece {
            encontrados.push(format!(
                "{origen}: «{texto}» usa «{prohibido}»; decí «{en_su_lugar}»"
            ));
        }
    }
    for frase in frases(&minusculas) {
        let primera = frase.split_whitespace().next().unwrap_or("");
        if TUTEO.contains(&primera) {
            encontrados.push(format!(
                "{origen}: «{texto}» tutea con «{primera}»; el texto usa voseo"
            ));
        }
    }
    encontrados
}

/// El código con saltos de línea `\n` y sin su módulo de pruebas.
fn sin_pruebas(codigo: &str) -> String {
    let codigo = codigo.replace("\r\n", "\n");
    // `mod visual_tests;` también lleva `#[cfg(test)]`: solo cuentan los módulos con cuerpo.
    let fin = ["pruebas {", "pruebas_inline {", "tests {"]
        .iter()
        .filter_map(|nombre| codigo.find(&format!("#[cfg(test)]\nmod {nombre}")))
        .min();
    match fin {
        Some(fin) => codigo[..fin].to_string(),
        None => codigo,
    }
}

#[test]
fn las_acciones_de_la_barra_y_la_paleta_usan_el_glosario_y_el_voseo() {
    let mut todos = Vec::new();
    for accion in APP_ACTIONS {
        todos.extend(problemas("acción", accion.label()));
    }
    // Las etiquetas de las acciones del menú contextual.
    let principal = include_str!("main.rs").replace("\r\n", "\n");
    let inicio = principal
        .find("impl ContextAction {\n    fn label")
        .expect("la etiqueta de ContextAction está en main.rs");
    let bloque = &principal[inicio..];
    let bloque = &bloque[..bloque.find("fn source_markdown").unwrap_or(bloque.len())];
    for texto in literales(bloque) {
        todos.extend(problemas("menú contextual", &texto));
    }
    let _ = ContextAction::Paste.label();
    assert!(todos.is_empty(), "\n{}", todos.join("\n"));
}

#[test]
fn los_dialogos_usan_el_glosario_y_el_voseo() {
    let codigo = sin_pruebas(include_str!("dialog.rs"));
    let codigo = codigo.as_str();
    let mut todos = Vec::new();
    for texto in literales(codigo) {
        todos.extend(problemas("diálogo", &texto));
    }
    assert!(todos.is_empty(), "\n{}", todos.join("\n"));
}

#[test]
fn los_avisos_usan_el_glosario_y_el_voseo() {
    let mut todos = Vec::new();
    for (archivo, codigo) in [
        ("main.rs", sin_pruebas(include_str!("main.rs"))),
        ("panel_list.rs", sin_pruebas(include_str!("panel_list.rs"))),
    ] {
        let textos = avisos(&codigo);
        assert!(
            archivo != "main.rs" || textos.len() > 150,
            "no se encontraron los avisos"
        );
        for texto in textos {
            todos.extend(problemas(&format!("aviso en {archivo}"), &texto));
        }
    }
    assert!(todos.is_empty(), "\n{}", todos.join("\n"));
}

#[test]
fn la_prueba_de_lenguaje_detecta_lo_que_debe() {
    assert_eq!(
        problemas("x", "Abrí una carpeta de apuntes"),
        Vec::<String>::new()
    );
    assert_eq!(problemas("x", "Elegí un recuadro"), Vec::<String>::new());
    assert!(!problemas("x", "Insertar callout").is_empty());
    assert!(!problemas("x", "Abre una carpeta de apuntes").is_empty());
    assert!(!problemas("x", "No se pudo: elige otra").is_empty());
    assert!(!problemas("x", "Copiar como TSV").is_empty());
    // Un verbo en tercera persona dentro de una frase no se confunde.
    assert!(problemas("x", "Visor MD guarda una copia").is_empty());
    assert!(problemas("x", "La recuperación sigue activa").is_empty());
}

/// Cualquier literal de la interfaz con un término retirado, no solo los de
/// avisos: títulos de paneles, textos de diálogos de archivo, estados.
#[test]
fn ningun_texto_de_la_interfaz_usa_un_termino_retirado() {
    let mut todos = Vec::new();
    for (archivo, codigo) in [
        ("main.rs", sin_pruebas(include_str!("main.rs"))),
        ("panel_list.rs", sin_pruebas(include_str!("panel_list.rs"))),
        ("menu.rs", sin_pruebas(include_str!("menu.rs"))),
    ] {
        for texto in literales(&codigo) {
            // Los registros de depuración no son interfaz.
            if texto.starts_with('[') {
                continue;
            }
            // Lo que va entre llaves es el nombre de una variable, no texto.
            let mut visible = String::new();
            let mut dentro = false;
            for c in texto.chars() {
                match c {
                    '{' => dentro = true,
                    '}' => dentro = false,
                    _ if !dentro => visible.push(c),
                    _ => {}
                }
            }
            let minusculas = visible.to_lowercase();
            for (prohibido, en_su_lugar) in PROHIBIDOS {
                let aparece = if *prohibido == "tsv" {
                    minusculas
                        .split(|c: char| !c.is_alphanumeric())
                        .any(|palabra| palabra == "tsv")
                } else {
                    minusculas.contains(prohibido)
                };
                if aparece {
                    todos.push(format!(
                        "{archivo}: «{texto}» usa «{prohibido}»; decí «{en_su_lugar}»"
                    ));
                }
            }
        }
    }
    assert!(todos.is_empty(), "\n{}", todos.join("\n"));
}
