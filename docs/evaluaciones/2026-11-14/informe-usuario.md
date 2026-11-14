# Prueba de usabilidad: Visor MD, participante «Lucía»

**Participante simulada:** Lucía, 21 años, estudiante de Biología. Usa Word, Google Docs y WhatsApp. No conoce Markdown.
**Fecha:** 2026-11-14. **Copia de la app:** `usuario`, con perfil limpio y ventana de 1280×760.
**Capturas:** `<eval>\usuario\capturas\`. Los archivos `m-*.png` son montajes de recortes.

---

## 1. Resumen

| Métrica | Valor |
|---|---|
| Tareas evaluadas (1–15) | 15 |
| Éxito sin dificultad | 6 (40 %): T1, T2, T3, T5, T10, T15 |
| Éxito con dificultad o parcial | 8 (53 %): T4, T6, T8, T9, T11, T12, T13, T14 |
| Fracaso o abandono | 1 (7 %): T7, búsqueda en todos los apuntes |
| **Tasa de éxito total (completadas)** | **14/15 = 93 %** |
| **SUS** | **35 / 100** (por debajo de lo aceptable: 68 es la media) |

T11 terminó bien, pero el panel nuevo heredó el modo «Comparar» y Lucía tuvo que cambiarlo. Por eso cuenta como «con dificultad».

**Las 5 peores fricciones**
1. **No pude escribir en los buscadores** («Buscar en carpeta» y «Buscar en documento»). El campo seguía mostrando el texto de ejemplo y «Sin resultados», así que la búsqueda en todos los apuntes no se pudo hacer (US-01).
2. **El editor muestra símbolos «por dentro»** (`#`, `**`, `|---|`). Además, el botón «H» no hace un título: resalta el texto (US-05).
3. **Lo escrito no queda como Lucía cree.** La frase en negrita terminó pegada al último punto de la lista. El símbolo ✓ quedó fuera de la tabla y no se ve en la vista de lectura (US-02, US-03).
4. **El índice del manual muestra de a 18 títulos.** La rueda del mouse mueve el documento y no la lista. Para llegar al «Capítulo 30» hubo que pulsar la flecha abajo unas 30 veces (US-04).
5. **Al abrir la carpeta no aparecen los archivos.** Solo sale un aviso técnico («Workspace indexado… presupuesto de memoria»). Exportar a PDF y cambiar el tema solo se encuentran escribiendo en «Más acciones» (US-06, US-08, US-09).

---

## 2. Primera impresión (test de 5 segundos)

Captura: `00-inicio.png`.

Al abrir aparece una ventana oscura con un cartel en el centro: «Recuperación local activada… Es texto sin cifrar, no reemplaza a Guardar…». Detrás se ve una pestaña «sin título.md», una fila de íconos sin nombre y otra con algunos rótulos (Bloques, Listas, Insertar, Estudio, Más). Hay un botón triple «Leer / Editar / Comparar». El título de la ventana no dice el nombre de la app.

- **¿Qué es esta app?** «Parece un bloc de notas, como un Word pero oscuro. Dice "Escribí aquí", así que es para escribir.»
- **¿Qué se puede hacer?** «Escribir, leer y comparar algo. Lo de "Estudio" me gusta, capaz que tiene cosas para estudiar.»
- **¿Qué le llama la atención?** «Que es todo verde y negro, muy de programador. Y el cartel que salta apenas abro.»
- **¿Qué no entiende?** «¿Qué es "recuperación local", "perfil local", "sin cifrar"? ¿Me tengo que preocupar? ¿Qué es ".md"? Los íconos de arriba no tienen nombre. ¿Qué es esa franja negra abajo?»

---

## 3. Tabla de tareas

