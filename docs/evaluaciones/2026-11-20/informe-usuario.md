# Informe de usabilidad: Visor MD (Lucía, ronda 2)

Capturas en `C:\Dev\archivo\eval-ronda-2\usuario\capturas\` (por ejemplo `t4g.png`). Solo se usó `ui.ps1` sobre la copia `usuario`. Se creó `apuntes\resumen.md` y `apuntes\resumen.pdf`. Los scripts auxiliares `u.sh` y `a.sh` quedaron en `usuario\`.

## 1. Resumen

- **Tasa de éxito:** 15 tareas numeradas (la 16 es exploración libre). 12 con éxito, 3 con dificultad (4, 7 y 8), 0 fracasos. Se completaron todas; el 80 % sin tropiezos.
- **SUS: 67,5 / 100**, apenas por debajo del promedio habitual (68).
- **Las 5 peores fricciones:**
  1. US-05: armar el título y la lista en el editor. Aparecen símbolos como `#`, `- ` y `**`, hubo un `#` suelto y un botón con aspecto de lista que insertó una cita.
  2. US-01: tildar una tarea deja el apunte «sin guardar» y nada avisa.
  3. US-03: los iconos de la segunda barra no tienen nombre. Para llegar al «Capítulo 30» hay que adivinar cuál es el índice.
  4. US-04: la búsqueda en carpeta muestra el resultado cortado y, al abrirlo, no lleva a la frase.
  5. US-07: las pestañas que quedan en un panel dividido desaparecen de la barra de pestañas.

## 2. Primera impresión (test de 5 segundos, `00-inicio.png`)

- **¿Qué es?** «Un programa para leer y escribir apuntes, tipo bloc de notas, que se llama Visor MD.»
- **¿Qué se puede hacer?** Abrir una carpeta de apuntes, abrir un archivo o crear un apunte nuevo (tres botones grandes). Arriba hay tres modos: Leer, Editar y Comparar.
- **¿Qué llama la atención?** El verde oscuro, tranquilo y ordenado. Los tres botones grandes. La barra de iconos de formato (B, I, etc.) que parece de Word.
- **¿Qué no entiendo?**
  - «Markdown» (no sé qué es).
  - Los atajos «Ctrl+Shift+O».
  - Qué es «Comparar».
  - La barra con «Estudio» e «Insertar».
  - La frase «Abrir tus archivos .md con Visor MD · Configurar», que parece un enlace pero no sé qué hace.

## 3. Tabla de tareas

| # | Tarea | Resultado | Acciones | Confianza |
|---|---|---|---|---|
| 1 | Abrir inicio.md | Éxito | 1 | 5 |
| 2 | Marcar «Leer capítulo 3» | Éxito (queda sin guardar) | 4 | 4 |
| 3 | Copiar el comando | Éxito (sin aviso visible) | 1 | 3 |
| 4 | Manual largo, Capítulo 30 | Con dificultad | 5 (+2 pasadas de puntero) | 4 |
| 5 | Volver a inicio | Éxito | 1 | 5 |
| 6 | Carpeta completa y abrir célula | Éxito | 3 | 5 |
| 7 | Buscar «Napoleón» en todo | Con dificultad | 4 | 3 |
| 8 | Crear `resumen.md` | Con dificultad | unas 35 (con correcciones) | 3 |
| 9 | Tabla y emoji | Éxito | 4 | 4 |
| 10 | Texto por dentro y resultado, lado a lado | Éxito | 1 | 5 |
| 11 | Dos apuntes lado a lado | Éxito | 3 | 4 |
| 12 | Achicar y dejar encima | Éxito | 2 | 4 |
| 13 | Claro y oscuro | Éxito | 2 | 5 |
| 14 | Exportar a PDF | Éxito | 2 | 5 |
| 15 | Cerrar sin guardar | Éxito | 6 | 5 |
| 16 | Exploración libre | 12 acciones, sin bloqueos | 12 | n/a |

## 4. Detalle por tarea

**1. Abrir inicio.** Pulso «Abrir archivo» en la bienvenida y se abre en modo Leer, con título, tabla, citas y enlaces. «Ah, se ve lindo, como una página.» Captura `t1a.png`.

**2. Marcar la tarea.** El texto no se ve de entrada y hay que bajar con la rueda (3 acciones; `t2a.png`). Toco el cuadrito de «Leer capítulo 3» y se tilda (`t2b.png`, `t3a.png`). Pero en la pestaña aparece un puntito (`inicio.md ●`) y abajo dice «sin guardar». «¿Se guardó o no? ¿Qué es ese puntito?» No hay mensaje. El archivo recién quedó guardado cuando lo guardé en la exploración libre, y eso lo hice porque lo vi, no porque la app me guiara.

