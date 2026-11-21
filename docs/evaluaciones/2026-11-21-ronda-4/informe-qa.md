# Informe de QA manual de UI/UX: Visor MD

Fecha de la evaluación: 21/11/2026 (prompt de la ronda del 14/11/2026). Evaluadora: QA manual de UI/UX. Fuentes: la hoja de requerimientos y la app en ejecución. No se leyó código ni documentación.

## 1. Resumen ejecutivo

**Calificación UX: 7,5 / 10.**

Visor MD tiene una identidad visual coherente («Papel y tinta»). Los dos temas se leen bien y los menús, diálogos y avisos son legibles y están en español. El diálogo de cierre es claro y Esc elige la opción segura. La paleta de acciones, el copiar con confirmación y el resaltado de enlaces rotos son aciertos reales. Los problemas se concentran en tres zonas. Una es el descubrimiento: iconos sin texto y tooltips que aparecen lejos del puntero. Otra es el feedback: el estado de «pulsado» casi no se ve y la confirmación de Copiar dura menos de un segundo. La tercera es la coherencia: los interruptores se escriben de varias maneras y las cuentas de palabras y caracteres cambian según el modo.

**Los 5 problemas principales**

1. **QA-01.** La barra de formato inserta `**texto**` al principio de la línea sin separarlo del contenido. Sobre un título lo rompe (`**texto**# Mis apuntes`). Un estudiante que no conoce Markdown no entiende el resultado.
2. **QA-02.** Los tooltips (incluida la vista previa del destino de un enlace, R14) aparecen anclados abajo a la izquierda o debajo de la barra, no junto al puntero, y tapan contenido (por ejemplo la primera tarea). No hay cambio de estilo en el enlace al pasar el puntero.
3. **QA-03.** Los controles de la barra de herramientas son iconos sin texto, y varios son ambiguos (dos «H», el icono de «>_» para la paleta, `N/` para otro formato). El tooltip solo aparece tras un retraso y a veces no aparece.
4. **QA-04.** Los contadores de la barra de estado cambian al cambiar de modo: 13774 caracteres en Editar contra 13484 en Leer; 107 palabras contra 102 en `inicio.md`. El dato parece poco fiable.
5. **QA-05.** Los interruptores se muestran de forma inconsistente: «Tema de noche: activado ✓», «Movimiento reducido: desactivado» (sin marca) y «Abrir archivos externos…: activado ✓». Con el tema de día activo, el menú dice «Tema de noche: desactivado», lo que es confuso. Además hay avisos que repiten el título del selector recién abierto («Dividir abajo · documento nuevo, abrir un archivo…»).

**Conteo de hallazgos:** S1: 0 · S2: 3 · S3: 13 · S4: 6 (22 en total).

**Conteo de la matriz:** de 31 requerimientos, 22 Cumple, 8 Cumple parcialmente, 0 No cumple y 1 No verificable (R23 por teclado, por las limitaciones de la herramienta: no llegan Ctrl, Alt ni Shift).

## 2. Alcance y entorno

