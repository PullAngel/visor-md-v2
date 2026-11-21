# Informe de usabilidad - Visor MD (Lucía, ronda 6)

Participante simulada: Lucía, 21 años, 2.º año de Biología. Ventana de 1280x720, copia `usuario`. Capturas en `C:\Dev\archivo\eval-ronda-6\usuario\capturas\`. Los límites de la herramienta (sin mayúsculas ni tildes, alguna tecla perdida al tipear, sin Ctrl/Shift) no se cuentan como problemas de la app. Un ejemplo: en el resumen quedó «importrtante» por una tecla perdida.

## 1. Resumen

- **Tasa de éxito:** 15 de 15 tareas evaluables completadas (la T16 es exploración libre). Éxito limpio: 12. Con dificultad: 3 (T8, T9, T14). Fracasos: 0. Contando la T7 con un desvío, pero lograda, como éxito.
- **SUS:** 70/100.
- **Las 5 peores fricciones:**
  1. Al dar Enter después de poner negrita se rompe el formato: queda un `**` suelto y una viñeta vacía (US-01).
  2. Al editar se ven símbolos de Markdown (`#`, `-`, `**`, `|---|`) que Lucía no entiende (US-02).
  3. Exportar a PDF no mostró confirmación visible (US-03).
  4. Con la ventana chica desaparecen los nombres de los menús (US-04).
  5. La búsqueda: la lupa solo busca en el documento y los resultados vienen con el texto cortado (US-05).

## 2. Primera impresión (test de 5 segundos)

Captura: `capturas\00-inicio.png`.
- **¿Qué es?** «Un programa para leer y escribir apuntes, parecido a un Word simple. Dice "Bienvenida a Visor MD".»
- **¿Qué se puede hacer?** Abrir una carpeta de apuntes, abrir un archivo, crear un apunte nuevo. Hay botones Leer / Editar / Comparar.
- **Lo que llama la atención:** las tres tarjetas grandes de inicio, el tema oscuro verde y las barras llenas de iconos.
- **Lo que no entiende:** «Markdown», «Comparar», «Estudio», la fila de iconos sin nombre, y la frase «Editar abre su Markdown». Tampoco qué es `Ctrl+Shift+O`.

## 3. Tabla de tareas

| # | Tarea | Resultado | Acciones | Confianza |
|---|-------|-----------|----------|-----------|
| 1 | Abrir inicio.md | Éxito | 2 | 5 |
| 2 | Marcar «Leer capítulo 3» | Éxito | 2 | 5 |
| 3 | Copiar comando | Éxito | 1 | 5 |
| 4 | Manual largo y Capítulo 30 | Éxito | 5 | 4 |
| 5 | Volver a inicio | Éxito | 1 | 5 |
| 6 | Carpeta completa y abrir célula | Éxito | 3 | 4 |
| 7 | Buscar «Napoleón» en todo | Éxito (con un desvío) | 6 | 4 |
| 8 | Crear «resumen» con lista y negrita | Con dificultad | cerca de 14 | 2 |
| 9 | Tabla y emoji | Con dificultad | cerca de 8 | 3 |
| 10 | Ver por dentro y lado a lado | Éxito | 2 | 5 |
| 11 | Dos apuntes lado a lado | Éxito | 4 | 4 |
| 12 | Achicar y dejar encima | Éxito | 3 | 4 |
| 13 | Claro y volver a oscuro | Éxito | 2 | 5 |
| 14 | Exportar a PDF | Con dificultad (sin confirmación visible) | 2 | 3 |
| 15 | Cerrar sin guardar | Éxito | 4 | 5 |
| 16 | Exploración libre | (no se califica) | 12 | - |

## 4. Detalle por tarea

**T1.** Clic en «Abrir archivo» de la bienvenida y elegí inicio.md; abrió directo en modo Leer. «Ah, se ve lindo, como una página.» Sin callejones. (`01a.png`)

**T2.** Bajé con la rueda y toqué el cuadradito de «Leer capítulo 3»; se tildó y apareció «Tarea actualizada · sin guardar: Ctrl+S la guarda, Ctrl+Z la deshace». Me gustó que avise. Pero la pestaña quedó con un punto: «¿tengo que guardar solo por tildar?». (`02b.png`)

**T3.** Botón «Copiar» junto al bloque bash; cambió a «Copiado» con tilde. Leí el portapapeles justo después y contenía `ping 192.168.0.1` y `git status`, correcto. (`03a.png`)

**T4.** Abrí manual-largo.md por Archivo > Abrir documento. Pasé el puntero por los iconos y el segundo de la barra dice «Índice del documento (Ctrl+Shift+L)». Se abrió un panel con 49 títulos. Tuve que bajar la lista con la rueda hasta «Capítulo 30» y hacer clic: saltó ahí y el panel se cerró. Pensé: «ojalá hubiera un cuadro para escribir 30». (`04c`, `04d`, `04e`, `04f`)

