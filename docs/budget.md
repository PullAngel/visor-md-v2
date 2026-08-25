# Presupuesto de tamaño y de arranque

Números estimados, no medidos. Este documento fija el presupuesto y el plan
para verificarlo; el Sprint 0 lo reemplaza por mediciones reales.

## Los tres números

| | Valor |
| --- | --- |
| **Ideal** | < 6 MB |
| **Objetivo de trabajo** | < 7 MB |
| **Techo duro** | 9,44 MB |

Se trabaja como si el límite fuera 7. Pasarlo exige una razón fuerte y anotada;
9,44 no se cruza nunca.

**La razón fuerte ya identificada:** Mermaid. Dijiste que es de las pocas cosas
por las que subirías el límite, y coincido: es lo que distingue un lector
técnico de uno de texto plano. Si el render nativo de flowchart y secuencia
entra en 1–1,5 MB, vale gastar hasta ~8,5 MB. Sigue por debajo del techo.

## Presupuesto de tamaño

| Componente | Estimación |
| --- | --- |
| Runtime de Rust + `std` | 0,3–0,8 MB |
| Parser (`comrak`) | 0,3–0,6 MB |
| Layout de texto (`parley` + `swash`) | 0,8–1,5 MB |
| Dibujo 2D (`tiny-skia`) | 0,5–1,0 MB |
| Resaltado de sintaxis (15–20 lenguajes) | 0,3–0,8 MB |
| Fuentes embebidas (3 subconjuntos latinos) | ~0,5 MB |
| Índice del workspace (SQLite, si entra) | 0–1,0 MB |
| Chrome | 0,5–2,0 MB |
| Iconos y recursos | 0,1 MB |
| **Total estimado** | **3,3–8,3 MB** |

**Lectura honesta:** el mejor caso queda cómodo en el ideal de 6 MB. El peor se
pasa de 7 pero sigue bajo el techo de 9,44. Las palancas, en orden de impacto:

1. **`tiny-skia` (software) en vez de Skia completo.** Sola, es la diferencia
   entre entrar y no entrar: Skia son más de 5 MB.
2. **Índice en JSON en vez de SQLite**, si el tamaño de bóveda esperado lo
   permite. Ahorra hasta 1 MB a costa de una búsqueda más lenta.
3. **Chrome dibujado a mano** si el toolkit elegido pesa demasiado.
4. **Bajar a dos familias tipográficas.** Ahorra ~200 KB; es la última palanca
   porque toca la identidad.

Ninguna fuente del sistema se embebe: solo los tres subconjuntos propios.

## Presupuesto de arranque

Dijiste que el arranque importa tanto como el peso. De acuerdo: es lo primero
que se nota y lo que decide si queda como predeterminado.

| Hito | Objetivo | Referencia |
| --- | --- | --- |
| Ventana visible | < 150 ms | Tinta: <100 ms · v1: ~2 s |
| Primer documento pintado | < 400 ms | v1: 3–4 s |
| Documento de 20 MB usable | Progreso visible < 300 ms | v1: no lo maneja |

**Las cuatro palancas de arranque:**

1. **Nada de trabajo en el arranque que no sea pintar.** El índice del
   workspace, los recientes y el chequeo de actualizaciones van después del
   primer pintado, no antes.
2. **Fuentes cargadas de forma perezosa**: primero la del documento, la del
   código solo cuando aparece un bloque.
3. **Parseo incremental**: pintar lo visible en cuanto está listo, seguir
   parseando el resto mientras tanto.
4. **Índice persistido**, nunca reconstruido en el arranque.

## Memoria

Meta cualitativa: un documento típico abierto no debería costar más que unas
decenas de MB, contra los cientos de un webview con su proceso de GPU. Se mide
en el Sprint 0.

## Notas sobre componentes opcionales

No cuentan contra el presupuesto del núcleo porque se descargan aparte:

| Componente | Peso estimado | Nota |
| --- | --- | --- |
| Corrector ortográfico (es + en) | ~2 MB en disco, ~4,5 MB en RAM | Por eso es aparte: el diccionario de inglés solo ya usa 4,5 MB de memoria con Hunspell |
| Mermaid nativo completo | 1,5–3 MB | Si alguna vez pasa de flowchart y secuencia |
| IA local | Cientos de MB, o 0 hablando con un Ollama existente | Ver `inference.md` |

## Plan de medición (Sprint 0)

1. Prototipo mínimo: abrir, parsear, dibujar encabezados, párrafos, listas y
   tablas.
2. Medir tamaño con `strip` + `panic=abort` + `opt-level=z`, arranque, primer
   pintado y RAM.
3. Repetir con el corpus de la v1: casos límite y Markdown de conversión
   defectuosa.
4. Comparar contra la v1 y contra Tinta instalado.
5. Decidir backend de dibujo, toolkit de chrome, almacenamiento del índice.

Si el Sprint 0 muestra que no se llega sin renunciar a algo esencial, eso es un
resultado válido y a tiempo: se replantea antes de construir nada encima.
