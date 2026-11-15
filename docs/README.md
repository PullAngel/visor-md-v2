# Documentación de Visor MD v2

Esta carpeta contiene especificaciones vivas, evidencia verificable e
investigación histórica. No todos los documentos tienen la misma autoridad.

## Lectura recomendada

Para comprender el proyecto en poco tiempo:

1. [`project-dashboard.md`](project-dashboard.md): avance, deuda, riesgos y
   próximo orden de trabajo en una sola vista.
   Para retomar el trabajo, leer después
   [`revision-2026-11-13.md`](revision-2026-11-13.md) y, para la interfaz,
   [`evaluacion-usabilidad-2026-11-14.md`](evaluacion-usabilidad-2026-11-14.md),
   con el plan definitivo vigente.
2. [`product.md`](product.md): usuarios, experiencias y alcance.
3. [`architecture.md`](architecture.md): dirección técnica y distancia respecto
   de la implementación actual.
4. [`threat-model.md`](threat-model.md) y [`security.md`](security.md): riesgos,
   controles y límites.
5. [`testing.md`](testing.md) y [`test-matrix.md`](test-matrix.md): estrategia de
   QA y evidencia por riesgo.

Para una revisión de seguridad o QA, empezar por esos dos últimos puntos y
contrastar los controles con `src/`, las fixtures y el workflow de CI. Para
seguir el desarrollo, consultar [`status.md`](status.md) y
[`roadmap.md`](roadmap.md). Las instrucciones para agentes están en
[`../AGENTS.md`](../AGENTS.md); la jerga se explica en
[`glossary.md`](glossary.md).

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
| [`manual-qa-v2.md`](manual-qa-v2.md) | Checklist integral manual, end to end y adversarial para un build candidato |
| [`manual-qa-sprint1.md`](manual-qa-sprint1.md) | Recorrido visual reproducible del Sprint 1 |
| [`dependencies.md`](dependencies.md) | Auditorías, grafo, advisories y deuda de suministro |
| [`glossary.md`](glossary.md) | Conceptos técnicos explicados de forma breve |
| [`budget.md`](budget.md) | Tamaño, arranque, memoria y reproducción |
| [`benchmarks/`](benchmarks/) | Reportes crudos de rendimiento versionados |
| [`roadmap.md`](roadmap.md) | Secuencia, dependencias y criterios de salida |
| [`project-dashboard.md`](project-dashboard.md) | Fotografía compacta de avance, deuda, riesgos y siguiente orden |
| [`revision-2026-11-13.md`](revision-2026-11-13.md) | Revisión integral: causa raíz de defectos abiertos, brecha con el concepto C y orden de trabajo vigente |
| [`rediseno-barra-mosaico-ventanas-2026-11-15.md`](rediseno-barra-mosaico-ventanas-2026-11-15.md) | Diseño de la barra y los menús (inventario de las 69 acciones), el mosaico, las ventanas y la asociación de archivos; origen de D-29 a D-37 y las fases 10 a 12 |
| [`evaluacion-usabilidad-2026-11-14.md`](evaluacion-usabilidad-2026-11-14.md) | Evaluación con QA de UI/UX y usuaria novata (SUS, heurísticas, 44 hallazgos verificados) y plan definitivo de cambios vigente |
| [`agente-ejecutor.md`](agente-ejecutor.md) | Guía operativa para ejecutar el plan sin acompañamiento: ciclo por tarea, verificación, pruebas visuales, reglas de parada y trampas del código |
| [`plan-ejecucion-2026-11-14.md`](plan-ejecucion-2026-11-14.md) | Plan de ejecución del rediseño: tareas por fase con función, prueba y criterio de terminado; estado vigente |
| [`herramientas-rediseno-2026-11-14.md`](herramientas-rediseno-2026-11-14.md) | 13 herramientas evaluadas para el rediseño; elegidas: plugin Design de Anthropic y pruebas de imagen de referencia |
| [`revision-interfaz-2026-11-14.md`](revision-interfaz-2026-11-14.md) | Paneo de interfaz (listas, alternancia de botones, pestañas, hover y pulsación) y plan de cambios por fases vigente |
| [`design.md`](design.md) | Sistema visual e interacción |
| [`design-audit.md`](design-audit.md) | Evidencia y plan vigente de corrección visual |
| [`ui-exploration-brief.md`](ui-exploration-brief.md) | Prompts y criterios para comparar propuestas visuales externas sin convertirlas en especificación |
| [`design-proposals/`](design-proposals/) | Exploraciones visuales y dirección elegida en septiembre de 2026 |
| [`ui-workspace-implementation.md`](ui-workspace-implementation.md) | Contrato trazable y secuencia del workspace visual elegido |
| [`editor-workspace-ui-audit.md`](editor-workspace-ui-audit.md) | Auditoría y secuencia UX de edición, índice y bóvedas |
| [`research/08-export-evaluation.md`](research/08-export-evaluation.md) | Evaluación de dependencias y aislamiento para PDF/DOCX antes de incorporarlas |
| [`decisions.md`](decisions.md) | ADR y decisiones reemplazadas |
| [`future.md`](future.md) | Ideas fuera del plan activo |
| [`android.md`](android.md) | Dirección y preparación de una futura versión móvil |

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
