# AGENTS.md

## Propósito y alcance

Visor MD v2 es una aplicación nativa para leer y editar Markdown y otros
archivos de texto inerte. Debe sentirse inmediata como un editor de texto
simple, ofrecer lectura editorial y permitir trabajar con documentos producidos
por IA, estudio y bóvedas existentes de Obsidian.

No es un navegador, un IDE, un chatbot ni un reemplazo completo de Obsidian.
No incorporar funciones solo porque un competidor las tenga.

## Prioridades no negociables

En caso de conflicto, evaluar explícitamente:

1. seguridad y ausencia de pérdida de datos;
2. corrección y estabilidad;
3. velocidad de apertura y respuesta;
4. bajo consumo de recursos;
5. experiencia de lectura y edición;
6. accesibilidad y Unicode;
7. superficie de ataque y dependencias reducidas;
8. tamaño del binario;
9. mantenibilidad y calidad visual.

El binario ideal pesa menos de 6 MB, el objetivo normal ronda 7 MB y el límite
deseado es menos de 8 MB. Superarlo exige medición, explicación y aprobación.
Nunca reducir seguridad, estabilidad, accesibilidad, Unicode o funciones
esenciales para alcanzar una cifra.

## Relación con el propietario

El propietario dirige el producto. Estudia redes y ciberseguridad y quiere
aprender durante el desarrollo. Explicar seguridad y QA con rigor en lenguaje
natural: definir jerga cuando ayude, conectar conceptos con riesgos, pruebas o
ejemplos de Visor MD y separar hechos medidos, inferencias y preferencias.

Los agentes deben cuestionar decisiones que contradigan estos principios,
explicar alternativas y señalar qué es difícil de revertir. Trabajar con
autonomía dentro del alcance aprobado, pero no cambiar silenciosamente producto,
arquitectura, publicación, dependencias estructurales o políticas de seguridad.

Resolver ambigüedades reversibles con la opción conservadora y documentarla.
Detenerse ante decisiones difíciles de revertir que afecten producto, datos,
seguridad, compatibilidad, arquitectura, dependencias o UX fundamental.

Dar actualizaciones breves y periódicas durante el trabajo, agrupadas por
resultado útil. Un commit, una prueba verde o una herramienta completada no son
por sí mismos un motivo para detener la ejecución ni pedir respuesta. Pedir QA
manual solo cuando la percepción humana aporte evidencia que no puede obtenerse
automáticamente.

Mientras exista trabajo seguro, aprobado y útil dentro del sprint activo,
continuar de forma autónoma. Detener la ejecución solo ante un bloqueante real:
decisión del propietario, cambio difícil de revertir de producto, seguridad o
UX, dependencia relevante, riesgo de pérdida de datos, permiso externo o QA que
solo una persona pueda realizar.

Un gate manual, visual o de plataforma pendiente bloquea únicamente la decisión
o el hito que necesita esa evidencia. Debe registrarse y cerrarse antes de
declarar terminado el sprint o milestone correspondiente, pero no impide tareas
automatizables e independientes del objetivo aprobado.

## Comunicación y estilo

En código, comentarios, tests y commits usar lenguaje humano, concreto y
funcional. No usar emojis decorativos, muletillas de IA ni comentarios que
describan lo evidente. El README puede explicar el flujo profesional asistido
por IA y la dirección humana del producto; evitar repetir ese metadiscurso en el
resto del repositorio.

## Invariantes permanentes

### Documento y producto

- Markdown es el formato principal; otros textos permanecen inertes.
- Conservar texto no editado y sintaxis desconocida tanto como sea posible.
- No reformatear, normalizar ni reescribir silenciosamente al guardar.
- El modelo debe preservar relación entre fuente, semántica y representación.
- Una sintaxis no está soportada solo porque el parser la reconozca: necesita
  modelo, layout, rendering, interacción coherente y pruebas.
