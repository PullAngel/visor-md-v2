# Informe de usabilidad: Visor MD (Lucía, ronda 3)

Capturas en `C:\Dev\archivo\eval-ronda-3\usuario\capturas\` (t0.png, t1.png, etc.).
Las "acciones" cuentan cada clic, escritura, rueda o tecla; no cuentan las capturas ni el `answer` que prepara el diálogo de archivos.

## 1. Resumen

- **Tasa de éxito:** 14 de 16 tareas con éxito, 2 con dificultad (T8 y T9), 0 fracasos. Éxito total o con dificultad: 100 %.
- **SUS:** 75 / 100.
- **Las 5 peores fricciones:**
  1. US-02: el botón Negrita sin texto elegido deja el cursor entre `**` y `**`. Si Lucía pulsa Enter ahí, rompe la negrita (T8 y T9).
  2. US-01: al poner el título con el menú Párrafo, el `#` terminó en un lugar raro (T8). Costó unas 12 acciones arreglarlo. El origen es incierto: puede ser la herramienta.
  3. US-03: la tabla se inserta como texto crudo con barras `|` y `---`. Lucía no sabe editarla (T9).
  4. US-04: subrayar exige selección; el aviso no explica cómo; el menú "Estudio" desaparece en modo Leer; el "amarillo" se ve verde oliva (T16).
  5. US-05: los resultados de la búsqueda aparecen cortados y hay un aviso confuso sobre que la búsqueda cubre "solo parte del contenido" (T6 y T7).

## 2. Primera impresión (test de 5 segundos, t0.png)

- **¿Qué es esta app?** "Es como un Word para apuntes, con una pantalla de bienvenida. Dice Visor MD y habla de apuntes."
- **¿Qué se puede hacer?** Abrir una carpeta de apuntes, abrir un archivo o crear un apunte nuevo. Arriba hay botones de Leer, Editar y Comparar, y una barra con negrita, cursiva, etc.
- **¿Qué me llama la atención?** Los tres botones grandes con atajos al costado. El fondo oscuro verde es lindo. La barra de iconos de formato se parece a la de Word.
- **¿Qué no entiendo?** "Leer, Editar, Comparar: ¿en qué se diferencian?" El texto de abajo lo explica ("Comparar muestra las dos cosas lado a lado"), pero "las dos cosas" no queda claro. Tampoco entiende los iconos sueltos (la H con una raya, el de enlace, el de `<>`) ni "Estudio". Piensa: "Markdown, ¿eso qué es?", y lo dice el link de abajo.
- Lucía no pulsó "Abrir tus archivos .md con Visor MD · Configurar": parecía un enlace de configuración del sistema.

## 3. Tabla de tareas

| # | Tarea | Resultado | Acciones | Confianza |
|---|-------|-----------|----------|-----------|
| 1 | Abrir inicio.md | Éxito | 1 | 5 |
| 2 | Marcar "Leer capítulo 3" | Éxito | 3 (+1 guardar) | 4 |
| 3 | Copiar el comando | Éxito | 1 | 5 |
| 4 | Manual largo, Capítulo 30 | Éxito | 4 | 4 |
| 5 | Volver a inicio | Éxito | 1 | 5 |
| 6 | Trabajar con la carpeta y abrir célula | Éxito | 3 | 4 |
| 7 | Buscar "Napoleón" en todo | Éxito | 5 (+ 11 de recuperación) | 4 |
| 8 | Crear resumen.md | Con dificultad | unas 27 | 3 |
| 9 | Tabla y emoji | Con dificultad | unas 20 | 3 |
| 10 | Texto por dentro y resultado lado a lado | Éxito | 1 (ya la había hecho en T8) | 5 |
| 11 | Dos apuntes lado a lado | Éxito | 4 | 4 |
| 12 | Achicar y fijar encima | Éxito | 4 (1 desperdiciada) | 4 |
| 13 | Claro y oscuro | Éxito | 2 | 5 |
| 14 | Exportar a PDF | Éxito | 2 | 4 |
| 15 | Cerrar sin guardar | Éxito | 3 | 5 |
| 16 | Exploración libre | Éxito (exploración) | unas 14 | 4 |

