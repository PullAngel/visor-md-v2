# Arquitectura

## Estado de este documento

Describe la arquitectura objetivo y separa explícitamente lo que existe de lo
que falta. El prototipo de Sprint 0 demostró la viabilidad de la pila gráfica,
pero la mayoría de las capas de seguridad y edición todavía no están
implementadas.

Ver [`status.md`](status.md) para la fotografía actual.

## Estructura implementada durante la recuperación

La separación empezó de forma incremental y con la suite verde:

- `src/main.rs`: aplicación, parser y modelo provisionales, layout y dibujo;
- `src/fonts.rs`: familias embebidas y registro tipográfico;
- `src/limits.rs`: límites defensivos y causas de degradación;
- `src/theme.rs`: paletas Papel y tinta y roles de color.

El prototipo retiene durante la sesión el texto UTF-8 que abrió, sus rangos de
modelo y metadatos de entrada: presencia de BOM UTF-8 y estilo observado de EOL
(`LF`, `CRLF` o mixto). El BOM no se presenta como contenido y los saltos no se
normalizan. Aún falta la identidad del archivo, la codificación fuera de UTF-8,
los parches y el guardado atómico antes de declarar edición fiel.

Parser, modelo, layout y aplicación todavía comparten `main.rs`. Se extraerán en
commits separados; esta lista describe el estado real y no la arquitectura final.

## Principios

1. Sin motor de scripts o DOM.
2. Una sola política para acceso secundario a archivos.
3. Un modelo documental que preserve fuente y semántica.
4. Todo nodo se valida antes de convertirse en dibujo o interacción.
5. Apertura y render normales sin red.
6. Trabajo pesado cancelable fuera del hilo de UI.
7. Trabajo por frame proporcional al contenido visible.
8. Componentes pesados o con red aislados del núcleo.
9. Windows y Linux considerados desde cada contrato.
10. Arquitectura demostrada por tests y mediciones.

## Pila validada

| Crate | Responsabilidad | Estado |
| --- | --- | --- |
| `winit` | Ventana, eventos y abstracción de plataforma | Validado en Sprint 0 |
| `softbuffer` | Presentación del framebuffer | Validado en Sprint 0 |
| `tiny-skia` | Dibujo 2D por software | Validado en Sprint 0 |
| `parley` | Layout de texto | Validado con riesgo de API pre-1.0 |
| `swash` | Fuentes y rasterización | Validado en Sprint 0 |
| `comrak` | CommonMark y extensiones GFM | Elegido, cobertura incompleta |

La pila sigue sujeta a revisión si no puede cumplir selección, IME,
accesibilidad, Unicode o edición profesional. La prueba de tamaño no convierte
cada decisión del prototipo en definitiva.

## Flujo de datos objetivo

```text
Entrada explícita
      |
      v
File policy y VFS
      |
      v
Bytes y metadatos de origen
      |
      v
Decodificación y límites
      |
      v
Parser cancelable
      |
      v
Modelo documental con rangos
      |
      v
Validador de capacidades
      |
      +--------------------+
      |                    |
      v                    v
Layout visible         Editor y comandos
      |                    |
      v                    v
Display list          Parches de fuente
      |                    |
      v                    v
Renderer              Guardado atómico
```

Workspace, anotaciones y exportadores consumen contratos públicos. No leen el
disco o inspeccionan estructuras internas por atajos.

## Capas

### Entrada y file policy

Clasifica cómo llegó el archivo:

- diálogo o explorador elegido por el usuario;
- argumento de línea de comandos o asociación;
- enlace explícito dentro de la app;
- referencia secundaria producida por contenido.

La intención y los permisos no son iguales en todos los casos. Un documento
principal elegido puede vivir en una ruta UNC. Una referencia secundaria no
obtiene ese permiso automáticamente.

### VFS

VFS significa sistema de archivos virtual. En este proyecto no pretende inventar
otro disco. Es una puerta única que aplica política antes de cada acceso.

Responsabilidades:

- canonicalizar;
- comprobar contención;
- distinguir archivo principal y recurso secundario;
- controlar UNC, rutas de dispositivo y streams alternativos;
- manejar symlinks y junctions;
- aplicar tamaño y tipo;
- producir errores explicables;
- registrar evidencia para tests sin almacenar contenido privado.

El parser, renderer, índice y exportadores no abren rutas por su cuenta.

### Decodificación

