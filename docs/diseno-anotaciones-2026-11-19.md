# Diseño de anotaciones de estudio (19 de noviembre de 2026)

Propuesta para cerrar la decisión **14.0** de la fase 14 («Herramientas de
estudio») de `docs/plan-ejecucion-2026-11-14.md`. Cubre subrayados de colores,
marcas de libro y, a futuro, imágenes ancladas y envío de la selección a una IA
o a Google. Este documento no cambia código.

## 1. Pedido, en una línea por requisito

| # | Requisito | Cómo se cumple (resumen) |
|---|---|---|
| R1 | Subrayar con colores a elección en modo Leer | Registro `subrayado` con color de una paleta cerrada |
| R2 | Marcas de libro (barra, cinta…) con texto opcional; a futuro, imágenes | Registro `marca` con `forma` y `nota`; campo `imagen` reservado |
| R3 | Una sola solución de almacenamiento para todo | Un único bloque de anotaciones al final del `.md` |
| R4 | Viajan en el mismo `.md`, sin archivo lateral obligatorio | El bloque es parte del archivo |
| R5 | Fáciles de crear y de borrar | Borrar todo = cortar un sufijo; el cuerpo queda idéntico byte a byte |
| R6 | No chocan con otros lectores ni contaminan el significado | Comentario HTML: invisible en GitHub, Obsidian (lectura), VS Code, Pandoc |
| R7 | Legible sin confundir a una IA | Va al final, se presenta a sí mismo en una frase y no toca el cuerpo |
| R8 | Estable y estándar | CommonMark (bloque HTML tipo 2) + JSON por línea (RFC 8259) + selectores W3C |

## 2. Lo que el código hace hoy (y condiciona el diseño)

- **Parser.** `markdown_options()` en `src/main.rs` activa `table`,
  `strikethrough`, `highlight` (`==texto==`), `autolink`, `tasklist` y
  `footnotes`. **No** activa `front_matter_delimiter`: un front matter YAML se
  lee como regla horizontal más un encabezado setext.
- **HTML.** `native_html` acepta solo `<br>`, `<kbd>`, `<mark>`, `<sub>`, `<sup>`
  exactos, sin atributos (`docs/security.md`, allowlist). Cualquier otro HTML en
  línea, incluidos `<span style=…>` y `<!-- -->` en línea, se dibuja como fuente
  monoespaciada inerte. Un `NodeValue::HtmlBlock` (por ejemplo, un comentario
  `<!-- … -->` en su propio bloque) se dibuja hoy **como líneas de código**:
  sin cambios, el bloque de anotaciones se vería al pie como código. La tarea
  14.3 lo reconoce y lo oculta.
- **`==texto==` y `<mark>`** ya existen con significado propio: son el
  resaltado **del autor**, y Obsidian usa `==` igual. Reutilizarlos para
  anotaciones personales mezclaría la voz del autor con la de quien estudia.
- **Modelo.** `Block` conserva `source: SourceRange` (offsets en bytes del
  archivo), `TextBuffer` sobre ropey, `BlockCursor` en el texto dibujado. Ya
  hay correspondencia dibujo↔fuente para selección y copia.
- **Guardado** atómico (`src/files.rs`) con detección de cambios externos sobre
  el archivo entero, y **recuperación** versionada (`src/recovery.rs`).
- **Política.** Sin dependencias nuevas salvo necesidad fuerte (ADR-14, ADR-38,
  `docs/budget.md`). No hay `serde_json`: el formato se elige para que un
  lector y escritor propios, pequeños y acotados, alcancen.

## 3. Familias comparadas

Ejemplo base: subrayar «mitocondria produce ATP» en amarillo y poner una marca
de libro azul en «## Fotosíntesis».

### Cómo queda el archivo en cada familia

