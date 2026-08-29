# Catálogo de funciones

Este catálogo describe producto y estado sin confundir intención con
implementación.

Estados:

- `Estable`: presente en el último commit estable y medido.
- `Parcial`: existe una base estable, pero falta evidencia o corrección importante.
- `Recuperación`: existe trabajo local incompleto.
- `Planificado`: pertenece al plan activo, pero todavía no está implementado.
- `Futuro`: idea válida fuera del plan inmediato.
- `Descartado`: no pertenece al producto actual.

La evidencia detallada vive en [`status.md`](status.md) y
[`test-matrix.md`](test-matrix.md).

## Núcleo de lectura

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Apertura de `.md` por argumento | Estable | Muestra ventana y contenido |
| Parsing Markdown básico | Estable | Casos actuales del prototipo |
| Tema claro y oscuro | Estable | Sistema y alternancia manual |
| Tipografía embebida | Parcial | Reproducción verificada; falta matriz Unicode y fallback |
| Virtualización inicial | Parcial | Debe eliminar recorridos O(n) por frame |
| Formato inline real | Parcial | Modelo, layout y dibujo nativos; falta corpus y QA visual sistemáticos |
| Listas, citas y reglas | Parcial | Casos anidados cubiertos; falta corpus y QA visual ampliados |
| Task list checkboxes | Parcial | Dibujo nativo, clic reversible y pruebas; falta QA de plataforma |
| CommonMark aplicable | Parcial | Corpus versionado de sintaxis soportada y política HTML; falta suite oficial seleccionada |
| GFM elegido | Parcial | Tablas, tachado, tareas, autolinks y notas al pie; falta corpus sistemático |
| Índice de encabezados | Parcial | `Ctrl+Shift+L` muestra y enfoca encabezados; falta panel filtrable y accesible |
| Plegado de secciones | Planificado | Sin perder posición ni selección |
| Búsqueda en documento | Parcial | `Ctrl+F` local, Unicode sin distinción de mayúsculas, resultados y navegación; faltan marcas de todas las coincidencias y QA visual |
| Vista de texto segura | Parcial | Fallback por límites y HTML inerte; falta QA end to end completo |

## Edición y archivos

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Modo fuente | Parcial | Selección, IME, undo y atajos; falta buffer escalable y split |
| Vista dividida | Planificado | Correspondencia estable fuente y render |
| Guardado atómico | Parcial | Sin corrupción ante fallo y conflictos probados; falta QA multiplataforma |
| Preservar sintaxis desconocida | Parcial | Fuente no se reserializa; falta property testing de round-trip |
| Detectar cambios externos | Parcial | Conflicto visible al guardar y al recuperar foco; falta QA multiplataforma |
| Recuperación de sesión | Parcial | Activa por defecto, separada por pestaña, versionada y desactivable con advertencia; falta QA de cierre inesperado |
| Crear documento | Parcial | Crea una pestaña nueva sin reemplazar el documento activo; falta diálogo visual de destino |
| Varios documentos y pestañas | Parcial | Barra visible con orden estable, cambio y cierre individual por mouse/teclado, estado, historial, scroll y recuperación separados y cierre global protegido; falta accesibilidad completa |
| Menú contextual | Parcial | Copia y pegado explícito según modo; faltan acciones de aplicación |
| Paleta de comandos | Parcial | `Ctrl+Shift+P` recorre por teclado el catálogo compartido de acciones esenciales; falta búsqueda por nombre y mouse |
| Paneles plegables | Parcial | Índice, notas y backlinks muestran listas acotadas con selección visible y se cierran con Escape; falta mouse y panel de búsqueda múltiple |
| `.txt` y otros textos inertes | Parcial | Extensiones no Markdown se muestran como texto inerte; falta QA y reconocimiento UX |
| Edición en vivo | Futuro | Solo tras editor y modelo estables |

## Seguridad

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Sin WebView ni JavaScript | Estable | Grafo de dependencias y runtime |
| Sin red durante uso normal | Parcial | Falta prueba automatizada de sockets |
| Límites de anidamiento | Recuperación | Tiempo, memoria y fallback |
| VFS central | Parcial | Workspace contenido y archivo principal limitado; recursos secundarios todavía no se abren |
| Política de rutas | Parcial | UNC, traversal, symlinks y junctions cubiertos para workspace; faltan recursos secundarios |
| Allowlist HTML semántica | Recuperación | `br`, `kbd`, `mark`, `sub` y `sup` nativos sin atributos; falta corpus sistemático y QA visual |
| Límites de imágenes | Planificado | Tipo, bytes, dimensiones y memoria |
| Consentimiento remoto | Planificado | Aislado, explícito y revocable |
| Confianza temporal de bóveda | Planificado | Solo amplía acceso local delimitado |
| Auditoría de dependencias | Parcial | Audit realizado, deny y SBOM pendientes |