| # | Tarea | Resultado | Acciones | Confianza | Problema principal |
|---|---|---|---|---|---|
| 1 | Abrir inicio.md | Éxito | 2 | 5 | La pestaña vacía «sin título.md» queda abierta |
| 2 | Marcar «Leer capítulo 3» | Éxito | 4 (incluye guardar) | 4 | Mensaje «guardado de forma atómica» |
| 3 | Copiar el comando | Éxito | 1 | 4 | El bloque copia dos comandos juntos (ping y git status) |
| 4 | Ir al Capítulo 30 del manual | Éxito con dificultad | ~38 (30 de flecha) | 3 | El índice muestra 18 títulos y la rueda no mueve la lista |
| 5 | Volver a inicio y ver la posición | Éxito | 1 | 5 | — (conservó la posición) |
| 6 | Abrir la carpeta y luego celula.md | Éxito con dificultad | 4 | 3 | Abrir la carpeta no muestra los archivos |
| 7 | Buscar «Napoleón» en todos los apuntes | **Fracaso** | ~15 | 1 | Los campos de búsqueda no recibieron texto |
| 8 | Crear resumen.md (título, lista, negrita) | Éxito con dificultad | ~25 | 3 | «H» no es título; markdown crudo; la negrita quedó dentro de la lista |
| 9 | Tabla y emoji | Éxito parcial | 7 | 2 | Tabla de ejemplo en texto crudo; no hay emojis; el ✓ se pierde en la vista |
| 10 | Ver código y resultado lado a lado | Éxito | 2 | 5 | — |
| 11 | Dos apuntes lado a lado | Éxito con dificultad | 5 | 4 | Solo se encuentra con clic derecho en la pestaña; el panel nuevo heredó «Comparar» |
| 12 | Ventana chica y siempre encima | Éxito con dificultad | 3 | 4 | La chincheta no tiene tooltip |
| 13 | Modo claro y volver | Éxito con dificultad | 3 | 4 | Solo está en «Más acciones» |
| 14 | Exportar a PDF | Éxito con dificultad | 4 | 4 | No hay botón visible; hubo que escribir «exportar» |
| 15 | Cerrar sin guardar | Éxito | 3 | 4 | Al cerrar se pierde la vista dividida; la «recuperación» no se encuentra |
| 16 | Exploración libre | — | ~12 | — | Ver la sección 4 |

---

## 4. Detalle por tarea

### T1: Abrir inicio.md (Éxito, 2 acciones, confianza 5)
- **Camino:** cerró el cartel con «Entendido», pasó el puntero por el ícono de carpeta y vio el tooltip «Abrir documento · Ctrl+O». Hizo clic y eligió `inicio.md`.
- La nota se abrió en modo «Leer» y se ve muy prolija (`02-inicio-abierto.png`).
- **Confusión:** «¿Por qué sigue la pestaña "sin título.md"? No la quiero.» El texto ocupa solo la mitad izquierda y a la derecha queda un espacio vacío grande. En el recuadro «Nota», la palabra queda partida por la barra verde.

### T2: Marcar la tarea (Éxito, 4 acciones, confianza 4)
- **Camino:** bajó con la rueda, vio «Tareas» e hizo clic en la casilla. Se tildó al instante (`04-tarea-marcada.png`).
- La pestaña pasó a mostrar «inicio.md *». «Ah, como en Word, el asterisco es que no guardé.» Pasó el puntero por el disquete, vio «Guardar · Ctrl+S» e hizo clic.
- **Confusión:** el aviso dice «Documento guardado de forma atómica». «¿Atómica? ¿Como una bomba?» (`06-guardado.png`). Además, el tooltip «Guardar» quedó en pantalla aunque el puntero ya estaba en otro lado.
- Verificación: el archivo quedó con `- [x] Leer capítulo 3`.

### T3: Copiar el comando (Éxito, 1 acción, confianza 4)
- Hizo clic en «Copiar» junto a `ping 192.168.0.1`. El botón cambió a «Copiado» y salió el aviso «Código copiado al portapapeles» (`07-copiar.png`).
- El portapapeles quedó con dos líneas, `ping 192.168.0.1` y `git status`. «Yo quería uno solo, pero bueno, vienen juntos.»

