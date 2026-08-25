# Roadmap

Este es el documento operativo del proyecto. Está escrito para que decir
**"avanza con el roadmap"** sea suficiente: cada sprint dice qué se construye,
qué prueba lo cubre, y qué tiene que ser cierto para pasar al siguiente.

## Cómo funciona

- **Sprints numerados**, en orden. No se salta uno.
- Cada sprint termina con un **criterio de salida** verificable. Si no se
  cumple, el sprint no está listo, por más que "casi".
- Cada sprint dice si necesita **prueba manual tuya**. Cuando la necesite, te
  paso qué probar en concreto y en qué orden.
- Las pruebas automáticas de un sprint quedan corriendo para siempre. Ningún
  sprint posterior avanza con una suite anterior en rojo.
- **Fuera del happy path desde el principio**: cada sprint prueba también el
  caso raro, no solo el que funciona.

## Estado

| Sprint | Qué entrega | Estado |
| --- | --- | --- |
| 0 | Prototipo y validación del presupuesto | ✅ **Cerrado**, criterio cumplido |
| 1 | Lector mínimo usable | ⬜ |
| 2 | Lector completo | ⬜ |
| 3 | Chrome y pestañas | ⬜ |
| 4 | Workspace | ⬜ |
| 5 | Obsidian y GitHub | ⬜ |
| 6 | Edición | ⬜ |
| 7 | Anotaciones y estudio | ⬜ |
| 8 | Distribución | ⬜ |
| 9 | Linux | ⬜ |

---

## Sprint 0 · Prototipo y validación del presupuesto

**El único sprint que puede cambiar todo el plan. Por eso va primero y solo.**

Construir: abrir un `.md`, parsear con `comrak`, dibujar encabezados, párrafos,
listas y tablas con `parley` + `tiny-skia`. Nada más: sin chrome, sin pestañas,
sin Mermaid.

Medir, con números reales y anotados en `budget.md`:
- Tamaño del binario con `strip`, `panic=abort`, `opt-level=z`.
- Tiempo hasta ventana visible y hasta primer pintado.
- RAM en reposo con un documento abierto.
- Rendimiento del scroll con un documento de 5 MB.

Decidir con esos números: backend de dibujo (software o GPU), toolkit de chrome
(sí o dibujo propio), estrategia de resaltado de sintaxis, almacenamiento del
índice.

Auditar con `cargo geiger` cuánto `unsafe` trae la pila elegida.

**Criterio de salida:** un binario que abre un documento simple, medido, por
debajo de 7 MB con margen. Si no se llega, se replantea alcance o presupuesto
**acá**, antes de construir nada encima.

**Prueba manual:** sí, corta. Abrir el prototipo y decirme si el arranque se
*siente* instantáneo. El número importa menos que la sensación.

### ✅ Resultado

**Criterio cumplido con holgura: 2,14 MB contra un objetivo de 7.** Los
números completos están en `budget.md`; el resumen:

| Métrica | Objetivo | Medido |
| --- | --- | --- |
| Tamaño del binario | < 7 MB | **2,14 MB** |
| Ventana visible | < 150 ms | **79 ms** |
| Primer pintado, documento normal | < 400 ms | **119 ms** |
| Scroll, documento normal | 60 fps | **186 fps** (5,4 ms) |
| Scroll, documento de 5 MB | 60 fps | **132 fps** (7,6 ms) |
| RAM, documento normal | decenas de MB | **19 MB** |
| Documento de 5 MB abierto | sin objetivo fijado | **698 ms** |
| RAM, documento de 5 MB | sin objetivo fijado | 120 MB |
| Dependencias | mínimas | **96, ninguna en C** |
| `cargo audit` | limpio | 0 vulnerabilidades, 1 aviso |

**Las cuatro decisiones que el sprint tenía que tomar, tomadas:**

1. **Backend de dibujo: software.** No hace falta GPU. ADR-17.
2. **Resaltado de sintaxis: no con `syntect` + Oniguruma**, porque arrastra C.
   Las alternativas quedan acotadas y se elige en el Sprint 2. ADR-14.
3. **Toolkit de chrome:** se pospone al Sprint 3 con información nueva. Dibujar
   texto y rectángulos con la pila propia salió barato, así que dibujar el
   chrome a mano dejó de ser el plan de emergencia y pasó a ser la opción por
   defecto razonable.
4. **Almacenamiento del índice:** sin decidir todavía, no hacía falta para el
   prototipo. Queda para el Sprint 4, con el margen de tamaño que ahora hay.

**Los tres problemas que aparecieron, y qué se hizo:**

| Problema medido | Causa | Solución | Resultado |
| --- | --- | --- | --- |
| 39 ms por cuadro (26 fps) | Rasterizar cada glifo en cada cuadro | Cache de glifos (ADR-15) | 5,4 ms (186 fps) |
| 393 MB con un documento de 5 MB | 43.194 layouts de parley vivos a la vez | Guardar solo posiciones (ADR-16) | 120 MB |
| 5,7 s para abrir un documento de 5 MB | Maquetar los 43.194 bloques para saber su alto | Estimar el alto (ADR-16) | 698 ms |

