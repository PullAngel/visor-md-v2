# Visor MD v2

Visor MD v2 es una aplicación nativa para leer, editar y estudiar Markdown con
la inmediatez de una herramienta de texto simple, calidad visual editorial y una
postura de seguridad verificable.

Está pensada principalmente para estudiantes y personas que trabajan con
documentos producidos por IA. También busca integrarse de forma cómoda y no
destructiva con bóvedas de Obsidian, repositorios y carpetas de documentación.

El objetivo no es agregar la mayor cantidad posible de funciones. Es construir
una aplicación cotidiana, rápida, bonita y confiable que trate cada documento
como entrada potencialmente hostil.

## Estado del proyecto

El proyecto está en recuperación y estabilización del Sprint 1.

- `main` contiene el estado principal actual, incluido el trabajo validado de
  recuperación de Codex.
- `archive/claude-pre-codex` conserva el estado histórico anterior a Codex y no
  se usa para desarrollo.
- `codex/sprint-1-recovery` queda conservada como rama de trabajo histórica.
- El trabajo heredado ya fue preservado, reconstruido y vuelve a compilar con
  sus pruebas verdes. Sigue siendo una recuperación en revisión, no una versión
  terminada.
- La documentación distingue lo comprobado, lo parcial y lo planificado.

La fotografía verificable del estado se mantiene en
[`docs/status.md`](docs/status.md). El contexto completo del traspaso está en
[`docs/workspace-handoff.md`](docs/workspace-handoff.md).

## Qué quiere ofrecer

- Apertura inmediata de `.md` mediante doble clic.
- Lectura con tipografía y layout cuidados.
- Cambio sencillo entre lectura, fuente y vista dividida.
- Edición fiel que preserve sintaxis que Visor MD no comprenda.
- Selección, copia, menú contextual y atajos de calidad profesional.
- Herramientas de estudio expresadas mediante Markdown portable.
- Trabajo cómodo con documentos creados por IA.
- Wikilinks, backlinks, callouts y navegación de bóvedas existentes.
- Exportación prioritaria a PDF, DOCX y formatos preparados para compartir.
- Soporte seguro de otros textos inertes sin convertirse en un IDE.

## Seguridad por diseño

Visor MD no usa un motor web para representar documentos y no ejecuta HTML,
JavaScript ni contenido incluido en un archivo.

Durante apertura y render normales:

- no hay telemetría;
- no se carga contenido remoto;
- no se siguen rutas locales automáticamente;
- no se ejecutan scripts;
- no se permite que un documento cambie la configuración;
- los límites de recursos degradan a una vista de texto segura.

Algunas acciones pueden habilitarse mediante consentimiento explícito, como
mostrar una imagen remota o confiar temporalmente en una bóveda. Esas excepciones
son delimitadas y nunca habilitan ejecución ni conexiones silenciosas.

La explicación completa está en [`docs/security.md`](docs/security.md) y el
análisis de atacantes, activos y escenarios está en
[`docs/threat-model.md`](docs/threat-model.md).

## Arquitectura

La dirección actual usa Rust y una pila nativa pequeña:

- `winit` para ventana y eventos;
- `softbuffer` para presentar píxeles;
- `tiny-skia` para dibujo 2D;
- `parley` y `swash` para texto;
- `comrak` para parsing Markdown.

No hay WebView, DOM ni runtime JavaScript. El prototipo inicial todavía es
monolítico y no implementa todas las capas documentadas. La recuperación debe
separar modelo documental, parsing, políticas, VFS, layout, rendering, edición y
guardado sin perder el presupuesto de tamaño.

Ver [`docs/architecture.md`](docs/architecture.md).

## Presupuesto

- Menos de 6 MB: resultado extraordinario.
- Alrededor de 7 MB: objetivo normal.
- Menos de 8 MB: límite deseado.
- Más de 8 MB: requiere medición, explicación y aprobación.

La cifra nunca justifica reducir seguridad, estabilidad, accesibilidad, Unicode
o funciones esenciales. Los resultados y el método de medición viven en
[`docs/budget.md`](docs/budget.md).

## Ingeniería verificable

El repositorio trata como entregables de primera clase:

- threat model;
- ADR;
- matriz de pruebas;
- fuzzing;
- benchmarks reproducibles;
- auditoría de dependencias;
- SBOM y licencias;
- evidencia Windows y Linux;
- documentación sincronizada con el código.

La estrategia de QA está en [`docs/testing.md`](docs/testing.md) y la matriz viva
en [`docs/test-matrix.md`](docs/test-matrix.md).

## Documentación

La puerta de entrada es [`docs/README.md`](docs/README.md). Allí se distingue
entre documentos vivos, evidencia de estado e investigación histórica.

Antes de contribuir o utilizar un agente de desarrollo, leer
[`AGENTS.md`](AGENTS.md).

Las contribuciones siguen [`CONTRIBUTING.md`](CONTRIBUTING.md) y los reportes
sensibles siguen [`SECURITY.md`](SECURITY.md).

## Compilación

Requisitos previstos:

- Rust estable con target MSVC en Windows;
- herramientas de compilación nativas de la plataforma;
- dependencias resueltas por Cargo.

Validación completa en Windows:

```powershell
.\scripts\check.ps1
```

El inventario reproducible de componentes se actualiza con:

```powershell
.\scripts\generate-sbom.ps1
```

Los comandos de Cargo también pueden ejecutarse por separado. Consultar
[`docs/status.md`](docs/status.md) antes de interpretar el alcance de una prueba
verde: muchas funciones del producto todavía están planificadas.

## Desarrollo asistido por IA

Visor MD se desarrolla mediante un flujo profesional asistido por IA. Angel
David Durán Erazo define el producto, los criterios, las prioridades, la
seguridad aceptable y las decisiones finales. Los agentes ayudan con
investigación, implementación, revisión y documentación bajo reglas explícitas
de preservación, evidencia y aprobación.

El objetivo del repositorio no es exhibir texto generado, sino demostrar
dirección de producto, ciberseguridad aplicada, QA verificable y capacidad para
llevar software real hasta una distribución profesional.

## Licencia

Visor MD v2 se distribuye bajo GNU GPL v3. Las fuentes y dependencias conservan
sus licencias respectivas y deben documentarse en los notices y la SBOM.
