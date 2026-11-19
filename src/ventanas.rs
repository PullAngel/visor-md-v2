//! Varias ventanas, un proceso por ventana (D-33).
//!
//! Cada proceso se anota en `<raíz>/ventanas/<pid>.txt` y renueva su latido
//! cada 2 s; una anotación sin latido hace más de 6 s se ignora y se borra. Para
//! pedirle algo a otra ventana (abrir un archivo, recibir una pestaña) se deja
//! un mensaje en su buzón, `<raíz>/buzon/<pid>/<n>.msg`, que ella lee al recibir
//! el foco y cada 500 ms. Sin hilos ni sockets: solo archivos pequeños y
//! atómicos dentro del perfil de la persona.
//!
//! Los mensajes son datos de otro proceso de la misma persona: las rutas se
//! abren como si vinieran de la línea de comandos, nunca se ejecutan, y todo
//! tamaño y cantidad está acotado.

use atomicwrites::{AllowOverwrite, AtomicFile};
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Cada cuánto se renueva el latido.
pub(crate) const HEARTBEAT_EVERY: Duration = Duration::from_secs(2);
/// Una ventana sin latido pasado este tiempo se considera cerrada.
pub(crate) const STALE_AFTER: Duration = Duration::from_secs(6);
/// Cada cuánto se mira el buzón.
pub(crate) const MAILBOX_POLL: Duration = Duration::from_millis(500);

const MAX_MESSAGES_PER_READ: usize = 64;
const MAX_MESSAGE_BYTES: u64 = 64 * 1024;
const MAX_PATHS_PER_MESSAGE: usize = 32;

/// Una ventana viva, tal como la anota su latido.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WindowInfo {
    pub(crate) pid: u32,
    /// Milisegundos desde 1970 del último latido.
    pub(crate) heartbeat_ms: u64,
}

/// Lo que una ventana le pide a otra.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Message {
    /// Abrir estos archivos, cada uno en su pestaña.
    Open(Vec<PathBuf>),
}

pub(crate) fn unix_millis(now: SystemTime) -> u64 {
    now.duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_millis() as u64)
}

/// El registro y el buzón de este proceso dentro de una raíz.
#[derive(Clone, Debug)]
pub(crate) struct Registry {
    root: PathBuf,
    pid: u32,
}

impl Registry {
    pub(crate) fn at(root: impl Into<PathBuf>, pid: u32) -> Self {
        Self {
            root: root.into(),
            pid,
        }
    }

    /// La raíz real de la persona, para este proceso. `None` si no hay perfil.
    pub(crate) fn for_this_process() -> Option<Self> {
        Some(Self::at(default_root()?, std::process::id()))
    }

    fn windows_dir(&self) -> PathBuf {
        self.root.join("ventanas")
    }

    fn mailbox_dir(&self, pid: u32) -> PathBuf {
        self.root.join("buzon").join(pid.to_string())
    }

    /// Anota este proceso (o renueva su latido).
    pub(crate) fn heartbeat(&self, now: SystemTime) -> Result<(), std::io::Error> {
        let dir = self.windows_dir();
        fs::create_dir_all(&dir)?;
        let text = format!("pid={}\nheartbeat={}\n", self.pid, unix_millis(now));
        AtomicFile::new(dir.join(format!("{}.txt", self.pid)), AllowOverwrite)
            .write(|file| file.write_all(text.as_bytes()))
            .map_err(|error| match error {
                atomicwrites::Error::Internal(error) | atomicwrites::Error::User(error) => error,
            })
    }

    /// Borra la anotación y el buzón de este proceso, al cerrar.
    pub(crate) fn unregister(&self) {
        let _ = fs::remove_file(self.windows_dir().join(format!("{}.txt", self.pid)));
        let _ = fs::remove_dir_all(self.mailbox_dir(self.pid));
    }

