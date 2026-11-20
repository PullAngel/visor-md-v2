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
- **D-44. Modo limpio (extra posterior a la versión final).** Las barras
  (ventana, pestañas, contextual y de estado) se pueden ocultar y queda solo el
  marco del texto. No es el «modo foco que oculta todo» que se descartó: las
  barras siguen a un gesto de distancia. Detalle en el extra X-01.

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
- [x] **5.2** Hover en las pestañas y ✕ al pasar el puntero (ya estaban);
  tooltip de D-15 con nombre completo, estado y ruta, y «Cerrar pestaña · Ctrl+W»
  sobre la ✕ (escena `pestana-tooltip`).
- [x] **5.3** La pestaña fijada va al inicio de `tab_order`, compacta (144 px,
  nombre y chincheta dibujada en acento) y sin ✕. `TabStrip` reparte el ancho;
  arrastrar no cruza el borde del grupo fijado; fijar o soltar mueve la pestaña
  al borde del grupo. La chincheta de texto «• » se retiró porque chocaba con
  la marca de sin guardar.
- [x] **5.4** Abrir un archivo reemplaza la pestaña activa si no tiene
  identidad, está vacía, no está sucia, no está fijada ni ocupada y el mosaico
  tiene un solo panel; la pestaña nueva ocupa su lugar.
- [~] **5.5** Pasa a la 11.7 (mosaico).
- [~] **5.6** Pasa a la 11.7 (mosaico).
- [~] **5.7** Pasa a la 11.1 (mosaico).
- [x] **5.8** Cierre de fase: referencias `dos-pestanas`, `pestana-fijada`,
  `pestana-tooltip` y `division`; `design-critique` del 15 de noviembre (la
  fijada se ensanchó de 128 a 144 px; el resto, a las fases 7 y 11). Las
  cabeceras de panel faltaban en las escenas sin ventana: ahora
  `visible_document_pane_layouts` usa el tamaño del último cuadro.

### Fase 6 · Enlaces, carpeta de apuntes y recuperación

- [x] **6.1** D-02: aviso «Abriendo en el navegador: dominio» (`web_domain`) e ícono ↗ en voladizo tras el enlace, para no tapar la coma que sigue.
- [x] **6.2** Enlace roto (la marca se calcula con una caché por documento, revisión y carpeta):
  - un `[[nota]]` que no resuelve en el índice se dibuja con subrayado punteado;
  - al pasar el puntero, el destino dice «Esta nota no existe»;
  - al hacer clic, se ofrece un diálogo «Crear nota» que crea un `.md` vacío
    dentro de la carpeta autorizada, con la validación de Guardar como.
- [x] **6.3** `[[nota]]` sin carpeta (si el archivo no está guardado, el aviso pide guardarlo; tras indexar, el enlace se sigue solo): diálogo «Usar la carpeta de este archivo
  como carpeta de apuntes» (ejecuta `choose_workspace` con la carpeta del
  documento) o «Cancelar».
- [x] **6.4** Una nota abierta desde un enlace abre en Leer (`open_mode_override`; una nota recién creada abre en Editar).
- [x] **6.5** Al elegir la carpeta se muestra el árbol (los chevrones se dibujan, porque la fuente de la interfaz no trae «▸» ni «▾»; reindexado silencioso tras guardar). Además:
  - se reindexa después de cada Guardar o Guardar como dentro de la carpeta;
  - la nota abierta se resalta;
  - las carpetas llevan chevron;
  - las rutas se muestran con « › ».
- [x] **6.6** Notas que enlazan aquí, con un extracto de la línea (hasta 80
  caracteres), sacado del fragmento que el índice ya guarda (`backlinks_with_context`).
- [x] **6.7** Lista de recuperaciones con título, origen y antigüedad; Enter
  restaura, Ctrl+Enter restaura todo, Supr descarta (Mayús+Supr, todo), con
  confirmación. Aviso al arrancar si quedaron documentos (con 13.6).
- [x] **6.8** `Dialog::close_window` lista los documentos (hasta 6 y «… y N
  más») y ofrece «Guardar todo», «Cerrar sin guardar» y «Cancelar»; Esc elige
  Cancelar. «Guardar todo» guarda cada documento (los nuevos piden destino) y
  cierra cuando termina el último; si uno falla, el cierre se detiene. Pruebas
  en `dialog.rs` y `main.rs`.

