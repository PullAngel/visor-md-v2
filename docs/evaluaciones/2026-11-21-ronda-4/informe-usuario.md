# Informe de usabilidad: Visor MD (ronda 4), Lucía

Participante: Lucía, 21 años, 2.º de Biología. Sesión con la copia `usuario`, ventana de 1280x720. Capturas en `C:\Dev\archivo\eval-ronda-4\usuario\capturas\`. Todo lo escrito sale de lo observado en pantalla.

## 1. Resumen

- **Tasa de éxito:** 14 de 15 tareas evaluables con éxito, 1 con dificultad (tarea 8), 0 fracasos. La tarea 16 era exploración libre y no se puntúa. Ninguna tarea llegó al límite de 15 acciones sin avance.
- **SUS:** 75 / 100.
- **Las 5 peores fricciones:**
  1. Al editar, Lucía ve símbolos crudos (`#`, `**importante**`, `| --- | --- |`) y la tabla se inserta como texto con barras (US-02, US-04).
  2. Al aplicar «Título 1» en una nota vacía y escribir, el `#` quedó en la línea equivocada y hubo que repararlo a mano (US-01).
  3. La lupa de arriba busca solo en el documento abierto. La búsqueda en todos los apuntes está dentro del menú «Carpeta» (US-03).
  4. Los cambios no se guardan solos y el aviso es un punto pequeño en la pestaña. `inicio.md` quedó con el punto toda la sesión (US-06).
  5. Con la ventana chica, la vista dividida se aprieta: texto de tabla superpuesto y títulos cortados (US-08).

## 2. Primera impresión (test de 5 segundos)

Captura: `00-inicio.png`.

- **¿Qué es esta app?** «Un lector y editor de apuntes, algo parecido a Word o a las notas del celu, pero oscuro y verde. Dice "Visor MD", no sé qué es MD.»
- **¿Qué se puede hacer?** «Abrir una carpeta de apuntes, abrir un archivo o hacer un apunte nuevo. Arriba hay Leer, Editar y Comparar, y una barra con negrita, cursiva y cosas así, como en Word.»
- **¿Qué me llama la atención?** «Los tres botones grandes de la bienvenida me dicen por dónde empezar. Los textos son cortos y en español de acá ("buscás", "escribí").»
- **¿Qué no entiendo?** «"MD" y "Comparar". Los iconitos de arriba no dicen qué hacen. Los "Ctrl+Shift+O" me dan igual. El ">_" parece una consola, ¿es para programadores?»

## 3. Tabla de tareas

| # | Tarea | Resultado | Acciones | Confianza | Capturas |
|---|---|---|---|---|---|
| 1 | Abrir `inicio.md` | Éxito | 1 | 5 | t01 |
| 2 | Marcar «Leer capítulo 3» | Éxito | 2 | 5 | t02a, t02b |
| 3 | Copiar comando de «Comandos útiles» | Éxito | 1 | 4 | t03 |
| 4 | Manual largo y Capítulo 30 | Éxito | 4 | 4 | t04a-t04d |
| 5 | Volver a inicio, ¿en el mismo lugar? | Éxito | 1 | 5 | t05 |
| 6 | Trabajar con toda la carpeta y abrir célula | Éxito | 3 | 5 | t06a-t06c |
| 7 | Buscar «Napoleón» en todos los apuntes | Éxito (con 1 desvío) | 6 | 4 | t07a-t07d |
| 8 | Apunte «resumen»: lista de 3, negrita, guardar | **Con dificultad** | ~20 (7 de reparación) | 3 | t08a-t08h |
| 9 | Tabla chiquita y emoji | Éxito | 7 | 4 | t09a-t09d |
| 10 | Texto por dentro y resultado, lado a lado | Éxito | 2 | 5 | t10 |
| 11 | Dos apuntes uno al lado del otro | Éxito | 3 | 4 | t11a-t11c |
| 12 | Achicar y dejar siempre encima | Éxito | 2 | 5 | t12a, t12b |
| 13 | Modo claro y volver a oscuro | Éxito | 2 | 5 | t13a, t13b |
| 14 | Exportar resumen a PDF | Éxito | 2 (+3 de repetición) | 4 | t14a-t14c |
| 15 | Escribir y cerrar sin guardar | Éxito | 4 | 5 | t15a-t15c |
| 16 | Exploración libre | n/a | ~11 | n/a | t16a-t16g |

