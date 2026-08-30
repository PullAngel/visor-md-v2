# Roadmap

Última revisión: 28 de agosto de 2026.

El roadmap ordena dependencias y criterios de salida. No promete fechas. Una
etapa solo se cierra con evidencia y el producto debe quedar en un punto útil al
final de cada hito.

## Reglas

- `main` representa el desarrollo principal actual y debe mantenerse compilable,
  entendible y recuperable.
- Las ramas adicionales se usan cuando aportan aislamiento real; no son un
  requisito para cada bloque de trabajo.
- La rama histórica de respaldo del estado anterior se conserva intacta como
  referencia y no se utiliza para desarrollo.
- No se inicia una etapa sobre una base rota.
- Seguridad, accesibilidad y documentación acompañan cada etapa.
- Una función reconocida por el parser no está terminada hasta llegar a UX y
  tests.
- Todo cambio de dependencia mide tamaño y superficie.
- El alcance de v2.0 puede entregarse mediante previews sin llamarlas estables.
- Un commit, checkpoint, suite verde, bloque técnico, mini-sprint o actualización
  documental no cierra la solicitud global ni inicia una espera. Mientras queden
  tareas automatizables, seguras y aprobadas, se continúa inmediatamente con el
  siguiente bloque coherente; pueden acumularse varios antes de un cierre.
- Un gate manual o QA visual pendiente se registra y debe cerrarse antes de
  declarar terminado su sprint o milestone, pero solo bloquea trabajo que
  realmente dependa de esa evidencia; no bloquea trabajo independiente.
- Un turno solo termina cuando el objetivo global está terminado, todo lo
  restante depende de una persona, continuar requiere un cambio no aprobado de
  alcance/arquitectura/seguridad/producto, o existe riesgo de pérdida de datos
  o un bloqueo técnico externo.

## Estado

| Etapa | Estado | Resultado |
| --- | --- | --- |
| Sprint 0 | Cerrado con deuda documental | Prototipo nativo medido |
| Recuperación | En cierre | Código preservado y estable; falta QA visual |
| Sprint 1 | Parcial | Lector mínimo profesional |
| Validación base | Iniciada | Gates Windows, auditoría, SBOM y benchmarks |
| Lector completo | Pendiente | Markdown cotidiano completo |
| Editor básico | En curso | Abrir, crear, edición fuente, guardado atómico, conflictos explícitos y recuperación local configurable; faltan buffer escalable y QA manual de los diálogos nativos |
| Chrome | En curso | Pestañas, acciones esenciales y paneles visibles con cierre protegido; faltan accesibilidad completa y ventana sin borde |
| Workspace | En curso | Carpeta explícita, VFS, índice acotado/cancelable y búsqueda por teclado; faltan árbol/panel y cambios externos |
| Obsidian | En curso | Wikilinks, callouts y navegación inicial de backlinks contenidos; faltan panel visible y actualización incremental |
| Estudio | En curso | Resaltado portable inicial; faltan herramientas portables |
| Exportación | Pendiente | PDF, DOCX y copias preparadas |
| Distribución | Pendiente | Windows y Linux profesionales |

## Plan de sprints ejecutables

Los sprints ordenan entregables pequeños y verificables. No representan fechas
ni autorizan adelantar una dependencia de seguridad para mostrar una función.
La siguiente unidad se inicia sobre una base técnicamente verde. Los criterios
humanos pendientes de una unidad anterior permanecen registrados y se cierran
antes de declarar su sprint o milestone terminado; no paralizan trabajo seguro
e independiente.

| Sprint | Objetivo | Criterio verificable de cierre |
| --- | --- | --- |
| 1A | Semántica segura de lectura | Inline, bloques, allowlist HTML cerrada, límites y modo seguro probados desde parser hasta dibujo. |
| 1B | Interacción de lectura | Selección, copia, navegación por teclado, foco y menú contextual inicial sin alterar archivos. |
| 1C | Robustez del lector | Corpus CommonMark y GFM acordado, límites de recursos, resize, DPI, Unicode y rendimiento con evidencia. |
| 2A | Frontera de archivos | VFS, tipo de texto inerte, límites de apertura y política de rutas demostrables antes de cargar recursos secundarios. |
| 2B | Editor y guardado fiel | Fuente, vista relacionada, undo, codificaciones, guardado atómico, conflictos y round-trip sin reescrituras. |
| 3 | Chrome de aplicación | Ventana sin borde, pestañas, comandos, menú y estados de documento sin pérdida de datos. |
| 4 | Workspace seguro | Carpetas, índice, búsqueda y cambios externos dentro de una política de VFS y confianza limitada. |
| 5 | Compatibilidad Obsidian | Wikilinks, backlinks, callouts y rutas relativas sin migrar ni reescribir bóvedas. |
| 6 | Estudio y flujos para IA | Herramientas Markdown portables, fragmentación, comparación y preparación de copias sin IA embebida. |
| 7 | Exportación aislada | PDF fiel, DOCX útil y copias de plataforma sin alterar el documento ni ampliar el núcleo innecesariamente. |
| 8 | Distribución verificable | CI, paquetes Windows y Linux, SBOM, notices, benchmarks y matriz de release. |

