# Servicio de recuperación ante cierres forzados y apagados

Fecha: 2026-11-15. Pedido del propietario (P-02): que un documento sin guardar
no se pierda si la aplicación se fuerza a cerrar o la computadora se apaga, con
una arquitectura segura y estable que no deje archivos basura ni agujeros de
seguridad.

Este documento parte de lo que ya existe en `src/recovery.rs` y propone el
rediseño. Las decisiones quedan en el plan como D-38 a D-43 y las tareas como la
fase 13 de [`plan-ejecucion-2026-11-14.md`](plan-ejecucion-2026-11-14.md).

## 1. Qué hay hoy y dónde falla

Hoy cada pestaña tiene un `RecoverySession` con su propio archivo
`session-<pid>-<nanos>.md` en `%LOCALAPPDATA%\VisorMD\recovery`. La escritura es
atómica (`atomicwrites`, con `sync_all`), versionada (una escritura vieja no
reemplaza a una nueva) y se dispara 3 s después del último cambio. Se borra al
guardar. Es una base sólida; los huecos son estos:

| # | Hueco | Consecuencia |
| --- | --- | --- |
| H1 | El plazo de 3 s se reinicia con cada pulsación y no hay tope | Quien escribe sin parar 5 minutos no tiene copia hasta que se detiene |
| H2 | Solo se escribe por pausa o al cerrar | Cambiar de pestaña, perder el foco o suspender no fuerzan una copia |
| H3 | Un archivo por pestaña y por sesión; los de sesiones que murieron esperan 14 días | Se acumulan archivos huérfanos (hoy: 16 en esta PC de desarrollo) |
| H4 | No se registra a qué documento pertenece cada copia ni cuál era su base en disco | No se puede ofrecer «restaurar en su archivo» ni avisar que el archivo cambió |
| H5 | Solo se restaura «la más reciente» | Con varios documentos, el resto queda inalcanzable |
| H6 | Nadie ofrece la recuperación al arrancar | La persona tiene que saber que existe y buscar «Abrir recuperación» |
| H7 | No hay forma de saber si el dueño de una copia sigue vivo | No se puede distinguir «sesión muerta» de «otra ventana abierta» (D-33: una ventana, un proceso) |
| H8 | `recovery_root()` cae a `std::env::temp_dir()` | En Linux sería `/tmp`, compartido y con nombres predecibles |
| H9 | El archivo temporal de `atomicwrites` que queda tras un corte no coincide con `session-*.md` | Nunca se limpia |
| H10 | Si la escritura falla, solo se registra en el log | La persona cree tener copia y no la tiene |
| H11 | La carga y la escritura no tienen suma de verificación | Un archivo truncado se restaura sin avisar |
| H12 | Un panic con `panic = "abort"` no deja nada | Se pierde lo escrito desde la última copia |

## 2. Qué se puede y qué no se puede garantizar

| Evento | Se puede reaccionar | Qué se pierde, como máximo |
| --- | --- | --- |
| Cerrar la ventana | Sí (`CloseRequested`) | Nada: ya fuerza la copia |
| Panic | Sí, con un gancho de pánico | Nada si el registro compartido está al día |
| Matar el proceso (Administrador de tareas, `taskkill /F`) | No | Lo escrito desde la última copia |
| Apagado o reinicio de Windows | A medias: `winit` 0.30 no expone `WM_QUERYENDSESSION` | Lo escrito desde la última copia |
| Corte de luz, pantalla azul | No | Lo escrito desde la última copia, si el disco confirmó la escritura |

Conclusión de diseño: **no se puede depender de un aviso de cierre**. El
servicio garantiza una **pérdida máxima acotada** (objetivo: 10 s de escritura)
y lo hace con copias continuas, no con un guardado de último momento. Los
ganchos de cierre son un refuerzo, no el mecanismo.

## 3. Principios

1. **Un archivo por documento, no por sesión.** Una identidad estable de
   documento (`doc_key`) tiene a lo sumo una copia. No hay historial de copias.
2. **Todo o nada.** Se escribe en un archivo temporal, se sincroniza y se
   reemplaza de forma atómica. Una lectura ve la copia anterior o la nueva,
   nunca una mezcla.
3. **El que escribe es dueño, y el dueño se sabe vivo sin adivinar.** Nada de
   «pid + reloj»: el sistema operativo libera el cerrojo cuando el proceso
   muere, de cualquier manera.
4. **Nunca se sobrescribe el trabajo de la persona.** Restaurar abre una
   pestaña; el archivo original solo cambia con Guardar, con su comprobación de
   conflicto de siempre.
