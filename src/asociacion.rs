//! Asociación de archivos con Visor MD (D-34).
//!
//! En Windows registra el programa en «Abrir con» para las extensiones que la
//! persona elija, solo bajo su usuario (`HKCU`), llamando a
//! `%SystemRoot%\System32\reg.exe` con argumentos separados: sin shell, sin
//! dependencias nuevas y leyendo de vuelta cada cambio. Windows no deja que un
//! programa se declare el predeterminado: después se abre
//! `ms-settings:defaultapps` y la persona confirma.
//!
//! La prueba usa la raíz `HKCU\Software\VisorMDPrueba`, nunca el registro real.

use std::io::{Error, ErrorKind};
use std::path::Path;

/// Extensiones de Markdown, texto y otros formatos de texto, en ese orden.
pub(crate) const MARKDOWN: [&str; 4] = [".md", ".markdown", ".mdown", ".mkdn"];
pub(crate) const TEXT: [&str; 1] = [".txt"];
pub(crate) const OTHER: [&str; 5] = [".json", ".yaml", ".yml", ".toml", ".csv"];

/// Nombre del tipo de documento que se registra.
pub(crate) const PROG_ID: &str = "VisorMD.Document";

/// Una raíz de claves de clases de archivos del registro.
#[derive(Clone, Debug)]
pub(crate) struct Root(String);

impl Root {
    /// La del usuario: `HKCU\Software\Classes`.
    pub(crate) fn user() -> Self {
        Self(r"HKCU\Software\Classes".to_string())
    }

    /// La de las pruebas: un rincón propio, lejos de las asociaciones reales.
    #[cfg(test)]
    pub(crate) fn test() -> Self {
        Self(r"HKCU\Software\VisorMDPrueba\Classes".to_string())
    }

    fn key(&self, tail: &str) -> String {
        format!(r"{}\{tail}", self.0)
    }
}

/// Solo las extensiones de las listas de arriba se pueden registrar.
fn is_known_extension(extension: &str) -> bool {
    MARKDOWN
        .iter()
        .chain(TEXT.iter())
        .chain(OTHER.iter())
        .any(|known| *known == extension)
}

/// El ejecutable va entre comillas en el comando: no puede traerlas, y debe ser
/// una ruta absoluta.
fn command_for(exe: &Path) -> Result<String, Error> {
    let text = exe
        .to_str()
        .filter(|text| exe.is_absolute() && !text.contains(['"', '\n', '\r']))
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "ruta del programa no válida"))?;
    Ok(format!("\"{text}\" \"%1\""))
}

#[cfg(windows)]
mod sistema {
    use super::*;
    use std::process::{Command, Output};

    fn reg() -> std::path::PathBuf {
        let windows = std::env::var_os("SystemRoot").unwrap_or_else(|| "C:\\Windows".into());
        Path::new(&windows).join("System32").join("reg.exe")
    }

    fn run(args: &[&str]) -> Result<Output, Error> {
        Command::new(reg()).args(args).output()
    }

    fn ok(args: &[&str]) -> Result<(), Error> {
        let output = run(args)?;
        if output.status.success() {
            Ok(())
        } else {
            Err(Error::other(format!(
                "reg.exe {} terminó con error",
                args.first().copied().unwrap_or("")
            )))
        }
    }

    pub(super) fn add_value(
        key: &str,
        name: Option<&str>,
        kind: &str,
        data: &str,
    ) -> Result<(), Error> {
        match name {
            Some(name) => ok(&["add", key, "/v", name, "/t", kind, "/d", data, "/f"]),
            None => ok(&["add", key, "/ve", "/t", kind, "/d", data, "/f"]),
        }
    }

    pub(super) fn has_value(key: &str, name: &str) -> bool {
        run(&["query", key, "/v", name])
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn default_value(key: &str) -> Option<String> {
        let output = run(&["query", key, "/ve"]).ok()?;
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
    }

    pub(super) fn delete_value(key: &str, name: &str) -> Result<(), Error> {
        if !has_value(key, name) {
            return Ok(());
        }
        ok(&["delete", key, "/v", name, "/f"])
    }

    pub(super) fn delete_key(key: &str) -> Result<(), Error> {
        if !run(&["query", key]).is_ok_and(|output| output.status.success()) {
            return Ok(());
        }
        ok(&["delete", key, "/f"])
    }

    pub(super) fn open_default_apps() -> Result<(), Error> {
        Command::new("explorer.exe")
            .arg("ms-settings:defaultapps")
            .spawn()
            .map(|_| ())
    }
}

#[cfg(not(windows))]
mod sistema {
    use super::*;

    fn unsupported() -> Error {
        Error::new(ErrorKind::Unsupported, "solo disponible en Windows")
    }

