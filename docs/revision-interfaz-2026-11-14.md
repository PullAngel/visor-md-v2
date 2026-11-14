# Revisión de interfaz · 14 de noviembre de 2026

Revisión de la interfaz pedida por el propietario después de probar el build
`58f904d`. El objetivo es que la app sea más agradable de usar **sin cambiar
su estilo**: Papel + Tinta, verde de acento, chrome propio, plano dentro del
documento y elevación solo para lo que flota ([`design.md`](design.md)).

El documento tiene tres partes:

- el pedido;
- el paneo, con lo observado en vivo y en el código;
- el plan de cambios por fases.

Este documento no modifica el código.

## Pedido

1. Las listas desplegables (paleta, menús de herramientas, índice, archivos)
   tienen que poder recorrerse con la rueda del mouse. Hoy solo se recorren con
   las flechas, se ven toscas y cuesta ubicarse cuando hay muchas opciones.
2. Un botón que abre algo tiene que cerrarlo con un segundo clic. Hoy el
   índice queda abierto y hay que buscar una ✕ en otro lugar. Esto aplica a
   todo lo que funcione igual.
3. Las pestañas necesitan un verde algo distinto para reconocerlas de un
   vistazo.
4. Animación simple al pulsar botones: Copiar → Copiado con algo más visual, y
   sombreado al pasar el cursor sobre todo lo que se pueda tocar.
5. Revisión profunda de detalles parecidos y plan antes de cambiar código.

## Método

- Se lanzó una copia privada del build actual con un documento de 61
  encabezados, a 900×560. La entrada se envió con mensajes de ventana, sin
  mover el mouse ni usar el portapapeles del usuario. Se capturaron Leer,
  Editar, el índice, la rueda sobre el índice, un segundo clic en el botón, la
  paleta, el menú Listas y dos pestañas.
- Se leyó el código de cada superficie interactiva en `src/main.rs`:
  - paneles (`panel_window`, `navigation_panel_geometry`, `activate_panel_at`);
  - menús (`ContextMenu`, `CommandPaletteScope`);
  - barra (`toolbar_action_is_active`, `toolbar_action_at`);
  - pestañas, botón Copiar y controles de ventana;
  - el manejador de la rueda (`WindowEvent::MouseWheel`).

## Paneo

Severidad: **A**, frena el uso o confunde; **B**, se nota y resta calidad;
**C**, pulido.

### 1. Listas y menús desplegables

| # | Hallazgo | Causa en el código | Sev. |
| --- | --- | --- | --- |
| L1 | La rueda sobre una lista mueve el documento de abajo, no la lista. | `MouseWheel` siempre desplaza el panel de documento enfocado; no consulta si el puntero está sobre un panel. | A |
| L2 | No hay desplazamiento propio: las filas visibles se calculan a partir de la fila elegida, así que ver la fila 40 obliga a elegirla con flechas. | `panel_window(total, selected, capacity)` deriva la ventana del elemento elegido; no existe un offset de scroll independiente. | A |
| L3 | La paleta y los menús muestran siempre 9 filas, aunque la ventana tenga espacio para 20. | `PANEL_CAPACITY = 9` fijo en los paneles flotantes; solo la columna de workspace usa `panel_capacity(geometry)`. | B |
| L4 | No hay barra de desplazamiento; la ubicación solo aparece como texto en el pie («1–11 de 61 · Esc para cerrar»). | No se dibuja ningún indicador. | B |
| L5 | Las filas no se resaltan bajo el cursor. Solo se ve la elegida, con un rectángulo recto. | El dibujo de `NavigationPanelRows` solo pinta `selected`; no conoce el puntero. | B |
| L6 | Los menús Bloques, Listas, Insertar, Estudio y Escritura aparecen siempre en el mismo lugar a la izquierda, no debajo del botón que los abrió. | Son la paleta con un `CommandPaletteScope` y usan la geometría fija `PANEL_X`. | B |
| L7 | La paleta y los menús son planos: sin sombra ni borde, a diferencia de los avisos, diálogos y el menú contextual. | Usan `fill_rect` con `floating_paint`, no `draw_floating_surface`. | B |
| L8 | El atajo va pegado al nombre («Nuevo documento · Ctrl+N»), no alineado a la derecha; no hay iconos ni separadores por grupo en 69 acciones. | `label()` concatena el atajo; las filas son solo texto. | B |
| L9 | El filtro de la paleta no parece un campo de texto: el título cambia a «Acciones: x · n resultados». | No hay caja de búsqueda ni cursor. | C |
| L10 | El tooltip del botón queda encima del menú recién abierto y lo tapa. | El tooltip depende del hover y no se oculta cuando hay un panel abierto. | B |
| L11 | Las listas del índice, los símbolos y la división no tienen PageUp, PageDown, Home ni End (los símbolos sí tienen Re Pág y Av Pág); el índice da la vuelta del último al primero. | Cada `handle_*_key` implementa su propio subconjunto. | C |
| L12 | El índice no marca la sección en la que estás ni abre con ella elegida; siempre empieza en la fila 0. | `show_document_outline` fija `outline_match = 0`. | C |

