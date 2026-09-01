# Estado actual

Última revisión: 30 de agosto de 2026.

## Resumen

Visor MD v2 ya es una base nativa de uso diario en desarrollo: abre Markdown y
texto inerte, conserva varios documentos, edita y guarda con protección ante
conflictos, ofrece vista dividida y recorre una carpeta autorizada mediante una
VFS acotada. La referencia anterior a Codex permanece intacta en
`archive/claude-pre-codex`.

El lector y el editor tienen gates automáticos verdes. Permanecen pendientes el
QA humano acumulado, la actualización incremental de algunos modelos visibles,
accesibilidad completa y distribución.

## Git

- Rama principal: `main` y `origin/main`, con el estado validado de Codex.
- Rama de trabajo conservada: `codex/sprint-1-recovery`; no es la referencia
  principal ni debe usarse como descripción del estado publicado.
- Referencia histórica inmutable: `archive/claude-pre-codex` y
  `origin/archive/claude-pre-codex`, ambos en el antiguo `main` (`090e9de`).
- Documentación base: `738faff`.
- Fuentes y reproducción: `d566518`.
- Renderer y modo seguro: `a54c9d6`.
- Los snapshots de preservación no son parte del producto.
- La recuperación se aplicó a `main` por avance rápido, sin reescritura de
  historia; los commits anteriores siguen accesibles por ambas referencias.

Consultar [`workspace-handoff.md`](workspace-handoff.md) para el inventario
exacto del traspaso.

## Implementado en el último commit estable

- ventana nativa con `winit`;
- framebuffer por software;
- parsing Markdown básico;
- layout de texto y render visible;
- virtualización inicial;
- tema claro y oscuro;
- detección del tema del sistema;
- fuentes embebidas;
- perfiles release orientados a tamaño;
- mediciones iniciales de apertura, scroll y memoria.

## Trabajo recuperado y probado

- tramos inline con estilo;
- negrita, cursiva y anidamiento;
- tachado y decoraciones;
- listas, blockquotes y reglas horizontales;
- task lists;
- límites de recursión;
- fallback completo a fuente inerte;
- rangos de fuente para bloques y tramos;
- texto UTF-8 abierto retenido durante la sesión junto a esos rangos;
- destinos de enlaces e imágenes preservados como datos;
- semántica de tablas y lenguaje de bloques de código;
- HTML desconocido visible e inerte;
- allowlist HTML nativa y sin atributos: `br`, `kbd`, `mark`, `sub` y `sup`;
- búsqueda binaria del tramo visible;
- scroll limitado por el viewport real;
- fallos de inicialización y presentación gráfica reportados sin `panic`;
- 53 pruebas unitarias y adversariales, incluido un corpus integrado de lector
  y un barrido adversarial determinista.
- copia explícita de selección: texto visible con `Ctrl+C` y Markdown fuente de
  bloques con `Ctrl+Shift+C`; falta QA manual con aplicaciones externas.
- apertura primaria desde un mismo handle, limitada a 16 MiB y UTF-8 válido;
  no habilita recursos secundarios ni navega rutas del documento.
- apertura y parsing fuera del hilo de interfaz; la ventana se crea mientras el
  modelo se prepara y no recibe un estado parcial.
- enlaces visibles con cursor y destino declarado al hover, sin resolver rutas
  ni abrir navegador o filesystem.
- enlaces web y correo en azul subrayado; destinos relativos en verde y
  destinos bloqueados en tono tenue, sin concederles capacidad alguna.
- Tab y Shift+Tab recorren enlaces sin abrirlos; el foco tiene resalte y muestra
  el destino antes de cualquier acción futura.
- un clic o Enter sobre un enlace enfocado delega solo `http`, `https` y `mailto:` al
  sistema, sin shell ni prefetch; destinos locales esperan VFS y los bloqueados
  permanecen inactivos.
