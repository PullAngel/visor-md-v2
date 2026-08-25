# Visor MD v2 — fase de planificación

**Estado: planeando. No hay código de producto en este repositorio.**

Segunda versión de [Visor MD](https://github.com/PullAngel/visor-md). La v1 está
terminada, publicada y auditada, y corre sobre WebView2. La v2 cambia de
dirección: **nativa, sin motor web, por debajo de 7 MB, para Windows y Linux**,
con el modelo de seguridad de la v1 llevado más lejos y herramientas de estudio
integradas.

En una frase: la herramienta que dejás como predeterminada para abrir cualquier
`.md`, cómoda para estudiar, que se conecta con el segundo cerebro que ya usás,
segura por construcción y tan liviana que arranca antes de que sueltes el mouse.

## Documentación

Empezá por **[`docs/roadmap.md`](docs/roadmap.md)**: es el documento operativo,
el que dice qué se construye y en qué orden.

| Documento | Qué contiene |
| --- | --- |
| [`docs/roadmap.md`](docs/roadmap.md) | **Sprints en orden, con criterio de salida y pruebas.** El documento de trabajo |
| [`docs/vision.md`](docs/vision.md) | Para qué existe y para quién |
| [`docs/product.md`](docs/product.md) | Qué hace, modos de vista, alcance de la v2.0 |
| [`docs/design.md`](docs/design.md) | Identidad visual decidida: color, ventana, iconos, tipografía, movimiento |
| [`docs/architecture.md`](docs/architecture.md) | Cómo está construido y por qué |
| [`docs/security.md`](docs/security.md) | **Blindaje: superficies de ataque, defensas y qué se sacrifica** |
| [`docs/threat-model.md`](docs/threat-model.md) | Modelo de amenaza y las cuatro propiedades |
| [`docs/features.md`](docs/features.md) | Catálogo completo de funciones en checklist, con pros y contras |
| [`docs/study-brainstorm.md`](docs/study-brainstorm.md) | Estudio: explicaciones de cada concepto y estado de cada idea |
| [`docs/connectivity.md`](docs/connectivity.md) | Obsidian, Logseq, GitHub y la puerta a IA local |
| [`docs/budget.md`](docs/budget.md) | Presupuesto de tamaño y de arranque, con plan de medición |
| [`docs/decisions.md`](docs/decisions.md) | Registro de decisiones con su motivo |
| [`docs/inference.md`](docs/inference.md) | IA local: principios y límites |
| [`docs/future.md`](docs/future.md) | Horizonte más allá del roadmap |

## Cómo se llegó acá

[`docs/research/`](docs/research) guarda la exploración previa: diez proyectos
estudiados y tres arquitecturas comparadas, cuando el requisito de <7 MB sin
WebView2 todavía no estaba fijo. Se conserva como bitácora de decisión, no como
plan vigente.

## Licencia

[GNU GPLv3](LICENSE), igual que la v1.