**Cierre de la fase 6 (16 de noviembre).** Prueba en vivo con la herramienta de
evaluación: escribir, matar el proceso a la fuerza, reabrir, abrir la lista de
recuperaciones por teclado (F10) y restaurar funciona; la copia pasa a la sesión
nueva. La prueba halló un defecto anterior, no de esta fase: pasado el último
plazo (un aviso, una animación) el bucle de eventos quedaba en una espera ya
vencida y giraba al 100 % de CPU. Se corrigió con `control_flow_for` y una
prueba. Con la herramienta, los clics solo llegan a la build de evaluación
(lleva el parche del puntero); con la build normal sirven las teclas.

### Fase 7 · Lenguaje, avisos y primer arranque

- [x] **7.1** Inventario de textos. El resultado está en
  `docs/evaluaciones/2026-11-14/inventario-textos.md` (etiquetas, diálogos y
  los 200 y pico avisos, con lo que cambió).

  ```bash
  rg -n '"[^"]*[a-záéíóúñ][^"]*"' src/main.rs src/dialog.rs | rg -v 'log.push|\[medicion\]|assert'
  ```

- [x] **7.2** D-25 aplicado a etiquetas, diálogos, avisos, títulos de panel y
  textos de los diálogos de archivo. No se pasó por `ux-copy`: las frases se
  corrigieron aplicando el glosario y el voseo sin reescribirlas.
- [x] **7.3** D-26: los avisos de abrir y cerrar paneles se quitaron (el pie del
  panel ya dice «Esc para cerrar»); el aviso va centrado en el área del documento
  a 24 px de su borde inferior. **Desviación:** dura 2,4 s solo si es corto; los
  largos suman 35 ms por letra pasadas las 40, hasta 6 s, porque 2,4 s no alcanza
  para leer una frase larga.
- [x] **7.4** Prueba de lenguaje en `src/lenguaje.rs`: recorre `APP_ACTIONS`, las
  etiquetas del menú contextual, los diálogos y todos los literales de la
  interfaz (no los registros), y falla con un término retirado o un imperativo de
  tuteo al empezar una frase.
- [x] **7.5** D-27: pestaña inicial «Bienvenida» con tres botones y dos líneas
  (`src/bienvenida.rs`); se cierra al escribir o al abrir un documento. El aviso
  de privacidad de la recuperación es un aviso de 6 s, «Más detalles» abre el
  diálogo. **Desviación:** la bienvenida queda en modo Editar (no en Leer) para
  que escribir funcione al instante.
- [x] **7.6** Ya estaba hecha por la 10.2 (botón día/noche como control de
  ventana) y por el menú Archivo, que agrupa Guardar, Guardar como y Exportar a
  PDF.
- [x] **7.7** Íconos: carpeta abierta (Abrir carpeta de apuntes), carpeta con
  lupa (Buscar en carpeta) y triángulo (plegar). El marcador, `</>` y «Título»
  ya existían.
- [x] **7.8** Chevron de los menús rotulados a 4 px del texto de cada etiqueta.

**Cierre de la fase 7 (16 de noviembre).** Las pruebas de imagen cubren la
bienvenida; en vivo se vieron la bienvenida y el aviso de privacidad con el
ejecutable real. No se probaron en vivo los clics sobre los botones de
bienvenida (la herramienta no los registra en la versión normal; sí hay pruebas
que fijan su geometría y su efecto).

### Fase 8 · Hover, pulsación y lectura

- [x] **8.1** Hover y pulsación (D-14) dibujados con transición en la barra
  principal, los menús rotulados, la barra contextual, el selector de modo, los
  controles de ventana, las pestañas, el botón «+», las filas de lista y los
  botones de diálogo. El hover entra en 120 ms, sale en 80 y la pulsación deja un
  velo de acento de 160 ms; con movimiento reducido es inmediato. Los fondos de
  barra tienen radio de 6 px. Verificado en vivo con el hover del botón «+».
- [x] **8.2** Copiar: botón redondeado con ícono de dos hojas; al pulsar, fondo
  de acento con alfa 76 y un ✓ que se traza en 160 ms; a los 1,6 s vuelve en
  200 ms; ya no hay aviso adicional. El botón se ensanchó a 86 px para el texto
  «Copiado».
- [x] **8.3** Chincheta rellena y en acento cuando está activa, con ayuda
  «Fijada · clic para soltar»; ícono de restaurar y ayuda «Restaurar» cuando la
  ventana está maximizada. Además, «Quitar del mosaico» tiene ayuda propia
  (`UiTarget::PaneClose`, pendiente de la fase 11).
