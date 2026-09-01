# Auditoría visual: retorno a Papel + Tinta

Fecha: 1 de septiembre de 2026. Este documento convierte el feedback visual en
criterios de trabajo. No sustituye [`design.md`](design.md), que conserva la
autoridad sobre la identidad del producto.

## Evidencia

La captura reproducible de v2 con `tests/fixtures/sprint1-visual.md` mostró que
la tipografía editorial y la sintaxis ya tienen una base útil, pero el chrome se
percibe como texto técnico sobre un lienzo oscuro. El bloque de código repetía
el botón Copiar por línea y el contraste entre documento, barra y estado era
insuficiente. La referencia de v1 aporta una composición coherente, aunque no
su verde saturado ni su densidad de controles.

## Desviaciones confirmadas

1. La barra usa rótulos provisionales donde el sistema aprobado pide iconos
   suaves y estados sobrios.
2. Las superficies nocturnas no comunican con claridad fondo, base, elevado y
   flotante.
3. Pestañas y estado pueden perder presencia visual al competir con el viewport.
4. Código, tablas, citas y paneles necesitan reglas compartidas de espaciado y
   borde para verse como parte de un mismo producto.
5. La composición aún no ha pasado QA visual sistemático en día, noche, edición,
   dividida, ventanas estrechas y DPI alto.

## Revisión profunda: causas, no solo síntomas

La segunda inspección usa capturas controladas de lectura, fuente y vista
dividida. Distingue los defectos que una persona ve de la decisión concreta que
los provoca. Es importante porque cambiar solamente colores o márgenes no
resolvería la sensación de prototipo.

| Área | Evidencia observada | Causa actual | Impacto | Prioridad |
| --- | --- | --- | --- | --- |
| Chrome superior | Acciones como texto plano dentro de la misma franja que movería la ventana | La barra empieza en `y=8`, mide 28 px y comparte la franja de chrome de 40 px | Se percibe como interfaz de pruebas, no como aplicación editorial | P0 |
| Fuente | Todo el Markdown fuente aparece verde, incluso el texto ordinario | Cada línea de fuente se representa internamente como `Kind::Code`, cuyo rol de tinta es `Accent` | El verde deja de ser guía y el editor se parece a una terminal | P0 |
| Vista dividida | El título y párrafos se parten demasiado pronto; un H1 ocupa dos líneas incluso en la ventana inicial | La división es siempre 50/50, sin ancho mínimo editorial ni alternativa compacta | Comparar se vuelve peor que alternar entre modos | P0 |
| Viewport | La altura desplazable descuenta la barra inferior pero no reserva de forma explícita el chrome superior | Cálculos de scroll, selección y dibujo parten de convenciones distintas | Riesgo de contenido bajo controles, marcas o overlays en alturas reducidas | P0 |
| Superficies | Documento, código, barra y paneles se confunden en noche | La paleta solo modela fondo y una superficie; la misma superficie se reutiliza para bloques y elementos elevados | Se pierde profundidad y orientación sin necesidad de sombras pesadas | P0 |
| Código y callouts | Un bloque de código se pinta por línea y los callouts no comparten una caja de grupo consistente | El parser emite líneas de código y el renderer dibuja una caja por `Slot` | Bandas repetidas, bordes discontinuos y botón de copia visualmente aislado | P1 |
| Pestañas y estado | La banda inferior tiene solo 28 px para pestañas, cierres y estado; el estado puede desaparecer si hay pestañas | Un único rectángulo contiene dos roles que compiten por ancho | La multitarea existe, pero no se siente robusta ni fácil de leer | P1 |
| Paneles y menús | Se colocan en coordenadas fijas y usan la misma superficie que el documento | `PANEL_Y`, overlays y menús no derivan de una geometría de chrome ni de una capa flotante | Pueden tapar contenido y no comunican que son temporales | P1 |
| Tipografía | Newsreader en lectura es la parte más conseguida; Sora no llega a distinguir una jerarquía de UI por falta de iconos y escala | Las familias aprobadas están bien incorporadas, pero los roles de interfaz aún usan rótulos provisionales | No cambiar familias: primero corregir composición y roles | P1 |
| Accesibilidad visual | Hay foco y teclado básicos, pero los iconos aún no tienen una representación visual ni un estado de foco de suficiente tamaño | La barra se diseñó primero como atajos etiquetados | Un refactor de iconos debe mantener foco, menú y paleta como nombre textual de la acción | P1 |

### Diagnóstico de composición

El documento no está encerrado en un área de lectura claramente distinguible y
el chrome no tiene una función visual propia. Hoy ambos se construyen desde
coordenadas independientes: los márgenes del documento, la barra superior, los
paneles y la banda inferior no nacen de una misma descripción de viewport. Por
eso el resultado se ve correcto por partes, pero no como una sola ventana.

La corrección debe crear primero una geometría compartida: chrome superior,
área de contenido, chrome inferior y, cuando exista, dos paneles de comparación
con anchura mínima. Sobre esa geometría se dibujan después superficies, iconos
e información. No conviene invertir el orden.

### Diagnóstico de color y profundidad

La paleta aprobada ya definía cuatro niveles nocturnos: fondo, base, elevado y
flotante. El código actual solo dispone de fondo y superficie; por eso una
misma tinta se usa para el área documental, código, barra, menú y panel. La
solución no requiere una estética brillante ni sombras permanentes: basta con
introducir roles `base`, `elevated` y `floating`, usar borde estructural y
reservar el verde para estados, enlaces internos, filetes y confirmaciones.