## 4. Detalle por tarea

**T1.** Con el botón «Abrir archivo» de la bienvenida (el elegido en el selector fue `inicio.md`), la nota abre en modo Leer, con títulos, tabla y bloques de código. Un solo paso. «Ah, se ve lindo, parece una página.»

**T2.** Hay que bajar con la rueda (1 acción) y tocar el cuadradito de «Leer capítulo 3» (1 acción). Se tilda y aparece el aviso «Tarea actualizada · sin guardar: Ctrl+S la guarda, Ctrl+Z la deshace». Se entiende, pero «sin guardar» sugiere que si no hace nada, se pierde. La pestaña quedó con un punto (ver US-06).

**T3.** Cada bloque de código tiene un botón «Copiar» a la derecha. Al tocarlo cambia a «✓ Copiado». El portapapeles, leído justo después, tenía `ping 192.168.0.1` y `git status` (las dos líneas del bloque). Lucía esperaba un solo comando; el botón copia el bloque entero (US-05).

**T4.** Camino: el icono de carpeta de arriba (70,21) abrió `manual-largo.md` y reconoció la ruta preparada. El icono de la barra de abajo (67,92) abrió un panel «Índice · 49 títulos». La lista es larga: hubo que bajar la rueda en el panel y tocar «Capítulo 30». Salta ahí y el panel se cierra solo. Con rueda directa habría sido más lento. Falta una forma de escribir «30» para filtrar (US-12).

**T5.** Tocar la pestaña `inicio.md` la dejó en la misma posición (la tabla y «Tareas» donde estaban). Sí, quedó donde estaba.

**T6.** Menú «Carpeta» > «Abrir carpeta» (la carpeta `apuntes` se eligió con el selector preparado). Se abre un panel «Archivos» con árbol (`biologia`, `historia.md`, `inicio.md`, `manual-largo.md`) y un aviso «Carpeta de apuntes indexada: 5 notas, 0 omitidas; la búsqueda cubre solo parte del contenido» (US-07). Se tocó `celula.md` y abrió. Sin problema.

**T7.** Camino: la lupa de arriba (150,21) abrió «Buscar en el documento», que no sirve para todos los apuntes (callejón, 1 acción + Esc). Luego «Carpeta» > «Buscar en carpeta», se escribió `napoleon` (sin tilde por límite de la herramienta) y apareció 1 resultado: «Revolución francesa · contenido: Termin…». Al tocarlo abrió `historia.md` con «Napoleón» resaltado y «1 de 1». Encontró el término igual sin tilde. Lucía duda: «¿Es solo en un archivo? ¿Dónde más?» (La carpeta tiene 5 notas; el panel dice «1 resultado».) El resultado se muestra con el título de la nota, no con el nombre de archivo (US-13).

**T8.** Camino: «+» crea una nota («Documento nuevo · Ctrl+Shift+S para elegir destino») con la pista «Escribí aquí. El formato está en la barra de arriba». El menú «Párrafo» muestra «Título 1 (H1)», «Lista con viñetas», etc. Se eligió Título 1 y se escribió `resumen` + Enter. Resultado: la línea 1 quedó `resumen` sin marca y apareció un `#` suelto en la línea 2 (US-01). Reparación: clic en la línea 1, Título 1 otra vez, borrar el `#` de la línea 2 con 3 retrocesos (la herramienta no entiende «backspace», sino «back»). Después: Enter, «Lista con viñetas» y tres elementos (la lista continuó sola), Enter x2 para salir, botón **B** y la palabra. En el editor se ve `**importante**`, sin negrita visible (US-02). Disco de guardar (110,21): como la nota no tenía destino, se usó el nombre preparado `resumen.md` y quedó «Guardado». «Con los asteriscos me asusté: pensé que lo había escrito mal.»

