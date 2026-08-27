# Registro de decisiones

Formato ADR liviano: cada decisión conserva contexto, opción elegida y motivo.
Un ADR aceptado no se reescribe para fingir que siempre estuvo actualizado. Una
decisión posterior lo reemplaza y este índice lo hace visible.

## Estado y reemplazos

| ADR | Estado | Nota |
| --- | --- | --- |
| 1 a 6 | Aceptados | Aplican con los límites actuales |
| 7 | Reemplazado por ADR-21 | Visor MD no incorpora IA propia |
| 8 | Reemplazado por ADR-25 | Windows y Linux en v2.0, macOS futuro |
| 9 | Reemplazado por ADR-20 | Sintaxis portable por defecto |
| 10 a 13 | Aceptados | Requieren implementación y evidencia |
| 14 | Parcialmente reemplazado por ADR-24 | Se evalúa código nativo por plataforma |
| 15 | Aceptado | Cache con presupuesto todavía pendiente |
| 16 | Aceptado con condición | Estimar y corregir progresivamente |
| 17 | Aceptado con gate | Reabrir si accesibilidad o UX lo exige |
| 18 | Reemplazado por ADR-26 | `STAT`, itálica y reproducción revisadas |
| 19 a 31 | Aceptados | Modelo, alcance, seguridad y recuperación |
| 32 | Aceptado | Contexto progresivo y verificación proporcional |
| 33 | Aceptado | Portapapeles de texto explícito y sin lectura |
| 34 | Aceptado | Apertura primaria limitada desde el mismo handle |

## ADR-1: Nativo, sin motor web

**Contexto.** La v1 usa WebView2. Pesa 30 MB en disco, arranca en 3-4 s y su
seguridad depende de sanitizar HTML activo correctamente para siempre.

**Decisión.** La v2 es nativa: no empaqueta ni usa ningún motor web para
renderizar el documento.

**Por qué.** El requisito de <7 MB descarta de plano cualquier cosa que
empaquete un Chromium (Electron: 80-150 MB). Apoyarse en el WebView2 del
sistema (como hace la v1, o como haría Tauri) evita ese peso, pero conserva el
motor de scripts como superficie de ataque permanente. La vía nativa elimina
esa superficie por construcción: no hay JavaScript que ejecutar porque no hay
intérprete. Es la lección central de Tinta.

**Costo aceptado.** Perder la fidelidad "gratis" de un navegador: Mermaid,
KaTeX y HTML arbitrario dejan de venir resueltos. Ver ADR-5 y `product.md`.

## ADR-2: Lenguaje: Rust

**Contexto.** Tinta usa C++. El pedido explícito fue no elegir C++ por inercia
y comparar Go, Lisp y otros.

**Decisión.** Rust.

**Por qué, comparado.**

| Lenguaje | A favor | En contra | Veredicto |
| --- | --- | --- | --- |
| **Rust** | Seguridad de memoria sin recolector de basura; ecosistema maduro de parseo y render de texto; binarios pequeños; sin pausas de GC | Curva de aprendizaje; tiempos de compilación | **Elegido** |
| C++ (Tinta) | El más chico posible; control total; MD4C ya existe | La seguridad de memoria sobre entrada no confiable es exactamente el terreno de los CVE de corrupción: un lector nativo en C++ contradice la tesis de seguridad del proyecto en la capa del lenguaje | Descartado |
| Go | Simple; binarios razonables | El recolector de basura mete jitter de latencia perceptible al hacer scroll de documentos grandes; los toolkits GUI (Fyne) pasan de 10 MB y no se sienten nativos; Gio es inmediato y arrastra el runtime + GC | Descartado |
| Zig | Aún más chico que Rust; control fino | Ecosistema inmaduro para GUI y texto; sin la red de seguridad de memoria de Rust | Descartado |
| Lisp (SBCL) | Expresividad; interactividad | No hay camino realista a un GUI nativo <7 MB: la imagen de SBCL sola ya rompe el presupuesto; tooling GUI marginal | Descartado |

