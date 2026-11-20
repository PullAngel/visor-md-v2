# Informe de QA manual de UI/UX: Visor MD

Evaluación del 20 de noviembre de 2026 (prompt de la ronda 2). QA manual sénior de UI/UX, primera vez con la aplicación. Fuentes: la hoja de requerimientos y la app en ejecución. No se leyó código ni documentación. Las capturas están en `C:\Dev\archivo\eval-ronda-2\qa\capturas\`.

## 1. Resumen ejecutivo

**Calificación UX: 7 de 10.**

Visor MD tiene una base sólida y cuidada. El lenguaje visual «Papel y tinta» es coherente. La tipografía de lectura es muy buena. Casi todos los menús, paneles y diálogos se comportan con criterio. Los diálogos propios son claros y Esc siempre elige la opción segura. Copiar confirma sin ruido, el índice flotante es útil y la ventana de 640×480 se adapta bien.

Lo que baja la nota es lo siguiente:
- un defecto visible al dividir la ventana;
- la búsqueda sin campo de texto visible;
- el enlace roto casi igual a uno válido;
- demasiados avisos redundantes;
- contrastes de íconos y de texto secundario justos;
- la navegación por teclado, que se queda corta.

No se encontró ningún hallazgo S1 (pérdida de datos o bloqueo).

### Los 5 problemas principales
1. **QA-01 (S2).** Al dividir con un documento que estaba en Editar, el panel muestra el texto fuente con un botón Copiar en cada línea. Además el selector del panel dice «Leer».
2. **QA-02 (S2).** El enlace roto a una nota no se distingue del válido. Sin carpeta de apuntes es idéntico, y con carpeta solo cambia un subrayado punteado. Ni el aviso emergente ni el color dicen que está roto.
3. **QA-03 (S2).** Buscar en el documento y buscar en la carpeta no muestran un campo de texto. Lo escrito aparece en una etiqueta. No hay «2 de 3», ni botones anterior/siguiente, ni resaltado de todas las coincidencias.
4. **QA-06 (S3).** Los avisos repiten lo obvio o hablan en jerga técnica: «Vista de lectura actualizada», «Sección plegada; la fuente permanece intacta», «búsqueda limitada por presupuesto de memoria».
5. **QA-11 (S3).** Teclado: Tab en Leer salta entre enlaces y avisa «Este documento no tiene enlaces». F6 solo alterna entre dos zonas. El foco apenas se ve y deja avisos pegados encima del contenido.

## 2. Alcance y entorno

- App `qa.exe` con perfil limpio, controlada solo con `ui.ps1`. No se tocaron mouse ni teclado reales. No se pulsó ningún enlace web.
- Tamaños probados: 1100×720 (mayoría), 640×480 y maximizada (1366×738). Temas noche (por defecto) y día.
- Material: `manual-largo.md` (49 títulos), `inicio.md`, `historia.md` y un documento nuevo `prueba-qa.md`. También se vio la carpeta `apuntes`, con 5 notas.
- Aproximadamente 200 comandos de interacción y 136 capturas, más montajes de recorte.
- Contrastes: medidos sobre las capturas, tomando el píxel más contrastante frente al fondo dominante. Son valores aproximados, un poco por debajo del real porque el suavizado de bordes atenúa los trazos finos.
- Portapapeles: se leyó una sola vez, justo después del Copiar de un bloque de código. Contenía solo el código de los apuntes.
- Archivos creados dentro de `qa\apuntes\`: `prueba-qa.md`, `exportado.pdf` y `exportado2.pdf`.
- Límites de la herramienta, no reportados como fallos: sin Ctrl/Alt/Shift (no se probaron atajos con Ctrl), sin mayúsculas ni tildes al escribir, diálogos de archivo nativos reemplazados.
- No verificado:
  - recuperación tras cierre brusco (R21);
  - efecto real de «siempre encima» (R18);
  - Movimiento reducido;
  - división hasta 4 paneles y «dividir abajo» completo;
  - fijar pestaña;
  - menú contextual de tabla;
  - tabla, emoji y bloque de código desde la barra;
  - confirmación antes de abrir un enlace web.

## 3. Matriz de requerimientos

| ID | Resultado | Evidencia |
| --- | --- | --- |
| R1 | Cumple | `manual-largo.md`, `inicio.md` e `historia.md` abren formateados en Leer (17, 43, 55). |
| R2 | Cumple | Nuevo documento (66), escritura (68), Guardar con ruta (72): la pestaña pasa a «prueba-qa.md» y la barra marca «guardado». |
| R3 | Cumple | Leer (18), Editar con números de línea (64) y Comparar lado a lado (65). Los conteos difieren entre modos (QA-05). |
| R4 | Cumple parcialmente | Abrir, cambiar, cerrar y reordenar con arrastre funcionan (131). Recuerda la posición de lectura: `manual-largo` vuelve donde estaba (100). **Fijar** no se verificó. |
| R5 | Cumple parcialmente | Dividir a la derecha con elección del contenido funciona (90–92). Defecto grave en QA-01 y QA-16. Abajo y hasta 4 paneles no verificados. |
| R6 | Cumple parcialmente | Barra con negrita, cursiva, tachado, código, resaltado, títulos, tarea, cita, enlace, tabla, bloque y símbolo/emoji (80–89 y 73–75). Solo se comprobó que negrita envuelve la palabra (73) y que Enter continúa listas. Íconos ambiguos (QA-08). |
| R7 | Cumple | «- uno», Enter y «dos» generaron «- dos». Enter en el elemento vacío terminó la lista (68). |
| R8 | Cumple | Copiar pasa a «Copiado» con marca de verificación en el botón (45). El portapapeles tenía solo las dos líneas del bloque. No hay aviso duplicado. |
| R9 | Cumple | Un clic marca la tarea (35) y la pestaña pasa a «sin guardar». |
| R10 | Cumple | Flecha junto al título plegó «Ejercicio 15» (42). El título conserva el estado plegado al cambiar de tema (100). |
| R11 | Cumple | Panel Índice con 49 títulos (30–33). Índice flotante en ventana ancha, con sección actual marcada (128). |
| R12 | Cumple parcialmente | Cuenta resultados («3 resultados»), recorre con Enter y marca en la barra lateral (61–63). Sin campo visible, solo se resalta la coincidencia actual (QA-03). |
| R13 | Cumple | Carpeta indexada (5 notas). Panel Archivos en árbol (57), Buscar en carpeta (59), Notas que enlazan aquí (60) y `[[nota]]` abre en pestaña nueva (55). Detalles en QA-02 y QA-04. |
| R14 | Cumple parcialmente | Al pasar el puntero sobre el enlace web se ve la URL real (49) y el enlace es azul con ícono de salida. La confirmación previa al abrir no se probó (regla de no pulsar enlaces). Enlace roto poco distinguible (QA-02). |
| R15 | Cumple parcialmente | Existe, lista 77 acciones, filtra al escribir y tiene contador (12–16). Es un menú, no muestra campo, y su botón es «>_» en vez de «···» (QA-07). |
| R16 | Cumple parcialmente | Menú contextual adaptado a texto (36), tarea con «Marcar o desmarcar tarea · Espacio» (37) y pestaña (40). Sin acción para plegar en el menú del título (38). Tabla no verificada. |
| R17 | Cumple parcialmente | Cambio de tema con tooltip y aviso (98–100). Movimiento reducido aparece en Vista pero no se probó. |
| R18 | Cumple parcialmente | La chincheta tiene tooltip, estado activo visible y se desactiva con un segundo clic (101–104). El efecto sobre otras ventanas no es verificable aquí. |
| R19 | Cumple | Exportar a PDF generó el archivo (1,6 MB) y avisó «PDF exportado: exportado2.pdf · 7 páginas» (130). |
| R20 | Cumple | Cerrar pestaña (70) y cerrar ventana (135): títulos y botones claros («Cerrar sin guardar», «Cancelar», «Guardar»). Esc elige Cancelar y avisa «Seguís editando; no se cerró nada» (71, 136). |
| R21 | No verificable | Hay opción «Recuperación de cambios: activado ✓» en Carpeta (10) y los diálogos la mencionan, pero no se forzó un cierre brusco. |
| R22 | Cumple parcialmente | Muestra palabras, caracteres, línea, columna, modo y guardado. Los conteos cambian al pasar de Leer a Editar (QA-05). |
| R23 | Cumple parcialmente | Atajos visibles en menús y tooltips. Navegación con flechas y Enter en menús funciona (134). Foco débil y comportamiento de Tab/F6 limitado (QA-11). No se pudieron probar atajos con Ctrl. |
| N1 | Cumple parcialmente | A 640×480 los menús se reúnen en «≡», la barra se compacta y no hay controles encimados (120–125). Excepciones: la pestaña «Notas que enlazan aquí» invade el documento (121) y, maximizada, el texto queda pegado a la izquierda (127). |
| N2 | Cumple parcialmente | Hover visible en casi todo (2, 3, 20–25) y estado pulsado en la chincheta (102). Quedan sin señal propia el enlace roto y los títulos plegables al pasar el puntero. |
| N3 | Cumple | Índice (30–31), Notas (57–58), paleta (12–16) y menús se cierran con un segundo clic. El botón queda resaltado mientras el panel está abierto. |
| N4 | Cumple | Rueda en la paleta (13), índice (32) y menú de 640 (124). Flechas, Enter y Esc en menús (134). |
| N5 | Cumple parcialmente | Cuerpo de texto 13–15:1. Texto secundario 5,9–7,4:1. Íconos de la barra 2,0–4,1:1 (QA-13). |
| N6 | Cumple parcialmente | Todo en español, con voseo coherente. Hay jerga («Anotaciones sin lugar», «presupuesto de memoria») y pie de listas con contadores (QA-20). |
| N7 | No cumple | Hay avisos que repiten lo obvio y otros muy largos; algunos tapan contenido (QA-06). |
| N8 | Cumple parcialmente | Botones, menús y diálogos son coherentes. Se rompen los estados de conmutadores (QA-09), el nombre y el ícono de la paleta (QA-07) y el ícono ≡ usado para dos funciones. |

## 4. Heurísticas de Nielsen

| # | Heurística | Puntaje | Lo bueno | Problemas |
| --- | --- | --- | --- | --- |
| 1 | Visibilidad del estado del sistema | 3,5 | Pestaña con punto de «sin guardar», barra de estado con modo y guardado, botones resaltados mientras el panel está abierto. | Avisos ruidosos (QA-06). Búsqueda sin contador de posición (QA-03). Estado pegado en tooltips (QA-12). |
| 2 | Relación entre sistema y mundo real | 4 | Español natural, «Cerrar sin guardar», «Notas que enlazan aquí». | Jerga: «sin lugar», «presupuesto de memoria», «Fuente + lectura». |
| 3 | Control y libertad del usuario | 4,5 | Esc siempre seguro, segundo clic cierra paneles, diálogo de cierre con Cancelar. | El índice se cierra solo al elegir un título (QA-17). |
| 4 | Consistencia y estándares | 3 | Estética unificada. Menús de contexto, diálogos y botones coinciden. | Estados de conmutadores (QA-09), ícono ≡ doble, H doble (QA-08), paleta (QA-07). |
| 5 | Prevención de errores | 4 | Guardado protegido. Aviso de carpeta antes de abrir enlaces entre notas. Enlace a nota inexistente ofrece crearla. | Abrir carpeta desde un enlace pide decisión sin explicar qué cambia. |
| 6 | Reconocer antes que recordar | 3 | Tooltips con atajo, menús agrupados con encabezados, pantalla de bienvenida con atajos. | Barras de íconos sin etiqueta. Búsqueda sin campo. Menú Estudio de 20+ ítems sin jerarquía. |
| 7 | Flexibilidad y eficiencia | 3,5 | Paleta filtrable, atajos listados, menú contextual, índice flotante. | Tab/F6 limitados (QA-11). Paleta sin atajo visible en ancho completo. |
| 8 | Diseño estético y minimalista | 3,5 | Mucho aire, jerarquía clara en lectura, color acento único. | Cuatro formas de abrir paneles (barra, pestañas internas, menú, paleta). Muchos avisos. Bandas por línea en Editar. |
| 9 | Ayuda a reconocer y recuperar errores | 3,5 | Mensajes claros («Esta nota no existe… Podés crearla»). | Enlace roto sin marca clara (QA-02). Sin recuperación visible tras un error en la búsqueda. |
| 10 | Ayuda y documentación | 3,5 | Texto de bienvenida breve y mensajes de ayuda en vacío («Escribí aquí…»). | Desaparece el enlace «Configurar» tras cancelar; la ayuda sobre Leer/Editar/Comparar no se vuelve a mostrar. |

## 5. Hallazgos

### QA-01. Dividir con un documento en Editar muestra la fuente como bloques de código
- **Área:** Paneles divididos (R5). **Severidad:** S2. **Frecuencia:** reproducida de forma consistente.
- **Pasos:** en `historia.md` ya visto en Editar, abrir Vista > Dividir a la derecha y elegir `historia.md`. Pulsar «Leer» en el panel derecho.
- **Esperado:** el panel derecho muestra el documento formateado, con el selector acorde.
- **Observado:** el panel derecho se abre en texto fuente monoespaciado con el selector marcando «Leer» (91, 92). Al pulsar «Leer» queda en fuente pero cada línea lleva su propio botón «Copiar» (93, 95). Solo se corrige pulsando «Editar» y luego «Leer» (96).
- **Evidencia:** 91, 92, 93, 95, 96.
- **Recomendación:** que el panel nuevo renderice según el modo marcado y que el selector refleje siempre el modo real.

### QA-02. El enlace roto casi no se distingue de uno válido
- **Área:** Enlaces entre notas (R13, R14). **Severidad:** S2. **Frecuencia:** siempre.
- **Pasos:** abrir `inicio.md`, mirar «nota-que-no-existe» y pasar el puntero.
- **Esperado:** señal clara de enlace roto antes de pulsarlo.
- **Observado:** sin carpeta de apuntes, el enlace tiene el mismo verde subrayado que uno válido (48, 50). El aviso dice «enlace a otra nota: nota-que-no-existe», igual que para uno válido (47, 50). Al pulsar aparece primero «No hay carpeta de apuntes» (51) y después «Esta nota no existe» (53). Con carpeta activa solo cambia a un subrayado punteado de bajo contraste (54).
- **Evidencia:** 47, 48, 50, 51, 53, 54.
- **Recomendación:** color o ícono propio (por ejemplo tachado o color de error) y tooltip «Nota inexistente · se puede crear».

### QA-03. Buscar sin campo de texto visible ni contador de posición
- **Área:** Buscar (R12, R13). **Severidad:** S2. **Frecuencia:** siempre.
- **Pasos:** pulsar la lupa, escribir «ter». Repetir en Buscar en carpeta.
- **Esperado:** campo editable, «2 de 3», anterior/siguiente, todas las coincidencias resaltadas.
- **Observado:** una etiqueta flotante «Buscar: ter · 3 resultados» sin caret (62). Solo la coincidencia actual está resaltada (62, 63). Enter avanza sin indicar posición. En la carpeta, el texto escrito reemplaza el título del panel («Buscar en apuntes: celula · 4 resultados», 59) y los resultados se truncan con «…».
- **Evidencia:** 61, 62, 63, 59.
- **Recomendación:** campo real con placeholder, botones y contador «n de m».

### QA-04. La pestaña «Notas que enlazan aquí» desborda el panel lateral
- **Área:** Panel lateral (N1). **Severidad:** S3. **Frecuencia:** siempre que se abre el panel.
- **Pasos:** abrir Índice o Notas en 1100×720 y 640×480.
- **Esperado:** las cuatro pestañas caben dentro del panel de 303 px.
- **Observado:** el texto llega a x=384, fuera del panel. A 640 se superpone al título del documento («Notas que enlazan aq» sobre «Capítulo 14») (30, 60, 121).
- **Evidencia:** 30, 60, 121.
- **Recomendación:** acortar («Enlaces»), usar íconos o permitir que las pestañas se compriman.

### QA-05. Los conteos de palabras y caracteres cambian entre Leer y Editar
- **Área:** Barra de estado (R22). **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** abrir `historia.md` y pasar de Leer a Editar.
- **Esperado:** el mismo documento, el mismo conteo.
- **Observado:** `historia.md`: 56 palabras y 384 caracteres en Leer, 59 y 436 en Editar (55, 64). `manual-largo.md`: 2036 y 13484 en Leer, 2037 y 13774 en Editar (127, 126).
- **Evidencia:** 55, 64, 126, 127.
- **Recomendación:** usar el mismo criterio en todos los modos o aclarar que Editar cuenta los símbolos de formato.

### QA-06. Avisos redundantes, largos o en jerga
- **Área:** Avisos (N6, N7). **Severidad:** S3. **Frecuencia:** muy alta.
- **Pasos:** cambiar de modo, plegar una sección, marcar una tarea, dividir, abrir la carpeta.
- **Esperado:** avisos breves que confirmen acciones no evidentes.
- **Observado:**
  - «Vista de lectura actualizada» al marcar una tarea (35) y al cambiar de modo (96, 127).
  - «Vista dividida actualizada» (65).
  - «Edición de fuente · F2 para volver a lectura» (64).
  - «Sección plegada; la fuente permanece intacta» (42).
  - «Dividir a la derecha · documento nuevo, abrir un archivo o una pestaña abierta», que repite el encabezado del menú (90).
  - «Carpeta de apuntes indexada: 5 notas, 0 omitidas; búsqueda limitada por presupuesto de memoria» (52).
  - Varios aparecen sobre el texto que se está leyendo (35, 42).
  - El aviso de bienvenida («Tus cambios sin guardar se protegen… · Más detalles») desaparece antes de poder leerlo y pulsarlo (01, 04).
- **Evidencia:** 01, 04, 35, 42, 52, 64, 65, 90, 96.
- **Recomendación:** eliminar los avisos de «actualizado», dejar solo confirmaciones de resultado (Copiado, PDF exportado) y reescribir los técnicos.

### QA-07. Botón de la paleta con ícono y nombre inconsistentes
- **Área:** Paleta de acciones (R15, N8). **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** localizar el botón de la paleta en la barra superior y en 640×480.
- **Esperado:** un botón «···» con un nombre estable.
- **Observado:**
  - El botón tiene un ícono «>_» de terminal, sin etiqueta (01, 11).
  - La lista abre con «Acciones · escribe para filtrar» (12) y, a 640, con «Buscar una acción · Ctrl+Shift+P» (123).
  - A 640 el menú se reúne en «≡», el mismo ícono que el panel Índice (21, 122).
  - El filtro se ve como texto en la cabecera, sin campo ni caret (14).
- **Evidencia:** 11, 12, 14, 21, 122, 123.
- **Recomendación:** ícono «···» o lupa, un único nombre, campo de filtro visible y atajo mostrado en el ancho completo.

### QA-08. Íconos ambiguos en las barras
- **Área:** Barras de herramientas (N2, N8). **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** pasar el puntero sobre los íconos de Editar y de Leer.
- **Esperado:** íconos distinguibles sin leer el tooltip.
- **Observado:**
  - «Resaltar texto» (H) es casi idéntico al ícono de títulos (H con «Párrafo») (83, 89).
  - «Código en línea» usa una «N» con barra (82, 89).
  - «Plegar o desplegar sección enfocada» usa un triángulo tipo «reproducir» (21).
  - Los tooltips funcionan, pero con 14 íconos sin etiqueta el estudiante debe pasar el mouse sobre cada uno (20–25, 80–89).
- **Evidencia:** 21, 82, 83, 89.
- **Recomendación:** íconos con rotulador, `<>` y «plegar» distintos, y etiquetas en las acciones principales.

### QA-09. Estados de los conmutadores inconsistentes
- **Área:** Menús (N8). **Severidad:** S3. **Frecuencia:** en todos los conmutadores.
- **Pasos:** abrir Vista, Archivo y Carpeta.
- **Esperado:** una convención única para mostrar «activado».
- **Observado:** «Tema de noche: activado ✓» pero «Movimiento reducido: desactivado» sin marca (09). «Abrir archivos externos en la ventana abierta: activado ✓» y «Recuperación de cambios: activado ✓» (06, 10). Texto y marca dicen lo mismo.
- **Evidencia:** 06, 09, 10, 12.
- **Recomendación:** usar solo marca de verificación y nombres neutros («Tema de noche», «Movimiento reducido»).

### QA-10. Maximizada, la columna de texto queda pegada a la izquierda
- **Área:** Diseño responsivo (N1). **Severidad:** S3. **Frecuencia:** siempre en ventanas anchas.
- **Pasos:** maximizar (1366×738) y abrir un documento en Leer o Editar.
- **Esperado:** composición equilibrada.
- **Observado:** el texto ocupa cerca de 700 px (de x=48 a 750) y deja unos 600 px vacíos a la derecha (126, 127). Con el índice flotante se llena solo un rincón (128). También a 1100 queda un margen derecho de ~320 px (18).
- **Evidencia:** 18, 126, 127, 128.
- **Recomendación:** centrar la columna o alinear el índice flotante junto al texto.

### QA-11. Teclado: Tab y F6 limitados y foco poco visible
- **Área:** Accesibilidad por teclado (R23, N4). **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** en Leer pulsar Tab dos veces, luego F6 repetido, con flechas y Enter en los menús.
- **Esperado:** foco visible que recorra barra, pestañas, contenido y estado.
- **Observado:**
  - Tab en Leer salta entre enlaces y muestra «Este documento no tiene enlaces» (106). Sin enlaces no hay otro efecto.
  - F6 solo alterna entre la barra de herramientas y el contenido (108, 111, 112), sin pasar por menús, pestañas ni barra de estado.
  - El foco en los botones es un recuadro tenue (109).
  - En el contenido no se ve foco alguno (111).
  - En los menús, flechas y Esc funcionan (133, 134).
- **Evidencia:** 106, 107, 108, 109, 111, 112, 133, 134.
- **Recomendación:** anillo de foco de acento en todos los controles, Tab como recorrido de controles y F6 por regiones completas.

### QA-12. Tooltips pegados o lejos del puntero
- **Área:** Tooltips (N2, N7). **Severidad:** S3. **Frecuencia:** frecuente.
- **Pasos:** pasar el puntero sobre un enlace en el cuerpo y usar teclado sobre los botones.
- **Esperado:** el tooltip aparece cerca del elemento y desaparece al irse el foco o el puntero.
- **Observado:**
  - Los tooltips de enlaces aparecen abajo a la izquierda, pegados a la barra de estado, lejos del enlace (47, 49, 50).
  - Quedan visibles tras hacer clic (55, 51).
  - El tooltip de foco «Nuevo documento · Ctrl+N» quedó sobre un título hasta pulsar otra tecla (121, 108).
- **Evidencia:** 47, 49, 50, 55, 108, 121.
- **Recomendación:** anclar al elemento y ocultar al activar o al perder el foco.

### QA-13. Contraste de íconos y de texto secundario justo
- **Área:** Color y contraste (N5). **Severidad:** S3. **Frecuencia:** siempre.
- **Medición aproximada, noche:**
  - cuerpo 14,95:1;
  - texto secundario (estado, modos inactivos) 6,4–7,4:1;
  - íconos de barra 4,1:1;
  - tema y chincheta 3,6:1;
  - minimizar 2,5:1.
- **Medición aproximada, día:**
  - cuerpo 13,1:1;
  - texto secundario 5,9–6,1:1;
  - íconos de barra 3,1:1;
  - tema 2,7:1;
  - minimizar 2,0:1.
- **Observado:** el texto secundario cumple 4,5:1 pero no 7:1. Los íconos de la ventana quedan por debajo de 3:1, sobre todo en día. En día, «Leer/Comparar» inactivos se leen apagados (99).
- **Evidencia:** 01, 18, 43, 100.
- **Recomendación:** subir los íconos a 3:1 o más y el texto de estado a 7:1.

### QA-14. Primer arranque confuso
- **Área:** Primer arranque (R22). **Severidad:** S3. **Frecuencia:** siempre en perfil limpio.
- **Pasos:** lanzar con perfil limpio y mirar la bienvenida.
- **Esperado:** pantalla de bienvenida sin controles de edición.
- **Observado:**
  - Abre con el selector en «Editar» y la barra de formato completa sobre una pantalla de bienvenida sin documento (01).
  - La barra de estado dice «0 palabras · 0 caracteres · Lín 1, Col 1 · Edición · nuevo».
  - El aviso de protección «Más detalles» desaparece en pocos segundos (04).
  - El enlace «Abrir tus archivos .md con Visor MD · Configurar» desaparece para siempre tras cerrar el diálogo con Esc (05b).
  - El diálogo de registro se entiende, pero «Quitar / Cancelar / Registrar» no aclara que Esc no registra.
- **Evidencia:** 01, 04, 05, 05b.
- **Recomendación:** ocultar edición en la bienvenida, aviso persistente hasta cerrarlo y conservar el enlace.

### QA-15. El contenido se desplaza al interactuar
- **Área:** Lectura (N2). **Severidad:** S3. **Frecuencia:** frecuente.
- **Pasos:** en `manual-largo.md` abrir y cerrar el índice, o marcar una tarea.
- **Esperado:** el texto no cambia de posición.
- **Observado:**
  - Al abrir y cerrar el índice, «Capítulo 2» pasa de y=431 a y=458 (18, 31).
  - Al marcar una tarea, el título de la sección sube unos 26 px (33, 35); un clic derecho inmediato cayó en un espacio vacío en vez de la tarea (36).
- **Evidencia:** 18, 31, 33, 35, 36.
- **Recomendación:** mantener la posición y reservar el alto antes de que aparezcan elementos.

### QA-16. Paneles divididos: selector duplicado, barra cambiante y números de línea perdidos
- **Área:** Paneles (R3, R5). **Severidad:** S3. **Frecuencia:** siempre al dividir.
- **Pasos:** dividir con dos documentos y enfocar cada panel.
- **Esperado:** controles de modo claros y estado estable.
- **Observado:**
  - Cada panel trae su propio Leer/Editar/Comparar y el selector global se duplica (92).
  - La barra bajo las pestañas cambia entre formato (Editar) y lectura según el panel enfocado (92, 93).
  - El panel izquierdo en Editar pierde sus números de línea cuando el foco pasa al derecho (93, 95).
  - La pestaña de cada panel sale de la barra principal (91).
- **Evidencia:** 91, 92, 93, 95.
- **Recomendación:** un solo control por panel, estado estable y números de línea siempre.

### QA-17. El índice se cierra al elegir un título
- **Área:** Índice (R11). **Severidad:** S4. **Frecuencia:** siempre.
- **Pasos:** abrir el índice y pulsar «Capítulo 14».
- **Esperado:** saltar y poder seguir eligiendo.
- **Observado:** el panel se cierra al saltar y el título queda pegado a la barra, con su borde superior casi cortado (33).
- **Evidencia:** 32, 33.
- **Recomendación:** mantener abierto el panel o dejar un margen superior al título.

### QA-18. Editar: bandas por línea y números mal alineados
- **Área:** Vista Editar (R3). **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:**
  - Cada línea tiene una banda de fondo y entre líneas queda un hueco (64, 68).
  - Los números de línea quedan 2 px más arriba que el texto («8» en y=135, texto en y=137) (64).
  - La tabla y los bloques sobresalen del margen del texto (x=36–42 frente a 48) (43).
- **Evidencia:** 43, 64, 68.
- **Recomendación:** alinear la base de los números y unificar el margen.

### QA-19. Menús contextuales poco ricos
- **Área:** Contextual (R16). **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:**
  - Sobre un título no ofrece plegar (38).
  - El menú no tiene separadores ni íconos y se abre encima del elemento pulsado (36–38).
  - Un clic derecho en la zona vacía de la barra de pestañas abre el menú de texto en vez del de pestaña (39).
- **Evidencia:** 36, 37, 38, 39, 40.
- **Recomendación:** añadir «Plegar sección» y abrir los menús sin tapar lo pulsado.

### QA-20. Microcopy con jerga y pies técnicos
- **Área:** Textos (N6). **Severidad:** S4. **Frecuencia:** frecuente.
- **Observado:**
  - Menú Estudio de más de 20 ítems con «Anotaciones sin lugar» y «Ver el bloque de anotaciones» (76).
  - Cada ítem de Insertar repite «Insertar …» (75).
  - Los pies de lista dicen «1–17 de 49 · Esc para cerrar» (30, 57).
  - «Subrayar» en Estudio frente a «Resaltar texto» en la barra (76, 83).
  - Estado «carpeta activa» (52).
- **Evidencia:** 30, 52, 57, 75, 76, 83.
- **Recomendación:** nombres de uso común, quitar el prefijo repetido y simplificar los pies.

### QA-21. Dividir: menú lejos de su origen y nombre de pestaña que cambia
- **Área:** Paneles y pestañas (R2, R5). **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:**
  - El selector de «Dividir» se abre en la esquina superior izquierda, lejos de Vista o de la pestaña (90, 132).
  - Un documento nuevo se llama «sin título» y pasa a llamarse como su primera línea («uno») antes de guardar (66, 68).
- **Evidencia:** 66, 68, 90, 132.
- **Recomendación:** anclar el menú al origen y mantener «sin título» hasta guardar.

### Conteo por severidad
| Severidad | Cantidad | Hallazgos |
| --- | --- | --- |
| S1 | 0 | n/a |
| S2 | 3 | QA-01, QA-02, QA-03 |
| S3 | 13 | QA-04 a QA-16 |
| S4 | 5 | QA-17 a QA-21 |

## 6. Fortalezas

- Identidad visual coherente («Papel y tinta»), con acento verde único y sombras solo en lo flotante.
- Tipografía de lectura excelente, con jerarquía clara de títulos y cuerpo a 13–15:1.
- Diálogos propios claros y seguros: botones con verbos, Esc siempre elige la opción segura y el aviso lo confirma.
- Copiar del bloque de código funciona, confirma en el propio botón y copia solo el código.
- Los paneles y menús se cierran con un segundo clic y muestran su estado abierto.
- La paleta reúne 77 acciones y filtra al escribir.
- Menús completos, agrupados, con atajos visibles y navegación por flechas.
- Índice y panel de carpeta funcionales, con índice flotante que sigue la lectura.
- Adaptación a 640×480 sin controles encimados (excepto QA-04) y barra de edición compacta.
- Lista continua con Enter y cierra con Enter vacío.
- Guardado, exportación a PDF y recuerdo de la posición de lectura por pestaña funcionan.

## 7. Recomendaciones priorizadas

1. **Corregir QA-01** (panel dividido con fuente y Copiar por línea) y estabilizar QA-16.
2. **Rehacer la búsqueda** con campo visible, contador «n de m», anterior/siguiente y resaltado de todas las coincidencias (QA-03).
3. **Marcar claramente los enlaces rotos** con estilo y tooltip propios (QA-02).
4. **Limpiar los avisos**: quitar los de «actualizado», acortar y reescribir los técnicos, y no tapar contenido (QA-06).
5. **Teclado y foco**: anillo de foco de acento, F6 por regiones completas, Tab por controles y tooltips que no queden pegados (QA-11, QA-12).
6. **Unificar la paleta y el ícono «≡»** (QA-07), los conmutadores (QA-09) y los íconos ambiguos (QA-08).
7. **Subir el contraste** de íconos y de texto de estado a los umbrales de N5 (QA-13).
8. **Arreglar el desbordamiento de la pestaña «Notas que enlazan aquí»** (QA-04) y centrar la columna en ventanas anchas (QA-10).
9. **Igualar los conteos entre modos** (QA-05) y evitar saltos de contenido (QA-15).
10. **Pulir el primer arranque** (QA-14) y la microcopy (QA-20, QA-21), además de los detalles S4 (QA-17 a QA-19).