## 4. Detalle por tarea

**T1.** Lucía ve "Abrir archivo" y pulsa. El archivo abre directo en modo Leer (t1.png). "Se ve lindo, con tabla y todo."

**T2.** Hay que bajar con la rueda (2 giros) hasta "Tareas". Pulsó el cuadradito de "Leer capítulo 3" y se tildó al instante (t2b.png). Apareció un aviso: "Tarea actualizada · sin guardar: Ctrl+S la guarda, Ctrl+Z la deshace". La pestaña quedó con un puntito. Buscó el disquete de arriba y lo pulsó; el puntito se fue (t2c.png). Cita: "Ah, tengo que guardar. ¿Por qué no se guarda solo? En Docs sí."

**T3.** Vio el botón "Copiar" al lado del bloque bash y lo pulsó. Cambió a "Copiado" con una tilde (t3.png). El portapapeles tenía `ping 192.168.0.1` y `git status`. Cita: "Perfecto, esto es mejor que en Word." Duda: el bloque tiene dos comandos y no sabía si "el comando" era uno solo; copió el bloque entero.

**T4.** Abrió el manual con el icono de carpeta de arriba. Vio 40 capítulos largos y pensó: "Bajar a mano va a tardar". Probó los iconos de la segunda barra. El primero abre un panel "Índice · 49 títulos" (t4b.png). Lo bajó con la rueda y pulsó "Capítulo 30" (t4c.png). Fue directo al capítulo y el panel se cerró solo (t4d.png). Cita: "¡Un índice! Eso sí me sirve." Queja menor: hay que bajar la lista a mano, sin filtro por número.

**T5.** Pulsó la pestaña "inicio.md" y quedó en "Comandos útiles", donde estaba (t5.png). La posición se conservó. "Bien."

**T6.** Lucía buscó en el menú "Carpeta" y eligió "Abrir carpeta" (t6a.png). El aviso decía: "Carpeta de apuntes indexada: 5 notas, 0 omitidas; la búsqueda cubre solo parte del contenido" (t6b.png). Cita: "¿Cómo que solo parte del contenido? ¿Me falta algo?" Se abrió el panel Archivos con las notas y la subcarpeta biología. Pulsó celula.md y abrió (t6c.png).

**T7.** Pulsó la lupa de la segunda barra. Escribió "Napoleon" (sin tilde, porque la herramienta no pone tildes). La caja mostró "napoleonon" y 0 resultados (t7a.png). No sé si es de la app o de la herramienta que escribe, así que no lo cuento como defecto de la app. Borró todo con retroceso (11 pulsaciones) y escribió "napoleon" (t7c.png). Dio 1 resultado: "Revolución francesa · contenido: Termin…", con el texto cortado. Pulsó y abrió historia.md con "Napoleón" resaltado y una barra de búsqueda (t7d.png). Que encontrara "Napoleón" tipeando "napoleon" sin tilde y en minúsculas es un acierto. Cita: "No sé si el resultado es el único porque dice '1 de 1', pero me encantó que me lleve a la palabra."

**T8.** Pulsó "+" y salió una nota "sin título" en modo Editar con una guía: "Escribí aquí. El formato está en la barra de arriba" (t8a.png). Abrió el menú "Párrafo" (t8b.png) y eligió "Título 1 (H1)". Escribió "resumen" y pulsó Enter. Quedó "resumen" en la línea 1 y un `#` suelto en la línea 2 (t8c.png). Cita: "¿Qué es ese numeral? Yo quería un título." Con Comparar vio que no era título (t8d.png). Tras varios intentos, con retroceso y reaplicando el título, quedó `# resumen` y a la derecha se veía un título grande (t8h.png). Fueron unas 12 acciones de arreglo (hubo un `#` duplicado intermedio, t8e.png a t8g.png).