| Familia | Texto en el `.md` |
|---|---|
| (a) Marca en línea | `La ==mitocondria produce ATP== mediante…` · `<mark>mitocondria…</mark>` · `<span class="hl-amarillo">…</span>` |
| (b) Comentario HTML en el cuerpo | `La <!--a1{-->mitocondria produce ATP<!--}a1--> mediante…` |
| (c1) Bloque final, comentario HTML | cuerpo intacto + al final `<!-- visor-md:anotaciones v1 … -->` |
| (c2) Bloque final, referencias ocultas | cuerpo intacto + al final `[//]: # ("{…json…}")`, una por línea |
| (d) Front matter YAML | al principio: `---` / `anotaciones:` / `  - cita: …` / `---` |
| (e) Notas al pie | `La mitocondria produce ATP[^a1] …` + `[^a1]: subrayado amarillo` |
| (f) Ancla mínima + bloque final | `La <!--a1-->mitocondria…` en el cuerpo + detalle al final |
| (g) Archivo lateral | `apuntes.md` intacto + `apuntes.md.anotaciones` al lado |

### Cómo se ve en otros lectores

| Familia | GitHub | Obsidian | VS Code (vista previa) | Texto plano leído por una IA |
|---|---|---|---|---|
| (a) `==` | Se ve `==…==` literal (GFM no lo soporta) | Resaltado, pero como si fuera del autor | `==` literal | Cree que el autor enfatizó; cambia el significado |
| (a) `<mark>` | Resaltado | Resaltado | Resaltado | Igual que `==`; además ensucia cada frase |
| (a) `<span>` | Atributo `class` descartado, se ve texto normal | Texto normal | Texto normal | Etiquetas en medio de las frases |
| (b) Comentarios en el cuerpo | Invisible | Invisible en lectura, visible al editar | Invisible | Ruido dentro de las frases; parte palabras para búsquedas y citas |
| (c1) Bloque final HTML | Invisible (visible solo en «Raw» y en diffs) | Invisible en lectura; texto atenuado al editar | Invisible | Lo ve al final, separado, con una frase que dice qué es |
| (c2) Referencias ocultas | Invisible | Invisible | Invisible | Líneas crípticas `[//]: #`; truco poco conocido |
| (d) Front matter | Tabla visible arriba | Choca con «Propiedades» | Bloque visible o ignorado según extensión | Lo primero que lee: contamina el contexto inicial |
| (e) Notas al pie | Notas visibles numeradas | Notas visibles | Visibles | Las toma como notas del autor |
| (f) Ancla + bloque | Invisible | Invisible en lectura | Invisible | Ruido en las frases (menor que b) |
| (g) Lateral | Nada | Nada | Nada | Nada; pero no viaja con el archivo |

### Evaluación

Escala: ✔ bien · ~ aceptable · ✘ mal.

| Criterio | a | b | c1 | c2 | d | e | f | g |
|---|---|---|---|---|---|---|---|---|
| Invisible en otros lectores | ✘ | ✔ | ✔ | ✔ | ✘ | ✘ | ✔ | ✔ |
| No contamina a una IA | ✘ | ✘ | ✔ | ~ | ✘ | ✘ | ~ | ✔ |
| No rompe Markdown (listas, tablas, código, citas) | ~ | ✘ | ✔ | ~ | ✘ | ~ | ✘ | ✔ |
| Diff limpio en git | ✘ | ✘ | ✔ | ✔ | ~ | ✘ | ~ | ✔ |
| Sin choque con Obsidian | ✘ | ✔ | ✔ | ✔ | ✘ | ~ | ✔ | ✔ |
| Robusto ante ediciones externas | ~ | ✘ | ✔ | ✔ | ✔ | ~ | ✘ | ~ |
| Borrar todo de golpe | ✘ | ✘ | ✔ | ✔ | ~ | ✘ | ✘ | ✔ |
| Viaja en el documento | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | ✘ |
| Admite marcas, notas e imágenes en un solo esquema | ✘ | ~ | ✔ | ~ | ✔ | ✘ | ✔ | ✔ |

Razones de los ✘ más importantes:

- **(a)** Cambia el contenido: el resaltado pasa a parecer del autor, `==` no
  existe en GitHub, y no lleva color, forma ni nota. Un subrayado que cruza dos
  párrafos o una celda de tabla obliga a partir y reescribir el Markdown.
- **(b) y (f)** Un comentario dentro de un bloque de código o de un código en
  línea deja de ser comentario y se vuelve texto visible; dentro de una celda
  de tabla o de un enlace puede romper el análisis; parte palabras
  («mito<!--a1-->condria»), lo que estropea búsquedas en otros programas, citas
  de una IA y diffs. Cualquier editor externo puede mover o borrar un ancla.