- [x] **8.4** Barra de desplazamiento del documento (`src/scrollbar.rs`): 6 px,
  se ve 1 s después de desplazar y al pasar el puntero por los 12 px del borde
  derecho, y se arrastra (`UiTarget::ScrollThumb`). Verificada en vivo con la
  versión de evaluación: aparece al pasar el puntero y arrastrarla desplaza el
  documento.
- [x] **8.5** Recuadro: el rótulo va dentro del bloque, en la primera línea, a
  12 px del filete y con un punto de acento; el texto arranca en una sangría fija
  de 108 px (`CALLOUT_INDENT`), igual para todos los recuadros. El rótulo es el
  control de plegado. Decisión de la persona: sangría fija.
- [x] **8.6** Código: 12 px entre cercas contiguas (las líneas de una misma
  cerca siguen pegadas) y el lenguaje, en tono tenue, junto a «Copiar» (lectura
  y vista previa de Comparar).
- [x] **8.7** Escala: H2 sube a 26 px (H1 31, H3 20, H4 17), así H2 y H3 quedan
  más separados. No se hizo H3 tenue (bajaría su contraste) ni se centró la
  columna (chocaba con el índice flotante). Criterio delegado a Claude.
- [x] **8.8** D-24: aplicado (`dim` de día a `#5A5043`, `external_link` a
  `#1A5AAE`, mínimos de día subidos a 5,5 en la prueba de `theme.rs`). Las cifras
  están en [`design.md`](design.md), calculadas con la fórmula de WCAG; la
  medición con Accessibility Insights sobre la ventana real queda para la fase 9.
- [x] **8.9** D-28: capa de texto del PDF. Se encontraron y corrigieron dos
  defectos: cada celda de una tabla escribía el texto de toda la fila y un enlace
  o un cambio de estilo repetía el párrafo entero por cada tramo. Una prueba
  extrae los fragmentos de `export_pdf_bytes` y los compara con el texto de los
  bloques.
- [x] **8.10** Cierre de fase: referencias de todas las escenas actualizadas,
  revisión de accesibilidad y de diseño en
  [`revision-cierre-fase-8.md`](evaluaciones/2026-11-14/revision-cierre-fase-8.md)
  (corrigió el contraste del tirador de la barra; deja abierto el árbol de
  accesibilidad) y ejecutable nuevo.

**Pendiente de la fase 11 que sigue abierto:** el recuadro vacío de la tira de
pestañas cuando todos los documentos están en paneles y el resalte del divisor
junto al borde del panel enfocado.

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

### Fase 14 · Herramientas de estudio (pedido del 19 de noviembre)

Surgió de usar el programa para estudiar. Va después de la fase 9, antes del
instalador. El diseño completo está en
[`diseno-anotaciones-2026-11-19.md`](diseno-anotaciones-2026-11-19.md) y se
probó con un prototipo sobre una guía real de 116 KB (sección 12 del diseño).

**Decisión de almacenamiento (D-45).** El texto del documento no se toca nunca.
Subrayados, marcas de libro y lo que se agregue después viven en **un único
comentario HTML al final del `.md`** (`<!-- visor-md:anotaciones v1 … -->`), con
una frase fija que avisa que no es parte del texto y una línea JSON por
anotación, anclada por cita y contexto. Viaja con el archivo, es invisible en
GitHub, Obsidian y VS Code, y «quitar todas» devuelve el archivo idéntico byte a
byte.

**Decisiones de la persona (19 de noviembre).**

- Se guardan como cualquier cambio: «•» y Ctrl+S; la recuperación las cubre.
- El PDF las incluye, con una casilla para quitarlas al exportar.
- Viajan siempre en el archivo; existe «Guardar copia sin anotaciones».
- Enviar a IA o a Google: sin decidir; se pensará como función propia.
- **El bloque no se ve al leer.** En Leer no aparece nunca. En Editar aparece
  **siempre contraído** en una sola línea al final («▸ Anotaciones de estudio ·
  42»), salvo que la persona lo abra; contraído no alarga el documento, así la
  barra de desplazamiento sigue dando la idea real del largo y de cuánto falta.
- **El color se elige** en cada subrayado y cada marca, y se puede cambiar
  después; la leyenda le da a cada color el significado que la persona quiera.
- **En el índice, las marcas se ven de forma muy sutil** y sin romper el
  formato aunque haya muchas: ver 14.6.

Cada tarea deja `cargo test` y `cargo clippy` en verde.

- [x] **14.0** Diseño aprobado y probado con un prototipo (26 de 26 casos borde).
  ADR-39 registrado en [`decisions.md`](decisions.md).