- resize y cambio de escala del sistema invalidan los layouts visibles antes de
  redibujar. El cuerpo, los márgenes, las sangrías, los marcadores y la
  tipografía se reconstruyen en píxeles físicos conservando el ancho lógico de
  línea; queda QA manual de DPI para la percepción de controles flotantes y
  para monitores con escalas distintas.
- los rangos que Comrak informa para enlaces cubren hoy el destino y no toda su
  sintaxis; es suficiente para el lector inerte actual, pero debe resolverse
  antes de edición, round-trip fino o activación de enlaces.
- menú contextual propio inicial: solo ofrece copia de una selección y reutiliza
  el portapapeles explícito; no abre rutas, navegador ni acciones de documento.
- los límites de render muestran una banda superior de modo seguro dentro de la
  ventana y mantienen la fuente inerte disponible para lectura y copia.
- un fallo de apertura asíncrona conserva la ventana y muestra un mensaje
  inerte simple dentro del lienzo; el detalle técnico queda solo en el registro
  local de la sesión.
- solo extensiones Markdown pasan al parser; otros textos UTF-8 se muestran de
  forma inerte, sin comportamiento de IDE ni ejecución.
- `F2` entra en una primera edición de fuente: el Markdown se muestra como
  texto inerte, teclado normal e IME insertan texto y Backspace, Delete,
  Ctrl+Z y Ctrl+Y aplican parches reversibles. Clic y arrastre actualizan cursor
  y selección de fuente con el layout visible. `F2` vuelve a derivar la vista
  Markdown en un hilo de trabajo; un fallo conserva la fuente editada. La
  primera vista posterior a una edición mide los bloques de forma exacta para
  impedir solapamientos; después vuelve la virtualización estimada. `Ctrl+Z`
  desde lectura también actualiza la vista sin perder el historial.
- la fuente editable vive en un Rope por pestaña. Inserción, borrado, navegación
  y undo/redo convierten los offsets de bytes solamente después de comprobar un
  límite UTF-8 exacto. La serialización para parser, guardado y recuperación
  conserva el texto, BOM y EOL; la representación visible de todas las líneas
  aún se reconstruye tras cada cambio y queda como optimización pendiente.
- `F3` abre una comparación lado a lado: fuente editable a la izquierda y
  lectura derivada a la derecha. Un retraso corto agrupa la escritura antes de
  parsear y los resultados conservan identidad de pestaña y revisión. La mitad
  derecha no acepta pegado ni se usa para guardar. Queda QA manual de escritura
  prolongada, ventanas estrechas, DPI y correspondencia de desplazamiento.
- las task lists se dibujan sin depender de glifos de fuente y permiten cambiar
  `[ ]` por `[x]` con clic sobre la casilla. La mutación toca un solo byte de la
  fuente y entra al mismo historial reversible que el editor.
- cada fila GFM se representa como celdas con layouts, estilos y alineación
  independientes; los bordes y el encabezado ya no dependen de dibujar una
  línea aplanada con caracteres `|`. La selección de rangos dentro de una celda
  sigue pendiente: se desactiva antes que devolver offsets falsos.
- guardar compara la identidad y los bytes base antes del reemplazo atómico.
  Ante un conflicto externo, no sobrescribe: permite conservar la edición,
  elegir una copia o recargar solo después de escribir una recuperación local.
- el título muestra `*` junto al nombre cuando hay cambios sin guardar. Es un
  indicador persistente, independiente de avisos temporales o del modo actual.
- abrir o crear conserva el documento actual como otra pestaña lógica; se puede
  recorrer con `Ctrl+PageUp` y `Ctrl+PageDown`, y `Ctrl+W` aplica cierre
  protegido al documento activo. Cada pestaña conserva fuente, historial,
  identidad y recuperación propios. El cierre de la ventana comprueba todos los
  documentos modificados y solo continúa después de preservar cada recuperación.
- la barra inferior presenta las pestañas en un orden estable, marca cambios
  con `*` y permite elegir cada documento con mouse. La posición de lectura se
  conserva al cambiar. Cada guardado asíncrono lleva la identidad de su pestaña,
  por lo que se puede continuar en otra nota sin atribuirle el resultado, el
  baseline o la limpieza de recuperación. Aperturas y renders también llevan
  identidad y versión propias: pueden completar una pestaña inactiva sin
  congelar ni reemplazar la activa.