La lista funcionó bien: Párrafo → "Lista con viñetas", y Enter crea la siguiente viñeta (t8i.png). Para la negrita pulsó **B** sin texto elegido y escribió "importante". Apareció `**importante**` con el cursor en medio (t8j.png). Cita: "¿Y esos asteriscos? ¿Se van?" Guardó con el disquete, y como la nota era nueva se abrió el diálogo de archivos (preparado con `answer`); quedó `resumen.md` (t8k.png).

**T9.** Pulsó en un espacio vacío, Enter dos veces y el icono de tabla. El cursor seguía dentro de la negrita, así que Enter partió la negrita en dos (`**importante` y `**` aparte, t9a.png). Para repararlo hizo varios pasos (t9b.png). Hubo una rareza de la herramienta: al escribir `*` salió `+`, y escribir "importante" salió "impportante" (t9c.png); lo atribuyo a la herramienta. Rehízo la negrita seleccionando la palabra y usando B, y esa vía anduvo. La tabla es un texto crudo con barras: `| Columna | Valor |`, `| --- | --- |`, `| texto | valor |` (t9a.png). A la derecha se ve linda, pero Lucía no sabe cómo agregar filas. Cita: "Esto es como programar. ¿No puedo hacerlo con celditas como en Word?" El emoji fue fácil: Insertar → "Insertar símbolo o emoji", con una lista con nombres como "emoji destacado" (t9e.png, t9f.png). Insertó una estrella.

**T10.** En T8 ya había pulsado "Comparar". Dice "Vista dividida · fuente editable y lectura sincronizada" (t8d.png). Izquierda el texto "por dentro" con numeración de líneas, derecha el resultado. Cita: "Ah, esto es lo que hace el Markdown: lo de la izquierda es la receta y lo de la derecha el plato."

**T11.** Menú Vista → "Dividir a la derecha con otra pestaña" (t11a.png). Salió una lista "¿qué mostrar en el panel nuevo?" con Documento nuevo, Abrir un archivo e inicio.md, manual-largo.md, celula.md, historia.md (t11b.png). Eligió historia.md y quedó al lado de resumen.md (t11c.png). Cita: "Qué bueno que me deje elegir cuál." Duda: el panel de la izquierda quedó en Comparar y se apretó, con tres columnas; para cerrar el panel hay que reconocer un icono pequeño arriba a la derecha.

**T12.** Cerró el panel con ese icono y achicó la ventana a 800x500. Los menús "Archivo, Editar, Vista, Carpeta" se juntaron en un icono de hamburguesa (t12a.png). Pulsó "Vista" en su posición antigua y no pasó nada (1 acción perdida). Pasó el puntero por el pin de arriba: "Fijar la ventana encima de las demás" (t12b.png). Pulsó: "Ventana fijada encima de las demás · clic en la chincheta para soltarla" (t12c.png). Cita: "El cartelito me dijo qué hacía antes de pulsar."

**T13.** Pulsó el sol de arriba a la derecha y pasó a claro, con aviso "Tema de día · Ctrl+Shift+D para volver" (t13a.png). Pulsó de nuevo y volvió a oscuro (t13b.png). En modo claro el icono es una luna. Cita: "El icono muestra lo que va a pasar, no lo que hay. Me confundí un segundo."

**T14.** Archivo → "Exportar a PDF" (t14a.png). Se abrió el diálogo de guardar (preparado con `answer`) y se creó `resumen.pdf`. A los 3 segundos ya no había aviso (t14b.png). Lo repetí para ver el aviso (t14c.png): "PDF exportado: resumen.pdf · 1 página". Solo dice el nombre, no la carpeta. Cita: "¿Y dónde quedó? Tengo que acordarme de dónde lo guardé."

**T15.** Creó una nota nueva y escribió "hola esto es una prueba". La pestaña se llamó con ese texto (t15a.png). Pulsó la X de la pestaña y salió un diálogo claro: "¿Guardar los cambios de «hola esto es una prueba»?" con "Cerrar sin guardar", "Cancelar" y "Guardar" (t15b.png). Lucía, por ser una prueba, eligió "Cerrar sin guardar" y la pestaña se cerró con "Pestaña cerrada" (t15c.png). Cita: "Está claro y encima me dice que quedan en una recuperación por si me arrepiento." Si hubiera sido un apunte de estudio, habría elegido "Guardar".