5. **El contenido de una copia es un dato no confiable.** Entra por la misma
   tubería segura que abrir un archivo (límites, modo seguro, HTML inerte).
6. **Presupuesto duro.** Número de copias, bytes totales y edad tienen tope;
   al pasarlo se descarta lo más viejo y se avisa, no se acumula.
7. **Fallar a la vista.** Si no se puede escribir, la interfaz lo dice; no hay
   falsa seguridad.

## 4. Arquitectura

```text
        hilo de interfaz                                   hilo escritor
 ┌──────────────────────────┐   Rope::clone (O(1))   ┌───────────────────────────┐
 │ App / DocumentState      │ ─────────────────────► │ buzón «el último gana»    │
 │  · política de disparo   │   (doc_key, revisión,  │  · descarta si no cambió  │
 │    (policy.rs, pura)     │    Rope, metadatos)    │    (suma de verificación) │
 └──────────────────────────┘                        │  · serializa (format.rs)  │
          ▲                                          │  · escribe atómico        │
          │ estado: «copia hace 4 s» / «falló»       │  · reintenta con espera   │
          │                                          └─────────────┬─────────────┘
          │                                                        ▼
   ┌──────┴───────────┐        ┌────────────────────────────────────────────────┐
   │ gancho de pánico │──────► │ %LOCALAPPDATA%\VisorMD\recovery\               │
   │ (vuelca el       │        │   s-<sesión>.lock        ← cerrojo del proceso │
   │  registro)       │        │   s-<sesión>\<doc_key>.vmdr   ← una por doc    │
   └──────────────────┘        └────────────────────────────────────────────────┘
```

### 4.1 Módulos (en `src/recovery/` y `src/recovery.rs`, sin `unsafe`)

| Módulo | Responsabilidad | ¿Puro? |
| --- | --- | --- |
| `formato.rs` | Codificar y decodificar el archivo `.vmdr`, suma de verificación, límites | Sí |
| `politica.rs` | Cuándo escribir: pausa, tope, eventos; cuota y expulsión | Sí |
| `almacen.rs` | Rutas, sesión y cerrojo, escritura atómica, listado, adopción, barrido | No (disco) |
| `escritor.rs` | Hilo único con buzón; reintentos; estado visible | No (hilo) |
| `registro.rs` | Registro compartido que lee el gancho de pánico | No |

`main.rs` solo ve una interfaz pequeña:
`Recuperacion::{iniciar, instantanea(doc_key, rope, meta), descartar(doc_key),
forzar_todo(), huerfanas(), adoptar(doc_key), estado()}`.

### 4.2 Identidad del documento

`doc_key`: 128 bits aleatorios generados al crear la pestaña (del generador del
sistema; `getrandom` ya está en el árbol de dependencias). **No se deriva de la
ruta ni del contenido** (evita colisiones entre dos pestañas y que la ruta
termine en un nombre de archivo). Viaja con la pestaña al moverla de ventana
(D-33) y al restaurar (se adopta el mismo).

### 4.3 Formato `.vmdr`

Texto con cabecera acotada y cuerpo con longitud; se lee de a trozos y nunca se
interpreta como código.

```text
VISORMD-RECOVERY 1\n
doc_key: <32 hex>\n
session: <32 hex>\n
origin_path: <ruta o vacío, solo informativa>\n
base: <largo>:<mtime>:<crc32> del archivo en disco al abrir o guardar, o vacío\n
title: <título provisional, 80 caracteres>\n
mode: reading|editing|compare\n
cursor: <offset>\n
saved_at: <unix>\n
body_len: <bytes>\n
body_crc32: <hex>\n
\n
<body_len bytes UTF-8>
```

- La cabecera tiene tope de 4 KiB y claves conocidas; una clave desconocida se
  ignora, una versión mayor que la propia **se conserva sin tocar** y se avisa.
- Se verifica `body_len` contra el tamaño real del archivo y el CRC-32
  (`crc32fast` ya está en `Cargo.lock`; si no se quiere la dependencia directa
  son 15 líneas propias). Una copia que no pasa la verificación no se restaura:
  se muestra como «dañada» y se descarta con el barrido.
- Todo valor mostrado en pantalla pasa por la misma limpieza de caracteres de
  control que los títulos de pestaña.

### 4.4 Disposición en disco y cerrojo de sesión

