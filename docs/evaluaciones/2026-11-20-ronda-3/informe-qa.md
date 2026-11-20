# Informe de QA manual de UI/UX: Visor MD

Evaluación del 20 de noviembre de 2026. Fuentes: la hoja de requerimientos y la app en ejecución (copia `qa`). No se leyó código ni documentación del proyecto. Evidencias en `C:\Dev\archivo\eval-ronda-3\qa\capturas\`.

## 1. Resumen ejecutivo

**Calificación UX: 7,3 / 10.**

Visor MD tiene una identidad visual coherente («Papel y tinta»), una lectura muy cómoda y un diálogo de cierre ejemplar. Las funciones centrales (abrir, leer, editar, comparar, buscar, carpeta, índice, copiar código, tareas) funcionan. Lo que baja la nota es la comunicación: casi todos los estados de la interfaz se explican con avisos que se repiten, varios paneles y barras tapan contenido, y hay inconsistencias entre modos que desorientan.

**Los 5 problemas principales**

1. La barra de herramientas cambia de contenido según el modo. En Leer son los paneles (Índice, Archivos, Buscar, Enlazan aquí). En Editar y Comparar son las ayudas de formato (QA-01). Un clic en el mismo lugar hace cosas distintas, y en Editar no hay forma de abrir los paneles con un botón. En la prueba, un clic pensado para el panel aplicó negrita y modificó el documento.
2. Con la ventana dividida la barra de pestañas queda vacía y solo muestra «+». Las pestañas pasan a las cabeceras de los paneles (QA-02).
3. El cuadro de Buscar flota sobre el texto y tapa la coincidencia activa. No hay barra de búsqueda propia (QA-03).
4. Los avisos (toasts) son excesivos: repiten lo obvio («Vista de lectura», «Pestaña cerrada», «Seguís editando; no se cerró nada») y algunos son largos y tapan texto (QA-04).
5. Clic en otro título de menú con un menú abierto solo cierra el menú. Pasar el puntero sí lo cambia. El mismo gesto da dos comportamientos (QA-05). Se suman la confirmación de «Copiar» casi invisible por su brevedad (QA-06) y el borde de las casillas de tarea vacías con contraste de 1,4:1 (QA-07).

## 2. Alcance y entorno

- App: Visor MD, copia `qa`, perfil limpio en cada arranque. Temas noche y día. Tamaños 640×480, 1100×720 y maximizada (la pantalla de la prueba da 1366×738).
- Archivos usados: `apuntes\inicio.md`, `historia.md`, `manual-largo.md` (49 títulos), `biologia\celula.md`, `fotosintesis.md`. Se crearon `prueba-qa.md` y `salida.pdf` dentro de `qa\apuntes`.
- Aproximadamente 190 órdenes de interacción (clics, movimientos, teclas, rueda, capturas) y unas 150 capturas. Por debajo del rango pedido de 150 a 250 interacciones útiles en varios flujos secundarios (arrastrar para dividir, exportar a PDF a fondo, Ctrl+Z en edición larga).
- Límites de la herramienta (no son hallazgos):
  - Sin Ctrl, Alt ni Shift. Los atajos no se pudieron ejecutar, solo se verificó que estén comunicados.
  - `type` altera el texto escrito. Se obtuvo «3 pruueeba» por «# prueba», «textexot» por «texto» y «capiitulo» por «capitulo». Se atribuye a la herramienta, no a la app. Las pruebas de escritura con vocales no son concluyentes.
  - Al minimizar con el botón de la app y volver a maximizar, la ventana quedó negra. Hubo que cerrar y relanzar. Tampoco hay forma de restaurar desde la herramienta, así que no se evaluó.
  - Tras restaurar desde maximizada la captura mostró un área negra y la ventana desplazada. Se relanzó para el recorrido por teclado.
- Contraste medido sobre las capturas (muestreo de píxeles).

## 3. Matriz de requerimientos

| ID | Resultado | Evidencia |
| --- | --- | --- |
| R1 | Cumple | `inicio.md`, `historia.md` y `manual-largo.md` se abren y se ven formateados: títulos, tabla, citas, listas, código, enlaces, tareas (`01.png`, `manual-1.png`). |
| R2 | Cumple | «+» crea «sin título» con texto guía «Escribí aquí…». Guardar con diálogo propio dejó `prueba-qa.md` y el aviso «Guardado» (`guardado.png`). La pestaña toma el nombre de la primera línea. |
| R3 | Cumple | Leer, Editar (fuente con números de línea) y Comparar (fuente y vista a lado) funcionan. En Comparar el scroll de la vista sigue al de la fuente (`comparar-scroll.png`). Observaciones: QA-12 y QA-13. |
| R4 | Cumple parcialmente | Abrir, cambiar, cerrar, fijar y reordenar por arrastre funcionan. Cada pestaña recuerda modo y posición (`tab-inicio.png`). La pestaña fijada muestra chincheta y se acorta (`tab-fijada.png`). Con la ventana dividida la barra queda vacía (QA-02). |
| R5 | Cumple parcialmente | «Dividir a la derecha» ofrece un selector (documento nuevo, abrir archivo, pestaña abierta) y crea el panel con borde de foco (`split-2.png`). Se pudo cerrar un panel. No se probó dividir abajo ni 4 paneles. Observaciones: QA-02 y QA-14. |
| R6 | Cumple parcialmente | Hay botones de negrita, cursiva, tachado, código, título, tarea, cita, enlace, bloque, tabla, y menús Párrafo, Insertar y Estudio. Negrita sin selección inserta `**texto**` con la palabra seleccionada (buena ayuda). Faltan tooltips en algunos íconos y hay íconos ambiguos (QA-10). No se probaron resaltado, símbolos ni emojis hasta el final. |
| R7 | Cumple | Enter en «- uno» continuó la lista; Enter en elemento vacío la terminó (`lista-enter-vacio.png`). Al terminar inserta además una línea en blanco extra (QA-16). |
| R8 | Cumple parcialmente | Copiar copia solo el código del bloque (portapapeles verificado con el bloque Python). La confirmación «✓ Copiado» existe pero duró menos de 0,3 s en las capturas (`copiar-inm.png`; `copiar-a.png` ya no la mostraba). Ver QA-06. |
| R9 | Cumple | Un clic marca y desmarca la casilla en Leer, la pestaña pasa a «sin guardar» y aparece aviso (`tarea-click.png`). Sin respuesta visible al pasar el puntero (QA-08). |
| R10 | Cumple | La flecha junto al título pliega y despliega. El estado se conserva al cambiar de modo. La flecha mide unos 10 px (QA-09). La acción «Plegar o desplegar sección enfocada» (ícono ▷) es poco clara (QA-10). |
| R11 | Cumple | Índice con 5 y con 49 títulos, salto al clic y con Enter. Índice flotante opcional en ventanas anchas (`max-flotante-hov.png`). Observaciones: QA-11 y QA-15. |
| R12 | Cumple parcialmente | Resultados resaltados, contador «1 de 13», Enter siguiente, Mayús+Enter anterior, «sin resultados» (`busca-17.png`). El cuadro tapa la coincidencia activa (QA-03). La búsqueda en Edición no se probó. |
| R13 | Cumple | Abrir carpeta indexa 5 notas. Hay pestañas Archivos, Índice, Buscar (con ruta «biología › celula.md») y Enlazan aquí (2 resultados). Sin carpeta, los avisos guían bien («Elegí primero una carpeta…»). Los `[[nota]]` aparecen como enlace. Clic en un enlace sin carpeta abierta no mostró respuesta (QA-17). |
| R14 | Cumple parcialmente | Al pasar sobre un enlace web aparece «enlace web: https://…». Sobre uno roto: «Esta nota no existe: … · clic para crearla». El enlace roto se distingue con subrayado punteado ámbar (solo con la carpeta abierta). No se pulsó un enlace web (regla). El aviso aparece abajo a la izquierda, lejos del puntero (QA-18). |
| R15 | Cumple parcialmente | Existe la paleta con filtro («tema» → 1 resultado) y 77 acciones. El botón es «>_», no «···» como dice la hoja. A 640 px desaparece y lo reemplaza un menú hamburguesa que contiene «Buscar una acción» (QA-19). |
| R16 | Cumple parcialmente | Clic derecho en texto ofrece 4 acciones. Clic derecho en pestaña ofrece Dividir a la derecha, Dividir abajo, Fijar y Cerrar. No se probaron el menú de tarea ni el de tabla. |
| R17 | Cumple | Cambio día/noche con el ícono sol/luna, y Ctrl+Shift+D. «Movimiento reducido» está en Vista y en la paleta. No se verificó el efecto de la opción. |
| R18 | Cumple parcialmente | La chincheta tiene tooltip «Fijar la ventana encima de las demás» y se alterna. No se pudo comprobar que la ventana quede encima del resto (no hay otra ventana). Falta indicación visible de estado más fuerte (QA-20). |
| R19 | Cumple parcialmente | Desde la paleta se exportó a PDF y se generó `salida.pdf` en la carpeta. No se vio aviso de confirmación 2,5 s después. No se abrió el PDF. |
| R20 | Cumple | Diálogos «¿Guardar los cambios de «inicio.md»?» y «¿Cerrar Visor MD?» con tres botones nombrados. Esc elige la opción segura (el aviso dice «Seguís editando; no se cerró nada»). Foco inicial en Guardar y orden de Tab coherente (`dlg-cerrar.png`, `dlg-esc.png`, `dlg-cerrar-app.png`). |
| R21 | No verificable | Hay opción «Recuperación de cambios: activado ✓» y «Abrir recuperación». No se provocó un cierre brusco (prohibido tocar otros procesos). |
| R22 | Cumple parcialmente | Barra con palabras, caracteres, modo y estado («guardado», «sin guardar», «nuevo»). Línea y columna aparecen solo en Editar y Comparar, no en Leer. Los conteos cambian entre modos (102 palabras en Leer y 107 en Editar para el mismo archivo) (QA-12). |
| R23 | Cumple parcialmente | Tab recorre los enlaces del documento y muestra foco. F6 salta a la barra («Nuevo documento · Ctrl+N»). Los atajos se muestran en menús y tooltips. El foco visible es una caja tenue sobre los enlaces (QA-21). No se probaron los atajos con Ctrl. |
| N1 | Cumple parcialmente | A 640×480 los menús pasan a un menú hamburguesa y no hay controles encimados. Comparar sigue legible (`s640-comparar.png`). En Edición, la barra de formato ocupa todo el ancho con íconos de ~36 px. Maximizada se ve bien, pero el contenido queda pegado a la izquierda con gran espacio vacío (QA-22). |
| N2 | Cumple parcialmente | Hover visible en botones, menús, pestañas y Copiar. Sin hover en casillas de tarea, flechas de plegado y filas de Archivos (QA-08). Pulsado: hubo cambio sutil. |
| N3 | Cumple parcialmente | Menús, paleta, índice y panel lateral se cierran con un segundo clic y el botón queda resaltado (`ind-abierto.png`, `pal-toggle.png`). Excepción: QA-05 (clic en otro menú). |
| N4 | Cumple | Rueda en documento, índice largo (49 títulos) y menús. Teclado con flechas y Enter en índice y paleta. Menú Estudio con barra de desplazamiento. |
| N5 | Cumple parcialmente | Texto del cuerpo: 14,95:1 (noche) y 13,1:1 (día). Texto secundario: 5,9 a 7,4:1. Cumple 7:1 en cuerpo y 4,5:1 en secundario, pero el texto secundario no llega a 7:1 en estado y etiquetas (6,1 a 6,4:1), lo cual es aceptable porque la hoja exige 4,5:1. Falla en bordes de casillas (1,4:1) y en la flecha de plegado en noche (5,2:1, correcto). Ver QA-07. |
| N6 | Cumple parcialmente | Todo en español, con voseo coherente. Hay jerga que el público no conoce: «mosaico», «Estudio», «Leyenda: qué significa cada color», «indexada», «omitidas», «cubre solo parte del contenido» (QA-23). |
| N7 | No cumple | Hay avisos repetitivos y largos. Ver QA-04. |
| N8 | Cumple parcialmente | Hay coherencia visual general. Incoherencias: QA-01 (barra), QA-05 (menús), tooltips de distinto estilo y posición (QA-18), flechas de plegado solo en el panel con foco (QA-14), dos íconos H iguales (QA-10). |

Conteo. R: 10 Cumple, 12 Cumple parcialmente, 0 No cumple, 1 No verificable. N: 1 Cumple, 6 Cumple parcialmente, 1 No cumple, 0 No verificable.

## 4. Heurísticas de Nielsen

| Heurística | Puntaje | Lo bueno | Problemas |
| --- | --- | --- | --- |
| 1. Visibilidad del estado | 4 | Barra de estado completa (modo, guardado, carpeta activa, «indexando carpeta»). Punto en la pestaña con cambios. Pestaña activa con línea verde. Botones de panel resaltados cuando están abiertos. | Los avisos son demasiado frecuentes y se repiten (QA-04). La confirmación de Copiar es muy fugaz (QA-06). Sin indicación clara de ventana fijada (QA-20). |
| 2. Relación con el mundo real | 4 | Español natural con voseo. Textos como «Escribí aquí. El formato está en la barra de arriba». El diálogo de cierre habla con claridad. | Jerga: «mosaico», «Estudio», «indexada», «omitidas» (QA-23). |
| 3. Control y libertad | 4 | Esc cierra paneles y menús. Ctrl+Z deshace y el aviso lo indica. El diálogo de cierre elige lo seguro con Esc. Deshacer desde el menú Editar funcionó. | Cerrar la última pestaña crea un documento nuevo sin avisar. No hay forma de deshacer una pestaña cerrada. |
| 4. Consistencia y estándares | 3 | Estilo visual unificado. Menús con grupos, atajos a la derecha, ✓ para estados. | La barra de herramientas cambia de significado según el modo (QA-01). Clic y hover en menús difieren (QA-05). Tooltips de distinto formato («Tarea» frente a «Aplicar código en línea»). Dos íconos H idénticos. Flechas de plegado solo en el panel enfocado. |
| 5. Prevención de errores | 4 | Diálogo de cierre claro. Casilla marcada avisa «sin guardar». Mensajes guía sin carpeta («Elegí primero una carpeta…»). | Un clic en la barra cambia el documento sin aviso previo (QA-01). Clic en tarea modifica el texto sin pedir nada, aunque es reversible. |
| 6. Reconocer en vez de recordar | 3 | Atajos visibles en menús y tooltips. Paleta con filtro. | Íconos sin texto (QA-10). Las acciones de panel, plegado y «Enlazan aquí» exigen aprender íconos. Sin guía de primer arranque (QA-24). |
| 7. Flexibilidad y eficiencia | 4 | Atajos, paleta de 77 acciones, menús, hamburguesa en pantallas chicas, índice flotante. | El menú Estudio tiene más de 20 entradas y es denso. Sin atajos para dividir paneles. |
| 8. Diseño estético y minimalista | 3 | Estética sobria, tipografía de lectura excelente, mucho espacio en blanco, pocas cajas. | Demasiados botones y menús en la parte superior (tres filas de controles). Los avisos se acumulan. Menú Estudio con exceso de opciones. Pantalla vacía sin guía. |
| 9. Ayudar a reconocer y corregir errores | 4 | «Esta nota no existe: … · clic para crearla». «Abrí una carpeta de apuntes para ver qué notas enlazan aquí». Búsqueda «sin resultados». | El clic en enlaces sin carpeta no da respuesta (QA-17). La búsqueda no ofrece ayuda ante cero resultados. |
| 10. Ayuda y documentación | 3 | Texto guía en documento nuevo. Tooltips con atajos. «Leyenda» en Estudio. | Sin ayuda general, sin bienvenida, sin explicación de Markdown ni del flujo de carpeta (QA-24). |

Promedio: 3,6 / 5.

## 5. Hallazgos

### QA-01 La barra de herramientas cambia de significado según el modo
- Área: barra superior, segunda fila. Referencia: N8, R6, R11, R13.
- Severidad: S2. Frecuencia: siempre al pasar de Leer a Editar o Comparar.
- Pasos: abrir un documento en Leer. La segunda fila muestra seis íconos de paneles (Índice, Plegar, Abrir carpeta, Notas, Buscar en carpeta, Enlazan aquí). Pasar a Editar o Comparar.
- Esperado: acciones estables y en el mismo lugar, o ambas barras disponibles.
- Observado: la fila se reemplaza por las ayudas de formato (negrita, cursiva, etc.). En Editar y Comparar no hay botones de Índice, Archivos, Buscar ni Enlazan aquí. Un clic en (30,92) que en Leer abre el Índice aplicó negrita en Comparar: `**texto**` en el documento y la pestaña quedó «sin guardar» (`s640-panel.png`).
- Evidencia: `s640-panel.png`, `modo-editar.png`, `01.png`.
- Recomendación: mantener los botones de panel en un lugar fijo (por ejemplo, borde izquierdo o la barra de arriba) y dejar la barra de formato como una segunda fila separada. Si no, mostrar una etiqueta de grupo.

### QA-02 Con la ventana dividida la barra de pestañas queda vacía
- Área: pestañas y mosaico. Referencia: R4, R5.
- Severidad: S2. Frecuencia: siempre que hay dos paneles.
- Pasos: abrir dos documentos, Vista > Dividir a la derecha, elegir `manual-largo.md`.
- Esperado: pestañas visibles y reordenables.
- Observado: la barra de arriba muestra solo «+». Los nombres pasan a la cabecera de cada panel, junto con selectores propios Leer, Editar y Comparar. Cerrar o fijar pestañas exige el clic derecho o el menú. Al cerrar un panel las pestañas reaparecen.
- Evidencia: `split-2.png`, `split-foco-der.png`.
- Recomendación: mantener la barra de pestañas con ambas, marcando cuál está en cada panel.

### QA-03 El cuadro de Buscar tapa el texto y la coincidencia activa
- Área: búsqueda en documento. Referencia: R12.
- Severidad: S2. Frecuencia: siempre.
- Pasos: pulsar la lupa, escribir «17», pulsar Enter.
- Esperado: una barra propia, fuera del texto, con la coincidencia activa visible y diferenciada.
- Observado: un cuadro de una línea flota sobre el contenido (a unos 160 px del borde superior). La coincidencia 2 de 13 quedó bajo el cuadro. La coincidencia activa casi no se diferencia de las demás (todas verde apagado). La ayuda larga («Enter siguiente · Mayús+Enter anterior · Esc cierra») ocupa mucho ancho.
- Evidencia: `busca-17.png`, `busca-17-enter.png`.
- Recomendación: anclar la barra arriba del documento, desplazar el texto para que la coincidencia activa quede libre y darle un color distinto.

### QA-04 Avisos repetitivos y largos
- Área: avisos. Referencia: N7, N6.
- Severidad: S3. Frecuencia: casi cada acción.
- Pasos: cambiar de modo, cerrar una pestaña, cancelar un diálogo, marcar una tarea, abrir una carpeta.
- Esperado: avisos breves que confirmen acciones no obvias.
- Observado:
  - «Vista de lectura», «Edición de fuente · F2 para volver a lectura», «Vista dividida · fuente editable y lectura sincronizada» (cada cambio de modo).
  - «Pestaña cerrada».
  - «Seguís editando; no se cerró nada» tras Esc.
  - «Tarea actualizada · sin guardar: Ctrl+S la guarda, Ctrl+Z la deshace».
  - «Carpeta de apuntes indexada: 5 notas, 0 omitidas; la búsqueda cubre solo parte del contenido» (91 caracteres).
  - «Panel dividido a la derecha · el nuevo documento se enfoca con un clic» (la frase es confusa: el panel contiene un documento existente).
  - Aparecen abajo y centrados y cubren una línea de texto del documento (por ejemplo, sobre «Texto con negrita…»).
- Evidencia: `modo-comparar.png`, `tarea-click.png`, `carpeta-abierta.png`, `dlg-esc.png`, `vacio.png`, `split-2.png`.
- Recomendación: eliminar los avisos de acciones obvias (cambio de modo, cerrar, cancelar). Dejar los que aportan información nueva y acortarlos a menos de 50 caracteres.

### QA-05 Clic en otro título de menú solo cierra el menú
- Área: barra de menús. Referencia: N3, N8.
- Severidad: S3. Frecuencia: siempre.
- Pasos: abrir «Vista» con un clic y hacer clic en «Carpeta». Repetir con el puntero.
- Esperado: el menú cambia, como hace al pasar el puntero.
- Observado: el clic cerró el menú abierto y no abrió el otro (`carpeta-click-vista.png`). Pasar el puntero con un menú abierto sí cambia de menú (`m-archivo-hover-editar.png`). Se necesita un segundo clic.
- Recomendación: que el clic en otro título abra ese menú.

### QA-06 La confirmación de Copiar dura muy poco
- Área: bloque de código. Referencia: R8, N2.
- Severidad: S3. Frecuencia: siempre.
- Pasos: pulsar «Copiar» en un bloque y capturar.
- Esperado: confirmación visual claramente perceptible.
- Observado: el botón cambia a «✓ Copiado» pero ya no se ve en capturas tomadas 0,25 s después. No aparece aviso general. El portapapeles sí tiene el código del bloque. En reposo el botón ya tiene fondo verde, así que el hover no se distingue del estado normal.
- Evidencia: `copiar-inm.png` frente a `copiar-a.png`, `hov-copiar2.png`.
- Recomendación: mantener «Copiado» unos 1,5 s. Que el estado de reposo sea más neutro y el hover más marcado.

### QA-07 Bordes de casillas de tarea con contraste insuficiente
- Área: tareas. Referencia: N5, R9.
- Severidad: S3. Frecuencia: siempre, ambos temas.
- Medición: borde de la casilla vacía contra el fondo = 1,44:1 (noche) y 1,48:1 (día). Para controles se espera 3:1 como mínimo.
- Observado: la casilla vacía casi desaparece, y la marcada es verde sólido. Tampoco responde al pasar el puntero.
- Evidencia: `l3.png`, `dia-1.png`, `hov-tarea.png`.
- Recomendación: borde de al menos 3:1 y hover visible.

### QA-08 Elementos interactivos sin respuesta al hover
- Área: tareas, flechas de plegado, fila de archivos. Referencia: N2.
- Severidad: S3. Frecuencia: siempre.
- Observado: casillas de tarea, flechas de plegado y títulos no cambian al pasar el puntero (`hov-tarea.png`). Las flechas de plegado miden unos 10 px.
- Recomendación: cambio de cursor y fondo suave, y ampliar la zona de clic.

### QA-09 Zona de clic de la flecha de plegado muy pequeña
- Área: títulos. Referencia: R10, accesibilidad.
- Severidad: S3. Frecuencia: siempre.
- Observado: la flecha mide aproximadamente 10×10 px. Un clic desviado (38,128 en lugar de 32,128) plegó otra sección sin querer (`panel-archivos-vacio.png`).
- Recomendación: hacer clicable todo el margen izquierdo del título (mínimo 24×24 px).

### QA-10 Íconos ambiguos y duplicados
- Área: barras de herramientas. Referencia: R6, N8, heurística 6.
- Severidad: S3. Frecuencia: siempre.
- Observado:
  - Dos íconos «H» casi idénticos: título (181) y el menú Párrafo (219).
  - El ícono de enlace aparece en el botón de la barra (393) y en el menú «Insertar» (507).
  - El ícono del código en línea parece una «N» inclinada con barra.
  - El ícono ▷ para «Plegar o desplegar sección enfocada» sugiere reproducir.
  - Tooltips inconsistentes: «Tarea» frente a «Aplicar código en línea». Algunos botones (181 en edición) no mostraron tooltip.
  - El botón de paleta es «>_» (terminal), no «···».
- Evidencia: `ed-tb-143.png`, `ed-tb-181.png`, `ed-tb-316.png`, `tb-67.png`, `h-paleta.png`.
- Recomendación: íconos diferenciados, tooltips con la misma estructura («Verbo + objeto · atajo») y etiquetas de texto en las acciones principales.

### QA-11 Pestañas del panel lateral recortadas
- Área: panel lateral. Referencia: N1, R13.
- Severidad: S4. Frecuencia: siempre.
- Observado: la pestaña «Enlazan aquí» se corta en el borde del panel (la «í» llega a x=312 con panel de 303 px).
- Evidencia: `panel-enlazan.png`.
- Recomendación: ampliar el panel, acortar a «Enlaces» o repartir el ancho.

### QA-12 Cifras de palabras y caracteres cambian según el modo; línea y columna ausentes en Leer
- Área: barra de estado. Referencia: R22.
- Severidad: S3. Frecuencia: siempre.
- Observado: `inicio.md` marca 102 palabras y 705 caracteres en Leer y 107 y 892 en Editar. También «Lín 1, Col 1» aparece solo en Editar y Comparar, aun cuando el documento está desplazado a la línea 17 (no refleja el scroll). La diferencia puede deberse a que Leer cuenta texto visible, pero no se explica.
- Evidencia: `01.png` frente a `modo-editar.png`, `comparar-scroll.png`.
- Recomendación: usar un único criterio y aclararlo con un tooltip.

### QA-13 Fuente: interlineado exagerado y números de línea desalineados
- Área: modo Editar y Comparar. Referencia: R3, jerarquía visual.
- Severidad: S4. Frecuencia: siempre.
- Observado: cada línea de la fuente ocupa un bloque de unos 35 px con un hueco entre líneas, lo que duplica el alto. Los números de línea quedan 5 a 10 px por encima de la base del texto (por ejemplo «19» en y=179 y texto en y=191).
- Evidencia: `modo-editar.png`, `tab-inicio.png`.
- Recomendación: alinear la línea base y reducir el espaciado entre líneas.

### QA-14 Flechas de plegado solo en el panel con foco
- Área: mosaico. Referencia: N8, R5, R10.
- Severidad: S4. Frecuencia: siempre con dos paneles.
- Observado: en el panel sin foco desaparecen las flechas (`split-foco-der.png` frente a `split-2.png`).
- Recomendación: mostrarlas siempre.

### QA-15 Dos resaltados simultáneos en listas (hover y teclado)
- Área: índice, paletas. Referencia: heurística 1.
- Severidad: S4. Frecuencia: a veces.
- Observado: con el puntero sobre una fila y el teclado en otra, ambas se resaltan con el mismo color (`mi-teclas.png`: «Ejercicio 30» y «Ejercicio 40»).
- Recomendación: que el hover sea más tenue que la selección por teclado.

### QA-16 Al terminar una lista se inserta una línea en blanco de más
- Área: edición. Referencia: R7.
- Severidad: S4. Frecuencia: siempre.
- Observado: Enter dos veces tras «- dos» dejó una línea vacía (línea 4) y el cursor en la 5, en lugar de quedar en la 4.
- Evidencia: `lista-enter-vacio.png`.
- Recomendación: que Enter en un elemento vacío elimine el marcador y mantenga el cursor en esa línea.

### QA-17 Clic en un enlace entre notas sin carpeta abierta no da respuesta
- Área: enlaces `[[nota]]`. Referencia: R13, heurística 9.
- Severidad: S3. Frecuencia: siempre sin carpeta.
- Pasos: abrir `inicio.md` sin carpeta de apuntes y pulsar «biologia/celula» y «historia».
- Esperado: abrir la nota o avisar «Abrí una carpeta para seguir este enlace».
- Observado: solo se colocó el cursor. No hubo aviso en la captura a 0,8 s. Sin carpeta, los demás botones sí avisan.
- Evidencia: `wiki-click.png`.
- Recomendación: avisar o ofrecer abrir la carpeta.

### QA-18 Los avisos al pasar el puntero aparecen lejos de él
- Área: tooltips. Referencia: R14, N2.
- Severidad: S4. Frecuencia: siempre.
- Observado: los tooltips de la barra aparecen a unos 35 px debajo de la barra, alineados a la izquierda del botón y a veces lejos del puntero. El de enlaces (web, nota, nota rota) aparece fijo abajo a la izquierda de la ventana aunque el puntero esté en otra zona, y puede tapar texto. El tooltip de los botones del mosaico («Quitar del mosaico») sí está cerca del botón.
- Evidencia: `hov-web.png`, `hov-roto.png`, `tb-30.png`, `split-hov-cerrar.png`.
- Recomendación: colocar el tooltip junto al elemento, con la misma regla en toda la app.

### QA-19 El botón de la paleta no es «···» y a 640 px se reemplaza
- Área: paleta. Referencia: R15, N1.
- Severidad: S3. Frecuencia: siempre.
- Observado: el botón es «>_». A 640 px aparece un menú hamburguesa que reúne los menús y la paleta, y el botón «>_» desaparece. La paleta no tiene un campo de texto visible: la cabecera «Buscar una acción · escribí para filtrar» actúa como campo.
- Evidencia: `h-paleta.png`, `pal-abierta.png`, `s640-hamb.png`.
- Recomendación: usar el ícono acordado y un campo de entrada visible.

### QA-20 La chincheta no muestra su estado con claridad
- Área: controles de ventana. Referencia: R18, N3.
- Severidad: S4. Frecuencia: siempre.
- Observado: la chincheta tiene el mismo aspecto que los otros controles. Las capturas con la opción activa se perdieron por el problema de la herramienta, así que el contraste del estado activo no se pudo medir. La paleta indica «Ventana siempre encima: desactivado».
- Recomendación: verificar que el estado activo se distinga (relleno de acento).

### QA-21 Foco de teclado tenue y orden de Tab inesperado
- Área: teclado. Referencia: R23, accesibilidad.
- Severidad: S3. Frecuencia: siempre.
- Observado: Tab empieza por los enlaces del documento y no por la barra superior. El foco se ve como una caja tenue (verde apagado sobre verde oscuro) sin anillo. F6 llevó el foco a «+» con un recuadro claro. Un segundo F6 no mostró dónde quedó el foco. En el diálogo de cierre el anillo de foco sí es nítido.
- Evidencia: `kt1.png`, `kt3.png`, `kf1.png`, `kf2.png`, `dlg-tab1.png`.
- Recomendación: usar el mismo anillo del diálogo en toda la app.

### QA-22 En ventanas anchas el contenido queda pegado a la izquierda
- Área: maquetación. Referencia: N1.
- Severidad: S4. Frecuencia: siempre en 1100 px o más.
- Observado: el texto ocupa unos 740 px alineados a la izquierda y deja 300 a 600 px vacíos a la derecha, tanto en Leer como en Editar (`modo-editar.png`, `max-flotante-hov.png`). En Comparar sí se reparte. El índice flotante recorta el texto largo («Asamblea Nacional (1789–1791)» sobresale del cuadro).
- Recomendación: centrar la columna o ajustar el ancho; permitir que el índice flotante parta líneas.

### QA-23 Jerga y términos poco claros para el público
- Área: microcopy. Referencia: N6.
- Severidad: S4. Frecuencia: frecuente.
- Observado: «mosaico» («Quitar del mosaico», «Mosaico» en Vista); «Estudio» (menú con subrayado, marcas de libro, anotaciones sin lugar); «indexada», «omitidas», «cubre solo parte del contenido»; «Enlazan aquí»; «Copiar para Discord o correo» (sin explicación); «recuperación»; «sin guardar» con «sin título». Cada ítem del menú Insertar repite «Insertar».
- Recomendación: lenguaje llano («paneles», «Resaltar y notas», «Carpeta lista: 5 notas»).

### QA-24 Sin primer arranque guiado ni estados vacíos útiles
- Área: primer uso y estados vacíos. Referencia: heurística 10, público estudiante.
- Severidad: S3. Frecuencia: cada arranque limpio.
- Observado: el primer arranque abre el documento pedido o un «sin título» en blanco, sin bienvenida ni guía. Un documento vacío en Leer es una pantalla totalmente vacía (`vacio-leer.png`) sin texto como «Este documento está vacío. Pasá a Editar para escribir». El texto guía solo existe en Editar.
- Recomendación: pantalla de inicio con tres acciones (nuevo, abrir, carpeta) y estados vacíos con guía.

### QA-25 Más de 20 opciones en el menú Estudio
- Área: menú Estudio. Referencia: N6, heurística 8.
- Severidad: S4. Frecuencia: al abrirlo.
- Observado: el menú tiene decenas de entradas (subrayar en seis colores, marcas de libro, anotaciones sin lugar) y barra de desplazamiento. Es un nivel de detalle alto para un estudiante que recién empieza.
- Evidencia: `ed-estudio.png`.
- Recomendación: agrupar en submenús o dejar lo básico visible.

### QA-26 Controles de ventana y estados de color bajo el diálogo
- Área: diálogo de cierre. Referencia: R20, N8.
- Severidad: S4. Frecuencia: al cerrar con cambios.
- Observado: al cerrar la app el botón X de la ventana se mantiene rojo y brillante bajo la sombra del diálogo, mientras el resto está atenuado. No coincide con la paleta de un solo acento verde.
- Evidencia: `dlg-cerrar-app.png`.
- Recomendación: atenuar también el control de ventana y usar el color de peligro solo en el botón del diálogo.

### Hallazgos de color y paleta
- Rojo suave en «Cerrar sin guardar» (correcto como advertencia). Azul en enlaces web, verde en enlaces de nota y ámbar en roto: tres colores más allá del «único acento verde» declarado. Se considera una desviación menor y justificada por función.

### Conteo por severidad
- S1: 0.
- S2: 3 (QA-01, QA-02, QA-03).
- S3: 12 (QA-04, 05, 06, 07, 08, 09, 10, 12, 17, 19, 21, 24).
- S4: 11 (QA-11, 13, 14, 15, 16, 18, 20, 22, 23, 25, 26).
- Total: 26 hallazgos (S1 0, S2 3, S3 12, S4 11).

## 6. Fortalezas

- Identidad visual coherente y agradable. El tema de noche verde oscuro y el de día crema se sienten parte de la misma familia. La tipografía de lectura es excelente y el contraste del cuerpo supera 13:1 en ambos temas.
- Diálogo de cierre ejemplar: título claro, explicación breve, tres acciones nombradas, foco inicial seguro y Esc que conserva el trabajo (R20).
- Los menús muestran grupos, atajos y estados («Tema de noche: activado ✓»), y la paleta con 77 acciones se filtra bien.
- Cada pestaña recuerda modo y posición de lectura. El título de «sin título» se toma de la primera línea.
- Copiar copia exactamente el código del bloque.
- Índice, búsqueda con contador y recorrido, y carpeta de apuntes con backlinks funcionan y resultan útiles.
- Tooltips informativos para enlaces: «enlace web: …», «Esta nota no existe: … · clic para crearla».
- Mensajes de guía sin carpeta abierta: «Elegí primero una carpeta de apuntes con Ctrl+Shift+O».
- Adaptación a 640×480: los menús pasan a un menú compacto sin encimar controles.
- Listas largas: la rueda y el teclado funcionan, y el pie indica «33–49 de 49».

## 7. Recomendaciones priorizadas

1. **Fijar la barra de paneles y separarla de la de formato** (QA-01). Mayor impacto en comprensión y en evitar cambios accidentales.
2. **Mantener las pestañas en el mosaico** (QA-02).
3. **Anclar la búsqueda** y diferenciar la coincidencia activa (QA-03).
4. **Recortar los avisos**: quitar los obvios y acortar los demás (QA-04).
5. **Unificar clic y hover en menús** (QA-05).
6. **Mejorar retroalimentación**: «Copiado» más largo (QA-06), hover en tareas y flechas con zona mayor (QA-07, 08, 09), contraste de bordes (QA-07).
7. **Pulir íconos y tooltips** (QA-10, QA-18, QA-19).
8. **Teclado y foco**: anillo claro y orden de Tab razonable (QA-21).
9. **Primer arranque y estados vacíos** (QA-24, QA-17).
10. **Detalles de maquetación y microcopy** (QA-11, 12, 13, 14, 15, 16, 22, 23, 25, 26).
