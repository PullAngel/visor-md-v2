# Presupuesto de tamaño y de arranque

Este documento conserva las mediciones históricas de Sprint 0 y define cómo se
medirán las siguientes etapas. Los números de Sprint 0 describen el commit y el
corpus medidos. El checkpoint de recuperación se identifica por separado.

Equipo de medición: Windows 10 Pro, toolchain MSVC, `rustc` 1.98.0. Binario
compilado con `opt-level = "z"`, `lto = true`, `codegen-units = 1`,
`panic = "abort"` y `strip = true`.

## Los tres números

| | Valor |
| --- | --- |
| **Ideal extraordinario** | < 6 MB |
| **Objetivo de trabajo** | alrededor de 7 MB |
| **Límite deseado** | < 8 MB |
| **Medido en el Sprint 0** | **2,14 MB** |
| **Checkpoint de recuperación** | **2.995.712 bytes, 2,86 MiB** |

Superar 8 MB exige medición, explicación y aprobación. El límite no se usa para
recortar seguridad, estabilidad, accesibilidad, Unicode o funciones esenciales.
Mermaid, correctores y otros componentes pesados no tienen permiso automático
para ampliar el núcleo.

---

## Resultados del Sprint 0

### Tamaño del binario

| Estado | Tamaño |
| --- | --- |
| Con las funciones por defecto de las dependencias | 2,66 MB |
| **Con las funciones recortadas (lo que se distribuye)** | **2,14 MB** |
| Diferencia | 536 KB |

El recorte está explicado en el ADR-14: las funciones por defecto de `comrak`
arrastraban su interfaz de línea de comandos y `syntect` con Oniguruma, una
librería de expresiones regulares escrita en C. Sacarlas ahorró medio mega y
dejó el camino Windows medido sin esa dependencia. La auditoría posterior mostró
dependencias nativas diferentes en Linux, incluido `fontconfig`, por lo que no se
afirma ausencia universal de C.

### Arranque, documento normal (8,9 KB)

| Etapa | Medido |
| --- | --- |
| Leer y parsear el archivo | 1 ms |
| `EventLoop::new` | 5 ms |
| `FontContext::new` (fuentes del sistema) | 3 ms |
| Arranque del bucle de eventos | 10 ms |
| `create_window` (Windows) | 69 ms |
| **Ventana visible** | **79 ms** |
| Superficie de píxeles | 0 ms |
| Posicionar los 77 bloques | 18 ms |
| **Primer pintado** | **119 ms** |
| **Proceso completo (abrir, pintar, salir)** | **168 ms** |

Contra el objetivo: ventana visible <150 ms → **79 ms**. Primer pintado
<400 ms → **119 ms**. Los dos hitos se cumplen con margen.

Para tener referencia de cuánto de eso es nuestro y cuánto es Windows: una
ventana de `winit` **vacía**, sin nada adentro, tarda 141 ms de proceso
completo en el mismo equipo. Es decir que abrir, parsear, maquetar y dibujar un
documento entero agrega unos **30 ms** sobre el costo de existir como ventana.

### Arranque, documento grande (5,03 MB, 43.194 bloques)

| Etapa | Con medida exacta | **Con alturas estimadas** |
| --- | --- | --- |
| Parseo | 483 ms | 522 ms |
| Ventana visible | 567 ms | 612 ms |
| Posicionar 43.194 bloques | 5122 ms | **10 ms** |
| **Primer pintado** | 5710 ms | **647 ms** |
| **Proceso completo** | 5622 ms | **698 ms** |

La diferencia es el ADR-16. Maquetar los 43 mil bloques para saber cuánto mide
cada uno costaba 5 segundos; estimar el alto contando caracteres cuesta 10 ms,
con un error del 5,2 % en el alto total del documento, que solo afecta la
proporción de la barra de scroll. Los bloques que se ven se maquetan de verdad.

Lo que queda por mejorar acá es el parseo: 522 ms en el hilo de interfaz. El
roadmap actualizado lo mueve al cierre correcto de Sprint 1, porque una entrada
grande no debe congelar la UI ni esperar a una etapa posterior.