**T9.** Botón de tabla (507,92): inserta `| Columna | Valor |` con una fila de ejemplo. En el editor son barras y guiones, no una grilla (US-04). «Insertar» > «Insertar símbolo o emoji» abre un selector excelente con iconitos y nombres en español («emoji idea»). Se eligió el foco de idea; entró dentro de la celda. En «Comparar» se ve como tabla de verdad.

**T10.** Botón «Comparar»: texto a la izquierda, resultado a la derecha, y un aviso «Vista dividida · fuente editable y lectura sincronizada». Claro y rápido.

**T11.** Menú «Vista» > «Dividir a la derecha con otra pestaña» (bajo el título «Mosaico»). Abre una lista «¿qué mostrar en el panel nuevo?» con las pestañas abiertas. Se tocó `celula.md`. Quedan `resumen.md` y `celula.md` lado a lado. Hay botones Leer/Editar/Comparar en cada panel y otros globales arriba (US-09).

**T12.** El icono de chincheta (1154,21) mostró «Ventana fijada encima de las demás · clic en la chincheta para soltarla». Se achicó a 640x480 con la herramienta: los menús pasan a un icono de «hamburguesa», pero con la vista dividida el texto se amontona (US-08).

**T13.** El sol arriba a la derecha (478,21 con la ventana chica) pasa a «Tema de día». El fondo es crema, no blanco. Se vuelve a oscuro con el mismo botón y con el aviso «Tema de noche». Intuitivo.

**T14.** «Archivo» > «Exportar a PDF». Se generó `resumen.pdf` (61 KB, visible en la carpeta). En mi captura, tomada 2 segundos después, no se veía confirmación. Al repetir, con la respuesta ya consumida, apareció «Exportación a PDF cancelada», así que el aviso existe pero es breve (US-11).

**T15.** Nueva nota, se escribió «idea para el parcial» y se tocó la X de la pestaña. Diálogo: «¿Guardar los cambios de "idea para el parcial"? Si cerrás sin guardar, los cambios quedan en una recuperación que podés abrir después», con botones «Cerrar sin guardar», «Cancelar», «Guardar» (el último, resaltado). Lucía elige «Cerrar sin guardar», porque era una nota de prueba; si fuera algo importante elegiría Guardar. Pestaña cerrada y vuelve a la nota anterior.

**T16.** Acciones: menú «Estudio» (subrayar, marcas de libro, «anotaciones sin lugar»); arrastrar una frase + el lápiz amarillo, que subraya en amarillo como en Word («Subrayado en amarillo»); icono «>_» que abre una lista «Buscar una acción» (probé «imprimir», 0 resultados); clic en el enlace roto «nota-que-no-existe», con un diálogo «Esta nota no existe... Crear nota / Ahora no»; clic derecho sobre el texto («Copiar texto», «Copiar Markdown original», «Copiar para Discord o correo», «Herramientas de estudio»); soltar la chincheta; cerrar el panel de la izquierda («Panel cerrado · la pestaña continúa abierta»). No pulsé el enlace a Wikipedia: Lucía esperaba que se abriera el navegador.

## 5. Problemas