- [x] **14.1** Formato puro (`src/anotaciones.rs`, sin interfaz ni
  dependencias): modelo, lector y escritor. Ida y vuelta byte a byte, orden
  determinista, límites, contenido desconocido conservado, y los separadores de
  línea de Unicode (U+2028, U+2029, U+0085) siempre escapados, como `<` y `>`.
  Pruebas: `bloque_vacio_no_se_escribe`, `ida_y_vuelta_conserva_bytes`,
  `escritura_ordena_por_posicion_tipo_e_id`, `escapa_menor_y_mayor_en_cadenas`,
  `escapa_los_separadores_de_linea_de_unicode`,
  `ninguna_nota_puede_cerrar_el_comentario`, `version_mayor_se_conserva_inerte`,
  `linea_invalida_se_conserva_al_final`, `claves_desconocidas_se_conservan`,
  `limites_vuelven_inerte_el_bloque`, `crlf_y_lf_producen_mismas_posiciones`.
- [x] **14.2** Anclaje puro (mismo módulo), según la sección 6 del diseño:
  posición guardada, cita exacta, desempate por contexto y por cercanía,
  espacios colapsados; si nada coincide, la anotación queda sin lugar y nunca
  se borra sola. Pruebas: `camino_rapido_ancla_en_su_posicion`,
  `cita_unica_se_ancla_tras_insertar_texto_antes`,
  `cita_repetida_desempata_por_contexto`, `empate_total_elige_la_mas_cercana`,
  `espacios_colapsados_reanclan_tras_reajuste_de_linea`,
  `texto_borrado_deja_huerfana_sin_eliminarla`,
  `reanclar_dos_veces_da_el_mismo_resultado`, `tope_de_apariciones_se_respeta`.
- [x] **14.3** Separar cuerpo y bloque al abrir, guardar, recargar y recuperar.
  - **Leer:** el bloque no se dibuja nunca y no cuenta en el alto del documento
    (el texto del documento no lo incluye: `DocumentState.annotations` lo guarda
    aparte y se une al guardar).
  - **Editar:** el bloque se muestra como **una sola línea contraída**, con un
    chevron dibujado y el total, en la barra de estado («▸ Anotaciones de
    estudio · 42»). Un clic la despliega como un panel de solo lectura sobre la
    barra; Esc o otro clic la contraen, y se contrae sola al salir de Editar.
    **Desviación:** la línea va en la barra de estado y no al final del texto:
    la vista de fuente es una lista de bloques ligada a rangos de la fuente, y
    una línea sintética ahí rompía cursor y selección. Contraída no suma alto,
    así la barra de desplazamiento refleja el largo del cuerpo. Abrirla con
    Enter y copiar su texto quedan para la 14.5 (menú Estudio); mientras tanto
    el archivo se puede abrir en otro editor.
  - **Comparar:** igual que Editar.
  - Buscar, copiar, contar palabras y caracteres solo ven el cuerpo.
  - Si el cuerpo termina dentro de un bloque de código o comentario sin cerrar,
    el bloque no se escribe (no se reconocería al abrir): se guarda el texto y
    un aviso lo explica; las anotaciones siguen en memoria.
  - La recuperación y el traspaso entre ventanas guardan el texto completo
    (cuerpo y bloque), así una copia restaurada conserva las anotaciones.
  Pruebas: `cuerpo_y_bloque_se_separan_al_abrir`,
  `bloque_dentro_de_codigo_no_se_reconoce`, `texto_despues_del_bloque_lo_invalida`,
  `agregar_y_quitar_deja_el_archivo_identico` (con y sin salto final, CRLF y
  BOM), `leer_no_dibuja_el_bloque_ni_lo_suma_al_alto`,
  `editar_muestra_el_bloque_contraido_en_una_linea`,
  `abrir_el_bloque_lo_muestra_de_solo_lectura`,
  `escribir_al_final_no_entra_en_el_bloque`,
  `cuerpo_con_codigo_sin_cerrar_rechaza_guardar_anotaciones`,
  `recuperacion_restaura_cuerpo_y_anotaciones`, `buscar_y_copiar_ignoran_el_bloque`,
  `el_bloque_de_anotaciones_viaja_aparte_y_se_guarda_con_los_mismos_bytes`.