- **(c2)** Invisible y sin cambios en el parser, pero frágil: una definición no
  puede interrumpir un párrafo (sin línea en blanco previa se vuelve texto
  visible), no admite líneas en blanco, exige escapar `(` `)` `\` y depende de
  una rareza de CommonMark que lectores no estándar muestran.
- **(d)** Con el parser actual se ve como regla + encabezado; en GitHub como
  tabla; en Obsidian pisa las propiedades del usuario; y es lo primero que lee
  una IA.
- **(g)** Correcto pero incumple R4: al mover, enviar o sincronizar el `.md`
  las anotaciones se pierden. Queda como **exportación opcional**, no como
  almacenamiento.

## 4. Recomendación: cuerpo intacto + un bloque final en comentario HTML

**Elegida: (c1).** Cero anclas en el cuerpo. Todo vive en un único comentario
HTML al final del archivo, con una línea JSON por anotación y anclaje por cita
textual (W3C `TextQuoteSelector`) más una posición orientativa
(`TextPositionSelector`).

Por qué es la ingeniosa y no solo la segura:

1. **El cuerpo no se toca nunca.** Quitar todas las anotaciones es cortar un
   sufijo; el archivo vuelve a ser el original byte a byte (se prueba).
2. **Invisible donde importa, explícito donde se lee crudo.** Los
   renderizadores ocultan comentarios; quien lee el texto plano (persona o IA)
   encuentra al final una frase que dice qué es y que no forma parte del texto.
3. **Es estándar sin dependencias.** Bloque HTML tipo 2 de CommonMark + JSON
   Lines + vocabulario de la Web Annotation del W3C. Otro programa puede leerlo
   sin conocer Visor MD.
4. **Imposible de cerrar por accidente.** Dentro del JSON, `<` y `>` se
   escriben siempre como `<` y `>` (JSON válido): ninguna nota o cita
   puede producir `-->` ni `<!--`.
5. **Diff de una línea por cambio.** Orden y claves deterministas: agregar un
   subrayado es una línea `+` en git.
6. **Se ancla sin hash ni índice.** La posición guardada se verifica contra la
   cita; si coincide, el anclaje es O(1). Solo si el texto cambió se busca.

### Alternativas descartadas (resumen)

| Alternativa | Motivo |
|---|---|
| `==texto==` / `<mark>` | Cambia el significado (parece del autor); sin color ni nota; choca con Obsidian |
| `<span>` con clase | Ensucia cada frase; atributos fuera de la allowlist de seguridad |
| Comentarios o anclas en el cuerpo | Se vuelven visibles en código, rompen tablas, parten palabras, diffs ruidosos |
| `[//]: # (…)` | Frágil tras párrafos, escapes incómodos, truco poco reconocible |
| Front matter YAML | Visible con el parser actual y en GitHub; choca con Propiedades de Obsidian |
| Notas al pie | Visibles como contenido del autor |
| Archivo lateral | No viaja; queda solo como «Exportar anotaciones» futura |

## 5. Formato v1

### 5.1 Ejemplo completo

````markdown
# Biología celular

La mitocondria produce ATP mediante la respiración celular.

## Fotosíntesis

La luz se convierte en energía química en el cloroplasto.

<!-- visor-md:anotaciones v1
Anotaciones personales de estudio hechas con Visor MD (subrayados y marcas de libro). No forman parte del texto del documento; cada línea cita el fragmento al que se refiere.
{"id":"a3","tipo":"leyenda","color":"amarillo","nota":"idea clave"}
{"id":"a1","tipo":"subrayado","color":"amarillo","pos":23,"antes":"# Biología celular\n\nLa ","cita":"mitocondria produce ATP","despues":" mediante la respiraci"}
{"id":"a2","tipo":"marca","color":"azul","forma":"cinta","pos":81,"antes":"ante la respiración celular.\n\n","cita":"## Fotosíntesis","despues":"\n\nLa luz se convierte","nota":"repasar antes del parcial"}
-->
````

### 5.2 Sintaxis

```
archivo     = cuerpo separador bloque
separador   = [EOL] EOL                 ; EOL si el cuerpo no termina en salto, más una línea en blanco
bloque      = "<!-- visor-md:anotaciones v" versión EOL
              descripción EOL
              *( registro EOL )
              "-->" [EOL]
versión     = 1*DIGIT                   ; hoy "1"
descripción = la frase fija del ejemplo (la app la reescribe; al leer se ignora)
registro    = objeto JSON en una sola línea (RFC 8259), sin espacios fuera de cadenas
```

Reglas de reconocimiento (todas obligatorias):

1. La línea de apertura es exactamente `<!-- visor-md:anotaciones v<N>` en la
   columna 0, precedida por línea en blanco o inicio de archivo.
2. Después de la línea `-->` solo puede haber espacios y saltos de línea hasta
   el fin del archivo. Si hay texto después, **no** es un bloque de anotaciones
   (se trata como contenido normal).
3. Con el render enriquecido disponible, comrak debe ver ese tramo como un
   `HtmlBlock` de nivel superior (no dentro de lista, cita ni código). Así un
   documento que *explica* este formato dentro de un bloque de código no se
   confunde.
4. Si hay varios bloques candidatos solo cuenta el último; los demás son
   contenido normal (aviso «hay un bloque de anotaciones repetido»).

### 5.3 Campos

| Clave | Tipo | Obligatoria | Valores |
|---|---|---|---|
| `id` | cadena | sí | `a` + entero decimal; nuevo = máximo + 1; no se reutiliza |
| `tipo` | cadena | sí | `subrayado` · `marca` · `leyenda` |
| `color` | cadena | sí | `amarillo` · `verde` · `azul` · `rosa` · `violeta` |
| `forma` | cadena | solo `marca` | `barra` (por defecto) · `cinta` · `punto` · `bandera` |
| `pos` | entero | `subrayado`, `marca` | Inicio orientativo, en caracteres Unicode del cuerpo normalizado a LF |
| `antes` | cadena | `subrayado`, `marca` | Hasta 32 caracteres previos a la cita (puede ser vacía) |
| `cita` | cadena | `subrayado`, `marca` | Texto exacto de la fuente del cuerpo, normalizado a LF |
| `despues` | cadena | `subrayado`, `marca` | Hasta 32 caracteres siguientes |
| `nota` | cadena | no | Texto libre de la persona; en `leyenda`, el significado del color |
| `imagen` | cadena | reservada | Ruta relativa futura (sección 9); v1 la conserva sin usarla |

- Una **marca** ancla un punto: su `cita` son los primeros caracteres (hasta
  48) de la línea donde está, desde ese punto.
- **Claves desconocidas** se conservan con su valor original, después de las
  conocidas y en el orden en que llegaron.
- `cita` es fuente Markdown, no texto dibujado: es la autoridad que se guarda y
  la que otro programa puede buscar con un simple `find`. La app traduce
  fuente↔dibujo con los rangos que ya mantiene.

### 5.4 Codificación, escape y saltos de línea

- Archivo en UTF-8, como hoy. Una BOM inicial pertenece al cuerpo.
- Cadenas JSON con escapes estándar; además la app **siempre** escribe `<` como
  `<` y `>` como `>`, y los controles como `\n`, `\t` o `\uXXXX`.
  Al leer se acepta cualquier JSON válido; si una línea contiene `-->` literal
  no puede existir, porque el bloque ya habría terminado allí.
- El bloque se escribe con el salto de línea dominante del documento (CRLF o LF,
  `document_line_ending`). Dentro de `cita`, `antes` y `despues` los saltos se
  guardan siempre como `\n`, y `pos` cuenta sobre el cuerpo con CRLF→LF. Así un
  cambio de CRLF a LF hecho por git o por otro editor no mueve nada.

### 5.5 Orden determinista

1. Registros `leyenda`, por color en el orden de la paleta.
2. Resto por `pos` ascendente, luego `tipo` (`marca` antes que `subrayado`),
   luego número de `id`.
3. Claves en el orden de la tabla 5.3, después las desconocidas.
4. Líneas no entendidas (5.7), al final, en su orden original.

Volver a guardar sin cambios produce exactamente los mismos bytes.

### 5.6 Límites

| Límite | Valor | Al superarlo |
|---|---|---|
| Tamaño del bloque | 1 MiB | Bloque inerte: se conserva sin tocar, no se interpreta |
| Registros | 5 000 | Igual que arriba |
| Línea de registro | 16 KiB | Esa línea pasa a «no entendida» |
| `cita` | 4 096 bytes | La app no crea el subrayado: «subrayá en partes más cortas» |
| `nota` | 2 000 caracteres | El cuadro de texto no acepta más |
| `antes` / `despues` | 32 caracteres | Se recortan al escribir |
| Profundidad JSON | 1 (objeto plano de cadenas y enteros) | Línea «no entendida» |

El lector es propio (sin dependencia nueva), recorre bytes una vez, no
recursa, y respeta `src/limits.rs`.

### 5.7 Contenido desconocido o dañado (modo seguro)

| Situación | Qué hace la app |
|---|---|
| Versión mayor que la conocida (`v2`) | Conserva el bloque byte a byte, no muestra anotaciones, desactiva crear/quitar y avisa «anotaciones de una versión más nueva» |
| Línea que no es JSON válido (por ejemplo, marcadores de conflicto de git) | La conserva literal, al final; aviso «N líneas de anotaciones no se entienden; se conservan» |
| Registro válido con `tipo`, `color` o `forma` desconocidos | Lo conserva tal cual; no lo dibuja; lo cuenta en el aviso |
| Bloque que excede límites | Inerte, conservado, sin interpretar |
| Documento en modo seguro (render degradado) | Las anotaciones no se dibujan; el bloque se conserva y se reescribe idéntico |
| El cuerpo termina dentro de un bloque de código o HTML sin cerrar (el bloque quedaría tragado) | No se guardan anotaciones en el archivo; aviso que explica el motivo; el cuerpo no se corrige solo |

Nada del bloque se ejecuta, abre rutas, consulta red ni cambia preferencias:
son datos, igual que el resto del documento (`docs/security.md`).

## 6. Anclaje y re-anclaje

Todo opera sobre el **cuerpo normalizado** (CRLF→LF), en caracteres Unicode.

### Al abrir o al detectar un cambio externo

Para cada registro con cita, en orden y sin estado compartido:

1. **Camino rápido.** Si el texto en `pos` es igual a `cita` → anclado. O(|cita|).
2. **Búsqueda exacta.** Todas las apariciones de `cita` en el cuerpo (tope:
   1 000). Si hay una sola → anclado.
3. **Desempate determinista** si hay varias: mayor coincidencia de `antes`
   (caracteres comparados hacia atrás) + `despues` (hacia adelante); empate →
   menor distancia a `pos`; empate → menor offset.
4. **Espacios.** Si no hubo coincidencias, repetir 2–3 comparando con
   secuencias de espacios en blanco colapsadas (cubre re-ajustes de línea de
   otro editor).
5. Si nada coincide → **huérfana**.

No se usa distancia de edición ni coincidencia difusa en v1: el resultado
depende solo de cuerpo y registro, y cuesta O(n) por cita en el peor caso.

### Huérfanas

- Nunca se borran solas. Se guardan con su última `cita` y `pos`.
- Se listan en un panel «Anotaciones sin lugar» (Estudio), con la cita y la
  nota, y tres acciones: **Reubicar** (selección actual pasa a ser la nueva
  cita), **Copiar cita**, **Quitar**.
- El índice muestra «N sin lugar» al pie si hay alguna marca huérfana.

### Mientras se edita dentro de la app

Las anclas viven como rangos del `TextBuffer` y se desplazan con cada edición
(como un cursor): antes del cambio no se mueven, después se corren, si la
edición cae dentro el rango se ajusta, y si el texto anclado desaparece entero
la anotación queda huérfana. `cita`, `antes` y `despues` se recalculan del
texto actual **al guardar**. Deshacer aplica el desplazamiento inverso y luego
re-ancla solo las huérfanas (paso 2), así un borrado deshecho recupera su
subrayado.

## 7. Paleta y formas

Nombres de color fijos en el archivo; el tono real lo decide el tema. Valores
de partida para `src/theme.rs`, a confirmar por prueba de contraste:

| Color | Día (fondo, texto `#2B251C`) | Noche (fondo, texto `#ECE8DE`) |
|---|---|---|
| `amarillo` (predeterminado) | `#F3E08A` | `#5A4A12` |
| `verde` | `#CFE3B8` | `#2E5238` |
| `azul` | `#C9DDF0` | `#22405E` |
| `rosa` | `#F2CFD8` | `#5C2A3A` |
| `violeta` | `#DDD3F0` | `#3F3360` |

- Texto sobre cada fondo ≥ 4,5:1 (WCAG AA), en ambos temas.
- Los cinco deben distinguirse entre sí y del `==resaltado==` del autor (hoy
  verde `mark`). Para que nunca se confundan, el subrayado personal se dibuja
  como trazo de resaltador con esquinas redondeadas y el del autor sigue igual;
  además **Vista → Ocultar mis anotaciones** muestra el documento original.
- Formas de marca (lista cerrada): `barra` (franja vertical en el margen, junto
  al bloque), `cinta` (lengüeta que asoma del borde), `punto`, `bandera`. Todas
  en el margen izquierdo, fuera de la medida del texto, como hoy las viñetas.

## 8. Relación con lo existente

| Función | Comportamiento |
|---|---|
| **Leer** | Subrayados dibujados sobre los tramos; marcas en el margen; clic en una marca con nota muestra la nota. El bloque no se dibuja |
| **Editar** | El bloque aparece **siempre contraído** en una sola línea al final («▸ Anotaciones de estudio · N»); clic o Enter lo abre como texto de solo lectura y se contrae al salir de Editar. Contraído no alarga el documento: la barra de desplazamiento refleja el largo del cuerpo. Escribir nunca puede romperlo. Los subrayados se insinúan con un fondo tenue (decisión de la persona, 19 de noviembre) |
| **Comparar** | Compara cuerpos. Si las anotaciones difieren, una línea de resumen («anotaciones: 3 distintas») y nada más |
| **Guardar** | Archivo = cuerpo + separador + bloque, en el mismo reemplazo atómico. Crear o quitar anotaciones marca la pestaña con «•» como cualquier cambio. Si no hay anotaciones, no se escribe bloque |
| **Cambios externos** | La detección sigue sobre el archivo entero. Al recargar se separa cuerpo/bloque y se re-ancla (sección 6) |
| **Recuperación** | La instantánea guarda el archivo serializado completo; cuerpo y anotaciones vuelven juntos, sin cambiar el formato de recuperación |
| **Archivo de solo lectura** | Anotaciones en memoria; aviso; «Guardar copia» las lleva a otro archivo |
| **Buscar** | Solo el cuerpo. Las notas se buscan desde el panel de anotaciones |
| **Copiar** (`Ctrl+C`, `Ctrl+Shift+C`) | Texto visible o fuente de bloques del cuerpo; nunca el bloque |
| **Índice** | Sin filas nuevas: cada título lleva a la derecha puntos de color de 6 px de las marcas de su sección, uno por color, como máximo tres y «+N»; espacio reservado fijo, así muchas marcas no cambian la fila. Ayuda con la lista de marcas al pasar el puntero; clic lleva a la primera de ese color. Los subrayados no aparecen (decisión de la persona, 19 de noviembre) |
| **Exportar PDF (D-28)** | La capa de texto no cambia ni incluye el bloque. Los subrayados se dibujan como fondos y las marcas en el margen, según la decisión P2 |
| **Preparar fragmentos para IA** | Usa solo el cuerpo: `ai_fragment_document` recibe el cuerpo, nunca el bloque. Una casilla «incluir mis subrayados» (apagada) los agrega como lista aparte, explícita |
| **Quitar todas** | Estudio → «Quitar todas las anotaciones de este documento…», con confirmación que dice cuántas. Se puede deshacer hasta cerrar. Al guardar, el cuerpo queda idéntico al original |
| **Guardar copia sin anotaciones** | Escribe solo el cuerpo, para compartir el `.md` sin las notas personales |
| **Abrirlo en versiones anteriores** | Verían el bloque al pie como código: inofensivo y legible |

## 9. A futuro

- **Imágenes ancladas.** Campo `imagen` con ruta relativa al documento. Sigue
  las reglas actuales: VFS, raíz concedida, PNG con consentimiento, nunca carga
  automática. No se incrustan imágenes en base64 (inflaría el `.md`).
- **Enviar a IA o a Google (antes 14.3).** La app sigue sin conectarse: el
  gesto copia la selección visible y abre el navegador del sistema en el
  servicio elegido; para Google, la búsqueda lleva el texto en la dirección,
  así que se avisa la primera vez. Nunca se envían anotaciones ni notas salvo
  casilla explícita. Va en su propia tarea, después de 14.9.
- **Exportar a otros formatos de anotación**: archivo lateral JSON-LD con el
  modelo W3C, o «llevar subrayados al texto como `==…==`» para Obsidian. Solo
  exportaciones; el almacenamiento sigue siendo el bloque.

## 10. Plan de tareas

Reemplaza la numeración 14.1–14.3 del plan: subrayar pasa a 14.5, marcas a 14.6
y el envío a 14.11. Cada tarea deja `cargo test` y `cargo clippy` en verde.

- **14.0** Aprobar este documento y registrar un ADR («Anotaciones en un bloque
  final de comentario HTML»).
- **14.1 Formato puro** (`src/anotaciones.rs`, sin interfaz, sin dependencias).
  Modelo `Anotacion`, `Bloque`, lector y escritor.
  Acepta: ida y vuelta byte a byte; orden determinista; límites; contenido
  desconocido conservado.
  Pruebas: `bloque_vacio_no_se_escribe`, `ida_y_vuelta_conserva_bytes`,
  `escritura_ordena_por_posicion_tipo_e_id`, `escapa_menor_y_mayor_en_cadenas`,
  `ninguna_nota_puede_cerrar_el_comentario`, `version_mayor_se_conserva_inerte`,
  `linea_invalida_se_conserva_al_final`, `claves_desconocidas_se_conservan`,
  `limites_vuelven_inerte_el_bloque`, `crlf_y_lf_producen_mismas_posiciones`.
- **14.2 Anclaje puro** (mismo módulo).
  Acepta: algoritmo de la sección 6, determinista, sin estado.
  Pruebas: `camino_rapido_ancla_en_su_posicion`,
  `cita_unica_se_ancla_tras_insertar_texto_antes`,
  `cita_repetida_desempata_por_contexto`, `empate_total_elige_la_mas_cercana`,
  `espacios_colapsados_reanclan_tras_reajuste_de_linea`,
  `texto_borrado_deja_huerfana_sin_eliminarla`,
  `reanclar_dos_veces_da_el_mismo_resultado`, `tope_de_apariciones_se_respeta`.
- **14.3 Separar cuerpo y bloque** al abrir, guardar, recargar y recuperar;
  ocultar el bloque en Leer y mostrarlo contraído en una línea en Editar
  (ver la sección 8 y el plan).
  Acepta: el editor no muestra el bloque; guardar sin cambios no altera bytes.
  Pruebas: `cuerpo_y_bloque_se_separan_al_abrir`,
  `bloque_dentro_de_codigo_no_se_reconoce`,
  `texto_despues_del_bloque_lo_invalida`,
  `agregar_y_quitar_deja_el_archivo_identico` (con y sin salto final, CRLF y
  BOM), `cuerpo_con_codigo_sin_cerrar_rechaza_guardar_anotaciones`,
  `recuperacion_restaura_cuerpo_y_anotaciones`,
  `buscar_y_copiar_ignoran_el_bloque`.
- **14.4 Paleta** en `src/theme.rs`.
  Prueba: `paleta_de_estudio_conserva_contraste_en_ambos_temas`,
  `colores_de_estudio_se_distinguen_entre_si_y_del_resaltado`.
- **14.5 Subrayar** desde la selección en Leer: menú Estudio, menú contextual,
  atajo, elegir color, cambiar color, quitar uno.
  Prueba: `subrayado_cruza_bloques_y_se_dibuja_por_tramos`,
  `subrayar_marca_el_documento_sin_guardar`,
  `ocultar_mis_anotaciones_muestra_el_original`. QA manual con capturas.
- **14.6 Marcas de libro**: margen, forma, color, nota; en el índice como puntos
  de color sutiles junto al título de su sección (ver la sección 8 y el plan).
  Prueba: `marca_aparece_en_el_indice_entre_sus_titulos`,
  `marca_sin_nota_usa_su_cita_en_el_indice`.
- **14.7 Editar**: desplazar anclas con cada edición y con deshacer; panel
  «Anotaciones sin lugar» con Reubicar.
  Prueba: `editar_antes_del_subrayado_lo_desplaza`,
  `borrar_el_texto_anclado_lo_deja_huerfano`,
  `deshacer_el_borrado_recupera_el_subrayado`.
- **14.8 Integraciones**: fragmentos para IA sin anotaciones (casilla
  opcional), PDF según P2, «Quitar todas», «Guardar copia sin anotaciones»,
  «Ver anotaciones como texto», resumen en Comparar.
  Prueba: `fragmentos_para_ia_excluyen_anotaciones_por_defecto`,
  `pdf_no_incluye_el_bloque_en_la_capa_de_texto`,
  `quitar_todas_se_puede_deshacer`, `copia_sin_anotaciones_es_el_cuerpo`.
- **14.9** Documentar en `docs/features.md`, `docs/security.md` (el bloque como
  dato inerte) y `docs/test-matrix.md`; QA manual en Leer, Editar y con el
  archivo abierto en otro editor.
- **14.10** (futuro) Imágenes ancladas.
- **14.11** (futuro) Enviar la selección a una IA o a Google.

## 11. Decisiones que le tocan a la persona

| # | Decisión | Recomendada (primera) | Alternativa |
|---|---|---|---|
| P1 | Cómo se guardan las anotaciones | Como cualquier cambio: «•» y Ctrl+S; la recuperación cubre cierres inesperados | Guardado automático al crear cada una (toca el archivo sin pedirlo) |
| P2 | Exportar a PDF | Incluir subrayados y marcas dibujados, con una casilla para quitarlos | PDF siempre limpio |
| P3 | Compartir el `.md` | Las anotaciones viajan siempre; existe «Guardar copia sin anotaciones» | Preguntar al guardar si un archivo tiene notas |
| P4 | Envío a IA o Google (14.11) | Abrir el navegador con la selección visible, aviso la primera vez, nunca notas salvo casilla | Solo copiar al portapapeles, sin abrir nada |

## Lo más importante

- Se recomienda un **único comentario HTML al final del `.md`** con una línea
  JSON por anotación; el cuerpo del documento nunca se modifica.
- Es invisible en GitHub, Obsidian, VS Code y Pandoc, y una IA lo encuentra al
  final con una frase que dice que no es parte del texto.
- El anclaje usa cita + contexto + posición (W3C); si el texto desaparece, la
  anotación queda **huérfana y se conserva**, nunca se borra sola.
- «Quitar todas» devuelve el archivo idéntico byte a byte; lo desconocido o
  dañado se conserva sin tocar.
- Se empieza por el módulo puro `src/anotaciones.rs` (14.1–14.2); la persona
  decide solo P1–P4.

## 12. Hallazgos del prototipo sobre un archivo real (19 de noviembre)

Se generó el bloque sobre una guía de estudio de 116 KB (36 subrayados y marcas
más casos borde deliberados) con un prototipo en Python y se leyó de vuelta: 26
de 26 casos correctos. Lo que cambia o se precisa en el diseño:

1. **Separadores de línea de Unicode.** U+2028, U+2029 y U+0085 son JSON válido
   sin escapar, pero muchos programas (`splitlines`, editores) parten la línea
   ahí y el registro quedaba dañado. El escritor debe escribirlos siempre como
   `\uXXXX` (igual que `<` y `>`), y el lector debe partir solo por `\n` o
   `\r\n`.
2. **Lector estricto sin HTML.** Un renderizador con el HTML apagado muestra el
   bloque como texto visible; con HTML permitido (VS Code, GitHub) no se ve. La
   frase fija del bloque cumple ahí su función de aviso.
3. **Tamaño.** 42 registros ocupan 11 KB (10 % del archivo); cada anotación
   cuesta unos 250 bytes por la cita y el contexto.
4. **Hoy la app dibuja el bloque como código** en la vista de lectura (es un
   `HtmlBlock`). La tarea que lo oculta en Leer es imprescindible antes de
   exponer la función.
5. **Casos borde probados:** cita repetida, emoji fuera del BMP, cita que cruza
   un salto de línea, dentro de tabla y de bloque de código, nota con `-->`,
   `<!--`, comillas, barra invertida y saltos, nota de 2 000 caracteres, clave y
   tipo desconocidos, línea dañada por un conflicto de git, versión `v2`, texto
   después del bloque, dos bloques, cuerpo vacío, sin salto final y con BOM,
   CRLF, inserción al principio, espacios duplicados y borrar el texto citado.
