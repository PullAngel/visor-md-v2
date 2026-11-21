# Informe de usabilidad: Visor MD (ronda 5), por Lucía

Participante: Lucía, 21 años, estudiante de Biología. Primera vez con la app. Capturas en `C:\Dev\archivo\eval-ronda-5\usuario\capturas\`. Una sola pasada, ventana de 1280x720.

## 1. Resumen

- **Tasa de éxito:** 13 de 16 tareas con éxito; 3 con dificultad (T7, T8, T9); 0 fracasos. Todas las tareas se completaron.
- **SUS: 67,5 / 100.** Está por debajo del promedio habitual (68).
- **Las 5 peores fricciones:**
  1. US-01: la negrita queda rota si después se inserta otra cosa (quedan `**` sueltos).
  2. US-02: la lupa de arriba busca solo en el documento abierto; para buscar en todos los apuntes hay que ir a otro lado.
  3. US-03: el resultado de búsqueda muestra el título y un texto cortado, no el archivo ni la palabra encontrada.
  4. US-04: el aviso «la búsqueda cubre solo parte del contenido» asusta y no se entiende.
  5. US-08: con la ventana chica, la tabla del panel «Comparar» se pisa; tampoco se ve si la ventana quedó encima.

## 2. Primera impresión (test de 5 segundos)

Captura: `00.png`.

- **¿Qué es?** «Una app para abrir y escribir notas, tipo Word pero más simple. Dice Visor MD y "apuntes", así que es para notas.»
- **¿Qué se puede hacer?** «Abrir una carpeta de apuntes, abrir un archivo o hacer un apunte nuevo. Y hay Leer, Editar y Comparar.»
- **¿Qué llama la atención?** «Los tres botones grandes del medio, que se entienden de una. El color verde oscuro es lindo.»
- **¿Qué no entiendo?** «Qué es "Markdown" o "Comparar". Los dibujitos de arriba, que son muchos y no sé para qué sirven. "Ctrl+Shift+O" no me dice nada. "Abrir tus archivos .md con Visor MD · Configurar" parece letra chica de otra cosa.»

## 3. Tabla de tareas

| # | Tarea | Resultado | Acciones | Confianza |
|---|-------|-----------|---------:|----------:|
| 1 | Abrir inicio.md | Éxito | 2 | 5 |
| 2 | Marcar «Leer capítulo 3» | Éxito | 3 | 5 |
| 3 | Copiar el comando | Éxito | 1 | 5 |
| 4 | Manual largo, «Capítulo 30» | Éxito | 4 | 4 |
| 5 | Volver a inicio | Éxito | 1 | 5 |
| 6 | Abrir carpeta y la célula | Éxito | 3 | 5 |
| 7 | Buscar «Napoleón» en todo | Con dificultad | 6 | 4 |
| 8 | Crear `resumen.md` | Con dificultad | 17 | 3 |
| 9 | Tabla y emoji | Con dificultad | 4, más unas 20 para arreglar | 2 |
| 10 | Texto por dentro y renderizado | Éxito | 1 | 5 |
| 11 | Dos apuntes lado a lado | Éxito | 3 | 4 |
| 12 | Achicar y fijar encima | Éxito (ver nota) | 2 | 3 |
| 13 | Modo claro y volver | Éxito | 2 | 5 |
| 14 | Exportar a PDF | Éxito | 2 | 4 |
| 15 | Cerrar sin guardar | Éxito | 4 | 5 |
| 16 | Exploración libre | Completada | 12 | n/a |

## 4. Detalle por tarea

**T1: abrir inicio.md.** Éxito, 2 acciones. Camino: «Abrir archivo» de la bienvenida (`t1-a.png`). El documento se abrió en modo Leer, bien formateado. «Perfecto, se ve como un documento de verdad.»

**T2: marcar «Leer capítulo 3».** Éxito, 3 acciones: dos rodadas de scroll hasta «Tareas» y un clic en el cuadradito (`t2-b.png`, `t2-c.png`). El cuadradito se tildó al instante. La pestaña mostró un puntito y apareció «Tarea actualizada · sin guardar: Ctrl+S la guarda, Ctrl+Z la deshace». «Ah, sin guardar. Uso el diskette de arriba.» Lo pulsó y el puntito desapareció (`t2-save.png`; no lo conté como acción). El scroll rueda poco por vez, hizo falta repetirlo. Sin errores.

**T3: copiar el comando.** Éxito, 1 acción: el botón «Copiar» de la caja bash. Cambió a «✓ Copiado» (`t3-a.png`). Leí el portapapeles justo después y tenía `ping 192.168.0.1` y `git status`. «Se nota que se copió, me gusta.»

**T4: «Capítulo 30» del manual largo.** Éxito, 4 acciones (`t4-a.png` a `t4-d.png`). Camino: abrir archivo (icono de carpeta chica), probar el icono de rayitas (resultó ser un Índice de 49 títulos), bajar la lista con la rueda y clic en «Capítulo 30». Cayó justo en el título. No hubo callejones. Lo que cuesta es adivinar que ese icono es un índice; no hay texto que lo diga hasta que se abre.

**T5: volver a inicio.** Éxito, 1 acción: clic en la pestaña (`t5-a.png`). Quedó en «Comandos útiles» y «Tareas» como la había dejado, con la tarea tildada. «Sí, quedé donde estaba.»

**T6: trabajar con toda la carpeta.** Éxito, 3 acciones: menú «Carpeta», «Abrir carpeta», y clic en `celula.md` del panel de archivos (`t6-a.png` a `t6-c.png`). Apareció una lista de 5 notas más un aviso (ver US-04). «¿"Cubre solo parte del contenido"? ¿Se me perdió algo?» Pese a eso, la nota se abrió bien.

**T7: buscar «Napoleón» en todos los apuntes.** Con dificultad, 6 acciones, 1 callejón. Camino: lupa de arriba (`t7-a.png`). Decía «Buscar en el documento», o sea solo la nota abierta. Escape. Menú «Carpeta», «Buscar en carpeta» (`t7-d.png`). Escribí `napoleon` sin tilde y igual lo encontró (`t7-e.png`). Un resultado: «Revolución francesa · contenido: Termin…». «¿Dónde está eso? Si no abro, no sé qué archivo es.» Clic en el resultado y se abrió `historia.md` con «Napoleón» resaltado (`t7-f.png`), con contador «1 de 1». El resultado final es muy bueno. El camino no fue evidente. Confianza 4.

**T8: crear `resumen.md`.** Con dificultad (resultado correcto, camino largo), unas 17 acciones (`t8-a.png` a `t8-m.png`).
- Crear: botón «+». El mensaje «Escribí aquí. El formato está en la barra de arriba» ayudó.
- Título: escribí «resumen», abrí «Párrafo» y elegí «Título 1 (H1)». El texto pasó a `# resumen`. «¿Y ese numeral? Yo no lo escribí.»
- Lista: «Párrafo», «Lista con viñetas», tres ítems con Enter. Quedó bien.
- Negrita: salir de la lista requirió Enter dos veces y dejó una línea vacía de más. Botón **B**, escribí «importante». Quedó como `**importante**`. «No sé qué son los asteriscos, pero el botón B funcionó.»
- Guardar: diskette, destino `resumen.md`. La pestaña pasó a «resumen.md» y salió «Guardado».
- Confianza 3: funcionó, pero vi símbolos raros que no entiendo.