- una pestaña puede fijarse desde la paleta o el menú contextual. La marca `•`
  es solo de sesión, no se escribe ni se guarda en el Markdown, y evita su
  cierre hasta liberarla explícitamente.
- una barra superior sobria adapta la jerarquía al modo. En lectura hace visibles
  Nuevo, Abrir, Guardar, Editar, Buscar y Más; en edición muestra Guardar, Leer,
  Negrita, Cursiva, H2, Lista, Enlace y Más. Usa las mismas acciones que atajos,
  menú y paleta, para evitar comportamientos distintos según cómo se invoque una
  operación. El acento de modo solo aparece durante edición o vista dividida;
  en lectura, `Editar` vuelve al reposo.
- el viewport reserva la franja inferior completa. El final del documento ya no
  queda bajo pestañas o estado, y bordes de tablas, separadores de pestañas y la
  división de la vista usan el color estructural definido por el sistema visual.
- en Windows la ventana usa chrome sin borde con controles propios de minimizar,
  maximizar o restaurar y cerrar, además de arrastre en su franja libre y resize
  por bordes. El cierre conserva la protección de cambios sin guardar; Linux y
  otros sistemas mantienen decoraciones nativas hasta que exista QA equivalente.
- pasar el mouse por un enlace o recorrerlo con Tab muestra dentro del lienzo su
  tipo y destino declarado. Esta vista previa no resuelve rutas ni toca red o
  disco; el título conserva el destino completo si el aviso debe abreviarse.
- en edición, `Ctrl+B`, `Ctrl+I` y `Ctrl+K`, el menú contextual y la paleta
  aplican negrita, cursiva o enlace como un único cambio reversible. Sin
  selección dejan `texto` seleccionado para reemplazarlo al escribir; nunca
  reserializan el resto del documento. Un enlace con rótulo ya seleccionado
  deja `https://` listo para reemplazar, sin leer el portapapeles.
- el mismo catálogo puede anteponer `## ` o `- ` a la línea actual para crear
  un encabezado H2 o una viñeta. La selección permanece sobre el contenido y
  undo revierte cada ayuda como una sola edición, incluso con CRLF y Unicode.
- la barra de estado cuenta palabras del texto visible del último render y
  muestra una estimación conservadora de lectura a 200 palabras por minuto. No
  tokeniza, no usa red y no promete precisión lingüística para idiomas sin
  separación por espacios.
- la paleta permite filtrar ese catálogo por nombre y mantiene una selección
  acotada para teclado y mouse. La consulta es efímera y nunca se persiste ni se
  mezcla con búsquedas del documento o de la carpeta.
- actualizar el índice de la carpeta forma parte del mismo catálogo de acciones,
  además de `Ctrl+Shift+I`. La actualización sigue siendo explícita, cancelable
  y reemplaza resultados viejos sin instalar vigilancia permanente del disco.
  Recorre la estructura para descubrir notas agregadas o eliminadas, pero reutiliza
  título, encabezados, wikilinks y texto de búsqueda de notas que siguen intactas.
  Esa reutilización solo ahorra trabajo: la VFS vuelve a validar toda navegación.
- índice del documento, árbol plegable de notas, búsqueda de carpeta y backlinks se
  presentan como paneles plegables de varias filas. Las listas largas mantienen
  visible la selección sin construir layouts para todos los resultados y
  conservan navegación por teclado o mouse contenida en el índice autorizado.
- Guardar un documento nuevo, incluso vacío, abre Guardar como. Seguir un enlace,
  backlink o resultado del workspace abre otra pestaña y ya no obliga a guardar
  primero: los cambios del documento anterior permanecen protegidos en su
  propio estado y recuperación.
