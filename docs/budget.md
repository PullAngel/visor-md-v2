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
| **Checkpoint de recuperación actual** | **3.103.232 bytes, 2,960 MiB** |
| **Checkpoint de wikilinks y callouts** | **3.202.048 bytes, 3,054 MiB** |

Superar 8 MB exige medición, explicación y aprobación. El límite no se usa para
recortar seguridad, estabilidad, accesibilidad, Unicode o funciones esenciales.
Mermaid, correctores y otros componentes pesados no tienen permiso automático
para ampliar el núcleo.

### Checkpoint de wikilinks y callouts del 27 de agosto de 2026

El commit `79e6828` se compiló en Windows MSVC con el perfil release después de
agregar navegación de bóveda contenida y callouts nativos. La comprobación fue
de tamaño; no sustituye una nueva medición de arranque o QA visual.

| Medida | Resultado |
| --- | --- |
| Binario Windows | 3.202.048 bytes, 3,054 MiB |
| SHA-256 | `3C48257A1AD7A58802AA2F5E2561DFE6544A67E4ECE0499B260F249D6950504E` |
| Margen frente al límite deseado | 4,946 MiB |

El incremento respecto de la medición inicial del editor es 57.344 bytes. No
agrega dependencias ni capacidades de red: corresponde al modelo, navegación y
dibujo nativo incorporados en el ejecutable.

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

La recuperación ahora conserva además el texto UTF-8 abierto durante toda la
sesión para que selección, copia y edición futura partan de la fuente y no de
la vista. Eso agrega como máximo el tamaño del texto decodificado al consumo de
un documento. Es un coste deliberado de integridad de datos; la medición de
memoria debe repetirse antes de cerrar el Sprint 1 porque las cifras históricas
son anteriores a esa retención.

### Portapapeles de texto sin funciones de imagen

La integración de `arboard 3.6.1` se compiló con `default-features = false`.
La medición refleja el uso de texto para copia explícita; no incluye
decodificadores de imágenes ni una función de pegado.

| Medida | Resultado |
| --- | --- |
| Binario Windows | 3.019.264 bytes, 2,879 MiB |
| SHA-256 | `4C2641EA2A8DC62D127B47719B517CA544DCA15F4F9506D6BCDDF707A9B98139` |
| Variación frente al checkpoint anterior | +5.632 bytes, +0,19 % |
| Margen frente al límite deseado | 5,121 MiB |

El resultado es una medida de tamaño, no de arranque. El siguiente checkpoint de
rendimiento medirá el flujo de apertura asíncrona, porque mover el parser fuera
del hilo de interfaz cambia más la percepción que esta dependencia.

### Apertura primaria limitada

Extraer la lectura a la capa de archivos añadió validación del handle, tamaño y
UTF-8 sin dependencias nuevas.

| Medida | Resultado |
| --- | --- |
| Binario Windows | 3.021.312 bytes, 2,881 MiB |
| SHA-256 | `A3CD5DFDE3903876D6FDE79151845320599B427AF6BA9BCF55B68933A3FDD9C9` |
| Variación frente al checkpoint anterior | +2.048 bytes, +0,07 % |
| Margen frente al límite deseado | 5,119 MiB |

### Gate de lector y archivos del 27 de agosto de 2026

El gate completo verificó formato, Clippy, 58 pruebas, SBOM, documentación y
release tras copia, texto inerte, apertura limitada y preparación asíncrona.

| Medida | Resultado |
| --- | --- |
| Binario Windows | 3.031.040 bytes, 2,891 MiB |
| SHA-256 | `69119E4E3D5D93543BC35B0494140775E0917C3E6540DA930E2BC44CB91DE469` |
| Margen frente al límite deseado | 5,109 MiB |

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

### HEAD actual después de estabilizar el modelo

El commit `6176a82` se compiló de nuevo después de los cambios de modelo,
límites y separación modular. La validación usó `rustc 1.98.0`
(`x86_64-pc-windows-msvc`) y el mismo perfil release. No se reutilizó el binario
de `a54c9d6`.

| Medida | Resultado |
| --- | --- |
| Binario Windows | 2.996.736 bytes, 2,858 MiB |
| SHA-256 | `5BE94A1980BC4A042A760EAE7626BC8404067C92CE0BD909DBA26DD377A527A1` |
| Documento | `docs/architecture.md`, 11,0 KB, 214 bloques |
| Repeticiones consecutivas | 5 |
| Parseo | 2 a 4 ms |
| Ventana visible, mediana | 90 ms |
| Primer pintado, mediana | 103 ms |
| Scroll, mediana de promedios | 4,5 ms por cuadro |
| Outlier de primer pintado en la serie | 631 ms |