    pub(super) fn add_value(_: &str, _: Option<&str>, _: &str, _: &str) -> Result<(), Error> {
        Err(unsupported())
    }
    pub(super) fn has_value(_: &str, _: &str) -> bool {
        false
    }
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn default_value(_: &str) -> Option<String> {
        None
    }
    pub(super) fn delete_value(_: &str, _: &str) -> Result<(), Error> {
        Err(unsupported())
    }
    pub(super) fn delete_key(_: &str) -> Result<(), Error> {
        Err(unsupported())
    }
    pub(super) fn open_default_apps() -> Result<(), Error> {
        Err(unsupported())
    }
}

/// Registra Visor MD en «Abrir con» para las extensiones dadas y lo verifica.
pub(crate) fn register(root: &Root, exe: &Path, extensions: &[&str]) -> Result<(), Error> {
    let command = command_for(exe)?;
    if let Some(unknown) = extensions.iter().find(|ext| !is_known_extension(ext)) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("extensión no admitida: {unknown}"),
        ));
    }
    sistema::add_value(&root.key(PROG_ID), None, "REG_SZ", "Documento de Visor MD")?;
    sistema::add_value(
        &root.key(&format!(r"{PROG_ID}\shell\open\command")),
        None,
        "REG_SZ",
        &command,
    )?;
    for extension in extensions {
        sistema::add_value(
            &root.key(&format!(r"{extension}\OpenWithProgids")),
            Some(PROG_ID),
            "REG_NONE",
            "",
        )?;
    }
    // Leer de vuelta: si algo no quedó, se dice.
    for extension in extensions {
        if !is_registered(root, extension) {
            return Err(Error::other(format!("no quedó registrada {extension}")));
        }
    }
    Ok(())
}

/// `true` si Visor MD figura en «Abrir con» para esa extensión.
pub(crate) fn is_registered(root: &Root, extension: &str) -> bool {
    is_known_extension(extension)
        && sistema::has_value(&root.key(&format!(r"{extension}\OpenWithProgids")), PROG_ID)
}

/// El comando registrado, leído del registro; sirve para comprobar a qué
/// ejecutable apunta.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn registered_command(root: &Root) -> Option<String> {
    sistema::default_value(&root.key(&format!(r"{PROG_ID}\shell\open\command")))
}

/// Quita el registro de las extensiones dadas y el tipo de documento.
pub(crate) fn unregister(root: &Root, extensions: &[&str]) -> Result<(), Error> {
    for extension in extensions {
        if !is_known_extension(extension) {
            continue;
        }
        sistema::delete_value(&root.key(&format!(r"{extension}\OpenWithProgids")), PROG_ID)?;
    }
    sistema::delete_key(&root.key(PROG_ID))
}

/// Abre la pantalla de aplicaciones predeterminadas de Windows, donde la
/// persona confirma.
pub(crate) fn open_default_apps_settings() -> Result<(), Error> {
    sistema::open_default_apps()
}

#[cfg(all(test, windows))]
mod pruebas {
    use super::*;

    fn limpiar() {
        let _ = std::process::Command::new(
            Path::new(&std::env::var_os("SystemRoot").unwrap_or_else(|| "C:\\Windows".into()))
                .join("System32")
                .join("reg.exe"),
        )
        .args(["delete", r"HKCU\Software\VisorMDPrueba", "/f"])
        .output();
    }

    #[test]
    fn registrar_se_lee_de_vuelta_y_se_quita_sin_tocar_el_registro_real() {
        limpiar();
        let raiz = Root::test();
        let exe = Path::new(r"C:\Programas de prueba\VisorMD.exe");
        assert!(!is_registered(&raiz, ".md"));
        register(&raiz, exe, &MARKDOWN).expect("se registra");
        for extension in MARKDOWN {
            assert!(is_registered(&raiz, extension), "{extension}");
        }
        assert!(!is_registered(&raiz, ".txt"), "solo las pedidas");
        let comando = registered_command(&raiz).expect("el comando se lee de vuelta");
        assert!(comando.contains("VisorMD.exe"), "{comando}");
        unregister(&raiz, &MARKDOWN).expect("se quita");
        assert!(!is_registered(&raiz, ".md"));
        assert!(registered_command(&raiz).is_none());
        limpiar();
    }

    #[test]
    fn rechaza_rutas_y_extensiones_peligrosas() {
        let raiz = Root::test();
        assert!(register(&raiz, Path::new("relativa.exe"), &MARKDOWN).is_err());
        assert!(register(&raiz, Path::new("C:\\a\"b.exe"), &MARKDOWN).is_err());
        assert!(register(&raiz, Path::new(r"C:\ok.exe"), &[".exe"]).is_err());
        assert!(register(&raiz, Path::new(r"C:\ok.exe"), &[".md\\..\\x"]).is_err());
        assert!(!is_registered(&raiz, ".exe"));
        limpiar();
    }
}

#[cfg(test)]
mod pruebas_comunes {
    use super::*;

    #[test]
    fn el_comando_cita_el_programa_y_el_archivo() {
        let comando = command_for(Path::new(if cfg!(windows) {
            r"C:\x\VisorMD.exe"
        } else {
            "/x/VisorMD"
        }))
        .unwrap();
        assert!(comando.starts_with('"') && comando.ends_with(r#""%1""#));
    }

    #[test]
    fn solo_las_extensiones_de_las_listas_son_conocidas() {
        assert!(is_known_extension(".md") && is_known_extension(".csv"));
        assert!(
            !is_known_extension(".exe") && !is_known_extension("md") && !is_known_extension("")
        );
    }
}