### 2. Abrir y cerrar (comportamiento de alternancia)

| # | Hallazgo | Causa en el código | Sev. |
| --- | --- | --- | --- |
| T1 | Volver a pulsar Índice, Archivos, Buscar en carpeta o Backlinks no cierra la columna: la reabre y reinicia la selección. Para cerrar hay que usar la ✕ de la columna o Esc. | Las acciones `show_*`/`open_*` solo abren. La columna es «acoplada» y el clic en la barra llega a `perform_action`. | A |
| T2 | Buscar en documento tampoco se cierra con su botón. | `open_document_search` solo abre. | A |
| T3 | Con un menú o la paleta abiertos, el clic en **otro** botón solo cierra lo abierto; hay que volver a pulsar para abrir el nuevo. | El clic fuera de un panel flotante llama a `dismiss_navigation_panel` y termina el evento. | B |
| T4 | Los botones no muestran que su panel está abierto. | `toolbar_action_is_active` solo conoce el modo de edición. | B |
| T5 | Tocar Leer, Editar o Comparar con un menú abierto también se pierde en el primer clic, por la misma causa que T3. | Igual que T3. | C |

Mismo patrón, para revisar con la regla nueva: el índice flotante, el selector
de símbolos, el menú contextual (clic derecho sobre el mismo lugar) y Espacio
de trabajo (`WorkspaceHub`).

### 3. Pestañas

| # | Hallazgo | Sev. |
| --- | --- | --- |
| P1 | La pestaña activa usa el color del documento (`surface #0D1712`) sobre el fondo de la barra (`bg #08100C`); la diferencia es casi imperceptible y lo único que la distingue es el filete verde superior. | B |
| P2 | Las pestañas inactivas no tienen fondo y el hover usa `elevated`, que apenas se ve sobre el fondo. | B |
| P3 | El botón + y la ✕ de la pestaña no cambian al pasar el cursor, más allá de un fondo tenue. | C |

### 4. Hover y respuesta al pulsar

Inventario de lo interactivo y su respuesta actual:

| Elemento | Hover | Al pulsar |
| --- | --- | --- |
| Barra principal y barra contextual | Rectángulo recto `elevated` | Nada |
| Selector Leer · Editar · Comparar | Sin hover propio | Indicador animado (180 ms) |
| Pestañas, ✕ y + | Fondo tenue, casi invisible | Nada |
| Controles de ventana | Fondo `surface`; rojo en cerrar | Nada |
| Botón Copiar | **Ninguno** | Cambia el texto a «Copiado» |
| Cabecera de panel dividido (modo, ✕) | **Ninguno** | Nada |
| Filas de menús y paleta | Solo en el menú contextual | Nada |
| Destinos de la columna (Archivos, Índice…) | Fondo flotante | Nada |
| Casillas de tarea y triángulos de sección | Solo cambia el cursor | Nada |
| Botones de diálogo | Sí (alfa) | Nada |

Hallazgos:

- **H1 (B).** No hay un modelo común de «objetivo interactivo». El hover se
  recalcula en cada lugar con geometrías propias, lo que explica que algunas
  superficies no lo tengan. Lo mismo pasa con el cursor de mano, que en
  `CursorMoved` se decide con una lista de casos.
- **H2 (B).** Ningún botón responde visualmente al pulsar.
- **H3 (B).** El botón Copiar es un rectángulo recto sin hover. La
  confirmación es solo el cambio de texto, sin icono ni transición.

### 5. Otros detalles

