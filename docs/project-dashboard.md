# Dashboard operativo de Visor MD v2

Última actualización: 16 de septiembre de 2026.

Este documento resume el avance real sin sustituir las especificaciones. Sirve
para responder tres preguntas rápidamente:

1. qué puede hacer una persona hoy;
2. qué está implementado pero aún necesita evidencia o pulido;
3. qué falta para declarar v2.0 distribuible.

No usa un porcentaje único como si todas las tareas costaran lo mismo. Una
función de exportación o una frontera de seguridad puede requerir más trabajo y
riesgo que muchas mejoras pequeñas de interfaz. La lectura correcta es la
capacidad de uso diario más los criterios de cierre de cada fila.

## Lectura rápida

| Área | Estado | Qué significa hoy |
| --- | --- | --- |
| Lector seguro de Markdown | **Sólido, pendiente de QA formal** | Lee Markdown y texto inerte, aplica la sintaxis anunciada, se degrada de forma visible ante límites y no ejecuta contenido. |
| Editor y preservación de archivos | **Funcional, pendiente de cierre** | Permite editar, deshacer, guardar atómicamente, detectar conflictos y preservar UTF-8, BOM y EOL; faltan QA humano y optimizaciones de render incremental. |
| Aplicación de uso diario | **Funcional, parcial** | Tiene pestañas, acciones, cierre protegido, paneles, tema, comparación y chrome propio de Windows. Hasta cuatro documentos distintos pueden verse en divisiones aisladas; faltan QA visual, foco por teclado, ventanas separadas y accesibilidad completa. |
| Workspace y Obsidian esencial | **Funcional, parcial** | Puede indexar una carpeta autorizada, buscar, navegar vínculos y backlinks dentro de VFS; faltan escala, corpus y QA multiplataforma. |
| Estudio y preparación para IA | **Base funcional, parcial** | Ofrece Markdown portable, copias preparadas y fragmentos; faltan comparación entre versiones y controles más finos de fragmentos. |
| Exportación y distribución | **No iniciado como producto** | Hay investigación; PDF, DOCX, paquetes e instaladores todavía no son capacidades de la aplicación. |

**Conclusión de producto:** la aplicación ya es una base de lectura y edición
local utilizable en desarrollo. Aún no corresponde llamarla v2.0 final ni
recomendarla como reemplazo estable para todas las notas: faltan el cierre de
QA, accesibilidad, multitarea completa, exportación y distribución.

## Capacidades comprobadas

Estas funciones tienen implementación y pruebas automáticas; no implica que el
QA humano asociado ya esté cerrado.

| Dominio | Listo ahora | Evidencia principal |
| --- | --- | --- |
| Render Markdown | Encabezados, énfasis anidado, código, listas, tareas, citas, tablas, notas al pie, autolinks, callouts, resaltado y allowlist HTML semántica sin atributos. | [`features.md`](features.md), [`testing.md`](testing.md) |
| Seguridad de contenido | Sin WebView, DOM ni JavaScript; sin red implícita; HTML hostil visible e inerte; límites de entrada y fallback seguro. | [`security.md`](security.md), [`threat-model.md`](threat-model.md) |
| Tipografía y tema | Fuentes embebidas, día/noche, selector Leer/Editar/Comparar con indicador animado y opción de reducir movimiento. | [`design.md`](design.md), [`status.md`](status.md) |
| Edición | Rope UTF-8, IME, selección, pegado explícito, cortar/copiar, undo/redo, CRLF atómico, vista dividida y ayudas Markdown. | [`architecture.md`](architecture.md), [`features.md`](features.md) |
| Integridad de archivos | Límite de apertura, UTF-8 válido, preservación BOM/EOL, guardado atómico, detección de conflicto y recuperación local explícita. | [`security.md`](security.md), [`test-matrix.md`](test-matrix.md) |
| Trabajo diario | Crear, abrir, guardar, pestañas reordenables/fijables, cierre protegido, paleta, menú contextual, barra de estado y tema. | [`features.md`](features.md), [`manual-qa-v2.md`](manual-qa-v2.md) |
| Workspace | Carpeta explícita, VFS contenida, índice en memoria/cancelable, árbol, búsqueda, encabezados, wikilinks, aliases y backlinks. | [`connectivity.md`](connectivity.md), [`editor-workspace-ui-audit.md`](editor-workspace-ui-audit.md) |
| Recursos locales | Placeholder seguro e imagen PNG local solo tras confirmación y validación dentro de la raíz autorizada. | [`security.md`](security.md), [`dependencies.md`](dependencies.md) |
| Calidad actual | 245 pruebas locales, Clippy estricto y SBOM vigente; el release local tras las divisiones mide 3,400 MiB. | [`status.md`](status.md), [`budget.md`](budget.md) |