### Rendimiento del scroll

| Documento | Por cuadro | Equivalente |
| --- | --- | --- |
| 8,9 KB | 5,4 ms | 186 fps |
| 5,03 MB | 7,6 ms | 132 fps |

Medido sobre 240 cuadros recorriendo el documento entero, con el modo
`--bench` del prototipo, para que no dependa de que alguien arrastre la rueda
del mouse.

**El número que importa acá no es el final sino el camino.** La primera versión
daba 39 ms por cuadro (26 fps), que se siente pastoso. La causa era rasterizar
cada glifo en cada cuadro. Con una cache de glifos pasó a 5,4 ms. Ver ADR-15.

### Memoria

| Documento | Working set | Privada |
| --- | --- | --- |
| 8,9 KB | 19,1 MB | 8,5 MB |
| 5,03 MB | 120,4 MB | 151,3 MB |

La meta cualitativa era "unas decenas de MB para un documento típico, contra
los cientos de un webview". **19 MB** lo cumple de sobra: la v1 con WebView2
arranca en cientos de MB por sus procesos de navegador y GPU.

Los 120 MB del documento de 5 MB también salen de una corrección: la primera
versión usaba **393 MB** porque mantenía vivos los 43 mil layouts de parley.
Guardar solo la posición y rehacer el layout de lo visible bajó a 120 MB. Lo
que queda es el árbol de comrak más el texto de los bloques, no el maquetado.

## Checkpoint de recuperación del 26 de agosto de 2026

Estas cifras pertenecen al commit recuperado `a54c9d6`. Se midió el ejecutable
directamente después de compilar release, para no confundir LTO con tiempo de
apertura.

| Medida | Resultado |
| --- | --- |
| Binario Windows | 2.995.712 bytes, 2,86 MiB |
| SHA-256 | `8E63A6843BED47DF1DD12F94630C2D9E307E5209A32DFE85834FD9AE122CF0B2` |
| Documento | `docs/architecture.md`, 10,4 KB, 207 bloques |
| Parseo | 1 ms |
| Primer pintado, ejecución cálida | 110 ms |
| Primeras ejecuciones atípicas observadas | 388 ms y 965 ms |
| Scroll automatizado, 240 cuadros | 4,9 ms, 203 fps equivalentes |

La variación es evidencia, no todavía una conclusión sobre arranque frío. El
protocolo futuro debe realizar varias ejecuciones, registrar percentiles y
declarar estado de caché, equipo y carga del sistema.

---

## Presupuesto de tamaño: estimado contra medido

| Componente | Estimación previa | Realidad |
| --- | --- | --- |
| Runtime de Rust + `std` | 0,3–0,8 MB | Incluido en los 2,14 |
| Parser (`comrak`) | 0,3–0,6 MB | Incluido |
| Layout de texto (`parley` + `swash`) | 0,8–1,5 MB | Incluido |
| Dibujo 2D (`tiny-skia`) | 0,5–1,0 MB | Incluido |
| **Subtotal medido, todo lo anterior junto** | **1,9–3,9 MB** | **2,14 MB** |
| Fuentes embebidas (Sora + Newsreader + JetBrains Mono) | ~0,5 MB | **0,41 MB, medido** |
| **Subtotal con tipografía inicial** | | **2,54 MB** |
| Resaltado de sintaxis | 0,3–0,8 MB | Pendiente, ver ADR-14 |
| Índice del workspace | 0–1,0 MB | Pendiente, Sprint 4 |
| Chrome | 0,5–2,0 MB | Pendiente, Sprint 3 |
| Iconos y recursos | 0,1 MB | Pendiente |
| **Proyección histórica** | | **3,9–6,9 MB** |

La estimación original daba 3,3–8,3 MB. La proyección de 3,9–6,9 MB fue útil
para validar dirección, pero no incluye todavía editor, accesibilidad,
exportación o todas las dependencias multiplataforma. No se usa como promesa.

