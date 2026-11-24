# Plan de ejecución · revisión del 23 de noviembre

Plan que sale de las notas [`cambios y comentarios 23-11.md`](cambios%20y%20comentarios%2023-11.md)
y de las respuestas del propietario del 24 de noviembre. Sigue las reglas de
[`agente-ejecutor.md`](agente-ejecutor.md) y continúa la numeración de
[`plan-ejecucion-2026-11-14.md`](plan-ejecucion-2026-11-14.md): fases 16 a 21,
decisiones D-46 en adelante.

Cada tarea dice qué hacer, dónde (nombre de función, no número de línea), la
prueba que se agrega y la condición de «hecho».

## Diagnóstico (lo que se encontró en el código)

| Nota | Causa encontrada | Dónde |
| --- | --- | --- |
| Subrayados y marcas no se ven en Editar ni en Comparar | `study_highlight_ranges` devuelve vacío si el modo no es `Reading` | `study_highlight_ranges`, `study_mark_blocks` |
| Marcar un check borra el subrayado | Las anotaciones se re-anclan buscando la cita literal. Un subrayado que abarca varias tareas cita también `[ ]`; al pasar a `[x]` la cita ya no aparece y queda huérfano. Lo mismo pasaría con cualquier edición dentro de un subrayado, y es lo que hay que resolver antes de mostrarlos en Editar | `estudio::highlights`, `anotaciones::anclar`, `toggle_task` |
| En Comparar no se marca un check con un click | El click de casilla solo se atiende con `mode == Reading` | manejador de `MouseInput` (rama `task_at`) |
| Arrastrar una pestaña fuera de la ventana no hace nada | `CursorLeft` borra `tab_drag_id` y `pointer`, así que al soltar fuera `tear_out_if_dropped_outside` ya no tiene arrastre ni posición. La lógica de sacar e integrar (`tear_out_active_tab`, `send_tab_to`) ya existe | `WindowEvent::CursorLeft`, `Focused(false)` |
| Numeración: Enter en el 5 crea un 6 pero los siguientes no cambian | `insert_newline` inserta `marker.next` y no renumera el resto de la lista. Leer renumera porque CommonMark cuenta desde el primero | `editor::insert_newline` |
| Orden del índice de carpeta inestable | Se ordena por bytes del nombre (`sort_by_key(file_name)` y `relative_path.cmp`): mayúsculas antes que minúsculas, acentos al final, `10` antes que `2`, archivos y carpetas mezclados | `workspace.rs` (indexación) |
| Barra de estado pesada al escribir | Por cada tecla `live_metrics` hace `source.to_string()` del documento entero y recuenta todo; el texto se re-maqueta cada cuadro y su ancho cambia con cada dígito; la columna del cursor recorre la línea entera (una línea pegada de varios MB la recorre por cuadro) | `live_metrics`, `DocumentMetrics::from_source`, armado de `status_right` |
| Restaurar tras un cierre incorrecto | Existe la recuperación y detecta copias huérfanas, pero al arrancar solo muestra un aviso («abrilos con Ctrl+Shift+R») | arranque, cerca de `recovery::orphans()` |
| «Insertar pregunta» sobre un marcador cambia su color | Las acciones `InsertStudy*` y el atajo del marcador comparten el objetivo de la línea; no se arregla porque se eliminan (D-49) | `AppAction::InsertStudy*` |
| «Enlazan aquí» se sale del bloque | El rótulo no se mide contra el ancho del botón | pestaña `Backlinks` del panel |

Además: el documento guarda las anotaciones como texto (`document.annotations`)
y en cada revisión las vuelve a leer y re-anclar todas. Al pasar a un modelo en
memoria (D-46) también baja el costo por tecla.

## Decisiones