| ID | Sev. | Tarea | Esperado | Pasó | Captura |
|---|---|---|---|---|---|
| US-01 | S3 | T8 | Elegir «Título 1» y escribir daba un título en la línea 1 | La línea 1 quedó sin `#` y un `#` suelto quedó en la línea 2; hubo que reaplicar el estilo y borrar a mano. Puede depender del orden de eventos de la herramienta, pero Lucía lo vio así | t08c, t08d |
| US-02 | S3 | T8, T9 | Que la negrita se vea en negrita | En «Editar» se ven `**importante**` y `#`, sin pista de que son marcas. El botón **B** no cambia nada visible | t08g |
| US-03 | S3 | T7 | Que la lupa busque en todos mis apuntes | La lupa busca solo en el documento. La búsqueda de carpeta está dentro de «Carpeta» (no es evidente) | t07a, t07b |
| US-04 | S3 | T9 | Una tabla con celdas | Se inserta como texto con barras y guiones. Se ve como tabla solo en «Comparar» | t09a, t10 |
| US-05 | S4 | T3 | Copiar «el comando» | Copia las dos líneas del bloque | t03 |
| US-06 | S3 | T2, T15 | Que los cambios queden guardados | No hay guardado automático. El aviso es un punto pequeño en la pestaña; `inicio.md` quedó «sin guardar» toda la sesión y Lucía no se dio cuenta | t02b, t16g |
| US-07 | S4 | T6 | Un mensaje que se entienda | «La búsqueda cubre solo parte del contenido» no se entiende: ¿qué parte? | t06b |
| US-08 | S3 | T12 | Poder achicar sin romper la pantalla | A 640 px, con la vista dividida: «Columna» y «Valor» se superponen, el título «resumen» se corta, los botones de panel desaparecen | t12b |
| US-09 | S4 | T11 | Dividir la pantalla con un nombre claro | El menú dice «Mosaico»; además, hay dos juegos de Leer/Editar/Comparar (global y por panel) | t11a, t11c |
| US-10 | S4 | T16 | Un icono con sentido | «>_» parece una consola y abre una lista de comandos que no sabe buscar «imprimir». El menú «Estudio» tiene términos como «anotaciones sin lugar» | t16c, t16d, t16a |
| US-11 | S4 | T14 | Una confirmación clara de que se exportó | No vi mensaje de éxito en mi captura. La ruta de destino es la que yo elegí, pero el app no la muestra | t14b |
| US-12 | S4 | T4 | Llegar al «Capítulo 30» escribiendo | El índice de 49 títulos no se puede filtrar, solo recorrer. El panel también resalta lo que queda bajo el puntero, lo que confunde | t04b, t04c |
| US-13 | S4 | T7 | Ver el nombre del archivo y la frase | El resultado muestra el título («Revolución francesa») y el texto cortado con «…» | t07c |

## 6. SUS detallado

| # | Ítem | Puntaje (1-5) |
|---|---|---|
| 1 | Creo que me gustaría usarla con frecuencia | 4 |
| 2 | Me pareció innecesariamente compleja | 2 |
| 3 | Me pareció fácil de usar | 4 |
| 4 | Necesitaría ayuda de una persona técnica | 2 |
| 5 | Las funciones están bien integradas | 4 |
| 6 | Hay demasiada inconsistencia | 2 |
| 7 | La mayoría aprendería a usarla rápido | 4 |
| 8 | Me pareció muy incómoda | 2 |
| 9 | Me sentí muy segura usándola | 4 |
| 10 | Necesité aprender muchas cosas antes de empezar | 2 |

Cálculo: ítems impares (puntaje - 1) = 3+3+3+3+3 = 15; ítems pares (5 - puntaje) = 3+3+3+3+3 = 15; (15 + 15) x 2,5 = **75**.

## 7. Gustos y frustraciones

**Lo que más le gustó**
- Los botones de la bienvenida, y que las notas se vean lindas al abrir.
- El selector de símbolos y emojis, con nombres en español.
- Los avisos breves en cada acción («Ventana fijada encima...», «Tema de día...»).
- Subrayar como en Word y el diálogo para cerrar sin guardar, que explica la recuperación.
- «Comparar», para ver el texto y el resultado juntos.
- Que el índice de un documento largo lleve directo al capítulo.

**Las 5 mayores frustraciones**
1. Los símbolos crudos al editar (`#`, `**`, barras de tabla) (US-02, US-04).
2. El título que quedó mal en la nota vacía (US-01).
3. Buscar en todos los apuntes no está en la lupa (US-03).
4. El «sin guardar» silencioso (US-06).
5. La vista dividida apretada con la ventana chica (US-08).

**Lo que no encontró**
- Un botón de imprimir (buscó «imprimir» en la lista de acciones y no hay).
- Una forma de filtrar el índice escribiendo.
- Un lugar claro donde ver la ruta de lo exportado.
- La negrita visible mientras se escribe.

**¿La recomendaría?** Sí, con reservas: «Se la recomendaría a una compañera que ya sepa un poco de Markdown. Para leer y buscar es muy buena. Para escribir, me gustaría que se viera como Word.»

## 8. Cita final

«Leer mis apuntes es un placer, y encontrar todo es rápido. Pero cuando escribo, me muestra los asteriscos y las rayitas como si tuviera que saber programar.» - Lucía