**Las fuentes iniciales, medidas.** Las primeras tres familias recortadas sumaron
409,8 KB y costaron cerca de 1 ms de registro. El working tree posterior agregó
Newsreader Italic y regeneró los subconjuntos conservando `STAT`; los cuatro
archivos locales suman 694.332 bytes. La recuperación automatizó el proceso y
reprodujo los cuatro hashes con fonttools 4.63.0. Ver
`assets/fonts/README.md`.

Las palancas siguen disponibles pero ya no hacen falta con urgencia:

1. **`tiny-skia` (software) en vez de Skia completo.** Confirmada como la
   decisión correcta: además de tamaño, rinde 186 fps por software. No hace
   falta GPU. Ver ADR-17.
2. **Índice en JSON en vez de SQLite**, si el tamaño de bóveda lo permite.
3. **Chrome dibujado a mano** si el toolkit elegido pesa demasiado.
4. **Bajar a dos familias tipográficas.** Con el margen actual, esta palanca
   probablemente no se necesite: la identidad tipográfica se puede pagar.

Ninguna fuente del sistema se embebe: solo cuatro archivos de tres familias
propias, incluida Newsreader Italic.

## Presupuesto de arranque

| Hito | Objetivo | Referencia | **Medido** |
| --- | --- | --- | --- |
| Ventana visible | < 150 ms | Tinta: <100 ms · v1: ~2 s | **79 ms** |
| Primer documento pintado | < 400 ms | v1: 3–4 s | **119 ms** |
| Documento de 5 MB pintado | Progreso visible < 300 ms | v1: no lo maneja | 647 ms ⚠️ |

El único que no cumple es el documento gigante, y se sabe exactamente por qué:
522 ms de parseo sincrónico. La palanca 3 de abajo lo resuelve y está en el
cierre de Sprint 1.

**Las cuatro palancas de arranque:**

1. **Nada de trabajo en el arranque que no sea pintar.** El índice del
   workspace, los recientes y el chequeo de actualizaciones van después del
   primer pintado, no antes.
2. **Fuentes cargadas de forma perezosa**: primero la del documento, la del
   código solo cuando aparece un bloque.
3. **Parseo incremental**: pintar lo visible en cuanto está listo, seguir
   parseando el resto mientras tanto. **Es la única pendiente que importa:
   vale 522 ms en un documento de 5 MB.**
4. **Índice persistido**, nunca reconstruido en el arranque.

## Notas sobre componentes opcionales

No cuentan contra el presupuesto del núcleo porque se descargan aparte:

| Componente | Peso estimado | Nota |
| --- | --- | --- |
| Corrector ortográfico (es + en) | ~2 MB en disco, ~4,5 MB en RAM | Por eso es aparte: el diccionario de inglés solo ya usa 4,5 MB de memoria con Hunspell |
| Mermaid nativo completo | 1,5–3 MB | Si alguna vez pasa de flowchart y secuencia |
| Exportador DOCX | Por medir | Debe aislarse si compromete el núcleo |

## Cómo reproducir estas mediciones

El prototipo trae un modo de medición que no depende de que nadie mire la
pantalla ni mueva el mouse. Compilar una vez y ejecutar el binario evita sumar
el tiempo de compilación a la apertura:

```powershell
cargo build --release
& ".\target\release\visor-md.exe" documento.md --bench=240
```

```powershell
& ".\target\release\visor-md.exe" documento.md --bench=0
```

```powershell
& ".\target\release\visor-md.exe" documento.md --bench=0 --exacto
```

Las mediciones se acumulan en memoria y se imprimen recién al salir. No es un
detalle: la primera versión las imprimía a medida que ocurrían y, con la salida
redirigida a un archivo, **cada `eprintln` costaba más que el trabajo que
pretendía cronometrar**. Llegó a reportar 1247 ms de primer pintado donde el
proceso entero tardaba 168 ms. Una herramienta de medición que se mide a sí
misma miente, y en este caso mintió por un factor de diez.