## Trabajo iniciado pero no cerrado

| Frente | Qué existe | Qué falta para cerrar | Siguiente evidencia útil |
| --- | --- | --- | --- |
| Sprint 1 lector | Modelo, renderer, límites y regresiones. | Corpus CommonMark/GFM consolidado y QA visual de release, tipografía, tablas y HTML permitido. | Recorrido de [`manual-qa-sprint1.md`](manual-qa-sprint1.md). |
| Editor resistente | Buffer, historial, archivos y render asíncrono versionado. | Render Markdown incremental, accesibilidad de IME/selección y QA de diálogos/archivos reales. | Casos Unicode, CRLF, conflicto externo y escritura prolongada de [`manual-qa-v2.md`](manual-qa-v2.md). |
| Chrome y multitarea | Pestañas, comparación fuente/vista y divisiones reales de hasta cuatro documentos, con selector explícito, foco, scroll y raster aislados por hoja. | Foco por teclado, transición con reducir movimiento, ventanas separadas y accesibilidad. | QA de cuatro hojas, foco, scroll, DPI y cierre de hoja frente a cierre de pestaña. |
| Movimiento y estados | Tema y selector de modo animados con ruta instantánea. | Aplicar el mismo criterio a pestañas, paneles y futuras divisiones, sin animar texto ni entrada. | QA con reducir movimiento y ventanas de varios DPI. |
| Workspace | Índice y navegación segura ya funcionan; la búsqueda explica si coincidió por título, ruta, encabezado o contenido. | Escala de bóvedas grandes, actualización granular, corpus adversarial y QA de estados largos/vacíos. | Benchmark sintético y QA de bóveda fixture. |
| Obsidian | Vínculos, aliases, encabezados, callouts y backlinks. | Corpus de bóvedas más amplio, QA real de compatibilidad y recursos secundarios limitados. | Abrir, editar y reabrir fixture sin cambios ajenos. |
| Estudio/IA | Preguntas plegables, callouts de estado, conceptos, resúmenes y fragmentos. | Comparación no destructiva y selección/guardado individual de fragmentos. | Pruebas de preservación y UX de una nota larga. |

## Pendiente por completo o casi por completo

| Bloque | Alcance acordado | Condición para comenzar o cerrar |
| --- | --- | --- |
| Comparación de versiones | Ver diferencias sin modificar el original. | Diseñar un modelo no destructivo que no confunda comparación fuente/vista. |
| Exportación PDF | Salida visual fiel desde el modelo seguro, sin navegador embebido. | Evaluar implementación, licencia, tamaño, advisories y superficie antes de sumar dependencia. |
| Exportación DOCX | Documento útil para universidad/trabajo: estructura antes que fidelidad píxel. | Mismo gate de cadena de suministro y corpus de documentos. |
| Copias de plataforma completas | Discord/correo ya tienen base; faltan flujos de adjunto y refinamiento. | Criterio de formato por destino sin alterar fuente. |
| Distribución Windows/Linux | Paquetes, asociaciones, CI reproducible, notices y matriz release. | QA real por plataforma, especialmente Wayland/Linux. |
| Accesibilidad completa | Semántica accesible, lector de pantalla, foco y contraste validados. | Diseño de capa accesible nativa y QA con tecnologías de asistencia. |
| Fuzzing reproducible | Parser, editor, rutas e imágenes. | Harness limitado y reproducible, sin convertir la suite normal en una prueba lenta. |

## Obstáculos y riesgos activos