**La razón que decide.** Todo el proyecto se vende como "seguro por
construcción". Elegir C++, donde un `.md` malformado puede provocar un
desbordamiento de búfer en el parser, como el CVE-2026-5525 de Notepad++ en su
manejo de rutas por arrastre, socavaría esa tesis en la capa más baja. Rust
elimina esa clase entera de fallos sin pagar el precio de latencia de un
recolector de basura. Es el único lenguaje que satisface a la vez "seguro" y
"liviano y fluido".

## ADR-3: La superficie del documento se dibuja a mano, no con un widget de texto

**Contexto.** Un documento Markdown renderizado no es una UI de widgets: es
texto reflowable con estilos inline mezclados, enlaces clicables, bloques de
código resaltados, selección que cruza párrafos. Los toolkits de widgets
(Slint, egui) están pensados para botones y formularios, no para esto.

**Decisión.** La vista del documento se construye sobre una pila de **layout de
texto + dibujo 2D** (candidatos en Rust: `parley` para layout, `swash` para
glifos, `tiny-skia` para dibujo por software). El "chrome" de la app (pestañas,
barra lateral, menús, diálogos) sí puede usar un toolkit liviano.

**Por qué.** Es lo que hace un lector serio y lo que evita pelear contra las
suposiciones de un framework de widgets. `tiny-skia` (dibujo por software)
sobre Skia completo mantiene el presupuesto de tamaño; ver `budget.md`.

**Riesgo principal, declarado.** Esta es la parte más grande y más incierta del
proyecto. Se valida con un prototipo antes que nada (ver `roadmap.md`, Fase 0).

## ADR-4: Un solo punto de sanitización, formalizado

**Contexto.** La v1 ya tiene un único `clean()` con DOMPurify al final del
pipeline. ByteMD demuestra que ese patrón (árbol sintáctico tipado, un solo
saneo al final) es el correcto. EasyMDE demuestra el error opuesto: saneo
opcional, apagado por defecto.

**Decisión.** El parser (candidato: `comrak`, CommonMark + GFM en Rust puro)
produce un árbol tipado. Ningún nodo llega a la superficie de dibujo sin pasar
por un único validador. No existe forma de apagarlo.

**Por qué.** En un renderizador nativo la "sanitización" cambia de forma: no
hay HTML que limpiar, hay una **allowlist de nodos que el renderizador sabe
dibujar**. Un nodo HTML crudo fuera de esa lista no se ejecuta, no hay dónde,
se muestra como texto inerte o se descarta. Es más fuerte que sanitizar: lo
desconocido no tiene un motor donde correr.

## ADR-5: Mermaid y matemática: fuera del núcleo de la v2.0

**Contexto.** Reimplementar el layout de 22 tipos de diagrama Mermaid
nativamente es, según se estableció en la exploración, el ítem más caro de
todo el proyecto, más que el parser. La matemática nativa (equivalente a
KaTeX) es igual de pesada y sin una librería madura reutilizable clara.

**Decisión.** La v2.0 **no** renderiza Mermaid ni matemática de forma nativa.
Muestra la fuente del diagrama/fórmula en un bloque con estilo, igual que hace
la v1 cuando Mermaid no está disponible. El render real queda como componente
**opcional, de descarga separada y claramente marcado**, para una versión
posterior, y solo si se encuentra una vía 100% local que no rompa el
presupuesto ni la política de red.

**Por qué.** Meter Mermaid al núcleo rompe el presupuesto de 7 MB o el
cronograma. Mostrar la fuente es honesto y útil (el texto Mermaid es legible),
y no bloquea el resto del producto. Es una simplificación deliberada con techo
conocido.

## ADR-6: Conexión con Obsidian y GitHub por archivos, no por API

**Contexto.** El pedido incluye "facilidades para conectar con segundos
cerebros como Obsidian y también GitHub".

**Decisión.** La conexión es **por sistema de archivos**, no por API de red.
Obsidian: abrir la carpeta de una bóveda como workspace, entender `[[wikilinks]]`,
los callouts y la estructura `.obsidian/`. GitHub: entender un repo ya clonado
(enlaces relativos, GFM fiel), no un cliente de la API de GitHub.