- App: Visor MD, ejecutable `qa.exe`, perfil limpio, controlada solo con `ui.ps1`.
- Tamaños probados: 1100×720 (principal), 640×480 (mínimo) y ventana maximizada (1366×738 capturados, la pantalla disponible).
- Temas: noche (inicio y mayor parte de la sesión) y día. Se cambió con el botón de la barra superior.
- Material usado: carpeta `apuntes\` (`historia.md`, `inicio.md`, `manual-largo.md`, `biologia\celula.md`, `biologia\fotosintesis.md`).
- Interacciones: unas 150 acciones de ratón y teclado, y unas 135 capturas en `capturas\`.
- Pruebas hechas: primer arranque, abrir carpeta, abrir archivos desde el panel, los tres modos, edición de listas, formato, tabla, símbolos y emojis, casillas de tarea, plegado, búsqueda en el documento, paneles de Archivos, Índice, Buscar y «Enlazan aquí», Copiar, menú contextual, pestañas (nueva, cerrar, reordenar, fijar, menú contextual), mosaico (derecha/abajo), diálogo de cierre, fijar ventana, exportar PDF, Guardar como, recuperación y paleta de acciones.
- Medición de contraste: se muestrearon píxeles de mis capturas.
- No se pulsó ningún enlace web. El portapapeles se leyó solo tras un Copiar de la app (contenido de los apuntes, correcto).
- Límites de la herramienta, no reportados: no hay Ctrl/Alt/Shift, `type` no escribe mayúsculas ni tildes, no se ven diálogos nativos.
- No verificado: arrastrar una pestaña a otra ventana, «Mover pestaña a otra ventana», exportar PDF a revisión visual, recuperación tras cierre forzado, drag and drop externo, foco con la rueda sobre el panel de Índice flotante.

## 3. Matriz de requerimientos

| ID | Resultado | Evidencia |
| --- | --- | --- |
| R1 | Cumple | `historia.md` e `inicio.md` se abren formateados en Leer (capturas 16, 94). |
| R2 | Cumple | Archivo > Nuevo documento; escribir; Guardar como a `prueba-qa.md`; aviso «Guardado», título de pestaña cambia (46, 129). |
| R3 | Cumple parcialmente | Los tres modos funcionan y Comparar muestra fuente y vista (45). La sincronización fina falla: en `historia.md` la sección plegada («Directorio») aparece desplegada en la vista de Comparar y hay un hueco irregular entre párrafos de ambas columnas. En Editar, el editor ocupa 740 px fijos alineados a la izquierda. |
| R4 | Cumple parcialmente | Abrir, cambiar, cerrar, reordenar por arrastre (119) y fijar (123) funcionan. Cada pestaña recuerda el modo. No se comprobó la posición de lectura de forma controlada. Abrir un archivo desde el panel reemplazó la pestaña «Bienvenida» sin avisar (15, 16). A 640 px los títulos se truncan y chocan con la X (132). |
| R5 | Cumple | «Dividir abajo» con otra pestaña (87); cada panel con su selector de modo y botón «Quitar del mosaico» (91b). No se probó el límite de 4 paneles. |
| R6 | Cumple parcialmente | Existe barra con negrita, cursiva, tachado, código, resaltado, títulos, listas, tarea, cita, tabla, enlace, bloque, símbolos y emojis (107, 109, 111). La negrita inserta `**texto**` al inicio de la línea, mal colocada (QA-01). Los iconos no tienen texto. |
| R7 | Cumple parcialmente | Enter continúa «- » y un segundo Enter en el elemento vacío termina la lista (48, 49). Tras terminarla queda una línea en blanco de más (líneas 3 y 4). |
| R8 | Cumple parcialmente | El botón Copiar copia solo el código (portapapeles verificado: `ping 192.168.0.1` y `git status`). La confirmación «✓ Copiado» aparece pero dura menos de un segundo (99 sí, 99b ya no). En un intento (96, 0,4 s después) no se vio. No hay aviso. |
| R9 | Cumple | Clic en la casilla la marca y avisa «Tarea actualizada · sin guardar: Ctrl+S la guarda, Ctrl+Z la deshace» (38). |
| R10 | Cumple parcialmente | La flecha pliega y despliega (41). La sección plegada no muestra qué o cuánto oculta, y se pierde el estado en Comparar (R3). |
| R11 | Cumple | Panel Índice con «7 títulos» (22). Índice flotante a 1366 px y también a 1100 px (106, 124). Falta un aviso al activar la opción. |
| R12 | Cumple parcialmente | Resultados resaltados y recorrido con Enter/Mayús+Enter y flechas, con «2 de 6» (35, 37). La coincidencia actual casi no se distingue de las demás, y el estado y el texto buscado comparten un solo campo (QA-10). |
| R13 | Cumple | Elegir carpeta, árbol, «Buscar» y «Enlazan aquí» existen. Enlaces `[[nota]]` funcionan (hover «enlace a otra nota: inicio»). El aviso de indexado es alarmante (QA-12). |
| R14 | Cumple parcialmente | El destino real se muestra al pasar el puntero («enlace web: https://es.wikipedia.org/wiki/Markdown», 100). El enlace roto se distingue con subrayado punteado amarillo y el tooltip «Esta nota no existe… clic para crearla» (101). El tooltip tapa contenido y aparece lejos del puntero (QA-02). Los colores azul y amarillo salen del acento verde único. |
| R15 | Cumple parcialmente | La paleta se abre, filtra, y lista 77 acciones con rueda y teclado (61, 63, 68). Se abre con un icono «>_», no con «···» como indica la hoja (QA-09). |
| R16 | Cumple | Clic derecho sobre tarea: «Marcar o desmarcar tarea · Espacio», Copiar texto, Copiar Markdown original, Copiar para Discord o correo, Herramientas de estudio (39). En pestañas: Dividir, Fijar o soltar, Cerrar (81, 120). No se probó sobre tablas. |
| R17 | Cumple parcialmente | El cambio de tema funciona y avisa «Tema de día · Ctrl+Shift+D para volver» (56). La opción de movimiento reducido existe pero no se pudo verificar su efecto. |
| R18 | Cumple | La chincheta fija y avisa; el botón queda marcado (54). No se comprobó con otras ventanas encima. |
| R19 | Cumple | Archivo > Exportar a PDF generó `export.pdf` (94 KB) y avisó «PDF exportado: export.pdf · 1 página» (127). |
| R20 | Cumple | Diálogo «¿Guardar los cambios de «uno»?» con tres botones nombrados (Cerrar sin guardar, Cancelar, Guardar). Esc cancela y mantiene la pestaña (50, 53). |
| R21 | Cumple parcialmente | Existe «Recuperación de cambios: activado ✓» y «Abrir recuperación», que avisó «No hay documentos sin guardar para recuperar» (130). No se pudo provocar un cierre brusco. |
| R22 | Cumple parcialmente | La barra muestra palabras, caracteres, línea, columna, modo y guardado. Los contadores cambian con el modo (QA-04). «1 palabras» (plural incorrecto, 49). En Leer, línea y columna desaparecen. |
| R23 | No verificable | Foco visible con Tab/F6 sí se vio (71, 73), pero no llegan Ctrl, Alt ni Shift, por lo que no se pudo probar ningún atajo. Se verificó que los atajos estén comunicados en menús, tooltips y avisos. |
| N1 | Cumple parcialmente | A 640×480 no hay controles encimados: los menús se agrupan en una hamburguesa y se pierden los textos de Estudio e Insertar (75, 76). La paleta «>_» desaparece de la barra. Los títulos de pestaña se truncan (132). Maximizada (1366×738) funciona, pero el contenido queda a la izquierda y deja un gran espacio vacío (104). |
| N2 | Cumple parcialmente | Los botones de la barra, el menú y las filas muestran hover. Los enlaces, el botón Copiar y la chincheta tienen hover mínimo o nulo. El estado pulsado casi no se aprecia. |
| N3 | Cumple | Archivo, Vista y las paletas se cierran con un segundo clic (05, 66). Los paneles de carpeta e índice también (22, 23, 24, 25), y el botón queda subrayado y resaltado mientras está abierto. |
| N4 | Cumple | La paleta se recorre con rueda y flechas («31–44 de 77», 68). La rueda funciona en documentos largos (59, 60) y el menú Estudio tiene barra de desplazamiento. |
| N5 | Cumple | Medido: noche, texto principal 11,6 : 1 y secundario 6,4 a 7,05 : 1; día, texto principal 13,1 : 1 y secundario 6,1 a 6,8 : 1. El secundario cumple ≥4,5:1 pero el estado y los números de línea quedan debajo de 7:1. La coincidencia activa de búsqueda es de bajo contraste. |
| N6 | Cumple parcialmente | Español coherente con voseo. Hay jerga («mosaico», «Anotaciones sin lugar», «Leyenda», «Herramientas de estudio») y textos largos en la barra de estado. |
| N7 | Cumple parcialmente | Avisos breves y bien ubicados en general (no tapan el centro). Algunos repiten lo obvio o son largos: «Dividir abajo · documento nuevo, abrir un archivo o una pestaña abierta», «Edición de fuente · F2 para volver a lectura». |
| N8 | Cumple parcialmente | Menús, paletas y diálogo comparten estilo. Los interruptores y los tooltips son incoherentes, hay dos «H» de distinto significado y los enlaces usan tres colores. |

## 4. Heurísticas de Nielsen

| # | Heurística | Puntaje | Lo bueno | Problemas |
| --- | --- | --- | --- | --- |
| 1 | Visibilidad del estado | 4 | Barra de estado completa, punto «•» de cambios sin guardar, avisos, botón de panel marcado, pestaña activa con línea verde. | Contadores cambian con el modo; «indexando carpeta» sin progreso; la confirmación de Copiar es fugaz. |
| 2 | Coincidencia con el mundo real | 4 | Español natural, nombres como Leer, Editar, Comparar, Guardar. | Jerga: «mosaico», «anotaciones sin lugar», «Leyenda»; «H» y «Párrafo» no dicen «título». |
| 3 | Control y libertad | 4 | Deshacer funciona, Esc cierra paneles y elige lo seguro en el diálogo, los avisos explican cómo deshacer una tarea. | «Cerrar sin guardar» a un clic de «Guardar» (color distinto, bien). Abrir un archivo reemplaza «Bienvenida» sin avisar. |
| 4 | Consistencia y estándares | 3 | Estilo visual unificado. | Interruptores de tres formas; icono «>_» para la paleta; tooltips en posiciones distintas. |
| 5 | Prevención de errores | 4 | Diálogo de cierre claro, enlaces rotos marcados, avisos tras acciones destructivas. | Negrita sobre línea vacía rompe títulos; «Quitar todas las anotaciones» está a la vista junto a acciones inocuas. |
| 6 | Reconocer antes que recordar | 3 | Menús con atajos, tooltips con atajo, paleta buscable, texto de ayuda en la bienvenida. | Barra de herramientas sin textos; varios iconos ambiguos; tooltips con retraso y mal ubicados. |
| 7 | Flexibilidad y eficiencia | 4 | Atajos, paleta, menú contextual, índice flotante, tipeo para buscar. | Atajos inverificables aquí; la paleta no muestra los atajos de todas las acciones. |
| 8 | Diseño estético y minimalista | 4 | «Papel y tinta» coherente, mucho aire, jerarquía clara en la lectura. | La barra de herramientas muestra 12+ iconos más 5 menús más pestañas; en el editor cada línea es una caja con fondo. |
| 9 | Ayuda con los errores | 3 | «Enfocá un enlace con Tab antes de abrirlo»; «No hay documentos sin guardar para recuperar». | «La búsqueda cubre solo parte del contenido» alarma sin explicar; el campo de búsqueda con «sin resultados» no sugiere nada. |
| 10 | Ayuda y documentación | 3 | Bienvenida con tres acciones y una frase sobre los modos. | No hay ayuda de Markdown ni tour; la bienvenida desaparece al abrir un archivo y no se puede volver a ver; «Configurar» y «Abrir tus archivos .md con Visor MD» son poco claros. |

## 5. Hallazgos

### QA-01. La negrita se inserta mal sobre una línea vacía de selección
- **Área y referencia:** Editor, barra de formato, R6.
- **Severidad:** S2. **Frecuencia:** siempre sin texto seleccionado.
- **Pasos:** abrir `inicio.md`, pasar a Editar con el cursor al inicio de la línea 1 (`# Mis apuntes de estudio`) y pulsar **B**.
- **Esperado:** un marcador de posición separado o un aviso («Seleccioná texto»); el título intacto.
- **Observado:** se inserta `**texto**` pegado al `#`: `**texto**# Mis apuntes de estudio`. El título deja de serlo. No hay aviso. Recuperable con Deshacer (117).
- **Evidencia:** capturas 114, 115, 117.
- **Recomendación:** si no hay selección, insertar `**texto**` con «texto» seleccionado y espacio previo, o mostrar un aviso. No tocar los marcadores de título.