- La interfaz debe ser sobria, editorial, accesible y de divulgación progresiva,
  nunca sobrecargada.

### Plataforma y capacidades

- La arquitectura objetivo es nativa, sin WebView, DOM ni JavaScript.
- La apertura y el render normal no realizan conexiones de red implícitas.
- Un documento nunca ejecuta scripts ni activa comportamiento por sí mismo.
- El contenido del documento no obtiene acceso a archivos, red o configuración
  fuera de capacidades concedidas explícitamente por la persona usuaria.
- Las entradas que exceden límites deben degradar de forma visible y segura, sin
  ocultar contenido ni perderlo silenciosamente.

Los detalles de política no se duplican aquí: consultar la documentación del
dominio afectado antes de cambiar una frontera de confianza.

## Economía de implementación

Preferir la solución correcta más pequeña que satisfaga el requisito actual.

Antes de crear código o infraestructura:

1. comprobar si la capacidad ya existe;
2. reutilizar o extender una implementación existente cuando sea claro;
3. preferir stdlib y dependencias presentes si son adecuadas;
4. evitar abstracciones, configuración o generalización para casos hipotéticos;
5. no implementar funciones futuras "por si acaso";
6. no agregar dependencias si una solución pequeña, mantenible y clara basta.

Eliminar o simplificar código es preferible a agregar infraestructura cuando
ambas opciones cumplen. La simplicidad nunca justifica pérdida de datos, menor
seguridad, errores ocultos, menor accesibilidad o comportamiento implícito.

## Fuentes de autoridad y contexto progresivo

Antes de modificar:

1. leer este archivo;
2. revisar rama, HEAD, `git status` y el diff relevante;
3. clasificar el riesgo del cambio;
4. leer solo la documentación y ADR del dominio afectado;
5. inspeccionar implementación y pruebas relacionadas;
6. informar contradicciones relevantes antes de decidir.

La documentación puede estar desactualizada y el código puede estar incompleto.
No asumir que uno de los dos representa la verdad sin comprobarlo. Las decisiones
explícitas del propietario prevalecen sobre documentación histórica, salvo que
contradigan seguridad, integridad de datos o restricciones fundamentales.

| Si se modifica... | Consultar como mínimo... |
| --- | --- |
| producto, UX o alcance | `docs/product.md`, `docs/design.md`, `docs/roadmap.md` |
| parser, Markdown, HTML o límites | `docs/security.md`, `docs/threat-model.md`, `docs/testing.md` |
| rutas, archivos, VFS, guardado o vaults | `docs/security.md`, `docs/connectivity.md`, `docs/architecture.md` |
| layout, rendering, fuentes o accesibilidad | `docs/design.md`, `docs/architecture.md`, `docs/testing.md` |
| rendimiento, memoria o tamaño | `docs/budget.md`, `docs/architecture.md` |
| dependencias, toolchain o código nativo | `docs/dependencies.md`, `docs/security.md`, ADR relacionados |
| decisiones históricas o trabajo heredado | `docs/workspace-handoff.md`, `docs/decisions.md` |

`docs/workspace-handoff.md` es evidencia histórica: consultarlo al reconstruir
contexto, no como lectura obligatoria de cada tarea.

## Verificación proporcional al riesgo

Clasificar por el comportamiento afectado, no solo por el archivo tocado. Cuando
una tarea cubra varios niveles, aplicar el mayor.

### Nivel 1: cambio normal

Ejemplos: documentación, textos, estilo aislado, UI sin frontera de confianza,
bug local o refactor interno pequeño.

- ejecutar formatter, lint/typecheck y pruebas directamente afectadas cuando
  corresponda;
- usar `scripts/check-docs.ps1` y `git diff --check` para documentación;
- hacer QA manual focalizado si la percepción visual importa;
- construir release solo si el cambio puede afectar el ejecutable, distribución,
  rendimiento o si cierra un bloque relevante.

### Nivel 2: cambio sensible