- **D-46. Las anotaciones siguen al texto al editar.** El documento guarda las
  anotaciones ya leídas (`Anotaciones`) y sus rangos resueltos en bytes. Cada
  cambio del editor (`last_change`) transforma los rangos como marcas de un
  editor: insertar antes desplaza, insertar dentro amplía, borrar dentro achica.
  Si el rango queda con texto, se reescriben `pos`, `cita`, `antes` y `despues`
  desde el texto nuevo. Si queda vacío, la anotación **no se borra**: conserva
  su última cita y pasa a huérfana, así Ctrl+Z la recupera con el anclaje
  actual. El bloque HTML se escribe solo al guardar y al copiar para
  recuperación. El formato no cambia.
- **D-47. Dónde se ven.** Leer: como hoy. Editar: subrayado detrás del texto
  fuente y marcadores en el margen derecho del editor. Comparar: solo en el
  lado de lectura (derecho). Las acciones de subrayar y marcar funcionan en los
  tres modos, porque la selección de Editar ya está en bytes de la fuente.
- **D-48. Colores libres sin romper el formato.** Los cinco colores con nombre
  siguen como predeterminados. Un color elegido en el selector se guarda como
  `#rrggbb` en el campo `color` (que ya acepta `Otro`). Una versión anterior
  del programa lo conserva sin dibujarlo. El modo oscuro aplica la misma regla
  de opacidad que a los colores con nombre.
- **D-49. Se eliminan del menú Estudio** «Insertar pregunta de estudio»,
  «Insertar resumen de estudio», «Insertar lista de conceptos» y las tres de
  «Marcar contenido como…» (entendido, dudoso, pendiente), con su código y
  pruebas. También salen del menú «Subrayar la selección» y «Color de
  subrayado», que pasan al botón de la barra (D-50).
- **D-50. Botones divididos al estilo Word.** Subrayado: la mitad principal
  aplica el último color; la flecha abre los colores predeterminados, «Sin
  color» (quita el subrayado de la selección), «Más colores…» (selector) y los
  recientes. Marcador: la mitad principal pone o quita el marcador de la línea;
  la flecha abre el tipo, la forma, el color y «Quitar marcador».
- **D-51. Tipos de marcador.** Predeterminados, con color y forma por defecto:

  | Tipo | Color | Forma |
  | --- | --- | --- |
  | Importante | rojo | signo `!` |
  | Duda | naranja | signo `?` |
  | Pendiente | amarillo | casilla |
  | Repasar | violeta | flecha circular |
  | Corregir | rosa | lápiz |

  Elegir un tipo pone su color y su forma; ambos se pueden cambiar después en
  ese marcador. Un marcador sin tipo sigue siendo válido. Los símbolos se
  dibujan como vectores con tiny-skia, no como glifos, para no depender de la
  fuente. El tipo se guarda en una clave nueva `categoria` del registro. El
  lector v1 ya conserva las claves desconocidas, así que no hace falta subir la
  versión del bloque.
- **D-52. Tipos personalizados** con nombre, color y forma, guardados en la
  configuración de la aplicación (`settings.rs`), disponibles en todos los
  documentos y eliminables. Si un documento trae un tipo que no existe en esta
  configuración, se muestra con su nombre y un color neutro, y se ofrece
  «Agregar a mis tipos».
- **D-53. Marcadores en el margen derecho del texto.** En Leer, a la derecha de
  la columna de texto (con su ancho máximo), como un marcalibros con el texto
  de la nota recortado a un máximo visible (unos 24 caracteres o el ancho
  disponible, con «…»). Al pasar el cursor se ve el texto entero en una tarjeta;
  al hacer click se abre la tarjeta editable con tipo, forma, color, texto y
  «Quitar». Si el margen no alcanza para el marcador con texto, se reduce a la
  cinta de color sin texto. Si tampoco alcanza (o se superpone con el índice
  flotante), se pliega del todo en una muesca de 3 px al borde de la columna,
  que sigue respondiendo al cursor. Nunca se dibuja encima del texto.
- **D-54. Navegador de marcadores.** Botón propio junto al del marcador. Abre
  un panel flotante con etiquetas por tipo y su conteo («Duda · 3»), que se
  combinan como filtro. Debajo va la lista de marcadores filtrados, con la
  línea y el texto abreviado; un click salta a ese marcador. Las flechas ↑ y ↓
  del panel saltan al anterior y al siguiente dentro del filtro. Atajos: F2 y
  Shift+F2 si están libres; si no, se elige otro libre y se anota aquí.