| # | Hallazgo | Sev. |
| --- | --- | --- |
| D1 | Al abrir el índice, el aviso queda centrado en la ventana y la columna lo tapa a medias («…l documento · flechas eligen…»). | B |
| D2 | Abrir un menú o panel siempre dispara un aviso con instrucciones («listas y tareas · elegí una variante»). Repite lo que ya dice el panel y compite con avisos útiles. | B |
| D3 | El tooltip se dibuja como una cita (filete verde a la izquierda, sin sombra), pegado bajo la barra, y aparece sin demora. | C |
| D4 | «Comparar» llena su segmento hasta el borde; Leer y Editar tienen aire. | C |
| D5 | El icono `>` de «plegar sección» no se entiende; parece un prompt. | C |
| D6 | Entre la barra contextual y el inicio del documento hay unos 48 px vacíos en Leer y Editar. Revisar si es margen intencional o espacio heredado del antiguo panel. | C |
| D7 | La chincheta fijada debería verse claramente activa (acento) también sin hover. Verificar en la fase de pulido. | C |

### Lo que funciona y se mantiene

- La paleta Papel + Tinta y su contraste (las pruebas de `theme.rs` exigen 7:1
  en texto y 4,5:1 en acento y tenue).
- Los diálogos propios, el menú contextual y los avisos, que ya flotan con
  sombra y son la referencia visual para menús y paleta.
- El selector de modo animado, que respeta **Reducir movimiento**.
- La columna de workspace acoplada: no tapa el documento y se mantiene.

## Principios para los cambios

1. **Un clic abre, el mismo clic cierra.** El botón muestra si su panel está
   abierto. Pulsar otro botón cambia directo, en un solo clic. Esc y la ✕
   siguen funcionando.
2. **Lo que flota parece flotar:** menús, paleta, tooltips y avisos usan
   `draw_floating_surface`. Lo acoplado (la columna) sigue plano.
3. **Las listas se comportan como listas.** La rueda desplaza lo que está bajo
   el puntero. El desplazamiento no depende de la fila elegida, el hover
   resalta la fila y una barra fina indica la posición.
4. **Todo lo que se puede tocar responde dos veces:**
   - Hover, con un fundido de 120 ms.
   - Pulsación, con un destello de 160 a 180 ms. Lo que sale, sale más rápido
     que lo que entra.

   El texto nunca se mueve; solo cambian color y opacidad. Con **Reducir
   movimiento** todo es instantáneo, sin perder el estado.
5. **Menos avisos, más contexto en el lugar.** Las instrucciones de un panel
   van en su pie, no en un aviso.
6. **Sin dependencias nuevas.** Todo se hace con tiny-skia y parley. Las
   animaciones despiertan el event loop solo en las fechas límite, como hoy el
   cambio de tema.

## Plan de cambios

Cada fase termina en uno o dos commits pequeños en `main`, con `cargo fmt`,
`cargo clippy --all-targets -- -D warnings`, `cargo test` y
`scripts/check-docs.ps1`, más una captura en vivo de lo que cambió. El orden
pone primero lo que bloquea el uso (A) y la base que reutilizan las fases
siguientes.

### Fase 1 · Base: objetivos interactivos y animación

- Agregar `UiTarget`, un enum de todo lo interactivo:
  - botón de barra (i) y botón contextual (i);
  - segmento de modo;
  - pestaña (i), ✕ de pestaña y botón +;
  - control de ventana;
  - fila de lista (i) y fila de menú (i);
  - Copiar (bloque, vista);
  - botón de cabecera de panel;
  - destino de columna;
  - botón de diálogo;
  - casilla de tarea y triángulo de sección.
- Agregar una sola función `ui_target_at(x, y)` que usen el dibujo, el clic y
  el cursor de mano.
- Agregar al estado `hover: Option<(UiTarget, Instant)>` y
  `press: Option<(UiTarget, Instant)>`, con una función de progreso suavizado
  (smoothstep) como la de `ModeTransition`. `about_to_wait` programa el
  próximo cuadro solo mientras haya una transición activa. Reducir movimiento
  la vuelve instantánea.
- Pruebas:
  - `ui_target_at` coincide con las geometrías existentes de
    `toolbar_action_at`, pestañas y Copiar a 640 px, 900 px y ventana
    maximizada;
  - el progreso es 0 → 1 y queda en 1 con Reducir movimiento.

Esta fase no produce cambios visibles. Reduce el riesgo de las demás.

### Fase 2 · Abrir y cerrar con el mismo botón (T1–T5)