Las cinco medidas de primer pintado fueron 104, 87, 631, 103 y 99 ms. Cuatro
quedaron dentro del presupuesto de 400 ms y una reprodujo la variación de
creación de ventana observada antes. Todavía falta un protocolo que explique esa
variación en Windows. No se presenta la mediana como garantía de arranque frío.

### Allowlist HTML nativa

Tras agregar la representación nativa cerrada de `kbd`, `mark`, `sub` y `sup`,
además de `br`, se reconstruyó el perfil release desde el working tree. No se
tomó como medida de arranque: este build solo verifica el impacto de tamaño.

| Medida | Resultado |
| --- | --- |
| Binario Windows | 3.000.320 bytes, 2,861 MiB |
| SHA-256 | `A869BBBA56E286C9B218C6CA0B290CCC4663EEF29B0A432757CD2E9C12106DEA` |
| Variación frente a `6176a82` | +3.584 bytes |
| Margen frente al límite deseado | 5,139 MiB |

El aumento es menor que 0,2 % y procede de lógica y dibujo ya presentes, sin
agregar dependencias, fuentes ni capacidades de red o disco. La evidencia de
arranque anterior permanece vigente para el comportamiento no modificado; este
cambio todavía requiere QA visual de ambos temas.

### Selección, teclado, autoscroll y Ctrl+A

La selección inicial usa el mismo layout de Parley que determina las líneas y
los glifos. Esto evita una segunda geometría aproximada: el rectángulo pintado
corresponde a lo que la persona ve, incluso cuando el texto se ajusta de línea.
También se pinta un cursor fino cuando la selección está colapsada, para que las
flechas no muevan un foco invisible. Shift+flechas conserva el ancla y extiende
el foco, sin usar un modelo de coordenadas paralelo. Al arrastrar cerca de un
borde, el scroll avanza en pasos acotados y vuelve a calcular el foco con el
layout visible. Ctrl+A crea una selección del documento completo sin requerir
portapapeles ni capacidades adicionales. Las flechas verticales consultan el
layout para saltar entre líneas envueltas, en vez de calcular posiciones por
cantidad de caracteres. El cursor de mouse usa la misma prueba de impacto que
la selección y no habilita interacción sobre contenido no seleccionable.

| Medida | Resultado |
| --- | --- |
| Binario Windows | 3.013.632 bytes, 2,874 MiB |
| SHA-256 | `89E29E2411E5FE5CE46763F6C36BD6CC9336BB91A999EB69F4F53D4E685C14A8` |
| Variación frente a la verificación anterior | 0 bytes |
| Margen frente al límite deseado | 5,126 MiB |

No se agregaron dependencias ni capacidades nuevas. La medición verifica tamaño,
no reemplaza la QA manual de selección, contraste y comportamiento en pantallas
con distintas escalas. Los hashes difieren porque las compilaciones no son aún
reproducibles bit a bit; el tamaño idéntico confirma que la ampliación no
introdujo una regresión medible en el presupuesto.

### Serie automatizada inicial

El reporte versionado
[`benchmarks/2026-08-26-windows.json`](benchmarks/2026-08-26-windows.json) se
generó en `f7caff8` con diez procesos consecutivos y el working tree rastreado
limpio. El ejecutable es el mismo build release de `92eef4f`.

| Medida | Mediana | P95 | Máximo |
| --- | --- | --- | --- |
| Parseo | 2 ms | 17 ms | 17 ms |
| Ventana visible | 89 ms | 600 ms | 600 ms |
| Primer contenido | 102,5 ms | 612 ms | 612 ms |

El scroll de 240 cuadros promedió 4,4 ms. Nueve aperturas entregaron contenido
entre 97 y 134 ms; una tardó 612 ms. El P95 usa nearest rank, por lo que en una
serie de diez conserva el peor valor. La evidencia apunta a creación de ventana
y no al parser, pero todavía no demuestra su causa.

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

Para una serie con muestras crudas, mediana y percentil 95:

```powershell
.\scripts\benchmark-startup.ps1 -Document .\docs\architecture.md -Runs 10
```

El reporte incluye commit, estado del working tree, toolchain, tamaño y hash del
ejecutable. `cacheState` y `systemLoad` quedan declarados como no controlados; la
herramienta hace repetible la recolección, pero no convierte una serie cálida en
una medición de arranque frío.
