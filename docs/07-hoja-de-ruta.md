# Hoja de ruta

Todavía a nivel de planificación: describe el orden razonable para
construir esto, no compromete fechas ni implica que ya arrancó.

## Fase 0 — validar el supuesto más caro antes de tocar nada más

Prototipo acotado de la Propuesta B (nativo, solo lo básico: encabezados,
párrafos, listas, tablas). Medir arranque y peso reales. Es la única fase
cuyo resultado puede cambiar el resto del plan, así que va primero y
aislada. Si no justifica el costo de mantener dos renderizadores, se
descarta acá mismo y el resto de la hoja de ruta no cambia en nada.

## Fase 1 — el contrato IPC y la separación de procesos

El corazón de la Propuesta A. Enumerar y tipar cada operación que el
proceso de render puede pedirle al proceso host. Reimplementar la lógica
de contención de rutas de la v1 (`safe_media_path`) del lado del host,
validando cada pedido antes de cruzarlo. Sin esto, nada más de la v2 tiene
sentido — es la base de la que depende la mejora de seguridad principal.

## Fase 2 — paridad funcional con la v1

Portar render, edición, pestañas y ventanas sobre la nueva separación de
procesos, con la suite de pruebas de la v1 (`test_files.py`, `smoke.py`,
`seguridad.py`) corriendo en verde contra la arquitectura nueva antes de
seguir. Ningún requisito nuevo entra en esta fase — es exclusivamente no
perder nada de lo que la v1 ya prueba.

## Fase 3 — bitácora de confianza y aislamiento de diagramas

Las piezas de la Propuesta C que se suman sin fricción sobre A: reemplazar
el interruptor global de carpetas de confianza por la bitácora auditable,
y mover Mermaid a su propio contexto de navegación con CSP propio.

## Fase 4 — workspace y edición estructural

Carpetas persistentes, recientes, favoritos (de ThisIs-Developer).
Renombrado seguro de encabezados, pegar imagen como archivo, autocompletado
de enlaces (de idea-multimarkdown). Es la fase de mayor superficie de
producto nuevo, y la última porque depende de que el workspace tenga ya
una noción estable de "documentos relacionados" que solo existe una vez
que hay carpetas persistentes.

## Fase 5 — arranque percibido

El splash nativo que pinta el texto crudo antes de que WebView2 termine de
inicializar. Se deja para el final a propósito: es una mejora de
percepción, no de arquitectura, y tiene más sentido medirla contra el
sistema ya completo que contra uno a medio construir.

## Lo que no tiene fase asignada todavía

Retomar la Propuesta B a escala completa, si la Fase 0 la justificó. Se
trataría como una iniciativa aparte, con su propia hoja de ruta, no como
una fase más de esta.
