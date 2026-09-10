# Catálogo de funciones

Este catálogo describe producto y estado sin confundir intención con
implementación.

Estados:

- `Estable`: presente en el último commit estable y medido.
- `Parcial`: existe una base estable, pero falta evidencia o corrección importante.
- `Recuperación`: existe trabajo local incompleto.
- `Planificado`: pertenece al plan activo, pero todavía no está implementado.
- `Futuro`: idea válida fuera del plan inmediato.
- `Descartado`: no pertenece al producto actual.

La evidencia detallada vive en [`status.md`](status.md) y
[`test-matrix.md`](test-matrix.md).

## Núcleo de lectura

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Apertura de `.md` por argumento | Estable | Muestra ventana y contenido |
| Parsing Markdown básico | Estable | Casos actuales del prototipo |
| Tema claro y oscuro | Parcial | Sistema y alternancia manual con transición nativa de 200 ms, sin mover contenido; la preferencia local de reducir movimiento la vuelve instantánea |
| Tipografía embebida | Parcial | Reproducción verificada; falta matriz Unicode y fallback |
| Virtualización inicial | Parcial | Debe eliminar recorridos O(n) por frame |
| Formato inline real | Parcial | Modelo, layout y dibujo nativos; falta corpus y QA visual sistemáticos |
| Listas, citas y reglas | Parcial | Casos anidados cubiertos; falta corpus y QA visual ampliados |
| Task list checkboxes | Parcial | Dibujo nativo, objetivo de clic ampliado sin invadir el texto, cursor de enlace, cambio reversible del marcador y re-render versionado de lectura; falta QA de plataforma |
| CommonMark aplicable | Parcial | Corpus versionado de sintaxis soportada y política HTML; falta suite oficial seleccionada |
| GFM elegido | Parcial | Tablas, tachado, tareas, autolinks y notas al pie; falta corpus sistemático |
| Copia de tablas | Parcial | Menú y paleta copian la tabla elegida como TSV desde celdas semánticas; la selección parcial visible y las flechas respetan celdas, omiten separadores y conservan columna al subir o bajar. Falta QA manual. |
| Índice de encabezados | Parcial | `Ctrl+Shift+L` muestra y enfoca encabezados; falta panel filtrable y accesible |
| Plegado de secciones | Parcial | Triángulo nativo en encabezados, estado por pestaña y fuente intacta; falta QA visual y accesibilidad ampliada |
| Búsqueda en documento | Parcial | `Ctrl+F` local, Unicode sin distinción de mayúsculas, recorrido por cada coincidencia, término activo resaltado y marcas globales por bloque; falta QA visual |
| Palabras y tiempo de lectura | Parcial | La barra cuenta texto visible del último render y estima minutos a 200 palabras; falta QA con idiomas sin separación por espacios |
| Vista previa de enlaces | Parcial | Mouse y teclado muestran tipo y destino declarado antes de abrir, sin resolverlo ni acceder a recursos; falta QA visual |
| Vista de texto segura | Parcial | Fallback por límites y HTML inerte; falta QA end to end completo |