**Por qué.** Es lo más liviano y lo más seguro: no hay tokens, no hay red, no
hay superficie nueva. Una bóveda de Obsidian *ya es* una carpeta de `.md`; un
repo clonado *ya es* una carpeta de `.md`. La conexión más potente es también
la más barata. Ver `connectivity.md`. **Alternativa más perezosa descartada:**
no hacer nada especial y tratar la bóveda como carpeta común. Se descarta
porque entender wikilinks es justo lo que ningún competidor liviano hace bien,
y es barato.

## ADR-7: IA local para estudio, opt-in, nunca en el núcleo

**Contexto.** El proyecto apunta a tomar notas para y durante el estudio.
Resúmenes, tarjetas de repaso y preguntas sobre las notas son funciones
naturales de ese caso.

**Decisión.** Si se suma IA, corre **local** (un modelo pequeño en el equipo),
es **opt-in**, y es un **componente aparte** que no cuenta contra el
presupuesto del núcleo. Nada de las notas sale del equipo jamás. Ver
`inference.md`.

**Por qué.** Coherencia con toda la tesis de seguridad y privacidad: un
segundo cerebro que manda tus notas a un servidor ajeno para "resumirlas" es
exactamente lo que este proyecto existe para no hacer.

## ADR-8: Multiplataforma: Windows y Linux desde el día uno, macOS en paralelo

**Contexto.** La v1 es solo Windows. Se pidieron VMs descartables de Linux, y
se dejó a criterio decidir qué hacer con macOS.

**Decisión.** Windows y Linux son objetivos de la v2.0. macOS se compila y se
prueba desde temprano, pero no se publicita hasta tener pruebas propias.

**Por qué.** Mantener la puerta abierta cuesta poco: elegir dependencias
portables y aislar lo específico del sistema en una sola capa. Retrofitear
después cuesta mucho, porque obliga a desarmar suposiciones ya metidas en todo
el código. Barato ahora, sin deuda después.

## ADR-9: Resaltado: sidecar por defecto, incrustado en un clic

**Contexto.** Se quería subrayar sin romper el `.md` ni cómo lo lee Obsidian.

**Decisión.** Las anotaciones viven en un archivo paralelo por defecto. Un
ajuste por documento las incrusta como `==texto==`.

**Por qué.** `==texto==` **es la sintaxis nativa de Obsidian**, así que
incrustar no rompe nada y el resaltado viaja con la nota. Aun así el valor por
defecto es sidecar, porque un archivo ajeno no debería cambiar por haberlo
abierto. El sidecar guarda el texto además del rango, para poder reubicar el
resaltado si la nota se edita por fuera.

## ADR-10: Sin autoguardado por defecto

**Contexto.** Pedido explícito: que no se autoguarde sobre el original, pero
que no se pierda trabajo ante un cierre inesperado.

**Decisión.** Las modificaciones no tocan el archivo original hasta guardar. La
recuperación usa un archivo temporal aparte. El autoguardado se puede activar
desde configuración avanzada.

**Por qué.** Son dos necesidades distintas que suelen confundirse: no perder
trabajo, y no modificar un archivo sin permiso. Un temporal aparte cubre la
primera sin violar la segunda.

## ADR-11: Renombrar encabezados no actualiza enlaces en la v2.0

**Contexto.** Se pidió revisar si choca con Obsidian.

**Decisión.** Fuera de la v2.0.

**Por qué.** Choca. Obsidian tiene su propia lógica de actualización de enlaces
al renombrar, y dos herramientas reescribiendo los mismos archivos con
criterios distintos es una receta para corromper una bóveda. Si entra después,
será con confirmación explícita mostrando qué archivos va a tocar.

## ADR-12: Corrector ortográfico como componente descargable

**Contexto.** Se preguntó cuánto pesa para español e inglés.

**Decisión.** Fuera del núcleo; componente descargable en el futuro.

**Por qué.** Los diccionarios de Hunspell pesan ~1 MB por idioma en disco, pero
el de inglés solo usa ~4,5 MB de RAM al cargarse. Contra un presupuesto de 7 MB
de binario, no cierra. Mismo trato que la IA local y por la misma razón.