**T16.** Exploración libre, unas 14 acciones. Abrió el menú "Estudio" y vio "Subrayar", "Marcas de libro" y "Anotaciones" (t16a.png). Pensó: "Esto es para estudiar, me encanta." Intentó subrayar con el texto sin seleccionar y salió "Seleccioná el texto que querés subrayar" (t16b.png). En modo Leer, "Estudio" no está en la barra (t16c.png). Con clic derecho sobre el texto encontró "Copiar texto", "Copiar Markdown original", "Copiar para Discord o correo" y "Herramientas de estudio" (t16d.png). Dentro estaba el mismo menú de subrayado (t16e.png). Con doble clic no seleccionó la palabra (puede ser límite de la herramienta); con arrastre sí. "Subrayar en amarillo" aplicó un resaltado que se ve verde oliva (t16f.png, t16g.png). Cita: "Pedí amarillo y salió verde. ¿O es amarillo apagado?" Después de subrayar, la pestaña quedó "sin guardar". No probé el icono de la consola (>_) ni el triángulo de la segunda barra: Lucía no sabe qué hacen.

## 5. Problemas

| ID | Sev. | Tarea | Esperado | Lo que pasó | Captura |
|----|------|-------|----------|-------------|---------|
| US-01 | S2 | T8 | Elegir "Título 1" en el menú y escribir "resumen" lo deja como título | Quedó "resumen" sin formato y un `#` suelto en la línea siguiente. Costó unas 12 acciones arreglarlo. No sé si es de la app o de la herramienta que escribe (clic y escritura seguidos); si fuera la app, afecta a cualquiera que escriba rápido. | t8c.png, t8d.png, t8e.png, t8h.png |
| US-02 | S2 | T8, T9 | El botón B pone negrita y puedo seguir escribiendo normal | Sin texto elegido, el botón deja `**\|**` con el cursor en medio. No hay forma visible de salir de la negrita; con Enter se rompe (`**importante` y `**` sueltos). Con la palabra seleccionada, en cambio, funciona bien. | t8j.png, t9a.png, t9c.png |
| US-03 | S3 | T9 | Insertar una tabla me da una grilla para completar | La tabla se inserta como texto con barras y guiones, con valores de relleno ("Columna", "texto"). Lucía no sabe agregar filas ni columnas. Se ve prolija a la derecha. | t9a.png |
| US-04 | S3 | T16 | Subrayar con el menú "Estudio" en cualquier modo | "Subrayar" sin selección avisa "Seleccioná el texto que querés subrayar" pero no dice cómo; "Estudio" falta en modo Leer y hay que descubrir el clic derecho; "amarillo" se ve verde oliva; el subrayado deja la nota "sin guardar". | t16b.png, t16c.png, t16d.png, t16f.png, t16g.png |
| US-05 | S3 | T6, T7 | Resultados completos y un aviso claro | El aviso "la búsqueda cubre solo parte del contenido" asusta sin explicar. Los resultados salen con el texto cortado ("contenido: Termin…") y el panel de resultados no deja ver el contexto. | t6b.png, t7c.png |
| US-06 | S3 | T2 | Tildar la tarea y que quede hecho | La tarea se tilda, pero queda "sin guardar" y hay que acordarse de guardar (aviso con atajos que Lucía no usa). Hay que buscar el disquete. | t2b.png |
| US-07 | S3 | T8 | Un modo donde no vea `#`, `-` ni `**` | En Editar, Lucía ve símbolos crudos (`#`, `- `, `**`). Entiende el resultado solo en Comparar. El pie "Edición" no explica qué es Markdown. | t8c.png, t8i.png, t8j.png |
| US-08 | S4 | T14 | Saber dónde quedó el PDF | El aviso "PDF exportado: resumen.pdf · 1 página" no dice la carpeta ni ofrece abrirlo, y desaparece en pocos segundos. | t14b.png, t14c.png |
| US-09 | S4 | T12 | Los menús siguen en su lugar al achicar | Con 800 px los menús se juntan en una hamburguesa y el que Lucía recordaba (Vista) ya no está donde estaba. | t12a.png |
| US-10 | S4 | T13 | El icono muestra el tema actual | El icono muestra el tema al que se pasa (sol en oscuro, luna en claro). | t13a.png, t13b.png |
| US-11 | S4 | T4 | Ir a "Capítulo 30" rápido | El índice funciona, pero con 49 títulos hay que bajar la lista a mano; no hay caja para filtrar. | t4c.png |
| US-12 | S4 | T11 | Cerrar el panel dividido fácilmente | El icono para cerrar el panel es muy chico y sin texto; el panel izquierdo queda apretado en Comparar. | t11c.png |
| US-13 | S4 | T0 | Entender qué hacen "Leer / Editar / Comparar" | La pantalla de bienvenida lo explica en letra chica; "las dos cosas" no es claro. Los iconos de la segunda barra (triángulo, `>_`) no se entienden sin pasar el puntero. | t0.png |