## Edición y archivos

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Modo fuente | Parcial | Buffer Rope, selección, IME, undo y atajos; navegación vertical por caracteres Unicode y CRLF atómico para cursor, selección y borrado; falta actualización incremental de la vista fuente |
| Ayudas de formato | Parcial | Negrita, cursiva, enlace, encabezado, viñeta, tarea, cita, código, tabla, resaltado, wikilink y callout editan fuente Unicode como cambios reversibles desde atajo, menú, barra o paleta; faltan QA de descubribilidad y variantes contextuales |
| Vista dividida | Parcial | `F3` compara fuente editable y render de la misma revisión; falta QA de edición prolongada, DPI y accesibilidad |
| Modo por documento | Parcial | Recuerda lectura, edición o vista dividida mediante hasta 128 claves hash locales; falta QA de reinicio |
| Guardado atómico | Parcial | Sin corrupción ante fallo y conflictos probados; falta QA multiplataforma |
| Preservar sintaxis desconocida | Parcial | Fuente no se reserializa; falta property testing de round-trip |
| Detectar cambios externos | Parcial | Conflicto visible al guardar, recuperar foco o reactivar pestaña; respuestas viejas y recargas fallidas no reemplazan la edición local; falta QA multiplataforma |
| Recuperación de sesión | Parcial | Activa por defecto, separada por pestaña, versionada y desactivable con advertencia; falta QA de cierre inesperado |
| Crear documento | Parcial | Crea una pestaña nueva sin reemplazar el documento activo; Guardar abre el diálogo de destino incluso para un documento vacío; falta plantilla inicial |
| Varios documentos y pestañas | Parcial | Barra visible, cambio y cierre por mouse/teclado, `Ctrl+Tab`, `Ctrl+PageUp/PageDown`, pestañas fijables por sesión, estado, historial, scroll, selección, plegado, anclas y recuperación separados; aperturas, renders y guardados se dirigen por pestaña sin congelar las demás; falta accesibilidad completa |
| Menú contextual | Parcial | Copia y pegado explícito según modo, búsqueda, cambio de vista y guardado; faltan acciones de workspace y estados deshabilitados visibles |
| Paleta de comandos | Parcial | `Ctrl+Shift+P` muestra el catálogo compartido, filtra por nombre y ejecuta por teclado o mouse; falta accesibilidad semántica completa |
| Barra de acciones | Parcial | En lectura prioriza archivo, búsqueda y espacio de trabajo; un selector segmentado directo cambia entre Leer, Editar y Comparar sin recorrer estados. En edición muestra guardar y ayudas Markdown esenciales. Mouse y F6 comparten acciones; 640 × 480 lógicos evita controles ocultos; falta semántica de lector de pantalla |
| Chrome de ventana | Parcial | En Windows no hay borde del sistema: minimizar, maximizar o restaurar, cerrar, arrastrar y redimensionar usan `winit`; otras plataformas conservan controles nativos hasta completar QA accesible |
| Reducir movimiento | Parcial | Acción persistente local que vuelve instantáneo el cambio de tema; no guarda contenido, rutas ni permisos. Debe cubrir paneles, pestañas y futuras divisiones cuando tengan transición visual |
| Paneles plegables | Parcial | Índice, árbol plegable de notas, búsqueda de carpeta y backlinks muestran listas acotadas con selección visible, navegación por teclado o mouse y cierre con Escape; falta accesibilidad completa |
| Actualizar workspace | Parcial | El catálogo y `Ctrl+Shift+I` reconstruyen de forma explícita el índice cancelable cuando la carpeta cambió; falta invalidación más precisa por archivo |
| `.txt` y otros textos inertes | Parcial | Extensiones no Markdown se mantienen inertes al abrir, editar, comparar, actualizar y Guardar como; falta QA de reconocimiento UX |
| Edición en vivo | Futuro | Solo tras editor y modelo estables |

## Seguridad

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Sin WebView ni JavaScript | Estable | Grafo de dependencias y runtime |
| Sin red durante uso normal | Parcial | Falta prueba automatizada de sockets |
| Límites de anidamiento | Recuperación | Tiempo, memoria y fallback |
| VFS central | Parcial | Workspace contenido, archivo principal limitado, enlaces relativos e imágenes PNG resueltos desde su nota; faltan futuros recursos secundarios |
| Política de rutas | Parcial | UNC, traversal, symlinks y junctions cubiertos para workspace, navegación y PNG local; falta QA adversarial multiplataforma |
| Allowlist HTML semántica | Recuperación | `br`, `kbd`, `mark`, `sub` y `sup` nativos sin atributos; falta corpus sistemático y QA visual |
| Límites de imágenes | Parcial | PNG local: firma, 8 MiB, 8192 por lado y 16 millones de píxeles; falta corpus hostil ampliado |
| Consentimiento remoto | Planificado | Aislado, explícito y revocable |
| Confianza temporal de bóveda | Planificado | Solo amplía acceso local delimitado |
| Auditoría de dependencias | Parcial | Audit y SBOM reproducible realizados; faltan deny, notices y CI por target |

## Selección y accesibilidad

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Selección con mouse | Parcial | Selección visible con autoscroll y copia; falta QA visual completo |
| Selección con teclado | Parcial | Flechas, Ctrl+flechas por palabras, Inicio/Fin, Ctrl+Inicio/Fin, RePág/AvPág, Ctrl+A, Escape y extensiones con Shift; falta foco accesible y QA de plataforma |
| Copiar bloque o documento | Parcial | En lectura `Ctrl+C` copia texto visible y `Ctrl+Shift+C` copia Markdown de bloques completos; en edición copia la selección fuente exacta, incluso si la vista previa todavía se actualiza |
| Barra de estado | Parcial | Muestra modo, cambios sin guardar y estado de carpeta sin ocupar herramientas permanentes |
| Alto contraste | Planificado | Matriz Windows y Linux |
| Reduce motion | Parcial | La preferencia local ya elimina la transición de tema; debe cubrir toda transición futura |
| IME | Parcial | Commits de IME insertan en el buffer Unicode; falta QA multilingüe por plataforma |
| Unicode y fallback | Parcial | Corpus multilingüe y fallback de glifos presentes; falta QA visual sistemático |
| Lector de pantalla | Planificado | Semántica accesible demostrada |

## Trabajo con IA

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Copiar Markdown de un bloque | Parcial | `Ctrl+Shift+C` conserva la fuente de bloques completos |
| Fragmentar documento largo | Parcial | La paleta de estudio prepara una pestaña nueva con bloques completos bajo un límite orientativo; no altera el original ni corta una estructura para cumplirlo. Falta elegir, copiar o guardar cada fragmento desde un panel dedicado |
| Comparar versiones | Planificado | Diferencias legibles y no destructivas |
| Archivo listo para adjuntar | Planificado | Markdown portable |
| Copia para Discord o correo | Parcial | Desde lectura, el menú contextual y la paleta preparan bloques seleccionados sin abrir ni resolver enlaces; faltan variantes y QA con destinos reales |
| Estimación de tokens | Parcial | Aproximación local por caracteres visibles, sin tokenizer ni red; sirve para orientar, no para cotizar un proveedor |
| IA propia | Descartado | No pertenece a Visor MD |