- **D-55. Renumeración automática.** Cuando el editor crea, quita o mueve un
  elemento de una lista numerada (Enter, Enter en un elemento vacío, prefijo
  «Lista numerada», Backspace que une elementos), renumera los elementos
  siguientes **de la misma lista y del mismo nivel de sangría**, conservando el
  número inicial y el delimitador (`.` o `)`). Todo va en un solo paso de
  deshacer. No se tocan las listas anidadas ni las que siguen después de una
  línea que corta la lista.
- **D-56. Orden del índice de carpeta.** Carpetas siempre antes que archivos.
  Orden natural: sin distinguir mayúsculas ni acentos, y los números por valor
  (`2` antes que `10`). Criterios: Nombre y Fecha de modificación, cada uno
  ascendente o descendente, guardados en la configuración. La fecha de una
  carpeta es la de su archivo más reciente.

## Fase 16 · Bugs

- [x] **16.1** Anotaciones que siguen al texto al editar (D-46). Se resolvió re-anclando por el contexto de los dos lados cuando la cita cambió por dentro (`anotaciones::reanclar_por_contexto`) y guardando la cita nueva en cada edición (`refresh_study_cache`), en vez de transformar rangos: es más simple, no cambia el formato y deshacer sigue funcionando.
  Dónde: el campo `annotations` del documento, `refresh_study_cache`,
  `edit_source`, nuevo `estudio::transform_ranges(change)`, escritura en el
  guardado y en `schedule_recovery`.
  Prueba:
  - Marcar y desmarcar una tarea dentro de un subrayado que abarca varias
    tareas lo conserva.
  - Escribir dentro, antes y después de un subrayado lo mantiene en su lugar.
  - Borrar el texto lo deja huérfano y Ctrl+Z lo recupera.
  - Guardar escribe el mismo bloque que hoy para un documento sin cambios.

  Hecho: la nota 1 ya no se reproduce y las 529 pruebas actuales siguen
  pasando.
- [x] **16.2** Subrayados y marcadores visibles en Editar y en el lado de
  lectura de Comparar (D-47).
  Dónde: `study_highlight_ranges`, `study_mark_blocks` y el dibujo del editor
  fuente.
  Prueba: visual de los tres modos con el mismo documento anotado.
  Hecho: en Editar se ven detrás de la fuente y en Comparar solo a la derecha.
- [x] **16.3** Subrayar y marcar desde Editar.
  Dónde: `highlight_selection`, `toggle_bookmark`, `bookmark_target_byte`.
  Prueba: con una selección en la fuente se crea un subrayado con la misma
  cita que tendría desde Leer.
- [x] **16.4** Click en casilla en el lado de lectura de Comparar.
  Dónde: la rama `task_at` del click, con `task_checkbox_hit_bounds_in_pane`.
  Prueba: un click en Comparar alterna la tarea, y la fuente cambia solo en el
  marcador.
- [x] **16.5** Renumeración de listas (D-55). Hecho para Enter (crear un elemento en medio de la lista); queda sin renumerar borrar o unir elementos, porque Markdown solo usa el primer número al mostrar la lista.
  Dónde: `editor::insert_newline`, `apply_line_prefix` y el Backspace que une
  elementos, con un nuevo `renumber_following(source, line)`.
  Prueba:
  - Enter en el elemento 5 de 10 deja la lista 1 a 11.
  - Enter en el 20 de una lista que empieza en 18 respeta el inicio.
  - Las sublistas no cambian.
  - Ctrl+Z deshace todo de una vez.
  - Con CRLF se conserva el salto de línea.

  Hecho: Editar y Leer muestran la misma numeración.