## ADR-13: Toolchain MSVC desde el Sprint 0, no GNU de transición

**Contexto.** Windows necesita un enlazador para compilar Rust. Hay dos
objetivos disponibles: MSVC (requiere instalar Visual Studio Build Tools, 2-4
GB) o GNU vía MinGW-w64 (unos cientos de MB, instalable junto con `rustup` sin
nada aparte).

**Decisión.** MSVC desde el primer commit del Sprint 0.

**Por qué.** Se evaluó instalar GNU primero para iterar rápido y migrar a MSVC
recién en el Sprint 8 de distribución, pero el objetivo declarado es la mejor
calidad a largo plazo, no la instalación más liviana hoy. MSVC es el objetivo
que Windows trata como de primera clase, el que espera la Microsoft Store para
firmar, y el que hace que el binario medido en el Sprint 0 sea exactamente el
que se termina distribuyendo, sin una diferencia de tamaño entre objetivos que
explicar más adelante.

## ADR-14: Dependencias sin funciones por defecto, y ninguna en C

**Contexto.** El Sprint 0 compiló el prototipo y auditó el árbol de
dependencias real para Windows. Las funciones por defecto de dos crates
metían cosas que nadie pidió:

- `comrak` traía su interfaz de línea de comandos (`clap`, `shell-words`,
  `xdg`, `fmt2io`) y, sobre todo, `syntect-onig`: resaltado de sintaxis
  apoyado en **Oniguruma**, una librería de expresiones regulares escrita en
  C, vía `onig` y `onig_sys`.
- `tiny-skia` traía `png-format`, un decodificador de PNG que todavía no se
  usa.

**Decisión.** Las dos entran con `default-features = false`. De `tiny-skia` se
piden explícitamente solo `std` y `simd`.

**Por qué.** El ADR-2 elige Rust porque la seguridad de memoria sobre entrada
no confiable es la tesis del proyecto. Un decodificador de expresiones
regulares en C, enlazado por arrastre de una opción por defecto que nadie
revisó, contradice esa tesis en silencio. Que además ahorre medio mega es un
premio, no el motivo.

**Medido.** El binario pasó de 2,66 MB a **2,14 MB** (536 KB menos) y el árbol
de dependencias de 144 a **96 crates**, sin ninguna dependencia en C.

**Consecuencia para el resaltado de sintaxis.** El Sprint 0 tenía que decidir
esa estrategia y esta medición la decide por descarte: **no vía `syntect` con
Oniguruma**. Como el renderizado es nativo y no genera HTML, el resaltado hay
que hacerlo sobre el árbol propio igual. Las opciones que quedan, a evaluar en
el Sprint 2: `syntect` con su motor `fancy-regex` (Rust puro), `tree-sitter`,
o un resaltador propio por tokens para los quince o veinte lenguajes que
importan. La regla que hereda de este ADR: ninguna que arrastre C.

**Regla permanente.** Toda dependencia nueva entra con
`default-features = false` y se le habilitan solo las funciones que se usan.
Antes de agregar una, se revisa qué arrastra con `cargo tree`.

## ADR-15: Cache de glifos rasterizados

**Contexto.** La primera versión del prototipo dibujaba a 39 ms por cuadro
(26 fps) en un documento normal. Se siente pastoso: el objetivo es 60 fps, o
sea menos de 16 ms.

**Decisión.** Los glifos ya rasterizados se guardan en una cache, indexados
por (fuente, tamaño, glifo). El pixmap también se reusa entre cuadros en vez
de reservarse de nuevo.

**Por qué.** La sospecha inicial fue el pixmap (2,7 MB reservados y puestos a
cero por cuadro), pero la aritmética señalaba a otro lado: una pantalla de
texto son unos 2300 glifos, y rasterizar un contorno cuesta del orden de 15
microsegundos, lo que da unos 35 ms. Coincidía casi exacto con los 39 ms
medidos. El pixmap era ruido al lado de eso.