## Obsidian y workspace

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Abrir carpeta o bóveda | Parcial | Sin migración ni cambios implícitos; al recuperar foco compara fuera de UI hasta 1.024 rutas ya indexadas y sugiere `Ctrl+Shift+I` si cambiaron |
| Lista de notas | Parcial | Panel plegable con árbol del índice; abre solo tras resolver dentro de VFS y señala cambios externos limitados |
| Búsqueda de bóveda | Parcial | Panel con resultados múltiples del índice en memoria; falta actualización incremental |
| Wikilinks | Parcial | Resolución contenida y explícita; el índice ignora wikilinks literales en código o escapados y el panel de diagnóstico distingue rutas absolutas, UNC, `file:` o traversal bloqueadas de notas ausentes |
| Backlinks | Parcial | Panel plegable muestra y navega backlinks contenidos; falta incrementalidad |
| Callouts | Parcial | Sintaxis Obsidian elegida y render nativo; la bóveda fixture cubre aliases, callouts, duplicados, código literal y rutas hostiles; falta corpus de compatibilidad ampliado |
| Etiquetas y frontmatter | Planificado | Lectura sin reescritura |
| Link peek | Planificado | Reusa renderer con límites |
| Referencias de bloque | Futuro | Requiere identidad estable |
| Grafo visual | Futuro | Solo con valor demostrado |

## Estudio

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Resaltado `==texto==` | Parcial | Render nativo y fuente preservada; falta corpus de compatibilidad y QA visual |
| Sidecar para datos no portables | Planificado | Formato versionado y recuperable |
| Preguntas y respuestas | Parcial | Inserta un callout `[!QUESTION]-` portable y permite plegar su respuesta solo en lectura; falta QA de accesibilidad y corpus de compatibilidad. |
| Ocultar respuesta | Parcial | Las respuestas de preguntas se pliegan en lectura sin alterar la fuente; faltan variantes de práctica y QA accesible. |
| Estados de aprendizaje | Parcial | La paleta inserta callouts portables `[!SUCCESS]`, `[!WARNING]` y `[!TODO]` para entendido, duda y pendiente; falta edición contextual de estados ya existentes y QA visual. |
| Resúmenes | Parcial | La paleta inserta una estructura Markdown portable o prepara desde una selección una pestaña derivada editable con plantilla y fuente literal; no resume automáticamente ni altera el original. Falta exportación dedicada. |
| Lista de conceptos | Parcial | Inserta una estructura Markdown o prepara desde una selección una pestaña editable con plantilla y fuente literal. Conserva EOL, es reutilizable en Obsidian, Git y otros lectores, y no extrae conceptos automáticamente. Falta exportación dedicada. |
| Repaso espaciado simple | Planificado tardío | Estado separado y sincronización segura |
| Pomodoro | Futuro | Solo si no distrae ni infla producto |

## Contenido enriquecido

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Imágenes locales | Parcial | Placeholder, confirmación por PNG, VFS relativa a la nota y límites de bytes, dimensiones y memoria; falta presentación inline |
| Imágenes remotas confirmadas | Planificado | Componente de red aislado |
| Copiar bloque de código | Parcial | Acción visible y explícita; falta QA visual y de plataformas destino |
| Resaltado de código | Planificado | Sin runtime o dependencia desproporcionada |
| Matemática | Futuro | Componente opcional seguro |
| Mermaid nativo | Futuro | Sin servicio remoto ni scripts |
| HTML arbitrario | Descartado | Se muestra inerte |

## Exportación y distribución

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| PDF fiel | Planificado | Tipografía, paginado y seguridad |
| DOCX | Planificado con investigación | Compatibilidad y coste cerrados |
| Copia para plataformas | Parcial | Desde lectura, el menú contextual y la paleta copian Markdown de bloques seleccionados y vuelven legibles los wikilinks de Obsidian para Discord o correo; faltan variantes de formato y QA con destinos reales. |
| HTML autónomo | Futuro | Sin recursos remotos ni scripts |
| Instalador Windows | Planificado | Firma, asociación y desinstalación |
| Paquete Linux | Planificado | Integración de escritorio y licencias |
| Actualizaciones | Futuro | Explícitas, firmadas y sin telemetría |

## Reglas contra crecimiento accidental

Una función no entra solo porque un competidor la tenga. Debe:

1. resolver un caso de uso aprobado;
2. respetar el threat model;
3. tener criterio de finalización;
4. justificar dependencias y tamaño;
5. tener pruebas proporcionales al riesgo;
6. encajar en la interfaz sin volverla sobrecargada.