- [x] **14.4** Paleta de estudio en `src/theme.rs`: cinco colores con nombre
  (amarillo, verde, azul, rosa, violeta), con su tono de día y de noche. Cada
  color tiene un fondo para el subrayado (el texto conserva 7:1 sobre él) y un
  trazo para marcas y puntos del índice (3:1 sobre el documento).
  Pruebas: `paleta_de_estudio_conserva_contraste_en_ambos_temas`,
  `colores_de_estudio_se_distinguen_entre_si_y_del_resaltado`.
- [x] **14.5** Subrayar desde la selección (Leer y también Editar): menú
  Estudio (grupo «Subrayar»), «Herramientas de estudio» en el menú contextual de
  Leer y Ctrl+Shift+H. **Color:** «Subrayar en amarillo, verde, azul, rosa o
  violeta»; «Subrayar» usa el último elegido. Aplicar un color sobre un
  subrayado que ya existe le cambia el color sin mover el ancla; «Quitar el
  subrayado» quita los que toca la selección o el cursor. **Leyenda:**
  «Leyenda: qué significa cada color» abre un campo sobre el documento (↑↓
  cambia de color, Enter guarda, Esc cancela) y el aviso al subrayar la nombra.
  «Mostrar u ocultar mis anotaciones» muestra el original. «Ver el bloque de
  anotaciones» es la vía de teclado de la línea contraída de Editar (14.3).
  Subrayar marca el documento sin guardar («•», Ctrl+S, recuperación) sin tocar
  el texto. Lo dibujado se traduce a la fuente carácter a carácter (el texto de
  Leer es una subsecuencia de su Markdown), así la cita guarda la fuente exacta,
  con su sintaxis. Solo dos acciones nuevas en la paleta general (subrayar y
  ocultar); las demás viven en el menú Estudio. Pruebas:
  `subrayado_cruza_bloques_y_se_dibuja_por_tramos`,
  `subrayar_usa_el_ultimo_color_elegido`, `cambiar_el_color_no_mueve_el_ancla`,
  `subrayar_marca_el_documento_sin_guardar`,
  `ocultar_mis_anotaciones_muestra_el_original`, más las de `estudio::pruebas`.
  **Falta:** QA manual con capturas en la ventana real.
- [x] **14.6** Marcas de libro: una por línea, con forma (barra, cinta, punto,
  bandera), **color elegido** (el último usado) y nota opcional, en el margen
  izquierdo de Leer. Ctrl+Shift+M (o menú Estudio > «Marcas de libro») la pone o,
  si la línea ya tiene una, la quita; hay acciones para cambiar su forma, su
  color y su nota. **En el índice, sutiles y sin romper el formato:**
  - no se agregan filas: cada título puede llevar, a la derecha, **puntos de
    color de 6 px** de las marcas de su sección (las que caen entre ese título
    y el siguiente);
  - un punto por color distinto, en el orden de la paleta, **como máximo tres**;
    si hay más colores, un «+N» tenue;
  - el lugar de los puntos está **reservado y es fijo** (40 px) en todas las
    filas del índice de un documento con anotaciones: el título no se mueve ni
    se recorta distinto por tener o no marcas, aunque una sección tenga decenas;
    sin anotaciones el índice queda como siempre;
  - al pasar el puntero, la ayuda lista las marcas de la sección (las primeras
    cinco con su nota o su cita, y «y N más»); clic en un punto lleva a la
    primera marca de ese color en la sección;
  - lo mismo en el índice flotante (con 30 caracteres de título); los
    subrayados no aparecen en el índice.
  Pruebas: `marca_suma_un_punto_al_titulo_de_su_seccion`,
  `muchas_marcas_no_cambian_la_fila_del_indice`,
  `mas_de_tres_colores_muestran_mas_n`,
  `la_ayuda_lista_las_marcas_de_la_seccion`,
  `marca_antes_del_primer_titulo_va_al_comienzo_del_indice`,
  `la_marca_de_libro_se_pone_se_dibuja_y_aparece_en_el_indice`.
  **Falta:** las marcas en Editar (ahí solo viajan en el archivo) y QA manual.
- [x] **14.7** Editar con anotaciones. Como las anotaciones se anclan por cita
  y contexto contra el texto de ahora, cada edición (y cada deshacer) las
  desplaza sola: borrar el texto anclado la deja **sin lugar** sin borrarla, y
  deshacer la recupera. Al guardar, cada una anclada recalcula su posición y su
  contexto (la cita no cambia). La línea contraída de Editar dice «· N sin lugar»
  cuando las hay, y el menú Estudio ofrece «Copiar las citas de anotaciones sin
  lugar» y «Quitar las anotaciones sin lugar».
  **Desviación:** no hay panel con «Reubicar» por anotación; se difiere a un
  uso real (copiar la cita y volver a subrayar cubre el caso). Pruebas:
  `editar_antes_del_subrayado_lo_desplaza`,
  `borrar_el_texto_anclado_lo_deja_huerfano`,
  `deshacer_el_borrado_recupera_el_subrayado`,
  `editar_con_anotaciones_las_desplaza_y_las_deja_sin_lugar_sin_borrarlas`.
