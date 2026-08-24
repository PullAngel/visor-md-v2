# Visor MD v2 — fase de planificación

**Estado: planeando. No hay código de producto en este repositorio.**

Segunda versión de [Visor MD](https://github.com/PullAngel/visor-md). La v1
está terminada, publicada y auditada, y corre sobre WebView2. La v2 cambia de
dirección: **nativa, sin motor web, por debajo de 7 MB**, en la filosofía de
[Tinta](https://github.com/oipoistar/tinta) pero con el modelo de seguridad de
Visor MD llevado más lejos, y con la profundidad de producto de
[ThisIs-Developer/Markdown-Viewer](https://github.com/ThisIs-Developer/Markdown-Viewer).

El objetivo en una frase: la herramienta que uno deja como predeterminada para
abrir cualquier `.md`, cómoda para estudiar y tomar notas, que se conecta con
el segundo cerebro que ya usás (Obsidian, un repo de GitHub), segura por
construcción y tan liviana que arranca antes de que sueltes el mouse.

## Documentación

| Documento | Qué contiene |
| --- | --- |
| [`docs/vision.md`](docs/vision.md) | Para qué existe y para quién |
| [`docs/producto.md`](docs/producto.md) | Qué hace, funciones, alcance de la v2.0 |
| [`docs/arquitectura.md`](docs/arquitectura.md) | Cómo está construido y por qué |
| [`docs/decisiones.md`](docs/decisiones.md) | Registro de decisiones: lenguaje, stack, qué se descartó y con qué motivo |
| [`docs/calculos.md`](docs/calculos.md) | El presupuesto de 7 MB y de arranque, con plan de medición |
| [`docs/audit.md`](docs/audit.md) | Modelo de amenaza y por qué el enfoque nativo mejora la seguridad de la v1 |
| [`docs/conectividad.md`](docs/conectividad.md) | Integración con Obsidian y con GitHub |
| [`docs/inference.md`](docs/inference.md) | IA local para estudio (resúmenes, tarjetas, preguntas), opt-in y sin salir del equipo |
| [`docs/brainstorm-estudio.md`](docs/brainstorm-estudio.md) | Ideas para superar a los readers actuales, apoyadas en apps de estudio |
| [`docs/roadmap.md`](docs/roadmap.md) | Fases concretas de construcción |
| [`docs/futuro.md`](docs/futuro.md) | Horizonte largo, más allá del roadmap |

## Cómo se llegó a esta dirección

Los documentos [`docs/01-investigacion.md`](docs/01-investigacion.md) a
[`docs/07-hoja-de-ruta.md`](docs/07-hoja-de-ruta.md) son el registro de la
exploración previa: diez proyectos estudiados y tres arquitecturas comparadas,
cuando el requisito de <7 MB y "sin WebView2" todavía no estaba fijo. Ese
requisito, ahora firme, resuelve la pregunta que quedaba abierta en
[`06-comparacion-y-decision.md`](docs/06-comparacion-y-decision.md) a favor de
la vía nativa. Se conservan como bitácora de decisión, no como plan vigente:
el plan vigente es el de la tabla de arriba.

## Licencia

[GNU GPLv3](LICENSE), igual que la v1.