## Selección y accesibilidad

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Selección con mouse | Parcial | Selección visible con autoscroll y copia; falta QA visual completo |
| Selección con teclado | Parcial | Flechas, Inicio/Fin, RePág/AvPág, Ctrl+A, Escape y extensiones con Shift; falta foco y atajos por línea reales |
| Copiar bloque o documento | Parcial | `Ctrl+C` copia texto visible; `Ctrl+Shift+C` copia Markdown de bloques completos |
| Barra de estado | Parcial | Muestra modo, cambios sin guardar y estado de carpeta sin ocupar herramientas permanentes |
| Alto contraste | Planificado | Matriz Windows y Linux |
| Reduce motion | Planificado | Todas las transiciones respetan preferencia |
| IME | Planificado | Escritura de idiomas compatibles |
| Unicode y fallback | Planificado | Corpus multilingüe |
| Lector de pantalla | Planificado | Semántica accesible demostrada |

## Trabajo con IA

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Copiar Markdown de un bloque | Parcial | `Ctrl+Shift+C` conserva la fuente de bloques completos |
| Fragmentar documento largo | Planificado | No corta estructuras de forma destructiva |
| Comparar versiones | Planificado | Diferencias legibles y no destructivas |
| Archivo listo para adjuntar | Planificado | Markdown portable |
| Copia para Discord o correo | Planificado | Resultado previsible |
| Estimación de tokens | Futuro condicionado | Solo si el coste es insignificante |
| IA propia | Descartado | No pertenece a Visor MD |

## Obsidian y workspace

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Abrir carpeta o bóveda | Parcial | Sin migración ni cambios implícitos; una marca de cambio al recuperar foco sugiere `Ctrl+Shift+I` para volver a indexar la raíz elegida |
| Lista de notas | Parcial | `Ctrl+Shift+T` recorre rutas ya indexadas y abre solo tras resolverlas dentro de VFS; falta panel plegable y estado de disco |
| Búsqueda de bóveda | Parcial | `Ctrl+Shift+F` consulta el índice en memoria y abre solo notas contenidas; faltan panel y resultados múltiples visibles |
| Wikilinks | Parcial | Resolución contenida y explícita; falta panel de diagnóstico |
| Backlinks | Parcial | `Ctrl+Shift+B` muestra y navega backlinks contenidos; faltan panel plegable e incrementalidad |
| Callouts | Parcial | Sintaxis Obsidian elegida y render nativo; falta corpus de bóvedas |
| Etiquetas y frontmatter | Planificado | Lectura sin reescritura |
| Link peek | Planificado | Reusa renderer con límites |
| Referencias de bloque | Futuro | Requiere identidad estable |
| Grafo visual | Futuro | Solo con valor demostrado |

## Estudio

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Resaltado `==texto==` | Parcial | Render nativo y fuente preservada; falta corpus de compatibilidad y QA visual |
| Sidecar para datos no portables | Planificado | Formato versionado y recuperable |
| Preguntas y respuestas | Planificado | Sintaxis portable elegida |
| Ocultar respuesta | Planificado | Accesible y sin alterar fuente |
| Estados de aprendizaje | Planificado | Convención visible y editable |
| Lista de conceptos | Planificado | Exportable |
| Repaso espaciado simple | Planificado tardío | Estado separado y sincronización segura |
| Pomodoro | Futuro | Solo si no distrae ni infla producto |

## Contenido enriquecido

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| Imágenes locales | Planificado | VFS, límites y errores visibles |
| Imágenes remotas confirmadas | Planificado | Componente de red aislado |
| Copiar bloque de código | Parcial | Acción visible y explícita; falta QA visual y de plataformas destino |
| Resaltado de código | Planificado | Sin runtime o dependencia desproporcionada |
| Matemática | Futuro | Componente opcional seguro |
| Mermaid nativo | Futuro | Sin servicio remoto ni scripts |
| HTML arbitrario | Descartado | Se muestra inerte |

## Exportación y distribución

| Función | Estado | Criterio mínimo |
| --- | --- | --- |
| PDF fiel | Planificado | Tipografía, paginado y seguridad |
| DOCX | Planificado con investigación | Compatibilidad y coste cerrados |
| Copia para plataformas | Planificado | Discord y correo priorizados |
| HTML autónomo | Futuro | Sin recursos remotos ni scripts |
| Instalador Windows | Planificado | Firma, asociación y desinstalación |
| Paquete Linux | Planificado | Integración de escritorio y licencias |
| Actualizaciones | Futuro | Explícitas, firmadas y sin telemetría |

## Reglas contra crecimiento accidental

Una función no entra solo porque un competidor la tenga. Debe:

1. resolver un caso de uso aprobado;
2. respetar el threat model;
3. tener criterio de finalización;
4. justificar dependencias y tamaño;
5. tener pruebas proporcionales al riesgo;
6. encajar en la interfaz sin volverla sobrecargada.
