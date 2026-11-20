//! Formato de las anotaciones de estudio (ADR-39, diseño D-45).
//!
//! Las anotaciones viven en un único comentario HTML al final del `.md`:
//!
//! ```text
//! <!-- visor-md:anotaciones v1
//! Frase fija que avisa que no es parte del texto.
//! {"id":"a1","tipo":"subrayado",...}
//! -->
//! ```
//!
//! Este módulo es el formato puro: modelo, lector y escritor. No toca la
//! interfaz, el disco ni el texto del documento. El lector es propio (JSON de
//! un solo nivel, cadenas y enteros), recorre cada línea una vez y no recursa.
//! Nada de lo que lee se ejecuta, abre o consulta: son datos.

// Hasta que la tarea 14.3 lo conecte con el documento, solo lo usan las pruebas.
#![allow(dead_code)]

/// Frase fija de la segunda línea. Avisa a quien abra el archivo en otro
/// lector (o a una IA) que el bloque no es parte del texto.
pub(crate) const DESCRIPCION: &str = "Anotaciones personales de estudio hechas con Visor MD (subrayados y marcas de libro). No forman parte del texto del documento; cada línea cita el fragmento al que se refiere.";

pub(crate) const VERSION_ACTUAL: u32 = 1;
pub(crate) const APERTURA: &str = "<!-- visor-md:anotaciones v";
pub(crate) const CIERRE: &str = "-->";