### T4: Manual largo, llegar al Capítulo 30 (Éxito con dificultad, ~38 acciones, confianza 3)
- **Camino:** abrió el archivo con la carpeta de arriba. «Es larguísimo y no veo barra para bajar.» No hay barra de desplazamiento visible.
- Recorrió los íconos de la segunda barra (`m-09.png`). Los tooltips son claros: Índice del documento, Plegar, Abrir carpeta, Notas de la carpeta, Buscar en carpeta y «Backlinks» («¿back qué?»).
- Abrió el «Índice». La lista muestra «1–18 de 49» y termina en el Capítulo 15 (`10-indice.png`).
- **Callejón 1:** usó la rueda sobre la lista y se movió el documento, no la lista (`11-indice-rueda.png`). «¿Eh? Yo quería bajar en la lista.»
- **Callejón 2:** hizo clic en «Capítulo 15» para acercarse. El panel se cerró solo. Al reabrirlo, la lista empezaba otra vez arriba de todo (`13-indice-2.png`).
- El aviso decía «flechas eligen». Bajó con la flecha unas 30 veces hasta ver el «Capítulo 30» (`14-indice-flechas.png`) e hizo clic (`15-cap30.png`).
- «Llegué, pero "rápido" no fue.»

### T5: Volver a inicio (Éxito, 1 acción, confianza 5)
- Hizo clic en la pestaña «inicio.md» y volvió justo donde estaba, en Tareas (`16-vuelta-inicio.png`). «¡Bien! Como las pestañas del navegador.»

### T6: Trabajar con la carpeta y abrir celula.md (Éxito con dificultad, 4 acciones, confianza 3)
- **Camino:** usó el ícono «Abrir carpeta» de la segunda barra y eligió `apuntes`. No cambió nada visible. Solo apareció el aviso «Workspace indexado: 5 notas, 0 omitidas; búsqueda limitada por presupuesto de memoria» (`17-carpeta.png`).
- «¿Y mis archivos dónde están? ¿Qué es un workspace? ¿Presupuesto de memoria?»
- Probó el ícono «Notas de la carpeta» y apareció el árbol con biologia/, celula.md y los demás (`18-notas-carpeta.png`). Hizo clic en celula.md y se abrió (`19-celula.png`). El panel lateral se cerró solo.

### T7: Buscar «Napoleón» en todos los apuntes (FRACASO, ~15 acciones, confianza 1)
- **Camino:** comparó las lupas. La de arriba dice «Buscar en documento» y la de abajo, «Buscar en carpeta». «Es la de abajo, quiero todos mis apuntes.»
- Hizo clic y se abrió la pestaña «Buscar» con «Buscar en apuntes...». Escribió «napoleon». El campo siguió mostrando el texto de ejemplo y «Sin resultados» (`21-buscar-napoleon.png`).
- Hizo clic sobre el campo y volvió a escribir: no pasó nada (`22-buscar-2.png`). Probó letras sueltas y tampoco (`m-23.png`).
- Probó la lupa de arriba («Buscar en documento…»): tampoco se escribió nada (`24-buscar-doc.png`).
- Entró por «Más acciones», escribió «buscar» (ahí sí se escribió) y eligió «Buscar en carpeta». Al escribir, el campo seguía vacío (`28-buscar-carpeta-3.png`). Probó también desde la pestaña «Buscar» y con Enter (`m-29.png`).
- «Me rindo. Escribo y no aparece nada. ¿Está roto?»
- **Nota del evaluador:** con la misma herramienta de escritura, el texto sí entró en el editor y en el filtro de «Más acciones». Solo los campos de búsqueda no lo recibieron. No se puede descartar del todo una interacción con la herramienta de prueba, pero para Lucía la búsqueda no funcionó.

### T8: Crear resumen.md (Éxito con dificultad, ~25 acciones, confianza 3)
- **Camino:** usó la pestaña «sin título.md», que estaba en «Editar», y escribió «resumen».
- Recorrió los tooltips de la barra de formato (`m-30.png`). Cosas que la confundieron:
  - El ícono «H» dice «Resaltar texto»: «Yo pensé que H era de título, como en Docs.»
  - El ícono del código en línea parece una «N».
  - Al lado hay otra «H» con el rótulo «Bloques».