- [x] **14.8** Integraciones: los fragmentos para IA, el PDF, la búsqueda y las
  métricas parten del cuerpo, así que nunca incluyen el bloque. El PDF pinta los
  subrayados por línea (fondo de día) y existe «Exportar a PDF sin
  anotaciones» en el menú Estudio (en lugar de una casilla en un diálogo).
  «Quitar todas las anotaciones» pide confirmación y se restaura con
  «Restaurar las anotaciones quitadas» mientras la pestaña siga abierta;
  «Guardar copia sin anotaciones» escribe el cuerpo. **Falta:** resumen de
  anotaciones en Comparar (diferido). Pruebas:
  `fragmentos_para_ia_excluyen_anotaciones_por_defecto`,
  `pdf_no_incluye_el_bloque_en_la_capa_de_texto`,
  `pdf_sin_anotaciones_si_se_desmarca_la_casilla`,
  `quitar_todas_se_puede_deshacer`, `copia_sin_anotaciones_es_el_cuerpo`.
- [x] **14.9** Documentado en `docs/features.md`, `docs/security.md` (el bloque
  como dato inerte), `docs/test-matrix.md` y el QA manual de `docs/manual-qa-v2.md`.
  **Pendiente para la persona:** el QA manual en la ventana real (Leer, Editar,
  otro editor y GitHub).
- [ ] **14.10** (futuro) Imágenes ancladas.
- [ ] **14.11** (futuro, sin decidir) Enviar la selección a una IA o a Google.

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

- [x] **11.1** D-31 (la tira filtra con `visible_tab_ids`; selector de modo a 10,5 px y «Quitar del mosaico» con un panel y un signo menos; la ✕ de verdad ya no aparece en las cabeceras): la cabecera de panel es la pestaña del archivo (nombre,
  cambios, selector de modo a escala 0,85 y «Quitar del mosaico» con ícono
  propio); la tira filtra los documentos que están en un panel. Con un solo
  panel, todo como hoy. Absorbe la 5.7.
- [x] **11.2** D-32 divisor (`PaneDivider`, `divider_at`, `divider_fraction_for_pointer`; resaltado en acento y cursor de redimensionar): `normalized_pane_fraction` 0,15–0,85,
  `set_fraction_at`, objetivo `UiTarget::PaneDivider`, cursor de redimensionar,
  arrastre, doble clic a 50 % y mínimo de 240 px. Pruebas en `panes.rs`.
- [x] **11.3** Teclado Alt+Shift+flechas (pasos de 5 %, la división más cercana en ese sentido; respeta el mínimo útil).
- [x] **11.4** `swap_panes` (intercambia hoja, documento, posición y caché) y
  arrastre de cabecera sobre otro panel, con fantasma y resalte del destino
  (sin fantasma con movimiento reducido). Umbral de 6 px: antes es un clic.
- [x] **11.5** Soltar una pestaña de la tira sobre un panel (centro reemplaza,
  borde divide, `split_pane_side`) y soltar una cabecera sobre la tira (quitar).
  Zonas con acento al 18 %; Esc cancela. Una pestaña que sale de la tira hacia
  el documento pasa de reordenar a soltar.
- [x] **11.6** Foco visible: filete de acento de 2 px y cabecera con `palette.tab`
  en el panel enfocado.
- [x] **11.7** Cerrar una pestaña conserva la división (era la 5.5): otra
  pestaña de la tira ocupa el panel y solo si no queda ninguna se cierra el
  panel. Un panel creado con una pestaña que estaba en Comparar abre en Leer
  (era la 5.6; los documentos nuevos conservan su modo).
- [x] **11.8** Cierre de fase: referencias `division`, `division-arrastrando`,
  `mosaico-4` y `mosaico-soltar`; `design-critique`; ejecutable nuevo. Prueba
  en vivo del 15 de noviembre con la build de evaluación (el parche
  `scripts/evaluacion/build-evaluacion.patch` se regeneró y aplica limpio):
  dividir, arrastrar el divisor a 700 px e intercambiar paneles arrastrando la
  cabecera funcionan. No verificado en vivo: el doble clic sobre el divisor
  (la herramienta no lo reproduce; la lógica está en el código) y Alt+Shift+
  flechas (las combinaciones no llegan por la herramienta; hay prueba
  unitaria). Crítica: el recuadro vacío de la tira cuando todos los documentos
  están en paneles, el tooltip de «Quitar del mosaico» (no hay objetivo de
  cabecera) y el resalte del divisor junto al borde del panel enfocado pasan
  a la fase 8.

