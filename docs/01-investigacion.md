# Investigación: qué hace cada proyecto y qué lección deja

Diez proyectos revisados. No todos compiten con Visor MD — algunos son
librerías para que un desarrollador las incruste en su propia app, uno es un
plugin de IDE, y uno ni siquiera es un visor de Markdown. Igual, a todos se
les sacó una lección concreta, incluidos los que en la comparación anterior
se descartaron demasiado rápido.

## Tinta — la referencia de rendimiento

C++ nativo, sin motor web: renderiza con Direct2D y DirectWrite directamente
sobre GDI, y parsea con MD4C. El resultado son números que ningún proyecto
basado en navegador puede igualar: **binario de 1,9 MB, arranque bajo
100 ms**, frente a los 90-350 MB y 1,5-3 s de Typora, Obsidian o VS Code.

**La lección que importa más**: no tener un motor de JavaScript no es una
mitigación de seguridad, es la ausencia estructural de la superficie de
ataque entera. Visor MD sanitiza HTML activo; Tinta, si no interpreta HTML
en absoluto, no tiene nada que sanitizar porque no hay nada que ejecutar.
Es una categoría de defensa distinta, y potencialmente superior, con un
costo real: pierde `<details>`, `<kbd>`, alertas con estilo propio, y
cualquier cosa que dependa de HTML embebido con intención.

**Lo que no se sabe y hay que probar, no asumir**: la documentación de Tinta
no dice nada sobre política de red para imágenes remotas ni sobre cómo
trata un `<script>` embebido si de hecho lo encuentra. No hay wiki, no hay
declaración de modelo de amenaza. Antes de asumir que Visor MD le gana en
seguridad, habría que instalar el portable y tirarle el corpus de
`tests/security/` de la v1, no dar nada por sentado.

**Otras lecciones**: firma de código vía Microsoft Store evita la advertencia
de SmartScreen sin pagar un certificado — vale la pena evaluarlo para
distribución. Sin plugins ni extensibilidad: es una herramienta cerrada,
completa pero no puede crecer sin que un solo mantenedor la toque a mano.

## ThisIs-Developer/Markdown-Viewer — la referencia de profundidad de producto

Web app / PWA / contenedor Neutralino, con explorador de archivos,
carpetas anidadas, pestañas, favoritos, siete motores de diagramas
(Mermaid, PlantUML, Graphviz, D2, Vega-Lite, WaveDrom, Markmap), modelos 3D
vía Three.js, exportación a PDF/PNG/HTML, y hasta compartir en vivo.

**La lección de producto**: lo que separa "abre un archivo" de "herramienta
de trabajo diario" no es el renderizador, es el **workspace persistente**:
carpetas, recientes, favoritos, que sobreviven entre sesiones. Visor MD v1
tiene pestañas, pero se resetean cada arranque — no hay noción de proyecto
ni de carpeta de trabajo.

**La lección de seguridad, y es la más útil de las diez**: su propia wiki
declara el principio "on-device editing" — todo lo normal se queda en el
dispositivo, y **"cualquier función que mande contenido afuera está
disparada por el usuario y está documentada"**. Es literalmente el mismo
principio que ya rige las imágenes remotas en Visor MD, aplicado por
alguien más a un problema más amplio. La aplicación concreta: sus motores
de diagramas remotos (PlantUML, D2, Graphviz vía servicios externos) violan
ese principio para esos casos puntuales, y lo avisan. Eso es exactamente lo
que Visor MD debería evitar si algún día suma más motores de diagramas que
Mermaid: **solo sumar los que corran 100% local (WASM o binario propio
auditado), nunca uno que mande la fuente del diagrama a un servicio de
terceros**, ni con aviso.

**Nota de arquitectura**: Neutralino, la base del contenedor de escritorio,
es conceptualmente primo de pywebview — usa el motor web que ya trae el
sistema operativo en vez de empaquetar uno propio. No es una prueba de que
haga falta abandonar WebView2; es una confirmación de que ese camino
(motor del sistema, no navegador propio) es el correcto para mantenerse
liviano sin ir a nativo puro.

## Moji — el primo más parecido en filosofía de seguridad

Electron con React y CodeMirror 6. Declara explícitamente su postura:
*"sandboxed renderer, context isolation, nodeIntegration: false, DOMPurify
sanitization"*. Es la aplicación de escritorio, de las diez, que más se
parece en criterio a Visor MD — piensa en el mismo problema, con otro motor.

**Lección de arquitectura, la más aplicable de toda la investigación**: esa
frase describe un **proceso de renderizado separado del proceso con
privilegios**, comunicados por un canal angosto. Visor MD v1 corre todo en
un proceso, con un puente pywebview que expone métodos de Python
directamente a JavaScript. Funciona y hasta ahora no se le encontró una
vulnerabilidad de ese puente específicamente — pero es una superficie más
ancha de la que necesitaría ser. Separar el proceso que toca el sistema de
archivos del proceso que renderiza contenido no confiable, con un contrato
de mensajes angosto entre los dos, es exactamente la mejora que "mejorar
mucho" la seguridad de la v1 debería significar.

**Otra lección concreta**: manejo explícito de archivos grandes (streaming,
parseo en Web Worker, virtualización sobre 20 MB). Visor MD v1 no tiene
ninguna estrategia para eso — decisión consciente en su momento, pero un
documento de 15-20 MB hoy renderiza sincrónico y podría trabar la interfaz.

