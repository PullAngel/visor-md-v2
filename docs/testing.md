# Estrategia de QA

QA significa aseguramiento de calidad. No consiste solamente en buscar errores al
final. En Visor MD implica definir qué debe ser cierto, construir evidencia y
detener una entrega cuando esa evidencia no alcanza.

## Objetivos

La estrategia debe demostrar:

- que Markdown válido se interpreta correctamente;
- que entrada hostil no ejecuta código ni agota recursos sin límite;
- que abrir y guardar no destruye contenido;
- que la UI sigue siendo rápida y utilizable;
- que Windows y Linux se comportan de forma compatible;
- que las dependencias y licencias son conocidas;
- que una función visible también funciona con teclado, DPI y errores reales.

## Pirámide de pruebas

### Unitarias

Prueban funciones pequeñas y rápidas: políticas de rutas, límites, conversión del
AST, marcadores, rangos y operaciones de edición.

El modelo de historial prueba además que un parche no parte UTF-8, que undo/redo
reconstruyen exactamente el texto, que una edición nueva invalida redo y que el
presupuesto de historial sacrifica únicamente pasos antiguos de undo.

El buffer escalable se prueba también con una edición en el centro de un texto
Unicode de cientos de KiB, vecinos multibyte, CRLF, línea final vacía y
reconstrucción exacta después de undo.

La vista dividida prueba que un resultado de render actualiza la lectura sin
reemplazar los bloques fuente editables, conserva el modo y reserva una
geometría independiente. La percepción durante escritura rápida, DPI y ventanas
estrechas permanece como QA manual.

Son útiles para localizar un fallo, pero no demuestran por sí solas que la
aplicación completa funciona.

### Integración

Prueban varias capas juntas: abrir, parsear, construir el modelo, maquetar y
producir comandos de dibujo. También cubren VFS, guardado y exportadores.

### Corpus

Un corpus es una colección versionada de entradas y resultados esperados.

Se usarán:

- ejemplos oficiales CommonMark aplicables;
- extensiones GFM y Obsidian elegidas;
- documentos reales anonimizados;
- casos históricos de v1;
- entradas patológicas y adversariales.

### Property testing

En lugar de comprobar solo ejemplos concretos, genera muchas variantes y verifica
propiedades. Ejemplo: editar un rango no debe modificar bytes fuera de ese rango.

El núcleo actual también ejecuta un **barrido adversarial determinista**: 128
combinaciones reproducibles de fragmentos Markdown, HTML inerte, Unicode y
estructura incompleta. Es una red de regresión barata que comprueba ausencia de
`panic` en cada suite normal. No se presenta como fuzzing ni sustituye una
campaña de fuzzing con corpus, cobertura y duración registrados.

### Fuzzing

Un fuzzer genera entradas inesperadas continuamente. Busca panic, bloqueos,
consumo excesivo y estados imposibles. No reemplaza casos diseñados a mano.

Campañas prioritarias:

- parser y conversión de AST;
- límites de profundidad;
- rangos y edición;
- rutas, wikilinks y VFS;
- decodificación y dimensiones de imágenes;
- importación y exportación.

### End to end

Prueban recorridos visibles completos, como abrir, cambiar a edición, modificar,
guardar, cerrar y volver a abrir. Deben mantenerse pocas y centradas en flujos
críticos para evitar una suite lenta y frágil.

### QA manual

Se reserva para percepción y entornos difíciles de automatizar:

- calidad tipográfica;
- animaciones;
- selección y menú contextual;
- lector de pantalla;
- IME;
- alto contraste;
- varios DPI y monitores;
- sensación de arranque y scroll.

Cada sesión manual usa una lista corta, registra plataforma y deja resultado. No
se sustituye una prueba automatizable por memoria humana.

## Seguridad dentro de QA

Una prueba de seguridad debe verificar la propiedad relevante.

Ejemplos:

- No basta con que una URL no se vea. Se monitorean sockets para demostrar que no
  hubo conexión.
- No basta con que `..` sea rechazado como texto. Se prueban symlinks, junctions,
  cambios de archivo y rutas UNC.
- No basta con que 5.000 citas no produzcan panic. Se mide cancelación, tiempo,
  memoria y entrada al modo seguro.
- No basta con escapar `<script>`. Se comprueba que ningún nodo HTML no permitido
  llegue al renderer como comportamiento activo.

Esta diferencia es importante en ciberseguridad: probar apariencia comprueba lo
que se observa; probar una propiedad comprueba lo que el sistema puede hacer.

## Gates por cambio

`AGENTS.md` decide el nivel de riesgo. Esta sección describe las evidencias que
corresponden a cada dominio; no exige una auditoría completa para un cambio que
no puede invalidarla.

| Tipo de cambio | Mínimo | Evidencia adicional cuando aplica |
| --- | --- | --- |
| Documentación | enlaces locales y `git diff --check` | ADR o estado si cambia una decisión |
| UI aislada | formatter, lint y tests cercanos | QA visual, teclado, DPI o accesibilidad |
| Parser o Markdown | formatter, lint, tests y corpus afectado | patologías, rangos, property test o fuzzing |
| Rendering o fuentes | layout y tests cercanos | regresión visual, Unicode, rendimiento |
| VFS, rutas o guardado | unitarios e integración | traversal, symlinks, fallos, round-trip |
| Workspace e índice | unitarios e integración | cambios y borrados en subcarpetas, límites y resultados tardíos |
| Red, HTML, enlaces o imágenes | casos positivos y negativos | ausencia de red, evasiones y phishing |
| Dependencias | build, tests y SBOM vigente | audit, deny, licencias, transitivas y tamaño |
| Release o milestone | gate local completo | auditoría, benchmark, matriz manual y riesgos |