| Tipo | Situación | Impacto | Tratamiento actual |
| --- | --- | --- | --- |
| QA humano | Hay un checklist amplio pendiente de ejecutar sobre builds recientes. | No permite declarar cierre visual, DPI, accesibilidad o compatibilidad real. | Se registra en manual QA; no bloquea trabajo independiente. |
| Multitarea | Las divisiones internas ya protegen la propiedad única de cada documento, pero no existe aún una sesión compartida entre ventanas ni foco de panel por teclado. | Impide declarar completa la multitarea y dificulta accesibilidad. | Cerrar QA de las cuatro hojas, añadir foco predecible y diseñar ventanas separadas sobre la misma propiedad. |
| Linux/Wayland | `ttf-parser 0.25.1` llega de forma transitiva por decoraciones Wayland de `winit` y RustSec lo marca no mantenido. | Deuda de suministro para distribución Linux, no vulnerabilidad conocida ni código usado por el release Windows. | Monitorear upstream y revisar antes de publicar Linux; no cambiar el grafo sin QA. |
| Renderer actual | La fuente se actualiza por líneas, pero el Markdown renderizado se deriva de nuevo tras editar. | Puede costar más en documentos extensos. | Mantener límites y versionado; optimizar solo con benchmark que demuestre necesidad. |
| Tamaño y dependencias | El presupuesto permite margen, pero exportación e imágenes pueden aumentarlo rápido. | Riesgo de superar 8 MiB o ampliar superficie. | Toda dependencia nueva pasa por ADR, licencia, advisories, transitivas y medición. |
| Monolito | `src/main.rs` aún concentra renderer, ventana e integración. | Hace más lenta y riesgosa la evolución estructural. | Extraer solo fronteras que desbloqueen una capacidad comprobable, no modularizar por estética. |

## Patrones que se repiten y cómo corregirlos

| Patrón observado | Consecuencia | Regla de corrección |
| --- | --- | --- |
| Declarar una capacidad por tener modelo o tests, antes de que su interacción sea perceptible. | El producto parece más avanzado en documentación que en una ventana real. | Marcar **parcial** hasta que modelo, UI, interacción, seguridad y QA proporcional estén conectados. |
| Corregir una observación visual sin una regresión pequeña. | El mismo defecto puede volver en otro refactor. | Cada fallo reproducible recibe una prueba o un ítem de QA con condición clara. |
| Repetir builds release lentos después de cambios que no afectan el ejecutable. | Consume tiempo sin aumentar evidencia. | Usar `cargo check` y pruebas focalizadas durante iteración; reservar release/gates para hitos sensibles. |
| Actualizar varios documentos con el mismo detalle. | Aparecen contradicciones y el mantenimiento se vuelve pesado. | Dashboard resume y enlaza; cada regla vive en una única fuente normativa. |
| Convertir QA manual pendiente en una pausa global. | Fragmenta el avance autónomo. | Registrar el gate y avanzar en trabajo independiente; exigirlo solo al cerrar el hito afectado. |
| Intentar resolver una función grande con un atajo visual. | Puede duplicar buffers, debilitar guardado o confundir foco. | Para paneles, ventanas, exportación y seguridad, primero fijar propiedad, límites y pruebas. |

## Próximo orden de trabajo

1. cerrar QA focalizado de lectura/edición y de las divisiones visibles;
2. completar foco por teclado, estados de movimiento y lifecycle de paneles;
3. endurecer escala y compatibilidad de workspace/Obsidian;
4. completar comparación de versiones y herramientas de estudio pendientes;
5. diseñar ventanas separadas sobre la propiedad única ya verificada;
6. evaluar exportación aislada y luego distribución profesional.

## Cómo mantenerlo actualizado

Al cerrar un bloque relevante, actualizar solo estas piezas:

1. **Lectura rápida:** si cambia la capacidad que una persona puede usar hoy.
2. **Fila afectada:** mover trabajo de “iniciado” a “comprobado” solo con
   evidencia enlazable.
3. **Obstáculos:** agregar o retirar riesgos con condición concreta, no notas
   vagas.
4. **Estado y métricas:** actualizar [`status.md`](status.md) con pruebas,
   tamaño, gate o medición reproducible.
5. **Fuente normativa:** actualizar la especificación real (`security`,
   `architecture`, `design`, `features` o `roadmap`) antes que este resumen.

El dashboard no reemplaza [`roadmap.md`](roadmap.md): el roadmap decide orden y
criterios; este documento muestra la fotografía y la deuda operativa actual.