### QA-02. Los tooltips aparecen lejos del puntero y tapan contenido
- **Área y referencia:** Tooltips y enlaces, R14, N2.
- **Severidad:** S2. **Frecuencia:** siempre.
- **Pasos:** en Leer, mover el puntero sobre un enlace; mover el puntero sobre un botón de la barra.
- **Esperado:** tooltip junto al puntero; el enlace cambia al pasar el puntero.
- **Observado:** el tooltip de enlaces va abajo a la izquierda, fijo, y tapa texto (la primera tarea en 100, 101, «Período del Terror» en 20). Los de la barra salen bajo la barra, desplazados. El enlace no cambia de estilo con el hover. Los tooltips aparecen solo tras un retraso largo y a veces no aparecen.
- **Evidencia:** 17b, 19b, 20, 21, 100, 101.
- **Recomendación:** anclar al puntero o al elemento, no tapar contenido, y dar un hover visible al enlace.

### QA-03. Barra de herramientas con iconos ambiguos y sin texto
- **Área y referencia:** Barra de herramientas, N6, N8, heurística 6.
- **Severidad:** S2. **Frecuencia:** siempre.
- **Pasos:** observar la barra en Editar y Leer.
- **Esperado:** iconos reconocibles o con texto para el público de estudiantes.
- **Observado:** en Editar hay dos «H» (una suelta y la de «Párrafo»), un icono tipo «N/» y otros sin significado claro. En Leer hay 9 iconos sin texto. La paleta tiene el icono «>_» en vez de «···». El botón de carpeta tiene una flecha minúscula. Solo se descubre el significado con el tooltip.
- **Evidencia:** 01, 16, 75, 109.
- **Recomendación:** etiquetas junto a los botones principales, iconos distintos y paleta con «···».