## Sprints largos de ejecución

Estos sprints agrupan las unidades anteriores para que cada período de trabajo
termine en un producto más útil, no solo en una lista de refactors. Sus gates
técnicos no se saltan: si uno falla, se corrige antes de depender de esa
propiedad. Los gates humanos pendientes quedan registrados y no paralizan otros
bloques independientes.

| Sprint largo | Alcance agrupado | Resultado para usar | Gate de salida |
| --- | --- | --- | --- |
| A. Lector seguro y usable | Recuperación, 1A, 1B y 1C | Abrir Markdown y texto inerte rápido, leerlo con formato, seleccionar, copiar, navegar y abrir enlaces externos explícitos sin que el documento obtenga permisos | Modelo y renderer completos para la sintaxis anunciada; fallback visible; corpus y patologías verdes; QA visual pendiente registrado; release menor de 8 MB |
| B. Documento fiel y editor básico | 2A y 2B | Abrir, crear, editar fuente, comparar vista y guardar sin reescrituras silenciosas | VFS, rangos de fuente fiables, undo/redo, encoding/EOL, guardado atómico y conflictos probados en Windows y Linux |
| C. Aplicación de trabajo | 3 y 4 | Pestañas, comandos, carpeta de trabajo, índice y búsqueda seguros | Chrome accesible; cierre sin pérdida; índice limitado, cancelable y contenido dentro de VFS; benchmark de carpeta grande |
| D. Buen ciudadano de Obsidian y estudio | 5 y 6 | Navegar bóvedas existentes, usar wikilinks/backlinks/callouts y preparar estudio o contenido para IA | No migración ni ruido Git; enlaces contenidos; herramientas de estudio portables o sidecars versionados; cero IA embebida |
| E. Salida profesional | 7 y 8 | Exportar PDF/DOCX/copia de plataforma y distribuir para Windows/Linux | Exportación aislada; CI y paquetes reproducibles; SBOM, licencias, benchmarks, threat model y matriz de release completos |

### Sprint largo activo: A. Lector seguro y usable

Se considera en cierre técnico, no terminado: parser, modelo, render, límites,
selección, copia, menú contextual y apertura externa explícita están presentes.
Quedan QA visual, DPI/Unicode y evidencia de rendimiento actualizada para su
cierre formal. El corpus, modo seguro y contrato de edición source-first ya
permiten avanzar en el Sprint B sin usar rangos ambiguos de enlaces como
autoridad de escritura.

### Orden inmediato

El trabajo activo mantiene abiertos los gates humanos del Sprint A mientras
construye la aplicación diaria en este orden: documento fiel primero; luego
pestañas, acciones y cierre protegido; después paneles de carpeta y búsqueda;
y finalmente la compatibilidad esencial de Obsidian y el pulido de lectura. La
validación visual de tipografía y allowlist HTML se cierra antes de declarar A
terminado, pero no paraliza estas tareas independientes. Los refactors internos
solo se adelantan cuando desbloquean una de esas capacidades o reducen un riesgo
concreto de integridad, seguridad o estabilidad.

## Sprint 0: viabilidad nativa

### Objetivo

Demostrar que Rust y una pila de dibujo nativa podían cumplir tamaño, apertura y
scroll sin WebView.

### Resultado real

- ventana y superficie por software;
- parsing y layout básicos;
- virtualización inicial;
- mediciones con documento normal y grande;
- tema y fuentes incorporados en el último commit estable;
- binario ampliamente debajo del límite actual.

### Deuda que no se oculta

- reproducibilidad tipográfica incompleta en aquel checkpoint, cerrada durante
  la recuperación;
- Linux no medido con la misma profundidad;
- auditoría de `unsafe` y C parcial;
- fuente y toolchain no fijados completamente;
- accesibilidad no demostrada.