Fuera de la lista porque dudo del origen: el "napoleonon" de T7 y los caracteres cambiados (`*` por `+`, "impportante") de T9. Parecen fallas de la herramienta de escritura, no de la app.

## 6. SUS detallado

| # | Ítem | Puntaje (1 a 5) | Contribución |
|---|------|-----------------|--------------|
| 1 | Creo que usaría esta app con frecuencia | 4 | 3 |
| 2 | Me pareció innecesariamente compleja | 2 | 3 |
| 3 | Me pareció fácil de usar | 4 | 3 |
| 4 | Necesitaría ayuda de alguien técnico | 2 | 3 |
| 5 | Las funciones están bien integradas | 4 | 3 |
| 6 | Hay demasiada inconsistencia | 2 | 3 |
| 7 | La mayoría aprendería rápido a usarla | 4 | 3 |
| 8 | Me pareció muy engorrosa | 2 | 3 |
| 9 | Me sentí muy segura al usarla | 4 | 3 |
| 10 | Tuve que aprender muchas cosas antes de empezar | 2 | 3 |

Suma de contribuciones: 30. Puntaje SUS: 30 x 2,5 = **75**.

Lucía no sabe nada de Markdown y ahí flaquea: en los ítems 4 y 10 puso 2, no 1, por los símbolos crudos y la tabla de barras.

## 7. Gustos y frustraciones

**Lo que más le gustó**
- El botón "Copiar" con el cambio a "Copiado" (T3).
- El índice del documento para llegar al capítulo 30 (T4).
- Comparar: ver la "receta" y el "plato" lado a lado (T10).
- Los avisos con atajo y los cartelitos al pasar el puntero (T12, T13).
- El diálogo de cerrar sin guardar, con su "recuperación" (T15).
- El menú "Estudio" para subrayar y marcar (T16).

**Las 5 mayores frustraciones**
1. La negrita sin selección y el cursor atrapado entre asteriscos (US-02).
2. El título que no salió como título y el `#` suelto (US-01).
3. La tabla como texto con barras (US-03).
4. El subrayado: sin selección, ausente en Leer, color que no coincide (US-04).
5. El aviso "la búsqueda cubre solo parte del contenido" y los resultados cortados (US-05).

**Lo que no encontró**
- Dónde quedó el PDF exportado (US-08).
- Cómo agregar filas a la tabla (US-03).
- Cómo salir de la negrita sin romperla (US-02).
- Qué hacen el triángulo de la segunda barra y la consola `>_`; no los probó.
- El menú "Estudio" en modo Leer, hasta probar el clic derecho (US-04).

**¿La recomendaría?** Sí, con reservas, a una compañera que estudie y lea mucho: leer, copiar, buscar, ir a un capítulo y dividir la pantalla le resultaron sencillos. Para escribir apuntes con formato, le diría que primero pruebe la barra de iconos con texto seleccionado.

## 8. Cita final

"Para leer mis apuntes es buenísima y me ahorra mucho. Para escribir me siento haciendo un trabajo de programadora. Si me dejaran ver la tabla como en Word, y si la negrita no me dejara atrapada, le pondría un 9."
