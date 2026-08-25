# Registro de decisiones

Formato ADR liviano: cada decisión con contexto, opción elegida y el motivo
por el que se descartó el resto. Fecha implícita: fase de planificación de la
v2.

## ADR-1 — Nativo, sin motor web

**Contexto.** La v1 usa WebView2. Pesa 30 MB en disco, arranca en 3-4 s y su
seguridad depende de sanitizar HTML activo correctamente para siempre.

**Decisión.** La v2 es nativa: no empaqueta ni usa ningún motor web para
renderizar el documento.

**Por qué.** El requisito de <7 MB descarta de plano cualquier cosa que
empaquete un Chromium (Electron: 80-150 MB). Apoyarse en el WebView2 del
sistema —como hace la v1, o como haría Tauri— evita ese peso, pero conserva el
motor de scripts como superficie de ataque permanente. La vía nativa elimina
esa superficie por construcción: no hay JavaScript que ejecutar porque no hay
intérprete. Es la lección central de Tinta.

**Costo aceptado.** Perder la fidelidad "gratis" de un navegador: Mermaid,
KaTeX y HTML arbitrario dejan de venir resueltos. Ver ADR-5 y `product.md`.

## ADR-2 — Lenguaje: Rust

**Contexto.** Tinta usa C++. El pedido explícito fue no elegir C++ por inercia
y comparar Go, Lisp y otros.

**Decisión.** Rust.

**Por qué, comparado.**

| Lenguaje | A favor | En contra | Veredicto |
| --- | --- | --- | --- |
| **Rust** | Seguridad de memoria sin recolector de basura; ecosistema maduro de parseo y render de texto; binarios pequeños; sin pausas de GC | Curva de aprendizaje; tiempos de compilación | **Elegido** |
| C++ (Tinta) | El más chico posible; control total; MD4C ya existe | La seguridad de memoria sobre entrada no confiable es exactamente el terreno de los CVE de corrupción —un lector nativo en C++ contradice la tesis de seguridad del proyecto en la capa del lenguaje | Descartado |
| Go | Simple; binarios razonables | El recolector de basura mete jitter de latencia perceptible al hacer scroll de documentos grandes; los toolkits GUI (Fyne) pasan de 10 MB y no se sienten nativos; Gio es inmediato y arrastra el runtime + GC | Descartado |
| Zig | Aún más chico que Rust; control fino | Ecosistema inmaduro para GUI y texto; sin la red de seguridad de memoria de Rust | Descartado |
| Lisp (SBCL) | Expresividad; interactividad | No hay camino realista a un GUI nativo <7 MB: la imagen de SBCL sola ya rompe el presupuesto; tooling GUI marginal | Descartado |

**La razón que decide.** Todo el proyecto se vende como "seguro por
construcción". Elegir C++ —donde un `.md` malformado puede provocar un
desbordamiento de búfer en el parser, como el CVE-2026-5525 de Notepad++ en su
manejo de rutas por arrastre— socavaría esa tesis en la capa más baja. Rust
elimina esa clase entera de fallos sin pagar el precio de latencia de un
recolector de basura. Es el único lenguaje que satisface a la vez "seguro" y
"liviano y fluido".

## ADR-3 — La superficie del documento se dibuja a mano, no con un widget de texto

**Contexto.** Un documento Markdown renderizado no es una UI de widgets: es
texto reflowable con estilos inline mezclados, enlaces clicables, bloques de
código resaltados, selección que cruza párrafos. Los toolkits de widgets
(Slint, egui) están pensados para botones y formularios, no para esto.

**Decisión.** La vista del documento se construye sobre una pila de **layout de
texto + dibujo 2D** (candidatos en Rust: `parley` para layout, `swash` para
glifos, `tiny-skia` para dibujo por software). El "chrome" de la app —pestañas,
barra lateral, menús, diálogos— sí puede usar un toolkit liviano.

**Por qué.** Es lo que hace un lector serio y lo que evita pelear contra las
suposiciones de un framework de widgets. `tiny-skia` (dibujo por software)
sobre Skia completo mantiene el presupuesto de tamaño; ver `budget.md`.

**Riesgo principal, declarado.** Esta es la parte más grande y más incierta del
proyecto. Se valida con un prototipo antes que nada (ver `roadmap.md`, Fase 0).

## ADR-4 — Un solo punto de sanitización, formalizado

**Contexto.** La v1 ya tiene un único `clean()` con DOMPurify al final del
pipeline. ByteMD demuestra que ese patrón —árbol sintáctico tipado, un solo
saneo al final— es el correcto. EasyMDE demuestra el error opuesto: saneo
opcional, apagado por defecto.

**Decisión.** El parser (candidato: `comrak`, CommonMark + GFM en Rust puro)
produce un árbol tipado. Ningún nodo llega a la superficie de dibujo sin pasar
por un único validador. No existe forma de apagarlo.