- Hizo doble clic sobre la palabra, que no quedó seleccionada, y pulsó «H». La palabra quedó partida: `resu==texto==men…` (`32-H-resaltar.png`). «¡Me rompió la palabra!» Borró todo con la tecla de retroceso.
- Abrió «Bloques» y eligió «Convertir línea en encabezado H1» (`33-bloques.png`). «¿Encabezado H1? Supongo que es el título grande.» Apareció `# resumen` (`m-36.png`). «¿Y ese numeral? Yo quería que se viera grande.»
- Abrió «Listas» y eligió «Convertir línea en lista con viñetas». Escribió mitosis, Enter, meiosis, Enter, adn. Los guiones siguen solos al pulsar Enter, cosa que le gustó (`m-38.png`).
- Pulsó Enter dos veces para salir de la lista y escribió «lo mas importante: ». Pulsó **B** y apareció `**texto**` con «texto» seleccionado. Escribió «nucleo» y lo reemplazó bien (`m-40.png`). «Ah, qué práctico.»
- Guardó con el disquete y eligió `resumen.md`. Aviso: «Documento creado y guardado de forma atómica» (`41-guardar-resumen.png`).
- **Error que no vio en el momento:** en la vista de lectura, la frase en negrita quedó como parte del último punto de la lista: «adn lo mas importante: **nucleo**» (`49-comparar.png`).
- **Anomalía no confirmada:** dos veces aparecieron letras de más al escribir justo después de un clic: «resumenmen» y «resumemen#» (`m-31.png`, `34-h1.png`). Más tarde, en «Más acciones», se escribió «recuperp» en lugar de «recuper». Al escribir otra vez con calma salió bien. Puede ser la herramienta de prueba.

### T9: Tabla y emoji (Éxito parcial, 7 acciones, confianza 2)
- Pulsó el ícono de cuadrícula, «Insertar tabla Markdown». Apareció en texto `| Columna | Valor |`, `| --- | --- |`, `| texto | valor |` (`42-tabla.png`). «Esto no es una tabla, son rayitas. ¿Cómo agrego filas?»
- En «Insertar» encontró «Insertar símbolo o emoji» (`43-insertar.png`). El mismo menú tiene opciones que no entendió: «enlace de bóveda», «callout de nota». El contador dice «1–7 de 7» pero muestra 8 opciones.
- La lista muestra flechas, ✓, ≡, … («1–9 de 40») y ningún emoji de verdad (`44-emoji.png`). La rueda no mueve la lista y escribir «planta» no la filtra (`m-45.png`, `m-46.png`). Eligió «visto ✓».
- El ✓ quedó pegado después del último `|` de la tabla (`47-visto.png`). En la vista de lectura **no aparece** (`49-comparar.png`). «Puse el tilde y desapareció.»

### T10: Ver el texto «por dentro» y cómo queda (Éxito, 2 acciones, confianza 5)
- El tooltip de «Comparar» dice «Comparar fuente y vista · F3». Hizo clic y vio el código a la izquierda y el resultado a la derecha (`49-comparar.png`). «¡Esto sí! Ahora entiendo para qué son los numerales.» Gracias a esta vista descubrió los errores de T8 y T9.

### T11: Dos apuntes lado a lado (Éxito con dificultad, 5 acciones, confianza 4)
- Primero probó «Espacio de trabajo» y se abrió el árbol de notas (`50-espacio.png`). **Detalle:** el árbol sigue diciendo «5 notas» y no muestra el `resumen.md` recién guardado.
- Hizo clic derecho en la pestaña celula.md y apareció «Dividir a la derecha / Dividir abajo / Fijar o soltar / Cerrar» (`51-rclick-tab.png`). Eligió «Dividir a la derecha». La app preguntó qué mostrar en el panel nuevo y eligió resumen.md (`52-dividir.png`, `53-lado-a-lado.png`).
- El panel nuevo se abrió en modo «Comparar» y quedó con tres columnas apretadas. Lo pasó a «Leer» (`54-lado-leer.png`). «Hay dos botones de Leer/Editar, los de arriba y los de cada lado. Confunde un poco.»