### QA-04. Contadores de la barra de estado cambian con el modo
- **Área y referencia:** Barra de estado, R22.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** abrir `manual-largo.md` y `inicio.md` en Editar y luego en Leer.
- **Esperado:** mismas cifras para el mismo documento.
- **Observado:** `manual-largo.md`: 13774 caracteres en Editar y 13484 en Leer. `inicio.md`: 107 palabras / 892 caracteres en Editar y 102 / 705 en Leer. En Leer desaparecen línea y columna. «1 palabras».
- **Evidencia:** 58, 75, 94, 107.
- **Recomendación:** contar siempre lo mismo (o rotular «con marcas»), y pluralizar bien.

### QA-05. Interruptores inconsistentes y confusos
- **Área y referencia:** Menús Vista, Archivo, Carpeta y paleta, N8, N6.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** abrir Vista, Archivo, Carpeta y la paleta en ambos temas.
- **Esperado:** un mismo patrón para estados.
- **Observado:** «Tema de noche: activado ✓» pero «Movimiento reducido: desactivado» sin marca. Con tema de día: «Tema de noche: desactivado». Se repite estado y marca. «Abrir archivos externos en la ventana abierta: activado ✓» es largo y poco claro.
- **Evidencia:** 05, 09, 11, 61.
- **Recomendación:** casillas o marca única, verbos de acción («Usar tema de noche»).