Sprint 0 no se reabre, pero esa deuda entra en Validación base.

## Etapa 1: recuperación del working tree

### Objetivo

Preservar lo valioso del trabajo interrumpido, eliminar estados parciales y
establecer una base compilable que no bloquee edición futura.

### Trabajo

1. Registrar baseline, hashes, toolchain y diff.
2. Clasificar temas, fuentes, inline, bloques, límites, markers y tests.
3. Recuperar compilación para caracterizar comportamiento.
4. Definir el contrato mínimo del modelo documental con rangos y semántica.
5. Trasladar las funciones válidas al modelo correcto.
6. Cerrar proceso, licencias y cobertura de fuentes.
7. Separar commits revisables sin integrar snapshots.

### Criterios de salida

- source actual compilable;
- tests reconstruidos y verdes;
- `Marker` conectado de parser a dibujo;
- casillas verificadas en parsing, layout y rendering;
- límite de anidamiento y fallback probados;
- modelo preparado para selección y edición;
- fuentes reproducibles y licenciadas;
- ningún cambio heredado perdido sin decisión registrada;
- diff dividido en commits coherentes.

### Estado verificable

La parte automatizable de la recuperación está cerrada: compila, tiene 40 tests,
fallback completo, modelo con rangos, fuentes reproducibles, commits separados y
release medido. La inspección visual y tipográfica continúa pendiente. El
contrato de round-trip debe revisarse antes del editor, no simularse mediante un
guardado que todavía no existe.

## Etapa 2: cierre de Sprint 1

### Objetivo

Entregar un lector mínimo profesional, seguro y usable que sirva de base al
editor.

### Trabajo

- separar módulos mínimos;
- modelo documental con rangos;
- CommonMark aplicable;
- extensiones GFM ya aprobadas y completas;
- allowlist HTML semántica;
- límites y vista segura;
- parsing fuera del camino crítico de UI;
- layout, alturas, scroll, resize y DPI correctos;
- Unicode y fallback;
- selección, copia y navegación por teclado;
- menú contextual inicial;
- accesibilidad mínima demostrada;
- diseño Papel y tinta preservado;
- casos valiosos portados desde v1.

### Criterios de salida

- corpus CommonMark acordado verde;
- cada sintaxis declarada llega a rendering;
- enlaces y bloques conservan semántica y origen;
- entradas patológicas no bloquean ni desbordan;
- modo seguro visible y útil;
- apertura normal sin red ni acceso secundario arbitrario;
- selección y copia utilizables;
- resize, zoom y DPI sin corrupción;
- trabajo por frame principalmente proporcional a contenido visible;
- release medido contra menos de 8 MB;
- documentación y matriz sincronizadas.

## Etapa 3: validación base

### Objetivo

Convertir seguridad, calidad y rendimiento en propiedades demostrables antes de
ampliar el producto.

### Trabajo

- CI Windows MSVC y Linux;
- format, clippy, tests y release;
- CommonMark oficial y corpus del proyecto;
- fuzzing de parser, modelo y rutas;
- property tests;
- VFS, UNC, traversal, symlinks y junctions;
- monitor de sockets;
- benchmarks versionados;
- auditoría de dependencias, licencias y `unsafe`;
- SBOM;
- matriz visual y de accesibilidad;
- threat model y ADR actualizados.

### Criterios de salida

- gates verdes en Windows y Linux;
- cero red durante apertura y render normales;
- contención de archivos demostrada;
- campaña de fuzzing sin hallazgos abiertos críticos;
- SBOM reproducible;
- advisories resueltos o aceptados explícitamente;
- benchmarks repetibles con umbrales;
- código, tests y documentación alineados.

## Etapa 4: lector completo

### Alcance

- tablas;
- footnotes y autolinks;
- enlaces y destinos visibles;
- imágenes locales seguras;
- bloques de código y resaltado medido;
- índice filtrable;
- búsqueda con marcas de scroll;
- plegado de secciones;
- contador de palabras y lectura;
- copiar tablas como TSV;
- link peek seguro.

### Criterios de salida

- documentos GitHub y Obsidian elegidos se leen sin pérdida importante;
- imágenes pasan VFS y límites;
- tablas son seleccionables y copiables;
- navegación por teclado completa;
- documentos grandes mantienen presupuesto;
- toda sintaxis soportada tiene corpus y UX de error.

## Etapa 5: editor básico y guardado

Esta etapa se adelanta respecto del roadmap histórico. Guardar fielmente condiciona
workspace, anotaciones y Obsidian, por lo que descubrirlo tarde sería costoso.