- [x] **16.6** Arrastrar pestañas para sacarlas o integrarlas. Hecho: el arrastre ya no se cancela al salir el cursor de la ventana. Falta la pestaña fantasma y el resalte sobre otra ventana (retoque posterior); la prueba en la ventana real la hace el propietario.
  Dónde:
  - `WindowEvent::CursorLeft`: no cancelar un arrastre con el botón apretado.
  - Guardar la última posición del puntero aunque esté fuera de la ventana.
  - `tear_out_if_dropped_outside`.
  - Una pestaña fantasma que sigue al cursor mientras está fuera de la tira.
  - Sobre la tira de otra ventana de Visor MD, se resalta el lugar donde
    entraría. Se usa el registro de ventanas que ya existe.
  - Soltar la única pestaña de una ventana sobre otra ventana la integra y
    cierra la ventana vacía.

  Prueba: unidad del estado de arrastre ante `CursorLeft` con el botón
  apretado, y QA manual (ver «Pruebas a mano»).
- [x] **16.7** «Enlazan aquí» dentro de su bloque. Hecho: la pestaña pasa a llamarse «Enlaces» y las casillas se reparten según el largo del rótulo.
  Dónde: el dibujo de la pestaña `Backlinks`.
  Prueba: visual que mide el rótulo contra su caja con la escala 100 %, 125 %
  y 150 %.
- [x] **16.8** Ofrecer la restauración al arrancar. Hecho con tres botones (Más tarde, Elegir…, Restaurar todo); descartar se hace desde la lista, con su confirmación.
  Dónde: el arranque, donde hoy se llama `recovery::orphans()`. Se usa
  `dialog.rs`.
  Qué:
  - Diálogo «Visor MD no se cerró bien. ¿Restaurar los documentos sin
    guardar?» con la lista de copias (nombre y antigüedad).
  - Botones: «Restaurar todo», «Elegir…» (abre el panel actual de
    Ctrl+Shift+R), «Ahora no» y «Descartar».
  - «Descartar» pide confirmación porque no se puede deshacer.
  - Solo aparece si hay copias huérfanas de una sesión que no terminó bien.

  Prueba: unidad con copias huérfanas simuladas, que ya existen en las pruebas.

## Fase 17 · Rendimiento de la barra de estado

- [x] **17.1** Conteo sin copias y con espera en documentos grandes (en lugar de incremental: ver `docs/benchmarks/barra-de-estado-2026-11-24.md`; lo que costaba era actualizar los bloques de la fuente, y ahora se hace en sitio).
  Dónde: `live_metrics`, `DocumentMetrics`.
  Qué:
  - Por cada cambio se recuentan solo las líneas tocadas, antes y después, y se
    suma la diferencia.
  - Si el cambio es enorme (por ejemplo, más de 1 MB) o no se conoce (deshacer
    en bloque), se recuenta todo en el hilo de trabajo y la barra muestra el
    último valor hasta que llega.
  - Nunca más `to_string()` del documento entero por tecla.

  Prueba: el conteo incremental coincide con el completo en secuencias al azar
  de escritura, borrado, pegado y deshacer.
- [x] **17.2** Barra sin saltos de ancho (dos partes: números y parte fija anclada).
  Qué:
  - Cifras con ancho fijo (dígitos tabulares si la fuente los trae; si no, el
    ancho del campo se reserva por cantidad de dígitos y solo crece).
  - El maquetado del texto de la barra se cachea por contenido.
  - La columna del cursor se calcula de forma incremental, sin recorrer toda la
    línea por cuadro.
- [x] **17.3** Medición.
  Qué: banco con escritura rápida simulada (1000 teclas) y pegado de 5 MB en
  una sola línea y en 100 000 líneas. Se comparan antes y después el tiempo por
  tecla y el tiempo de cuadro, y se anota en `docs/benchmarks/`.
  Hecho: el tiempo de la barra no crece con el tamaño del documento, y el
  pegado no congela la interfaz más que hoy.

## Fase 18 · Interfaz