Aplica a parser, modelo documental, HTML, links, imágenes, filesystem, VFS,
guardado, codificaciones, límites, red, datos no confiables, dependencias,
código nativo, `unsafe` o actualizaciones.

- cumplir Nivel 1;
- identificar propiedades de seguridad, preservación o rendimiento afectadas;
- ejecutar pruebas específicas y añadir regresión para cada defecto real;
- medir build release, tamaño o rendimiento cuando pueda cambiar;
- actualizar la especificación y threat model cuando cambie una política o
  frontera de confianza;
- para dependencias, revisar licencia, advisories, transitivas, superficie y
  SBOM según `docs/dependencies.md`.

### Nivel 3: auditoría o milestone

Aplica a releases, hitos, cambios arquitectónicos, fronteras de confianza,
capacidades de red, cambios grandes de dependencias o investigación de un
hallazgo de seguridad.

- ejecutar la suite amplia y gates de `docs/testing.md`;
- actualizar o revisar SBOM, advisories, benchmarks, matriz y documentación;
- revisar threat model, riesgos residuales y QA manual focalizado;
- registrar evidencia reproducible. No repetir una auditoría amplia si ningún
  cambio pudo invalidarla.

`scripts/check.ps1` es el gate local completo de Windows. Es útil al cerrar
bloques sensibles o milestones; no reemplaza el juicio de riesgo ni obliga a
recompilar release para una corrección puramente documental.

## Arquitectura y dependencias

La dirección actual usa Rust, `winit`, `softbuffer`, `tiny-skia`, `parley`,
`swash` y `comrak`. La implementación presente es un prototipo en separación;
la arquitectura objetivo no debe confundirse con código ya existente. Consultar
`docs/architecture.md` antes de reorganizaciones relevantes.

No cambiar dependencias, toolchain, licencia, arquitectura ni políticas de
seguridad sin explicar impacto y obtener la aprobación correspondiente. Las
decisiones duraderas, de formato persistente, compatibilidad o dependencia
estructural requieren ADR práctico. No crear ADR para detalles locales.

## Git y preservación

`main` representa el último estado estable. Trabajar en ramas `codex/...` o la
rama acordada.

Antes de tocar archivos, distinguir cambios heredados de cambios propios y
preservar trabajo ajeno. No restaurar, resetear, borrar, mover ni sobrescribir
sin autorización explícita.

No integrar como producto snapshots de recuperación, backups, artifacts de
diseño, binarios de medición ni temporales. Los commits deben ser pequeños,
coherentes y compilables; no mezclar recuperación, refactor, función nueva y
actualización masiva de dependencias. En una rama de trabajo aprobada, crear y
publicar commits coherentes al cerrar cada bloque técnico; hacerlo no cierra la
tarea ni exige informar cada hash de inmediato. Comunicar hashes agrupados al
cerrar un hito, ante una revisión o si ayudan a recuperar un estado.

## Documentación y cierre

La documentación es evidencia operativa. Al cambiar comportamiento, actualizar
la fuente normativa afectada, pruebas, estado o ADR cuando corresponda. Evitar
duplicar reglas: usar enlaces hacia la autoridad temática de `docs/README.md`.

Una tarea está terminada cuando:

- satisface su objetivo;
- aplica el nivel de verificación apropiado;
- no deja estructuras parcialmente conectadas;
- no modifica trabajo ajeno accidentalmente;
- documentación y comportamiento coinciden;
- la deuda aceptada queda registrada;
- el estado Git es entendible y comunicable.

Para cambios sensibles deben existir pruebas de las propiedades afectadas. Para
milestones y releases se exige auditoría amplia. "Funciona en mi máquina" y "no
hizo crash" nunca bastan como única evidencia.

Un bloque interno puede estar terminado y versionado sin que la solicitud global
lo esté. No emitir cierre final mientras queden pasos seguros del roadmap o del
objetivo explícito del propietario; seguir con el siguiente bloque coherente.