**T5.** Clic en la pestaña inicio.md: quedé en el mismo lugar en el que estaba (Comandos útiles / Tareas). Bien. (`05a.png`)

**T6.** Menú Carpeta > Abrir carpeta; apareció el panel «Archivos» con el árbol. Clic en celula.md y la abrió. Aviso en pantalla: «Carpeta de apuntes indexada: 5 notas, 0 omitidas; la búsqueda cubre solo parte del contenido»: Lucía no entiende «solo parte del contenido» y se preocupa. (`06b.png`, `06c.png`)

**T7.** Primero toqué la lupa de arriba: es «Buscar en este documento» (el texto menciona Ctrl+Shift+F para toda la carpeta). Cerré con Esc, abrí Carpeta > «Buscar en carpeta», escribí «napoleon» (sin tilde) y apareció un resultado: «Revolución francesa · contenido: Termin…» (texto cortado). Al hacer clic abrió historia.md con «Napoleón» resaltado y «1 de 1». Funciona aun sin tilde. Callejón: la lupa. (`07b`, `07c`, `07d`)

**T8.** «+» creó «sin título 6» en modo Editar. Escribí «resumen», abrí «Párrafo» y elegí «Título 1 (H1)»: la pantalla muestra `# resumen` («¿qué es ese numeral?»). Para la lista usé Párrafo > Lista con viñetas y tres ítems con Enter. Para la negrita pulsé la B y escribí la palabra: apareció `**importante**` con los asteriscos visibles. Al dar Enter, el cursor estaba antes de los `**` de cierre: quedó una viñeta vacía y otra con `**` suelto; en modo Leer se ve «atp **importante» y una viñeta con `**` (`09d.png`). Tuve que arreglarlo borrando a mano varias veces, con asteriscos y todo. Guardé con Archivo > Guardar: apareció «Guardado» y la pestaña pasó a `resumen.md` (el diálogo de archivo no se ve en esta build; usé la respuesta preparada). (`08c`, `08d`, `08e`, `09d`, `09h`)

**T9.** Botón de tabla de la barra: insertó una tabla con «Columna / Valor». «Insertar > Insertar símbolo o emoji» abrió una lista filtrable; elegí «emoji importante» (chincheta) y se agregó. En modo Leer se ven la tabla y el emoji bien. Para editar la tabla hay que tocar texto con barras `|`. (`09a`, `09c`, `09d`)

**T10.** «Editar» muestra el texto «por dentro» y «Comparar» muestra fuente y resultado lado a lado, con el aviso «Vista dividida · fuente editable y lectura sincronizada». Muy claro. (`10a.png`)

**T11.** Vista > «Dividir a la derecha con otra pestaña» ofreció una lista de apuntes; elegí celula.md y quedaron dos paneles, cada uno con su propio Leer/Editar/Comparar. Se entiende. En la lista, una fila queda resaltada (historia) distinta de la que toqué. (`11b`, `11c`)

**T12.** Vista > «Ventana siempre encima: desactivado» la activó; salió «Ventana fijada encima de las demás · clic en la chincheta para soltarla» y apareció una chincheta verde arriba. Para achicar usé la herramienta de redimensionar (560x520): todo se reacomoda, pero desaparecen los nombres de los menús Archivo/Editar/Vista/Carpeta y las pestañas se cortan («inici…», «manual-…»). (`12a`, `12b`)

**T13.** Icono de sol arriba a la derecha: pasó a claro con aviso «Tema de día · Ctrl+Shift+D para volver». Otro clic y volvió a oscuro. El icono muestra el tema al que vas a pasar (sol en oscuro, luna en claro), lo cual al principio confunde. (`13a`, `13b`)

