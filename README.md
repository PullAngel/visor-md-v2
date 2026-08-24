# Visor MD v2 — fase de planificación

**Estado: planeando. No hay código de producto en este repositorio.**

Esto es la preparación de una segunda versión de
[Visor MD](https://github.com/PullAngel/visor-md), el visor y editor de
Markdown para Windows. La v1 ya está terminada, publicada y auditada; este
repositorio no la reemplaza — la audita a ella misma frente a la competencia
real, y decide si vale la pena, y cómo, construir algo mejor.

El objetivo declarado: una herramienta potente, usable a diario por
profesionales de IT, liviana y segura — apoyada en el modelo de amenaza de la
v1, pero llevándolo más lejos.

## Cómo leer esto

1. [`docs/01-investigacion.md`](docs/01-investigacion.md) — qué se estudió y
   qué lección concreta deja cada proyecto, con las llamadas de atención que
   más pesan: **Tinta** en tamaño y arranque, **ThisIs-Developer/Markdown-Viewer**
   en profundidad de funciones.
2. [`docs/02-requisitos.md`](docs/02-requisitos.md) — lo que la v2 tiene que
   cumplir, derivado de esa investigación y no inventado de cero.
3. Tres arquitecturas candidatas, comparadas sin favoritismos:
   - [`docs/03-propuesta-a-hibrida.md`](docs/03-propuesta-a-hibrida.md)
   - [`docs/04-propuesta-b-nativa.md`](docs/04-propuesta-b-nativa.md)
   - [`docs/05-propuesta-c-workspace.md`](docs/05-propuesta-c-workspace.md)
4. [`docs/06-comparacion-y-decision.md`](docs/06-comparacion-y-decision.md) —
   la matriz de comparación y la recomendación final, con su razonamiento.
5. [`docs/07-hoja-de-ruta.md`](docs/07-hoja-de-ruta.md) — cómo se secuenciaría
   la construcción, todavía a nivel de planificación.

## Por qué un repositorio aparte

La v1 tiene su propia disciplina de commits, su release, su README dirigido a
recruiters y colegas. Meter acá un proceso de decisión de arquitectura en
plena obra la habría ensuciado. Cuando (y si) esto se convierte en código,
pasa a vivir en `visor-md` como una rama o una v2 real; hasta entonces, la
documentación de la decisión vive sola, versionada, revisable y sin apuro.

## Licencia

Pensado para seguir bajo [GNU GPLv3](../visor md/LICENSE), igual que la v1.