```text
%LOCALAPPDATA%\VisorMD\recovery\
  s-<sesión A>.lock          ← abierto sin compartir por el proceso A mientras vive
  s-<sesión A>\<doc>.vmdr
  s-<sesión B>.lock          ← de un proceso que murió: se puede abrir
  s-<sesión B>\<doc>.vmdr    ← huérfana
  aviso-privacidad-v1
```

- Al iniciar, el proceso crea su `s-<sesión>.lock` y lo **mantiene abierto con
  `share_mode(0)`** (`std::os::windows::fs::OpenOptionsExt`, sin `unsafe`). En
  Linux, `File::try_lock` de la biblioteca estándar. Mientras el proceso viva,
  nadie más puede abrirlo; cuando muere, por la causa que sea, el sistema lo
  libera.
- **Una sesión es huérfana si su `.lock` se puede abrir.** Esto reemplaza toda
  heurística de pid y sobrevive a la reutilización de pids, a otra ventana viva
  y a un proceso congelado.
- **Adopción por renombrado atómico:** restaurar mueve
  `s-<vieja>\<doc>.vmdr` a `s-<mía>\<doc>.vmdr`. Si dos ventanas arrancan a la
  vez, una gana y la otra recibe «no encontrado» y lo ignora. El mismo mecanismo
  sirve para sacar una pestaña a otra ventana (D-33: «lo sin guardar viaja como
  copia de recuperación»).
- Una sesión sin copias ni cerrojo vivo se borra en el barrido; el directorio
  nunca crece sin que haya trabajo sin guardar.

### 4.5 Política de disparo (`politica.rs`)

| Disparo | Plazo |
| --- | --- |
| Pausa tras escribir | 2 s después del último cambio |
| **Tope** durante una ráfaga continua | 10 s desde la primera pulsación sin copia (corrige H1) |
| Cambiar de pestaña, perder el foco, minimizar, abrir un diálogo | Inmediato |
| Suspender o evento de energía de la ventana | Inmediato |
| Cerrar pestaña o ventana con cambios | Inmediato y con confirmación de escritura (como hoy) |
| Gancho de pánico | Inmediato, desde el registro |

Reglas de ahorro: no se escribe si el contenido no cambió desde la última copia
(se compara el CRC-32); no se escribe un documento vacío; no se escribe uno
idéntico a su base en disco; se borra la copia cuando el documento vuelve a ser
igual a lo guardado. Documentos mayores que 2 MiB usan un plazo de pausa de 5 s
y un tope de 30 s para que la escritura no compita con la edición.

### 4.6 Escritor (`escritor.rs`)

- **Un solo hilo**, no uno por escritura como hoy: recibe por un canal y
  conserva por `doc_key` solo la **última** instantánea pendiente («el último
  gana»). Una ráfaga produce una escritura, no cien.
- La instantánea es un `Rope::clone()`, que en `ropey` es O(1) y comparte
  memoria; se convierte a bytes en el hilo escritor, no en el de interfaz.
- Una escritura fallida se reintenta con espera creciente (2, 4, 8… 60 s). El
  estado resultante (`Al día · hace N s`, `Reintentando`, `Falló: motivo`) lo
  lee la interfaz.
- Cierra de forma ordenada: `forzar_todo()` espera a que el buzón quede vacío
  con un plazo de 3 s y devuelve si todo quedó en disco; el cierre con cambios
  solo continúa si es así (comportamiento actual, preservado).

### 4.7 Gancho de pánico y señales

- `std::panic::set_hook` toma el **registro compartido** (`Arc<Mutex<…>>` con
  el último `Rope` por `doc_key`, que cuesta lo mismo que una referencia) y
  hace una escritura síncrona de todo lo pendiente antes de abortar. Con
  `panic = "abort"` el gancho sí corre.
- El gancho no toca la interfaz ni el estado de la app: solo el registro. Si el
  cerrojo del registro está tomado, intenta `try_lock` y sigue sin él.
- **Fuera de alcance de la versión 1:** `WM_QUERYENDSESSION` / `WM_ENDSESSION`.
  Exigen `windows-sys` y `unsafe`, que `src/main.rs` prohíbe salvo en la capa de
  integración con el sistema operativo ya prevista. Se deja como la tarea 13.9,
  opcional, en un módulo propio revisado a mano. Con el tope de 10 s el apagado
  pierde poco aun sin ella.

### 4.8 Arranque, restauración y conflictos

1. Se crea la sesión y su cerrojo. Se ejecuta el barrido (4.9).
2. Se listan las sesiones huérfanas leyendo **solo cabeceras** (barato, sin
   cargar cuerpos).