La apertura actual admite UTF-8 válido, detecta y retira solamente el BOM UTF-8
para que no aparezca en pantalla, y conserva su presencia junto con el patrón
de EOL. La reconstrucción de prueba devuelve los mismos bytes si no hubo
edición. Secuencias UTF-8 inválidas se rechazan antes del parser.

La apertura también conserva una huella portátil de tamaño y fecha de
modificación de la versión leída. La capa de guardado la comparará antes de
reemplazar el destino; todavía falta una identidad de handle específica de cada
plataforma para reemplazo atómico. Otros encodings se admitirán solo si pueden
preservarse sin sustitución silenciosa. No se añade una conversión de
codificación por comodidad del renderer.

### Parser

`comrak` produce un AST. El parsing debe ocurrir en una tarea cancelable y no
bloquear la ventana. Límites de bytes, nodos, profundidad y tiempo se aplican
antes y durante la conversión.

Un AST de biblioteca no es el modelo completo de la aplicación. Puede cambiar
entre versiones y no siempre conserva la información necesaria para round-trip.

### Modelo documental

Representación canónica propia que conserva:

- rangos en la fuente;
- estructura padre e hijo;
- semántica de bloques e inline;
- enlaces y destinos;
- idioma de código;
- tablas y alineación;
- task markers;
- IDs de encabezados y bloques cuando existan;
- sintaxis desconocida;
- diagnósticos;
- límites y degradaciones.

No aplana el contenido destructivamente. El layout y el renderer reciben vistas
derivadas.

### Validador de capacidades

Decide qué puede convertirse en comportamiento visible:

- nodos Markdown conocidos;
- allowlist HTML semántica;
- enlaces que solo responden a acción explícita;
- imágenes que pasaron política;
- sintaxis desconocida como texto inerte.

No existe una opción para saltarse esta capa.

### Layout

Construye geometría de texto y bloques para un ancho, DPI, zoom y tema. Debe
producir información suficiente para:

- dibujo;
- selección;
- hit testing;
- navegación de teclado;
- correspondencia con fuente;
- accesibilidad;
- cálculo correcto de scroll.

Las estimaciones de altura permiten respuesta temprana, pero deben corregirse al
obtener medidas reales. El error no puede acumularse indefinidamente.

### Display list y renderer

La display list es una lista limitada de comandos de dibujo ya validados. El
renderer no interpreta Markdown, HTML, rutas o URLs.

`tiny-skia` pinta esos comandos y `softbuffer` presenta el resultado. El cache de
glifos evita rasterizar cada carácter en cada frame.

### Interacción y accesibilidad

Hit testing, selección, foco, teclado, IME y semántica accesible comparten el
mismo layout. No deben reconstruir una geometría paralela que pueda divergir de
lo que el usuario ve.

Si la superficie dibujada a mano no puede exponer accesibilidad suficiente, se
revisa el enfoque antes de continuar el editor.

### Editor

Opera sobre rangos y genera parches controlados. No serializa de nuevo el AST
completo para cambios pequeños. Esto permite preservar sintaxis desconocida.

La primera implementación es **source-first**: el buffer contiene el Markdown
UTF-8 original y la vista renderizada es una derivación reemplazable. Los rangos
del parser sirven al lector, pero una edición no depende de que una biblioteca
informe la extensión exacta de toda sintaxis inline. Esto evita usar, por
ejemplo, el rango del destino de un enlace como si cubriera también sus
corchetes y paréntesis.

`editor::EditHistory` ya define parches reversibles de bytes UTF-8, undo/redo,
revisiones y un presupuesto de 4 MiB para historial. `editor::SourceEditor`
añade cursor y selección sin partir caracteres Unicode. No conservan snapshots
completos de un documento de hasta 16 MiB. La interfaz de edición, IME, guardado
y recuperación ya consumen esta capa. Su límite actual es que el buffer fuente
sigue siendo una `String`: la migración a un buffer escalable se hará como un
refactor documentado, sin cambiar el formato ni la semántica de guardado.

`DocumentState` concentra la identidad, fuente, metadatos de preservación,
editor, modo, bloques renderizables y degradación de cada documento. La
aplicación conserva varios estados documentales y rota con ellos una sesión de
recuperación independiente dentro de una misma estructura, por lo que fuente y
recuperación no pueden desalinearse. Cada estado conserva también su posición
de scroll. La caché de layout y render sigue perteneciendo a
la ventana: se descarta o reconstruye al cambiar de documento, en vez de
confundirse con datos persistentes. El cambio de documento invalida resultados
de render anteriores mediante una generación monotónica. Falta trasladar el
resto del estado visual, como foco y selección de lectura, a cada pestaña y
reemplazar el selector temporal por una barra visible.