**T9: tabla y emoji.** Con dificultad (falló el contenido).
- Tras la negrita apreté Enter dos veces y el botón de tabla (icono cuadriculado, que no explica qué es).
- Pasó algo malo: el cursor estaba dentro de la negrita, entre `importante` y `**`. Los Enter partieron los asteriscos, y quedaron `**importante` y, al final de la nota, un `**` suelto (`t9-a.png`).
- «¡Se rompió mi negrita!»
- Emoji: Insertar, «Insertar símbolo o emoji», lista con dibujitos y nombres. Muy fácil. Elegí «emoji idea» y quedó dentro de la celda de la tabla (`t9-c.png`, `t9-d.png`).
- Para arreglarlo escribí `**` a mano y salió `++` (límite de la herramienta). Borré la línea y repetí con el botón B, ahora con el cursor bien posicionado. Guardé y la nota quedó correcta (`t10-c.png`). Fueron unas 20 acciones extra, que no cuento en la tarea.
- Confianza 2 en ese momento.

**T10: por dentro y lado a lado.** Éxito, 1 acción: botón «Comparar» (`t10-a.png`). Aparecieron código y resultado, con el aviso «Vista dividida · fuente editable y lectura sincronizada». «Ah, era eso. Por el nombre no lo hubiera adivinado, pero se entiende al verlo.» Mostró el `**importante` roto del problema anterior, así que la comparación sirvió para descubrir el error.