**T14.** Archivo > Exportar a PDF. Esperé unos segundos y no vi ningún aviso: Lucía no sabe si salió ni dónde quedó. Como evaluadora comprobé que `resumen.pdf` apareció en `apuntes\` (49 KB), pero ella no habría podido saberlo. Posible que el aviso haya durado menos que mi espera. (`14a.png`)

**T15.** «+», escribí «cosas para el examen» y toqué la X de la pestaña; fallé un clic porque la X se había movido al agrandarse las pestañas. Diálogo: «¿Guardar los cambios de "cosas para el examen"?» con «Cerrar sin guardar», «Cancelar» y «Guardar»; aclara que los cambios quedan en una recuperación. Lucía elegiría «Cerrar sin guardar» por ser un borrador. (`15c`, `15d`)

**T16 (exploración libre, 12 acciones).** Menú «Estudio»: lista larguísima de subrayados, marcas de libro y «anotaciones sin lugar»: «esto es demasiado, no sé para qué sirve». El icono `>_` abre «Buscar una acción» con filtro; escribí «pdf» y apareció «Exportar a PDF»: útil. Clic en el enlace «nota-que-no-existe»: diálogo amable «Esta nota no existe… Crear nota / Ahora no»; elegí «Ahora no». Hover del icono de triángulo: «Plegar o desplegar sección enfocada»; al pulsarlo plegó «Comandos útiles». No pulsé el enlace de Wikipedia: esperaba que abriera el navegador. (`16a` a `16g`)

## 5. Problemas

| ID | Sev. | Tarea | Esperado | Pasó | Captura |
|----|------|-------|----------|------|---------|
| US-01 | S2 | T8 | Dar Enter después de la negrita y seguir en la lista | Quedó `**` suelto y una viñeta vacía; el formato se rompe en modo Leer | `09d.png` |
| US-02 | S3 | T8/T9/T10 | Ver el texto con formato al editar, como en Word | Muestra `#`, `-`, `**`, `\|---\|`: incomprensible sin saber Markdown | `08c.png`, `08d.png` |
| US-03 | S2 | T14 | Aviso «PDF guardado en…» | No vi confirmación ni ubicación | `14a.png` |
| US-04 | S3 | T12 | Con ventana chica, los menús siguen accesibles | Desaparecen los nombres de menú y se cortan las pestañas | `12b.png` |
| US-05 | S3 | T7 | Una búsqueda que cubra toda la carpeta | La lupa busca solo en el documento; la de carpeta está en el menú Carpeta; los resultados salen cortados («Termin…») | `07b.png`, `07c.png` |
| US-06 | S3 | T6 | Aviso entendible | «la búsqueda cubre solo parte del contenido» sin explicar | `06b.png` |
| US-07 | S3 | T4 | Ir a «Capítulo 30» rápido | Hay que recorrer una lista de 49 títulos; no vi caja para escribir en el índice | `04e.png` |
| US-08 | S3 | T16 | Menú acotado | «Estudio» tiene más de 20 entradas con jerga (anotaciones, marcas, «sin lugar») | `16a.png` |
| US-09 | S4 | T2 | Que avise sin dejar la pestaña «sin guardar» | Tildar una tarea deja el apunte sin guardar | `02b.png` |
| US-10 | S4 | T13 | Icono que indique el tema actual | Sol/luna indican el destino, no el estado | `13a.png` |
| US-11 | S4 | T9 | Poder editar la tabla con botones | Solo se edita como texto con barras | `09a.png` |
| US-12 | S4 | T15 | X de pestaña siempre en el mismo lugar | La X se desplaza al cambiar el ancho de las pestañas | `15a.png` |
| US-13 | S4 | T11 | Selección clara en la lista | Fila resaltada distinta de la elegida | `11b.png` |

## 6. SUS detallado (1 = muy en desacuerdo, 5 = muy de acuerdo)

| Ítem | Puntaje |
|------|---------|
| 1. Usaría la app con frecuencia | 4 |
| 2. La encontré innecesariamente compleja | 2 |
| 3. Fue fácil de usar | 4 |
| 4. Necesitaría ayuda de alguien técnico | 3 |
| 5. Las funciones están bien integradas | 4 |
| 6. Hay demasiada inconsistencia | 2 |
| 7. La mayoría aprendería rápido | 4 |
| 8. Fue muy engorrosa | 2 |
| 9. Me sentí segura usándola | 3 |
| 10. Tuve que aprender muchas cosas antes | 2 |

Cálculo: ítems impares (puntaje - 1) = 3+3+3+3+2 = 14; pares (5 - puntaje) = 3+2+3+3+3 = 14. 28 x 2,5 = **70**.

## 7. Gustos y frustraciones

**Lo que más le gustó:** los avisos cuando algo pasa (tarea, tema, ventana encima), el diálogo de cerrar sin guardar, el buscador que encuentra «napoleon» sin tilde, «Comparar», el botón Copiar y que el enlace roto ofrezca crear la nota.

**5 mayores frustraciones:**
1. La negrita rompiendo la lista al dar Enter (`**` suelto).
2. Ver `#`, `**` y `|---|` al editar.
3. El PDF sin confirmación visible.
4. Los menús que desaparecen al achicar la ventana.
5. La búsqueda: la lupa no busca en todo y los resultados vienen cortados.

**Lo que no encontró:** un campo para ir a un capítulo por nombre; cómo agregar filas a una tabla sin tocar barras; confirmación o ubicación del PDF; una explicación del menú «Estudio».

**¿La recomendaría?** Sí, con reservas: «A mi compañera sí, porque leer y buscar es lindo y rápido, pero a alguien que no sepa de símbolos le diría que use Comparar para no asustarse».

## 8. Cita final

«Leer apuntes es facilísimo; escribirlos me da miedo cuando aparecen los asteriscos.»