- los enlaces Markdown relativos se resuelven desde la carpeta de la nota que
  los declara y no desde la raíz global. La nota base y el destino deben estar
  contenidos por la VFS; las anclas `nota.md#encabezado` viajan con la pestaña
  que se abre y `#encabezado` navega solo dentro del documento actual.
- una imagen Markdown sigue siendo un placeholder hasta una acción explícita.
  Clic o Enter permite confirmar una vista previa de un PNG local contenido por
  la VFS; firma, bytes, dimensiones y memoria se limitan antes de decodificar en
  segundo plano. Las imágenes remotas y rutas no permitidas continúan
  bloqueadas sin conexión. Escape o un clic descartan el único pixmap retenido.
- una tabla enfocada o seleccionada puede copiarse como TSV desde el menú
  contextual o la paleta. Se usan las celdas del modelo, no los separadores
  visibles ni una reserialización del Markdown; saltos y tabuladores internos
  se vuelven espacios para no romper filas o columnas al pegar.
- lectura, edición y vista dividida se recuerdan por documento en una
  configuración local versionada. Se conservan hasta 128 identificadores hash,
  nunca rutas, nombres, contenido o permisos; una ruta no conocida abre en
  lectura y un formato de configuración desconocido vuelve al valor seguro.
- los encabezados muestran un control discreto para plegar su sección en modo
  lectura. El estado pertenece a la pestaña y solo altera la geometría visible:
  fuente, historial y selección permanecen intactos. La lista compacta de
  bloques visibles evita recorrer contenido plegado en cada cuadro; buscar,
  seguir un enlace o elegir el índice revela solo los ancestros necesarios.
- cambiar de pestaña conserva también la selección de lectura. En edición, el
  cursor y la selección se reconstruyen desde el `SourceEditor` de esa pestaña,
  por lo que volver a ella no exige pulsar una tecla para recuperar el caret.
- la búsqueda local marca en el borde derecho la posición relativa de todos los
  bloques coincidentes. Reutiliza el resultado de la consulta y omite contenido
  plegado, por lo que no introduce un recorrido completo durante cada cuadro.
  El resultado activo resalta el término exacto mediante un mapeo de minúsculas
  Unicode hacia offsets válidos del texto original. Enter recorre cada aparición,
  incluso cuando una misma línea o bloque contiene varias.

Evidencia actual en Windows:

- formato, Clippy estricto y 161 pruebas verdes el 31 de agosto de 2026 después
  de incorporar plegado de secciones; el gate completo de
  146 pruebas inmediatamente anterior también verificó SBOM, 40 documentos y
  build release;
- release Windows posterior al chrome sin borde: 3.473.920 bytes, 3,313 MiB,
  SHA-256 `4D38FCD8C45242D6FD9DF6DB981BFB7754A97E9F10A258B174DF965EBC1F4E81`.
  El smoke `--bench=0` abrió y pintó la fixture de 23 bloques sin error; sus
  tiempos de 682 ms de ventana visible y 1.764 ms de primer pintado son una
  muestra aislada bajo carga de compilación, no una serie de rendimiento;
- release Windows actual: 3.471.360 bytes, 3,310 MiB, SHA-256
  `FB61E47AA8EC50ECD823202D9AF047E4065C7780DDA1B1536FD1C658D9FF3D2E`;
- smoke test release con `--bench=0` y la fixture visual: salida 0, ventana
  visible en 359 ms y primer cuadro con 23 bloques en 907 ms. Es una muestra
  funcional aislada, no una nueva serie estadística de arranque; la medición de
  layout de ese proceso fue atípica (534 ms), por lo que no se interpreta como
  regresión sin una serie controlada;
- `cargo audit` contra 1.226 advisories: cero vulnerabilidades conocidas y una
  advertencia permitida, `RUSTSEC-2026-0192`, por `ttf-parser 0.25.1` transitivo
  no mantenido. No es una vulnerabilidad demostrada y sigue bajo seguimiento;
