# Cálculos: presupuesto de 7 MB y de arranque

Números estimados, no medidos. El propósito de este documento es fijar un
presupuesto y un plan para verificarlo, no prometer un resultado. La Fase 0 del
roadmap existe precisamente para reemplazar estas estimaciones por mediciones.

## Presupuesto de tamaño (binario, sin comprimir)

| Componente | Estimación | Notas |
| --- | --- | --- |
| Runtime de Rust + `std` | 0,3–0,8 MB | Base de cualquier binario Rust, ya con `panic=abort` y `strip` |
| Parser (`comrak`) | 0,3–0,6 MB | CommonMark + GFM |
| Layout de texto (`parley` + `swash`) | 0,8–1,5 MB | La parte más difícil de acotar |
| Dibujo 2D (`tiny-skia`) | 0,5–1,0 MB | Por software; Skia completo sería +5 MB y rompería el presupuesto |
| Resaltado de sintaxis | 0,3–1,5 MB | `syntect` con muchas gramáticas es pesado; `tree-sitter` con pocas gramáticas es más chico. A decidir |
| Chrome (toolkit o dibujo propio) | 0,5–2,0 MB | El rango más amplio; depende de la decisión de Fase 0 |
| Iconos, recursos, config | 0,2 MB | Iconos SVG propios, sin fuentes embebidas (se usan las del sistema) |
| **Total estimado** | **2,9–7,6 MB** | El techo superior roza o pasa el presupuesto |

**Lectura honesta.** El presupuesto es *plausible pero no holgado*. En el mejor
caso queda en ~3 MB, cómodo. En el peor, con Skia completo, muchas gramáticas
de sintaxis y un toolkit de chrome pesado, se pasa de 7 MB. Las tres palancas
para no pasarse, en orden de impacto:

1. **`tiny-skia` (software) en vez de Skia completo.** Sola, es la diferencia
   entre entrar y no entrar.
2. **Pocas gramáticas de sintaxis**, cargadas bajo demanda, en vez de las ~200
   de `syntect`. Se embeben las 15-20 más comunes; el resto, sin resaltar.
3. **Chrome dibujado a mano** si el toolkit elegido pesa demasiado.

Ninguna fuente se embebe: se usan las de Windows. Embeber una sola fuente
variable costaría 0,3–1 MB y se evita salvo que la Fase 0 muestre que hace
falta para consistencia visual.

## Presupuesto de arranque

| Hito | Objetivo | Referencia |
| --- | --- | --- |
| Ventana visible | < 200 ms | Tinta: <100 ms |
| Primer documento pintado | < 1 s para el caso común (documento de tamaño típico) | v1: 3–4 s |
| Documento de 20 MB usable | Progreso visible en < 300 ms, parseo en hilo aparte | v1: no lo maneja |

El objetivo no es batir los <100 ms de Tinta —eso exigiría renunciar a cosas
que la v2 sí quiere— sino bajar de forma clara y perceptible del piso de
WebView2 de la v1, que es lo que el usuario objetivo nota.

## Presupuesto de memoria

Sin cifra dura todavía. La meta cualitativa: un documento típico abierto no
debería costar más que unas decenas de MB de RAM, contra los cientos que
consume un webview con su proceso de GPU. Se mide en Fase 0.

## Plan de medición (Fase 0)

1. Prototipo mínimo: abrir un `.md`, parsear con `comrak`, dibujar encabezados,
   párrafos, listas y tablas con `parley` + `tiny-skia`. Sin Mermaid, sin
   matemática, sin chrome elaborado.
2. Medir: tamaño del binario con `strip` + `panic=abort` + `opt-level=z`;
   tiempo a ventana visible; tiempo a primer pintado con un documento de
   referencia; RAM en reposo con el documento abierto.
3. Repetir con el corpus de la v1: `tests/estres.md` (casos límite),
   `tests/security/conversion-defectuosa.md` (Markdown roto de un conversor).
4. Comparar contra la v1 y contra Tinta instalado.
5. Con esos números reales, decidir: backend de dibujo (software vs GPU),
   toolkit de chrome (sí o dibujo propio), estrategia de sintaxis.

Si la Fase 0 muestra que <7 MB no es alcanzable sin renunciar a algo esencial,
eso es un resultado válido y a tiempo: se replantea el presupuesto o el
alcance antes de haber construido nada grande encima.