### QA-06. Avisos que repiten la acción o son largos
- **Área y referencia:** Avisos, N7.
- **Severidad:** S3. **Frecuencia:** frecuente.
- **Pasos:** Vista > Dividir abajo; F2; chincheta.
- **Observado:** el aviso repite el título del selector que acaba de abrirse («Dividir abajo · documento nuevo, abrir un archivo o una pestaña abierta»); «Edición de fuente · F2 para volver a lectura»; el aviso de carpeta ocupa casi todo el ancho. Algunos tapan la parte inferior del documento.
- **Evidencia:** 84, 86, 42, 54, 13.
- **Recomendación:** quitar el aviso cuando el selector ya lo explica y acortar los demás. Mantener el de tarea, que sí informa.

### QA-07. El panel lateral recorta su pestaña «Enlazan aquí»
- **Área y referencia:** Panel de carpeta, N1.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** abrir el panel de carpeta.
- **Observado:** la pestaña «Enlazan aquí» llega al borde del panel (13, 80), con la «í» pegada. En el listado de Archivos la nota abierta se resalta igual que el hover. «1–6 de 6» cuenta la carpeta como elemento mientras el título dice «5 notas».
- **Evidencia:** 13, 25, 80.
- **Recomendación:** acortar a «Enlaces», diferenciar la nota actual y unificar el conteo.