**3. Copiar el comando.** Cada bloque de código tiene un botón «Copiar». Lo pulso en el primero (el de «ping… git status») y el portapapeles contiene justo esas dos líneas. En la captura de medio segundo después el botón sigue diciendo «Copiar» y no se ve ningún aviso. «Supongo que se copió… no sé.» Captura `t3a.png`.

**4. Capítulo 30.** Abro el manual con el icono de carpeta de arriba (`t4a.png`). El documento es largo y no sé dónde está el capítulo 30. En la segunda barra hay cinco iconos sin nombre. Paso el puntero por el último y dice «Notas que enlazan aquí» (callejón, `t4b.png`). Pulso las tres rayitas de la izquierda y se abre el panel «Índice · 49 títulos» (`t4d.png`). Desplazo la lista dos veces con la rueda y pulso «Capítulo 30» (`t4f.png`, `t4g.png`). El panel se cierra solo y el título queda arriba. «El índice me salvó, pero lo encontré de casualidad.»

**5. Volver a inicio.** Pulso la pestaña `inicio.md`. Estoy casi donde estaba: unos 2 píxeles de diferencia, en «Comandos útiles». Captura `t5a.png`. «Sí, perfecto.»

**6. Carpeta completa.** Pruebo el menú «Carpeta» (`t6a.png`). «Abrir carpeta» y elijo la carpeta de apuntes. Se abre un panel con las 5 notas, dentro de «biologia» está `celula.md`. Salió un aviso: «Carpeta de apuntes indexada: 5 notas, 0 omitidas; búsqueda limitada por presupuesto de memoria» (`t6b.png`). «Esto último no lo entiendo, ¿se rompió algo?» Pulso `celula.md` y se abre (`t6c.png`). La barra de estado dice «carpeta activa».

**7. Napoleón.** Abro «Buscar» (lupa con hoja), arriba del panel (`t7a.png`). Mientras no escribo, el panel dice «Sin resultados», y eso me confunde. Escribo «napole», porque no sé poner la «ó» desde mi teclado. Sale 1 resultado: «Revolución francesa · contenido: Termin…» (`t7b.png`). No dice dónde ni muestra la palabra. Lo pulso y se abre `historia.md` desde arriba (`t7c.png`). Tengo que bajar hasta «Termina con el golpe de Napoleón» en el bloque del Directorio (`t7d.png`). «Está, pero tuve que cazarlo yo.»

**8. Crear `resumen.md`.** Pulso «+» arriba y aparece una nota vacía en modo Editar, con «Escribí aquí. El formato está en la barra de arriba» (`t8a.png`). Pulso «Párrafo» y elijo «Título 1 (H1)» antes de escribir.