### Alcance

- editor fuente;
- vista dividida;
- mapeo fuente y render;
- undo y redo;
- ayudas Markdown discretas;
- crear, abrir, guardar y guardar como;
- guardado atómico;
- cambios externos y conflictos;
- EOL, BOM y UTF-8;
- preservación de sintaxis desconocida;
- recuperación ante cierre inesperado.

### Criterios de salida

- round-trip probado con contenido conocido y desconocido;
- ningún guardado parcial ante fallo simulado;
- IME, selección y portapapeles verificados;
- edición de documentos grandes permanece responsiva;
- Obsidian y GitHub releen los archivos de prueba sin cambios inesperados.

## Etapa 6: chrome, pestañas y comandos

### Alcance

- ventana sin borde;
- pestañas, fijado y estado sucio;
- varios documentos;
- menú principal y contextual;
- paleta de comandos;
- paneles progresivos;
- búsqueda global de comandos;
- siempre encima si supera QA de plataforma;
- animaciones y reduce motion.

### Criterios de salida

- acciones esenciales descubribles sin atajos;
- profundidad disponible sin sobrecarga visual;
- cierre nunca pierde cambios;
- teclado y lector de pantalla cubren comandos;
- artifacts aprobados usados como referencia visual.

## Etapa 7: workspace

### Alcance

- abrir carpeta;
- árbol de archivos;
- índice incremental;
- búsqueda en carpeta;
- navegación rápida;
- papelera recuperable;
- confianza temporal delimitada;
- detección de cambios externos.

### Criterios de salida

- una carpeta grande no bloquea UI;
- índice respeta VFS, ignores y límites;
- borrar es recuperable;
- confiar no habilita red o ejecución;
- cambios externos se reflejan de manera predecible.

## Etapa 8: Obsidian y GitHub

### Alcance

- wikilinks;
- backlinks;
- callouts;
- etiquetas y frontmatter en lectura;
- referencias existentes a encabezados o bloques;
- rutas relativas compatibles;
- comportamiento Git amistoso, sin reescrituras masivas.

### Criterios de salida

- bóvedas de prueba abren sin migración;
- backlinks se actualizan incrementalmente;
- enlaces no escapan de la política;
- guardar no genera ruido innecesario en Git;
- sintaxis no soportada se preserva.

## Etapa 9: estudio y trabajo con IA

### Alcance

- resaltado portable;
- sidecars solo para estado no portable;
- preguntas y respuestas;
- ocultar respuestas;
- estados de aprendizaje;
- listas de conceptos;
- repaso espaciado simple;
- copiar Markdown de un bloque;
- fragmentar documentos;
- comparar versiones;
- preparar adjuntos y copias.

### Criterios de salida

- Obsidian interpreta las anotaciones portables;
- sidecars tienen versión, recuperación y sincronización;
- fragmentar no rompe bloques estructurales;
- comparar no modifica archivos;
- no hay modelo de IA ni conexión implícita.

## Etapa 10: exportación

### Prioridad

1. PDF visualmente fiel.
2. DOCX compatible con universidad y trabajo.
3. Copia preparada para plataformas.

### Criterios de salida

- exportar contenido hostil no ejecuta recursos;
- PDF conserva tipografía, enlaces y paginado acordados;
- estrategia DOCX tiene compatibilidad y coste medidos;
- componentes pesados quedan aislados cuando conviene;
- exportación no altera el documento fuente.

## Etapa 11: distribución

### Windows

- MSVC;
- instalador y desinstalador;
- asociación de archivos;
- firma cuando sea viable;
- integración de tema y accesibilidad;
- paquete, SBOM y notices.

### Linux

- build y paquetes elegidos;
- asociación MIME;
- fontconfig y dependencias nativas documentadas;
- Wayland y X11 según soporte real;
- accesibilidad y DPI;
- paquete, SBOM y notices.

### Criterios de salida v2.0

- recorridos principales estables en Windows y Linux;
- instaladores reproducibles;
- datos del usuario protegidos;
- threat model revisado;
- matriz de release completa;
- tamaño, memoria y arranque publicados;
- riesgos residuales documentados;
- aprobación manual del producto.

## Después de v2.0

Solo se evalúan con evidencia de uso:

- edición en vivo;
- matemática opcional;
- Mermaid nativo;
- corrector descargable;
- grafo;
- plugins declarativos sin código arbitrario;
- macOS;
- funciones de estudio adicionales.

Ver [`future.md`](future.md).