**T11: dos apuntes juntos.** Éxito, 3 acciones: Vista, «Dividir a la derecha con otra pestaña», elegir `historia.md` de la lista (`t11-a.png` a `t11-c.png`). Quedaron dos paneles, cada uno con su barra. Estaba en «Comparar», así que el panel izquierdo se veía muy apretado (código más lectura).

**T12: achicar y dejar encima.** Éxito, con una limitación, 2 acciones.
- Chinche de arriba a la derecha: con el puntero encima dice «Fijar la ventana encima de las demás» (`t12-a.png`). Clic, se pone verde y sale «Ventana fijada encima de las demás · clic en la chincheta para soltarla» (`t12-b.png`).
- Achicar: lo hice con el comando de tamaño de la herramienta, 700x480. Los menús se juntaron en un icono de hamburguesa (bien), pero la tabla del panel de lectura se superpuso: «ColumnaValor» (`t12-c.png`).
- No puedo confirmar que la ventana quedó realmente sobre las demás: solo veo el estado y el aviso, no hay otra ventana al lado para comparar.
- Confianza 3.

**T13: modo claro y oscuro.** Éxito, 2 acciones: icono de sol (`t13-a.png`), «Tema de día. Ctrl+Shift+D para volver», y de nuevo (`t13-b.png`), «Tema de noche». El modo claro es color crema, no blanco. «Me esperaba blanco, pero es agradable para leer.»

**T14: exportar a PDF.** Éxito, 2 acciones: Archivo, «Exportar a PDF» (`t14-a.png`). Mensaje «PDF exportado: resumen.pdf · 1 página» (`t14-b.png`). Verifiqué que el archivo existe en la carpeta de apuntes. No abrí el PDF. El aviso no dice dónde quedó guardado.

**T15: cerrar sin guardar.** Éxito, 4 acciones: «+», escribir «idea para el examen», clic en la X de la pestaña (`t15-a.png`, `t15-b.png`). Apareció «¿Guardar los cambios de "idea para el examen"?» con tres botones: «Cerrar sin guardar» (rojizo), «Cancelar», «Guardar» (verde, resaltado). El texto aclara que sin guardar quedan en una recuperación. Lucía elegió **Guardar**, como haría de verdad, por miedo a perder algo. Eligió destino `idea.md` y la pestaña se cerró (`t15-c.png`). «Muy claro, me tranquilizó la frase de la recuperación.»