**Lo que queda anotado para más adelante:** el parseo de un documento de 5 MB
cuesta 522 ms en el hilo de interfaz. Es lo único que falta para que un archivo
gigante también abra instantáneo, y ya estaba previsto en el Sprint 2.

**Aviso de `cargo audit`:** `ttf-parser` 0.25.1 figura como **sin
mantenimiento** (RUSTSEC-2026-0192). No es una vulnerabilidad. Entra de forma
transitiva por la pila de fuentes. Hay que vigilarlo: si aparece un fallo real
en un crate sin mantenedor, no va a haber parche. Se revisa en el Sprint 1, que
es cuando se toca la capa de fuentes.

**Lo que quedó sin hacer, dicho claro:** el sprint pedía auditar con
`cargo geiger` cuánto `unsafe` trae la pila. **No se corrió.** En su lugar se
auditó el árbol de dependencias completo, que dio el hallazgo más importante
del sprint (el C que estaba enlazado sin que nadie lo pidiera, ADR-14), pero no
es lo mismo: falta el conteo de bloques `unsafe` por crate. Queda pendiente
para el Sprint 1.

Lo que sí quedó puesto: **`#![forbid(unsafe_code)]`** en el código propio, que
ahora es una regla que hace fallar la compilación, no una intención escrita en
un documento.

**Prueba manual pendiente:** falta que abras el prototipo y digas si el
arranque se *siente* instantáneo. El número dice 119 ms; la sensación la
tenés que confirmar vos.

---

## Sprint 1 · Lector mínimo usable

- CommonMark completo.
- ✅ Fuentes embebidas y la escala tipográfica de `design.md`.
- ✅ Tema Papel + Tinta, claro y oscuro.
- Ventana sin borde, con su capa de integración con el sistema.
- Scroll con virtualización.

**Pruebas automáticas:** corpus de CommonMark; el documento de estrés de la v1
renderiza sin panic; fuzzing del parser en marcha.

**Criterio de salida:** se puede leer un documento real de principio a fin y se
ve bien.

**Prueba manual:** sí. Abrí tus propios `.md` y buscá lo que se vea mal.

### Avance parcial

**Fuentes y tema ya están, adelantados desde el Sprint 0.** Sora, Newsreader y
JetBrains Mono embebidas (409,8 KB, subconjunto latino, ver
`assets/fonts/README.md`), con el tema Papel + Tinta completo: sigue al tema
del sistema operativo al abrir y se alterna a mano con `T`. Costo: 400 KB de
binario y 1 ms de arranque. El binario total pasó de 2,14 a **2,54 MB**.

Falta todavía: CommonMark completo (hoy corre GFM básico heredado del Sprint
0), la ventana sin borde con su capa de integración, y la virtualización de
scroll ya presente en el layout pero sin pulir para el caso de encabezados que
cruzan el borde de la pantalla.

---

## Sprint 2 · Lector completo

- GFM: tablas, tareas, tachado, notas al pie, autolinks.
- Resaltado de sintaxis.
- Alertas de GitHub y callouts de Obsidian.
- Imágenes locales, remotas bloqueadas.
- Bloque de Mermaid mostrando su fuente con estilo.
- Índice lateral, con el contador de palabras abajo.
- Plegado de secciones entre encabezados.

**Pruebas automáticas:** suite de seguridad de la v1 portada entera y en verde
(incluidos los tres caminos de red que encontró en su día); corpus de
conversión defectuosa; límites de recursos.

**Criterio de salida:** paridad de lectura con la v1, con las cuatro propiedades
de seguridad verificadas por pruebas.

**Prueba manual:** sí. Comparar lado a lado con la v1 y con GitHub.

---

## Sprint 3 · Chrome y pestañas

- Pestañas, con las animaciones de `design.md`.
- Ventanas múltiples y arrastre de pestañas entre ellas.
- **Dividir a la derecha y abajo**, abriendo una pestaña nueva con la opción de
  crear archivo, abrir archivo o cerrar. No un duplicado.
- Menú contextual por zona.
- Barra de herramientas con desplegable de H1–H6, botones con símbolo y
  tooltip, menú de símbolos y entidades.
- **Siempre encima**, junto a minimizar y cerrar.

**Pruebas automáticas:** ocho archivos a la vez aterrizan en una ventana; cerrar
con cambios sin guardar avisa; el estado de "siempre encima" sobrevive a
minimizar, maximizar y cambio de escritorio virtual.

**Criterio de salida:** se puede trabajar con varios documentos sin fricción.

**Prueba manual:** sí, y **fuera del happy path** como pediste para "siempre
encima": probarlo con pantalla completa de otra app, con un segundo monitor, al
bloquear y desbloquear la sesión, y con la ventana minimizada.