### QA-08. Abrir un archivo desde el panel reemplaza la pestaña de bienvenida y cierra el panel sin avisar
- **Área y referencia:** Pestañas y panel, R4, heurística 3.
- **Severidad:** S3. **Frecuencia:** primer uso.
- **Pasos:** desde la bienvenida, abrir la carpeta y hacer clic en `historia.md`.
- **Observado:** la pestaña «Bienvenida» desaparece y el panel se cierra (15, 16). La bienvenida no vuelve a mostrarse. Al abrir el siguiente archivo, el panel también se cierra.
- **Recomendación:** mantener el panel abierto (opcional), permitir reabrir la bienvenida desde un menú.

### QA-09. La paleta de acciones no es «···» y se pierde a 640 px
- **Área y referencia:** Paleta, R15, N1.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Observado:** el botón es un icono «>_» (parece una terminal); a 640 px la paleta se sustituye por una hamburguesa y el botón desaparece de la barra (75, 133). La lista abre en un panel desplegable junto al botón, no centrado.
- **Evidencia:** 61, 75, 133.
- **Recomendación:** usar «···» o «Acciones» y mantener visible el botón.

### QA-10. Búsqueda en el documento: estado y texto en el mismo campo, coincidencia activa poco visible
- **Área y referencia:** Búsqueda, R12.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** icono de lupa, escribir «la», Enter.
- **Observado:** el campo muestra «Buscar: la| · 2 de 6 · Enter siguiente · Mayús+Enter anterior · Esc cierra»: la consulta, el contador y las instrucciones son un solo texto con un «|» literal. La coincidencia actual solo se distingue levemente de las otras. Con «sin resultados», Enter igualmente desplaza la lectura. La búsqueda reemplaza el resto de la barra de herramientas.
- **Evidencia:** 31, 33, 35, 37.
- **Recomendación:** separar entrada y contador, con la coincidencia activa de color más fuerte, y no desplazar sin resultados.

### QA-11. Hover y pulsado casi inexistentes en varios controles
- **Área y referencia:** N2, estados.
- **Severidad:** S3. **Frecuencia:** frecuente.
- **Observado:** enlaces sin cambio al hover (20); el botón Copiar no cambia visiblemente al hover ni al pulsar (95, 96); la chincheta y los iconos de panel tienen cambio mínimo (91b); el hover de iconos de la barra no siempre se ve (17). La confirmación «✓ Copiado» dura menos de un segundo (99 frente a 99b) y no hay aviso.
- **Evidencia:** 20, 95, 96, 99, 99b.
- **Recomendación:** hover y pulsado visibles, y confirmar Copiar al menos 1,5 s o con aviso.