En Windows, `scripts/check.ps1` ejecuta el gate local completo: formatter,
Clippy, tests, SBOM, enlaces de documentación y build release. `-SkipRelease`
sirve para ciclos de código rápidos; no reemplaza release al cerrar un bloque
sensible. Para una edición puramente documental, `scripts/check-docs.ps1` y
`git diff --check` son la evidencia normal, salvo que cambie una decisión que
requiera auditoría.

El workflow [`.github/workflows/ci.yml`](../.github/workflows/ci.yml) repite
formato, Clippy, pruebas y release en Windows MSVC y Linux. Windows verifica
además que el SBOM versionado se pueda regenerar sin diferencias. No publica
artefactos ni sustituye QA manual, fuzzing o auditoría de advisories.

Además:

- cambios Markdown: corpus y patologías;
- filesystem: matriz de rutas y guardado;
- rendering: regresión visual, DPI y selección;
- dependencias: audit, deny, licencias, SBOM y tamaño;
- rendimiento: benchmark antes y después;
- seguridad: caso positivo, negativo y forma de evasión.

Las series de arranque y scroll se recogen con
`scripts/benchmark-startup.ps1`. El reporte conserva muestras crudas para que un
promedio o una mediana no oculten outliers.

## Evidencia de release

Una release candidata necesita:

- commit y toolchain identificados;
- CI verde en Windows y Linux;
- tests unitarios, integración y corpus;
- campaña de fuzzing registrada;
- matriz manual completada;
- benchmark reproducible;
- SBOM y notices;
- advisories resueltos o aceptados;
- threat model y documentación sincronizados;
- lista explícita de riesgos residuales.

## Tratamiento de fallos

Un test que descubre un defecto real no se elimina ni se debilita para recuperar
el color verde. Primero se determina si la especificación, el test o el código es
incorrecto. Toda regresión importante debe dejar una prueba que falle antes del
arreglo y pase después.

Los tests antiguos se recompilan. Un ejecutable anterior no demuestra el estado
del working tree actual.

## Checkpoint de recuperación

La suite cubre que una escritura de recuperación atrasada no sustituya la más
nueva y que una tarea pendiente no pueda recrear un snapshot después de
limpiarlo.

El 25 de agosto de 2026 el working tree de recuperación alcanzó:

- 40 pruebas unitarias y adversariales verdes en Windows MSVC;
- formato verde;
- `clippy` sin advertencias permitidas;
- build release verde;
- fixture manual [`../tests/fixtures/sprint1-visual.md`](../tests/fixtures/sprint1-visual.md).

El corpus de integración
[`../tests/fixtures/commonmark-gfm-reader.md`](../tests/fixtures/commonmark-gfm-reader.md)
acompaña la sintaxis que el lector representa hoy. No sustituye la suite oficial
CommonMark: evita que una ampliación futura declare soporte de una construcción
que no conserva modelo, rangos y representación.

El corpus incorpora una selección trazable a CommonMark 0.31.2: ejemplos 16
(salto forzado) y 20 (autolink con escape). Se verifican contra el modelo nativo
de Visor MD, no contra HTML: la aplicación no es un navegador. La selección se
ampliará de acuerdo con las capacidades que lleguen al renderer; no se declarará
conformidad completa hasta ejecutar una suite oficial compatible.

Además de la fixture, `casos_commonmark_gfm_anunciados_llegan_a_layout` cubre
casos pequeños de salto forzado, autolink escapado, énfasis anidado, listas,
tareas, tablas, código, HTML semántico permitido y HTML inerte. Para cada caso
comprueba parser, rangos del modelo y geometría finita con las fuentes reales.
Es deliberadamente un catálogo de capacidades propias, no una afirmación de
compatibilidad total con CommonMark.

La fixture no cuenta como prueba aprobada hasta registrar una inspección visual.
Sirve para repetir siempre el mismo documento con temas, énfasis, listas, tareas,
citas, tabla, código, HTML inerte, emoji y fallback Unicode.

El recorrido y los datos que deben registrarse están en
[`manual-qa-sprint1.md`](manual-qa-sprint1.md).

La recuperación ya portó dos regresiones conceptuales de v1: HTML hostil y
Markdown defectuoso de conversores. Se adaptaron a la arquitectura nativa; no se
copiaron sus aserciones sobre DOM o WebView, que ya no existen en v2.

La regresión de concurrencia dirige cada resultado de guardado por identidad de
documento. La prueba fija que una respuesta destinada a una pestaña inactiva no
puede mutar la activa; las revisiones siguen determinando si el resultado
guardado representa también la edición más reciente.

Apertura y render aplican la misma regla: identidad de pestaña más solicitud o
revisión vigente. Cambiar de pestaña no invalida una tarea correcta, mientras
que una respuesta vieja para el mismo documento se descarta.