Responsabilidades:

- cursor y selección;
- IME;
- undo y redo;
- comandos Markdown;
- relación fuente y vista;
- estado sucio;
- detección de conflicto.

### Guardado

Es una operación separada y auditable:

1. validar destino e identidad;
2. construir bytes de salida;
3. escribir archivo temporal en el destino permitido;
4. sincronizar cuando corresponda;
5. reemplazar de forma atómica;
6. conservar o informar permisos y errores;
7. actualizar identidad y estado sucio.

No hay autoguardado por defecto. La recuperación de sesión usa almacenamiento
separado y nunca se presenta como guardado definitivo.

La recuperación local actual escribe snapshots limitados y separados por
pestaña en el perfil del usuario, fuera de documentos y bóvedas. El cierre de
la ventana no continúa si no puede preservar todos los documentos modificados.
Las escrituras se serializan y versionan: una tarea anterior no puede reemplazar
una recuperación más nueva ni recrearla después de un guardado que la eliminó.
La restauración es una acción explícita que crea un documento sin destino y marcado como modificado.
La primera sesión muestra un aviso claro de que esa copia es texto sin cifrar;
las sesiones propias de más de catorce días se eliminan de forma limitada. Falta
configuración persistente y QA de cierre inesperado.

### Chrome y comandos

Ventana, pestañas, menús, paneles y paleta de comandos consumen un catálogo de
acciones. Una acción debe poder invocarse desde mouse o teclado y declarar cuándo
está disponible. El catálogo inicial ya unifica las acciones esenciales entre
los atajos del lector, los atajos del editor y `Ctrl+Shift+P`; la disponibilidad
contextual y los controles de mouse todavía se completan de forma incremental.

### Workspace e índice

La primera implementación usa un índice regenerable en memoria, sin SQLite ni
sidecar de contenido. Ya existe un recorrido acotado que canoniza cada entrada,
omite `.git` y `.obsidian`, descarta escapes y extrae títulos, encabezados y
wikilinks de Markdown UTF-8 permitido. La selección de carpeta y el indexado ya
corren fuera de la UI; los enlaces relativos solo se abren tras resolverlos con
la raíz autorizada. Los wikilinks `[[nota]]`, `[[nota|alias]]`,
`[[nota#encabezado]]` y `[[#encabezado]]` se representan como enlaces nativos:
el índice exige una coincidencia única y rechaza ambigüedades en vez de elegir
por orden. Una ancla igual a un encabezado visible se aplica después de abrir la
nota. El recorrido del índice se cancela cooperativamente al elegir otra carpeta
y su versión evita publicar resultados tardíos. `Ctrl+Shift+F` consulta el
índice local y solo abre la nota elegida después de resolver su ruta con la VFS.
`Ctrl+Shift+I` vuelve a crear el índice de la raíz ya concedida y cancela el
recorrido anterior si aún estaba activo. `Ctrl+Shift+B` presenta los backlinks
del documento actual y solo abre una selección tras resolverla de nuevo con la
VFS. Todavía faltan paneles plegables y detección de cambios externos del
workspace.

Los callouts conocidos de Obsidian dentro de una cita (`NOTE`, `INFO`, `TIP`,
`WARNING`, `CAUTION`, `DANGER` e `IMPORTANT`) reciben una presentación nativa
sin ejecutar HTML ni interpretar atributos. Variantes desconocidas permanecen
como citas con su marcador fuente visible; editar y guardar conserva la sintaxis
original.

Las aperturas y los resultados de vista llevan una versión de solicitud y de
revisión. Si una tarea termina tarde después de una apertura o edición posterior,
tanto su resultado como su error se descartan en vez de reemplazar el documento
activo o borrar un aviso actual.

Indexa incrementalmente una carpeta permitida:

- archivos y metadatos;
- encabezados;
- wikilinks;
- backlinks y términos de búsqueda para uso futuro de la UI.

El recorrido se limita a 10.000 notas Markdown, 512 KiB por nota y 64 MiB de
lectura acumulada. La búsqueda retiene como máximo 8 MiB de fragmentos de
fuente, con un tope de 8 KiB por nota y corte en límite Unicode válido. Al
alcanzar un presupuesto, la UI lo informa; no se intercambia disco o memoria
ilimitados por una coincidencia adicional.