**Por qué.** En un renderizador nativo la "sanitización" cambia de forma: no
hay HTML que limpiar, hay una **allowlist de nodos que el renderizador sabe
dibujar**. Un nodo HTML crudo fuera de esa lista no se ejecuta —no hay dónde—,
se muestra como texto inerte o se descarta. Es más fuerte que sanitizar: lo
desconocido no tiene un motor donde correr.

## ADR-5 — Mermaid y matemática: fuera del núcleo de la v2.0

**Contexto.** Reimplementar el layout de 22 tipos de diagrama Mermaid
nativamente es, según se estableció en la exploración, el ítem más caro de
todo el proyecto, más que el parser. La matemática nativa (equivalente a
KaTeX) es igual de pesada y sin una librería madura reutilizable clara.

**Decisión.** La v2.0 **no** renderiza Mermaid ni matemática de forma nativa.
Muestra la fuente del diagrama/fórmula en un bloque con estilo, igual que hace
la v1 cuando Mermaid no está disponible. El render real queda como componente
**opcional, de descarga separada y claramente marcado**, para una versión
posterior — y solo si se encuentra una vía 100% local que no rompa el
presupuesto ni la política de red.

**Por qué.** Meter Mermaid al núcleo rompe el presupuesto de 7 MB o el
cronograma. Mostrar la fuente es honesto y útil (el texto Mermaid es legible),
y no bloquea el resto del producto. Es una simplificación deliberada con techo
conocido.

## ADR-6 — Conexión con Obsidian y GitHub por archivos, no por API

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
no hacer nada especial y tratar la bóveda como carpeta común — se descarta
porque entender wikilinks es justo lo que ningún competidor liviano hace bien,
y es barato.

## ADR-7 — IA local para estudio, opt-in, nunca en el núcleo

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

## ADR-8 — Multiplataforma: Windows y Linux desde el día uno, macOS en paralelo

**Contexto.** La v1 es solo Windows. Se pidieron VMs descartables de Linux, y
se dejó a criterio decidir qué hacer con macOS.

**Decisión.** Windows y Linux son objetivos de la v2.0. macOS se compila y se
prueba desde temprano, pero no se publicita hasta tener pruebas propias.

**Por qué.** Mantener la puerta abierta cuesta poco: elegir dependencias
portables y aislar lo específico del sistema en una sola capa. Retrofitear
después cuesta mucho, porque obliga a desarmar suposiciones ya metidas en todo
el código. Barato ahora, sin deuda después.

## ADR-9 — Resaltado: sidecar por defecto, incrustado en un clic

**Contexto.** Se quería subrayar sin romper el `.md` ni cómo lo lee Obsidian.

**Decisión.** Las anotaciones viven en un archivo paralelo por defecto. Un
ajuste por documento las incrusta como `==texto==`.

**Por qué.** `==texto==` **es la sintaxis nativa de Obsidian**, así que
incrustar no rompe nada y el resaltado viaja con la nota. Aun así el valor por
defecto es sidecar, porque un archivo ajeno no debería cambiar por haberlo
abierto. El sidecar guarda el texto además del rango, para poder reubicar el
resaltado si la nota se edita por fuera.

## ADR-10 — Sin autoguardado por defecto

**Contexto.** Pedido explícito: que no se autoguarde sobre el original, pero
que no se pierda trabajo ante un cierre inesperado.

**Decisión.** Las modificaciones no tocan el archivo original hasta guardar. La
recuperación usa un archivo temporal aparte. El autoguardado se puede activar
desde configuración avanzada.

**Por qué.** Son dos necesidades distintas que suelen confundirse: no perder
trabajo, y no modificar un archivo sin permiso. Un temporal aparte cubre la
primera sin violar la segunda.

## ADR-11 — Renombrar encabezados no actualiza enlaces en la v2.0

**Contexto.** Se pidió revisar si choca con Obsidian.

**Decisión.** Fuera de la v2.0.

**Por qué.** Choca. Obsidian tiene su propia lógica de actualización de enlaces
al renombrar, y dos herramientas reescribiendo los mismos archivos con
criterios distintos es una receta para corromper una bóveda. Si entra después,
será con confirmación explícita mostrando qué archivos va a tocar.

## ADR-12 — Corrector ortográfico como componente descargable

**Contexto.** Se preguntó cuánto pesa para español e inglés.

**Decisión.** Fuera del núcleo; componente descargable en el futuro.

**Por qué.** Los diccionarios de Hunspell pesan ~1 MB por idioma en disco, pero
el de inglés solo usa ~4,5 MB de RAM al cargarse. Contra un presupuesto de 7 MB
de binario, no cierra. Mismo trato que la IA local y por la misma razón.
