# Futuro

El horizonte más allá del roadmap. Es dirección, no compromiso: dónde *podría*
ir la v2 si los sprints cierran. Se escribe para no perder el norte.

## La idea rectora

Visor MD v2 no quiere ser un segundo cerebro. Quiere ser **la mejor ventana para
mirar dentro del que ya tenés**: rápida, liviana, segura, y que respeta que tus
notas son tuyas y viven en archivos que controlás. Cristal, no caja fuerte. Ese
posicionamiento debería sobrevivir a cualquier función futura.

## En orden de prioridad

### 1 · Edición en vivo
Escribir sobre el documento renderizado, con las marcas visibles solo en la
línea del cursor, lo que viste en Obsidian. Es el objetivo grande siguiente:
el que más cambia la sensación de uso, y también el más difícil. Exige cursor y
selección sobre texto renderizado y reconstrucción parcial del árbol en cada
pulsación. Ver `architecture.md`.

**Presupuesto: crédito de Fable 5.** Es uno de los dos ítems que se separaron
como los más difíciles del proyecto. Cuando llegue su turno en el roadmap, se
avisa antes de activar el crédito reservado para esta tarea.

### 2 · Mermaid nativo
Empezando por flowchart y secuencia, que son los que la gente usa de verdad. No
los 22 tipos. Es la única función por la que vale gastar del techo de tamaño.

**Presupuesto: crédito de Fable 5.** El otro de los dos ítems más difíciles.
Mismo trato que la edición en vivo: se avisa antes de activar el crédito.

### 3 · IA local por el camino barato
Hablar con un Ollama que ya tengas instalado: cero peso agregado, cero modelo
que mantener, y nada sale del equipo. Resumir una nota, proponer tarjetas de
repaso, responder preguntas sobre tus apuntes. Ver `inference.md`.

### 4 · macOS publicado
Se compila y prueba desde temprano; se publica cuando tenga pruebas propias.

### 5 · Repaso espaciado completo
Sobre la base simple del Sprint 7: estadísticas, ajuste de intervalos, repaso
por carpeta o por etiqueta.

### 6 · Corrector ortográfico
Componente descargable, español e inglés. Aparte por su costo de memoria.

### 7 · Plugins descargables
El lugar donde entraría KaTeX, y donde funciones de nicho pueden vivir sin
engordar el núcleo. Exige resolver antes el modelo de confianza: un plugin es
código de terceros, y eso choca de frente con la tesis del proyecto. No entra
hasta tener una respuesta buena a eso.

### 8 · Grafo de notas y referencias de bloque
Solo si la pila de dibujo demostró que rinde, y si el modelo de datos lo
soporta sin retorcerse.

## Lo que el futuro NO debe erosionar

Por más funciones que se sumen, tres cosas no se tocan:

1. **El núcleo liviano.** Todo lo grande es componente opcional aparte.
2. **Nada sale del equipo sin que lo pidas.** Ni notas, ni fuentes de diagramas,
   ni telemetría.
3. **Seguro por construcción.** No se agrega nada que reintroduzca un motor de
   scripts o una superficie de ejecución, por conveniente que sea.

Si una función futura obliga a romper una de las tres, no entra —o entra como
un producto distinto, no como Visor MD v2. Esa disciplina es lo que evita que
el proyecto se convierta, versión a versión, en otra cosa pesada e insegura más.

## El riesgo real

No es técnico, es de sostenibilidad. simpler-paper murió archivado. Un proyecto
de un mantenedor sobrevive si el alcance está acotado, las pruebas cubren lo
importante, y **cada versión es un punto de parada válido**, no un compromiso
abierto que hay que sostener para siempre.

La v2 se diseña para poder detenerse en cualquier sprint completo y seguir
siendo útil.
