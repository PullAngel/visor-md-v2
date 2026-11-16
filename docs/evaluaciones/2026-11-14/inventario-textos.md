# Inventario de textos de la interfaz

Fecha: 16 de noviembre de 2026. Tarea 7.1 del plan de ejecución. Generado desde
el código después de aplicar el glosario (D-25) y los avisos (D-26); la prueba
`src/lenguaje.rs` lo mantiene al día: falla si aparece un término retirado o un
imperativo de tuteo en las etiquetas, los diálogos o los avisos.

## Reglas

- Voseo en todo: «Escribí», «Elegí», «Abrí», «Guardá».
- Glosario (D-25): carpeta de apuntes, notas que enlazan aquí, recuadro, enlace
  a otra nota, guardado, copiar para hoja de cálculo, título / subtítulo,
  recuperación.
- Un aviso confirma algo que no se ve (guardado, copiado, exportado) o explica
  un error. No se avisa al abrir ni cerrar un panel o un menú: la ayuda va en el
  pie del panel.
- Los registros (`self.log`) quedan fuera.

## Cambios de esta tarea

| Antes | Ahora |
| --- | --- |
| workspace, espacio de trabajo, carpeta de trabajo | carpeta de apuntes |
| Backlinks | Notas que enlazan aquí |
| enlace de bóveda | enlace a otra nota |
| callout | recuadro |
| TSV | copiar para hoja de cálculo |
| encabezado | título |
| «documento guardado de forma atómica» | «Guardado» |
| recuperación local, recuperación local temporal | recuperación |
| «escribe», «elige», «activa edición», «selecciona», «espera» | «escribí», «elegí», «activá Editar», «seleccioná», «esperá» |
| Avisos al abrir y cerrar paneles («… · Escape cierra») | Quitados; el pie del panel ya dice «Esc para cerrar» |
| Aviso de 2,2 s + 35 ms por letra, centrado en la ventana | 2,4 s (más 35 ms por letra pasadas las 40), centrado en el área del documento a 24 px del borde inferior |

## Etiquetas de acciones (`AppAction::label`)

- Nuevo documento · Ctrl+N
- Abrir documento · Ctrl+O
- Guardar · Ctrl+S
- Guardar como · Ctrl+Shift+S
- Cerrar pestaña · Ctrl+W
- Cerrar panel sin cerrar la pestaña
- Fijar o soltar pestaña
- Alternar lectura y edición · F2
- Comparar fuente y vista · F3
- Alternar disposición de comparación
- Dividir a la derecha con otra pestaña
- Dividir abajo con otra pestaña
- Cambiar tema día o noche · Ctrl+Shift+D
- Activar o desactivar movimiento reducido
- Fijar la ventana encima de las demás
- Insertar símbolo o emoji
- Mostrar u ocultar el índice flotante
- Exportar a PDF
- Cortar · Ctrl+X
- Aplicar negrita · Ctrl+B
- Aplicar cursiva · Ctrl+I
- Aplicar tachado
- Aplicar código en línea
- Insertar enlace · Ctrl+K
- Título 1 (H1)
- Título 2 (H2)
- Título 3 (H3)
- Título 4 (H4)
- Título 5 (H5)
- Título 6 (H6)
- Lista con viñetas
- Lista numerada
- Tarea
- Cita
- Insertar bloque de código
- Insertar tabla Markdown
- Insertar regla horizontal
- Resaltar texto
- Insertar enlace a otra nota
- Insertar recuadro
- Párrafo: títulos, listas y citas
- Insertar enlaces y bloques
- Estudio y notas
- Insertar pregunta de estudio
- Insertar resumen de estudio
- Insertar lista de conceptos
- Marcar contenido como entendido
- Marcar contenido como dudoso
- Marcar contenido como pendiente
- Preparar resumen desde selección
- Preparar conceptos desde selección
- Preparar fragmentos para IA
- Copiar para Discord o correo
- Plegar o desplegar sección enfocada
- Buscar en documento · Ctrl+F
- Copiar tabla para hoja de cálculo
- Abrir carpeta · Ctrl+Shift+O
- Actualizar índice de carpeta · Ctrl+Shift+I
- Cancelar actualización de carpeta
- Buscar en carpeta · Ctrl+Shift+F
- Índice del documento · Ctrl+Shift+L
- Notas de la carpeta · Ctrl+Shift+T
- Carpeta de apuntes
- Notas que enlazan aquí · Ctrl+Shift+B
- Revisar enlaces a otras notas
- Abrir recuperación · Ctrl+Shift+R
- Activar o desactivar recuperación
- Buscar una acción · Ctrl+Shift+P
- Archivo
- Editar
- Vista
- Deshacer · Ctrl+Z
- Rehacer · Ctrl+Y
- Copiar · Ctrl+C
- Pegar · Ctrl+V
- Carpeta
- Menú
- simbolo
- Símbolo