**Medido.** De 39,0 ms a **5,4 ms** por cuadro en un documento normal (26 fps
a 186 fps), y de 47,4 ms a 7,6 ms en uno de 5 MB. Costo en tamaño: **2,5 KB**.
Es la mejor relación esfuerzo/resultado de todo el Sprint 0.

**Pendiente para el Sprint 1.** La cache ignora la posición subpíxel, porque
parley ya entrega posiciones alineadas a píxel. Si al embeber las fuentes
propias se nota pérdida de calidad en el trazo, hay que agregar la posición
subpíxel a la clave, a costa de multiplicar el tamaño de la cache.

## ADR-16: Las alturas se estiman, no se maquetan

**Contexto.** Para saber dónde cae cada bloque en la barra de scroll hay que
saber cuánto mide. Maquetarlos todos para averiguarlo costaba **5122 ms** en
un documento de 5 MB con 43.194 bloques. El documento tardaba 5,7 segundos en
aparecer.

**Decisión.** El alto de cada bloque se **estima** contando caracteres y
dividiendo por cuántos entran por línea. Los bloques que efectivamente se ven
se maquetan de verdad, y ese layout se cachea mientras estén en pantalla.

**Por qué.** El alto exacto de un bloque que está a doscientas pantallas de
distancia no le importa a nadie: solo participa de la proporción de la barra
de scroll. Pagar cinco segundos por esa precisión es un mal negocio.

**Medido.** Posicionar los 43.194 bloques pasó de 5122 ms a **10 ms**. El
documento completo abre en **698 ms** en vez de 5622 ms. El error acumulado en
el alto total es de **5,2 %**, y no afecta lo que se dibuja.

**Costo aceptado y pendiente.** La barra de scroll miente un 5 %, y a medida
que se maquetan bloques reales las posiciones de los siguientes deberían
corregirse. Esa corrección progresiva, sin que el texto salte bajo el cursor,
es trabajo del Sprint 2. La regla de `design.md` de que "el texto nunca se
mueve" aplica: si corregir una posición hace saltar lo que se está leyendo, se
corrige al salir de esa zona, no mientras se lee.

**Corrección a `architecture.md`.** El documento decía "virtualización:
dibujar solo lo visible". La medición muestra que dibujar nunca fue el
problema. Lo caro es **maquetar**, y lo verdaderamente caro es **conservar** lo
maquetado. La virtualización que importa es de layout y de memoria, no de
dibujo.

## ADR-17: Dibujo por software confirmado, sin GPU

**Contexto.** El ADR-3 eligió `tiny-skia` (software) sobre Skia completo por
presupuesto de tamaño, dejando anotado que si el scroll no rendía habría que
evaluar `vello` (GPU) midiendo el costo en tamaño.

**Decisión.** Se queda el dibujo por software. No se evalúa GPU.

**Por qué.** No hace falta: 186 fps en un documento normal y 132 fps en uno de
5 MB, por software, en un equipo de escritorio común. El presupuesto de cuadro
para 60 fps es 16 ms y estamos usando 5,4.

**Lo que se gana además.** Sin dependencia de GPU ni de drivers: una máquina
virtual descartable de Linux con aceleración mal configurada, un escritorio
remoto o un equipo viejo dibujan igual. Para un visor que quiere ser el
predeterminado de `.md` en cualquier máquina, eso vale más que los fps que
sobran.

## ADR-18: Fuentes variables completas, recortadas al vuelo, no estáticas por peso

**Contexto.** `design.md` pide una variable por familia, no un archivo por
peso. Google Fonts distribuye Sora, Newsreader y JetBrains Mono como fuentes
variables completas (miles de glifos, todos los pesos en un `fvar`), demasiado
grandes para embeber tal cual: ~750 KB combinadas antes de recortar.

**Decisión.** Se recortan con `fonttools` al subconjunto latino (Basic Latin +
Latin-1 + Latin Extended-A y puntuación general), conservando el eje variable
de peso (`fvar`) y solo los `name-IDs` que `fontique` necesita para reconocer
la familia por nombre (1, 2, 4, 6, 16, 17). Se descarta `STAT`, que es
metadata de presentación para selectores de estilo que esta app no tiene.