- Agregar `open_surface() -> Option<AppAction>`, que dice qué acción abrió la
  superficie visible (índice, archivos, búsqueda en carpeta, backlinks,
  búsqueda en documento, paleta, cada menú de herramientas, símbolos, índice
  flotante).
- En el clic de la barra:
  - Si la acción pulsada es la abierta, se cierra.
  - Si es otra, se cierra la actual y se abre la nueva en el mismo clic.
  - El clic fuera de un panel flotante, pero sobre un botón o el selector de
    modo, ya no se descarta.
- `toolbar_action_is_active` recibe el estado de la app. El botón cuya
  superficie está abierta muestra el filete de acento y el fondo activo, igual
  que hoy Editar.
- Clic derecho en el mismo lugar con el menú contextual abierto: lo cierra.
- Pruebas puras de la tabla «acción pulsada × superficie abierta →
  resultado», con la regla de seguridad intacta: cerrar nunca descarta texto
  ni cambia el documento.

### Fase 3 · Listas desplazables (L1–L5, L11, L12)

- Agregar `ListView { selected, scroll_top }`, compartido por la paleta, los
  menús, el índice, los archivos, la búsqueda, los backlinks, los símbolos y
  el selector de división. Reemplaza el `panel_window(total, selected, …)`
  actual:
  - Las flechas mueven la selección y, solo si sale de vista, el scroll.
  - La rueda mueve el scroll sin cambiar la selección.
  - Se agregan PageUp, PageDown, Home y End en todas las listas, sin dar la
    vuelta en los extremos.
- La rueda consulta primero si el puntero está sobre una lista abierta (paneles
  flotantes y columna) y solo después desplaza el documento.
- La capacidad sale del alto disponible (`panel_capacity`) también en los
  paneles flotantes, con un tope de unas 14 filas para que un menú no cubra
  toda la ventana.
- Barra de desplazamiento fina de 4 px con bordes redondeados, en tono `dim`
  con alfa; se intensifica con el hover.
- Hover de fila: fondo de acento al 8 %. Fila elegida: acento al 17 % con
  esquinas de 6 px. Ambos con el fundido de la fase 1.
- El índice abre con la sección visible elegida y centrada, y marca la sección
  actual con un punto de acento.
- Pruebas:
  - `ListView` mantiene la selección en vista;
  - la rueda no cambia la selección;
  - los límites con 0, 1 y n elementos;
  - el clic sobre una fila desplazada elige el índice correcto.

### Fase 4 · Menús y paleta con cuerpo (L6–L10, D2, D3)

- Los menús de herramientas se convierten en desplegables anclados bajo su
  botón: se alinean a su borde izquierdo y se corren si no entran. La paleta
  general sigue arriba a la izquierda, pero con superficie flotante.
- `draw_floating_surface` (sombra en capas y borde) en la paleta, los menús y
  el tooltip.
- Filas en tres columnas:
  - icono de 16 px a la izquierda, reutilizando `draw_toolbar_icon` cuando la
    acción tiene uno;
  - nombre;
  - atajo alineado a la derecha en tono tenue.

  `AppAction` separa `label()` de un `shortcut()` nuevo.
- Paleta sin filtro: separadores con el nombre del grupo (Archivo, Vista,
  Escritura, Estudio, Ventana).
- Paleta con filtro: lista plana por relevancia.
- Caja de búsqueda visible en la paleta, con lupa, texto y cursor, y el marcador
  «Escribí para filtrar…».
- El pie pasa a ser una pista breve («↑↓ elegir · Enter · Esc») y muestra el
  conteo solo al filtrar.
- El tooltip:
  - flota bajo el botón, con una demora de 400 ms;
  - se oculta al pulsar, mientras haya un menú abierto y al mover la rueda.
- Se quitan los avisos que solo explican cómo usar un panel recién abierto. Se
  conservan los que confirman una acción o un error.

### Fase 5 · Pestañas con su propio verde (P1–P3)

