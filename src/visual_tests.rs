//! Pruebas de imagen de referencia. Cada escena se dibuja sin ventana y se
//! compara píxel a píxel con una imagen guardada en `tests/visual/referencia/`.
//! Un cambio visual involuntario rompe la prueba; uno deliberado se aprueba con
//! `VISOR_BLESS=1 cargo test visual` y se revisa con `kompari`.
//!
//! Solo se usan las fuentes embebidas, de modo que el resultado no depende de
//! las fuentes instaladas. Las imágenes se generaron en Windows; en otras
//! plataformas la prueba se omite hasta confirmar que el rasterizado coincide.

use super::*;
use std::path::{Path, PathBuf};

const TAMANOS: [(u32, u32); 2] = [(640, 480), (1100, 720)];
/// Diferencia máxima por canal que se tolera (antialias entre CPU).
const TOLERANCIA_CANAL: u8 = 2;
/// Fracción máxima de píxeles que pueden superar la tolerancia.
const TOLERANCIA_PIXELES: f64 = 0.001;

const DOCUMENTO: &str = "# Apuntes de biología\n\nLa **célula** es la unidad básica de la vida. Hay dos tipos: *procariota* y *eucariota*.\n\n## Partes\n\n- Membrana\n- Citoplasma\n- Núcleo\n\n> [!NOTE]\n> Repasar antes del examen.\n\n```bash\nping 192.168.0.1\n```\n\n- [ ] Leer el capítulo 3\n- [x] Resumir la célula\n\n| Tipo | Núcleo |\n| --- | --- |\n| Procariota | No |\n| Eucariota | Sí |\n";

struct Escena {
    nombre: &'static str,
    modo: DocumentMode,
    preparar: fn(&mut App),
}

fn sin_cambios(_: &mut App) {}

fn abrir_paleta(app: &mut App) {
    app.open_command_palette();
}

fn abrir_menu_bloques(app: &mut App) {
    app.open_block_actions();
}

fn abrir_indice(app: &mut App) {
    app.show_document_outline();
}

fn abrir_segunda_pestana(app: &mut App) {
    app.perform_action(AppAction::NewDocument);
}

fn abrir_dialogo_de_cierre(app: &mut App) {
    let id = app.document.id;
    app.open_dialog(Dialog::close_tab("apuntes.md", true), Some(id));
}

fn mostrar_aviso(app: &mut App) {
    app.set_notice("Documento guardado");
}

/// Puntero quieto sobre «Guardar» desde hace más que el retraso del tooltip.
fn apuntar_a_guardar(app: &mut App) {
    let x = TOOLBAR_X + 2.0 * TOOLBAR_ITEM_WIDTH + 18.0;
    let y = TOOLBAR_Y + TOOLBAR_HEIGHT * 0.5;
    app.pointer = Some((x, y));
    let ahora = Instant::now();
    let desde = ahora
        .checked_sub(std::time::Duration::from_secs(2))
        .expect("el reloj permite restar dos segundos");
    app.ui_motion.set_hover(
        Some(UiTarget::Toolbar(AppAction::Save)),
        desde,
        app.reduce_motion,
    );
    app.tooltip.poll(Some(desde), ahora);
}

/// Puntero sobre el botón Copiar del primer bloque de código.
fn apuntar_a_copiar(app: &mut App) {
    let bloque = app
        .document
        .blocks
        .iter()
        .position(|block| matches!(block.kind, Kind::Code))
        .expect("el documento de prueba tiene un bloque de código");
    let ahora = Instant::now();
    let desde = ahora
        .checked_sub(std::time::Duration::from_secs(2))
        .expect("el reloj permite restar dos segundos");
    app.ui_motion.set_hover(
        Some(UiTarget::CodeCopy {
            block: bloque,
            preview: false,
        }),
        desde,
        app.reduce_motion,
    );
}

fn mostrar_tooltip(app: &mut App) {
    apuntar_a_guardar(app);
}

fn dialogo_con_el_puntero_sobre_un_boton(app: &mut App) {
    apuntar_a_guardar(app);
    abrir_dialogo_de_cierre(app);
}

const ESCENAS: [Escena; 12] = [
    Escena {
        nombre: "lectura",
        modo: DocumentMode::Reading,
        preparar: sin_cambios,
    },
    Escena {
        nombre: "edicion",
        modo: DocumentMode::SourceEditing,
        preparar: sin_cambios,
    },
    Escena {
        nombre: "comparar",
        modo: DocumentMode::Split,
        preparar: sin_cambios,
    },
    Escena {
        nombre: "paleta",
        modo: DocumentMode::Reading,
        preparar: abrir_paleta,
    },
    Escena {
        nombre: "menu-bloques",
        modo: DocumentMode::SourceEditing,
        preparar: abrir_menu_bloques,
    },
    Escena {
        nombre: "indice",
        modo: DocumentMode::Reading,
        preparar: abrir_indice,
    },
    Escena {
        nombre: "dos-pestanas",
        modo: DocumentMode::Reading,
        preparar: abrir_segunda_pestana,
    },
    Escena {
        nombre: "dialogo-cierre",
        modo: DocumentMode::SourceEditing,
        preparar: abrir_dialogo_de_cierre,
    },
    Escena {
        nombre: "aviso",
        modo: DocumentMode::Reading,
        preparar: mostrar_aviso,
    },
    Escena {
        nombre: "copiar-hover",
        modo: DocumentMode::Reading,
        preparar: apuntar_a_copiar,
    },
    Escena {
        nombre: "tooltip",
        modo: DocumentMode::Reading,
        preparar: mostrar_tooltip,
    },
    Escena {
        nombre: "dialogo-sin-tooltip",
        modo: DocumentMode::SourceEditing,
        preparar: dialogo_con_el_puntero_sobre_un_boton,
    },
];

