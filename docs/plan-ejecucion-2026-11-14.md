# Plan de ejecución · rediseño y correcciones

Plan para ejecutar sin acompañamiento el plan definitivo de
[`evaluacion-usabilidad-2026-11-14.md`](evaluacion-usabilidad-2026-11-14.md#plan-definitivo-de-cambios).
Las decisiones técnicas ya están tomadas (sección **Decisiones**). El agente
que ejecuta no las rediscute: las aplica. Si una resulta imposible, sigue la
regla de escalado de [`agente-ejecutor.md`](agente-ejecutor.md).

**Cómo se usa.** Abrir [`agente-ejecutor.md`](agente-ejecutor.md) (flujo,
comandos y reglas de parada) y tomar la **primera tarea `[ ]` de la fase
activa** en la tabla «Estado» del final. Cada tarea indica:

- Qué hacer.
- Dónde: nombre de la función actual. Los números de línea cambian; los nombres
  no.
- Prueba: qué se agrega y debe pasar.
- Hecho: la condición observable.

## Herramientas

| Herramienta | Estado | Uso |
| --- | --- | --- |
| kompari (`kompari.exe` en `~/.cargo/bin`) | Instalada | `kompari review tests/visual/referencia target/visual/actual` para ver y aprobar imágenes de referencia |
| Accessibility Insights for Windows 1.1 | Instalada | Analizador de contraste (Ctrl+R) en la tarea 8.8 |
| Plugin Design de Anthropic (`design@knowledge-work-plugins` v1.2.0) | Instalado con `claude plugin install`; se carga al iniciar una sesión nueva | `design-critique` al cerrar las fases 3, 5 y 8; `ux-copy` en la 7; `accessibility-review` en la 8; `user-research` en la 9. **Si no está disponible, se salta ese paso y se anota en «Estado»; nunca bloquea.** |
| Herramienta de evaluación ([`scripts/evaluacion/`](../scripts/evaluacion/README.md)) | Lista | Capturas en vivo al cerrar cada fase; re-evaluación en la fase 9 |

## Decisiones

Tomadas el 14 de noviembre de 2026. Cada una cierra una pregunta que de otro
modo quedaría a criterio del ejecutor.

### Producto (las tres pendientes de la evaluación)

- **D-01. Edición.** Se queda con Markdown y más ayudas. No hay editor visual.
  Las ayudas son:
  - botón «Título»;
  - inserciones que respetan el contexto;
  - acciones de tabla;
  - Comparar como forma de aprender.
- **D-02. Enlaces externos.** Siguen en azul por seguridad y llevan el ícono ↗
  después del texto.
- **D-03. Tecla T.** *Reemplazada por D-30*: la tecla T sola se retira y el
  atajo pasa a Ctrl+Shift+D. Se suma un botón sol/luna, y al cambiar el tema
  aparece el aviso «Tema de día · Ctrl+Shift+D para volver» (o «de noche»).

### Funcionamiento

- **D-04. Texto tecleado.** Una sola función pura decide si un evento aporta
  texto a un campo de la interfaz:

  ```rust
  /// Texto que un evento de teclado aporta a un campo de la interfaz.
  /// Ctrl o Alt nunca producen texto; se descartan los caracteres de control.
  fn typed_text(text: Option<&str>, control: bool, alt: bool) -> Option<&str> {
      if control || alt { return None; }
      text.filter(|t| !t.is_empty() && !t.chars().any(char::is_control))
  }
  ```

  La usan la búsqueda en el documento, la búsqueda en la carpeta, el filtro del
  selector de símbolos y la paleta, que se refactoriza para usarla.
  `direct_source_text` (editor) no cambia: admite Tab y saltos de línea con sus
  propias reglas.
- **D-05. Contador en vivo.** No hay temporizador. `DocumentMetrics::from_source(&str)`
  se calcula en el cuadro solo cuando cambia `source_editor.revision()`, con una
  caché `metrics_cache: Option<(u64, DocumentMetrics)>` en `DocumentState`. En
  Leer se sigue usando `from_blocks`.
- **D-06. Clic sin puntero.** Al principio del manejador de clic izquierdo y del
  derecho, `let Some((x, y)) = self.pointer else { return; };`. Se eliminan los
  `self.pointer.map_or(0.0, …)` de ese manejador.
- **D-07. Rótulo de guardado.** `fn save_state_label(identity: bool, dirty: bool) -> &'static str`:
  - `"nuevo"` si no tiene identidad y no está sucio;
  - `"sin guardar"` si está sucio;
  - `"guardado"` en otro caso.

### Infraestructura de dibujo y pruebas

- **D-08. Dibujo sin ventana.** `redraw` se divide en tres partes:
  1. obtener el tamaño de `self.window` (sin ventana, terminar);
  2. `self.render_frame(w, h)?`, todo lo que hoy va desde el cálculo de
     `frame_start` hasta antes de presentar en `surface`, y que deja el cuadro
     en `self.pixmap`;
  3. presentar en softbuffer.

  Dentro de `render_frame`, las llamadas a la ventana usan
  `if let Some(window) = &self.window`, que ya es el patrón actual. Con
  `window == None` no hacen nada.
- **D-09. Proxy en pruebas.** El campo `proxy` pasa a ser `AppNotifier`:

  ```rust
  #[derive(Clone)]
  struct AppNotifier(Option<EventLoopProxy<AppEvent>>);
  impl AppNotifier {
      fn send_event(&self, event: AppEvent) -> Result<(), ()> {
          match &self.0 { Some(p) => p.send_event(event).map_err(|_| ()), None => Err(()) }
      }
  }
  ```

  Hay que ajustar los 22 usos de `proxy`. Un `Err` en pruebas equivale a que el
  trabajo en segundo plano no ocurre. Las pruebas cargan documentos de forma
  sincrónica (D-11).
- **D-10. Construcción de `App`.** El literal `App { … }` de `main` se mueve a
  `App::new(settings, font_cx, notifier, opening_path, …)`. `main` y las
  pruebas lo llaman. En pruebas: `App::for_tests(width, height, palette)`,
  con `AppNotifier(None)`, recuperación desactivada, `reduce_motion = true` y
  fuentes solo embebidas:

  ```rust
  let mut font_cx = FontContext {
      collection: Collection::new(CollectionOptions { shared: false, system_fonts: false }),
      source_cache: SourceCache::default(),
  };
  register_embedded_fonts(&mut font_cx);
  ```

- **D-11. Cargar Markdown en pruebas.** Se escribe
  `fn load_for_test(&mut self, markdown: &str, mode: DocumentMode)`. Fija la
  fuente, llama a `self.document.content_kind.prepare_view(markdown)` y aplica
  el resultado con `apply_view_outcome`, igual que la exportación PDF.
- **D-12. Imágenes de referencia.**
  - Módulo `src/visual_tests.rs`, declarado con `#[cfg(test)] mod visual_tests;`.
  - PNG por `Pixmap::encode_png` y `Pixmap::load_png` (tiny-skia ya tiene
    `png-format`).
  - Ruta: `tests/visual/referencia/{escena}-{noche|dia}-{w}x{h}.png`.
  - Si falla, escribe `target/visual/actual/…` y `target/visual/diff/…`, con los
    píxeles distintos en magenta.
  - Tolerancia: diferencia de canal ≤ 2, y como máximo el 0,1 % de los píxeles
    puede superarla.
  - `VISOR_BLESS=1 cargo test visual` reescribe las referencias.
  - Escenas: unas 18, en noche y día y a 640×480 y 1100×720; hasta 72 imágenes.
    Se empieza con 9 escenas en la tarea 2.4 y crecen con cada fase.
  - Si la CI de Linux da diferencias por rasterizado, se marca
    `#[cfg_attr(not(windows), ignore)]` y se anota en [`testing.md`](testing.md).
- **D-13. Objetivos interactivos.** `src/ui_target.rs` define `enum UiTarget`, con
  datos `Copy` y sin referencias:

  ```rust
  pub(crate) enum UiTarget {
      Toolbar(usize), ContextToolbar(usize), ModeSegment(u8),
      Tab(usize), TabClose(usize), NewTab, WindowControl(u8),
      ListRow { list: ListId, index: usize }, MenuRow(usize),
      CodeCopy { block: usize, preview: bool }, PaneHeader { pane: u64, part: u8 },
      WorkspaceDestination(u8), DialogButton(usize), TaskCheckbox(usize),
      SectionDisclosure(usize), ScrollThumb,
  }
  ```

  `App::ui_target_at(x, y) -> Option<UiTarget>` vive en `main.rs` y reutiliza
  las funciones de geometría existentes, sin duplicarlas.
- **D-14. Animación.**
  - Constantes: `HOVER_FADE = 120 ms` y `PRESS_FLASH = 160 ms`.
  - `fn transition_progress(started, now, duration, reduce_motion) -> f32` con
    smoothstep.
  - `about_to_wait` agenda el siguiente cuadro solo mientras alguna transición
    no terminó.
  - Con `reduce_motion`, el progreso es siempre 1.
- **D-15. Tooltip.**
  - Aparece tras 400 ms de puntero quieto sobre el mismo `UiTarget`.
  - Es una sola capa, dibujada al final de `render_frame`, encima de todo
    menos los diálogos.
  - Nunca se dibuja con un diálogo abierto.
  - Se oculta al pulsar, con la rueda o al abrir un menú.

### Listas y menús

- **D-16. `ListView`.** Módulo puro `src/list_view.rs`:

  ```rust
  pub(crate) struct ListView { pub selected: usize, pub top: usize }
  impl ListView {
      pub fn move_by(&mut self, delta: isize, len: usize, capacity: usize);   // sin vuelta en los extremos
      pub fn page(&mut self, down: bool, len: usize, capacity: usize);        // capacity - 1 filas
      pub fn home(&mut self); pub fn end(&mut self, len: usize, capacity: usize);
      pub fn scroll_rows(&mut self, rows: isize, len: usize, capacity: usize); // rueda: no cambia `selected`
      pub fn ensure_visible(&mut self, capacity: usize);
      pub fn visible(&self, len: usize, capacity: usize) -> std::ops::Range<usize>;
      pub fn row_to_index(&self, row: usize, len: usize) -> Option<usize>;
  }
  ```

  La rueda (`LineDelta(_, y)`) desplaza `-(y * 3.0).round()` filas, y
  `PixelDelta` desplaza `-y / PANEL_ROW_HEIGHT`. Reemplaza a `panel_window` y
  `panel_item_at`. Cada lista guarda su `ListView` en lugar del índice suelto.
- **D-17. Alternancia.** `fn open_surface(&self) -> Option<AppAction>` devuelve la
  acción que abrió lo que está visible. Al pulsar un botón con la acción `a`:
  - si `open_surface() == Some(a)`, se cierra;
  - si está abierta otra superficie, se cierra y se ejecuta `a` en el mismo clic;
  - si no hay nada abierto, se ejecuta `a`.

  Clic fuera de un panel flotante:
  - si cae sobre un `UiTarget` de barra, pestaña o selector de modo, se cierra
    el panel **y** se procesa el clic;
  - en el lienzo, solo se cierra (como hoy).
- **D-18. Geometría de menús.**
  - Ancho de 320 px en los menús y de 460 px en la paleta.
  - Los menús se anclan al borde izquierdo del botón que los abrió, en
    `y = CONTEXT_TOOLBAR_Y + CONTEXT_TOOLBAR_HEIGHT + 4`. Si no entran a la
    derecha, se alinean al borde derecho de la ventana menos `MARGIN`.
  - La paleta va en `x = MARGIN`, `y = DOCUMENT_VIEWPORT_TOP + 8`.
  - Hasta 14 filas visibles.
  - La barra de desplazamiento mide 4 px y va a 2 px del borde.
- **D-19. Filas.**
  - Ícono de 16 px a 12 px del borde.
  - Nombre a 36 px.
  - Atajo alineado a la derecha, a 12 px, en `dim`.
  - Hover: acento con alfa 20.
  - Fila elegida: acento con alfa 44 y radio de 6 px.

  `AppAction::label()` pierde el «· atajo» y se agrega
  `AppAction::shortcut() -> Option<&'static str>`.

### Edición

- **D-20. Salir de una lista.** Enter en un ítem vacío borra el marcador y deja
  la línea vacía; el cursor queda en una línea nueva debajo. `"- a\n- |"` pasa a
  `"- a\n\n|"`. La línea en blanco es la que separa la lista del párrafo.
- **D-21. Bloques.** `fn insert_block(source, cursor, template, eol)` en
  `editor.rs`:
  - Si el cursor no está al inicio de una línea vacía, primero abre una línea
    nueva.
  - Garantiza una línea en blanco antes, salvo al inicio del documento, y una
    después, salvo al final.

  La usan tabla, código, regla, recuadro y las plantillas de estudio. Ajuste
  hecho al verificar en vivo: un bloque nunca reemplaza texto seleccionado ni
  parte una línea. Va debajo de la línea del cursor o, si el cursor está al
  inicio de una línea con texto, encima de ella. Solo el bloque de código con
  selección la envuelve.
- **D-22. Formato sin selección.** Si el cursor está dentro de una palabra o al
  final de ella (alfanuméricos Unicode y guion bajo), se envuelve la palabra.
  Con el cursor justo antes de una palabra, entre espacios o en una línea
  vacía, se inserta el texto guía seleccionado, como antes. (Ajuste hecho al
  implementarla: una prueba existente fijaba que, antes de una palabra, se
  inserta el texto guía.)

### Visual y lenguaje

- **D-23. Pestañas.** Roles nuevos en `Palette`:

  | Tema | `tab` | `tab_idle` |
  | --- | --- | --- |
  | Noche | `#17301F` | `#112218` |
  | Día | `#DCE6CF` | `#EDE4CF` |

  La prueba de contraste se extiende: `text`/`tab` ≥ 7, `dim`/`tab` ≥ 4,5 y
  `dim`/`tab_idle` ≥ 4,5.
- **D-24. Contraste del tema de día.** Medido el 14 de noviembre:
  - `dim` pasa de `#6C6151` a `#5A5043`:

    | Fondo | Antes | Después |
    | --- | --- | --- |
    | `surface` | 5,2:1 | 6,8:1 |
    | `bg` | 4,5:1 | 5,9:1 |
    | `elevated` | 4,7:1 | 6,1:1 |
    | `tab` | — | 6,1:1 |

  - `external_link` de día pasa de `#1E68C4` (4,7:1) a `#1A5AAE` (5,8:1).
  - Los íconos de las barras usan `dim` y nunca un color más claro.
  - La prueba de `theme.rs` sube los mínimos de día: `dim` ≥ 5,5 sobre
    `surface` y `external_link` ≥ 5,5.
- **D-25. Glosario.**

  | Se dice | En lugar de |
  | --- | --- |
  | Carpeta de apuntes | workspace, espacio de trabajo, bóveda, carpeta de trabajo |
  | Notas que enlazan aquí | Backlinks |
  | Recuadro | callout |
  | Enlace a otra nota | enlace de bóveda, wikilink |
  | Guardado | guardado de forma atómica |
  | Copiar para hoja de cálculo | TSV |
  | Título / Subtítulo | encabezado H1 / H2–H6 |
  | Recuperación | recuperación local temporal |

  Se usa voseo en todo: «Escribí», «Elegí», «Abrí». Los términos retirados y las
  formas de tuteo («escribe», «elige», «abre», «pulsa») quedan prohibidos en los
  textos de la interfaz por la prueba de la tarea 7.4. La prueba no se aplica a
  los registros (`self.log`).
- **D-26. Avisos.**
  - Duran 2,4 s.
  - Se centran en el área del documento (`document_area_geometry`) a 24 px del
    borde inferior y se dibujan sobre paneles y columna.
  - No se emiten al abrir paneles o menús: esas pistas pasan al pie del panel.
  - Se conservan los de confirmación (guardado, copiado sin botón propio,
    exportado) y los de error.
- **D-27. Primer arranque.**
  - Si no hay archivo de argumento, la pestaña inicial es «Bienvenida»: una
    vista de lectura con tres botones grandes (Abrir carpeta de apuntes, Abrir
    archivo y Nuevo apunte) y dos líneas sobre Leer, Editar y Comparar.
  - Se cierra al abrir cualquier documento.
  - El diálogo de privacidad de la recuperación pasa a un aviso de 6 s, «Tus
    cambios sin guardar se protegen en este equipo · Más detalles», que abre el
    diálogo actual.
- **D-28. PDF.** Los operadores de texto invisible se emiten en orden de bloque
  y de línea. Los enlaces y las celdas no duplican texto. Una prueba extrae las
  cadenas `Tj` de la salida de `export_pdf_bytes` y compara su orden con el
  texto de los bloques.

### Barra, menús, mosaico y ventanas

Diseño completo, inventario de las 69 acciones y riesgos en
[`rediseno-barra-mosaico-ventanas-2026-11-15.md`](rediseno-barra-mosaico-ventanas-2026-11-15.md).

- **D-29. Menús.** La paleta «···» deja de ser el menú de todo y pasa a ser
  solo un buscador de acciones. Los menús son **Archivo, Vista y Carpeta**
  (fila 1, con **Editar** entre Archivo y Vista) y **Párrafo, Insertar y Estudio** (fila 2,
  según el modo). «Más formato Markdown» (`WRITING_ACTIONS`) se elimina; sus
  acciones ya están en otro menú. Cada `AppAction` está en al menos un menú o
  en la lista `PALETTE_ONLY`; una prueba lo exige y también que ningún menú
  repita una acción. La tabla de destinos está en la sección 2 del diseño.
- **D-30. Tema.** Botón día/noche como **primer** control de ventana (antes de
  la chincheta). Luna en día, sol en noche. Atajo **Ctrl+Shift+D**. La tecla T
  sola deja de cambiar el tema. El atajo se resuelve antes que los campos de
  texto y los atajos de lista. `WINDOW_CONTROL_COUNT` pasa a 5 y el ancho de
  cada control a 36 px; el botón se quita por debajo de 640 px.
- **D-31. Cada archivo una sola vez.** Con 2 a 4 paneles, la tira superior lista
  solo las pestañas que no están en un panel; la cabecera de cada panel es la
  pestaña de su archivo (nombre, cambios, selector de modo y «Quitar del
  mosaico» con ícono propio, no ✕). Con un solo panel no hay cabecera y la
  tira lista todo, como hoy.
- **D-32. Operaciones del mosaico.**
  - Divisor arrastrable de 6 px, límites 15–85 %, doble clic vuelve a 50 %,
    mínimo útil de 240 px por panel y Alt+Shift+flechas en pasos de 5 %.
  - Intercambio arrastrando una cabecera sobre otra.
  - Soltar una pestaña de la tira sobre un panel: centro reemplaza, borde
    (25 % exterior) divide.
  - Soltar una cabecera sobre la tira: «Quitar del mosaico».
  - Zonas de soltado con acento al 18 % y borde de acento; Esc cancela; con
    movimiento reducido no hay fantasma animado.
  - `normalized_pane_fraction` pasa a 0,15–0,85; nuevos `swap_panes` y
    `set_fraction_at` en `panes.rs`, con pruebas.
- **D-33. Ventanas.** Un proceso por ventana.
  - Registro `%LOCALAPPDATA%\VisorMD\ventanas\<pid>.json` con latido cada 2 s;
    se ignora y borra pasados 6 s.
  - Buzón `%LOCALAPPDATA%\VisorMD\buzon\<pid>\<n>.json`, leído al recibir el
    foco y cada 500 ms.
  - Lo sin guardar viaja como copia de recuperación; el origen cierra la
    pestaña solo tras la confirmación del destino.
  - Módulo nuevo `src/ventanas.rs`, sin hilos por mensaje.
- **D-34. Asociación de archivos.**
  - Registro por usuario (solo `HKCU`) con `%SystemRoot%\System32\reg.exe`, sin
    shell ni dependencias nuevas, y lectura de vuelta para verificar.
  - Después se abre `ms-settings:defaultapps`: Windows exige que la persona
    confirme.
  - Diálogo propio con Markdown (`.md .markdown .mdown .mkdn`), texto (`.txt`)
    y otros (`.json .yaml .yml .toml .csv`).
  - Ajuste «Abrir archivos externos»: en la ventana abierta (por defecto) o en
    una ventana nueva.
  - La línea de comandos acepta varias rutas.
  - Módulo nuevo `src/asociacion.rs`; la prueba usa la raíz
    `HKCU\Software\VisorMDPrueba`, nunca el registro real.
- **D-35. Texto recortado.** Todo texto de lista se recorta por ancho medido
  (`fit_menu_layout`, hecho). Una fila recortada muestra su texto completo en el
  tooltip de D-15. *Supuesto*: la frase del propietario sobre «pasar el cursor
  por encima» quedó cortada; se tomó como este tooltip y se confirma al
  probar.
- **D-36. Menú con secciones.** `enum MenuEntry { Item(AppAction), Heading(&str) }`
  en `src/menu.rs`. Los encabezados hacen de separadores y no se seleccionan; la
  navegación los salta (no hay `Separator`: las filas tienen todas la misma
  altura). Las alternancias muestran «✓» a la izquierda cuando están activas.
- **D-37. Barra adaptable.** `top_menu_actions(window_width)` y
  `top_menu_geometry` reparten el espacio de la fila 1: con lugar, Archivo,
  Vista, Carpeta y el buscador; si no, un solo «≡» con todo adentro (no hay
  nivel intermedio de íconos). Los mismos rectángulos sirven para dibujo,
  puntero, foco y tooltip. A 640 px caben los cuatro botones directos, el
  selector de modo, «≡» y los cinco controles. Prueba de solapes de 640 a
  2000 px.
- **D-38. Recuperación continua con pérdida acotada.** Ninguna garantía puede
  depender de un aviso de cierre (matar el proceso y el apagado de Windows no lo
  dan). El servicio escribe copias continuas: pausa de 2 s, **tope de 10 s** en
  una ráfaga continua, y de inmediato al cambiar de pestaña, perder el foco,
  abrir un diálogo o cerrar. Pérdida máxima: 10 s (30 s en documentos de más de
  2 MiB). Detalle en `docs/arquitectura-recuperacion-2026-11-15.md`.
- **D-39. Una copia por documento.** Identidad `doc_key` de 128 bits aleatorios
  (no derivada de la ruta ni del contenido); una sola copia atómica por
  documento, sin historial. Formato `.vmdr` con cabecera acotada, `body_len` y
  CRC-32; una copia dañada no se restaura. Cuota de 50 copias y 64 MiB; las
  huérfanas expiran a los 30 días; se vacían los temporales huérfanos.
- **D-40. Dueño por cerrojo, no por pid.** Cada proceso mantiene abierto sin
  compartir `s-<sesión>.lock` (`share_mode(0)` en Windows, `File::try_lock` en
  Linux, sin `unsafe`). Una sesión es huérfana si su cerrojo se puede abrir. La
  adopción de una copia es un renombrado atómico entre carpetas de sesión; es
  también el traspaso de una pestaña entre ventanas (D-33).
- **D-41. Restaurar nunca pisa.** Al arrancar se avisa sin ventana modal y se
  abre la lista de recuperaciones (6.7); nada se restaura solo. Origen sin
  cambios: pestaña ligada al archivo; origen cambiado: pestaña separada con
  aviso y Comparar; origen ausente: pestaña nueva. El contenido pasa por
  `recovered_document` y el modo seguro como cualquier archivo.
- **D-42. Un solo hilo escritor con el último gana, y gancho de pánico.**
  `Rope::clone` (O(1)) hacia un buzón por `doc_key`; reintentos con espera; un
  registro compartido que `std::panic::set_hook` vuelca antes del aborto. Un
  fallo de escritura se ve en la interfaz (el punto «•» cambia de tono y la
  descripción dice «Copia de seguridad hace N s»).
- **D-43. Seguridad del almacén.** Sin recurso a `temp_dir()` (si no hay perfil,
  la recuperación se desactiva con aviso); `create_new`; se rechazan enlaces
  simbólicos y puntos de reanálisis; nombres solo en hex; la ruta de origen es
  un dato mostrado, nunca un destino; el registro no contiene rutas ni
  contenido. Cifrado con DPAPI y `WM_ENDSESSION` fuera de la v1 (13.9, opcional,
  en un módulo aislado y revisado a mano porque exigen `unsafe`).

## Fases y tareas

Leyenda: `[ ]` pendiente, `[x]` hecha (con el hash corto del commit), `[~]`
aparcada (con el motivo).

### Fase 1 · Correcciones funcionales

- [x] (a76ddca) **1.1** Agregar `typed_text` (D-04) y usarlo en `handle_search_key` para
  llamar a `append_search_text`.
  - Prueba: `typed_text(Some("a"), false, false) == Some("a")`; con `control` o
    `alt` da `None`; `"\r"` da `None`.
  - Hecho: con la herramienta de evaluación, `type` escribe en Buscar en
    documento.
- [x] (a76ddca) **1.2** Lo mismo en `handle_workspace_search_key` con
  `append_workspace_search_text`, y la paleta pasa a usar `typed_text`.
  - Prueba: las de 1.1.
  - Hecho: «napoleon» encuentra `historia.md` en `scripts/evaluacion/apuntes`.
- [x] (a76ddca) **1.3** Aplicar D-06 en los manejadores de clic izquierdo y derecho.
  - Prueba: extraer `fn click_point(pointer: Option<(f32, f32)>) -> Option<(f32, f32)>`
    si hace falta para probarlo.
  - Hecho: no quedan `pointer.map_or(0.0` en el manejador de clic.
- [x] (5f9c2ae) **1.4** Implementar D-05.
  - Prueba: `from_source` con `"# Hola mundo\n\n- uno"` da 3 palabras;
    tildes y emojis cuentan bien; el código en cerca cuenta como texto.
  - Hecho: al escribir en Editar, el contador cambia.
- [x] (5f9c2ae) **1.5** Implementar D-07.
  - Prueba: la tabla de 4 combinaciones.
  - Hecho: un documento nuevo dice «nuevo».
- [x] (1a74b6f) **1.6** Concordancia: «1 resultado» y «n resultados»; el pie y la cabecera
  del árbol cuentan lo mismo.
  - Prueba: una función `plural(n, "resultado", "resultados")` con pruebas.
- [x] **1.7** Cierre de fase: release, captura de la búsqueda funcionando y
  ejecutable nuevo para el propietario.

### Fase 2 · Dibujo sin ventana, objetivos y animación

- [x] (01bacf8) **2.1** D-09 `AppNotifier`.
  - Hecho: compila, todas las pruebas verdes y la app funciona igual (captura).
- [x] (01bacf8) **2.2** D-10 `App::new` y `App::for_tests`.
  - Prueba: `App::for_tests(1100, 720, NIGHT)` se construye.
- [x] (898665a, 165e8bd) **2.3** D-08 `render_frame`, en dos commits:
  1. mover el código sin cambios;
  2. sustituir el acceso a la ventana.

  - Prueba: una prueba dibuja un documento cargado con D-11 y comprueba que el
    píxel (5,5) no sea transparente.
  - Hecho: `cargo run --release -- --bench` no empeora más de 5 %.
- [x] **2.4** D-12 imágenes de referencia. Escenas iniciales: `lectura`,
  `edicion`, `comparar`, `paleta`, `menu-bloques`, `indice`, `dos-pestanas`,
  `dialogo-cierre` y `aviso`.
  - Hecho: `VISOR_BLESS=1` genera las referencias, `cargo test visual` pasa y
    la documentación está en [`testing.md`](testing.md).
- [x] **2.5** D-13 `UiTarget` y `ui_target_at`. Reemplazar el cálculo del cursor
  de mano en `CursorMoved` por `ui_target_at(...).is_some()`.
  - Prueba: el centro de cada botón de barra, pestaña, control de ventana y
    Copiar a 640, 1100 y 1366 px devuelve su objetivo.
- [x] **2.6** D-14 estado de hover y pulsación, todavía sin dibujarlo.
  - Prueba: `transition_progress` en 0, la mitad, el final y con Reducir
    movimiento.
- [x] **2.7** D-15 capa de tooltip.
  - Prueba: escena `tooltip-columna` y escena «diálogo sin tooltip».
  - Hecho: se corrige el hallazgo del tooltip encima del diálogo.

### Fase 3 · Listas, menús y alternancia

- [x] **3.1** D-17 `open_surface` y la tabla de alternancia.
  - Prueba: tabla pura con 4 casos por botón de panel.
- [x] **3.2** D-17, clic fuera que también actúa.
  - Prueba: con la paleta abierta, un clic en Leer cambia el modo en un solo
    clic.
- [x] **3.3** D-16 `ListView`, como módulo y sus pruebas, sin integrarlo.
- [x] **3.4** Integrar `ListView` en la paleta y los cinco menús.
- [x] **3.5** Integrarlo en el índice, el árbol, la búsqueda en la carpeta, las
  notas que enlazan aquí, el diagnóstico, los símbolos y la división.
- [x] **3.6** La rueda va a la lista bajo el puntero (`MouseWheel`).
  - Prueba: con un puntero sobre la columna, la rueda no cambia `scroll`.
- [x] **3.7** D-18 geometría, `draw_floating_surface` y barra de desplazamiento.
  - Referencias: `paleta`, `menu-bloques` y `menu-mas` a 480 y 720 de alto.
- [x] **3.8** D-19 filas y `shortcut()` (hecho: atajo a la derecha y filas
  redondeadas; los íconos de fila y los grupos de la paleta quedan para la
  fase 7, junto con los íconos nuevos).
  - Prueba: ningún `label()` contiene «·».
- [ ] **3.9** Caja de búsqueda con borde, cursor y estado vacío «Escribí para
  buscar…» en la paleta y en la búsqueda de carpeta. Buscar en documento pasa a
  ser una barra anclada arriba a la derecha.
- [x] **3.9b** La búsqueda en el documento y en la carpeta ignora tildes y
  mayúsculas, con el mismo plegado que usa la paleta (`search_fold`).
  Decisión: «ñ» se pliega a «n», igual que en la paleta, para no complicar la
  regla.
  - Prueba: «napoleon» coincide con «Napoleón»; «ano» coincide con «año».
  - Origen: la verificación en vivo de la fase 1 mostró que «napoleon» daba 0
    resultados contra «Napoleón».
- [x] **3.10** Símbolos: emojis primero, filtro con `typed_text` y columna de
  24 px.
- [x] **3.11** Índice: abre con la sección visible elegida. En el flotante, la
  sección activa es el último título cuya `y` ≤ `scroll + viewport / 3` y, al
  final del documento, la última. Los títulos se truncan con «…».
- [x] **3.12** Alternancias con estado: rótulos dinámicos con ✓.
- [~] **3.13** Pasa a la 10.7 (menú contextual, junto con los menús nuevos). Alcance original:
  - en Editar: Cortar, Copiar, Pegar y Deshacer;
  - en una tarea: Marcar o Desmarcar;
  - en un enlace: Abrir y Copiar dirección;
  - en una tabla: Agregar fila y Agregar columna;
  - volteo vertical si no entra.
- [ ] **3.14** Cierre de fase:
  - capturas;
  - `design-critique`;
  - corregir lo S2+;
  - `/code-review`;
  - ejecutable nuevo.

### Fase 4 · Integridad al editar

Es independiente de las fases 2 y 3. Toca solo `editor.rs` y las funciones
`insert_*` de `main.rs`.

- [x] (c441983) **4.1** D-20 en `SourceEditor::insert_newline`.
  - Prueba: viñetas, numeradas, tareas y citas, con LF y CRLF.
- [x] (c3b3a76) **4.2** D-21 `insert_block`, y migrar `insert_code_block`,
  `insert_table`, `insert_callout` y las de cita, regla y pregunta.
  - Prueba: al inicio, en medio de un párrafo, después de una lista, al final y
    en un documento vacío.
- [x] (1c6f87a) **4.3** D-22 formato sobre la palabra.
  - Prueba: `resu|men` → `**resumen**`; con un emoji adyacente; con un cursor
    al borde de la palabra.
- [x] (98f6793) **4.4** Símbolo dentro de una tabla, en la última celda.
  - Prueba: después de insertarlo, la fila sigue siendo una fila de tabla en
    `prepare_view`.

### Fase 5 · Pestañas y paneles

- [x] **5.1** D-23 roles y prueba de contraste.
- [ ] **5.2** Hover en las pestañas, ✕ al pasar el puntero y tooltip con el
  nombre completo (usa D-15).
- [ ] **5.3** La pestaña fijada va al inicio de `tab_order`, compacta (solo
  nombre y chincheta) y sin ✕.
- [ ] **5.4** Abrir un archivo reemplaza la pestaña inicial si no tiene
  identidad, está vacía y no está sucia.
- [~] **5.5** Pasa a la 11.7 (mosaico).
- [~] **5.6** Pasa a la 11.7 (mosaico).
- [~] **5.7** Pasa a la 11.1 (mosaico).
- [ ] **5.8** Cierre de fase: referencias `dos-pestanas`, `pestana-fijada` y
  `division`; `design-critique`.

### Fase 6 · Enlaces, carpeta de apuntes y recuperación

- [ ] **6.1** D-02: aviso «Abriendo en el navegador: dominio» e ícono ↗.
- [ ] **6.2** Enlace roto:
  - un `[[nota]]` que no resuelve en el índice se dibuja con subrayado punteado;
  - al pasar el puntero, el destino dice «Esta nota no existe»;
  - al hacer clic, se ofrece un diálogo «Crear nota» que crea un `.md` vacío
    dentro de la carpeta autorizada, con la validación de Guardar como.
- [ ] **6.3** `[[nota]]` sin carpeta: diálogo «Usar la carpeta de este archivo
  como carpeta de apuntes» (ejecuta `choose_workspace` con la carpeta del
  documento) o «Cancelar».
- [ ] **6.4** Una nota abierta desde un enlace abre en Leer.
- [ ] **6.5** Al elegir la carpeta se muestra el árbol. Además:
  - se reindexa después de cada Guardar o Guardar como dentro de la carpeta;
  - la nota abierta se resalta;
  - las carpetas llevan chevron;
  - las rutas se muestran con « › ».
- [ ] **6.6** Notas que enlazan aquí, con un extracto de la línea (hasta 80
  caracteres).
- [ ] **6.7** Lista de recuperaciones (`ListView`) con origen, fecha y primera
  línea; Enter restaura y Supr descarta, con confirmación.
- [ ] **6.8** `Dialog::close_window` lista los documentos y ofrece «Guardar
  todo», «Cerrar sin guardar» y «Cancelar»; Esc elige Cancelar. Pruebas en
  `dialog.rs`.

### Fase 7 · Lenguaje, avisos y primer arranque

- [ ] **7.1** Inventario de textos. El resultado va a
  `docs/evaluaciones/2026-11-14/inventario-textos.md`.

  ```bash
  rg -n '"[^"]*[a-záéíóúñ][^"]*"' src/main.rs src/dialog.rs | rg -v 'log.push|\[medicion\]|assert'
  ```

- [ ] **7.2** Aplicar D-25 a todos los textos. Si `ux-copy` está disponible,
  pasarle el inventario y el glosario.
- [ ] **7.3** D-26: avisos.
- [ ] **7.4** Prueba de lenguaje. Recorre `APP_ACTIONS` (`label`) y las
  constantes de texto de los diálogos, y falla si encuentra un término
  prohibido.
- [ ] **7.5** D-27: bienvenida y aviso de recuperación.
- [ ] **7.6** D-03: botón sol/luna en `PRIMARY_TOOLBAR_ACTIONS`. «Exportar a
  PDF» va en un menú del botón Guardar (Guardar, Guardar como, Exportar a PDF).
- [ ] **7.7** Íconos en `draw_toolbar_icon`:
  - marcador (resaltar);
  - `</>` (código en línea);
  - carpeta abierta (Abrir carpeta);
  - carpeta con lupa (Buscar en carpeta);
  - triángulo (plegar);
  - botón «Título» (H1) en la barra de Editar.
- [ ] **7.8** Chevrons de los menús rotulados a 4 px del texto. Corrige el
  chevron encimado a 1100 px.

### Fase 8 · Hover, pulsación y lectura

- [ ] **8.1** Dibujo del hover y la pulsación (D-14) en todos los `UiTarget`.
  Los fondos de barra tienen radio de 6 px.
- [~] **8.2** Copiar (hecho: botón redondeado, hover y destello con
  programación de cuadros solo mientras anima; falta el ícono y el ✓ trazado):
  - botón redondeado con un ícono de dos hojas;
  - al pulsar, fondo de acento con alfa 76 y un ✓ que se traza en 160 ms;
  - a los 1,6 s vuelve en 200 ms;
  - se quita el aviso adicional.
- [ ] **8.3** Chincheta rellena en acento cuando está activa, con tooltip
  «Fijada · clic para soltar»; ícono de restaurar si `is_maximized()`.
- [ ] **8.4** Barra de desplazamiento del documento: 6 px, visible 1 s después
  de desplazar y al pasar el puntero por los 12 px del borde derecho.
  Arrastrable (`UiTarget::ScrollThumb`).
- [ ] **8.5** Recuadro: la etiqueta va dentro del bloque, en la primera línea,
  con un ícono, a 12 px del filete.
- [ ] **8.6** Código: 12 px entre bloques de código contiguos y el lenguaje en
  `dim` junto a Copiar.
- [ ] **8.7** Escala H2 y H3 más separada (H3 = 0,85 × H2, tenue). Columna de
  lectura centrada cuando sobran más de 200 px.
- [ ] **8.8** D-24: contraste de día. Medir con el analizador de Accessibility
  Insights y anotar las cifras en [`design.md`](design.md).
- [ ] **8.9** D-28: capa de texto del PDF.
- [ ] **8.10** Cierre de fase:
  - las referencias de todas las escenas, actualizadas;
  - `design-critique` y `accessibility-review`;
  - ejecutable nuevo.

### Fase 9 · Re-evaluación

- [ ] **9.1** Construir la build de evaluación: aplicar
  `scripts/evaluacion/build-evaluacion.patch` y quitar del parche el cambio
  que ya entró por D-06.
- [ ] **9.2** Repetir la evaluación de QA y la de usuaria con los mismos
  prompts: [`prompt-qa.md`](../scripts/evaluacion/prompt-qa.md) y
  [`prompt-usuario.md`](../scripts/evaluacion/prompt-usuario.md). El
  procedimiento está en [`agente-ejecutor.md`](agente-ejecutor.md#re-evaluación).
- [ ] **9.3** Comparar la ronda 1 con la ronda 2 (`user-research` si está
  disponible) y escribir `docs/evaluacion-usabilidad-ronda-2.md`.
- [ ] **9.4** Criterios: 0 hallazgos S1 o S2, SUS ≥ 68, ≥ 80 % de tareas sin
  dificultad, Nielsen ≥ 3,5 de promedio. Si no se cumplen, se agregan tareas a
  una fase 10 con los hallazgos nuevos y se vuelve a la 9.
- [ ] **9.5** Actualizar [`design.md`](design.md), [`features.md`](features.md)
  y [`manual-qa-v2.md`](manual-qa-v2.md); entregar el ejecutable al
  propietario.

### Fase 10 · Barra, menús y tema

Va después de la 3 y antes de la 5: los menús nuevos reemplazan lo que la 3.13
iba a ampliar, y el texto de la fase 7 se escribe ya sobre ellos.

- [x] **10.1** D-35 (parte hecha): el texto de las listas se recorta por ancho
  medido, no por caracteres (`fit_menu_layout`, prueba
  `los_textos_largos_se_recortan_por_ancho_y_nunca_se_salen_de_la_caja`).
- [x] **10.2** D-30 botón día/noche y Ctrl+Shift+D (hecho; el botón se quita por debajo de 668 px hasta la 10.5, que reubica el selector de modo):
  - `WindowControl::ToggleTheme` primero en `window_control_at`; contador y
    ancho a 5 × 36; ícono luna/sol con trazos; tooltip con atajo;
  - quitar el manejo de la tecla T sola (dos sitios: `window_event` y
    `handle_key`) y resolver Ctrl+Shift+D junto a Ctrl+Shift+T;
  - actualizar `AppAction::ToggleTheme::label`, `ui_target.rs` y las pruebas
    de controles (`window_control_at(...)`);
  - prueba: con el Índice abierto, Ctrl+Shift+D cambia el tema; la T sola no.
- [x] **10.3** D-35 tooltip de texto completo en filas recortadas de las listas
  abiertas (`UiTarget::ListRow`, escena `indice-recortado`).
- [x] **10.4** D-36 `MenuEntry` y su navegación (`menu.rs`): encabezados no
  seleccionables y saltados por flechas, Re Pág, Inicio y Fin; ya lo usan los
  menús Bloques y Estudio. Pendiente: «✓» a la izquierda de las alternancias
  (con la 10.6).
- [x] **10.5** D-37 barra adaptable: cuatro botones directos, selector de modo
  en x = 180 y, a su derecha, `top_menu_actions` / `top_menu_geometry` (Archivo,
  Vista, Carpeta y el buscador de acciones; en ventanas angostas un solo «≡»).
  Foco F6 en orden visual, prueba de solapes de 640 a 2000 px y referencias
  regeneradas. El botón de tema ya cabe en el mínimo de 640 px.
- [x] **10.6** Menús Archivo, Vista y Carpeta (fila 1) con las entradas de la
  sección 3.4 del diseño: contenido con encabezados, apertura bajo su botón,
  menú «≡», cambio de menú al pasar el puntero con uno abierto, F10 y flechas
  ← →. Las alternancias conservan el rótulo «…: activado ✓» (más claro para un
  lector de pantalla que un ✓ suelto); «Tema de noche» también lo tiene.
- [x] **10.7** Fila 2 y menú Editar. «Bloques» y «Listas» pasan a un solo menú
  **Párrafo**; se eliminan «Más formato Markdown» (`WRITING_ACTIONS`,
  `AppAction::WritingTools`) y `AppAction::ListTools`, también del menú
  contextual. **Editar** vive en la fila 1, entre Archivo y Vista (Deshacer,
  Rehacer, Cortar, Copiar, Pegar, Copiar para otros programas y Buscar), porque
  Deshacer, Copiar y Buscar sirven también en lectura. La fila de lectura
  conserva sus seis botones. El menú contextual completo (3.13) sigue aparte.
- [x] **10.8** Acciones nuevas: `Undo`, `Redo`, `Copy`, `Paste`, `MenuEdit`.
  (`MoveToNewWindow` llega con la 12.3.)
- [x] **10.9** Buscador de acciones: ícono `>_`, `WorkspaceHub` fuera de la
  paleta, prueba de inventario de D-29
  (`toda_accion_de_la_paleta_esta_en_un_menu_o_se_declara_solo_paleta`, con
  `PALETTE_ONLY` vacío) y categoría (Archivo, Editar, Vista, Carpeta, Párrafo,
  Insertar, Estudio, Formato) como texto atenuado a la derecha de cada fila sin
  atajo, solo al buscar (`action_category`; escena `paleta-filtrada`).
- [x] **10.11** Pulido que salió de la `design-critique` del 15 de noviembre:
  - los menús (no las búsquedas) no muestran la fila de título repetida ni el
    pie «1–13 de 13 · Esc para cerrar»; el pie solo aparece si hay desplazamiento;
  - el panel de un menú mide lo que mide su fila más larga (mínimo 220 px,
    máximo 420 px) y se ancla al botón;
  - los textos de las filas de Párrafo pasan de «Convertir línea en encabezado
    H1» a «Título 1» (con la 7.1 y la `ux-copy`);
  - la flecha ▾ de los menús sube a 6 px y se separa 6 px del texto, tanto en
    la fila 1 como en la 2;
  - el buscador de acciones usa el ícono `>_` en lugar de «···»;
  - «Comparar» no queda pegado al borde del selector (padding de 6 px).
- [x] **10.10** Cierre de fase. Hechas `design-critique` y `/code-review`
  (corregidos: avisos al abrir menús y encabezados como objetivo; aparcados con
  motivo: caché del ancho de menú y del recorte, F10 con un panel abierto y
  menú abierto al achicar la ventana). Prueba en vivo del 15 de noviembre con
  la build de evaluación (`scripts/evaluacion/`, sin mover el mouse ni usar el
  portapapeles; el parche tuvo que ajustarse a mano en «Guardar como» y debe
  regenerarse): Archivo, Editar, Vista y Carpeta se abren bajo su botón y
  cambian con el puntero; el buscador filtra y muestra la categoría; el botón
  sol/luna cambia el tema; a 640 px quedan «≡», el tema y los controles; un
  documento nuevo toma el nombre de su primera línea y muestra «•» y «sin
  guardar». Defecto hallado y corregido: los menús se cortaban en 14 filas y
  «Vista» (15) se desplazaba con lugar de sobra (`MAX_MENU_ROWS = 20`). Límites
  de la herramienta, no de la app: `type` no escribe «#» y Ctrl+N no llega.
  Capturas en `C:\Dev\archivo\eval-10-10\qa\capturas\`.

### Pedidos sueltos del propietario

- [x] **P-01** (15 de noviembre) Un documento nuevo se llama como su primer
  título (`src/titulo.rs`: H1, si no el título de menor nivel, si no la primera
  línea con texto) y lleva la marca «•» mientras no esté guardado, en la
  pestaña y en la cabecera de panel. Al guardar manda el nombre del archivo;
  «Guardar como» y «Exportar a PDF» proponen el título como nombre. La marca es
  «•» y no «●» porque la fuente de la interfaz no dibuja «●». Pendiente de la
  fase 7: que el aviso diga «sin guardar» con las palabras del glosario.

- [~] **P-02** (15 de noviembre) Que lo sin guardar no se pierda si la
  aplicación se fuerza a cerrar o la computadora se apaga, sin archivos basura
  ni agujeros de seguridad. Diseñado en
  `docs/arquitectura-recuperacion-2026-11-15.md` (D-38 a D-43); se ejecuta en la
  fase 13.

### Fase 11 · Mosaico

Absorbe las tareas 5.5, 5.6 y 5.7. Va después de la 5.

- [ ] **11.1** D-31: la cabecera de panel es la pestaña del archivo (nombre,
  cambios, selector de modo a escala 0,85 y «Quitar del mosaico» con ícono
  propio); la tira filtra los documentos que están en un panel. Con un solo
  panel, todo como hoy. Absorbe la 5.7.
- [ ] **11.2** D-32 divisor: `normalized_pane_fraction` 0,15–0,85,
  `set_fraction_at`, objetivo `UiTarget::PaneDivider`, cursor de redimensionar,
  arrastre, doble clic a 50 % y mínimo de 240 px. Pruebas en `panes.rs`.
- [ ] **11.3** Teclado Alt+Shift+flechas (pasos de 5 %).
- [ ] **11.4** `swap_panes` y arrastre de cabecera sobre cabecera, con fantasma y
  resalte del destino (sin fantasma con movimiento reducido).
- [ ] **11.5** Soltar una pestaña de la tira sobre un panel (centro reemplaza,
  borde divide) y soltar una cabecera sobre la tira (quitar). Zonas con
  acento al 18 %; Esc cancela.
- [ ] **11.6** Foco visible: filete de acento de 2 px y cabecera con `palette.tab`
  en el panel enfocado.
- [ ] **11.7** Cerrar una pestaña conserva la división (era la 5.5) y un panel
  nuevo abre en Leer (era la 5.6).
- [ ] **11.8** Cierre de fase: referencias `mosaico-2`, `mosaico-4`,
  `mosaico-soltar`; `design-critique`; ejecutable nuevo.

### Fase 12 · Ventanas y sistema

Va después de la 8 y antes de la 9.

- [ ] **12.1** D-33 `src/ventanas.rs`: registro con latido, buzón, lectura al
  recibir el foco y cada 500 ms. Pruebas con una raíz temporal.
- [ ] **12.2** Abrir desde fuera en la ventana existente: el proceso nuevo deja el
  mensaje, activa la ventana y sale. Varias rutas por línea de comandos.
- [ ] **12.3** Sacar una pestaña a una ventana nueva: arrastrar fuera, menú
  «Mover a ventana nueva» y doble clic en una cabecera de panel. Lo sin
  guardar viaja como copia de recuperación y el origen cierra tras la
  confirmación.
- [ ] **12.4** Traer una pestaña a otra ventana: soltar sobre la tira o un panel de
  otra ventana y «Mover a otra ventana ▸». La ventana que queda vacía se
  cierra sola.
- [ ] **12.5** D-34 `src/asociacion.rs`: registrar, leer de vuelta, quitar, abrir
  `ms-settings:defaultapps`; diálogo con extensiones y el ajuste de apertura
  externa; entrada en Archivo. Prueba con la raíz `VisorMDPrueba`.
- [ ] **12.6** Línea de la pantalla de bienvenida (D-27) que ofrece configurar la
  asociación, una sola vez.
- [ ] **12.7** Documentar el paquete: `.desktop` con tipos MIME y `xdg-mime default`
  para Linux; el instalador de Windows reutiliza 12.5 (`docs/roadmap.md`).
- [ ] **12.8** Cierre de fase: prueba manual con dos ventanas, arrastre entre ellas,
  apertura desde el Explorador y desde Claude; ejecutable nuevo.

### Fase 13 · Servicio de recuperación (P-02)

Diseño: `docs/arquitectura-recuperacion-2026-11-15.md`. Las tareas 13.1 y 13.2
corrigen huecos de hoy y se hacen **antes** de la fase 5; la 13.3 a la 13.6 se
hacen con la fase 6 (comparten la 6.7); la 13.7 y la 13.8 con la fase 12.

- [x] **13.1** Arreglos inmediatos sobre `src/recovery.rs`:
  - tope de 10 s en una ráfaga (H1) y escritura inmediata al cambiar de pestaña,
    perder el foco o abrir un diálogo (H2);
  - quitar el recurso a `temp_dir()` (H8) y desactivar la recuperación con un
    aviso si no hay perfil;
  - barrer los temporales huérfanos (H9);
  - mostrar en la interfaz que la copia falló (H10).

  Pruebas en `recovery.rs` y `main.rs`. Hecho con un aviso («No se pudo guardar
  la copia de seguridad: guardá el documento»), no con el tono del punto «•»,
  que queda para la 13.5. Plazos: pausa de 2 s y tope de 10 s. Sin perfil, el
  cierre ya no queda bloqueado.
- [x] **13.2** Un solo hilo escritor con buzón «el último gana» (en lugar de un
  hilo por escritura) y omitir la escritura si el contenido no cambió (hash de
  la biblioteca estándar, sin dependencia nueva).
- [ ] **13.3** `src/recuperacion/formato.rs` y `politica.rs` (puros), con todas
  las pruebas de la sección 7 del diseño.
- [ ] **13.4** `src/recuperacion/almacen.rs`: sesión con cerrojo, carpeta por
  sesión, una copia por `doc_key`, listado de huérfanas por cabecera, adopción
  por renombrado, barrido y cuota. Prueba de muerte real del proceso con
  `Child::kill` y prueba de adopción con dos hilos.
- [ ] **13.5** Migrar `main.rs` a la interfaz nueva (`iniciar`, `instantanea`,
  `descartar`, `forzar_todo`, `huerfanas`, `adoptar`, `estado`); quitar
  `RecoverySession` por pestaña; `doc_key` viaja con la pestaña.
- [ ] **13.6** Aviso al arrancar y lista de recuperaciones (con la 6.7) con las
  cuatro situaciones de D-41; «Restaurar todo» y «Descartar todo».
- [ ] **13.7** Gancho de pánico con el registro compartido (D-42) y traspaso de
  una pestaña entre ventanas por adopción (con la 12.3 y la 12.4).
- [ ] **13.8** Documentación: `docs/security.md`, `docs/threat-model.md`,
  `docs/architecture.md`, `docs/test-matrix.md` y el QA manual (matar con el
  Administrador de tareas y reiniciar Windows con `shutdown /r /t 0`).
- [ ] **13.9** *Opcional.* Módulo aislado de integración con el sistema operativo
  para `WM_ENDSESSION` y evaluar DPAPI; solo si 13.1 a 13.8 no bastan. Exige
  levantar `forbid(unsafe_code)` en ese módulo y revisión manual.

## Secuencia

```text
Fase 1 ─┬─ Fase 2 ── Fase 3 ── Fase 5 ── Fase 8 ─┬─ Fase 9
Fase 4 ─┘            Fase 6 ── Fase 7 ───────────┘
```

Orden por defecto para un solo ejecutor:
1 → 4 → 2 → 3 → 10 → **13.1–13.2** → 5 → 11 → 6 (+ 13.3–13.6) → 7 → 8 →
12 (+ 13.7–13.8) → 9.

La fase 10 va antes de la 5 y la 7 porque los menús nuevos cambian los textos y
las barras sobre las que ellas trabajan; la 11 absorbe tres tareas de la 5; la 12
va antes de la re-evaluación para que esta pruebe también ventanas y
asociación.

## Estado

| Fase | Estado | Nota |
| --- | --- | --- |
| Preparación | Hecho | Herramientas instaladas; decisiones D-01 a D-28 |
| 1 | Hecho | a76ddca, 5f9c2ae, 1a74b6f; ejecutable entregado |
| 4 | Hecho | c441983, c3b3a76, 1c6f87a, 98f6793 y el ajuste de bloques |
| 2 | Hecho | 01bacf8, 898665a, e127b66, 9d1a167, 5888201 y la capa de tooltip |
| 3 | Casi hecha | Hechas 3.1 a 3.8, 3.9b, 3.10 a 3.12. Pendientes: 3.9 (caja de búsqueda con borde y barra anclada); la 3.13 pasa a la 10.7 |
| 10 | Hecho | 10.1 a 10.11; prueba en vivo del 15 de noviembre |
| 5 | En curso | Hecha 5.1 (verde propio de la pestaña). Pendientes 5.2 a 5.4 y 5.8; la 5.5 a 5.7 pasan a la fase 11 |
| 11 | Pendiente | Mosaico: cabecera como pestaña, divisor, intercambio, soltado |
| 6 | Pendiente | — |
| 7 | Pendiente | — |
| 8 | En curso | Hecho el hover y destello de Copiar (8.2 parcial). Pendientes el resto de 8 |
| 12 | Pendiente | Ventanas (un proceso por ventana) y asociación de archivos |
| 9 | Pendiente | — |
| 13 | Diseñada | Servicio de recuperación (P-02): D-38 a D-43; 13.1 y 13.2 van antes de la fase 5 |