### Fase 12 · Ventanas y sistema

Va después de la 8 y antes de la 9.

- [x] **12.1** D-33 `src/ventanas.rs`: registro con latido (2 s; se ignora y se
  borra a los 6 s), buzón por proceso, lectura al recibir el foco y cada 500 ms.
  Pruebas con una raíz temporal. **Desviación:** los archivos son `.txt` y
  `.msg` con líneas `clave=valor` (no JSON), para no escapar rutas ni sumar una
  dependencia.
- [x] **12.2** Abrir desde fuera en la ventana existente: el proceso nuevo deja el
  mensaje y sale; la ventana abierta abre cada ruta en su pestaña y pasa al
  frente. Acepta varias rutas por línea de comandos y la bandera
  `--ventana-nueva`. Ajuste «Abrir archivos externos en la ventana abierta»
  (Archivo > Sistema). Verificado en vivo con dos procesos.
- [x] **12.3** Sacar una pestaña a una ventana nueva: «Mover pestaña a una
  ventana nueva» (Archivo > Pestaña), arrastrar la pestaña (o la cabecera de un
  panel) fuera de la ventana y doble clic en la cabecera de un panel. Lo sin
  guardar viaja como traspaso (13.7). **No verificado en vivo:** el arrastre y el
  doble clic (la herramienta no los reproduce bien; hay prueba unitaria).
- [x] **12.4** Traer una pestaña a otra ventana: «Mover pestaña a otra ventana»
  y soltar una pestaña fuera de esta ventana sobre otra de Visor MD (el latido
  anota dónde está cada ventana). La ventana que se queda sin pestañas se
  cierra sola.
- [x] **12.5** D-34 `src/asociacion.rs`: registrar en «Abrir con» (solo `HKCU`,
  con `reg.exe`), leer de vuelta, quitar y abrir `ms-settings:defaultapps`;
  diálogo «¿Abrir tus archivos con Visor MD?» (Archivo > Sistema). La prueba usa
  `HKCU\Software\VisorMDPrueba`. **Desviación:** el diálogo registra las tres
  familias de extensiones juntas, sin elegirlas una por una.
- [x] **12.6** Línea de la bienvenida que ofrece la asociación, una sola vez
  (se apaga al contestar el diálogo).
