# Informe de QA manual de UI/UX: Visor MD

Evaluación del 21 de noviembre de 2026 (prompt del 14 de noviembre). Todo lo reportado se observó en la app en ejecución (copia `qa`) y en la hoja de requerimientos. No se leyó código ni documentación del proyecto. Las capturas están en `C:\Dev\archivo\eval-ronda-6\qa\capturas\`.

---

## 1. Resumen ejecutivo

**Calificación UX: 7,5 / 10.**

Es una app con una identidad visual clara y coherente («Papel y tinta»), un microcopy en español rioplatense cuidado, buenos diálogos y excelente descubribilidad por tooltips con atajo. Los fallos graves son pocos, pero hay uno de integridad de datos (pestañas duplicadas del mismo archivo) y varios de consistencia en los menús, los paneles y la ventana mínima.

**Los 5 problemas principales**

1. **QA-01 (S2):** el mismo archivo se puede abrir en dos pestañas con contenido distinto (una sucia y una limpia). Los enlaces `[[nota]]` y el panel de archivos abren la copia limpia. Riesgo de sobrescribir cambios.
2. **QA-02 (S2):** el panel «···» de la hoja de requerimientos es un ícono `>_` (terminal), sin etiqueta. Es el único acceso a «todas las acciones» y su ícono no lo comunica.
3. **QA-03 (S3):** en 640×480 el menú contextual se sale de la ventana (borde derecho e inferior) y tapa la barra de estado. Los paneles laterales aplastan el contenido, y las pestañas truncan hasta la pestaña activa.
4. **QA-04 (S3):** las barras de búsqueda (en documento, en carpeta, paleta y selector de emojis) no tienen campo de texto visible ni botones. La consulta, las instrucciones y el contador van como una sola línea de texto.
5. **QA-05 (S3):** el menú superior no permite pasar de un menú abierto a otro con un clic (hay que cerrar y volver a abrir). Con el teclado, el foco salta y se ven varios resaltados a la vez.

**Conteo de requerimientos:** 23 funcionales + 8 no funcionales = 31. Cumple: 15. Cumple parcialmente: 14. No cumple: 0. No verificable: 2 (R19 y R21, con las herramientas disponibles).

**Conteo de hallazgos:** 24 en total. S1: 0. S2: 2. S3: 8. S4: 14.

---

## 2. Alcance y entorno

- **App:** Visor MD, copia `qa` con perfil limpio. Windows 10 (pantalla de 1366×738 al maximizar).
- **Tamaños probados:** 1100×720, 640×480 y maximizada (1366×738).
- **Temas:** noche y día, y los dos tamaños pequeños en ambos temas.
- **Interacciones:** unas 160 (clics, hover, rueda, teclado, arrastres).
- **Archivos usados:** `historia.md`, `inicio.md`, `manual-largo.md` (49 títulos), `biologia\celula.md`, `biologia\fotosintesis.md` y uno nuevo creado por mí (`prueba-qa.md`, dentro de `qa\apuntes\`).
- **Contraste:** medido sobre las capturas con un script propio, con píxeles de texto. Es una aproximación por el suavizado de bordes.
- **Límites declarados de la herramienta (no se reportan como hallazgos):** no llegan Ctrl, Alt ni Shift (no se probaron atajos con modificadores). `type` no escribe mayúsculas ni tildes. Los diálogos nativos están reemplazados.
- **No se probó:** exportar a PDF (R19), recuperación tras cierre brusco (R21), más de 2 paneles en el mosaico, ni selección de texto con formato (negrita, cursiva, etc.), porque no hay Shift ni arrastre fino de selección.

---

## 3. Matriz de requerimientos

| ID | Veredicto | Evidencia |
| --- | --- | --- |
| R1 | Cumple | `historia.md` abre formateado en Leer (`06-abrir`, `07-leer`). Tablas, citas, bloques de código y enlaces se ven bien (`39b`). |
| R2 | Cumple | Nuevo documento con ayuda «Escribí aquí…» (`70`). Guardar con nombre funciona y el toast dice «Guardado» (`73`). La pestaña pasó de «sin título 6» a `prueba-qa.md`. |
| R3 | Cumple | Leer, Editar (fuente con números de línea) y Comparar (lado a lado) (`23`, `24`). Sincronización de desplazamiento visible en `24`. F2/F3 cambian de modo (`97`, `98`). |
| R4 | Cumple parcialmente | Abrir, cambiar, cerrar, reordenar (arrastre, `89`) y fijar (`90`) funcionan. La posición de lectura se conservó al volver a `inicio.md`. Problema: el mismo archivo abre en pestañas duplicadas (QA-01) y los nombres iguales se truncan igual en 640 (QA-08). |
| R5 | Cumple parcialmente | Dividir a la derecha funciona, con selector de qué mostrar y cierre del panel (`57`, `60`). Solo probado con 2 paneles. En 640 se aplasta (QA-03). |
| R6 | Cumple parcialmente | Barra de formato completa y con tooltips (`01`, `74`, `75`). Menú Párrafo (H1–H6, listas, tarea, cita), Insertar (enlace, tabla, recuadro, bloque, regla, emoji). Tabla y emoji se insertaron bien (`78`–`80`). No pude probar negrita/cursiva sobre una selección. |
| R7 | Cumple parcialmente | Enter en «- dos» continuó la lista y un segundo Enter en el elemento vacío la terminó. Pero quedaron dos líneas vacías (líneas 4 y 5) en vez de una (`72`, ver QA-17). |
| R8 | Cumple | Botón **Copiar** en cada bloque. El portapapeles contiene solo el código (verificado en los dos bloques de `inicio.md`). El botón cambia a «✓ Copiado» (`42`). La confirmación dura menos de 1 s (QA-13). |
| R9 | Cumple | Una casilla se marcó con un clic en Leer (`21`). El toast explica Ctrl+S / Ctrl+Z. |
| R10 | Cumple | El triángulo pliega y despliega secciones (`49`). Queda un hueco extra bajo el título plegado (QA-18). |
| R11 | Cumple | Panel Índice con 7 y 49 títulos, salto al clic (`18`, `52`) e índice flotante en 1366 de ancho (`88`). Matiz: el título «actual» no siempre coincide con lo visible (QA-14). |
| R12 | Cumple | Resaltado de todas las coincidencias, la actual más marcada, contador «1 de 2», Enter siguiente y Esc cierra (`19`). Ignora tildes. Pero la interfaz es poco clara (QA-04). |
| R13 | Cumple parcialmente | Elegir carpeta, árbol de archivos, búsqueda en todos (`93`), Backlinks (`38`) y enlaces `[[nota]]` funcionan. Inconsistencias de conteo y un aviso confuso (QA-10, QA-11). |
| R14 | Cumple | Hover en enlace web muestra «enlace web: https://es.wikipedia.org/wiki/Markdown» (`59`). Los enlaces web tienen ícono ↗ y color azul distinto. El enlace roto es punteado amarillo (`61`) y abre un diálogo claro «Esta nota no existe» (`62`). Hay un residuo visual del tooltip (QA-12). |
| R15 | Cumple parcialmente | La paleta existe, se filtra escribiendo y muestra el contador y la categoría (`32`, `33`, `99`). Falla en la forma: el botón es un `>_` y no «···» (QA-02) y no hay campo de entrada visible (QA-04). |
| R16 | Cumple | Clic derecho distingue tarea (`22`), texto (`58`), tabla (`95`) y pestaña (`55`). En el texto sin selección ofrece «Copiar texto» (QA-19). |
| R17 | Cumple | Tema de día/noche por el botón sol/luna y por Vista (`17`). «Movimiento reducido» alterna y avisa (`100`). |
| R18 | Cumple | La chincheta se rellena en verde y avisa «Ventana fijada encima…» (`82`). No pude verificar el efecto «encima de otras ventanas» con las herramientas; sí el estado visual y el aviso. |
| R19 | No verificable | «Exportar a PDF» existe en el menú Archivo (`09b`). No se ejecutó (abre un diálogo de archivo y salida externa). |
| R20 | Cumple | Cierre con cambios muestra «¿Guardar los cambios de «historia.md»?» con tres botones claros (Cerrar sin guardar / Cancelar / Guardar). Esc elige Cancelar (`66`, `67`). Sin Sí/No. |
| R21 | No verificable | Hay opción «Recuperación de cambios: activado ✓» y «Abrir recuperación» en el menú Carpeta (`13`). No se pudo provocar un cierre brusco. |
| R22 | Cumple | Barra de estado: palabras, caracteres, línea y columna (en edición), modo y estado de guardado, «carpeta activa» (`23`, `73`). |
| R23 | Cumple parcialmente | Tab y F6 mueven el foco con foco visible (`53`, `54`). Flechas, PgDn, Esc y Enter funcionan en menús y paneles. Los atajos están comunicados en menús y tooltips. Ctrl+* no probable. Home/End no desplazan en Leer (QA-09). |
| N1 | Cumple parcialmente | A 640×480 no hay controles ocultos: los menús se pliegan en «≡» (`25`, `27`). Pero el menú contextual desborda y los paneles aplastan el contenido (QA-03). Maximizada: el texto queda pegado a la izquierda con gran vacío a la derecha (QA-06). |
| N2 | Cumple | Hay hover visible en botones, pestañas, menús y enlaces, y respuesta al pulsar. Las pestañas inactivas y los botones de modo del panel dividido tienen un hover débil. |
| N3 | Cumple parcialmente | Archivo/Editar/Vista/Carpeta, paleta, Índice y paneles se cierran con un segundo clic y muestran estado pulsado (`10`, `35`, `18b`). Falla al cambiar de un menú abierto a otro (QA-05). |
| N4 | Cumple parcialmente | La rueda recorre lista de índice (`51`), menú de 640 (`30`) y documento largo (`46`). Teclado: flechas y PgDn sí; el foco salta en el menú largo (QA-05) y Home/End no funcionan en Leer (QA-09). |
| N5 | Cumple parcialmente | Texto de cuerpo: 14,95:1 en noche y 13,1:1 en día (cumple 7:1). Secundario: 6,4:1 en noche (estado, modos inactivos), 6,1:1 en día; pestañas inactivas 5,9:1 en día; título del índice flotante 5,7:1 en noche. Todo ≥4,5:1. Pero el texto de interfaz «normal» (pestañas, estado) no llega a 7:1 en día. |
| N6 | Cumple parcialmente | El español es coherente y breve en general. El menú Estudio (jerga: «anotaciones sin lugar») y los avisos largos rompen la brevedad (QA-07, QA-11). |
| N7 | Cumple parcialmente | Muchos avisos son útiles (Copiado, Guardado, Tarea actualizada). Otros repiten lo obvio o tapan contenido (QA-11). |
| N8 | Cumple parcialmente | Los menús, diálogos y botones primarios son muy coherentes. Fallan los campos de búsqueda (cuatro versiones distintas), el nombre «documento/archivo/apunte» y las etiquetas de estado (QA-04, QA-15, QA-16). |

---

## 4. Heurísticas de Nielsen

| # | Heurística | Puntaje | Lo bueno | Problemas |
| --- | --- | --- | --- | --- |
| 1 | Visibilidad del estado | 4 | Barra de estado completa (modo, guardado, línea/columna). Punto «•» en pestañas sucias. Botón pulsado mientras el panel está abierto. Toasts. | Punto «•» muy pequeño. «Indexando carpeta» queda unos segundos en la barra. Guardar sin cambios no da respuesta. |
| 2 | Relación con el mundo real | 4 | Lenguaje de estudiante («Notas de la carpeta», «Estudio», «Cerrar sin guardar»). | «Anotaciones sin lugar», «Marca de libro», «Índice flotante» y «Mosaico» son jerga. |
| 3 | Control y libertad | 4 | Esc cierra todo. Cancelar seguro. Ctrl+Z comunicado. Cierre de panel conserva la pestaña. | Pestañas duplicadas (QA-01). No hay deshacer para «Quitar todas las anotaciones» más allá del texto. |
| 4 | Consistencia y estándares | 3 | Estilo visual, menús y diálogos muy uniformes. | Ícono de paleta, buscadores distintos, nombres de acciones y estados de menú (QA-02, QA-04, QA-15, QA-16). |
| 5 | Prevención de errores | 4 | Diálogo de cierre con Esc seguro. Nota inexistente pide confirmación. Enlace web muestra el destino. | Se puede duplicar el mismo archivo. |
| 6 | Reconocer antes que recordar | 3 | Tooltips con atajo en todos los íconos. Menús con atajos visibles. Paleta. | Muchos íconos sin etiqueta. La paleta aparece como `>_`. Los campos de búsqueda no se reconocen como campos. |
| 7 | Flexibilidad y eficiencia | 4 | Paleta de 77 acciones, atajos, F2/F3, F6, arrastre de pestañas. | Atajos con Ctrl no se pudieron comprobar. Home/End sin efecto en lectura. |
| 8 | Diseño estético y minimalista | 3 | Paleta de colores sobria, tipografía cuidada, lectura agradable. | Barra de herramientas densa de íconos. Menú Estudio de 20+ ítems. Encabezado doble de modo en paneles divididos. Cada línea de la edición es un bloque con fondo. |
| 9 | Ayudar a diagnosticar errores | 4 | Enlace roto distinguido y explicado. Aviso de carpeta indexada. | Aviso «0 omitidas; cubre solo parte del contenido» contradictorio (QA-11). |
| 10 | Ayuda y documentación | 3 | Pantalla de bienvenida con explicación de los tres modos y atajos. | La bienvenida no vuelve a mostrarse. No hay ayuda para las funciones de Estudio. No se explica qué es «Comparar» más allá de una línea. |

Promedio: 3,6 / 5.

---

## 5. Hallazgos

### QA-01 · Pestañas duplicadas del mismo archivo
- **Área / referencia:** Pestañas, panel de archivos, enlaces entre notas. R4, R13, N8.
- **Severidad:** S2. **Frecuencia:** siempre.
- **Pasos:** 1) Abrir `historia.md` y marcar una tarea (queda «sin guardar»). 2) Con la carpeta abierta, clic en `historia.md` del panel Archivos.
- **Esperado:** activar la pestaña ya abierta, o avisar.
- **Observado:** se abre una segunda pestaña «historia.md» con la versión guardada (limpia). La primera sigue sucia. El selector «Dividir» las lista con el mismo nombre (una con «•») y el enlace `[[historia]]` de `inicio.md` activó la copia limpia («Se activó la pestaña que ya contenía el documento»), no la sucia. Al guardar una se pisaría la otra.
- **Evidencia:** `37-celula.png`, `57a-picker.png`, `64-wikilink.png`.
- **Recomendación:** evitar duplicados por ruta (activar la pestaña existente, preferentemente la sucia). Si se permite, diferenciar los nombres (carpeta) y advertir de cambios en el otro duplicado.

### QA-02 · La paleta de acciones es un ícono de terminal
- **Área / referencia:** Barra superior. R15, heurísticas 4 y 6.
- **Severidad:** S2. **Frecuencia:** siempre.
- **Pasos:** observar el botón junto a «Carpeta».
- **Esperado:** botón «···» según la hoja.
- **Observado:** ícono `>_` (consola) sin etiqueta. Solo el tooltip «Buscar una acción · Ctrl+Shift+P» lo explica. Un estudiante que no programa lo asocia a la terminal y no lo toca.
- **Evidencia:** `04-tip-paleta.png`, `32-paleta.png`.
- **Recomendación:** usar «···» o una lupa con «Acciones». Dejar el tooltip.

### QA-03 · Ventana mínima 640×480: desbordes y aplastamientos
- **Área / referencia:** Menú contextual, paneles, pestañas. N1, N6.
- **Severidad:** S3. **Frecuencia:** siempre en 640×480.
- **Pasos:** a) Clic derecho en la tabla de `inicio.md`. b) Abrir Índice o Buscar. c) Tener 6 pestañas abiertas.
- **Esperado:** el menú cabe en la ventana. El contenido sigue legible. Las pestañas se distinguen.
- **Observado:** a) el menú contextual se sale por la derecha y por abajo, tapa la barra de estado y corta «Copiar tabla para hoja de cálculo». b) el panel ocupa 280 px de 640 y la tabla del documento queda recortada («Fotosíntesis» pisa «1,5»). c) las pestañas se truncan a «hist…», «histori….», «prue-…» y hasta la activa («fotosí…»), sin desplazamiento ni menú de pestañas.
- **Evidencia:** `95-ctx-tabla.png`, `92-640-buscar-panel.png`, `91-640-tabs.png`, `94-resultado.png`.
- **Recomendación:** reposicionar el menú dentro de la ventana. Panel lateral como superposición en anchos pequeños. Ancho mínimo por pestaña con desplazamiento y menú de pestañas.

### QA-04 · Campos de búsqueda sin campo visible
- **Área / referencia:** Buscar en documento, Buscar en carpeta, paleta, selector de emojis. R12, R13, R15, N8, heurística 6.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** abrir cada buscador y escribir.
- **Esperado:** un campo de texto reconocible, con botones anterior/siguiente y cerrar.
- **Observado:** a) Buscar en documento: una caja única con «Buscar: asamblea| · 1 de 2 · Enter siguiente · Mayús+Enter anterior · Esc cierra» (instrucciones dentro de lo escrito, sin botones). Reemplaza toda la barra de herramientas. b) Paneles y paleta: la primera fila es un título que cambia («Buscar una acción: tabla · 2 resultados»). En 640 se trunca («escribí lo que bu…»). Cuatro presentaciones distintas para lo mismo.
- **Evidencia:** `19-buscar.png`, `33-paleta-filtra.png`, `92`, `93`, `78-emoji.png`.
- **Recomendación:** un componente único: campo con marcador de posición, contador separado y botones ↑ ↓ ✕. Atajos como ayuda secundaria.

### QA-05 · Menú superior: no se cambia con un clic y el foco salta
- **Área / referencia:** Menús Archivo/Editar/Vista/Carpeta y menú «≡». N3, N4, heurística 4.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** 1) Abrir Editar. 2) Clic en Vista. 3) En el menú «≡» de 640, bajar con la rueda y pulsar flecha abajo.
- **Esperado:** (2) se abre Vista directo. (3) el foco sigue la posición visible.
- **Observado:** (2) el primer clic solo cierra Editar y deja Vista resaltado sin abrir; hay que clicar de nuevo. (3) el menú volvió al inicio y quedaron tres filas resaltadas a la vez (hover y teclado).
- **Evidencia:** `12-vista.png`, `31-menu-teclado.png`.
- **Recomendación:** al estar un menú abierto, el clic o el hover en otro lo abre. Un solo resaltado activo.

### QA-06 · Maximizada: documento pegado a la izquierda
- **Área / referencia:** Área de lectura y edición. N1.
- **Severidad:** S3. **Frecuencia:** siempre en ventanas anchas.
- **Pasos:** maximizar (1366 px) y abrir cualquier nota en Leer o Editar.
- **Esperado:** columna de lectura centrada o con índice a un lado.
- **Observado:** la columna de ~740 px queda en el borde izquierdo (x=48) y queda ~590 px vacíos a la derecha. En Editar, la fila de cada línea ocupa 740 px y la pantalla de bienvenida sí está centrada (inconsistente). El índice flotante solo aparece si se activa en Vista y no se avisa en el momento (no hay toast al activar en 1100 px).
- **Evidencia:** `84-max.png`, `85-max-leer.png`, `88-flotante.png`.
- **Recomendación:** centrar la columna. Activar el índice flotante por defecto en anchos grandes o avisar por qué no se ve.

### QA-07 · Menú «Estudio» abrumador
- **Área / referencia:** Barra de herramientas. N6, heurísticas 2 y 8.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Observado:** más de 20 ítems con jerga («Anotaciones sin lugar», «Restaurar las anotaciones quitadas», «Guardar copia sin anotaciones», «Marcas de libro») y desplazamiento propio. El botón se ve igual en Leer y en Editar. La función no se explica en la interfaz.
- **Evidencia:** `76-estudio.png`.
- **Recomendación:** agrupar en 2 o 3 submenús, renombrar con lenguaje de estudiante y dar una breve explicación en la primera apertura.

### QA-08 · Pestañas con nombre idéntico indistinguibles
- **Área / referencia:** Pestañas. R4, heurística 6.
- **Severidad:** S3. **Frecuencia:** cuando hay nombres repetidos o ventanas angostas.
- **Observado:** `historia.md` y `historia.md •` solo se distinguen por un punto de 3 px. En 640 ambas son «hist…» / «histori….». Un archivo en subcarpeta con nombre repetido tampoco se desambigua.
- **Evidencia:** `91-640-tabs.png`, `57a-picker.png`.
- **Recomendación:** mostrar la subcarpeta cuando coincide el nombre, y un punto más grande (o color) para «sin guardar».

### QA-09 · Home/End no desplazan en modo Leer
- **Área / referencia:** Teclado en Leer. R23, N4.
- **Severidad:** S3. **Frecuencia:** siempre.
- **Pasos:** abrir `manual-largo.md`, pulsar End y Home (también luego de PgDn).
- **Esperado:** ir al final y al inicio.
- **Observado:** sin efecto (PgDn y flechas sí).
- **Evidencia:** `45-manual-end.png`, `48-end.png`.
- **Recomendación:** soportar Home/End y Espacio.

### QA-10 · Conteos inconsistentes en el panel de archivos
- **Área / referencia:** Panel Archivos. R13.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** el título dice «apuntes · 5 notas» y el pie «1–6 de 6» (cuenta la carpeta). Tras plegar «biologia» pasa a «1–4 de 4» con el título igual.
- **Evidencia:** `36-carpeta.png`, `36b.png`.
- **Recomendación:** que el pie cuente solo lo que se ve o que ambos expresen lo mismo.

### QA-11 · Avisos largos, repetidos o contradictorios
- **Área / referencia:** Toasts. N6, N7.
- **Severidad:** S3. **Frecuencia:** habitual.
- **Observado:** «Carpeta de apuntes indexada: 5 notas, 0 omitidas; la búsqueda cubre solo parte del contenido» (alarmante y contradictorio, 90 caracteres). «Tarea actualizada · sin guardar: Ctrl+S la guarda, Ctrl+Z la deshace». «Dividir a la derecha · documento nuevo, abrir un archivo o una pestaña abierta» (repite el menú que se acaba de ver). «Panel dividido a la derecha · el nuevo documento se enfoca con un clic». «Leer» al cambiar de modo. Los toasts quedan encima de la última línea del texto (y en Comparar tapan texto).
- **Evidencia:** `36-carpeta.png`, `21-check.png`, `56-dividir.png`, `57-split.png`, `24-comparar.png`.
- **Recomendación:** avisos de una sola idea y menos de 40 caracteres. Quitar los que repiten lo que acaba de pasar o el menú. Mover la ayuda a tooltips.

### QA-12 · Tooltips: posición desacoplada y residuos
- **Área / referencia:** Tooltips y diálogos. N2, heurística 1.
- **Severidad:** S4. **Frecuencia:** siempre (posición); ocasional (residuo).
- **Observado:** a) todos los tooltips de la barra superior aparecen a ~100 px de su control, en una banda fija (y≈126), y alineados al borde izquierdo del botón, a veces lejos del puntero. b) tras pulsar un enlace roto, el tooltip «Esta nota no existe…» quedó flotando arriba a la derecha, por encima del velo del diálogo. c) el tooltip de «Notas que enlazan aquí» se corta en el borde de mi captura reducida (no en la app).
- **Evidencia:** `02-tip-nuevo.png`, `81-hover-pin.png`, `62-click-roto.png`.
- **Recomendación:** anclar bajo el control. Descartar el tooltip al abrir un diálogo.

### QA-13 · Confirmación de «Copiar» muy breve
- **Área / referencia:** Bloques de código. R8, N7.
- **Severidad:** S4. **Frecuencia:** variable.
- **Observado:** el botón cambia a «✓ Copiado» (verde más intenso), pero en una captura tomada 0,3 s después del clic ya no estaba, y a 1 s tampoco. El contenido sí se copia bien. Para personas lentas o con baja visión puede pasar desapercibido.
- **Evidencia:** `41-copiar-click.png` (sin cambio a 0,3 s), `42-copiar-py.png` (con confirmación), `43-copiar-1s.png` (ya revertido).
- **Recomendación:** mantener «Copiado» unos 1,5 s.

### QA-14 · Título «actual» del índice desfasado
- **Área / referencia:** Índice. R11.
- **Severidad:** S4. **Frecuencia:** ocasional.
- **Observado:** al abrir el índice con el documento en la parte superior resalta «Causas» en vez de «Revolución francesa». En el índice flotante (1366) marcaba «Capítulo 13» con «Capítulo 12» al tope. El resaltado de hover y el actual son casi iguales.
- **Evidencia:** `18-indice.png`, `88-flotante.png`.
- **Recomendación:** calcular el título actual por el borde superior de la vista. Diferenciar hover y actual.

### QA-15 · Terminología inconsistente
- **Área / referencia:** Textos. N6, N8.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** «Nuevo apunte» (bienvenida) vs «Nuevo documento» (menú y tooltip); «Abrir archivo» vs «Abrir documento»; «Carpeta de apuntes» (primer botón de la barra) vs «Abrir carpeta» (cuarto), con íconos muy parecidos y casi la misma función. «Notas» vs «apuntes» vs «documentos».
- **Evidencia:** `01-arranque.png`, `09b-menu-hover.png`, `08-tb-*` (montaje).
- **Recomendación:** elegir un vocabulario («apunte» o «documento») y diferenciar los dos botones de carpeta.

### QA-16 · Estados de interruptores en los menús
- **Área / referencia:** Menús Vista, Archivo y Carpeta. N8, heurística 4.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** «Tema de noche: activado ✓» repite el estado dos veces (texto y marca). La etiqueta cambia de significado con el estado (en día aparece «Tema de noche: desactivado»). «Abrir archivos externos en la ventana abierta: activado ✓» es una frase larga. «Movimiento reducido: desactivado» sin marca.
- **Evidencia:** `15-vista.png`, `09b-menu-hover.png`, `13-carpeta.png`.
- **Recomendación:** etiqueta fija («Tema de noche») con marca ✓ o interruptor.

### QA-17 · Enter en elemento vacío deja dos líneas vacías
- **Área / referencia:** Editar. R7.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Pasos:** escribir «- uno», Enter, «dos», Enter, Enter.
- **Esperado:** se quita la viñeta vacía y se sale de la lista.
- **Observado:** se quitó la viñeta, pero quedaron dos líneas vacías (líneas 4 y 5) y el cursor en la 5.
- **Evidencia:** `72-lista-fin.png`.
- **Recomendación:** salir de la lista dejando una sola línea vacía.

### QA-18 · Hueco bajo secciones plegadas
- **Área / referencia:** Leer. R10.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** el título «Capítulo 4» plegado deja una separación mayor que entre otros títulos, y no muestra cuánto contenido oculta.
- **Evidencia:** `49-plegado.png`.
- **Recomendación:** ajustar el espaciado y mostrar un indicador («3 líneas»).

### QA-19 · Menú contextual sin selección
- **Área / referencia:** Menú contextual. R16.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** clic derecho en un espacio vacío del documento ofrece «Copiar texto» y «Copiar Markdown original» sin selección. No se aclara qué copian.
- **Evidencia:** `58-ctx-texto.png`.
- **Recomendación:** nombrar el alcance («Copiar párrafo») o desactivar sin selección.

### QA-20 · Falta de estados deshabilitados
- **Área / referencia:** Menús y barras. N2, N8.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** «Deshacer», «Rehacer» y «Cancelar actualización de carpeta» se ven activos sin historial ni actualización en curso. La barra de formato de la pantalla de bienvenida (sin documento) se ve plenamente activa. «Guardar» en un documento ya guardado no da ninguna respuesta.
- **Evidencia:** `11-editar.png`, `13-carpeta.png`, `01-arranque.png`.
- **Recomendación:** atenuar lo que no aplica y mostrar el motivo en el tooltip.

### QA-21 · Edición: cada línea es un bloque con fondo y números algo desalineados
- **Área / referencia:** Modo Editar. Estilo «todo plano», N8.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** cada línea de la fuente se pinta como un bloque con fondo (en noche casi invisible, en día muy marcado, `23`), mientras la declaración de estilo dice que dentro del documento todo es plano. Los números de línea quedan ~2 px por encima de la base del texto. Las líneas largas se ajustan, pero los números no muestran continuación.
- **Evidencia:** `23-editar.png`, `72-lista-fin.png`.
- **Recomendación:** quitar el fondo por línea (o usar solo el de la línea actual) y alinear la base de los números.

### QA-22 · Alineación de bloques en Leer
- **Área / referencia:** Leer. Jerarquía/alineación.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** los bordes izquierdos no coinciden: títulos y texto x=48, cita x=48 (con borde), tabla x=42 y bloque de código x=36. Anchos distintos (cita 740, tabla 732, código 745). Filas de la tabla con línea doble y alturas desiguales.
- **Evidencia:** `39b-inicio-sin-panel.png`.
- **Recomendación:** una sola columna de contenido con bordes comunes y trazos de 1 px.

### QA-23 · Íconos del panel dividido y zonas de clic pequeñas
- **Área / referencia:** Mosaico, pestañas, controles. N2, accesibilidad.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** los botones de modo del panel dividido miden ~40×20 px y el de «cerrar panel» ~16 px, además de duplicar los botones Leer/Editar/Comparar de la barra superior. La «x» de pestaña y el punto de sin guardar rondan 12 px.
- **Evidencia:** `57-split.png`.
- **Recomendación:** zonas de clic de al menos 24–28 px y evitar duplicar los controles de modo.

### QA-24 · Pestaña «Enlazan aquí» recortada
- **Área / referencia:** Panel lateral. N1, N8.
- **Severidad:** S4. **Frecuencia:** siempre.
- **Observado:** la cuarta pestaña del panel («Enlazan aquí») toca el borde del panel y se corta en 1100 (el subrayado llega al borde), y su nombre no coincide con el del botón «Notas que enlazan aquí».
- **Evidencia:** `18-indice.png`, `38-backlinks.png`.
- **Recomendación:** acortar a «Enlaces» o dar más ancho.

---

## 6. Fortalezas

- **Identidad visual sólida:** «Papel y tinta» se cumple. Un solo acento verde, plano en el documento y sombra suave solo en menús, diálogos y avisos.
- **Dos temas bien resueltos:** el día y la noche mantienen jerarquía y contraste. El cuerpo supera 13:1 en ambos.
- **Tipografía de lectura excelente:** serif para el texto, monoespaciada para código y tablas, títulos claros.
- **Tooltips con atajo en todos los íconos** y menús con atajos visibles: gran descubribilidad.
- **Diálogos modelo:** cierre con cambios y nota inexistente con copy claro, botón primario destacado y Esc seguro.
- **Botón Copiar:** copia solo el código y confirma con «✓ Copiado».
- **Enlaces bien diferenciados:** externos (azul + ↗), de nota (verde) y rotos (punteado amarillo), con destino real al pasar el puntero.
- **Foco de teclado visible:** F6 y Tab muestran el foco con tooltip.
- **Adaptación a 640 px:** los menús se pliegan en «≡» y la barra se compacta, sin controles ocultos.
- **Paneles y listas con contador y ayuda de teclado** («1–7 de 7 · Esc para cerrar»). La rueda funciona en todas.
- **Barra de estado rica y concisa.**
- **Búsqueda tolerante a tildes** y apertura del resultado con el término resaltado.

---

## 7. Recomendaciones priorizadas

**Prioridad alta (corregir antes de liberar)**
1. Evitar pestañas duplicadas del mismo archivo, o diferenciarlas y advertir (QA-01).
2. Cambiar el ícono de la paleta a «···» o lupa con etiqueta (QA-02).
3. Arreglar los desbordes en 640×480: menú contextual, paneles como superposición y pestañas con desplazamiento (QA-03).
4. Unificar el componente de búsqueda con campo visible y botones (QA-04).

**Prioridad media**
5. Menús: cambio con un clic entre menús abiertos y un solo resaltado (QA-05).
6. Centrar la columna en ventanas anchas y explicar el índice flotante (QA-06).
7. Simplificar el menú Estudio y su vocabulario (QA-07).
8. Desambiguar nombres de pestañas (QA-08) y soportar Home/End en Leer (QA-09).
9. Recortar y unificar los avisos (QA-11).

**Prioridad baja (pulido)**
10. Tooltips anclados y sin residuos (QA-12). «Copiado» más duradero (QA-13).
11. Terminología y estados de menú coherentes (QA-15, QA-16).
12. Conteos del panel, índice «actual», plegado, contextual y estados deshabilitados (QA-10, QA-14, QA-18, QA-19, QA-20).
13. Pulido visual: fondo por línea en Editar, alineación de bloques, zonas de clic y pestaña recortada (QA-21 a QA-24). Enter en lista vacía (QA-17).