3. Si hay, la interfaz muestra un aviso amable, sin ventana modal:
   «Quedaron N documentos sin guardar de la última sesión · Revisar». No se
   restaura nada por sí solo.
4. «Revisar» abre la lista de recuperaciones (tarea 6.7): título, origen, hora y
   primera línea; Enter restaura, Supr descarta con confirmación, «Restaurar
   todo» y «Descartar todo».
5. Al restaurar se **adopta** el archivo y se abre una pestaña marcada como sin
   guardar, según la base:

| Estado del archivo de origen | Resultado |
| --- | --- |
| Sin origen (documento nuevo) | Pestaña nueva; el nombre sale del título (P-01) |
| Origen sin cambios desde la base | Pestaña ligada al archivo original, marcada como modificada; Guardar usa la comprobación de conflicto de siempre |
| **Origen cambió en disco** | Pestaña **separada**, sin ruta, con el aviso «el archivo cambió desde tu copia» y la opción de abrir Comparar; nunca se pisa nada |
| Origen ya no existe | Pestaña nueva con el nombre sugerido |

### 4.9 Barrido (contra la basura)

Corre al iniciar y cada hora, sin bloquear la interfaz:

- borra sesiones huérfanas **vacías** y `.tmp` huérfanos de más de 1 h;
- borra copias dañadas de más de 24 h;
- expulsa huérfanas de más de **30 días** (hoy 14; al ser una sola copia por
  documento y listada con aviso, se puede dar más margen sin costo);
- aplica la cuota: **50 copias y 64 MiB** en total; al pasarla, se expulsa lo más
  viejo y se muestra un aviso con el número descartado;
- nunca toca nada que no se llame como lo crea este módulo, ni un enlace
  simbólico, ni una sesión con cerrojo vivo.

Resultado: en régimen, los archivos en disco son **uno por documento sin guardar
de una sesión viva**, y uno por documento sin resolver de una sesión que murió,
hasta que la persona decide.

### 4.10 Interfaz

- El punto `•` de «sin guardar» (P-01) puede mostrar en su descripción emergente
  «Copia de seguridad hace 4 s» y pasar a un tono de advertencia si la última
  escritura falló. Es la única señal permanente; no se agrega un contador nuevo.
- Mensajes (para la fase 7): «Quedaron 2 documentos sin guardar de la última
  sesión», «No se pudo guardar la copia de seguridad: <motivo>. Guardá el
  documento», «Tu archivo cambió desde la copia; abrí Comparar para elegir».
- El interruptor «Recuperación de cambios» que ya existe se conserva.

## 5. Seguridad

| Amenaza | Control |
| --- | --- |
| Otro usuario lee las copias | Carpeta bajo el perfil del usuario (`%LOCALAPPDATA%`, con herencia de permisos del perfil). **Se elimina el recurso a `temp_dir()`** (H8): si el perfil no está disponible, la recuperación se desactiva con un aviso. En Linux, carpeta `0700` y archivos `0600` creados con `create_new` |
| Enlace simbólico o punto de reanálisis en la carpeta | Cada apertura usa `create_new` para crear y rechaza reparse points o symlinks al leer, listar y borrar (igual que el VFS) |
| Un archivo manipulado desborda memoria | Tope de cabecera (4 KiB), `body_len` contra el tamaño real, tope de documento ya existente, lectura acotada |
| Un nombre o ruta del documento escapa de la carpeta | Los nombres de archivo son siempre `doc_key` y sesión (hex aleatorio); la ruta de origen es un dato que se muestra, nunca un destino |
| Restaurar sobrescribe un archivo | No hay escritura al origen sin Guardar; ver 4.8 |
| Contenido hostil al restaurar | Entra por `recovered_document` y `safe_source_blocks`, como abrir un archivo; HTML inerte |
| Dos procesos escriben el mismo archivo | Imposible por diseño: solo el dueño (cerrojo vivo) escribe; la adopción es un renombrado atómico |
| Texto sin cifrar en disco | Se mantiene el aviso honesto (D-27) y el interruptor. **Cifrado con DPAPI queda fuera de la v1**: protege de otros usuarios y de copias del disco, no de un programa que corra como la misma persona, y exige `unsafe`. Se evalúa en la 13.9 |
| «Se borró» no es «se destruyó» | No se promete borrado seguro: en SSD y con copias de sombra no se puede garantizar. La documentación lo dice |
| Información en el log | El log registra el motivo del fallo, nunca rutas ni contenido |

## 6. Estabilidad y fallos