- La primera vez que lo hice, la pantalla quedó con «resumen» en la línea 1, sin `#`, y un `#` en una línea 2 (`t8c.png`). No sé por qué pasó. «No quise eso.» Tuve que borrar con la tecla de retroceso y se mezclaron las letras («resumesumen#», `t8e.png`).
- A la segunda, escribí «resumen» y apliqué «Título 1» con el cursor en esa línea. Quedó `# resumen` (`t8g.png`). Ahora veo `#` en pantalla y no sé qué es.
- Para la lista pulsé el icono que parecía de lista (el de rayitas, tercero después de «Párrafo») y escribió un `>` (cita) (`t8i.png`). Lo borré. Fui otra vez a «Párrafo» y elegí «Lista con viñetas»: sale `- ` (`t8j.png`).
- Escribí «uno», Enter, «dos», Enter, «tres»: continúa la lista sola, muy cómodo (`t8k.png`).
- Para la negrita, pulsé B sin seleccionar nada. Escribió `****` con el cursor en medio. Escribí «importante» y quedó `**importante**` (`t8m.png`). «¿Por qué tiene asteriscos? ¿Se va a ver en negrita?»
- Guardar: pulso el diskette, preparado el nombre `resumen.md`. Dice «Guardado» y la pestaña pasa a `resumen.md` (`t8n.png`). Bien que el nombre de la pestaña se arme solo con el título.
- Resultado correcto, pero con 35 acciones aproximadas y mucha duda.

**9. Tabla y emoji.** Pulso el icono de tabla (cuadrícula) y aparece una tabla con «Columna | Valor / texto | valor» (`t9a.png`). «Insertar» → «Insertar símbolo o emoji» abre una lista con filtro y descripciones («emoji idea», «emoji hecho»…; `t9c.png`). Elijo «emoji idea» y se inserta con aviso «Emoji idea insertado · Ctrl+Z para deshacer». Quedó dentro de la última celda de la tabla, justo donde estaba el cursor (`t9d.png`). Funciona, pero hay que mirar dónde está el cursor.

**10. Por dentro y lado a lado.** El botón «Comparar», arriba, muestra a la izquierda el texto con símbolos y a la derecha cómo queda (`t10a.png`). «¡Ah, esto es lo que hace Comparar!» Se actualiza solo. Es lo más claro de la app.

**11. Dos apuntes juntos.** Menú «Vista» → «Dividir a la derecha con otra pestaña» → elijo `historia.md` (`t11a.png` a `t11c.png`). Quedan dos paneles. Pero la pestaña `resumen.md` y la de `historia.md` ya no están en la barra de arriba. «¿Dónde están mis pestañas?» (US-07.)

**12. Achicar y dejar encima.** Achico la ventana a 700 px de ancho (arrastrar la esquina). Los menús de texto se juntan en un botón de tres rayas y la lista de apuntes queda angosta, con la tabla de la derecha pisada (`t12a.png`). Para dejarla encima: en el menú «Vista» vi «Ventana siempre encima», pero probé primero el icono de chincheta, de arriba a la derecha. Salió «Ventana fijada encima de las demás · clic en la chincheta para soltarla» (`t12b.png`). Claro.

**13. Claro y oscuro.** El icono de sol, arriba a la derecha, pasa a claro con aviso «Tema de día · Ctrl+Shift+D para volver». Otra vez y vuelve a noche (`t13a.png`, `t13b.png`). Dos acciones. El icono muestra un sol cuando estoy en oscuro y una luna cuando estoy en claro: muestra el destino, no el estado actual.

**14. PDF.** Menú «Archivo» → «Exportar a PDF». Aviso «PDF exportado: resumen.pdf · 1 página» (`t14b.png`). Verifiqué que el archivo existe (unos 50 KB), pero no lo abrí.

**15. Cerrar sin guardar.** «+», escribo «borrador» y uso «Archivo» → «Cerrar pestaña». Sale «¿Guardar los cambios de «borrador»?» con «Cerrar sin guardar», «Cancelar» y «Guardar» (`t15b.png`). El texto explica que los cambios quedan en una recuperación que se puede abrir después. Elijo «Cerrar sin guardar» porque era un borrador. Dice «Pestaña cerrada» (`t15c.png`). Muy bien explicado. Extraño: el documento nuevo no aparecía en la barra de pestañas, solo en el título del panel.

**16. Exploración libre (12 acciones).**
- Guardar `inicio.md` con el diskette: el puntito desaparece.
- Icono `>_`: abre una paleta de acciones con filtro (77 acciones) y atajos (`t16b.png`). Útil, pero asusta un poco.
- El icono con triángulo, segunda barra: plegó la sección «Comandos útiles» con aviso «Sección plegada; la fuente permanece intacta» (`t16c.png`). No lo esperaba por el dibujo del icono.
- Menú «Editar»: deshacer, cortar, copiar y pegar, y «Copiar para Discord o correo» (`t16d.png`). Me gustó eso.
- Menú «Estudio», en modo Editar: subrayar en colores, marcas de libro y anotaciones (`t16e.png`). Muchísimas opciones; no sé para qué sirve cada una.

## 5. Problemas

| ID | Sev. | Tarea | Esperaba / pasó | Captura |
|---|---|---|---|---|
| US-01 | S2 | 2 | Esperaba que tildar la tarea se guardara o que dijera que hay que guardar. Pasó: puntito en la pestaña y «sin guardar», sin aviso. Si cierro, se pierde la marca. | `t3a.png` |
| US-02 | S3 | 3 | Esperaba un «Copiado». Pasó: el botón sigue diciendo «Copiar» (no se vio aviso medio segundo después). El portapapeles sí quedó bien. | `t3a.png` |
| US-03 | S3 | 4 | Esperaba un botón con nombre para el índice. Pasó: cinco iconos sin texto; el tooltip del último dice «Notas que enlazan aquí» y el índice es el de las tres rayas. Las pestañas del panel («Archivos / Índice / Buscar / Notas que enlazan aquí») se pisan con el texto del documento. | `t4b.png`, `t4d.png` |
| US-04 | S3 | 7 | Esperaba ver la frase con «Napoleón» resaltada. Pasó: resultado cortado («Termin…»), no dice dónde y el documento se abre desde arriba; sin escribir dice «Sin resultados». | `t7a.png` a `t7d.png` |
| US-05 | S2 | 8 | Esperaba un título y una lista sin ver símbolos. Pasó: `#`, `- ` y `**` visibles; un `#` suelto la primera vez; el icono parecido a lista inserta una cita (`>`). | `t8c.png`, `t8i.png` |
| US-06 | S3 | 6 | Esperaba un mensaje simple. Pasó: «búsqueda limitada por presupuesto de memoria» (palabras técnicas). | `t6b.png` |
| US-07 | S3 | 11, 15 | Esperaba ver todas mis pestañas. Pasó: las pestañas que quedan en un panel dividido desaparecen de la barra (resumen, historia, borrador). | `t11c.png`, `t15a.png` |
| US-08 | S3 | 12 | Esperaba que la ventana chica siga legible. Pasó: con 700 px y dos paneles en Comparar, las columnas de la tabla se pisan. | `t12a.png` |
| US-09 | S4 | 9 | El emoji va dentro de la celda donde está el cursor, sin avisar. | `t9d.png` |
| US-10 | S4 | 13 | El icono del tema muestra el destino (sol en modo oscuro) en vez del estado actual. | `t13a.png`, `t13b.png` |
| US-11 | S4 | 16 | El icono de triángulo pliega secciones; el dibujo sugiere «reproducir». | `t16c.png` |

## 6. SUS detallado (Lucía; 1 = totalmente en desacuerdo, 5 = totalmente de acuerdo)

| Ítem | Afirmación | Respuesta | Puntos |
|---|---|---|---|
| 1 | Creo que usaría esta app con frecuencia | 3 | 2 |
| 2 | La encontré innecesariamente compleja | 3 | 2 |
| 3 | Fue fácil de usar | 4 | 3 |
| 4 | Necesitaría ayuda de alguien técnico | 2 | 3 |
| 5 | Las funciones están bien integradas | 4 | 3 |
| 6 | Hay demasiada inconsistencia | 3 | 2 |
| 7 | La mayoría aprendería rápido | 4 | 3 |
| 8 | Me resultó muy incómoda | 2 | 3 |
| 9 | Me sentí segura usándola | 4 | 3 |
| 10 | Tuve que aprender muchas cosas antes de empezar | 3 | 2 |

Los puntos de los ítems impares son la respuesta menos 1. Los de los pares son 5 menos la respuesta. Suma 27 puntos por 2,5: **67,5 / 100**.

## 7. Gustos y frustraciones

**Lo que más me gustó**
- El modo «Comparar»: ver el texto por dentro y cómo queda, a la vez.
- Los avisos que explican qué pasó y cómo volver («Tema de día · Ctrl+Shift+D para volver»).
- El diálogo de cerrar sin guardar, que tranquiliza.
- El índice del documento y la lista que continúa sola al escribir.
- Que la chincheta y el tema se cambien con un clic.

**Las 5 mayores frustraciones**
1. Los símbolos `#`, `- `, `**` en el editor, y el icono equivocado que insertó una cita.
2. Que marcar una tarea deje el apunte sin guardar y sin aviso.
3. Los iconos sin nombre de la segunda barra.
4. El resultado de búsqueda cortado, que no lleva a la frase.
5. Que las pestañas desaparezcan al dividir el panel.

**Lo que no encontré**
- Un botón con nombre para el índice.
- Cómo abrir el PDF desde la app.
- Una forma de escribir negrita sin ver los asteriscos.
- Un «copiado» después de Copiar.
- Dónde se configura que la app se abra con mis `.md` (el enlace «Configurar» no lo toqué).

**¿La recomendaría?** Sí, con una advertencia: «Es linda y hace mucho. Pero al principio hay que pelearse un poco con los símbolos, y yo se la pasaría a una compañera que no le tema a probar cosas.»

## 8. Cita final

«Cuando lo entendés, es cómodo: Comparar me encantó. Pero me hubiera gustado que me avisara cuando algo no está guardado y que me dijera qué es cada iconito.»

## Límites de esta prueba

Solo conté lo observado en pantalla. Los enlaces web no se pulsaron (no hubo ocasión de que Lucía los usara). No leí código, documentación ni `qa`. No abrí el PDF exportado.