- Dos roles nuevos en `Palette`: `tab` (pestaña activa) y `tab_idle` (fondo de
  las inactivas al pasar el cursor). Propuesta inicial, a validar con capturas:

  | Tema | `tab` activa | `tab_idle` hover | Barra (`bg`) |
  | --- | --- | --- | --- |
  | Noche | `#17301F` | `#112218` | `#08100C` |
  | Día | `#DCE6CF` | `#EDE4CF` | `#E8DDC7` |

  Es un verde algo más saturado que el documento; se distingue de un vistazo
  sin competir con el acento. Contraste medido:

  - Noche: texto sobre `tab` 11,6:1; tenue sobre `tab` 5,5:1 y sobre
    `tab_idle` 6,4:1; acento sobre `tab` 6,9:1.
  - Día: texto sobre `tab` 11,8:1; tenue sobre `tab` 4,7:1 y sobre `tab_idle`
    4,8:1; acento sobre `tab` 4,9:1. La pestaña activa conserva su filete superior y
  la base recta que la une con la barra contextual.
- La ✕ y el + tienen su propio hover, un círculo tenue de 20 px.
- Las pruebas de contraste de `theme.rs` se extienden a `text` sobre `tab` y
  `dim` sobre `tab_idle` (7:1 y 4,5:1).

### Fase 6 · Hover y pulsación en todo (H1–H3)

- Hover con fundido en todos los elementos de la tabla del punto 4. Los fondos
  de la barra pasan a esquinas de 6 px.
- Pulsación: destello de acento al 22 % que se apaga en 160 ms sobre el
  objetivo pulsado. En los controles de ventana, el mismo tono de su hover,
  más oscuro.
- **Copiar → Copiado:**
  - En reposo: botón redondeado con icono de dos hojas y «Copiar».
  - Hover: más contraste.
  - Al pulsar, en 160 ms, el fondo pasa a acento al 30 % y el icono cambia a
    un ✓ dibujado como trazo que se completa de izquierda a derecha. El texto
    pasa a «Copiado».
  - A los 1,6 s se desvanece de vuelta en 200 ms.
  - Con Reducir movimiento cambia de inmediato y vuelve igual.
- Casillas de tarea y triángulos de sección: anillo de hover y destello al
  pulsar.
- Cursor de mano en todo lo que devuelve `ui_target_at`, cursor de texto en el
  documento y flecha en el resto.

### Fase 7 · Pulido (D1, D4–D7)

- Los avisos se centran en el área del documento (descontando la columna) y se
  dibujan por encima de paneles y columna.
- El selector de modo reparte el ancho según el texto o se ensancha a 240 px si
  entra a 640 px. Se verifica con la aserción `const` actual.
- Icono nuevo para plegar sección: un triángulo de sección, igual al que se ve
  en los encabezados.
- D6: medir y decidir. Si el hueco no tiene función, se acerca el documento a
  la barra.
- La chincheta activa se ve en acento.
- Repaso completo en tema de día.

### Fase 8 · Verificación y documentación

- Capturas en vivo de cada superficie, en noche y día, a 640×480, 900×560 y
  maximizada.
- Actualizar [`design.md`](design.md):
  - roles `tab`/`tab_idle`;
  - tabla de estados hover y pulsado;
  - regla «un clic abre, el mismo clic cierra».
- Actualizar [`features.md`](features.md) y
  [`manual-qa-v2.md`](manual-qa-v2.md) con las comprobaciones nuevas.
- Nuevo ejecutable en `C:\Dev\archivo\visor-md-v2-ejecutable\` para la prueba
  del propietario. El anterior queda como respaldo.

## Riesgos y cómo se controlan

- **Más cuadros por segundo por las animaciones.** Solo hay transición
  mientras dura un hover o una pulsación (≤ 200 ms) y el loop despierta en la
  fecha límite, sin un bucle continuo. Se mide con el benchmark de scroll
  existente antes y después de la fase 6.
- **Clics que antes se descartaban ahora actúan (fase 2).** La tabla de pruebas
  cubre cada combinación. Cerrar una superficie nunca toca el documento.
- **Las geometrías se desincronizan.** Este fue el origen de bugs anteriores.
  La fase 1 deja una sola fuente de verdad para dibujo, clic y cursor.
- **El tamaño del binario.** No se agregan dependencias ni recursos; el
  presupuesto de 7 MB no corre riesgo.

## Estado

| Fase | Estado |
| --- | --- |
| Paneo y plan | Hecho (este documento) |
| 1 · Base de objetivos y animación | Pendiente |
| 2 · Abrir y cerrar con el mismo botón | Pendiente |
| 3 · Listas desplazables | Pendiente |
| 4 · Menús y paleta | Pendiente |
| 5 · Pestañas | Pendiente |
| 6 · Hover y pulsación | Pendiente |
| 7 · Pulido | Pendiente |
| 8 · Verificación y documentación | Pendiente |
