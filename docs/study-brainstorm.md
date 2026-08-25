# Estudio: ideas y explicaciones

Este documento tiene dos partes: primero las **explicaciones** de los conceptos
que me pediste aclarar, y después el **estado de cada idea** con la decisión
tomada.

---

# Parte 1 · Qué significa cada cosa

## Grafo de notas

Un dibujo donde cada nota es un punto y cada enlace entre notas es una línea.
Se ve la forma de tu conocimiento: qué temas están conectados, cuáles quedaron
sueltos, cuáles son el centro de todo.

Es la pantalla estrella de Obsidian, sí. Y coincido con tu instinto: **es más
bonito que útil**. Cuando la bóveda es chica no dice nada, y cuando es grande se
convierte en una nube de puntos ilegible.

**Decisión: fuera de la v2.0.** Documentado para el futuro.

## Estructura de outliner (Logseq)

En un editor normal escribís párrafos. En un outliner **todo es una lista
anidada**: cada línea es una viñeta que podés plegar, mover con su descendencia,
o enlazar sola.

La diferencia práctica: en Obsidian escribís un documento y le ponés títulos.
En Logseq escribís puntos que cuelgan de otros puntos, y la jerarquía es la
estructura del pensamiento, no un formato.

**No lo adoptamos.** Es un modelo de escritura distinto, no una función. Lo
único que tomamos de ahí es el **plegado de secciones**, que da parte del mismo
beneficio sin cambiar cómo escribís. Ya está en el Sprint 2.

## Backlinks

Al abrir una nota, ver **qué otras notas la enlazan a ella**.

Ejemplo: tenés `Rust.md`. En `Proyecto Visor.md` escribiste "lo hago en
[[Rust]]", y en `Lenguajes.md` también. Al abrir `Rust.md`, un panel te muestra
que esas dos notas la mencionan —aunque desde `Rust.md` nunca las hayas
enlazado.

Es el enlace al revés, y es la mitad del valor del grafo con una fracción del
trabajo: el índice de wikilinks ya se construye para poder navegar, y los
backlinks son ese mismo índice leído en la otra dirección. Casi gratis.

**Decisión: entra, Sprint 5.**

## Repaso espaciado desde el documento

La idea: convertir partes de tus propias notas en material de repaso, **sin
copiar nada a otra app**.

Cómo funcionaría en la práctica:

1. Estás leyendo tus apuntes de redes. Hay una línea que querés memorizar:
   `El puerto 443 es HTTPS`.
2. La marcás como repasable. Con una sintaxis mínima queda algo así:
   `El puerto 443 es ==HTTPS==` — lo resaltado es la respuesta.
3. La app guarda **aparte** (no en el `.md`) que esa línea es repasable, cuándo
   la viste por última vez y qué tan bien la recordaste.
4. Cuando abrís el modo repaso, te muestra `El puerto 443 es ___` y te pide la
   respuesta. Según si acertaste, te la vuelve a mostrar mañana, en tres días o
   en dos semanas.

Eso último es "espaciado": el intervalo crece cuando acertás y se acorta cuando
fallás. Es el mecanismo de Anki, y funciona porque repasar justo antes de
olvidar es lo que fija la memoria.

**Por qué es interesante para nosotros:** hoy tenés que elegir entre Anki
(tarjetas sin contexto, hay que copiar todo a mano) y Obsidian (contexto, pero
el repaso es un plugin de terceros). **Nadie liviano ocupa el medio.**

**El costo real, sin maquillar:** hay que llevar estado por ítem en un archivo
paralelo, y mantenerlo sincronizado si la nota cambia. Si borrás la línea, hay
que darse cuenta. Es más trabajo del que parece, pero no es memoria ni peso: es
un archivo de texto chico y unas fechas.

**Decisión: entra en el Sprint 7, en su forma simple.** Marcar, repasar,
intervalos. Sin estadísticas elaboradas ni algoritmos complejos.

## Tarjetas generadas con IA

Lo mismo de arriba, pero en vez de marcar a mano, un modelo local lee la nota y
propone las tarjetas: "de este párrafo salen tres preguntas, ¿las querés?".

Vos revisás y aceptás las que sirven. La IA propone; nunca decide sola.

**Decisión: futuro.** Depende de que la IA local exista como componente, y el
repaso manual tiene que funcionar bien primero.

## Referencia a nivel de bloque

Enlazar **un párrafo específico** de otra nota, no la nota entera. En Obsidian
es `[[nota^bloque]]`.

Sirve para citar exactamente una definición sin arrastrar todo el documento. El
costo es que cada párrafo necesita identidad propia y estable, lo que cambia el
modelo de datos: ya no alcanza con indexar archivos, hay que indexar partes.

**Decisión: navegar los que ya existen, sí (Sprint 5). Crearlos, futuro.**

## Resaltado persistente sin tocar el `.md`