## Diálogos (`src/dialog.rs`)

- `dialog.rs:91` Recuperación activada
- `dialog.rs:92` Visor MD guarda una copia de lo que no guardaste en tu perfil de usuario, para recuperarlo tras un cierre inesperado. La copia es texto sin cifrar, no reemplaza a Guardar y nunca se escribe dentro de tu carpeta de apuntes.
- `dialog.rs:104` ¿Desactivar la recuperación?
- `dialog.rs:105` La recuperación guarda copias sin cifrar de los documentos con cambios. Sin ella, un cierre inesperado puede perder lo que no guardaste. Al desactivarla se borran las copias de esta sesión.
- `dialog.rs:113` Mantener activa
- `dialog.rs:124` El archivo cambió fuera de Visor MD mientras tenías cambios sin guardar. Si recargás, tu versión queda en una recuperación antes de reemplazarla.
- `dialog.rs:126` El archivo cambió fuera de Visor MD. Podés recargar la versión nueva o seguir con la que ves ahora.
- `dialog.rs:129` El archivo cambió en el disco
- `dialog.rs:133` Guardar una copia
- `dialog.rs:138` Mantener la mía
- `dialog.rs:154` El archivo ya no está disponible
- `dialog.rs:155` Se borró, dejó de ser un archivo normal o ya no se puede leer. Tu documento sigue abierto e intacto aquí.
- `dialog.rs:158` Seguir editando
- `dialog.rs:163` Guardar una copia
- `dialog.rs:174` No se guardó: el archivo cambió
- `dialog.rs:175` Otro programa modificó el archivo después de que lo abriste, así que no se sobrescribió. Si recargás, tu versión queda antes en una recuperación.
- `dialog.rs:178` Recargar la externa
- `dialog.rs:183` Mantener la mía
- `dialog.rs:188` Guardar una copia
- `dialog.rs:199` Si cerrás sin guardar, los cambios quedan en una recuperación que podés abrir después.
- `dialog.rs:201` La recuperación está desactivada: si cerrás sin guardar, estos cambios se pierden.
- `dialog.rs:204` ¿Guardar los cambios de «{name}»?
- `dialog.rs:208` Cerrar sin guardar
- `dialog.rs:230` • {name}
- `dialog.rs:234` \n… y {} más
- `dialog.rs:236` Tenés cambios sin guardar en:
- `dialog.rs:238` Si cerrás sin guardar, quedan en recuperaciones que podés abrir al volver.
- `dialog.rs:240` La recuperación está desactivada: si cerrás sin guardar, esos cambios se pierden.
- `dialog.rs:243` ¿Cerrar Visor MD?
- `dialog.rs:247` Cerrar sin guardar
- `dialog.rs:253` Guardar todo
- `dialog.rs:264` ¿Mostrar esta imagen local?
- `dialog.rs:265` Se leerá una sola imagen PNG dentro de la carpeta de apuntes, con límites de 8 MiB y 16 millones de píxeles. El permiso no se recuerda.
- `dialog.rs:267` No cargar
- `dialog.rs:269` Mostrar imagen
- `dialog.rs:282` Esta nota no existe
- `dialog.rs:284` «{name}» todavía no está en tu carpeta de apuntes. Podés crearla vacía y empezar a escribirla.
- `dialog.rs:288` Ahora no
- `dialog.rs:292` Crear nota
- `dialog.rs:301` ¿Descartar lo recuperado?
- `dialog.rs:303` {summary} Si lo descartás, ese texto se borra de esta computadora y no se puede volver a abrir.
- `dialog.rs:324` No hay carpeta de apuntes
- `dialog.rs:326` Para abrir enlaces a otras notas hace falta una carpeta de apuntes. ¿Usás la carpeta «{folder}», donde está este archivo?
- `dialog.rs:329` Ahora no
- `dialog.rs:331` Usar esta carpeta