### T12: Ventana chica y siempre encima (Éxito con dificultad, 3 acciones, confianza 4)
- Achicó la ventana a 640×480. Ahí recién apareció abajo una barra de estado: «12 palabras · 82 caracteres · Lectura · guardado · carpeta activa» (`55-chica.png`). En la ventana grande, ese lugar es una franja negra.
- Pasó el puntero por la chincheta y **no apareció tooltip** (`56-hov-pin.png`). Hizo clic igual y salió el aviso «Ventana fijada encima de las demás · clic en la chincheta para soltarla» (`57-pin.png`). «Lo adiviné por el dibujito.»

### T13: Modo claro y volver (Éxito con dificultad, 3 acciones, confianza 4)
- No vio ningún sol o luna en la pantalla. Abrió «Más acciones», encontró «Cambiar tema día o noche · T en lectura» (`58-menu-tema.png`) y lo pulsó. El modo claro se ve lindo (`59-claro.png`), aunque la franja negra de abajo sigue igual.
- Para volver usó la tecla T, que el propio menú indicaba (`m-60.png`). «Lo de la T está bueno, pero tendría que haber un botón.»

### T14: Exportar a PDF (Éxito con dificultad, 4 acciones, confianza 4)
- No hay ningún «Exportar» ni «Imprimir» a la vista. «En Word está en Archivo…» Abrió «Más acciones» (69 opciones en páginas de 9) y escribió «exportar». Apareció «Exportar a PDF», con el contador «1 resultados» (`61-exportar.png`).
- Eligió `resumen.pdf`. Aviso: «PDF exportado: resumen.pdf · 1 página» (`62-pdf.png`). El archivo se creó (unos 50 KB).

