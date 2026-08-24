# Futuro

El horizonte largo, más allá del roadmap concreto. Esto es dirección, no
compromiso: dónde *podría* ir Visor MD v2 si las fases del roadmap cierran y
hay tiempo y ganas. Se escribe para no perder el norte, no para prometer.

## La idea rectora

Visor MD v2 no quiere ser un segundo cerebro. Quiere ser **la mejor ventana
para mirar dentro del segundo cerebro que ya tenés**: rápida, liviana, segura,
y que respeta que tus notas son tuyas y viven en archivos que vos controlás.
Ese posicionamiento —cristal, no caja fuerte— es lo que la separa de Obsidian y
de Notion, y lo que debería sobrevivir a cualquier función futura.

## Direcciones posibles

### Multiplataforma
La v1 es solo Windows. La elección de Rust + una pila de dibujo portable
(`parley`/`tiny-skia` corren en cualquier lado) deja la puerta abierta a macOS
y Linux sin reescribir el núcleo. No es un objetivo de la v2.0, pero el stack
se elige de forma que no lo impida. Es una diferencia real con la v1 y con
Tinta, ambas atadas a Windows.

### Diagramas nativos, de a poco
Mermaid completo es carísimo (ADR-5), pero un subconjunto —flowcharts y
diagramas de secuencia, los dos más usados— podría hacerse nativo con un
esfuerzo acotado, como componente opcional. No los 22 tipos: los dos que la
gente de verdad usa.

### Estudio como primera clase
Si el repaso espaciado y el modo foco (ver `brainstorm-estudio.md`) resultan
bien, Visor MD v2 podría volverse la herramienta de referencia para *estudiar
sobre notas en Markdown planas* —el hueco entre Anki (tarjetas sueltas, sin
contexto) y Obsidian (contexto, pero el repaso es un plugin de terceros—. Ese
hueco es real y nadie liviano lo ocupa.

### IA local como diferenciador de privacidad
Ver `inference.md`. El día que la IA local de calidad sea barata de embeber, un
segundo cerebro que resume, genera tarjetas y responde preguntas **sin que nada
salga del equipo** es un argumento de venta enorme frente a todas las apps que
mandan tus notas a una API. La v2 estaría posicionada para eso por diseño, no
por parche.

## Lo que el futuro NO debería erosionar

Por más funciones que se sumen, tres cosas no se tocan, porque son la identidad
del producto:

1. **<7 MB en el núcleo.** Todo lo grande es componente opcional aparte.
2. **Nada sale del equipo sin que el usuario lo dispare.** Ni notas, ni fuentes
   de diagramas, ni telemetría.
3. **Seguro por construcción.** No se agrega una función que reintroduzca un
   motor de scripts o una superficie de ejecución, por conveniente que sea.

Si una función futura obliga a romper una de las tres, la función no entra —o
entra como un producto distinto, no como Visor MD v2. Esa disciplina es lo que
evita que el proyecto se convierta, versión a versión, en otra cosa pesada e
insegura más.

## El riesgo real del futuro

No es técnico, es de sostenibilidad. simpler-paper murió archivado. Un proyecto
de un mantenedor sobrevive si el alcance está acotado, las pruebas cubren lo
importante, y cada versión es un punto de parada válido —no un compromiso
abierto que hay que sostener para siempre. La v2 se diseña para poder
detenerse en cualquier fase completa y seguir siendo útil, no para requerir
mantenimiento perpetuo.