## Avisos (`src/main.rs`, 202)

- `main.rs:8415` la recarga no reemplazó cambios nuevos
- `main.rs:8466` otra pestaña terminó de abrirse
- `main.rs:8497` no se pudo recargar; la edición actual sigue intacta
- `main.rs:8500` no se pudo abrir el documento
- `main.rs:8509` no se pudo abrir otra pestaña
- `main.rs:8557` vista dividida actualizada + vista de lectura actualizada
- `main.rs:8567` otra pestaña actualizó su vista de lectura
- `main.rs:8598` no se pudo actualizar la vista; la fuente sigue intacta + otra pestaña no pudo actualizar su vista; la fuente sigue intacta
- `main.rs:8624` Guardado
- `main.rs:8639` otra pestaña terminó de guardarse
- `main.rs:8643` se guardó una versión anterior; hay cambios nuevos + otra pestaña guardó una versión anterior
- `main.rs:8668` falló el guardado de otra pestaña; sus cambios siguen intactos
- `main.rs:8680` el archivo pudo haberse escrito, pero no se pudo verificar; no se reintentó y la edición local sigue protegida
- `main.rs:8717` Guardado
- `main.rs:8732` otra pestaña terminó de guardarse
- `main.rs:8736` se creó una versión anterior; hay cambios nuevos + otra pestaña guardó una versión anterior
- `main.rs:8778` Carpeta de apuntes {}: {}, {}{suffix} + actualizada + indexada + nota + notas + omitida + omitidas
- `main.rs:8802` No se pudo abrir la carpeta de apuntes
- `main.rs:8817` La carpeta de apuntes cambió: Ctrl+Shift+I actualiza el índice
- `main.rs:8873` el destino externo volvió a estar disponible
- `main.rs:8878` no se pudo comprobar si el archivo cambió fuera de Visor MD
- `main.rs:8915` PNG local cargado: {} × {} · Escape para cerrar
- `main.rs:9032` La recuperación de cambios no está disponible en esta cuenta: guardá seguido
- `main.rs:9038` Quedaron {} sin guardar de la última sesión: abrilos con Ctrl+Shift+R + documento + documentos
- `main.rs:9370` el sistema no habilitó redimensionar desde ese borde
- `main.rs:9453` el sistema no habilitó mover la ventana desde esa zona
- `main.rs:9630` La vista previa es de lectura: editá la fuente en el otro panel
- `main.rs:10447` comparación {}
- `main.rs:10538` Tema de noche · Ctrl+Shift+D para volver + Tema de día · Ctrl+Shift+D para volver
- `main.rs:10635` ventana fijada encima de las demás · clic en la chincheta para soltarla + la ventana ya no queda encima de las demás
- `main.rs:10650` no se pudo preparar el PDF: {error}
- `main.rs:10662` exportación a PDF cancelada
- `main.rs:10673` el PDF no puede reemplazar el documento abierto
- `main.rs:10697` PDF exportado: {file} · {pages} {} + página + páginas
- `main.rs:10704` no se pudo escribir el PDF; el documento no cambió
- `main.rs:10712` la preferencia del índice flotante no pudo guardarse + índice flotante activado · aparece en lectura si hay espacio al costado + índice flotante oculto
- `main.rs:10755` la preferencia de movimiento no pudo guardarse
- `main.rs:10757` movimiento reducido activado
- `main.rs:10759` movimiento reducido desactivado
- `main.rs:10845` No se pudo guardar la copia de recuperación: guardá el documento
- `main.rs:10866` Recuperación activada
- `main.rs:10879` Recuperación desactivada
- `main.rs:10883` la preferencia de recuperación no pudo guardarse
- `main.rs:11170` la ventana ya muestra el máximo de cuatro documentos
- `main.rs:11182` dividir {} · documento nuevo, abrir un archivo o una pestaña abierta
- `main.rs:11199` documento nuevo en el panel dividido
- `main.rs:11218` esa pestaña ya no está disponible para dividir
- `main.rs:11226` la pestaña elegida dejó de existir
- `main.rs:11235` no se pudo crear un panel sin repetir documentos
- `main.rs:11241` panel dividido {} · el nuevo documento se enfoca con un clic
- `main.rs:11255` la ventana ya muestra el máximo de cuatro documentos
- `main.rs:11271` no se pudo crear el panel
- `main.rs:11294` abrir en panel cancelado
- `main.rs:11304` ese documento ya estaba abierto; se activó su pestaña
- `main.rs:11317` activá Editar para insertar símbolos en el Markdown
- `main.rs:11340` activá Editar para insertar símbolos en el Markdown
- `main.rs:11344` {name} insertado · Ctrl+Z para deshacer
- `main.rs:11398` división de panel cancelada
- `main.rs:12028` paneles intercambiados
- `main.rs:12058` panel dividido {}
- `main.rs:12060` no se pudo crear un panel sin repetir documentos
- `main.rs:12191` solo hay un documento abierto
- `main.rs:12196` no se pudo identificar la pestaña activa
- `main.rs:12212` la pestaña seleccionada ya no está disponible
- `main.rs:12255` este es el único panel visible
- `main.rs:12261` no se pudo cerrar el panel actual
- `main.rs:12266` el panel restante no tiene un documento válido
- `main.rs:12274` la pestaña del panel restante ya no está disponible
- `main.rs:12298` panel cerrado · la pestaña continúa abierta
- `main.rs:12306` la pestaña está fijada; soltala antes de cerrarla
- `main.rs:12310` Esperá a que termine la operación de esta pestaña antes de cerrarla
- `main.rs:12363` se guardó, pero hay cambios nuevos: la pestaña sigue abierta
- `main.rs:12420` pestaña cerrada
- `main.rs:12441` pestaña fijada para esta sesión + pestaña liberada
- `main.rs:12462` edición de fuente · F2 para volver a lectura
- `main.rs:12468` no se pudo preparar la fuente: {error}
- `main.rs:12505` vista dividida · fuente editable y lectura sincronizada
- `main.rs:12511` no se pudo preparar la fuente: {error}
- `main.rs:12546` no se pudo actualizar la fuente: {error}
- `main.rs:12550` edición rechazada: {error:?}
- `main.rs:12556` Activá Editar para aplicar formato al Markdown
- `main.rs:12564` Activá Editar para insertar un enlace en el Markdown
- `main.rs:12572` Activá Editar para cambiar la estructura del Markdown
- `main.rs:12581` Activá Editar para cambiar el nivel del título
- `main.rs:12596` Activá Editar para insertar un bloque de código
- `main.rs:12605` Activá Editar para insertar una tabla
- `main.rs:12615` Activá Editar para insertar una regla horizontal
- `main.rs:12624` Activá Editar para insertar un recuadro
- `main.rs:12637` Activá Editar para insertar una pregunta de estudio
- `main.rs:12647` Activá Editar para insertar un resumen de estudio
- `main.rs:12657` Activá Editar para insertar una lista de conceptos
- `main.rs:12687` no se pudieron preparar fragmentos seguros
- `main.rs:12692` fragmentos preparados en una pestaña nueva · no se modificó el original
- `main.rs:12723` Seleccioná texto antes de preparar una nota de estudio
- `main.rs:12729` no se pudo preparar la selección
- `main.rs:12741` Seleccioná texto antes de preparar una nota de estudio
- `main.rs:12761` no se pudo preparar un resumen seguro
- `main.rs:12766` {label} preparados en una pestaña nueva · no se modificó el original
- `main.rs:12773` Activá Editar para marcar el estado de estudio
- `main.rs:13156` no se pudo acceder al portapapeles
- `main.rs:13166` el portapapeles no contiene texto utilizable
- `main.rs:13173` Esperá a que termine la recarga antes de guardar
- `main.rs:13184` no hay cambios para guardar
- `main.rs:13194` esta pestaña ya se está guardando
- `main.rs:13198` Guardando
- `main.rs:13220` Esperá a que termine la recarga antes de guardar una copia
- `main.rs:13229` guardar como cancelado
- `main.rs:13238` esta pestaña ya se está guardando
- `main.rs:13242` Guardando
- `main.rs:13357` No se pudo recargar: la recuperación no está disponible, guardá una copia
- `main.rs:13364` no se pudo conservar una recuperación; no se recargó el archivo externo
- `main.rs:13471` el documento de la pregunta ya no está activo; no se cambió nada
- `main.rs:13476` Recuperación activada
- `main.rs:13478` La recuperación sigue activa
- `main.rs:13483` se mantiene tu versión; el archivo del disco no se tocó
- `main.rs:13495` seguís editando; no se cerró nada
- `main.rs:13503` imagen local no cargada
- `main.rs:13543` No se cerró: no se pudo guardar la copia de recuperación
- `main.rs:13566` Esperá a que termine la operación actual antes de cerrar
- `main.rs:13608` no se cerró: no se pudieron conservar todas las recuperaciones
- `main.rs:13644` El cierre se detuvo: «{name}» no se guardó y sigue abierto
- `main.rs:13665` El cierre se detuvo: un documento no se guardó y sigue abierto
- `main.rs:13677` documento nuevo · Ctrl+Shift+S para elegir destino
- `main.rs:13706` abrir documento cancelado
- `main.rs:13718` abrir carpeta cancelado
- `main.rs:13730` Elegí primero una carpeta de apuntes con Ctrl+Shift+O
- `main.rs:13794` no hay una actualización de carpeta en curso
- `main.rs:13800` actualización de carpeta cancelada; se conserva el índice anterior
- `main.rs:13807` No hay documentos sin guardar para recuperar
- `main.rs:13856` No se pudo recuperar «{name}»: {reason}
- `main.rs:13863` No se pudo mostrar «{name}»: {error}
- `main.rs:14002` {} descartado{} + documento + documentos +  + s
- `main.rs:14014` no se puede recargar un documento que todavía no se guardó
- `main.rs:14019` esta pestaña ya se está recargando
- `main.rs:14029` recargando desde disco
- `main.rs:14068` El documento ya está abierto en esta pestaña
- `main.rs:14078` se activó la pestaña que ya contenía el documento
- `main.rs:14137` cargando documento
- `main.rs:14215` sin texto seleccionado
- `main.rs:14237` sin texto seleccionado
- `main.rs:14242` no se pudo copiar la selección de fuente
- `main.rs:14255` Seleccioná contenido antes de preparar una copia
- `main.rs:14284` Seleccioná una celda de la tabla antes de copiar
- `main.rs:14300` no se pudo acceder al portapapeles
- `main.rs:14324` no se pudo copiar: el portapapeles está ocupado
- `main.rs:14362` cortar solo está disponible durante la edición
- `main.rs:14367` Seleccioná texto para cortar
- `main.rs:14373` la selección no corresponde a texto UTF-8 válido
- `main.rs:14382` texto cortado · Ctrl+Z para deshacer
- `main.rs:14419` este documento no tiene enlaces
- `main.rs:14468` Enfocá un enlace con Tab antes de abrirlo
- `main.rs:14495` el enlace local no declara un destino
- `main.rs:14518` el enlace local fue bloqueado por la política de archivos
- `main.rs:14522` Abrí una carpeta de apuntes para seguir enlaces locales
- `main.rs:14528` destino bloqueado por la política de seguridad
- `main.rs:14534` Abriendo en el navegador: {}
- `main.rs:14539` Abriendo tu programa de correo
- `main.rs:14544` no se pudo abrir el enlace
- `main.rs:14556` imagen remota o ruta no permitida bloqueada; no se realizó conexión
- `main.rs:14563` solo se pueden previsualizar PNG locales contenidos
- `main.rs:14567` Abrí una carpeta de apuntes antes de cargar una imagen local
- `main.rs:14571` Guardá la nota dentro de la carpeta de apuntes antes de cargar imágenes
- `main.rs:14596` cargando PNG local con límites de seguridad
- `main.rs:14649` El enlace a otra nota no indica ninguna nota ni título
- `main.rs:14668` El enlace a otra nota es ambiguo: usá una ruta más precisa
- `main.rs:14672` El enlace a otra nota fue bloqueado por la política de archivos
- `main.rs:14688` El enlace a otra nota fue bloqueado por la política de archivos
- `main.rs:14714` Guardá esta nota para poder abrir enlaces a otras notas
- `main.rs:14733` Nota creada: {name}
- `main.rs:14737` Ya hay una nota con ese nombre: actualizá la carpeta
- `main.rs:14740` Ese nombre no sirve para una nota: usá uno simple, sin barras
- `main.rs:14744` No se pudo crear la nota
- `main.rs:14800` Abrí una carpeta de apuntes para ver qué notas enlazan aquí
- `main.rs:14804` El documento abierto no se puede asociar a la carpeta de apuntes
- `main.rs:14808` El documento abierto no está en la carpeta de apuntes
- `main.rs:14812` el documento abierto no está indexado como una nota Markdown
- `main.rs:14827` Ninguna nota de la carpeta enlaza a este documento
- `main.rs:14838` Abrí una carpeta de apuntes para revisar los enlaces a otras notas
- `main.rs:14848` Enlaces a otras notas: {found} resueltos, {missing} ausentes, {ambiguous} ambiguos, {blocked} bloqueados
- `main.rs:14884` La carpeta de apuntes ya no está disponible
- `main.rs:14894` La nota fue bloqueada por la política de archivos
- `main.rs:14925` El documento no tiene títulos
- `main.rs:15002` El documento no tiene el título pedido
- `main.rs:15033` Elegí primero una carpeta de apuntes con Ctrl+Shift+O
- `main.rs:15052` Elegí primero una carpeta de apuntes con Ctrl+Shift+O
- `main.rs:15057` La carpeta de apuntes no tiene notas Markdown indexadas
- `main.rs:15137` carpeta del árbol actualizada
- `main.rs:15141` La carpeta de apuntes ya no está disponible
- `main.rs:15151` la nota fue bloqueada por la política de archivos
- `main.rs:15218` no hay notas que coincidan con la búsqueda
- `main.rs:15231` la nota encontrada fue bloqueada por la política de archivos
- `main.rs:15420` el plegado de secciones está disponible en lectura
- `main.rs:15448` Enfocá un título para plegar su sección
- `main.rs:15474` sección plegada; la fuente permanece intacta + sección desplegada
- `main.rs:15499` respuesta plegada; el Markdown permanece intacto + respuesta desplegada
- `main.rs:15670` la vista previa se está actualizando; probá de nuevo en un momento
- `main.rs:15679` no se pudo localizar el bloque de código en la fuente
- `main.rs:15698` no se pudo localizar el bloque de código en la fuente
- `main.rs:15713` no se pudo localizar la tarea en la fuente
- `main.rs:15717` la tarea no conserva un marcador editable
- `main.rs:15727` tarea actualizada · Ctrl+Z para deshacer
- `main.rs:15732` Seleccioná una tarea para marcarla con Espacio

## Avisos (`src/panel_list.rs`, 0)