    /// Las otras ventanas vivas. Las que dejaron de dar latido se borran.
    pub(crate) fn others(&self, now: SystemTime) -> Vec<WindowInfo> {
        let Ok(entries) = fs::read_dir(self.windows_dir()) else {
            return Vec::new();
        };
        let now_ms = unix_millis(now);
        let mut alive = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("txt") {
                continue;
            }
            let Some(info) = read_window(&path) else {
                continue;
            };
            if info.pid == self.pid {
                continue;
            }
            if now_ms.saturating_sub(info.heartbeat_ms) > STALE_AFTER.as_millis() as u64 {
                let _ = fs::remove_file(&path);
                let _ = fs::remove_dir_all(self.mailbox_dir(info.pid));
                continue;
            }
            alive.push(info);
        }
        alive.sort_by_key(|info| std::cmp::Reverse(info.heartbeat_ms));
        alive
    }

    /// Deja un mensaje en el buzón de otra ventana. El archivo aparece de golpe:
    /// se escribe con otro nombre y se renombra.
    pub(crate) fn send(&self, target: u32, message: &Message) -> Result<(), std::io::Error> {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let dir = self.mailbox_dir(target);
        fs::create_dir_all(&dir)?;
        let Message::Open(paths) = message;
        if paths.is_empty() || paths.len() > MAX_PATHS_PER_MESSAGE {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "cantidad de rutas no válida",
            ));
        }
        let mut text = String::new();
        for path in paths {
            let path = path.to_str().ok_or_else(|| {
                std::io::Error::new(ErrorKind::InvalidInput, "ruta que no es texto")
            })?;
            if path.contains(['\n', '\r']) {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    "ruta con salto de línea",
                ));
            }
            text.push_str("open=");
            text.push_str(path);
            text.push('\n');
        }
        let name = format!(
            "{:020}-{}-{}",
            unix_millis(SystemTime::now()),
            self.pid,
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
        let temp = dir.join(format!("{name}.tmp"));
        fs::write(&temp, text)?;
        fs::rename(&temp, dir.join(format!("{name}.msg")))
    }

    /// Lee y borra los mensajes de este proceso, del más viejo al más nuevo.
    /// Un archivo ilegible, enorme o mal formado se borra sin efecto.
    pub(crate) fn take(&self) -> Vec<Message> {
        let dir = self.mailbox_dir(self.pid);
        let Ok(entries) = fs::read_dir(&dir) else {
            return Vec::new();
        };
        let mut files: Vec<PathBuf> = entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("msg"))
            .collect();
        files.sort();
        let mut messages = Vec::new();
        for path in files.into_iter().take(MAX_MESSAGES_PER_READ) {
            let parsed = fs::metadata(&path)
                .ok()
                .filter(|metadata| metadata.is_file() && metadata.len() <= MAX_MESSAGE_BYTES)
                .and_then(|_| fs::read_to_string(&path).ok())
                .and_then(|text| parse_message(&text));
            let _ = fs::remove_file(&path);
            messages.extend(parsed);
        }
        messages
    }
}

fn parse_message(text: &str) -> Option<Message> {
    let paths: Vec<PathBuf> = text
        .lines()
        .filter_map(|line| line.strip_prefix("open="))
        .filter(|path| !path.is_empty())
        .take(MAX_PATHS_PER_MESSAGE)
        .map(PathBuf::from)
        .collect();
    (!paths.is_empty()).then_some(Message::Open(paths))
}

fn read_window(path: &Path) -> Option<WindowInfo> {
    let metadata = fs::metadata(path).ok()?;
    if !metadata.is_file() || metadata.len() > 1024 {
        return None;
    }
    let text = fs::read_to_string(path).ok()?;
    let mut pid = None;
    let mut heartbeat_ms = None;
    for line in text.lines() {
        match line.split_once('=') {
            Some(("pid", value)) => pid = value.trim().parse().ok(),
            Some(("heartbeat", value)) => heartbeat_ms = value.trim().parse().ok(),
            _ => {}
        }
    }
    Some(WindowInfo {
        pid: pid?,
        heartbeat_ms: heartbeat_ms?,
    })
}