**Por qué.** Guardar una variable en vez de instancias estáticas por peso es
lo que permite pedir cualquier peso (incluidos los intermedios que
`docs/design.md` no fijó) sin sumar otro archivo. El recorte de Unicode es lo
que hace que sea viable en tamaño: de 750 KB a 409,8 KB, dentro del ~0,5 MB
que `budget.md` tenía presupuestado.

**El detalle no obvio.** Vaciar la tabla de nombres para ahorrar los últimos
bytes rompe el registro: `Collection::register_fonts` de `fontique` identifica
la familia leyendo su propio nombre interno, y una fuente sin nombre queda
registrada pero irreconocible por el código que la pide. La receta completa,
para reproducirla o ajustarla, está en `assets/fonts/README.md`.

**Licencia.** Las tres son SIL Open Font License 1.1, que permite embeber,
modificar (el recorte lo es) y redistribuir. La única restricción real es no
vender la fuente suelta bajo su nombre original sin permiso del autor, que no
aplica a este uso.

## ADR-19: El modelo documental preserva fuente y semántica

**Contexto.** El prototipo convirtió el AST en bloques y tramos preparados para
dibujo. Ese modelo pierde destinos, rangos, tablas, IDs y sintaxis desconocida.
Es suficiente para demostrar rendering, pero no para editar y guardar sin pérdida.

**Decisión.** El modelo canónico propio conserva rangos de fuente, estructura y
semántica. Layout y display list son derivados descartables.

**Por qué.** Selección, edición, comparación, anotaciones, enlaces y round-trip
dependen de relacionar lo visible con los bytes originales. Arreglarlo después de
workspace y estudio multiplicaría el retrabajo.

**Costo.** Más tipos, tests y memoria que un modelo aplanado. Se controla con
representaciones compactas y medición.

## ADR-20: Anotaciones portables por defecto

**Contexto.** El ADR-9 elegía sidecar por defecto para no tocar el `.md`. La
definición de producto posterior prioriza compatibilidad con Obsidian y que una IA
pueda releer el resultado.

**Decisión.** Usar sintaxis Markdown u Obsidian portable cuando exista. Ejemplo:
`==resaltado==`. Usar sidecar solo para estado que no se exprese limpiamente,
como fechas de repaso.

**Por qué.** El documento mantiene valor fuera de Visor MD. Evita un sistema de
anotaciones invisible para otras herramientas.

**Restricción.** No crear sintaxis exclusiva. Los sidecars restantes son
versionados, atómicos y detectan cambios de fuente.

## ADR-21: Sin IA propia dentro de Visor MD

**Contexto.** El ADR-7 y `inference.md` exploraban un componente local opcional.
La dirección de producto se refinó: los usuarios trabajan con IA, pero no quieren
otro chatbot ni un modelo integrado.

**Decisión.** Visor MD no incorpora modelo local o remoto. Ofrece herramientas de
formato, copia, fragmentación, comparación y preparación de Markdown.

**Por qué.** Mantiene privacidad, tamaño, superficie de ataque e identidad. La
interoperabilidad con IA aporta valor sin ejecutar inferencia.

## ADR-22: Presupuesto en tres bandas

**Contexto.** La documentación anterior mezclaba objetivo de 7 MB y techo de
9,44 MB. El producto necesita una regla simple que no convierta el tamaño en
enemigo de funciones esenciales.

**Decisión.** Menos de 6 MB es ideal extraordinario, alrededor de 7 MB es el
objetivo y menos de 8 MB es el límite deseado. Superar 8 MB requiere medición,
explicación y aprobación.

**Por qué.** Conserva presión contra dependencias innecesarias sin recortar
seguridad, estabilidad, accesibilidad o Unicode.

## ADR-23: Excepciones de seguridad delimitadas, invariantes fijas

**Contexto.** El núcleo offline debe trabajar con imágenes, enlaces y bóvedas
reales. Un bloqueo absoluto de todo acceso manual sería seguro pero poco útil.

**Decisión.** Configuración avanzada puede permitir imágenes remotas confirmadas,
recursos locales relativos, enlaces manuales, UNC principal, bóvedas confiables y
límites blandos mayores. El alcance es visible y revocable.

