# Informe de QA manual de UI/UX · Visor MD (versión de prueba)

Evaluación del 21 de noviembre de 2026. Todo lo descrito fue observado en la app en ejecución (copia `qa`); capturas en `C:\Dev\archivo\eval-ronda-5\qa\capturas\`.

## 1. Resumen ejecutivo

**Calificación UX: 7,5 / 10.**

Visor MD es una app coherente, liviana y con identidad visual clara («Papel y tinta»). Lo más sólido: tipografía de lectura, contraste en ambos temas, menús con atajos y agrupados, diálogos de cierre bien redactados, paletas y tooltips con atajos, teclado (F6, flechas, Tab) funcional con foco visible. Lo más débil: algunas funciones no se descubren o se comunican con jerga (Markdown crudo, «Estudio»), la barra de pestañas se vuelve ilegible con muchas pestañas o en 640 px, y hay varias inconsistencias menores de estados (hover de Copiar, foco, dobles resaltados, panel flotante desbordado).

**Cinco problemas principales**

1. Pestañas ilegibles con muchas abiertas / ventana mínima: nombres truncados a «sin…», y el marcador de cambios «•» se superpone con la «×» en la pestaña activa; varias pestañas «sin título» sin numerar (QA-01).
2. Las ayudas de formato insertan sintaxis Markdown cruda sin selección (`**texto**`), contra el público «sin saber Markdown» (QA-02).
3. El hover del botón **Copiar** casi no se distingue del estado normal, y el enlace web usa un segundo color (azul) fuera del acento único (QA-03, QA-12).
4. Mensajes contradictorios o poco claros: «No hay documentos sin guardar para recuperar» con documentos sin guardar abiertos; la carpeta «5 notas» frente a «1–6 de 6»; aviso «la búsqueda cubre solo parte del contenido» (QA-04).
5. Índice flotante: la entrada activa se desborda de su tarjeta, no aparece con paneles divididos, y el espacio vacío a la derecha en ventanas anchas no se aprovecha (QA-05).

## 2. Alcance y entorno

- App: Visor MD, copia `qa`, perfil limpio. Única fuente: hoja de requerimientos y la interfaz (no se leyó código ni documentación del proyecto).
- Control exclusivamente con `ui.ps1` (sin mouse/teclado reales). Sin Ctrl/Alt/Shift: los atajos con modificador se verificaron solo por su presencia en la interfaz (tooltips, menús).
- Tamaños: 1100×720, 640×480 y maximizada (1366×738). Temas: noche (inicio) y día.
- Material: `manual-largo.md` (49 títulos), `inicio.md`, `historia.md`, carpeta `apuntes` (con subcarpeta `biologia`). Se creó `nuevo.md` y se exportaron `manual.pdf` y `manual2.pdf` en `qa\apuntes\`.
- Aproximadamente 200 interacciones (clics, hover, rueda, teclas, escritura) y 126 capturas.
- No se abrió ningún enlace web. El portapapeles se leyó una sola vez, tras Copiar, y contenía el código del bloque `bash` de los apuntes.
- Contraste medido sobre las capturas (relación de luminancia de píxeles): noche texto 14,2–15:1, secundario 5,7–7,0:1; día texto 11,8–13,1:1, secundario 5,9–6,1:1.
- No verificables en este entorno: arrastrar y soltar archivos, recuperación tras cierre brusco, atajos con Ctrl, apertura real de enlaces web, diálogos nativos.

## 3. Matriz de requerimientos

| ID | Resultado | Evidencia |
| --- | --- | --- |
| R1 | Cumple | Abrir `manual-largo.md` e `inicio.md` por Abrir archivo: se ve formateado en Leer (cap. 14, 35). |
| R2 | Cumple | Nuevo documento (+), escribir y Guardar: pide nombre, queda `nuevo.md`, aviso «Guardado» (cap. 99–100). |
| R3 | Cumple | Leer / Editar (fuente con números de línea) / Comparar lado a lado con scroll sincronizado en `manual-largo.md` (cap. 59, 60, 107). Un documento vacío no muestra números de línea hasta que hay texto. |
| R4 | Cumple parcialmente | Abrir, cambiar, cerrar, reordenar (arrastre) y fijar funcionan (cap. 104, 105). Con muchas pestañas son ilegibles (QA-01); «fijada para esta sesión» no es evidente. Memoria de posición de lectura no verificada de forma explícita. |
| R5 | Cumple parcialmente | Dividir a la derecha funciona (cap. 73). «Dividir abajo» se ofreció pero no se completó (diálogo de archivo sin respuesta). No se probó el máximo de 4 paneles. |
| R6 | Cumple parcialmente | Barra con negrita, cursiva, tachado, código, títulos, listas, tareas, cita, enlace, tabla, bloques, símbolos y emojis (cap. 63–66, 101, 103). Sin selección inserta `**texto**` (QA-02). Iconos de código en línea / resaltado poco reconocibles (QA-09). |
| R7 | Cumple | Enter en «3. …» crea «4.», otra vez «5.»; Enter en elemento vacío termina la lista (cap. 61–62). Deja una línea en blanco de más (QA-13). |
| R8 | Cumple | Botón Copiar copia solo el código (verificado en portapapeles) y cambia a «✓ Copiado» (cap. 37). El hover casi no se ve (QA-03). |
| R9 | Cumple | Clic en casilla la marca; aviso «Tarea actualizada · sin guardar…» (cap. 31). El aviso es largo (QA-14). |
| R10 | Cumple | Triángulo junto al título pliega/despliega (cap. 28). Hay acción «Plegar o desplegar sección enfocada». |
| R11 | Cumple | Panel Índice (49 títulos), salto con Enter, rueda y End (cap. 23, 120–121); índice flotante en ventana ancha (cap. 94). Defectos en QA-05. |
| R12 | Cumple | Buscar en documento con resaltado, «1 de 6», Enter/Mayús+Enter y marcas en el scroll (cap. 56–57). Busca subcadenas («la» dentro de «célula»). Al abrirse oculta la barra de herramientas (QA-10). |
| R13 | Cumple parcialmente | Abrir carpeta, árbol de archivos, búsqueda en carpeta y «Enlazan aquí» funcionan; `[[nota]]` abre la nota (cap. 46–52). Resultados truncados sin fragmento visible y textos confusos (QA-04, QA-11). |
| R14 | Cumple parcialmente | Al pasar el puntero se muestra el destino real del enlace web (cap. 41). Enlace roto distinguido solo con subrayado punteado amarillo, y solo con carpeta abierta; sin carpeta es idéntico a uno válido (QA-06). |
| R15 | Cumple parcialmente | Paleta de acciones con 77 acciones, filtro, rueda, flechas, estado vacío y Esc (cap. 05–13). El botón es `>_`, no «···», y no hay campo de texto visible (QA-07). |
| R16 | Cumple | Clic derecho: menú de tarea, de texto y de pestaña con acciones distintas (cap. 32–34). No se probó el menú de tabla. |
| R17 | Cumple | Día/noche con el icono sol/luna y aviso (cap. 75–76). Movimiento reducido activable con aviso (cap. 116); su efecto no es observable en capturas estáticas. |
| R18 | Cumple | Chincheta junto a minimizar; estado activo visible y aviso «Ventana fijada encima…» (cap. 91). |
| R19 | Cumple | Exportar a PDF genera el archivo y avisa «PDF exportado: manual2.pdf · 7 páginas» (cap. 115). La primera exportación se hizo sin captura del aviso. |
| R20 | Cumple | Cerrar pestaña y cerrar ventana con cambios muestran diálogo propio: «Cerrar sin guardar / Cancelar / Guardar», Esc cancela (cap. 81–82, 125–126). Texto claro. Ver QA-04 sobre la recuperación. |
| R21 | No verificable | No se pudo forzar un cierre brusco. «Recuperación de cambios: activado ✓» está en el menú Carpeta y el diálogo de cierre promete recuperaciones, pero «Abrir recuperación» respondió «No hay documentos sin guardar para recuperar» (QA-04). |
| R22 | Cumple | Barra de estado: palabras, caracteres, Lín/Col, modo, estado de guardado y carpeta activa (cap. 59, 100). |
| R23 | Cumple parcialmente | F6 salta a la barra de herramientas, flechas recorren los botones, Tab cambia de grupo, Enter activa; foco visible (cap. 83–88). Los atajos con Ctrl no se pudieron ejecutar. Estilo de foco inconsistente (QA-08). |
| N1 | Cumple parcialmente | A 640×480 no hay controles encimados: los menús pasan a «☰», la barra de herramientas se compacta (cap. 77–80, 117–118). Pero las pestañas quedan ilegibles y en paneles divididos desaparecen los botones Leer/Editar/Comparar del panel (QA-01, QA-15). Maximizada se ve bien, con mucho espacio vacío (QA-05). |
| N2 | Cumple parcialmente | Casi todo tiene hover (resaltado + tooltip con atajo, cap. 02, 15–20). Excepción: Copiar (QA-03). |
| N3 | Cumple | Paleta, menús y paneles laterales (Índice, Archivos, Enlazan aquí) se cierran con un segundo clic, y el botón queda marcado como activo (cap. 05–08, 53–54). Los paneles laterales se cierran solos al abrir una nota o saltar a un título (comportamiento no comunicado, QA-16). |
| N4 | Cumple | Rueda y teclado en paleta (cap. 09, 11), Índice (cap. 119–120), menú «☰» (cap. 123) y documento. Un tick avanza unas 60 px (lento en documentos largos). |
| N5 | Cumple | Texto normal 11,8–15:1 y secundario 5,7–7,0:1 en ambos temas. El secundario queda por debajo de 7:1 pero supera el 4,5:1 pedido. |
| N6 | Cumple parcialmente | Todo en español, voz rioplatense coherente. Interfaz sobria. Hay jerga en el menú «Estudio» («anotaciones sin lugar», «marcas de libro») y textos largos en avisos (QA-04, QA-14, QA-17). |
| N7 | Cumple parcialmente | Avisos breves y a pie de ventana. Algunos repiten lo obvio o duplican el panel («Dividir a la derecha · …» repetido en el aviso), y tapan contenido unos segundos (QA-14). |
| N8 | Cumple parcialmente | Coherencia general muy buena. Fallos: segundo color en enlaces web, foco con tres estilos, resaltado doble en listas, tablas en tipografía monoespaciada (QA-08, QA-12, QA-18). |

Resumen: R Cumple 14 · Parcial 8 · No cumple 0 · No verificable 1; N Cumple 3 · Parcial 5.

## 4. Heurísticas de Nielsen

| # | Heurística | Puntaje | Lo bueno | Problemas |
| --- | --- | --- | --- | --- |
| 1 | Visibilidad del estado del sistema | 4,5 | Barra de estado completa, marcador «•» de cambios, avisos, pestaña activa con raya verde, botones de panel marcados. | Dos resaltados simultáneos en listas; «•» y «×» superpuestos. |
| 2 | Coincidencia con el mundo real | 4 | Español natural, «Tarea», «Cita», «Subrayar». | Jerga: «anotaciones sin lugar», «marcas de libro»; Markdown crudo (`**texto**`). |
| 3 | Control y libertad | 4,5 | Esc cierra todo; Deshacer/Rehacer en menú; Cancelar en diálogos; diálogo de cierre seguro. | Panel lateral que se cierra solo; «Cancelar actualización de carpeta» sin estado visible. |
| 4 | Consistencia y estándares | 3,5 | Menús, tooltips y diálogos uniformes. | Foco con tres estilos; enlace azul; paleta con `>_` en vez de «···»; hover de Copiar. |
| 5 | Prevención de errores | 4 | Diálogo de cierre con lista de archivos; enlaces rotos marcados con tooltip «clic para crearla». | Enlace roto solo se marca con carpeta abierta; varias pestañas «sin título» idénticas. |
| 6 | Reconocer antes que recordar | 4 | Tooltips con atajos, paleta filtrable, menús agrupados. | Iconos poco reconocibles (código, resaltado); pestañas ilegibles; Esc/Enter de la paleta sin campo visible. |
| 7 | Flexibilidad y eficiencia | 4,5 | Atajos, paleta de 77 acciones, F6/flechas, split, comparar. | Un tick de rueda avanza poco; sin numeración de pestañas. |
| 8 | Diseño estético y minimalista | 4 | Estilo «Papel y tinta» fiel, plano, sombras solo en flotantes. | Menú «Estudio» muy largo; espacio vacío en ventana ancha; tablas en monoespaciada. |
| 9 | Ayudar a reconocer y recuperarse de errores | 3,5 | Mensaje claro al abrir archivo inexistente («No se pudo abrir… Revisa que exista…»). | El error se muestra como texto plano sin icono ni color y abre una pestaña inútil; mensaje de recuperación contradictorio. |
| 10 | Ayuda y documentación | 3,5 | Pantalla de bienvenida con atajos; placeholder útil en documento nuevo; avisos explican el atajo. | Sin ayuda sobre qué es «Estudio», «Enlazan aquí» ni la sintaxis; pantalla de bienvenida desaparece al abrir. |

Promedio: 4,0 / 5.

## 5. Hallazgos

### QA-01 · Pestañas ilegibles y marcador de cambios solapado
- **Área / ref.:** barra de pestañas · R4, N1, N8.
- **Severidad:** S2. **Frecuencia:** siempre con 8+ pestañas o a 640 px.
- **Pasos:** abrir 8 pestañas con «+» (1366 px) y luego reducir a 640×480.
- **Esperado:** nombres reconocibles, desplazamiento de la tira o menú de pestañas; documentos nuevos distinguibles.
- **Observado:** pestañas se encogen hasta mostrar «sin…», «m…x ▪»; en la activa el punto de cambios y la «×» se superponen. Varias pestañas nuevas se llaman todas «sin título». Con 10 pestañas a 1366 px ya se trunca «manual-l… •».
- **Evidencia:** capturas 98, 105, 117.
- **Recomendación:** ancho mínimo por pestaña con scroll horizontal o menú desplegable de pestañas; numerar «sin título 2…»; reservar un hueco fijo para «•» y «×».

### QA-02 · Las ayudas de formato exponen Markdown sin selección
- **Área / ref.:** Editar, barra de formato · R6, público objetivo.
- **Severidad:** S3. **Frecuencia:** siempre que no hay selección.
- **Pasos:** en Editar, poner el cursor sin seleccionar y pulsar Negrita.
- **Esperado:** modo «escribir en negrita» o texto de ayuda legible; resultado no técnico.
- **Observado:** se inserta `**texto**` pegado a la palabra siguiente («cuarta **texto**causa»). En Leer queda «texto» en negrita como relleno.
- **Evidencia:** capturas 63, 86–88.
- **Recomendación:** si no hay selección, insertar marcadores y dejar el cursor en medio, o activar un modo de escritura; añadir espacios; evitar texto de relleno que se guarda.

### QA-03 · Hover de «Copiar» casi invisible
- **Área / ref.:** bloques de código · N2, R8.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** pasar el puntero sobre «Copiar» y compararlo con el otro bloque.
- **Esperado:** cambio de fondo/borde claro.
- **Observado:** el botón se ve igual con y sin hover (capturas 36 vs. botón vecino). Sí hay confirmación «✓ Copiado» tras pulsar.
- **Evidencia:** capturas 36, 37.
- **Recomendación:** aplicar el mismo hover que el resto de botones planos y un estado pulsado visible.

### QA-04 · Mensajes de recuperación y de carpeta contradictorios o confusos
- **Área / ref.:** Carpeta, avisos · N6, N7, R21, R13.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** (a) con documentos sin guardar, Carpeta → Abrir recuperación. (b) Abrir la carpeta `apuntes`. (c) Ver aviso y lista.
- **Esperado:** textos coherentes entre diálogo, menú y avisos.
- **Observado:** (a) «No hay documentos sin guardar para recuperar» con tres documentos sin guardar, mientras el diálogo de cierre promete recuperaciones. (b) Aviso «Carpeta de apuntes indexada: 5 notas, 0 omitidas; la búsqueda cubre solo parte del contenido» (largo y alarmante sin explicación). (c) Cabecera «apuntes · 5 notas» pero pie «1–6 de 6».
- **Evidencia:** capturas 46, 124, 125.
- **Recomendación:** explicar qué se recupera y cuándo; acortar y aclarar el aviso de indexado; unificar conteos (carpetas vs. notas).

### QA-05 · Índice flotante: desborde, ausencia con split y espacio desaprovechado
- **Área / ref.:** índice flotante · R11, N1.
- **Severidad:** S3. **Frecuencia:** siempre en la configuración descrita.
- **Pasos:** Vista → Índice flotante; ventana maximizada; cerrar split.
- **Esperado:** tarjeta contenida; algo visible tras activarlo.
- **Observado:** la entrada activa «Asamblea Nacional (1789–1791)» sobresale del borde derecho de la tarjeta. Con dos paneles no aparece (el aviso lo explica, pero el usuario no ve nada). En ventanas anchas el texto queda pegado a la izquierda con gran vacío; la rueda sobre el índice mueve el documento, no la lista.
- **Evidencia:** capturas 94, 96, 111.
- **Recomendación:** truncar con elipsis o ensanchar la tarjeta; centrar o limitar el ancho de lectura; permitir scroll propio.

### QA-06 · Enlace roto poco distinguible y dependiente de la carpeta
- **Área / ref.:** enlaces entre notas · R14.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** abrir `inicio.md` sin carpeta; luego con carpeta abierta.
- **Esperado:** enlace roto claramente distinto en todo contexto.
- **Observado:** sin carpeta, «nota-que-no-existe» se ve igual que los válidos (verde subrayado); con carpeta pasa a subrayado punteado amarillo, diferencia sutil y solo por color/trazo. El tooltip («Esta nota no existe · clic para crearla») es claro.
- **Evidencia:** capturas 40, 46, 47.
- **Recomendación:** estilo roto más evidente (tachado suave o color de error + icono) y no depender solo del color.

### QA-07 · Paleta de acciones: botón y campo de búsqueda poco descubribles
- **Área / ref.:** paleta · R15, N8.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** observar el botón de la paleta y abrirla.
- **Esperado:** botón «···» (según hoja) y un campo de texto reconocible.
- **Observado:** el botón es `>_`; la paleta muestra un título «Buscar una acción · escribí para filtrar» sin caja de texto ni cursor visible; lo escrito aparece en el mismo título («Buscar una acción: tema · 1 resultado»).
- **Evidencia:** capturas 04, 05, 10, 12.
- **Recomendación:** caja de texto con borde y cursor; ajustar icono o hoja de requerimientos.

### QA-08 · Foco visible con estilos inconsistentes
- **Área / ref.:** teclado · R23, N8.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Pasos:** F6, flechas y Tab sobre la barra de herramientas.
- **Esperado:** un mismo anillo de foco.
- **Observado:** subrayado verde bajo el botón (Leer), recuadro (Nuevo documento, Buscar), tooltip automático y borde completo en campos.
- **Evidencia:** capturas 83, 86, 87.
- **Recomendación:** unificar el indicador de foco.

### QA-09 · Iconos ambiguos en la barra de formato
- **Área / ref.:** barra de formato · R6, N2.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** el icono de código en línea (cuarto) y el de resaltar se parecen a letras sueltas; solo el tooltip («Aplicar código en línea») los explica.
- **Evidencia:** capturas 59, 67.
- **Recomendación:** iconos más reconocibles o etiqueta corta.

### QA-10 · Buscar reemplaza la barra de herramientas y no se ve dónde se escribe
- **Área / ref.:** buscar en documento · R12.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** al abrir Buscar desaparecen todos los botones de la barra; el campo muestra «Buscar: la| · 1 de 6 · Enter siguiente · Mayús+Enter anterior · Esc cierra» como una sola cadena, sin distinguir lo escrito de las instrucciones; al crecer ocupa más ancho.
- **Evidencia:** capturas 55–57.
- **Recomendación:** campo de texto separado, contador a la derecha y botones anterior/siguiente.

### QA-11 · Resultados de búsqueda en carpeta truncados y títulos distintos al nombre de archivo
- **Área / ref.:** panel Buscar · R13, N6.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Observado:** resultados como «Fotosíntesis · contenido: Relacionado: [[…» sin ver la coincidencia, y mezcla de título (La célula) con nombre de archivo (inicio.md en Enlazan aquí). La cabecera muestra un cursor literal «celula|» en el texto. La etiqueta «Enlazan aquí» roza el borde del panel.
- **Evidencia:** capturas 50–52.
- **Recomendación:** mostrar fragmento con la coincidencia resaltada; usar un solo criterio de nombre; acortar o reubicar la etiqueta.

### QA-12 · Segundo color de acento (azul) en enlaces web
- **Área / ref.:** estilo · N8.
- **Severidad:** S4. **Frecuencia:** siempre que hay enlaces web.
- **Observado:** «Wikipedia» en azul con icono ↗; el resto de la app usa un único acento verde. (La distinción es útil, pero rompe la regla declarada; los colores de resaltado de estudio son una excepción razonable.)
- **Evidencia:** capturas 40–41.
- **Recomendación:** distinguir con el icono ↗ y el subrayado, manteniendo el acento verde, o documentar la excepción.

### QA-13 · Enter en elemento vacío deja líneas en blanco de más
- **Área / ref.:** listas · R7.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** al terminar la lista, se generan dos líneas en blanco consecutivas (líneas 11–13 en la captura 62) en lugar de una.
- **Evidencia:** capturas 61–62.
- **Recomendación:** limpiar la viñeta vacía sin añadir línea extra.

### QA-14 · Avisos largos y repetitivos
- **Área / ref.:** avisos · N7.
- **Severidad:** S4. **Frecuencia:** frecuente.
- **Observado:** «Tarea actualizada · sin guardar: Ctrl+S la guarda, Ctrl+Z la deshace» (más de 70 caracteres, permanece sobre el texto varios segundos); «Dividir a la derecha · documento nuevo, abrir un archivo o una pestaña abierta» repite el título del selector; «Panel dividido a la derecha · el nuevo documento se enfoca con un clic» sobre contenido.
- **Evidencia:** capturas 31, 72, 73.
- **Recomendación:** avisos de una línea corta (≤ 40 caracteres) y sin repetir lo que ya se ve.

### QA-15 · Paneles divididos estrechos pierden controles de modo
- **Área / ref.:** split a 640 px · R5, N1.
- **Severidad:** S3. **Frecuencia:** siempre en paneles estrechos.
- **Observado:** a 1100 px cada panel tiene «Leer / Editar / Comparar»; a 640 px solo queda el botón de cerrar panel, y el cambio de modo exige el control global.
- **Evidencia:** capturas 73 vs. 77.
- **Recomendación:** menú compacto de modo en cada panel.

### QA-16 · Paneles laterales se cierran solos sin avisar
- **Área / ref.:** panel Archivos/Índice · N3.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** al abrir una nota (clic en enlace) o saltar a un título con Enter, el panel lateral se cierra. Escribir tras esa acción no hace nada visible (el campo ya no existe), lo que desorienta.
- **Evidencia:** capturas 48–51, 121.
- **Recomendación:** mantener el panel abierto (opción) o avisar con un texto breve.

### QA-17 · Menú «Estudio» largo y con jerga
- **Área / ref.:** menú Estudio · N6, R6.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Observado:** más de 20 elementos con scroll: «Anotaciones sin lugar», «Cambiar la forma de la marca», «Guardar copia sin anotaciones». No hay explicación de qué es una «marca de libro».
- **Evidencia:** captura 66.
- **Recomendación:** dividir en submenús, simplificar nombres y mostrar una ayuda corta.

### QA-18 · Detalles de consistencia y presentación
- **Área / ref.:** estilo · N8.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:**
  - Tablas con tipografía monoespaciada, distinta de la serif del cuerpo.
  - Dos filas resaltadas a la vez en paletas y listas (hover + selección de teclado, Índice tras End).
  - Números de línea ligeramente desalineados en vertical respecto al texto (capturas 99–100).
  - El editor ocupa ~740 px con vacío a la derecha en ventanas anchas.
  - Menú Archivo con la opción técnica «Abrir archivos externos en la ventana abierta: activado ✓».
  - Tooltip «Cerrar» tapa los botones del panel derecho en la captura 93.
- **Evidencia:** capturas 35, 11, 120, 99, 100, 7, 93.
- **Recomendación:** pulir tipografía de tablas, un solo resaltado activo, alinear números y reubicar tooltips.

### QA-19 · Error de apertura sin tratamiento visual propio
- **Área / ref.:** error al abrir archivo · heurísticas 9.
- **Severidad:** S3. **Frecuencia:** al abrir un archivo inexistente o ilegible.
- **Observado:** el texto («No se pudo abrir este documento. Revisa que exista, que sea legible y que no supere el límite de apertura.») aparece como párrafo normal del documento, sin icono ni color; además se abre una pestaña «noexiste.md» vacía y permite cambiar de modo. El texto usa «Revisa» (tuteo) y el resto de la app usa voseo («Revisá»).
- **Evidencia:** captura 106.
- **Recomendación:** panel de error con icono y color de error, botón para cerrar la pestaña, y voseo coherente.

## 6. Fortalezas

- Identidad visual coherente: noche verde oscuro y día crema, tipografía serif de lectura muy cómoda, sombras solo en elementos flotantes.
- Contraste excelente en ambos temas (texto ≥ 11,8:1; secundario ≥ 5,7:1).
- Tooltips con atajos en cada icono; menús bien agrupados con atajos; paleta potente con filtro, estado vacío y contador.
- Diálogos de cierre claros, con lista de archivos y Esc seguro; Guardar como acción principal.
- Paneles que se abren y cierran con el mismo botón y quedan marcados.
- Comparar con scroll sincronizado muy fiel; búsqueda con resaltado y marcas en el scroll; Índice navegable con rueda y teclado.
- Copiar copia exactamente el código y confirma con «✓ Copiado».
- Adaptación a 640×480 sin encimar controles (menús al «☰», barra compacta).
- Pantalla de bienvenida sobria y con atajos.
- Voz de los textos cercana y en español.

## 7. Recomendaciones priorizadas

1. **Pestañas (QA-01):** scroll horizontal o menú de pestañas, numerar «sin título», separar «•» de «×».
2. **Formato sin Markdown crudo (QA-02):** no insertar relleno visible ni pegado; modo de escritura.
3. **Mensajería de recuperación y carpeta (QA-04):** textos coherentes y breves.
4. **Estados de interacción (QA-03, QA-08, QA-18):** hover/pulsado de Copiar, foco unificado, un solo resaltado activo.
5. **Enlaces (QA-06, QA-12):** estilo de enlace roto evidente y sin depender de carpeta; un único acento.
6. **Índice flotante y anchos (QA-05):** contener la tarjeta, limitar ancho de lectura y centrarlo.
7. **Paleta y búsqueda (QA-07, QA-10, QA-11):** campos de texto visibles, fragmentos con coincidencia.
8. **Split y estrechos (QA-15, QA-16):** menú de modo por panel y panel lateral estable.
9. **Microcopy (QA-14, QA-17, QA-19):** avisos más cortos, menú «Estudio» simplificado, voseo uniforme.
10. **Pulido (QA-09, QA-13):** iconos más reconocibles y limpieza de líneas en blanco.