---

## Sprint 4 · Workspace

- Abrir una carpeta como espacio de trabajo, con árbol lateral.
- Recientes y favoritos persistentes.
- Búsqueda en toda la carpeta.
- Sesión restaurada, con recuperación ante cierre inesperado por archivo
  temporal aparte, sin autoguardar sobre el original.
- Tabla de contenido flotante, activable desde configuración avanzada.

**Pruebas automáticas:** el índice es incremental y no reindexa lo que no
cambió; una carpeta de 10.000 archivos no cuelga el arranque; matar el proceso
con cambios sin guardar los recupera al reabrir **sin haber tocado el original**.

**Criterio de salida:** se puede usar como herramienta de trabajo diaria.

**Prueba manual:** sí. Abrí una carpeta grande de verdad y contame si arrastra.

---

## Sprint 5 · Obsidian y GitHub

- Wikilinks `[[nota]]` y `[[nota|alias]]`: resolver, navegar, marcar rotos.
- Enlaces a encabezados y bloques.
- Backlinks.
- Repo clonado: enlaces relativos, raíz del repo, README automático.

**Pruebas automáticas:** ningún wikilink resuelve fuera de la bóveda; ciclos de
embed detectados; el índice sobrevive a que se renombre un archivo por fuera.

**Criterio de salida:** abrir una bóveda de Obsidian real y navegarla completa.

**Prueba manual:** sí, y es la más importante del proyecto. Usá tu bóveda real.

---

## Sprint 6 · Edición

- Editor de texto plano con barra de ayudas y atajos.
- Vista dividida con scroll sincronizado, con el bug de la v1 ya resuelto de
  origen: el panel donde se escribe manda, el otro sigue.
- Listas automáticas, indentado, pegar URL sobre selección.
- Lista numerada con el comportamiento corregido de la v1.
- Pegar imagen del portapapeles: la guarda y arma el enlace.
- Guardado atómico con codificación y fin de línea preservados.

**Pruebas automáticas:** las de edición de la v1 portadas, incluidos los dos
bugs corregidos como casos de regresión; guardar preserva codificación, BOM y
fin de línea; un corte durante el guardado no corrompe el original.

**Criterio de salida:** paridad de edición con la v1, sin sus dos bugs.

**Prueba manual:** sí. Escribí un documento largo entero.

---

## Sprint 7 · Anotaciones y estudio

- Resaltado desde modo lectura, en sidecar por defecto, con opción de incrustar
  como `==texto==`.
- Estadísticas de lectura al pie del panel de índice.
- Temporizador Pomodoro en configuración avanzada.
- Exportar a PDF directo, sin pasar por imprimir.

**Pruebas automáticas:** un resaltado sobrevive a que la nota se edite por
fuera; incrustar y desincrustar es reversible sin pérdida; el sidecar
manipulado no produce lectura fuera de la nota.

**Criterio de salida:** se puede estudiar sobre un documento sin salir de él.

**Prueba manual:** sí. Estudiá algo de verdad con la app.

---

## Sprint 8 · Distribución (Windows)

- Empaquetado portable dentro del presupuesto.
- Asociación de archivos y programa predeterminado.
- Instalación y desinstalación limpias, sin permisos de administrador.
- Release verificable por hash.
- Envío a Microsoft Store.

**Criterio de salida:** un release descargable que se fija como predeterminado
igual que la v1.

**Prueba manual:** sí. Instalación limpia en una VM.

---

## Sprint 9 · Linux

- Empaquetado AppImage y `.deb`.
- Asociación de archivos por `.desktop` y `xdg-mime`.
- Capa de integración con el sistema completa.

**Criterio de salida:** funciona en una VM de Ubuntu y de Fedora recién
instaladas.

**Prueba manual:** sí, en tus VMs descartables.

---

## Después de la v2.0, en orden de prioridad

1. **Edición en vivo** (escribir sobre el documento renderizado, estilo
   Obsidian). Es el objetivo grande que sigue: alto valor, alto costo, y hacerlo
   mal es peor que no hacerlo. Ver `architecture.md`.
2. **Mermaid nativo**, empezando por flowchart y secuencia.
3. **IA local** por el camino barato: hablar con un Ollama que ya tengas.
4. **macOS** publicado, si las pruebas en paralelo salieron bien.
5. **Repaso espaciado** completo.
6. Corrector ortográfico como componente descargable.
7. Grafo de notas, referencias a nivel de bloque, plugins.

## La regla que gobierna todo

Ningún sprint avanza con el criterio de salida sin cumplir o con una suite
anterior en rojo. Es la disciplina que evita terminar como simpler-paper
(archivado por un mantenedor sin red de seguridad): alcance acotado por sprint,
pruebas por sprint, y honestidad para frenar en el Sprint 0 si los números no
dan.