/// Tamaño máximo del bloque; más grande, queda inerte.
pub(crate) const MAX_BLOQUE_BYTES: usize = 1024 * 1024;
/// Registros máximos; más, el bloque queda inerte.
pub(crate) const MAX_REGISTROS: usize = 5_000;
/// Una línea más larga pasa a «no entendida».
pub(crate) const MAX_LINEA_BYTES: usize = 16 * 1024;
/// Contexto que se guarda antes y después de la cita.
pub(crate) const MAX_CONTEXTO: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Tipo {
    Subrayado,
    Marca,
    Leyenda,
    Otro(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Color {
    Amarillo,
    Verde,
    Azul,
    Rosa,
    Violeta,
    Otro(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Forma {
    Barra,
    Cinta,
    Punto,
    Bandera,
    Otro(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Valor {
    Texto(String),
    Entero(i64),
}

impl Tipo {
    fn de_texto(texto: &str) -> Self {
        match texto {
            "subrayado" => Self::Subrayado,
            "marca" => Self::Marca,
            "leyenda" => Self::Leyenda,
            otro => Self::Otro(otro.to_string()),
        }
    }

    fn texto(&self) -> &str {
        match self {
            Self::Subrayado => "subrayado",
            Self::Marca => "marca",
            Self::Leyenda => "leyenda",
            Self::Otro(otro) => otro,
        }
    }

    /// Orden entre registros con la misma posición: la marca va antes.
    fn rango(&self) -> u8 {
        match self {
            Self::Marca => 0,
            Self::Subrayado => 1,
            Self::Leyenda => 2,
            Self::Otro(_) => 3,
        }
    }
}

impl Color {
    fn de_texto(texto: &str) -> Self {
        match texto {
            "amarillo" => Self::Amarillo,
            "verde" => Self::Verde,
            "azul" => Self::Azul,
            "rosa" => Self::Rosa,
            "violeta" => Self::Violeta,
            otro => Self::Otro(otro.to_string()),
        }
    }

    pub(crate) fn texto(&self) -> &str {
        match self {
            Self::Amarillo => "amarillo",
            Self::Verde => "verde",
            Self::Azul => "azul",
            Self::Rosa => "rosa",
            Self::Violeta => "violeta",
            Self::Otro(otro) => otro,
        }
    }

    /// Posición en la paleta; los colores desconocidos van al final.
    fn rango(&self) -> u8 {
        match self {
            Self::Amarillo => 0,
            Self::Verde => 1,
            Self::Azul => 2,
            Self::Rosa => 3,
            Self::Violeta => 4,
            Self::Otro(_) => 5,
        }
    }
}

impl Forma {
    fn de_texto(texto: &str) -> Self {
        match texto {
            "barra" => Self::Barra,
            "cinta" => Self::Cinta,
            "punto" => Self::Punto,
            "bandera" => Self::Bandera,
            otro => Self::Otro(otro.to_string()),
        }
    }

    fn texto(&self) -> &str {
        match self {
            Self::Barra => "barra",
            Self::Cinta => "cinta",
            Self::Punto => "punto",
            Self::Bandera => "bandera",
            Self::Otro(otro) => otro,
        }
    }
}

/// Una anotación. `pos`, `antes`, `cita` y `despues` se miden sobre el cuerpo
/// normalizado a LF, en caracteres Unicode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Registro {
    pub(crate) id: String,
    pub(crate) tipo: Tipo,
    pub(crate) color: Color,
    pub(crate) forma: Option<Forma>,
    pub(crate) pos: Option<u64>,
    pub(crate) antes: Option<String>,
    pub(crate) cita: Option<String>,
    pub(crate) despues: Option<String>,
    pub(crate) nota: Option<String>,
    /// Reservada para imágenes ancladas (futuro); v1 la conserva sin usarla.
    pub(crate) imagen: Option<String>,
    /// Claves que este programa no conoce, en el orden en que llegaron.
    pub(crate) extras: Vec<(String, Valor)>,
}

impl Registro {
    /// Número del identificador (`a12` → 12).
    pub(crate) fn numero_de_id(&self) -> Option<u64> {
        numero_de_id(&self.id)
    }
}

fn numero_de_id(id: &str) -> Option<u64> {
    let digitos = id.strip_prefix('a')?;
    if digitos.is_empty() || !digitos.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digitos.parse().ok()
}

/// Contenido entendido de un bloque.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Anotaciones {
    pub(crate) registros: Vec<Registro>,
    /// Líneas que no se entendieron (por ejemplo, marcas de conflicto de git);
    /// se conservan literales, al final.
    pub(crate) lineas_no_entendidas: Vec<String>,
}

impl Anotaciones {
    pub(crate) fn esta_vacio(&self) -> bool {
        self.registros.is_empty() && self.lineas_no_entendidas.is_empty()
    }

    /// Identificador para una anotación nueva: máximo + 1, nunca reutilizado.
    pub(crate) fn nuevo_id(&self) -> String {
        let maximo = self
            .registros
            .iter()
            .filter_map(Registro::numero_de_id)
            .max()
            .unwrap_or(0);
        format!("a{}", maximo.saturating_add(1))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Inerte {
    /// Versión mayor que la conocida: se conserva byte a byte.
    VersionNueva(u32),
    /// Excede un límite de tamaño o de cantidad.
    Limites,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Lectura {
    /// No es un bloque de anotaciones (se trata como contenido normal).
    NoEsBloque,
    /// Es un bloque, pero no se interpreta; quien lo guarda lo escribe idéntico.
    Inerte(Inerte),
    Entendido(Anotaciones),
}

/// Lee un bloque que empieza en `<!-- visor-md:anotaciones v` y termina en
/// `-->` (el tramo exacto; separar el cuerpo es tarea del documento).
pub(crate) fn leer(bloque: &str) -> Lectura {
    let bloque_recortado = bloque.trim_end_matches(['\n', '\r', ' ', '\t']);
    if !bloque_recortado.starts_with(APERTURA) {
        return Lectura::NoEsBloque;
    }
    // Las líneas se parten solo por `\n` o `\r\n`: U+2028 y compañía no cuentan.
    let mut lineas = bloque_recortado.split('\n').map(quitar_cr);
    let primera = lineas.next().unwrap_or("");
    let digitos = &primera[APERTURA.len()..];
    if digitos.is_empty() || !digitos.bytes().all(|b| b.is_ascii_digit()) {
        return Lectura::NoEsBloque;
    }
    let lineas: Vec<&str> = lineas.collect();
    // El cierre es la última línea; cualquier `-->` antes cierra el comentario
    // allí y lo que sigue ya no es parte del bloque.
    let Some((ultima, cuerpo)) = lineas.split_last() else {
        return Lectura::NoEsBloque;
    };
    if *ultima != CIERRE || cuerpo.iter().any(|linea| linea.contains(CIERRE)) {
        return Lectura::NoEsBloque;
    }
    let version: u32 = match digitos.parse() {
        Ok(version) => version,
        Err(_) => return Lectura::Inerte(Inerte::VersionNueva(u32::MAX)),
    };
    if version == 0 {
        return Lectura::NoEsBloque;
    }
    if version > VERSION_ACTUAL {
        return Lectura::Inerte(Inerte::VersionNueva(version));
    }
    if bloque.len() > MAX_BLOQUE_BYTES {
        return Lectura::Inerte(Inerte::Limites);
    }

    let mut cuerpo = cuerpo.iter();
    let mut pendientes: Vec<&str> = Vec::new();
    // La segunda línea es la descripción, salvo que ya sea un registro.
    if let Some(segunda) = cuerpo.next()
        && segunda.trim_start().starts_with('{')
    {
        pendientes.push(segunda);
    }
    pendientes.extend(cuerpo);
    let pendientes: Vec<&str> = pendientes
        .into_iter()
        .filter(|linea| !linea.trim().is_empty())
        .collect();
    if pendientes.len() > MAX_REGISTROS {
        return Lectura::Inerte(Inerte::Limites);
    }

    let mut anotaciones = Anotaciones::default();
    for linea in pendientes {
        let registro = if linea.len() > MAX_LINEA_BYTES {
            None
        } else {
            leer_registro(linea)
        };
        match registro {
            Some(registro) => anotaciones.registros.push(registro),
            None => anotaciones.lineas_no_entendidas.push(linea.to_string()),
        }
    }
    Lectura::Entendido(anotaciones)
}

fn quitar_cr(linea: &str) -> &str {
    linea.strip_suffix('\r').unwrap_or(linea)
}

/// Escribe el bloque, de `<!--` a `-->` sin salto final. `None` si no hay nada
/// que guardar: un bloque vacío no se escribe.
pub(crate) fn escribir(anotaciones: &Anotaciones, salto: &str) -> Option<String> {
    if anotaciones.esta_vacio() {
        return None;
    }
    let mut orden: Vec<&Registro> = anotaciones.registros.iter().collect();
    // Orden determinista (diseño 5.5); `sort_by` es estable, así que el orden
    // de llegada desempata lo que quede igual.
    orden.sort_by_key(|registro| clave_de_orden(registro));

    let mut salida = format!("{APERTURA}{VERSION_ACTUAL}{salto}{DESCRIPCION}{salto}");
    for registro in orden {
        escribir_registro(registro, &mut salida);
        salida.push_str(salto);
    }
    for linea in &anotaciones.lineas_no_entendidas {
        salida.push_str(linea);
        salida.push_str(salto);
    }
    salida.push_str(CIERRE);
    Some(salida)
}

type ClaveDeOrden = (u8, u8, u64, u8, u64);

fn clave_de_orden(registro: &Registro) -> ClaveDeOrden {
    let id = registro.numero_de_id().unwrap_or(u64::MAX);
    if registro.tipo == Tipo::Leyenda {
        // 1. Leyendas, por color en el orden de la paleta.
        (0, registro.color.rango(), 0, 0, id)
    } else {
        // 2. El resto por posición, luego tipo (marca antes que subrayado), luego id.
        (
            1,
            0,
            registro.pos.unwrap_or(u64::MAX),
            registro.tipo.rango(),
            id,
        )
    }
}

fn escribir_registro(registro: &Registro, salida: &mut String) {
    salida.push('{');
    clave_texto("id", &registro.id, salida, true);
    clave_texto("tipo", registro.tipo.texto(), salida, false);
    clave_texto("color", registro.color.texto(), salida, false);
    if let Some(forma) = &registro.forma {
        clave_texto("forma", forma.texto(), salida, false);
    }
    if let Some(pos) = registro.pos {
        coma_y_clave("pos", salida, false);
        salida.push_str(&pos.to_string());
    }
    if let Some(antes) = &registro.antes {
        let recorte = ultimos_caracteres(antes, MAX_CONTEXTO);
        clave_texto("antes", recorte, salida, false);
    }
    if let Some(cita) = &registro.cita {
        clave_texto("cita", cita, salida, false);
    }
    if let Some(despues) = &registro.despues {
        let recorte = primeros_caracteres(despues, MAX_CONTEXTO);
        clave_texto("despues", recorte, salida, false);
    }
    if let Some(nota) = &registro.nota {
        clave_texto("nota", nota, salida, false);
    }
    if let Some(imagen) = &registro.imagen {
        clave_texto("imagen", imagen, salida, false);
    }
    for (clave, valor) in &registro.extras {
        match valor {
            Valor::Texto(texto) => clave_texto(clave, texto, salida, false),
            Valor::Entero(numero) => {
                coma_y_clave(clave, salida, false);
                salida.push_str(&numero.to_string());
            }
        }
    }
    salida.push('}');
}

fn primeros_caracteres(texto: &str, cantidad: usize) -> &str {
    match texto.char_indices().nth(cantidad) {
        Some((indice, _)) => &texto[..indice],
        None => texto,
    }
}

fn ultimos_caracteres(texto: &str, cantidad: usize) -> &str {
    let total = texto.chars().count();
    if total <= cantidad {
        return texto;
    }
    match texto.char_indices().nth(total - cantidad) {
        Some((indice, _)) => &texto[indice..],
        None => texto,
    }
}

fn coma_y_clave(clave: &str, salida: &mut String, primera: bool) {
    if !primera {
        salida.push(',');
    }
    cadena_json(clave, salida);
    salida.push(':');
}

fn clave_texto(clave: &str, valor: &str, salida: &mut String, primera: bool) {
    coma_y_clave(clave, salida, primera);
    cadena_json(valor, salida);
}

/// Escribe una cadena JSON. Además de lo que exige JSON, escapa siempre `<` y
/// `>` (nada puede cerrar el comentario ni abrir otro) y los separadores de
/// línea de Unicode (U+2028, U+2029 y U+0085), que muchos programas tratan como
/// salto de línea y partirían el registro.
fn cadena_json(texto: &str, salida: &mut String) {
    salida.push('"');
    for caracter in texto.chars() {
        match caracter {
            '"' => salida.push_str("\\\""),
            '\\' => salida.push_str("\\\\"),
            '\n' => salida.push_str("\\n"),
            '\r' => salida.push_str("\\r"),
            '\t' => salida.push_str("\\t"),
            '<' | '>' | '\u{0}'..='\u{1f}' | '\u{7f}' | '\u{85}' | '\u{2028}' | '\u{2029}' => {
                salida.push_str(&format!("\\u{:04x}", caracter as u32));
            }
            otro => salida.push(otro),
        }
    }
    salida.push('"');
}

// ---------------------------------------------------------------------------
// Lector de un registro: objeto JSON plano de cadenas y enteros.
// ---------------------------------------------------------------------------

fn leer_registro(linea: &str) -> Option<Registro> {
    let campos = leer_objeto(linea)?;
    let mut id = None;
    let mut tipo = None;
    let mut color = None;
    let mut forma = None;
    let mut pos = None;
    let mut antes = None;
    let mut cita = None;
    let mut despues = None;
    let mut nota = None;
    let mut imagen = None;
    let mut extras = Vec::new();
    for (clave, valor) in campos {
        match (clave.as_str(), valor) {
            ("id", Valor::Texto(texto)) if id.is_none() => id = Some(texto),
            ("tipo", Valor::Texto(texto)) if tipo.is_none() => tipo = Some(Tipo::de_texto(&texto)),
            ("color", Valor::Texto(texto)) if color.is_none() => {
                color = Some(Color::de_texto(&texto));
            }
            ("forma", Valor::Texto(texto)) if forma.is_none() => {
                forma = Some(Forma::de_texto(&texto));
            }
            ("pos", Valor::Entero(numero)) if pos.is_none() => {
                pos = Some(u64::try_from(numero).ok()?)
            }
            ("antes", Valor::Texto(texto)) if antes.is_none() => antes = Some(texto),
            ("cita", Valor::Texto(texto)) if cita.is_none() => cita = Some(texto),
            ("despues", Valor::Texto(texto)) if despues.is_none() => despues = Some(texto),
            ("nota", Valor::Texto(texto)) if nota.is_none() => nota = Some(texto),
            ("imagen", Valor::Texto(texto)) if imagen.is_none() => imagen = Some(texto),
            // Una clave conocida repetida o con otro tipo de valor: línea dañada.
            (
                "id" | "tipo" | "color" | "forma" | "pos" | "antes" | "cita" | "despues" | "nota"
                | "imagen",
                _,
            ) => return None,
            (_, valor) => extras.push((clave, valor)),
        }
    }
    let id = id?;
    numero_de_id(&id)?;
    let tipo = tipo?;
    let color = color?;
    if matches!(tipo, Tipo::Subrayado | Tipo::Marca)
        && (pos.is_none() || antes.is_none() || cita.is_none() || despues.is_none())
    {
        return None;
    }
    Some(Registro {
        id,
        tipo,
        color,
        forma,
        pos,
        antes,
        cita,
        despues,
        nota,
        imagen,
        extras,
    })
}

struct Cursor<'a> {
    texto: &'a str,
    indice: usize,
}

impl<'a> Cursor<'a> {
    fn espiar(&self) -> Option<char> {
        self.texto[self.indice..].chars().next()
    }

    fn avanzar(&mut self) -> Option<char> {
        let caracter = self.espiar()?;
        self.indice += caracter.len_utf8();
        Some(caracter)
    }

    fn espacios(&mut self) {
        while matches!(self.espiar(), Some(' ' | '\t' | '\r')) {
            self.indice += 1;
        }
    }

    fn consumir(&mut self, esperado: char) -> Option<()> {
        (self.avanzar()? == esperado).then_some(())
    }
}

fn leer_objeto(linea: &str) -> Option<Vec<(String, Valor)>> {
    let mut cursor = Cursor {
        texto: linea,
        indice: 0,
    };
    cursor.espacios();
    cursor.consumir('{')?;
    let mut campos: Vec<(String, Valor)> = Vec::new();
    cursor.espacios();
    if cursor.espiar() == Some('}') {
        cursor.avanzar();
    } else {
        loop {
            cursor.espacios();
            let clave = leer_cadena(&mut cursor)?;
            cursor.espacios();
            cursor.consumir(':')?;
            cursor.espacios();
            let valor = match cursor.espiar()? {
                '"' => Valor::Texto(leer_cadena(&mut cursor)?),
                _ => Valor::Entero(leer_entero(&mut cursor)?),
            };
            if campos.iter().any(|(existente, _)| *existente == clave) {
                return None;
            }
            campos.push((clave, valor));
            cursor.espacios();
            match cursor.avanzar()? {
                ',' => {}
                '}' => break,
                _ => return None,
            }
        }
    }
    cursor.espacios();
    (cursor.indice == linea.len()).then_some(campos)
}

fn leer_entero(cursor: &mut Cursor) -> Option<i64> {
    let inicio = cursor.indice;
    if cursor.espiar() == Some('-') {
        cursor.avanzar();
    }
    let digitos = cursor.indice;
    while matches!(cursor.espiar(), Some('0'..='9')) {
        cursor.avanzar();
    }
    let cantidad = cursor.indice - digitos;
    let sin_ceros = &cursor.texto[digitos..cursor.indice];
    if cantidad == 0 || (cantidad > 1 && sin_ceros.starts_with('0')) {
        return None;
    }
    // Fracciones y exponentes quedan fuera: el formato solo guarda enteros.
    if matches!(cursor.espiar(), Some('.' | 'e' | 'E')) {
        return None;
    }
    cursor.texto[inicio..cursor.indice].parse().ok()
}

fn leer_cadena(cursor: &mut Cursor) -> Option<String> {
    cursor.consumir('"')?;
    let mut salida = String::new();
    loop {
        match cursor.avanzar()? {
            '"' => return Some(salida),
            '\\' => match cursor.avanzar()? {
                '"' => salida.push('"'),
                '\\' => salida.push('\\'),
                '/' => salida.push('/'),
                'b' => salida.push('\u{8}'),
                'f' => salida.push('\u{c}'),
                'n' => salida.push('\n'),
                'r' => salida.push('\r'),
                't' => salida.push('\t'),
                'u' => {
                    let primero = cuatro_hex(cursor)?;
                    let caracter = if (0xD800..0xDC00).contains(&primero) {
                        // Par sustituto: debe seguir `\uXXXX` con la mitad baja.
                        cursor.consumir('\\')?;
                        cursor.consumir('u')?;
                        let segundo = cuatro_hex(cursor)?;
                        if !(0xDC00..0xE000).contains(&segundo) {
                            return None;
                        }
                        char::from_u32(0x10000 + ((primero - 0xD800) << 10) + (segundo - 0xDC00))?
                    } else {
                        // Una mitad baja sola no es un carácter válido.
                        char::from_u32(primero)?
                    };
                    salida.push(caracter);
                }
                _ => return None,
            },
            // JSON no admite controles sin escapar dentro de una cadena.
            caracter if (caracter as u32) < 0x20 => return None,
            caracter => salida.push(caracter),
        }
    }
}

fn cuatro_hex(cursor: &mut Cursor) -> Option<u32> {
    let mut valor = 0;
    for _ in 0..4 {
        valor = valor * 16 + cursor.avanzar()?.to_digit(16)?;
    }
    Some(valor)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn subrayado(id: &str, pos: u64, cita: &str) -> Registro {
        Registro {
            id: id.to_string(),
            tipo: Tipo::Subrayado,
            color: Color::Amarillo,
            forma: None,
            pos: Some(pos),
            antes: Some(String::new()),
            cita: Some(cita.to_string()),
            despues: Some(String::new()),
            nota: None,
            imagen: None,
            extras: Vec::new(),
        }
    }

    fn marca(id: &str, pos: u64, cita: &str) -> Registro {
        Registro {
            tipo: Tipo::Marca,
            color: Color::Azul,
            forma: Some(Forma::Cinta),
            ..subrayado(id, pos, cita)
        }
    }

    fn leyenda(id: &str, color: Color, nota: &str) -> Registro {
        Registro {
            id: id.to_string(),
            tipo: Tipo::Leyenda,
            color,
            forma: None,
            pos: None,
            antes: None,
            cita: None,
            despues: None,
            nota: Some(nota.to_string()),
            imagen: None,
            extras: Vec::new(),
        }
    }

    fn entendido(bloque: &str) -> Anotaciones {
        match leer(bloque) {
            Lectura::Entendido(anotaciones) => anotaciones,
            otra => panic!("se esperaba un bloque entendido, salió {otra:?}"),
        }
    }

    #[test]
    fn bloque_vacio_no_se_escribe() {
        assert_eq!(escribir(&Anotaciones::default(), "\n"), None);
    }

    #[test]
    fn ida_y_vuelta_conserva_bytes() {
        let original = Anotaciones {
            registros: vec![
                leyenda("a3", Color::Amarillo, "idea clave"),
                subrayado("a1", 23, "mitocondria"),
                marca("a2", 81, "## Fotosíntesis"),
            ],
            lineas_no_entendidas: vec!["<<<<<<< HEAD".to_string()],
        };
        for salto in ["\n", "\r\n"] {
            let primero = escribir(&original, salto).unwrap();
            let leido = entendido(&primero);
            let segundo = escribir(&leido, salto).unwrap();
            assert_eq!(primero, segundo, "volver a guardar cambió los bytes");
        }
    }

    #[test]
    fn escritura_ordena_por_posicion_tipo_e_id() {
        let mut anotaciones = Anotaciones {
            registros: vec![
                subrayado("a1", 50, "dos"),
                subrayado("a4", 10, "uno"),
                marca("a3", 50, "dos"),
                leyenda("a9", Color::Verde, "verde"),
                leyenda("a8", Color::Amarillo, "amarillo"),
                subrayado("a2", 50, "dos"),
            ],
            ..Anotaciones::default()
        };
        let bloque = escribir(&anotaciones, "\n").unwrap();
        let ids: Vec<String> = entendido(&bloque)
            .registros
            .iter()
            .map(|r| r.id.clone())
            .collect();
        // Leyendas por paleta; luego pos 10; luego pos 50: marca, y subrayados por id.
        assert_eq!(ids, ["a8", "a9", "a4", "a3", "a1", "a2"]);
        // El orden de llegada no influye.
        anotaciones.registros.reverse();
        assert_eq!(escribir(&anotaciones, "\n").unwrap(), bloque);
    }

    #[test]
    fn escapa_menor_y_mayor_en_cadenas() {
        let mut registro = subrayado("a1", 0, "<b>x</b> --> <!-- &");
        registro.nota = Some("a>b".to_string());
        let bloque = escribir(
            &Anotaciones {
                registros: vec![registro.clone()],
                ..Anotaciones::default()
            },
            "\n",
        )
        .unwrap();
        assert!(bloque.contains("\\u003cb\\u003ex"));
        assert_eq!(entendido(&bloque).registros[0], registro);
    }

    #[test]
    fn escapa_los_separadores_de_linea_de_unicode() {
        let mut registro = subrayado("a1", 0, "uno\u{2028}dos\u{2029}tres\u{85}cuatro");
        registro.nota = Some("x\u{7f}y\u{1}z".to_string());
        let bloque = escribir(
            &Anotaciones {
                registros: vec![registro.clone()],
                ..Anotaciones::default()
            },
            "\n",
        )
        .unwrap();
        for separador in ['\u{2028}', '\u{2029}', '\u{85}', '\u{7f}', '\u{1}'] {
            assert!(
                !bloque.contains(separador),
                "{separador:?} quedó sin escapar"
            );
        }
        // Un programa que parta por separadores de Unicode ve las mismas líneas.
        let por_unicode = bloque.split(['\u{2028}', '\u{2029}', '\u{85}']).count();
        assert_eq!(por_unicode, 1);
        assert_eq!(entendido(&bloque).registros[0], registro);
    }

    #[test]
    fn ninguna_nota_puede_cerrar_el_comentario() {
        for nota in ["-->", "--!>", "<!--", "<!-->", "-- >", "x-->y\n-->\nz"] {
            let mut registro = marca("a1", 0, "título");
            registro.nota = Some(nota.to_string());
            let bloque = escribir(
                &Anotaciones {
                    registros: vec![registro.clone()],
                    ..Anotaciones::default()
                },
                "\n",
            )
            .unwrap();
            assert_eq!(bloque.matches("-->").count(), 1, "nota {nota:?}");
            assert!(bloque.ends_with("-->"));
            assert_eq!(bloque.matches("<!--").count(), 1);
            assert_eq!(entendido(&bloque).registros[0].nota.as_deref(), Some(nota));
        }
    }

    #[test]
    fn version_mayor_se_conserva_inerte() {
        let bloque = "<!-- visor-md:anotaciones v2\nlo que sea\n{\"x\":1}\n-->";
        assert_eq!(leer(bloque), Lectura::Inerte(Inerte::VersionNueva(2)));
        let enorme = "<!-- visor-md:anotaciones v99999999999\n-->";
        assert!(matches!(
            leer(enorme),
            Lectura::Inerte(Inerte::VersionNueva(_))
        ));
    }

    #[test]
    fn linea_invalida_se_conserva_al_final() {
        let base = escribir(
            &Anotaciones {
                registros: vec![subrayado("a1", 0, "uno")],
                ..Anotaciones::default()
            },
            "\n",
        )
        .unwrap();
        let con_conflicto = base.replace(
            "\n-->",
            "\n<<<<<<< HEAD\n{\"id\":\"zz\",\"tipo\":\"marca\"}\n{roto\n-->",
        );
        let leido = entendido(&con_conflicto);
        assert_eq!(leido.registros.len(), 1);
        assert_eq!(
            leido.lineas_no_entendidas,
            [
                "<<<<<<< HEAD",
                "{\"id\":\"zz\",\"tipo\":\"marca\"}",
                "{roto"
            ]
        );
        let reescrito = escribir(&leido, "\n").unwrap();
        assert!(
            reescrito.ends_with("<<<<<<< HEAD\n{\"id\":\"zz\",\"tipo\":\"marca\"}\n{roto\n-->")
        );
    }

    #[test]
    fn claves_desconocidas_se_conservan() {
        let bloque = format!(
            "{APERTURA}1\n{DESCRIPCION}\n{{\"id\":\"a1\",\"tipo\":\"super\",\"color\":\"turquesa\",\"zeta\":7,\"alfa\":\"x\"}}\n-->"
        );
        let leido = entendido(&bloque);
        let registro = &leido.registros[0];
        assert_eq!(registro.tipo, Tipo::Otro("super".to_string()));
        assert_eq!(registro.color, Color::Otro("turquesa".to_string()));
        assert_eq!(
            registro.extras,
            [
                ("zeta".to_string(), Valor::Entero(7)),
                ("alfa".to_string(), Valor::Texto("x".to_string())),
            ]
        );
        assert_eq!(escribir(&leido, "\n").unwrap(), bloque);
    }

    #[test]
    fn limites_vuelven_inerte_el_bloque() {
        let mut demasiados = format!("{APERTURA}1\n{DESCRIPCION}\n");
        for numero in 0..=MAX_REGISTROS {
            demasiados.push_str(&format!(
                "{{\"id\":\"a{numero}\",\"tipo\":\"leyenda\",\"color\":\"rosa\"}}\n"
            ));
        }
        demasiados.push_str("-->");
        assert_eq!(leer(&demasiados), Lectura::Inerte(Inerte::Limites));

        let grande = format!(
            "{APERTURA}1\n{DESCRIPCION}\n{{\"id\":\"a1\",\"tipo\":\"leyenda\",\"color\":\"rosa\",\"nota\":\"{}\"}}\n-->",
            "x".repeat(MAX_BLOQUE_BYTES)
        );
        assert_eq!(leer(&grande), Lectura::Inerte(Inerte::Limites));

        // Una línea de más de 16 KiB no inutiliza el bloque: queda «no entendida».
        let larga = format!(
            "{APERTURA}1\n{DESCRIPCION}\n{{\"id\":\"a1\",\"tipo\":\"leyenda\",\"color\":\"rosa\",\"nota\":\"{}\"}}\n-->",
            "x".repeat(MAX_LINEA_BYTES)
        );
        let leido = entendido(&larga);
        assert!(leido.registros.is_empty());
        assert_eq!(leido.lineas_no_entendidas.len(), 1);
    }

    #[test]
    fn crlf_y_lf_producen_mismas_posiciones() {
        let mut registro = subrayado("a1", 12, "uno\ndos");
        registro.antes = Some("x\n".to_string());
        let anotaciones = Anotaciones {
            registros: vec![registro, marca("a2", 40, "## Título")],
            ..Anotaciones::default()
        };
        let lf = escribir(&anotaciones, "\n").unwrap();
        let crlf = escribir(&anotaciones, "\r\n").unwrap();
        assert_ne!(lf, crlf);
        assert_eq!(crlf, lf.replace('\n', "\r\n"));
        assert_eq!(entendido(&lf), entendido(&crlf));
        // Los saltos de la cita viajan escapados: el contenido no cambia con el EOL.
        assert!(lf.contains("uno\\ndos"));
    }

    #[test]
    fn lo_que_no_es_un_bloque_se_rechaza() {
        assert_eq!(leer("texto"), Lectura::NoEsBloque);
        assert_eq!(leer("<!-- otra cosa\n-->"), Lectura::NoEsBloque);
        assert_eq!(
            leer("<!-- visor-md:anotaciones v0\n-->"),
            Lectura::NoEsBloque
        );
        assert_eq!(leer("<!-- visor-md:anotaciones v1"), Lectura::NoEsBloque);
        // Un `-->` antes del final cierra el comentario: lo de después es otra cosa.
        assert_eq!(
            leer("<!-- visor-md:anotaciones v1\n-->\ntexto\n-->"),
            Lectura::NoEsBloque
        );
    }

    #[test]
    fn el_siguiente_id_no_reutiliza_numeros() {
        let anotaciones = Anotaciones {
            registros: vec![subrayado("a2", 0, "x"), subrayado("a9", 1, "y")],
            ..Anotaciones::default()
        };
        assert_eq!(anotaciones.nuevo_id(), "a10");
        assert_eq!(Anotaciones::default().nuevo_id(), "a1");
    }

    #[test]
    fn el_lector_acepta_json_valido_aunque_no_sea_el_que_escribe_la_app() {
        let linea = "{ \"id\" : \"a1\", \"tipo\":\"subrayado\", \"color\":\"rosa\", \"pos\": 3,\"antes\":\"\\u00e1\\ud83d\\ude00\",\"cita\":\"a\\/b\",\"despues\":\"\" }";
        let bloque = format!("{APERTURA}1\n{DESCRIPCION}\n{linea}\n-->");
        let leido = entendido(&bloque);
        let registro = &leido.registros[0];
        assert_eq!(registro.antes.as_deref(), Some("á😀"));
        assert_eq!(registro.cita.as_deref(), Some("a/b"));
        // Mitades sustitutas sueltas, números con fracción y repetidas: dañadas.
        for mala in [
            "{\"id\":\"a1\",\"tipo\":\"leyenda\",\"color\":\"rosa\",\"nota\":\"\\ud83d\"}",
            "{\"id\":\"a1\",\"tipo\":\"leyenda\",\"color\":\"rosa\",\"z\":1.5}",
            "{\"id\":\"a1\",\"id\":\"a2\",\"tipo\":\"leyenda\",\"color\":\"rosa\"}",
            "{\"id\":\"x1\",\"tipo\":\"leyenda\",\"color\":\"rosa\"}",
            "{\"id\":\"a1\",\"tipo\":\"subrayado\",\"color\":\"rosa\"}",
        ] {
            let bloque = format!("{APERTURA}1\n{DESCRIPCION}\n{mala}\n-->");
            let leido = entendido(&bloque);
            assert!(leido.registros.is_empty(), "{mala}");
            assert_eq!(leido.lineas_no_entendidas.len(), 1, "{mala}");
        }
    }

    #[test]
    fn el_contexto_se_recorta_al_escribir() {
        let mut registro = subrayado("a1", 0, "x");
        registro.antes = Some(format!("{}fin", "a".repeat(40)));
        registro.despues = Some(format!("inicio{}", "b".repeat(40)));
        let bloque = escribir(
            &Anotaciones {
                registros: vec![registro],
                ..Anotaciones::default()
            },
            "\n",
        )
        .unwrap();
        let leido = entendido(&bloque);
        let registro = &leido.registros[0];
        assert_eq!(
            registro.antes.as_ref().unwrap().chars().count(),
            MAX_CONTEXTO
        );
        assert!(registro.antes.as_ref().unwrap().ends_with("fin"));
        assert!(registro.despues.as_ref().unwrap().starts_with("inicio"));
        assert_eq!(
            registro.despues.as_ref().unwrap().chars().count(),
            MAX_CONTEXTO
        );
    }
}