Esto lo respondí en `vision.md` pero lo repito acá porque preguntaste
específicamente dónde se guarda.

Dos opciones, y **las dos son válidas**:

**Sidecar (por defecto).** Junto a `notas.md` vive un `notas.md.anot` que dice
"en el carácter 340 al 380 hay un resaltado amarillo". El `.md` queda intacto.
No es caché —no se borra solo— es un archivo tuyo, en tu carpeta, que podés
versionar o borrar.

**Incrustado (un clic).** El resaltado se escribe en el `.md` como
`==texto==`. Y acá está la buena noticia que no esperabas: **esa es la sintaxis
nativa de Obsidian**. Si abrís esa nota en Obsidian, el resaltado se ve
igual. No rompe nada.

Se cambia por documento, en los dos sentidos, sin pérdida.

**Decisión: entra en el Sprint 7, con sidecar por defecto.**

---

# Parte 2 · Estado de cada idea

## Entra en la v2.0

| Idea | Sprint | Nota |
| --- | --- | --- |
| Plegado de secciones entre encabezados | 2 | Lo viste en Obsidian; encaja natural con el árbol tipado |
| Contador de palabras y caracteres | 2 | Al pie del panel de índice, como pediste |
| Estadísticas de lectura | 2 | Mismo lugar; solo tiempo estimado, sin adornos |
| Dividir a la derecha y abajo | 3 | Abriendo pestaña **nueva** con opción de crear, abrir o cerrar. No duplicado |
| Barra de formato con desplegable H1–H6 | 3 | De ThisIs-Developer; mejor que tres botones fijos |
| Botones con símbolo y tooltip | 3 | También de ellos |
| Menú de símbolos y entidades | 3 | Ídem |
| Siempre encima | 3 | De Tinta. Con pruebas fuera del happy path |
| Tabla de contenido flotante | 4 | Activable desde configuración avanzada |
| Búsqueda en toda la bóveda | 4 | |
| Backlinks | 5 | |
| Resaltado persistente | 7 | Sidecar por defecto |
| Repaso espaciado, forma simple | 7 | |
| Temporizador Pomodoro | 7 | En configuración avanzada, interfaz mínima |
| Exportar a PDF directo | 7 | Sin pasar por imprimir, como en la v1 |

## Entra si sobra presupuesto

| Idea | Nota |
| --- | --- |
| Menú de emojis | Solo si no suma peso real. Un selector completo pesa; uno de los 200 más usados, no |
| Fijar pestaña | Lo viste en Obsidian. No molesta a quien no lo use |

## Futuro

| Idea | Por qué espera |
| --- | --- |
| Edición en vivo | El objetivo grande siguiente. Alto valor, alto costo |
| Mermaid nativo | El ítem más caro. Empezaría por flowchart y secuencia |
| Grafo de notas | Más bonito que útil |
| Crear referencias de bloque | Cambia el modelo de datos |
| Tarjetas con IA | Depende del componente de IA |
| Corrector ortográfico | Ver abajo |
| Plugins descargables | Ahí entraría KaTeX |

## Descartado

| Idea | Por qué |
| --- | --- |
| Modo foco que oculta todo | No te convenció, y coincido: el Pomodoro da el beneficio sin esconder la interfaz |
| Estructura de outliner | Modelo de escritura distinto, no una función |
| Motores de diagramas remotos | Mandan la fuente del diagrama a un servicio externo |

---

## Sobre ThisIs-Developer/Markdown-Viewer

Coincido con tu lectura: **tiene mucho y por eso se ve sobrecargado**. Lo que
vale la pena robarles es la barra de formato, que está mejor resuelta que la
nuestra:

- **Desplegable de encabezados** en vez de tres botones fijos. Ocupa un botón y
  da acceso a los seis niveles. La v1 gasta tres botones para llegar a la mitad.
- **Símbolos en vez de nombres, con tooltip al pasar.** Cabe más en menos ancho
  y la barra deja de gritar.
- **Menú de símbolos y entidades HTML.** Útil de verdad y barato.

Lo que **no** copiamos: la densidad. Ellos ponen todo a la vista siempre; la
identidad Papel + Tinta pide lo contrario —la barra se calla, el documento manda.

## Sobre el corrector ortográfico

Preguntaste qué tan pesado sería para español e inglés. Los números:

- Los diccionarios de Hunspell pesan **~1 MB por idioma en disco**, así que
  español + inglés son unos 2 MB.
- El problema no es el disco sino la **memoria**: el diccionario de inglés solo
  usa unos 4,5 MB de RAM al cargarse, y baja a ~3,9 MB con compresión.

Contra un presupuesto de 7 MB de binario, meter 2 MB de diccionarios y 8-9 MB
de RAM al núcleo no cierra.

**Decisión: componente descargable, futuro.** Quien lo quiera lo baja; quien no,
mantiene su Visor MD liviano. Es exactamente el mismo trato que la IA local, y
por la misma razón.