**T16: exploración libre, 12 acciones.**
1. Menú «Estudio» (`t16-1.png`): una lista larguísima de anotaciones, marcas de libro, etc. «Esto no es para mí, me abruma.» Escape.
2. Pasé el puntero por el triángulo: «Plegar o desplegar sección enfocada» (`t16-2.png`). Clic: se plegó «Comandos útiles» (`t16-3.png`).
3. Clic en el enlace «Historia» dentro de la nota: se activó la pestaña que ya la tenía abierta, con el aviso «Se activó la pestaña que ya contenía el documento» (`t16-4.png`).
4. Un clic en un espacio vacío del panel izquierdo: el contenido de ese panel se movió unos 60 px (`t16-5.png`).
5. Icono `>_`: tiene el cartel «Buscar una acción · Ctrl+Shift+P» (`t16-6.png`). Abre una lista de 77 acciones (`t16-7.png`). Escribí «pdf» y la lista se filtró a «Exportar a PDF» (`t16-8.png`). «Esto es genial para cuando no encuentro algo.»
6. Menú «Editar» con Deshacer, Copiar, y «Copiar para Discord o correo» (`t16-9.png`). «Eso me encanta.»
7. No pulsé el enlace «Wikipedia». Esperaba que abriera el navegador con la página (flecha ↗ al lado, que sí lo sugiere).

## 5. Problemas encontrados

| ID | Sev. | Tarea | Esperaba | Pasó | Captura |
|----|:----:|:-----:|----------|------|---------|
| US-01 | S2 | T9 | Que la negrita quedara cerrada y que poder seguir escribiendo, o agregar una tabla, no la rompiera. | Después del botón B el cursor queda entre las palabras y los `**` de cierre. Dos Enter cortan la negrita: queda `**importante` arriba y `**` suelto abajo. El texto de Lucía queda dañado sin aviso. | `t9-a.png`, `t10-a.png` |
| US-02 | S2 | T7 | Que la lupa de arriba buscara en todos los apuntes. | Esa lupa dice «Buscar en el documento». La búsqueda en todos los apuntes está en «Carpeta» o en una pestaña de panel con otra lupa. Hay dos buscadores casi iguales. | `t7-a.png`, `t7-d.png` |
| US-03 | S3 | T7 | Ver el nombre del archivo y la frase donde aparece «Napoleón». | Salió «Revolución francesa · contenido: Termin…», con el título (no el archivo) y el fragmento cortado, sin la palabra buscada. | `t7-e.png` |
| US-04 | S3 | T6 | Un aviso de que la carpeta se abrió y listo. | Dice «la búsqueda cubre solo parte del contenido». No explica qué se perdió ni qué hacer. Suena a error. | `t6-b.png` |
| US-05 | S3 | T8 | Que el título fuera un título, sin símbolos. | En Editar se ven `#`, `-` y `**` que Lucía no conoce. Funcionan, pero no se entienden. Salir de la lista exige dos Enter y deja una línea vacía de más. | `t8-e.png`, `t8-l.png` |
| US-06 | S3 | T12 | Que, al achicar, todo siguiera usable, y saber si quedó encima. | Con 700 px, la tabla del panel de lectura en «Comparar» se superpone («ColumnaValor»). No se ve ninguna señal clara de estar encima de otras ventanas, solo la chinche verde y el aviso. | `t12-c.png` |
| US-07 | S3 | T16 | Un menú «Estudio» sencillo. | Menú larguísimo (subrayado, marcas, anotaciones sin lugar, restaurar...). Abruma a una persona que solo quiere leer. | `t16-1.png` |
| US-08 | S4 | T10 | Que el botón dijera «lado a lado» o algo así. | «Comparar» no se entiende solo; se entiende después de usarlo, por el aviso. | `t10-a.png` |
| US-09 | S4 | T14 | Saber dónde quedó el PDF. | «PDF exportado: resumen.pdf · 1 página» no dice la carpeta ni da un botón para abrirlo. | `t14-b.png` |
| US-10 | S4 | T13 | Un modo claro blanco. | Es crema/sepia. No es un fallo, pero sorprende. | `t13-a.png` |
| US-11 | S4 | T16 | Que el panel no se moviera al pasar de un panel a otro. | El contenido del panel izquierdo se corrió unos 60 px tras un clic en un hueco, y los triángulos de las secciones desaparecieron un momento. | `t16-4.png`, `t16-5.png` |
| US-12 | S4 | T4 | Entender los iconos de la segunda barra. | Los iconos (índice, carpeta, lupa, hoja) no dicen nada hasta pasar el puntero; en T4 hubo que probarlos. | `t4-a.png` |