Nunca se permite ejecución, eventos HTML, cambios ordenados por documentos,
conexiones ocultas o desactivación del validador.

**Por qué.** Separa capacidad de autoridad. El usuario puede pedir una acción
concreta sin entregar permisos generales al documento.

## ADR-24: Código nativo y `unsafe` se auditan por target

**Contexto.** ADR-14 afirmaba que el árbol quedaba sin C. Era cierto para el
camino Windows medido tras desactivar Oniguruma, pero Linux incluye dependencias
nativas como fontconfig.

**Decisión.** Mantener default features desactivadas cuando convenga, pero auditar
el grafo real de cada target. C, C++ y `unsafe` no están prohibidos de manera
absoluta; requieren justificación, aislamiento, mantenimiento y evidencia.

**Por qué.** Una afirmación verificable por plataforma es más útil que una
promesa universal incorrecta.

## ADR-25: Windows y Linux son targets de v2.0

**Contexto.** ADR-8 incluía macOS desde el primer día. El alcance actual prioriza
terminar y validar Windows y Linux con un mantenedor y un presupuesto pequeño.

**Decisión.** Mantener contratos portables, CI y releases para Windows y Linux.
macOS es futuro y no bloquea v2.0.

**Por qué.** Evita deuda específica de Windows sin sostener tres matrices de QA
antes de que el núcleo madure.

## ADR-26: Pipeline tipográfico reproducible y `STAT` conservado

**Contexto.** El subset inicial descartó `STAT` y no incluyó Newsreader Italic.
La inspección visual mostró emparejamiento incompleto de estilos. El working tree
regeneró fuentes conservando `STAT` y agregó la variante itálica.

**Decisión.** Conservar metadata necesaria para identificación y selección de
estilo. Incluir Newsreader Italic si la cobertura y el emparejamiento lo exigen.
Automatizar origen, hashes, versión de fonttools, Unicode, tablas y notices antes
de cerrar la recuperación.

**Por qué.** Ahorrar metadata no compensa cursiva incorrecta o un proceso
imposible de reproducir. La identidad tipográfica cabe en el límite actual.

## ADR-27: La edición básica se adelanta

**Contexto.** El roadmap anterior construía workspace y Obsidian antes del
editor. El producto se definió como lector y editor, y el guardado fiel afecta el
modelo, anotaciones, índice y compatibilidad.

**Decisión.** Después del lector completo y la validación base, construir editor
fuente, vista dividida y guardado antes de workspace profundo.

**Por qué.** Reduce el riesgo de diseñar varias capas sobre un modelo que luego no
pueda preservar el documento.

## ADR-28: QA y documentación son entregables

**Contexto.** El proyecto también debe demostrar ingeniería verificable para
ciberseguridad, QA y trabajo profesional asistido por IA.

**Decisión.** Threat model, matriz de pruebas, fuzzing, benchmarks, SBOM, ADR,
status y documentación de release forman parte de la definición de terminado.

**Por qué.** Una afirmación de seguridad o rendimiento sin evidencia reproducible
es una intención. Mantener los artefactos junto al código convierte criterios en
controles revisables.

## ADR-29: Un límite defensivo degrada el documento completo

**Contexto.** El recorrido recursivo necesita topes contra citas, listas e inline
patológicos. Cortar una rama evita el stack overflow, pero produce una vista
parcial que puede ocultar contenido y luego guardarse como si fuera completa.

**Decisión.** Si el render enriquecido supera profundidad o cantidad de bloques,
se descarta todo el modelo derivado y se muestra la fuente completa como texto
inerte. El estado se comunica como modo seguro. Solo se rechaza el archivo si ni
esa representación cabe dentro de límites absolutos.

**Por qué.** En seguridad, fallar de forma cerrada no significa perder datos en
silencio. El fallback conserva verdad, disponibilidad y una ruta de lectura sin
interpretar contenido hostil.

## ADR-30: HTML desconocido es visible e inerte

**Contexto.** El prototipo no ejecutaba HTML, pero nodos inline y de bloque sin
hijos podían desaparecer durante el aplanado. Un documento que oculta parte de
su fuente rompe la confianza aunque no ejecute scripts.