### T15: Escribir y cerrar sin guardar (Éxito, 3 acciones, confianza 4)
- Hizo clic en la pestaña inicio y luego en «Editar» del panel. Apareció el texto con `##`, ```` ``` ```` y `|`. «Uy, se llenó de símbolos raros» (`64-inicio-editar.png`). Escribió « repasar el lunes» y pulsó la X de la pestaña.
- Diálogo: «¿Guardar los cambios de «inicio.md»? Si cerrás sin guardar, los cambios quedan en una recuperación local que podés restaurar después», con los botones Cerrar sin guardar / Cancelar / Guardar (`65-cerrar-sin-guardar.png`). «Clarísimo, como en Word.»
- Eligió **Cerrar sin guardar**, porque era una prueba y lo había escrito encima del título. El archivo no cambió.
- **Sorpresa:** también se cerró la vista dividida y quedó una sola nota (`66-tras-cerrar.png`). «¿Y mi pantalla partida?»

### T16: Exploración libre (~12 acciones)
- **Enlaces internos:** hizo clic en «inicio» dentro de célula y se abrió inicio, pero en modo Editar, con todo el código a la vista (`67-link-inicio.png`). «Yo quería leerla.»
- **La recuperación prometida:** buscó en «Más acciones» «recuper» y «restaurar». Ninguna dio resultados (`m-69.png`). «Me dijo que podía restaurar, pero ¿dónde?»
- **Estudio:** el menú ofrece «Insertar pregunta de estudio», «Marcar contenido como entendido / dudoso / pendiente», «Preparar fragmentos para IA» (`70-estudio.png`). «¡Esto me sirve para el parcial! Aunque "herramienta portable" no sé qué es.»
- **Enlace roto:** «nota-que-no-existe» se ve igual que los enlaces buenos. Al hacer clic sale el aviso «La nota indicada por el enlace no existe en la carpeta de trabajo» (`72-link-roto.png`). Es claro, pero no ofrece crearla.
- **Plegar secciones:** el triangulito junto a cada título pliega la sección (`73-plegar.png`). «Re útil para estudiar.» El aviso dice «Sección plegada; la fuente permanece intacta» («¿qué fuente?»).

---

## 5. Problemas de usabilidad detectados

| ID | Sev. | Tarea | Qué esperaba Lucía → qué pasó | Captura |
|---|---|---|---|---|
| US-01 | **S1** | T7 | Escribir en «Buscar en apuntes…» y ver resultados → el texto no entra en el campo de búsqueda de carpeta ni en el de documento; queda «Sin resultados». En el editor y en «Más acciones» sí se puede escribir. | 21, 22, m-23, 24, 28, m-29 |
| US-02 | S2 | T8 | Que tras Enter dos veces la lista terminara y empezara un párrafo nuevo → en la vista de lectura, la frase quedó como parte del último punto («adn lo mas importante: **nucleo**»). No hay aviso en el editor. | m-39, 49 |
| US-03 | S2 | T9 | Insertar un emoji y verlo en el apunte → la lista solo tiene símbolos tipográficos, no se filtra escribiendo y no se mueve con la rueda. El ✓ quedó después del `|` final de la tabla y no aparece en la vista de lectura. | 44, m-45, m-46, 47, 49 |
| US-04 | S2 | T4 | Bajar por la lista del índice con la rueda → la rueda mueve el documento. La lista se ve de a 18 títulos («1–18 de 49») sin barra; al elegir un título se cierra y al reabrir vuelve al principio. | 10, 11, 13, 14 |
| US-05 | S2 | T8, T15 | Un editor visual como Word o Docs → se ve markdown crudo (`#`, `**`, `|---|`, ```` ``` ````). El ícono «H» es «Resaltar texto», no título. El título está en «Bloques → encabezado H1». | 32, 33, m-36, 64 |
| US-06 | S2 | T6 | Al abrir la carpeta, ver la lista de archivos → solo sale un aviso técnico; hay que descubrir otro ícono («Notas de la carpeta»). | 17, 18 |
| US-07 | S3 | T11 | Ver resumen.md en el árbol después de guardarlo → el árbol sigue en «5 notas» y no lo muestra. | 50 |
| US-08 | S3 | T13, T14 | Botones visibles para tema y exportar o imprimir → solo están en «Más acciones» (69 opciones en páginas de 9) y hay que escribir para encontrarlos. | 58, 61 |
| US-09 | S3 | Varias | Mensajes en lenguaje común → jerga: «guardado de forma atómica», «Workspace indexado… presupuesto de memoria», «Backlinks», «enlace de bóveda», «callout», «tabla Markdown», «encabezado H1», «herramienta portable», «la fuente permanece intacta». | 06, 17, 43, 70, 73 |
| US-10 | S3 | T0 | Entrar y empezar → lo primero es un cartel técnico («Recuperación local… perfil local… sin cifrar») que preocupa más de lo que ayuda. | 00 |
| US-11 | S3 | T15/16 | Encontrar la «recuperación local que podés restaurar después» → no hay ninguna acción visible; buscar «recuper» o «restaurar» da 0 resultados. | 65, 68, m-69 |
| US-12 | S3 | T15 | Cerrar una pestaña y mantener la vista dividida → al cerrar la pestaña se deshizo también la división. | 66 |
| US-13 | S3 | T9 | Tabla editable visualmente → se inserta una plantilla de texto con «Columna/Valor/texto/valor» y no hay forma visible de agregar filas o columnas. | 42 |
| US-14 | S3 | T12 | Tooltip en todos los íconos → la chincheta (siempre encima) no tiene tooltip. | 56 |
| US-15 | S3 | T11 | Un solo control de modo → en la vista dividida conviven los botones Leer/Editar/Comparar generales y los de cada panel. El panel nuevo heredó «Comparar» y quedó apretado. | 53, 54 |
| US-16 | S3 | T16 | Abrir una nota desde un enlace para leerla → se abrió en Editar (el último modo usado), con el código a la vista. | 67 |
| US-17 | S3 | T8 | Doble clic para seleccionar una palabra y darle formato → no la seleccionó; «H» insertó `==texto==` en medio de la palabra y la partió. (El doble clic pudo verse afectado por la herramienta de prueba.) | 32 |
| US-18 | S4 | Todas | Ventana limpia → a 1280×760 hay una franja negra fija abajo. La barra de estado (palabras, guardado) solo se vio con la ventana chica. | 00, 55, 59 |
| US-19 | S4 | T1, T7 | Que la pestaña vacía «sin título.md» se cerrara al abrir un archivo → queda abierta. Además siguió marcada con «*» después de borrar todo lo escrito. | 02, 26 |
| US-20 | S4 | T2, T6 | Que el tooltip desaparezca al mover el puntero → el de «Guardar» y el de «Abrir carpeta» quedaron visibles. El de «Notas de la carpeta» se superpuso a las pestañas del panel. | 06, 17, 18 |
| US-21 | S4 | T9, T14 | Contadores correctos → «1–7 de 7» con 8 opciones visibles; «1 resultados». | 43, 61 |
| US-22 | S4 | T16 | Distinguir un enlace roto → se ve igual que los válidos; solo se sabe al hacer clic. | 03, 72 |
| US-23 | S4 | T3 | Copiar «el comando» → el bloque tiene dos comandos y el botón copia ambos. | 07 |
| — | (no confirmado) | T8, T16 | Al escribir justo después de un clic aparecieron letras de más («resumenmen», «resumemen#», «recuperp»). No se reprodujo al reintentar y puede deberse a la herramienta de prueba. | m-31, 34, 68 |

---

## 6. SUS detallado (respuestas de Lucía, de 1 a 5)

| # | Afirmación | Resp. | Contribución |
|---|---|---|---|
| 1 | Creo que me gustaría usar este sistema con frecuencia. | 3 | 2 |
| 2 | Encontré el sistema innecesariamente complejo. | 4 | 1 |
| 3 | Pensé que el sistema era fácil de usar. | 2 | 1 |
| 4 | Creo que necesitaría ayuda de una persona técnica para usarlo. | 4 | 1 |
| 5 | Las funciones del sistema estaban bien integradas. | 3 | 2 |
| 6 | Había demasiada inconsistencia en el sistema. | 3 | 2 |
| 7 | La mayoría de la gente aprendería a usarlo muy rápido. | 2 | 1 |
| 8 | Encontré el sistema muy engorroso de usar. | 3 | 2 |
| 9 | Me sentí muy segura usando el sistema. | 2 | 1 |
| 10 | Necesité aprender muchas cosas antes de poder arrancar. | 4 | 1 |
| | **Suma: 14 × 2,5** | | **SUS = 35** |

Interpretación: 35 está por debajo de 50, en la franja «pobre». Leer apuntes le resultó agradable. Escribir, buscar y encontrar funciones le costó.

---

## 7. Gustos, frustraciones y lo que no encontró

**Lo que más le gustó**
- La vista de lectura: los títulos grandes, las tablas y las casillas que se tildan con un clic.
- Las pestañas, que recuerdan dónde estaba en cada apunte.
- «Comparar»: ver el código y el resultado lado a lado le hizo entender el formato.
- El botón «Copiar» en el código, con su confirmación «Copiado».
- El diálogo de «¿Guardar los cambios?», claro como en Word.
- «Estudio» (marcar entendido o dudoso, preguntas) y plegar secciones.
- La chincheta para dejar la ventana encima y el modo claro.

**Top 5 de frustraciones**
1. No poder escribir en el buscador: la búsqueda en todos los apuntes no funcionó.
2. Los símbolos raros al editar (`#`, `**`, `|`) y que la «H» no haga un título.
3. Hacer algo y que el resultado quede distinto sin enterarse: la negrita dentro de la lista y el ✓ que desaparece.
4. El índice de 18 en 18, sin rueda y sin barra, y tener que pulsar la flecha 30 veces.
5. Funciones escondidas en «Más acciones» (PDF, tema) y avisos con palabras técnicas.

**Lo que no encontró nunca**
- Cómo buscar de verdad en todos sus apuntes.
- Emojis de verdad (solo flechas y símbolos).
- Dónde restaurar la «recuperación local» que prometía el diálogo.
- Cómo agregar filas o columnas a una tabla sin escribir rayitas.
- Un botón visible de exportar o imprimir y uno de modo claro u oscuro.

**¿La recomendaría?** «Para leer apuntes que ya están hechos, sí: se ven re lindos y las pestañas son cómodas. Para escribir desde cero o buscar algo, todavía no. A mi compañera que programa le va a encantar, a mí me costó.»

---

## 8. Cita final de Lucía

> «Leerla es un placer, pero para escribir tuve que aprender un idioma nuevo de numerales y asteriscos. Y cuando quise buscar algo en todos mis apuntes, escribía y no pasaba nada.»