## 6. SUS detallado

Escala 1 (muy en desacuerdo) a 5 (muy de acuerdo). Contribución: ítems impares = respuesta menos 1; pares = 5 menos respuesta.

| # | Ítem | Resp. | Contrib. |
|---|------|:-----:|:--------:|
| 1 | Me gustaría usar esta app con frecuencia | 3 | 2 |
| 2 | La encontré innecesariamente compleja | 3 | 2 |
| 3 | Me pareció fácil de usar | 4 | 3 |
| 4 | Necesitaría ayuda de alguien con conocimientos técnicos | 2 | 3 |
| 5 | Las funciones están bien integradas | 4 | 3 |
| 6 | Hay demasiada inconsistencia | 3 | 2 |
| 7 | La mayoría aprendería a usarla rápido | 4 | 3 |
| 8 | Me resultó muy engorrosa | 2 | 3 |
| 9 | Me sentí muy segura usándola | 4 | 3 |
| 10 | Necesité aprender muchas cosas antes de empezar | 2 | 3 |

Suma 27 x 2,5 = **67,5 / 100**.

Justificación breve: baja el 2 y el 6 por los dos buscadores parecidos y los símbolos de Markdown; sube el 3, el 7 y el 9 por los avisos y los mensajes de ayuda.

## 7. Gustos y frustraciones

**Lo que más le gustó**
- Los mensajes breves que explican qué pasó y cómo deshacerlo («sin guardar: Ctrl+S la guarda, Ctrl+Z la deshace», «Tema de día · Ctrl+Shift+D para volver»).
- El botón «Copiar» que cambia a «✓ Copiado».
- La ayuda en el cierre sin guardar y su texto sobre la recuperación.
- El selector de símbolos y emojis, con nombres en español.
- La lista de acciones del icono `>_`, que se filtra al escribir.
- Que la búsqueda ignore la tilde.

**Las 5 mayores frustraciones**
1. La negrita rota por dos Enter seguidos (US-01), que le dejó asteriscos en el texto.
2. Dos buscadores casi iguales y uno solo busca en la nota abierta (US-02).
3. Los símbolos `#`, `-` y `**` en el modo Editar (US-05).
4. El aviso «la búsqueda cubre solo parte del contenido» (US-04).
5. El menú «Estudio» enorme (US-07), y los iconos sin texto de la segunda barra (US-12).

**Lo que no encontró**
- Un botón o aviso para abrir la carpeta donde quedó el PDF.
- Una forma de ver el nombre del archivo en los resultados de búsqueda.
- Una señal visual clara de que la ventana está por encima de las demás.

**¿La recomendaría?** Sí, con reservas: «A mi compañera de Biología sí, para leer apuntes y buscar cosas. Para escribir, antes le avisaría que aparecen símbolos raros y que no apriete Enter dos veces después de poner negrita.»

## 8. Cita final

«Se entiende rápido cómo abrir y leer. Donde me trabé es donde aparecen cosas de programadores: los asteriscos y la búsqueda que no busca en todo. Me gustaría que la app me avisara qué está pasando, como hace con el copiado y el guardado.»

## Notas de método y límites de la herramienta

- Para todos los diálogos de archivo usé `answer` con la ruta que Lucía elegiría. Archivos creados: `apuntes\resumen.md`, `apuntes\resumen.pdf`, `apuntes\idea.md`. También se modificó `inicio.md` (tarea tildada, guardada).
- No se reportan los límites de la herramienta (mayúsculas, tildes, Shift). El `*` tecleado salió como `+` y obligó a rehacer la negrita con el botón B.
- No leí código, documentación ni la carpeta `qa`. No pulsé el enlace de Wikipedia. La app quedó cerrada con `close`.