/// La raíz de ventanas y buzones: junto a los demás datos de la aplicación.
fn default_root() -> Option<PathBuf> {
    // Las pruebas nunca tocan la carpeta real de la persona.
    #[cfg(test)]
    return Some(
        std::env::var_os("VISOR_TEST_WINDOWS_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                std::env::temp_dir()
                    .join(format!("visor-md-ventanas-prueba-{}", std::process::id()))
            }),
    );
    #[cfg(not(test))]
    {
        let base = std::env::var_os("LOCALAPPDATA")
            .or_else(|| std::env::var_os("XDG_STATE_HOME"))
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .or_else(|| {
                std::env::var_os("HOME")
                    .map(PathBuf::from)
                    .filter(|path| path.is_absolute())
                    .map(|home| home.join(".local").join("state"))
            })?;
        Some(base.join("VisorMD"))
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn raiz(nombre: &str) -> PathBuf {
        let raiz =
            std::env::temp_dir().join(format!("visor-md-ventanas-{nombre}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&raiz);
        raiz
    }

    fn en(segundos: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(1_800_000_000 + segundos)
    }

    #[test]
    fn una_ventana_ve_a_las_otras_vivas_y_no_a_si_misma() {
        let r = raiz("vivas");
        let (a, b) = (Registry::at(&r, 100), Registry::at(&r, 200));
        a.heartbeat(en(0)).unwrap();
        b.heartbeat(en(1)).unwrap();
        assert_eq!(
            a.others(en(2)).iter().map(|w| w.pid).collect::<Vec<_>>(),
            [200]
        );
        assert_eq!(
            b.others(en(2)).iter().map(|w| w.pid).collect::<Vec<_>>(),
            [100]
        );
        // La más reciente va primero.
        let c = Registry::at(&r, 300);
        assert_eq!(
            c.others(en(2)).iter().map(|w| w.pid).collect::<Vec<_>>(),
            [200, 100]
        );
        let _ = fs::remove_dir_all(r);
    }

    #[test]
    fn sin_latido_por_mas_de_6_s_la_ventana_se_ignora_y_se_borra() {
        let r = raiz("viejas");
        let (a, b) = (Registry::at(&r, 100), Registry::at(&r, 200));
        b.heartbeat(en(0)).unwrap();
        assert_eq!(a.others(en(5)).len(), 1, "a los 5 s sigue viva");
        assert!(a.others(en(7)).is_empty(), "a los 7 s no");
        assert!(
            !r.join("ventanas").join("200.txt").exists(),
            "se borró su anotación"
        );
        let _ = fs::remove_dir_all(r);
    }

    #[test]
    fn un_mensaje_llega_una_vez_y_en_orden() {
        let r = raiz("buzon");
        let (a, b) = (Registry::at(&r, 100), Registry::at(&r, 200));
        a.send(200, &Message::Open(vec![PathBuf::from("uno.md")]))
            .unwrap();
        a.send(
            200,
            &Message::Open(vec![PathBuf::from("dos.md"), PathBuf::from("tres.md")]),
        )
        .unwrap();
        let recibidos = b.take();
        assert_eq!(
            recibidos,
            vec![
                Message::Open(vec![PathBuf::from("uno.md")]),
                Message::Open(vec![PathBuf::from("dos.md"), PathBuf::from("tres.md")]),
            ]
        );
        assert!(b.take().is_empty(), "un mensaje no se entrega dos veces");
        assert!(a.take().is_empty(), "el buzón de otro no se lee");
        let _ = fs::remove_dir_all(r);
    }

    #[test]
    fn los_mensajes_mal_formados_enormes_o_con_rutas_raras_no_hacen_nada() {
        let r = raiz("hostil");
        let (a, b) = (Registry::at(&r, 100), Registry::at(&r, 200));
        let buzon = r.join("buzon").join("200");
        fs::create_dir_all(&buzon).unwrap();
        fs::write(buzon.join("0001.msg"), "basura\nsin formato\n").unwrap();
        fs::write(buzon.join("0002.msg"), vec![b'a'; 200_000]).unwrap();
        fs::write(buzon.join("0003.msg"), "open=\n").unwrap();
        fs::write(buzon.join("0004.tmp"), "open=a-medias.md\n").unwrap();
        assert!(b.take().is_empty());
        assert!(!buzon.join("0001.msg").exists() && !buzon.join("0002.msg").exists());
        // Enviar una ruta con salto de línea o ninguna ruta se rechaza.
        assert!(
            a.send(200, &Message::Open(vec![PathBuf::from("a\nopen=b")]))
                .is_err()
        );
        assert!(a.send(200, &Message::Open(vec![])).is_err());
        // Y no más de 32 rutas por mensaje.
        let muchas = (0..40).map(|i| PathBuf::from(format!("{i}.md"))).collect();
        assert!(a.send(200, &Message::Open(muchas)).is_err());
        let _ = fs::remove_dir_all(r);
    }

    #[test]
    fn cerrar_borra_la_anotacion_y_el_buzon() {
        let r = raiz("cerrar");
        let (a, b) = (Registry::at(&r, 100), Registry::at(&r, 200));
        b.heartbeat(en(0)).unwrap();
        a.send(200, &Message::Open(vec![PathBuf::from("x.md")]))
            .unwrap();
        b.unregister();
        assert!(a.others(en(1)).is_empty());
        assert!(!r.join("buzon").join("200").exists());
        let _ = fs::remove_dir_all(r);
    }
}