- [x] **12.7** Paquetes: ver [`roadmap.md`](roadmap.md#asociación-de-archivos-en-los-paquetes).
- [x] **12.8** Cierre de fase: verificado en vivo con dos procesos (abrir desde
  fuera y mover a una ventana nueva). Quedan para la persona: el arrastre real
  entre dos ventanas, abrir desde el Explorador con la asociación hecha y desde
  Claude.

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
- [x] **13.3** `src/recovery/formato.rs` (puro: cabecera, `body_len`, CRC-32,
  límites) con las pruebas de la sección 7 del diseño. La política de disparo
  quedó en `main.rs` (`queue_recovery`, `RECOVERY_MAX_WAIT`) porque ya estaba
  probada ahí.
- [x] **13.4** `src/recovery/almacen.rs`: sesión con cerrojo (`share_mode(0)`),
  carpeta por sesión, una copia por documento, listado de huérfanas por
  cabecera, adopción por renombrado con archivo de reclamo (en Windows el
  renombrado por manija dejaba ganar a dos hilos), barrido y cuota. Prueba de
  muerte real del proceso con `Child::kill` y de adopción con dos hilos.
- [x] **13.5** `main.rs` usa el almacén nuevo: `RecoverySession` conserva su
  interfaz (`write`, `submit`, `clear`) y suma `set_meta` (título y origen
  viajan en la cabecera), `adopt` y `recovery::orphans()`. Las copias de
  formato anterior (`session-*.md`) se listan y se descartan tras abrirlas.
- [x] **13.6** Aviso al arrancar y lista de recuperaciones, con «restaurar todo»
  y «descartar todo». Las cuatro situaciones de D-41 están: sin origen abre
  pestaña nueva; origen sin cambios (misma huella que la base) queda **ligada al
  archivo original**, modificada, y Guardar usa la comprobación de conflicto de
  siempre; origen cambiado, ausente o ya abierto abre la copia aparte, sin
  guardar, y el aviso lo dice. Restaurar nunca escribe en el original.
  Desviación: la oferta de Comparar quedó como texto del aviso («abrí el
  archivo para comparar»), no como botón; un botón que abra el original en un
  panel junto a la copia queda para la fase 8.
- [x] **13.7** Gancho de pánico con el registro compartido (D-42; el registro
  guarda una referencia barata al texto y el gancho escribe sin esperar
  cerrojos) y traspaso de una pestaña entre ventanas. **Desviación:** el
  traspaso no mueve el archivo de la sesión por renombrado (D-40); el origen
  escribe un archivo con el mismo formato en `recovery\traspaso\`, la otra
  ventana lo lee y avisa, y el origen cierra solo con esa confirmación.
- [x] **13.8** Documentación: `docs/security.md`, `docs/threat-model.md`,
  `docs/architecture.md`, `docs/test-matrix.md` y el QA manual (matar con el
  Administrador de tareas y reiniciar Windows con `shutdown /r /t 0`).
- [ ] **13.9** *Opcional.* Módulo aislado de integración con el sistema operativo
  para `WM_ENDSESSION` y evaluar DPAPI; solo si 13.1 a 13.8 no bastan. Exige
  levantar `forbid(unsafe_code)` en ese módulo y revisión manual.

## Extras posteriores a la versión final

Van después de la fase 9, del instalador de Windows y del paquete de Linux. No
entran en el alcance de la versión final.

- [ ] **X-01** (pedido del 15 de noviembre) **Modo limpio: barras ocultables.**
  Un interruptor oculta las barras superiores y deja un recuadro de texto muy
  limpio. Al llevar el cursor al borde de arriba, la barra aparece; al
  alejarlo, se esconde. Diseño previo, para validar al llegar:
  - **Qué se oculta:** fila de ventana y menús, tira de pestañas, barra
    contextual y barra de estado. Se pueden elegir por separado (arriba y
    abajo) en una segunda versión.
  - **Cómo aparecen:** una zona sensible de unos 6 px en el borde superior (y
    otra, opcional, en el inferior). La barra baja tras 150 ms de puntero
    quieto y se esconde 600 ms después de que el puntero sale de ella. Con
    movimiento reducido (`reduce_motion`) aparece y desaparece sin animación.
  - **Superpuestas, no empujan:** las barras flotan sobre el texto; el texto no
    se mueve ni se reordena al mostrarlas.
  - **Nunca se ocultan** mientras haya un menú, un diálogo, la búsqueda, un
    panel o un arrastre abierto, ni mientras el puntero esté sobre ellas.
  - **Teclado:** F6 y F10 las muestran y Esc las vuelve a ocultar; el
    interruptor tiene un atajo con Ctrl y vive en el menú Vista. Sin barras,
    sigue habiendo una forma de mover, maximizar y cerrar la ventana (la zona
    sensible muestra también los controles).
  - **Una pista mínima:** un filo de acento de 2 px arriba, para que se sepa
    que hay una barra escondida. Se puede apagar.
  - **Se recuerda** en la configuración, apagado por defecto.
  - **Depende de:** la fase 8 (animación y movimiento reducido), `UiTarget`
    (la zona sensible es un objetivo más), el orden de foco F6 y la pila de
    capas del tooltip.
  - **Pruebas:** zona sensible sin solapes de 640 a 2000 px, tiempos con un
    reloj simulado, escenas visuales con las barras ocultas y reveladas, y que
    nada se oculte con un diálogo abierto.

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
| 5 | Hecho | 5.1 a 5.4 y 5.8; la 5.5 a 5.7 pasaron a la fase 11 |
| 11 | Hecho | 11.1 a 11.8; tooltip de «Quitar del mosaico» a la fase 8 |
| 6 | Hecha | 6.1 a 6.8 y 13.3 a 13.6 |
| 7 | Hecha | 7.1 a 7.8 |
| 8 | Hecha | 8.1 a 8.10; quedan dos retoques de la fase 11 y el árbol de accesibilidad |
| 12 | Hecha | 12.1 a 12.8; el arrastre real entre ventanas queda por probar a mano |
| 9 | Pendiente | — |
| 14 | Hecha | 14.1 a 14.9 hechas; falta el QA manual en la ventana real y el resumen en Comparar |
| 13 | Hecha | 13.1 a 13.8; queda la opcional 13.9 (apagado de Windows y cifrado, con `unsafe`) |