### QA-12. Aviso de carpeta indexada alarmante e incompleto
- **Área y referencia:** Carpeta de trabajo, R13, N6.
- **Severidad:** S3. **Frecuencia:** al abrir una carpeta.
- **Observado:** «Carpeta de apuntes indexada: 5 notas, 0 omitidas; la búsqueda cubre solo parte del contenido». «Solo parte del contenido» alarma sin explicar qué falta; ocupa casi todo el ancho.
- **Evidencia:** 13.
- **Recomendación:** explicar el límite o no mostrarlo si no hay omisiones.

### QA-13. En 640×480 se pierden textos y títulos de pestañas
- **Área y referencia:** N1.
- **Severidad:** S3. **Frecuencia:** siempre en tamaño mínimo.
- **Observado:** las etiquetas de «Insertar» y «Estudio» desaparecen; los títulos de pestaña se truncan («histori…», «prueba-qa.…») y la X pisa el título del activo (132). No hay controles encimados, pero el editor queda estrecho con el panel abierto (80).
- **Evidencia:** 75, 80, 132.
- **Recomendación:** reducir el ancho de las pestañas inactivas, reservar espacio para la X.

### QA-14. Ventana maximizada: el contenido queda pegado a la izquierda
- **Área y referencia:** N1.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Observado:** a 1366 px el texto ocupa unos 780 px a la izquierda y el resto queda vacío (104). En Editar el editor tiene 740 px fijos. El índice flotante ayuda pero no es la opción por defecto.
- **Evidencia:** 104, 106, 42.
- **Recomendación:** centrar la columna o permitir ancho de lectura configurable.

### QA-15. Comparar: las columnas no están del todo sincronizadas
- **Área y referencia:** R3, Comparar.
- **Severidad:** S3. **Frecuencia:** al comparar documentos con secciones plegadas o párrafos largos.
- **Observado:** en `historia.md`, «Directorio» está plegada en Leer pero aparece desplegada en la vista de Comparar. Hay un hueco irregular entre el párrafo de «Asamblea Nacional» y «Convención» (45). El fondo en bandas del editor da mucho peso visual.
- **Evidencia:** 41, 45.
- **Recomendación:** sincronizar el plegado y alinear los bloques.

### QA-16. Menú contextual de tarea: lista plana, sin atajos ni separación
- **Área y referencia:** R16.
- **Severidad:** S3. **Frecuencia:** al usarlo.
- **Observado:** cinco entradas sin separadores; solo la primera muestra su atajo («Espacio»), el menú aparece pegado al borde inferior sobre la barra de estado (39); «Herramientas de estudio» no indica que abre un submenú.
- **Evidencia:** 39.
- **Recomendación:** agrupar y marcar el submenú.

### QA-17. Jerga y términos poco claros
- **Área y referencia:** Microcopy, N6.
- **Severidad:** S4. **Frecuencia:** constante.
- **Observado:** «mosaico» (tooltip «Quitar del mosaico») frente a «panel» en el resto de la interfaz; «Anotaciones sin lugar», «Leyenda: qué significa cada color», «Fijar o soltar la pestaña» (no indica el estado actual); «Cancelar actualización de carpeta» siempre visible aunque no haya nada en curso, sin aspecto deshabilitado (09).
- **Evidencia:** 26, 81, 91b, 09.
- **Recomendación:** unificar el vocabulario y deshabilitar o esconder lo que no aplica.

### QA-18. El menú Estudio es largo y mezcla grupos
- **Área y referencia:** Menú Estudio, N6, heurística 8.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** unas 17 acciones con barra de desplazamiento; incluye «Quitar todas las anotaciones» junto a acciones inocuas, sin confirmación visible.
- **Evidencia:** 26, 132.
- **Recomendación:** separar lo destructivo y confirmar.

