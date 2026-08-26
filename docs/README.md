# Documentación de Visor MD v2

Esta carpeta contiene especificaciones vivas, evidencia verificable e
investigación histórica. No todos los documentos tienen la misma autoridad.

## Lectura recomendada

Para comprender el proyecto en poco tiempo:

1. [`../AGENTS.md`](../AGENTS.md): reglas operativas y decisiones que no deben
   cambiarse accidentalmente.
2. [`status.md`](status.md): qué funciona realmente y qué está incompleto.
3. [`product.md`](product.md): usuarios, experiencias y alcance.
4. [`architecture.md`](architecture.md): dirección técnica y distancia respecto
   de la implementación actual.
5. [`security.md`](security.md): políticas de seguridad explicadas y controles.
6. [`roadmap.md`](roadmap.md): orden de recuperación y desarrollo.
7. [`testing.md`](testing.md): estrategia de QA y evidencia necesaria.
8. [`glossary.md`](glossary.md): jerga de ciberseguridad y QA en lenguaje
   natural.

La plantilla reutilizable para iniciar otros repositorios está en
[`../AGENT_WORKFLOW_TEMPLATE.md`](../AGENT_WORKFLOW_TEMPLATE.md). No es una
instrucción activa hasta adaptarla y renombrarla deliberadamente.

## Documentos vivos

Estos documentos deben cambiar cuando cambia el producto o su implementación:

| Documento | Responsabilidad |
| --- | --- |
| [`product.md`](product.md) | Promesa, usuarios, flujos, alcance y exclusiones |
| [`vision.md`](vision.md) | Identidad y criterios que resuelven conflictos |
| [`features.md`](features.md) | Catálogo mantenible y estado de funciones |
| [`architecture.md`](architecture.md) | Capas, contratos y estado real |
| [`security.md`](security.md) | Políticas, controles y explicación educativa |
| [`threat-model.md`](threat-model.md) | Activos, atacantes, amenazas y riesgo residual |
| [`connectivity.md`](connectivity.md) | Disco, red, Obsidian, GitHub y confianza |
| [`testing.md`](testing.md) | Estrategia de QA y definición de gates |
| [`test-matrix.md`](test-matrix.md) | Cobertura verificable por riesgo y plataforma |
| [`manual-qa-sprint1.md`](manual-qa-sprint1.md) | Recorrido visual reproducible del Sprint 1 |
| [`dependencies.md`](dependencies.md) | Auditorías, grafo, advisories y deuda de suministro |
| [`glossary.md`](glossary.md) | Conceptos técnicos explicados de forma breve |
| [`budget.md`](budget.md) | Tamaño, arranque, memoria y reproducción |
| [`benchmarks/`](benchmarks/) | Reportes crudos de rendimiento versionados |
| [`roadmap.md`](roadmap.md) | Secuencia, dependencias y criterios de salida |
| [`design.md`](design.md) | Sistema visual e interacción |
| [`decisions.md`](decisions.md) | ADR y decisiones reemplazadas |
| [`future.md`](future.md) | Ideas fuera del plan activo |

## Estado y evidencia

| Documento | Uso |
| --- | --- |
| [`status.md`](status.md) | Fotografía breve y actual del proyecto |
| [`workspace-handoff.md`](workspace-handoff.md) | Auditoría y traspaso histórico detallado |
| [`../assets/fonts/README.md`](../assets/fonts/README.md) | Procedencia y reproducción tipográfica |

El estado vivo debe actualizarse al cerrar cada bloque relevante. El handoff solo
se modifica cuando aparece evidencia histórica que cambia la reconstrucción.

## Investigación histórica

La carpeta [`research/`](research/) conserva alternativas y comparaciones que
ayudaron a elegir la arquitectura. Sus afirmaciones sobre competidores y costes
son fotografías temporales, no especificaciones actuales.

[`study-brainstorm.md`](study-brainstorm.md) conserva ideas y explicaciones de
estudio. Cuando contradiga `product.md`, `features.md` o un ADR posterior,
prevalece la decisión viva más reciente.

[`inference.md`](inference.md) registra una exploración histórica de IA local.
La decisión actual es no incorporar IA propia en Visor MD.

## Regla de mantenimiento

La documentación acompaña el trabajo, no se corrige meses después.

Al cambiar comportamiento:

1. actualizar la especificación afectada;
2. actualizar threat model o ADR si cambia un límite o decisión;
3. agregar o actualizar la prueba correspondiente;
4. registrar evidencia y estado;
5. evitar duplicar la misma verdad en muchos documentos.

Usar enlaces hacia la fuente autorizada en lugar de copiar párrafos enteros. Si
dos documentos vivos se contradicen, detener el cambio y resolver la autoridad.