## idea-multimarkdown — qué significa "de nivel profesional" en la edición

Plugin de IntelliJ IDEA. La versión gratuita cubre vista previa, resaltado y
autocompletado de enlaces; la versión paga ("Enhanced") suma refactor,
find-usages, validación y formato automático.

**La lección de producto**: "profesional" en este rubro no es sobre el
render, es sobre la **edición estructural**: renombrar un encabezado y que
los enlaces internos que apuntan a él se actualicen solos, pegar una imagen
del portapapeles y que aparezca como archivo + enlace ya armado,
autocompletar enlaces a otros documentos del mismo proyecto, navegar por
un árbol de estructura. Son exactamente las funciones que le faltan a
Visor MD v1 para pasar de "visor con edición" a "editor de verdad".

**La lección de negocio, no de arquitectura, pero vale anotarla**: que
JetBrains cobre justo por esas funciones es evidencia de que hay demanda
real por edición de Markdown de nivel profesional. No cambia que Visor MD
siga siendo libre — pero confirma que el hueco que se quiere llenar existe
y alguien ya paga por llenarlo peor (encerrado en un IDE, no como
herramienta standalone).

## bytemd, hashmd, EasyMDE (+ fork de SteamDB) — lecciones de arquitectura de plugins

Estos tres no son aplicaciones, son componentes para que un desarrollador
externo arme su propio editor. Compararlos con Visor MD en seguridad no
sería justo — la responsabilidad de sanitizar es de quien los usa, no del
componente en sí. Aun así, hay una lección de arquitectura real:

**ByteMD es el único que sanea por defecto**, sin que el desarrollador que
lo integra tenga que acordarse de hacerlo, y lo logra porque su sistema de
plugins opera sobre un **árbol sintáctico tipado** (remark/rehype), no
sobre texto HTML crudo. Cada plugin transforma el árbol; la sanitización
final es un único paso, al final, después de que todos los plugins ya
corrieron. Es el mismo patrón que ya usa `render.js` en Visor MD v1 — un
solo `clean()` con DOMPurify al final del pipeline — y confirma que es el
patrón correcto para formalizar en la v2: **ningún renderizador o plugin
inserta nada en el DOM salvo a través de ese único punto auditado**.

**EasyMDE es el contraejemplo a evitar**: ofrece un `sanitizerFunction`
como opción, apagado por defecto. Seguridad opcional es seguridad que la
mayoría de los que integran la librería nunca activa. La postura de
Visor MD —sanitización obligatoria, sin forma de apagarla— es la correcta
por contraste, y la v2 no debería aflojarla nunca, ni siquiera como
configuración avanzada.

## simpler-paper — la lección que no tiene que ver con el código

No es un visor de Markdown: es un generador de sitios de documentación
estática, del tamaño de una versión mínima de Docusaurus, y está
**archivado desde el 23 de junio de 2026**. Se incluyó por error en la
lista original, pero deja dos lecciones igual de válidas:

Primero, su lema — *"~3kb gzipped, sin framework"* — es un recordatorio de
que la disciplina de peso no es solo una métrica de rendimiento, es una
postura frente al usuario: no gastarle disco ni ancho de banda que no hace
falta gastar. Es la misma disciplina que ya está en la v1 (14 MB en vez de
los 150 de un Electron) y que la v2 no debería perder por sumar funciones.

Segundo, y más importante: un proyecto de un solo mantenedor, sin plan de
sostenibilidad, termina archivado. La v2 debería nacer con pruebas
(como ya tiene la v1) y con un alcance explícito, precisamente para no
correr esa suerte.

## Marcdown — el contraejemplo de "liviano sin criterio"

PWA sobre MarkedJS + Highlight.js + MathJax, sin ninguna mención de
sanitización en su documentación. Es liviano y funciona en el navegador sin
instalar nada, pero no se fija como programa predeterminado de Windows —
rompe el objetivo central de Visor MD— y no hay evidencia de que trate el
contenido como no confiable. Sirve como ejemplo de lo que "liviano" *no*
alcanza a ser si no viene acompañado de una postura de seguridad explícita.

## Resumen: qué se lleva la v2 de cada uno

| Proyecto | Se toma |
| --- | --- |
| Tinta | La ambición de tamaño y arranque como objetivo, no como copia literal — y la idea de que "sin motor de scripts" es una categoría de defensa propia |
| ThisIs-Developer/Markdown-Viewer | Workspace persistente (carpetas, recientes, favoritos) y el principio "nada sale del dispositivo sin que el usuario lo dispare", extendido a futuros motores de diagramas |
| Moji | Separación de proceso privilegiado y proceso de renderizado, con un canal de mensajes angosto — la mejora de seguridad más concreta de toda la investigación |
| idea-multimarkdown | Edición estructural: renombrado seguro de encabezados, pegar imagen como archivo+enlace, autocompletado |
| ByteMD | Formalizar el patrón "un solo punto de sanitización al final del pipeline" como regla arquitectónica, no como casualidad |
| EasyMDE | Qué no hacer: sanitización opcional apagada por defecto |
| simpler-paper | Disciplina de peso como postura, y sostenibilidad del proyecto vía pruebas y alcance definido |
| Marcdown | Contraejemplo: liviano sin postura de seguridad no es suficiente |