fn raiz() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn dibujar(escena: &Escena, palette: Palette, ancho: u32, alto: u32) -> Pixmap {
    let mut app = App::for_tests(palette);
    app.load_for_test(DOCUMENTO, escena.modo);
    (escena.preparar)(&mut app);
    if escena.nombre == "aviso" {
        // El aviso debe verse completo sin depender del reloj de la prueba.
        app.notice_visible_until = Some(Instant::now() + std::time::Duration::from_secs(3600));
    } else {
        app.notice_visible_until = None;
    }
    app.draw_for_test(ancho, alto).clone()
}

fn nombre_de_archivo(escena: &Escena, noche: bool, ancho: u32, alto: u32) -> String {
    format!(
        "{}-{}-{ancho}x{alto}.png",
        escena.nombre,
        if noche { "noche" } else { "dia" }
    )
}

/// Compara dos lienzos y devuelve la cantidad de píxeles que superan la
/// tolerancia, junto con una imagen que los marca en magenta.
fn comparar(esperado: &Pixmap, actual: &Pixmap) -> (usize, Pixmap) {
    let magenta = tiny_skia::PremultipliedColorU8::from_rgba(255, 0, 255, 255)
        .expect("el magenta opaco es válido");
    let mut diferencia = actual.clone();
    let mut malos = 0;
    for (indice, (a, b)) in esperado
        .pixels()
        .iter()
        .zip(actual.pixels().iter())
        .enumerate()
    {
        let distinto = [
            a.red().abs_diff(b.red()),
            a.green().abs_diff(b.green()),
            a.blue().abs_diff(b.blue()),
            a.alpha().abs_diff(b.alpha()),
        ]
        .iter()
        .any(|diferencia| *diferencia > TOLERANCIA_CANAL);
        if distinto {
            malos += 1;
            diferencia.pixels_mut()[indice] = magenta;
        }
    }
    (malos, diferencia)
}

fn guardar(pixmap: &Pixmap, ruta: &Path) {
    if let Some(carpeta) = ruta.parent() {
        std::fs::create_dir_all(carpeta).expect("no se pudo crear la carpeta de imágenes");
    }
    pixmap
        .save_png(ruta)
        .unwrap_or_else(|error| panic!("no se pudo guardar {}: {error}", ruta.display()));
}

#[test]
#[cfg_attr(not(windows), ignore = "las referencias se generaron en Windows")]
fn las_escenas_coinciden_con_su_imagen_de_referencia() {
    let aprobar = std::env::var_os("VISOR_BLESS").is_some();
    let referencias = raiz().join("tests/visual/referencia");
    let actuales = raiz().join("target/visual/actual");
    let diferencias = raiz().join("target/visual/diff");
    let mut fallos: Vec<String> = Vec::new();

    for escena in &ESCENAS {
        for (noche, palette) in [(true, NIGHT), (false, DAY)] {
            for (ancho, alto) in TAMANOS {
                let nombre = nombre_de_archivo(escena, noche, ancho, alto);
                let actual = dibujar(escena, palette, ancho, alto);
                let referencia = referencias.join(&nombre);
                if aprobar {
                    guardar(&actual, &referencia);
                    continue;
                }
                let Ok(esperado) = Pixmap::load_png(&referencia) else {
                    guardar(&actual, &actuales.join(&nombre));
                    fallos.push(format!(
                        "{nombre}: falta la referencia (VISOR_BLESS=1 cargo test visual)"
                    ));
                    continue;
                };
                if (esperado.width(), esperado.height()) != (actual.width(), actual.height()) {
                    guardar(&actual, &actuales.join(&nombre));
                    fallos.push(format!("{nombre}: cambió el tamaño de la imagen"));
                    continue;
                }
                let (malos, imagen_de_diferencia) = comparar(&esperado, &actual);
                let total = (actual.width() * actual.height()) as f64;
                if malos as f64 / total > TOLERANCIA_PIXELES {
                    guardar(&actual, &actuales.join(&nombre));
                    guardar(&imagen_de_diferencia, &diferencias.join(&nombre));
                    fallos.push(format!(
                        "{nombre}: {malos} píxeles distintos ({:.2} %)",
                        malos as f64 / total * 100.0
                    ));
                }
            }
        }
    }

    assert!(
        fallos.is_empty(),
        "escenas distintas de su referencia (revisar con `kompari review tests/visual/referencia target/visual/actual`):\n{}",
        fallos.join("\n")
    );
}

#[test]
fn la_comparacion_detecta_un_pixel_distinto_y_tolera_el_antialias() {
    let mut base = Pixmap::new(10, 10).unwrap();
    base.fill(tiny_skia::Color::from_rgba8(10, 20, 30, 255));
    let mut igual = base.clone();
    // Un canal fuera por 1 queda dentro de la tolerancia.
    igual.data_mut()[0] = igual.data()[0].saturating_add(1);
    assert_eq!(comparar(&base, &igual).0, 0);
    // Un canal fuera por mucho cuenta como píxel distinto y se marca.
    let mut distinto = base.clone();
    distinto.data_mut()[4] = distinto.data()[4].saturating_add(40);
    let (malos, marcada) = comparar(&base, &distinto);
    assert_eq!(malos, 1);
    assert_eq!(&marcada.data()[4..8], &[255, 0, 255, 255]);
}