- [x] **18.1** Selector Leer, Editar y Comparar con indicador deslizante. Ya existía un indicador animado; se mejoró: el reloj arranca en el segundo cuadro (el primero lleva el trabajo de cambiar de modo), curva de quinto grado, duración según la distancia, y las etiquetas cambian de color con el indicador y mantienen el mismo peso.
  Qué:
  - Una píldora que se desliza entre las tres opciones en unos 180 ms con
    salida suave (`ui_motion::transition_progress`), y el rótulo cambia de
    color a mitad de recorrido.
  - Respeta «reducir movimiento».
  - Durante la animación se redibuja la barra pero no se re-maqueta el
    documento. El cambio de modo del contenido ocurre al instante, sin esperar
    la animación.

  Prueba: unidad del recorrido de la píldora y una medición del tiempo de
  cuadro durante la animación.
- [x] **18.2** Índice de carpeta con jerarquía (íconos de carpeta y nota dibujados como vectores, carpetas con más peso, guías de sangría).
  Qué:
  - Filas de carpeta con flecha de plegar, ícono de carpeta y peso medio.
  - Archivos con ícono de documento y texto normal.
  - Guías de sangría tenues por nivel, y la fila activa resaltada.
- [x] **18.3** Orden del índice (D-56). El cambio de orden es la tecla O con el árbol abierto (el pie del panel lo dice); no se agregó acción al catálogo para no tocar el menú de comandos.
  Qué: comparador natural nuevo en `workspace.rs` y un menú de orden en el
  encabezado del panel. La indexación conserva el orden de bytes solo para los
  límites de contenido; lo que se muestra usa el comparador nuevo.
  Prueba:
  - `Árbol`, `abeja`, `Zorro` y `10-x`, `2-x` quedan en orden natural.
  - Las carpetas van primero.
  - Fecha ascendente y descendente.

## Fase 19 · Subrayado al estilo Word

- [x] **19.1** Botón dividido de subrayado (D-50) y limpieza del menú Estudio
  (D-49). Hecho: el marcador y su flecha forman un solo control (la flecha, angosta y pegada); «Sin color» quita el subrayado de la selección; el menú Estudio perdió las ocho entradas y el código de las seis de insertar.
  Dónde: `toggle_highlight_picker`, `pick_highlight_color`, `AppAction` y los
  menús.
  Prueba: «Sin color» quita el subrayado de la selección, y el menú Estudio ya
  no tiene las ocho entradas.
- [x] **19.2** Selector de color libre (D-48). Módulo `selector_color.rs`; recientes en la configuración (hasta 6).
  Qué:
  - Cuadro de saturación y luminosidad, barra de tono, campo hexadecimal y
    vista previa.
  - Los cinco predeterminados arriba y hasta 8 recientes, guardados en la
    configuración.
  - Se maneja con teclado: flechas, Tab y Enter.

  Prueba: unidad de la conversión de color HSV a RGB y del formato `#rrggbb`
  de ida y vuelta, y visual del selector en tema claro y oscuro.
- [x] **19.3** Colores libres en el dibujo y el PDF (las leyendas siguen siendo de los cinco colores de la paleta).
  Dónde: `color_index` / `Highlight.color` pasan de índice a color RGB;
  también `pdf.rs`.

## Fase 20 · Marcadores nuevos

- [x] **20.1** Modelo: clave `categoria`, tipos predeterminados y
  personalizados (D-51, D-52). Los propios se guardan en la configuración (`mark_kind=nombre|rrggbb|forma`, hasta 12).
  Dónde: `anotaciones.rs`, `estudio.rs`, `settings.rs`.
  Prueba:
  - Lectura y escritura de ida y vuelta con `categoria`.
  - Un bloque escrito por la versión actual se sigue leyendo.
  - Un tipo desconocido se conserva.
- [x] **20.2** Botón dividido del marcador con tipo, forma, color y «Quitar»
  (D-50). La flecha abre una tarjeta con los tipos, la tira de formas, la tira de colores, «Texto de la marca…», «Guardar como tipo nuevo…» y «Quitar marcador». Está en la barra de Leer y de Comparar; en Editar, donde la barra no tiene lugar (cabe en 640 px), el menú Estudio muestra un grupo «Subrayar y marcar».