- **Disco lleno o carpeta sin permiso:** la copia anterior sigue intacta (el
  reemplazo es atómico); se reintenta con espera; el estado visible cambia.
- **Antivirus que bloquea el reemplazo:** hasta 5 intentos con espera corta
  antes de contar el fallo (los bloqueos transitorios son comunes en Windows).
- **Reloj que cambia:** la política usa `Instant` para plazos; las fechas solo se
  muestran.
- **Documento enorme:** ver 4.5; el límite de documento ya existente corta antes.
- **Muchas pestañas:** el buzón conserva una instantánea por documento; el costo
  es el de las pestañas con cambios, no el de todas.
- **Actualización de la app con una versión de formato mayor:** las copias de una
  versión desconocida no se tocan.

## 7. Pruebas

| Capa | Pruebas |
| --- | --- |
| `formato.rs` | Ida y vuelta, cuerpo truncado, CRC distinto, cabecera enorme, clave desconocida, versión mayor, UTF-8 inválido, `body_len` mentiroso |
| `politica.rs` | Pausa, tope en ráfaga continua, eventos inmediatos, no escribir sin cambios, cuota y orden de expulsión |
| `almacen.rs` | Cerrojo vivo contra huérfano, adopción con dos hilos compitiendo (uno gana), barrido de `.tmp`, enlaces simbólicos rechazados, nombres solo hex |
| **Muerte real del proceso** | Prueba de integración que lanza el propio ejecutable de pruebas como hijo, le hace escribir copias, lo mata con `Child::kill` (equivale a `taskkill /F`), y verifica que la sesión queda huérfana y el contenido se recupera íntegro |
| Fallos | Directorio de solo lectura, disco simulado lleno, escritura interrumpida entre el temporal y el reemplazo |
| Interfaz | Escenas visuales del aviso y de la lista de recuperaciones, noche y día, 640 y 1100 px |
| Manual (`docs/manual-qa-v2.md`) | Escribir 30 s, matar con el Administrador de tareas, reabrir; escribir y reiniciar Windows con `shutdown /r /t 0`; documento nuevo y documento con archivo; archivo modificado fuera mientras tanto |

## 8. Alternativas descartadas

| Alternativa | Por qué no |
| --- | --- |
| Autoguardado sobre el propio archivo | Contradice «no hay autoguardado por defecto» y es peligroso con sincronizadores y Git |
| Diario de cambios (WAL) con deltas | Menos escritura, pero un archivo parcial es difícil de validar y la compactación es una segunda fuente de errores. Con una instantánea de pocos KiB cada 2 a 10 s, la instantánea completa es más simple y más robusta |
| Base de datos (SQLite) | Una dependencia de C contradice el ADR-2 por un caso que un archivo atómico resuelve |
| Un archivo por pestaña y por sesión (hoy) | Es la causa de la basura (H3) y no permite saber a qué documento pertenece |
| Detectar al dueño por pid | Los pids se reutilizan; el cerrojo no |
| Servicio residente o proceso aparte | Un agujero más de seguridad, ya que otro proceso tendría que recibir el texto, y un proceso residente sin pedirlo |
| Restaurar solo al abrir, sin aviso | Nadie sabe que existe (H6) |
| Restaurar automáticamente todo | Sorprende, mezcla trabajo viejo con nuevo y expone contenido sin que la persona lo pida |

## 9. Cómo encaja con el resto del plan

- **P-01 (título provisional):** `title` y el nombre sugerido salen de
  `src/titulo.rs`.
- **Fase 6.7 (lista de recuperaciones):** se construye sobre `huerfanas()` y
  `adoptar()`; no se hace por separado.
- **Fase 12 / D-33 (ventanas):** el traspaso de pestañas entre procesos es la
  misma adopción por renombrado.
- **Fase 7 (lenguaje, D-27):** el aviso de arranque y los mensajes de fallo
  siguen su glosario.
- **Fase 8:** la señal de «copia hace N s» comparte el sistema de descripciones
  emergentes.

## 10. Riesgo residual

- Matar el proceso o cortar la luz pierde, como máximo, lo escrito en los
  últimos 10 s (30 s en documentos de más de 2 MiB).
- Sin el gancho de apagado de Windows (13.9), un reinicio forzado tiene el mismo
  máximo; no hay forma de reducirlo sin `unsafe`.
- El texto queda sin cifrar mientras el documento no se guarde; la medida es
  acotar el tiempo (30 días), el volumen (cuota) y el acceso (perfil), y decirlo.