- el SBOM contiene 292 componentes externos y su generación fue verificada con
  Windows PowerShell 5.1 y PowerShell 7.6.4. El orden ordinal y JSON compacto
  evitan diferencias puramente culturales o de formato entre runners;

- `cargo test`: 142 de 142 pruebas verdes el 30 de agosto de 2026 tras
  pestañas, paneles, acciones visibles, recuperación configurable y guardados
  dirigidos por identidad de documento, incluida la navegación visible de la
  barra con teclado;
- release Windows del checkpoint `885c095`, anterior solo al foco de teclado y
  cambios documentales: 3.270.144 bytes, 3,12 MiB, SHA-256
  `12958985246E977EF56C48A2282E2AECB40B2DA7B70B6D4DEEEC8513CAE30539`;
- `cargo check`: verde;
- `cargo fmt -- --check`: verde;
- `cargo clippy --all-targets -- -D warnings`: verde;
- `cargo test`: 68 de 68 pruebas verdes tras lector, archivos y texto inerte;
- `cargo test --offline`: 104 de 104 pruebas verdes el 28 de agosto de 2026
  tras corregir el QA visual de tipografía, emoji, citas y copia;
- `cargo test --offline`: 107 de 107 pruebas verdes tras la estabilización de
  reflow, tareas reversibles y presentación real por celdas de tabla;
- `cargo test`: 121 de 121 pruebas verdes el 29 de agosto de 2026 tras el
  recorrido de notas indexadas y la señal de reindexado de workspace;
- release Windows del 29 de agosto de 2026: 3.236.864 bytes, 3,087 MiB,
  SHA-256 `0C31D0F7B092E9721B759160F1B0C56CAC7858BF51722B4F94D70CD972A7D972`;
- `scripts/check.ps1`: verde el 27 de agosto de 2026 tras el cierre parcial de
  Sprint A (formato, Clippy, 67 pruebas, SBOM, documentación y release);
- `scripts/check.ps1`: verde el 27 de agosto de 2026;
- `scripts/check.ps1`: verde el 26 de agosto de 2026 (formato, Clippy, pruebas,
  SBOM, documentación y release);
- último release de `6176a82`: 2.996.736 bytes, 2,858 MiB;
- working tree con allowlist HTML: 3.000.320 bytes, 2,861 MiB;
- working tree con selección por bloque: 3.009.536 bytes, 2,870 MiB;
- working tree con selección de teclado: 3.010.048 bytes, 2,871 MiB;
- working tree con autoscroll de selección: 3.011.584 bytes, 2,872 MiB;
- working tree con Ctrl+A: 3.012.096 bytes, 2,873 MiB;
- working tree con navegación vertical: 3.013.120 bytes, 2,874 MiB;
- working tree con cursor contextual: 3.013.632 bytes, 2,874 MiB;
- working tree con portapapeles de texto: 3.019.264 bytes, 2,879 MiB;
- working tree con apertura primaria limitada: 3.021.312 bytes, 2,881 MiB;
- gate lector y archivos: 3.031.040 bytes, 2,891 MiB;
- gate Sprint A parcial: 3.103.232 bytes, 2,960 MiB;
- correcciones posteriores al QA visual: 3.203.584 bytes, 3,055 MiB;
- reflow, tareas y tablas por celdas: 3.209.216 bytes, 3,061 MiB;
- conflictos de guardado explícitos: 3.211.264 bytes, 3,062 MiB;
- workspace acotado y lector ampliado: 3.222.016 bytes, 3,073 MiB;
- navegación y resaltado portable: 3.231.744 bytes, 3,082 MiB;
- `scripts/check.ps1`: formato, Clippy y 116 pruebas verdes el 28 de agosto
  de 2026; build release medida por separado en este checkpoint;
- primer pintado mediano sobre diez ejecuciones: 102,5 ms;
- P95 de primer pintado: 612 ms;
- scroll automatizado: 4,4 ms por cuadro.

La serie automatizada conserva una ejecución de 612 ms y las muestras crudas.
Ejecuciones anteriores alcanzaron 388, 587, 631 y 965 ms. No se les atribuye
causa porque todavía no se controlan caché, carga y planificación del sistema.