### QA-19. Tablas en Leer con tipografía de código
- **Área y referencia:** Auditoría visual, N8.
- **Severidad:** S4. **Frecuencia:** siempre que haya tablas.
- **Observado:** las celdas usan fuente monoespaciada y el borde es visible, mientras el resto del texto es serif; el estilo declarado habla de documento plano.
- **Evidencia:** 94, 124.
- **Recomendación:** usar la tipografía del cuerpo y separadores más sutiles.

### QA-20. Color: enlaces azul y amarillo contra «un único acento verde»
- **Área y referencia:** Estilo declarado, N8.
- **Severidad:** S4. **Frecuencia:** siempre con enlaces.
- **Observado:** enlaces web en azul, rotos con subrayado amarillo, notas en verde, y el color del resaltado en amarillo.
- **Evidencia:** 98, 100, 101.
- **Recomendación:** documentar la excepción o usar variaciones del acento y formas distintas.

### QA-21. Desalineación fina de iconos y números de línea
- **Área y referencia:** Auditoría visual.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** los números de línea quedan 2 a 3 px por encima del texto de su línea (42, 58); la flecha de «Párrafo», «Insertar» y «Estudio» queda más abajo que el texto; la flecha del botón de carpeta es muy pequeña.
- **Evidencia:** 42, 58, 109.
- **Recomendación:** alinear baselines.

### QA-22. Cuadros del editor y zona de clic pequeña
- **Área y referencia:** Accesibilidad básica, N2.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** el botón «Quitar del mosaico» mide unos 16 px; la X de las pestañas unos 14 px; la flecha de plegado unos 12 px. En el editor cada línea tiene un fondo en caja (peso visual alto).
- **Evidencia:** 87, 38, 14.
- **Recomendación:** ampliar la zona de clic a 24 px como mínimo.

## 6. Fortalezas

- Identidad visual sólida y coherente en noche y día. El texto principal tiene 11,6 a 13,1 : 1 y el secundario cumple el mínimo.
- Diálogo de cierre ejemplar: título claro, tres botones con verbos, foco inicial en Guardar y Esc seguro.
- Avisos informativos con el atajo o la forma de deshacer (por ejemplo, el de tarea).
- Menús y paletas con atajos visibles, agrupación y contador «1–14 de 77 · Esc para cerrar».
- Botones de panel con estado abierto/cerrado y cierre con segundo clic.
- Copiar copia solo el código y confirma con «✓ Copiado».
- Enlaces rotos distinguidos y destino real visible; búsqueda tolerante a tildes («celula» encuentra «célula»).
- La bienvenida ofrece tres acciones claras con su atajo.
- Responsive a 640 px sin controles encimados (menús agrupados en hamburguesa).
- Pestañas con indicador de cambios, fijado y reordenado por arrastre.

## 7. Recomendaciones priorizadas

1. **Corregir la inserción de formato sin selección (QA-01).** Evita romper títulos y confusión de principiantes.
2. **Rediseñar los tooltips (QA-02) y el hover y pulsado (QA-11).** Anclar al puntero, mostrar el destino de un enlace sin tapar contenido y hacer más visible Copiar.
3. **Etiquetar y desambiguar la barra de herramientas (QA-03, QA-09).** Textos en los botones principales, paleta con «···».
4. **Unificar la barra de estado (QA-04).** Mismos contadores en todos los modos y plurales correctos.
5. **Unificar los interruptores y el vocabulario (QA-05, QA-17).**
6. **Reducir y acortar avisos (QA-06, QA-12).**
7. **Mejorar la búsqueda (QA-10)** y la sincronización de Comparar (QA-15).
8. **Ajustes de ventana (QA-07, QA-13, QA-14).** Columna centrada al maximizar, pestañas estrechas a 640 px, pestaña del panel sin recorte.
9. **Pulido visual (QA-16, QA-18 a QA-22).**