**Decisión.** HTML no reconocido se representa literalmente con estilo de código
y nunca crea destinos activos, recursos, DOM o eventos. La allowlist aprobada de
`br`, `kbd`, `mark`, `sub` y `sup` se implementará como semántica nativa cerrada.

**Por qué.** El usuario ve lo que el archivo contiene y puede copiarlo o editarlo
sin exponer la aplicación a una superficie de navegador.

## ADR-31: El scroll consulta solo el intervalo visible

**Contexto.** La virtualización evitaba maquetar bloques lejanos, pero cada cuadro
recorría todos los slots para descubrir cuáles eran visibles y buscaba índices en
un vector. El coste seguía creciendo con el documento completo.

**Decisión.** Mantener slots ordenados y obtener inicio y fin visibles mediante
búsqueda binaria. Podar y dibujar solo ese rango. Calcular además el scroll máximo
con el alto real del viewport, no con una constante.

**Por qué.** El trabajo por cuadro queda principalmente ligado al contenido en
pantalla y el resize no permite desplazarse hacia una zona vacía inexistente.

## ADR-32: Contexto progresivo y verificación proporcional

**Contexto.** `AGENTS.md` acumuló propósito, seguridad, UX, arquitectura, QA y
presupuestos que ya estaban descritos en documentación especializada. Sus 614
líneas hacían que una tarea local cargara reglas ajenas y podía inducir gates
globales para cambios que no afectaban esas propiedades.

**Decisión.** `AGENTS.md` conserva prioridades, invariantes, relación de trabajo,
preservación Git, economía de implementación, mapa de autoridad y niveles de
riesgo. Las reglas detalladas permanecen en `security.md`, `threat-model.md`,
`connectivity.md`, `architecture.md`, `testing.md`, `budget.md`, `design.md`,
`product.md` y `dependencies.md`. La verificación se elige por comportamiento
afectado: normal, sensible o auditoría.

**Por qué.** Menos contexto recurrente mejora velocidad y foco sin reducir las
garantías. Un cambio documental no necesita recompilar release; parser, VFS,
guardado, red o dependencias siguen exigiendo evidencia más profunda. La tabla
de enrutamiento evita que la simplificación se convierta en omisión.

## ADR-33: Portapapeles de texto explícito y sin lectura

**Contexto.** Copiar fragmentos limpios y Markdown original es un caso de uso
central para estudio, IA y bóvedas, pero el portapapeles cruza el límite hacia
otras aplicaciones y puede introducir dependencias de plataforma.

**Decisión.** Usar `arboard` sin sus funciones por defecto. Crear y conservar el
portapapeles solo después de `Ctrl+C` o `Ctrl+Shift+C`; nunca leer, pegar,
mantener historial, copiar imágenes ni transmitir contenido. La vista se copia
como texto y la fuente se copia por bloques completos.

**Por qué.** Entrega una interacción habitual sin convertir el lector en un
observador del portapapeles ni reconstruir Markdown a partir de una vista. La
separación conserva la intención del documento y mantiene acotadas las nuevas
dependencias y permisos.

## ADR-34: Apertura primaria limitada desde el mismo handle

**Contexto.** El prototipo abría el archivo con `read_to_string` directamente.
Eso no establecía una cota de memoria antes del parser y dejaba una carrera
TOCTOU entre cualquier comprobación futura por ruta y la lectura real.

**Decisión.** La entrada principal explícita se abre una única vez, se consulta
como archivo normal desde ese handle, se lee como máximo hasta 16 MiB y debe ser
UTF-8 válido. UNC continúa permitido como archivo principal porque la persona
lo eligió; no concede acceso a archivos vecinos.

**Por qué.** El límite supera ampliamente el corpus de 5 MiB y evita que un
doble clic en texto hostil reserve memoria sin cota. Usar el mismo handle para
metadatos y bytes hace que la decisión se aplique al archivo efectivamente
leído. El valor es temporal: ampliar límites blandos requiere primero medir el
modo seguro y sus costes, no eliminar el techo por comodidad.