## Pendientes inmediatos

- primera ejecución remota de CI en Windows MSVC y Linux; hasta que GitHub
  informe ambos resultados, el workflow es configuración revisada, no evidencia
  de compatibilidad multiplataforma;

- verificación visual de task lists, decoraciones, temas y cursiva;
- QA manual focalizado de tablas con celdas largas, alineación y de la primera
  vuelta lectura-edición-lectura tras una edición grande;
- selección de ejemplos de la suite oficial CommonMark y ampliación GFM sistemática;
- separación incompleta de `main.rs`; fuentes y tema ya tienen módulos propios;
- VFS de recursos secundarios ya cubre PNG local confirmado; otros formatos o
  capacidades futuras deberán reutilizar la misma frontera sin permisos
  persistentes;
- lista efímera de notas indexadas con `Ctrl+Shift+T`: no recorre el disco ni
  acepta rutas del documento y vuelve a resolver la elección dentro de VFS;
  el árbol deriva directorios solo de rutas contenidas y permite plegarlos; el
  panel de backlinks ya está conectado;
- al volver a enfocar la ventana, una tarea compara metadatos de hasta 1.024
  rutas ya contenidas e indexadas. Si cambiaron, avisa y ofrece reconstrucción
  explícita con `Ctrl+Shift+I`; no instala un watcher ni descubre rutas nuevas;
- panel visible de resultados múltiples sobre el índice de ruta, encabezados y
  contenido acotado a 8 MiB de memoria; `Ctrl+Shift+F` ya permite buscar y abrir
  de forma contenida la coincidencia elegida;
- búsqueda local del documento con `Ctrl+F`, sin persistir ni transmitir la
  consulta, y búsqueda de carpeta con `Ctrl+Shift+F` sobre el índice en memoria;
- índice del documento con `Ctrl+Shift+L`, derivado de los bloques ya abiertos
  y sin reparsear ni acceder a archivos;
- presupuestos de recorrido para una carpeta grande: 10.000 notas, 512 KiB por
  nota y 64 MiB de lectura acumulada;
- QA manual del aviso inicial y recuperación tras cierre inesperado;
- QA manual de modificación externa mientras la ventana pierde y recupera foco;
- cancelación de parsing y política de reemplazo si cambia el documento;
- virtualización y alturas todavía aproximadas;
- cobertura y fallback tipográfico todavía sin matriz completa;
- medición de memoria pendiente de repetir tras retener la fuente de sesión;
- workflow de CI publicado para Windows MSVC y Linux; falta registrar su
  primera evidencia remota y mantenerla verde;
- fuzzing y validación independiente del SBOM;
- QA manual de copia, incluidos atajos, aplicaciones destino y plataforma;
- selección parcial con mouse y autoscroll, flechas y Shift+flechas en ambas
  direcciones, Ctrl+A y Escape; la copia ya tiene pruebas unitarias.

## Evidencia disponible

- mediciones de Sprint 0 en [`budget.md`](budget.md);
- decisiones históricas en [`decisions.md`](decisions.md);
- amenaza y controles en [`threat-model.md`](threat-model.md) y
  [`security.md`](security.md);
- snapshots `claude-working-tree.diff` y
  `claude-working-tree-status.txt` en el working tree local;
- backup externo creado por el propietario;
- artifacts visuales recuperados localmente.

## Próximo criterio de salida

La recuperación inicial termina cuando:

- se completa la inspección visual y no aparecen regresiones;
- se valida el modo seguro end to end, no solo en el modelo;
- el modelo recuperado recibe revisión de round-trip antes de editar;
- la selección de estilos y fallback tipográfico supera QA visual;
- el diff puede revisarse en commits pequeños;
- no se perdió trabajo heredado sin decisión explícita.

Las comprobaciones automatizables de esa lista están cubiertas. El cierre sigue
abierto por QA visual y por la revisión del contrato de round-trip; no por un
working tree roto.