El verde en toda la fuente es una desviación especialmente clara. El modo
fuente necesita JetBrains Mono y contraste suficiente, pero su tinta base debe
ser el texto normal; sintaxis futura, selección y acciones pueden aportar color
de forma limitada. No se debe introducir resaltado de sintaxis improvisado
solo para corregir esta apariencia.

### Diagnóstico de modos

Lectura debe ser el modo dominante. Fuente debe ser una herramienta precisa,
no un segundo producto con estética terminal. Vista dividida es útil solo si
cada panel alcanza una medida mínima cómoda; con una ventana de 900 px, una
división rígida deja demasiado poco espacio después de márgenes. La solución
conservadora es definir un umbral: en ventanas estrechas, ofrecer una división
asimétrica o volver explícitamente a una comparación alternable, sin recortar
ni comprimir el documento.

Esto no cambia el producto ni elimina la vista dividida. Evita que una función
aprobada parezca defectuosa en el tamaño inicial de la aplicación.

### Verificación de plataforma

Una captura posterior de la compilación release actual confirma que Windows sí
está usando el chrome propio: los controles se dibujan dentro de la franja de
Visor MD, no existe una segunda barra de título. La evidencia anterior mezclaba
una captura de una ejecución previa y no justifica duplicar chrome ni controles.

La misma captura detectó un defecto real distinto: el tamaño inicial de 900 ×
760 podía extender una ventana sin borde por debajo del área de trabajo de un
portátil y ocultar la barra inferior detrás de la barra del sistema. El tamaño
inicial se reduce de forma conservadora; el mínimo de 640 × 480 no cambia.

## Orden de corrección recomendado

1. Crear una única geometría de ventana y sus pruebas: chrome superior,
   contenido, banda inferior, hit testing, scroll y división.
2. Ampliar la paleta a las cuatro superficies aprobadas y reasignar cada capa.
3. Corregir la tinta del modo fuente y el fondo de fuente/división antes de
   incorporar cualquier resaltado sintáctico.
4. Sustituir los rótulos provisionales por iconos nativos suaves, conservando
   texto en foco, menú contextual y paleta de comandos.
5. Agrupar código, callouts y tablas como componentes documentales completos,
   no como una sucesión de líneas independientes.
6. Reequilibrar pestañas, estado, paneles y overlays sobre la nueva geometría.
7. Ejecutar las capturas definidas abajo en ventana inicial, mínima, amplia,
   día, noche, fuente, dividida y DPI alto. El QA humano decide si la identidad
   se siente lograda; no si un rectángulo se dibuja o no.

## Correcciones aplicadas durante la auditoría

Estas correcciones reducen defectos objetivos; no cierran aún el sprint visual.

- El viewport de documento ahora reserva una franja superior y otra inferior
  mediante una sola conversión de coordenadas. Scroll, hit testing, selección,
  botón de copia, divisor y marcas de búsqueda usan esa misma frontera.
- La paleta del renderer incorporó `base`, `elevated` y `floating`: documento,
  bloques, barra inferior, menús, paneles y avisos ya no usan una sola
  superficie indistinta.
- La fuente editable y los bloques de código usan tinta normal. El verde queda
  en enlaces, estados y filetes; no se añadió resaltado sintáctico artificial.
- Un bloque de código de varias líneas se pinta como una sola pieza visual. Dos
  cercas de código vecinas conservan sus fondos y acciones de copia separados.
- El tamaño inicial se hizo más conservador para que una ventana sin borde no
  esconda su barra inferior en áreas de trabajo reducidas o escaladas.
- La comparación fuente/vista dejó de dividir el ancho exactamente a la mitad:
  reserva 42 % para la fuente editable y 58 % para lectura, de modo que la
  vista renderizada no pierda medida editorial en la ventana inicial.
- Las dos barras usan ahora iconos suaves dibujados por el renderer nativo en
  lugar de rótulos provisionales. Los nombres completos permanecen en la
  paleta y los atajos; no se añade una fuente de iconos ni una dependencia.

Las pruebas cubren la geometría nueva, el rol de tinta de fuente y la separación
de cercas contiguas. Falta la comparación visual de release después de cada
siguiente ajuste de composición.

## Plan de corrección

### A. Fundaciones y defectos visibles

- corregir controles repetidos y clipping;
- reservar chrome y viewport con una única geometría, incluido scroll,
  selección, overlays y ventanas mínimas;
- establecer cuatro superficies y separadores consistentes;
- retirar el verde dominante de la fuente y proteger una medida mínima útil
  para la vista dividida;
- añadir pruebas de hit testing y geometría a cada defecto real.

### B. Chrome editorial

- reemplazar rótulos de acciones primarias por iconos suaves dibujados de forma
  nativa, manteniendo texto accesible en foco, menú y paleta;
- equilibrar jerarquía de título, acciones, pestañas y estado;
- aplicar hover, foco y activo sin convertir los botones en bloques verdes.

### C. Documento y edición

- recalibrar márgenes, ancho de lectura, ritmo vertical y bloques enriquecidos;
- unificar código, tablas, citas, callouts y paneles bajo la escala de
  superficies;
- hacer que fuente y vista dividida conserven identidad editorial y separación
  clara sin competir con el texto.

### D. Validación visual

- producir capturas reproducibles para lectura día/noche, edición, dividida,
  tablas, código, panel y ventana mínima;
- revisar contraste, foco, clipping, tamaño de objetivos y continuidad con
  `design.md`;
- registrar QA humano pendiente antes de declarar cerrado el sprint visual.

## Criterio de salida

Una persona debe reconocer una aplicación editorial antes de leer sus botones:
el documento domina, el chrome acompaña, el verde guía y ninguna capa se
confunde con otra. Las capturas y QA deben demostrarlo, no solo afirmarlo.
