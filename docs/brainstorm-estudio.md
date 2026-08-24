# Brainstorm: superar a los readers actuales, apoyado en apps de estudio

Ideas, no compromisos. La mayoría no entra en la v2.0; el valor del documento
es tener el mapa de hacia dónde puede crecer y por qué, para no reinventarlo
más adelante. Cada idea marca de dónde sale y cuánto cuesta.

## Qué hacen bien las apps de estudio, y qué robar de cada una

| App | Lo que hace bien | Qué se puede tomar |
| --- | --- | --- |
| **Obsidian** | Wikilinks, grafo de notas, bóveda de archivos planos, 1500+ plugins | Wikilinks (ya en v2.0); el grafo como visualización opcional |
| **Logseq** | Estructura de outliner, journals diarios, enlace a nivel de bloque | Enlace y referencia a nivel de bloque, no solo de nota |
| **Anki / RemNote** | Repaso espaciado, tarjetas, recuerdo activo | Generar tarjetas desde las notas y repasarlas sin salir del reader |
| **Notion** | Dashboards, notas estructuradas para tareas | Menos relevante: es lo pesado y en la nube que evitamos |

## Cómo mejorar sobre ThisIs-Developer/Markdown-Viewer

Ellos son el más completo en funciones, así que las ideas acá son sobre dónde
*flaquean*, no sobre copiarlos:

1. **Diagramas sin mandar nada afuera.** Su mayor defecto: PlantUML/D2/Graphviz
   se renderizan mandando la fuente del diagrama a servicios externos. La v2
   nunca hace eso —o el diagrama corre local, o se muestra la fuente. Es una
   ventaja de privacidad concreta, no retórica.
2. **Liviano de verdad.** Ellos son web/Neutralino; la v2 es nativa <7 MB. La
   misma profundidad de workspace, una fracción del peso.
3. **Wikilinks de Obsidian.** Ellos no los entienden. Es la puerta de entrada
   al público de segundos cerebros, que es enorme.
4. **Seguro por construcción**, no "sanitizado". Ver `audit.md`.

## Ideas de estudio, ordenadas por relación valor/costo

### Alto valor, costo razonable — candidatas a fase posterior cercana

- **Repaso desde el documento.** Marcar una frase o un par pregunta/respuesta en
  una nota (con una sintaxis mínima, por ejemplo `==resaltado==` o un bloque
  especial) y que Visor MD v2 la convierta en algo repasable con repetición
  espaciada, sin salir del reader. Es Anki adentro de tus propias notas, sin
  copiar y pegar a otra app. *(De Anki/RemNote. Costo: medio —hay que llevar
  estado de repaso por ítem.)*
- **Modo estudio / foco.** Una vista que oculta todo el chrome y muestra la
  nota sola, con temporizador tipo Pomodoro opcional. Barato y muy pedido por
  estudiantes. *(Costo: bajo.)*
- **Backlinks.** Al abrir una nota, mostrar qué otras notas la enlazan. Es la
  mitad del valor del grafo de Obsidian con una fracción del trabajo —solo hay
  que invertir el índice de wikilinks que ya se construye. *(De Obsidian/Logseq.
  Costo: bajo, si el índice ya existe.)*

### Valor alto, costo alto — horizonte más lejano

- **Grafo de notas.** La visualización estrella de Obsidian. Cara de dibujar
  bien y de que rinda. Solo si el layout de texto de la Fase 0 demuestra que la
  pila de dibujo aguanta. *(Costo: alto.)*
- **Generación de tarjetas con IA local.** Ver `inference.md`. Depende de que la
  IA local exista como componente. *(Costo: alto, opt-in.)*
- **Referencia a nivel de bloque** (estilo Logseq): enlazar y transcluir un
  bloque específico, no una nota entera. Potente para notas de estudio, pero
  cambia el modelo de datos. *(Costo: alto.)*

### Ideas baratas que suman comodidad

- **Resaltado persistente.** Resaltar texto en modo lectura y que quede
  guardado (como anotación aparte, sin tocar el `.md` original salvo que se
  pida). *(Costo: medio.)*
- **Tabla de contenido flotante** que sigue el scroll y muestra dónde estás.
  *(Costo: bajo; la v1 ya tiene índice lateral.)*
- **Exportar una nota a PDF con estilo de estudio** (márgenes para anotar a
  mano). *(Costo: bajo sobre el exportador.)*
- **Búsqueda en toda la bóveda**, no solo en el documento abierto. Casi
  obligatoria para un workspace real. *(Costo: medio.)*

## El filtro que ordena todo esto

Ninguna de estas ideas entra si rompe uno de los dos límites del proyecto:
**el presupuesto de 7 MB** (por eso la IA es componente aparte, por eso el
grafo espera a saber si la pila de dibujo rinde) y **la política de no mandar
nada afuera** (por eso los diagramas son locales o son fuente visible, por eso
la IA es local o no es). Toda idea buena que respete esos dos límites es
candidata; toda idea buena que los rompa, no.