El índice es regenerable y nunca la única copia de información del usuario. No
se persistirá contenido de bóveda mientras no haya una necesidad medida que
justifique tamaño, corrupción potencial y nueva superficie de privacidad.

### Anotaciones

La sintaxis portable vive en el `.md`. Estado no portable, como programación de
repasos, puede usar sidecar versionado. Un sidecar debe detectar que la fuente
cambió y degradar sin aplicar anotaciones al texto equivocado.

### Exportadores

PDF, DOCX y otros exportadores reciben modelo validado. No vuelven a parsear HTML
arbitrario ni cargan recursos ignorando VFS. Un exportador pesado puede vivir en
un binario o componente separado.

## Concurrencia y cancelación

El hilo de UI procesa eventos y presenta frames. Parsing, indexado, búsqueda,
exportación y otras tareas proporcionales al documento corren fuera de él.

Cada tarea lleva:

- identificador de documento y revisión;
- token de cancelación;
- presupuesto;
- resultado inmutable o mensaje de error.

Un resultado antiguo no reemplaza un documento más nuevo. Cerrar una pestaña o
editar mientras se parsea cancela o invalida trabajo anterior. El workspace no
instala un watcher: al recuperar foco compara una marca barata de su raíz con
la del último índice y, si cambió, avisa para reindexar explícitamente. Es una
señal de actualización, no una garantía de detección de cada cambio interno.

## Virtualización

Sprint 0 mostró que el coste importante no era pintar rectángulos, sino conservar
layouts, medir todos los bloques y rasterizar glifos repetidamente.

Reglas:

- índice de bloques y alturas compacto;
- búsqueda de rango visible mejor que recorrido lineal completo;
- layouts detallados solo cerca del viewport;
- cache con presupuesto y expulsión;
- estimaciones corregidas por medidas reales;
- prefetch pequeño y cancelable;
- nada proporcional a todo el documento por frame.

## Procesos y componentes opcionales

El núcleo puede ser un solo proceso sin intérprete. Capacidades con red,
decodificadores complejos o exportadores pesados deben evaluarse para aislamiento
en proceso separado con mensajes limitados.

No hay componente de IA previsto. Mermaid o matemática futuros no pueden
introducir scripts o servicios remotos.

## Plataformas

Windows y Linux son objetivos de v2.0. Lo específico vive detrás de contratos de
plataforma:

- asociaciones de archivos;
- diálogos y revelado en explorador;
- ventana sin borde;
- siempre encima;
- DPI, tema y accesibilidad;
- paths e identidad de archivo;
- instalación y actualización.

Windows usa MSVC. Linux requiere documentar fontconfig y otras dependencias
nativas reales. macOS permanece como posibilidad futura, no gate actual.

## Estado implementado frente al objetivo

| Área | Implementación actual | Objetivo |
| --- | --- | --- |
| Archivos | Apertura primaria limitada, guardado atómico, conflicto explícito, comprobación al recuperar foco y VFS de workspace | Política completa para recursos secundarios |
| Parsing | Hilo de trabajo, revisión de apertura y evento interno; errores conservan ventana | Cancelación cooperativa con límites |
| Modelo | `Block` y `Span` simplificados | Documento con rangos y semántica |
| Layout | Visible con estimaciones | Geometría para render e interacción |
| Rendering | Software nativo funcional | Display list validada y accesible |
| UI | Ventana, tema, menú contextual y avisos | Chrome, pestañas, comandos y paneles |
| Edición | Fuente alternable, undo/redo, portapapeles explícito, guardado fiel y recuperación local | Buffer escalable, split y accesibilidad completa |
| Workspace | Raíz explícita, VFS, índice acotado, wikilinks, callouts y navegación inicial de backlinks | Paneles, búsqueda, cancelación y detección de cambios |
| Seguridad | Límites, HTML inerte, VFS, guardado y recuperación con pruebas | Controles de recursos secundarios, campaña de fuzzing y validación de release |

## Riesgos arquitectónicos abiertos

- complejidad de selección y accesibilidad sobre dibujo propio;
- madurez de `parley`;
- round-trip fiel con `comrak`;
- estrategia de identidad de archivos multiplataforma;
- índice de workspace sin inflar binario;
- exportación DOCX;
- aislamiento de red para imágenes confirmadas;
- dependencia transitiva no mantenida;
- cobertura Unicode con fuentes pequeñas.

Cada riesgo debe cerrarse con prototipo medido, ADR o criterio explícito de
abandono.
