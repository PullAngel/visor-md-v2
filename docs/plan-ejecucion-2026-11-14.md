# Plan de ejecución · rediseño y correcciones

Preparación completa del plan definitivo de
[`evaluacion-usabilidad-2026-11-14.md`](evaluacion-usabilidad-2026-11-14.md#plan-definitivo-de-cambios),
con las herramientas elegidas en
[`herramientas-rediseno-2026-11-14.md`](herramientas-rediseno-2026-11-14.md).

Cada tarea dice **qué** se cambia, **dónde** (función o constante actual),
**cómo se prueba** y **cuándo está terminada**. Sirve para avanzar sin
acompañamiento: se toma la primera tarea pendiente de la fase en curso.

Los números de línea son de `2cc4a2f` y van a moverse. Las anclas que
valen son los nombres de función.

## Herramientas y entorno

| Herramienta | Estado | Uso |
| --- | --- | --- |
| Plugin **Design** de Anthropic (`ux-writing`, `design-critique`, `accessibility-review`, `user-research`) | Lo instala el propietario desde la tarjeta de claude.ai | Fase 7 (textos). Fases 3, 5 y 8 (crítica de capturas). Fase 8 (accesibilidad). Fase 9 (síntesis). |
| **kompari** (`kompari.exe`, CLI) | Instalado desde `linebender/kompari`, fuera de `Cargo.toml` | Revisar y aprobar las imágenes de referencia: `kompari review tests/visual/referencia target/visual/actual` |
| **Accessibility Insights for Windows** | Pendiente. El instalador oficial (firmado por Microsoft) se descargó, pero Windows canceló la elevación (código 1602). | Medir contraste con cuentagotas en la fase 8. Si no se instala, el contraste se valida con la prueba de `theme.rs` y la medición por píxel de las capturas. |
| Herramienta de evaluación ([`scripts/evaluacion/`](../scripts/evaluacion/README.md)) | Lista | Capturas en vivo de cada fase y re-evaluación en la fase 9 |
| `/code-review` y `/simplify` | Incluidas | Sobre el diff de cada fase, antes del último commit |

## Reglas para todas las fases

1. Commits pequeños en `main`, publicados, con
   `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
2. Cada commit pasa:
   - `cargo fmt --check`
   - `cargo clippy --all-targets -- -D warnings`
   - `cargo test`
   - `scripts/check-docs.ps1`
3. Cada defecto corregido recibe una prueba. La prueba debe fallar antes del
   cambio cuando sea posible.
4. Ningún cambio toca bytes del documento salvo los de la tarea, y las pruebas
   de `editor.rs` lo verifican.
5. Desde la fase 2, todo cambio visual actualiza las imágenes de referencia que
   corresponden. El commit dice cuáles y por qué.
6. Al cerrar cada fase:
   - capturas en vivo con la herramienta de evaluación, en noche y día, a
     640×480 y 1100×720;
   - `design-critique` sobre esas capturas, si el plugin está instalado;
   - se corrige lo que salga con severidad S2 o más;
   - `/code-review` sobre el diff de la fase;
   - se actualiza la tabla «Estado» al final de este documento y la de la
     evaluación.
7. Al terminar una fase, ejecutable nuevo en
   `C:\Dev\archivo\visor-md-v2-ejecutable\` con el anterior como respaldo.
   Solo si la fase cambió algo que el propietario puede probar.

---

## Fase 1 · Correcciones funcionales

Objetivo: cerrar el único S1 y los defectos de estado. No hay cambios visuales.

| # | Tarea | Dónde | Prueba | Terminada cuando |
| --- | --- | --- | --- | --- |
| 1.1 | Buscar en documento acepta `KeyEvent::text`, sin Ctrl ni Alt, como la paleta | `handle_search_key`. Reutilizar el criterio de `direct_source_text` y `handle_command_palette_key`. | Prueba unitaria: un evento con `text = "napo"` deja `search_query == "napo"`, y Ctrl+F no agrega texto | Se puede escribir en vivo con la herramienta de evaluación |
| 1.2 | Ídem para Buscar en carpeta | `handle_workspace_search_key` → `append_workspace_search_text` | Igual que 1.1 | Lucía puede completar la tarea 7 |
| 1.3 | Un clic sin puntero conocido no redimensiona ni actúa | Clic izquierdo en `window_event`: el cálculo de `resize_direction` exige `self.pointer.is_some()`. Es el cambio 3 de `build-evaluacion.patch`. | Prueba de `resize_direction_at` con puntero ausente más una revisión de los demás `map_or(0.0, …)` del manejador | No queda ningún `pointer.map_or(0.0` que active acciones |
| 1.4 | Contador de palabras y caracteres en vivo en Editar | Crear `DocumentMetrics::from_source(&str)`. La barra de estado lo usa cuando `mode.is_editable()`, recalculado con un retraso de unos 250 ms (fecha límite en `about_to_wait`, sin hilos). | Pruebas de `from_source` con Markdown, Unicode y bloques de código | Escribir actualiza el contador |
| 1.5 | Estado «nuevo · sin guardar» en documentos nunca guardados | Rótulos `"guardado"` y `"sin guardar"` en `redraw` (barra de estado); usar `source_identity.is_none()` | Prueba de la función de rótulo extraída | Un documento nuevo no dice «guardado» |
| 1.6 | Concordancia y conteos | `panel_footer` (se elimina en la fase 3, pero ahora se corrige el «1 resultados»), título de la paleta, cabecera del árbol | Pruebas de texto para 0, 1 y n | Ningún «1 resultados» ni conteo distinto de las filas |

Commits sugeridos:
1. 1.1 y 1.2 juntos.
2. 1.3.
3. 1.4 y 1.5.
4. 1.6.

Al cerrar la fase, ejecutable nuevo para el propietario: el buscador es lo que
más le importa.

---

## Fase 2 · Base: dibujo sin ventana, objetivos interactivos y animación

Objetivo: la infraestructura que usan las fases 3, 5 y 8. Habilita las pruebas
de imagen de referencia.

| # | Tarea | Dónde | Prueba | Terminada cuando |
| --- | --- | --- | --- | --- |
| 2.1 | Separar el dibujo del cuadro de la ventana | `redraw` (unas 3000 líneas). Extraer `fn render_frame(&mut self, pixmap: &mut Pixmap, width: u32, height: u32, now: Instant)`. `redraw` queda como envoltura que obtiene el tamaño, llama a `render_frame` y copia a softbuffer. Las llamadas a `window.request_redraw()` dentro del dibujo pasan a un indicador `redraw_requested`. | Las pruebas actuales siguen verdes y el benchmark `--bench` no empeora más de 5 % | El cuadro se dibuja en una prueba sin ventana |
| 2.2 | `App` de prueba sin ventana | Crear `App::for_tests(size, palette)` con fuentes embebidas solamente; reutilizar la inicialización de `main` | — | Se construye en `cargo test` |
| 2.3 | Pruebas de imagen de referencia | Módulo nuevo `src/visual_tests.rs` (solo `#[cfg(test)]`). Escenas iniciales: lectura, edición, comparación, paleta, menú Bloques, índice, dos pestañas, diálogo de cierre y aviso; cada una en noche y día, y a 640×480 y 1100×720 (36 imágenes). Las referencias van en `tests/visual/referencia/` y las fallas escriben en `target/visual/actual/` más un diff. | Diferencia de píxel con tolerancia de canal ≤ 2 y ≤ 0,1 % de píxeles distintos | `cargo test` compara las 36 escenas y `kompari review` muestra las diferencias |
| 2.4 | Regenerar referencias a propósito | Variable `VISOR_BLESS=1` que reescribe las referencias | — | Está documentado en [`testing.md`](testing.md) |
| 2.5 | `UiTarget` y `ui_target_at` | Enum nuevo en `src/ui_target.rs`. Lo usan el dibujo, el clic y `CursorMoved`. Reemplaza las comprobaciones sueltas de hover de `redraw` y la lista de casos del cursor de mano. | Para cada `UiTarget`, `ui_target_at` del centro de su rectángulo devuelve ese objetivo a 640, 1100 y 1366 px. Coincidencia con `toolbar_action_at`, `tab_index_at` y `code_copy_bounds_in_pane`. | El cursor de mano y el hover salen de una sola función |
| 2.6 | Estado de hover y pulsación con transición | `hover: Option<(UiTarget, Instant)>` y `press: Option<(UiTarget, Instant)>`; progreso con smoothstep como `ModeTransition`; fecha límite en `about_to_wait`; instantáneo con `reduce_motion` | Pruebas de progreso 0→1 y de Reducir movimiento | Sin cuadros extra cuando no hay transición (contador de cuadros en el log) |
| 2.7 | Capa única de tooltip | Un solo tooltip vivo, dibujado al final del cuadro, encima de paneles y diálogos. Se oculta al pulsar, al abrir un menú, con la rueda y con un diálogo modal. | Escena de referencia «tooltip sobre columna» y prueba de estado | El tooltip no queda detrás de la columna ni encima de un diálogo (hallazgo de la herramienta) |

Riesgo principal (2.1): `redraw` mezcla dibujo con efectos (autoscroll, cachés
y `request_redraw`). Se extrae en dos commits:

1. Mover sin cambiar comportamiento.
2. Aislar los efectos detrás de indicadores.

---

## Fase 3 · Listas, menús y botones que abren y cierran

| # | Tarea | Dónde | Prueba | Terminada cuando |
| --- | --- | --- | --- | --- |
| 3.1 | `open_surface() -> Option<AppAction>` y alternancia | Clic de la barra en `window_event`; `toolbar_action_is_active` recibe `&App` | Tabla pura «acción pulsada × superficie abierta → resultado» | Índice, Archivos, Buscar, Notas que enlazan aquí, búsqueda en el documento, paleta, los cinco menús y símbolos alternan |
| 3.2 | Cambiar de menú en un solo clic | Rama `navigation_panel_is_open()` del clic: si el clic cae en un botón, el selector de modo o una pestaña, se cierra el panel y se procesa el clic | Pruebas de la tabla 3.1 | Un clic sobre otro botón abre su menú |
| 3.3 | `ListView { selected, scroll_top }` | Reemplaza `panel_window` y `panel_item_at`; un `ListView` por lista (`outline_match`, `workspace_path_match`, `backlink_match`, `link_diagnostic_match`, `workspace_search_match`, `command_palette`, `symbol_picker`, `pane_split_match`) | La rueda no cambia la selección; las flechas mantienen la selección a la vista; PgUp, PgDn, Inicio y Fin; listas de 0, 1 y n elementos; clic sobre una lista desplazada | Navegación uniforme en las ocho listas |
| 3.4 | La rueda va a la lista bajo el puntero | `WindowEvent::MouseWheel`: primero la lista flotante o la columna bajo el puntero, después el documento | Prueba con un puntero sobre el panel | El documento no se mueve con un menú abierto |
| 3.5 | Alto según espacio y barra fina | `navigation_panel_geometry` usa `panel_capacity` con tope de 14 filas; barra de 4 px | Escenas de referencia de la paleta a 480 y 720 de alto | La paleta muestra más de 9 filas si cabe |
| 3.6 | Menús anclados y flotantes | Geometría desde el `UiTarget` del botón, con volteo en los bordes; `draw_floating_surface` en paleta y menús | Escenas de referencia de cada menú | Coincide con la crítica de diseño |
| 3.7 | Filas con ícono, nombre y atajo | Separar `AppAction::shortcut()` de `label()`; grupos en la paleta sin filtro | Prueba de que ningún `label()` contiene «·» | — |
| 3.8 | Caja de búsqueda real y estado vacío | Paleta y Buscar en carpeta: borde, cursor y «Escribí para buscar…» | Escenas de referencia | No aparece «Sin resultados» antes de escribir |
| 3.9 | Buscar en documento como barra anclada | `search_overlay` en `redraw`, arriba a la derecha del panel | Escena de referencia | No tapa el texto |
| 3.10 | Símbolos: filtro y emojis primero | `SYMBOLS` reordenado en secciones; `handle_symbol_picker_key` acepta texto; columna de ancho fijo | Prueba de filtrado («plan», «idea») | Los emojis se ven sin desplazarse |
| 3.11 | Índice: abre en la sección visible; flotante preciso | `show_document_outline` y la sección activa del flotante (`floating_outline_rows`): título más cercano al tercio superior, última sección al final; truncar con «…» | Pruebas de la sección activa | Se corrigen QA-12 y QA-13 |
| 3.12 | Alternancias con estado | `AppAction::label()` dinámico para Reducir movimiento, Recuperación, Índice flotante y Fijar | Prueba de rótulos según el estado | Con el ✓ visible |
| 3.13 | Menú contextual completo | `ContextMenu` y `ContextAction`: Cortar, Copiar, Pegar y Deshacer en Editar; tarea; enlace (Abrir, Copiar dirección); tabla (fila y columna); volteo vertical | Pruebas de las acciones por contexto | Se corrige QA-32 |

Cierre de la fase: `design-critique` sobre paleta, menús e índice.

---

## Fase 4 · Integridad al editar

Puede hacerse en paralelo con las fases 2 y 3. Toca `editor.rs` y las
funciones `insert_*` de `main.rs`, no el dibujo.

| # | Tarea | Dónde | Prueba |
| --- | --- | --- | --- |
| 4.1 | Salir de una lista deja una línea en blanco | `SourceEditor::insert_newline` (`editor.rs`) | «- a⏎⏎texto» produce `- a\n\ntexto`, y la vista no lo incluye en la lista |
| 4.2 | Bloques en línea propia con separación | `insert_code_block`, `insert_table`, `insert_callout` y las de cita y regla: función común `insert_block(template)` en `editor.rs` | Insertar después de texto, en medio de un párrafo, al inicio y al final del documento |
| 4.3 | Formato sin selección aplicado a la palabra | Envolturas inline (negrita, cursiva, resaltado, código): si el cursor está dentro de una palabra, se envuelve la palabra | `resu|men` + Resaltar → `==resumen==` |
| 4.4 | Símbolo dentro de una tabla | `insert_symbol`: si la línea es una fila de tabla y el cursor está después del último `|`, se coloca dentro de la última celda | La fila sigue siendo tabla en la vista |

Criterio de la fase: se repite la tarea 8 de Lucía y la vista de lectura queda
como ella espera.

---

## Fase 5 · Pestañas y paneles

| # | Tarea | Dónde | Prueba |
| --- | --- | --- | --- |
| 5.1 | Roles `tab` y `tab_idle` en `Palette` | `theme.rs` (valores medidos en [`revision-interfaz-2026-11-14.md`](revision-interfaz-2026-11-14.md#fase-5--pestañas-con-su-propio-verde-p1p3)); extender la prueba de contraste | Contraste de `text`/`dim` sobre ambos roles |
| 5.2 | Hover, ✕ al pasar el puntero y tooltip con el nombre completo | Dibujo de la barra de pestañas en `render_frame`, `UiTarget::Tab` | Escenas de referencia de pestañas con hover |
| 5.3 | Pestaña fijada anclada y compacta | Orden de `tab_order` al fijar; dibujo con un ícono de chincheta y sin ✕ | Prueba del orden |
| 5.4 | Reemplazar la pestaña «sin título» vacía al abrir un archivo | Apertura de documento: si la activa es nueva, sin cambios y vacía, se reutiliza | Prueba de estado |
| 5.5 | Cerrar una pestaña conserva la división | `close_active_document_tab` y `DocumentPaneTree` (`panes.rs`): solo se quita la hoja de ese documento | Prueba en `panes.rs` |
| 5.6 | Un panel nuevo abre en Leer | `choose_document_pane_split` | Prueba de modo |
| 5.7 | Un solo estilo de selector de modo | Las cabeceras de panel usan el mismo componente, más chico | Escena de referencia de la división |

Cierre de la fase: `design-critique` sobre las pestañas en los dos temas.

---

## Fase 6 · Enlaces, carpeta de apuntes y recuperación

| # | Tarea | Dónde |
| --- | --- | --- |
| 6.1 | Aviso «Abriendo en el navegador: dominio» e ícono ↗ en los enlaces web | `open_focused_link` (rama Web/Mail) y dibujo inline de enlaces |
| 6.2 | Enlace roto: estilo punteado, hover «Esta nota no existe» y clic «Crear nota» | Resolución de wikilinks contra el índice (`open_workspace_wikilink`), estilo en el dibujo inline y acción nueva `CreateNoteFromLink` |
| 6.3 | `[[nota]]` sin carpeta ofrece «Usar la carpeta de este archivo» | `open_workspace_wikilink`, `choose_workspace` con una ruta precargada |
| 6.4 | Una nota abierta desde un enlace abre en Leer | `open_document_path` con un modo explícito |
| 6.5 | Abrir la carpeta muestra el árbol; el árbol se actualiza al guardar y marca la nota abierta | `choose_workspace` → `show_workspace_files`; reindexado incremental tras guardar; chevrons; rutas con «›» |
| 6.6 | Notas que enlazan aquí, con la línea de la mención | `backlink_paths` pasa a guardar ruta y extracto |
| 6.7 | Lista de recuperaciones | `RestoreRecovery` abre una lista (`ListView`) con origen, fecha, vista previa, Restaurar y Descartar (`recovery.rs`) |
| 6.8 | Diálogo de cierre de la app unificado | `Dialog::close_window` (`dialog.rs`): lista de documentos, «Guardar todo», «Cerrar sin guardar» y «Cancelar»; Esc → Cancelar |

Cada tarea lleva pruebas en `dialog.rs`, `recovery.rs` o `workspace.rs`. La
política de seguridad del VFS no cambia: «Crear nota» usa la misma validación
que Guardar como dentro de la carpeta autorizada.

---

## Fase 7 · Lenguaje, avisos y primer arranque

| # | Tarea | Herramienta |
| --- | --- | --- |
| 7.1 | Inventario de todos los textos visibles: `label()`, avisos (`set_notice`), diálogos, tooltips y pies | Script `rg` sobre `main.rs` y `dialog.rs`, con salida en una tabla de trabajo |
| 7.2 | Glosario y voz: propuesta de la evaluación, revisada con `ux-writing` | Plugin Design |
| 7.3 | Reescribir los textos en voseo y con la voz única | Plugin Design más revisión manual |
| 7.4 | Prueba que prohíbe términos retirados (Workspace, bóveda, callout, Backlinks, atómica, TSV, «escribe», «elige») en los textos de la interfaz | Prueba Rust nueva |
| 7.5 | Quitar avisos redundantes y llevar las pistas al pie de los paneles | `set_notice` en las aperturas de paneles y menús |
| 7.6 | Avisos en el área del documento, encima de paneles | Dibujo de avisos en `render_frame` |
| 7.7 | Bienvenida: Abrir carpeta de apuntes, Abrir archivo y Nuevo apunte; recuperación como línea amable | Estado vacío inicial y `Dialog::recovery_privacy` pasa a un aviso no bloqueante con «Más detalles» |
| 7.8 | Botón sol/luna y menú de Guardar (Guardar, Guardar como, Exportar a PDF) | `PRIMARY_TOOLBAR_ACTIONS` y `draw_toolbar_icon` |
| 7.9 | Íconos nuevos: marcador, `</>`, carpeta abierta, carpeta con lupa, triángulo, botón «Título» | `draw_toolbar_icon`, `EDITING_CONTEXT_TOOLBAR_ACTIONS` y `READING_CONTEXT_TOOLBAR_ACTIONS` |
| 7.10 | Chevrons a 4 px del rótulo | Barra rotulada (`CONTEXT_TOOLBAR_GROUP_WIDTH`) |

El glosario final se copia a [`glossary.md`](glossary.md).

---

## Fase 8 · Hover, pulsación y lectura

| # | Tarea | Dónde |
| --- | --- | --- |
| 8.1 | Fundido de hover y destello al pulsar en todos los `UiTarget` | Dibujo con el estado de la fase 2 |
| 8.2 | Copiar: botón redondeado, ícono, ✓ trazado, sin aviso adicional | Dibujo del botón (`code_copy_bounds_in_pane`) y `copy_code_block` |
| 8.3 | Chincheta activa en acento con tooltip; ícono de restaurar al maximizar | Dibujo de `WindowControl` |
| 8.4 | Barra fina de desplazamiento del documento | Panel de documento, visible al desplazar o al pasar el puntero |
| 8.5 | Etiqueta del recuadro dentro del bloque | Dibujo de `Kind::Callout` |
| 8.6 | Separación entre bloques de código y etiqueta de lenguaje | Dibujo de `Kind::Code` |
| 8.7 | Escala H2/H3 y columna centrada en ventanas anchas | Estilos de `Kind::Heading` y cálculo del ancho de texto |
| 8.8 | Contraste del tema de día ≥ 4,5:1 con margen en `dim` e íconos | `theme.rs` y su prueba; `accessibility-review`; cuentagotas de Accessibility Insights si se instaló |
| 8.9 | Capa de texto del PDF en orden de lectura y sin duplicados | `pdf.rs` y `export_pdf_bytes`; verificar con el visor de Windows y copiar y pegar |

Cierre de la fase: `design-critique` y `accessibility-review` con capturas
completas en los dos temas.

---

## Fase 9 · Re-evaluación

1. Build de evaluación con `scripts/evaluacion/build-evaluacion.patch`. El
   cambio 3 ya estará en `main`, así que se quita del parche.
2. Mismos dos perfiles, misma hoja, mismas 16 tareas y perfil vacío.
3. `user-research` compara la ronda 1 con la ronda 2.
4. Criterios de salida (de la evaluación):

   | Indicador | Objetivo |
   | --- | --- |
   | Hallazgos S1 y S2 abiertos | 0 |
   | SUS | ≥ 68 |
   | Tareas sin dificultad | ≥ 80 % |
   | Heurísticas de Nielsen | ≥ 3,5 de promedio, ninguna < 3 |
   | Requerimientos «No cumple» | 0 |

5. Prueba manual del propietario con
   [`manual-qa-v2.md`](manual-qa-v2.md) actualizado.
6. Actualizar [`design.md`](design.md) y [`features.md`](features.md).

## Secuencia de trabajo

```text
Fase 1 ──────────────┐
Fase 4 (en paralelo) ┤
Fase 2 ── Fase 3 ── Fase 5 ── Fase 8 ──┐
           Fase 6 ── Fase 7 ───────────┴── Fase 9
```

La fase 4 es independiente del dibujo y puede ir en una sesión aparte que solo
toque `editor.rs` y las funciones `insert_*`.

## Estado

| Fase | Estado | Último commit |
| --- | --- | --- |
| Preparación (herramientas y este plan) | Hecho | — |
| 1 · Correcciones funcionales | Pendiente | — |
| 2 · Dibujo sin ventana, objetivos y animación | Pendiente | — |
| 3 · Listas, menús y alternancia | Pendiente | — |
| 4 · Integridad al editar | Pendiente | — |
| 5 · Pestañas y paneles | Pendiente | — |
| 6 · Enlaces, carpeta y recuperación | Pendiente | — |
| 7 · Lenguaje, avisos y primer arranque | Pendiente | — |
| 8 · Hover, pulsación y lectura | Pendiente | — |
| 9 · Re-evaluación | Pendiente | — |