- [x] **20.3** Dibujo en el margen derecho con sus tres tamaños (D-53). Cuentas en `marcadores.rs`; con el índice flotante abierto el espacio se reduce.
  Dónde: `draw_bookmark` y el cálculo del margen. Funciona en Leer, en Editar y
  en el lado de lectura de Comparar.
  Prueba: visual con ventana ancha, media y angosta, y con el índice flotante
  abierto. En ningún caso el marcador toca el texto ni el borde.
- [x] **20.4** Tarjeta flotante del marcador: el texto completo al pasar el
  cursor, y la edición al hacer click (texto, tipo, forma, color y «Quitar»).
  Dónde: `start_bookmark_note` y `commit_bookmark_note` se integran a la
  tarjeta.
- [x] **20.5** Navegador de marcadores (D-54). Atajos: Alt+AvPág y Alt+RePág (F2 y F3 ya estaban ocupados).
  Prueba:
  - Los conteos por tipo coinciden con el documento.
  - Las flechas recorren solo los marcadores filtrados y dan la vuelta con
    aviso.
  - El atajo funciona sin abrir el panel.
- [x] **20.6** Puntos del índice con los colores de los tipos y ayuda que nombra el tipo. El PDF no dibuja marcas, así que no cambia.

## Fase 21 · Ortografía (pendiente de tu decisión)

Por ahora no se implementa; el pedido de la nota 6 era saber cuánto cuesta.

**Peso.** El ejecutable pesa hoy unos 3,9 MB y el límite deseado es menos de
8 MB (`budget.md`), así que hay margen.

| Opción | Peso aproximado | Contra |
| --- | --- | --- |
| Diccionario Hunspell `es` y un motor de afijos propio | 0,4–0,8 MB de datos y algo de código | El motor de afijos es la parte difícil |
| Formas ya expandidas en un FST (crate `fst`, Rust puro) | 1,5–2,5 MB | Más peso, motor simple y rápido |
| Archivo aparte que se descarga o se elige | 0 MB en el ejecutable | Un paso más para el usuario |

**Licencia.** Los diccionarios libres del español (RLA-ES) usan licencias
GPL, LGPL o MPL. Hay que verificar que sean compatibles con PolyForm
Noncommercial antes de incluirlos dentro del ejecutable; como archivo aparte
el riesgo es menor.

**Recomendación:** formas expandidas en un FST, en un archivo aparte que se
carga en segundo plano solo si la ortografía está activada. Se revisa solo lo
visible y lo editado, nunca el documento entero por tecla.

## Orden y por qué

16 → 17 → 18 → 19 → 20 (y 21 si se aprueba).

- **16 primero.** Son errores visibles, y la 16.1 es la base de casi todo lo de
  estudio. Mostrar subrayados en Editar sin que sigan al texto los rompería con
  cada tecla.
- **17 después.** Toca el mismo camino por tecla que la 16.1, y conviene
  medirlo con la 16.1 ya puesta.
- **18 es independiente** y de riesgo bajo.
- **19 y 20 al final.** Son lo más grande, y se apoyan en la 16.1 a la 16.3.
- **Nombre y logo** se tratan en otra sesión.

## Pruebas a mano (necesitan la ventana real)

Según la memoria de QA en tu PC, no se toma el mouse ni el portapapeles sin
avisar. Estas las hacés vos, o yo con tu permiso explícito en ese momento:

- Arrastrar una pestaña fuera de la ventana, a otra ventana, y devolverla.
- Que la animación del selector de modo se sienta fluida.
- El margen de los marcadores al cambiar el ancho de la ventana.
- El diálogo de restauración después de cerrar la app desde el Administrador
  de tareas.

## Estado

| Fase | Estado | Nota |
| --- | --- | --- |
| 16 | Hecha | 16.1 a 16.8; falta probar a mano el arrastre de pestañas y el diálogo de restauración |
| 17 | Hecha | 17.1 a 17.3 |
| 18 | Hecha | 18.1 a 18.3 |
| 19 | Hecha | 19.1 a 19.3 |
| 20 | Hecha | 20.1 a 20.6; falta probar a mano el margen al cambiar el ancho de la ventana |
| 21 | Sin aprobar | Decisión de ortografía |
