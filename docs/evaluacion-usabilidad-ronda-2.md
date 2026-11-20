# Evaluación de usabilidad, ronda 2 (20 de noviembre de 2026)

Repite la evaluación del 14 de noviembre
([`evaluacion-usabilidad-2026-11-14.md`](evaluacion-usabilidad-2026-11-14.md))
con los mismos prompts, una QA de UI/UX y una usuaria novata, cada una con su
copia de la build de evaluación y un perfil vacío. La build es la del commit
`8f8cf53` (incluye las fases 6 a 8, 10 a 14). Informes completos:
[`evaluaciones/2026-11-20/informe-qa.md`](evaluaciones/2026-11-20/informe-qa.md) y
[`evaluaciones/2026-11-20/informe-usuario.md`](evaluaciones/2026-11-20/informe-usuario.md).

## Resultado frente a los criterios

| Criterio (9.4) | Meta | Ronda 1 | Ronda 2 | ¿Se cumple? |
| --- | --- | --- | --- | --- |
| Hallazgos S1 abiertos | 0 | 1 grupo (buscadores sin texto) | **0** | Sí |
| Hallazgos S2 abiertos | 0 | 8 grupos | **3 de la QA + 2 de la usuaria** | No |
| SUS (usuaria) | ≥ 68 | 35 | **67,5** | No, por 0,5 |
| Tareas sin dificultad | ≥ 80 % | 40 % (6 de 15) | **80 %** (12 de 15) | Sí |
| Nielsen, promedio | ≥ 3,5, ninguna < 3 | 2,9 | **3,6**, la más baja 3 | Sí |
| Calificación UX (QA) | — | 6 / 10 | **7 / 10** | — |
| Tareas completadas | — | 14 / 15 | **15 / 15** | — |

El producto mejoró en todos los indicadores: nadie pierde datos ni se bloquea,
se completan las 15 tareas y el SUS pasó de «pobre» a un paso del umbral. No
alcanza el cierre de la fase porque quedan hallazgos S2 y el SUS no llega a 68.
**Se abre la fase 15** con los hallazgos nuevos (regla 9.4).

## Qué se resolvió desde la ronda 1

Las dos evaluaciones lo señalan como bueno: Comparar, el índice del documento y
el índice flotante, la lista que continúa sola, la chincheta con su aviso, el
cambio de tema, la exportación a PDF, Copiar con confirmación en el propio
botón, los diálogos propios (Esc siempre elige lo seguro), la adaptación a
640×480 y la paleta filtrable de 77 acciones. La búsqueda ya se puede escribir
(era S1), las listas se desplazan y se pueden navegar, las inserciones de bloque
se aíslan y los conteos de Editar se actualizan.

## Hallazgos S2 abiertos

| Id | Hallazgo | Fuente |
| --- | --- | --- |
| QA-01 | Dividir con un documento en Editar muestra la fuente como bloques de código con «Copiar» en cada línea, y el selector dice «Leer» | QA |
| QA-02 | El enlace roto casi no se distingue del válido y el aviso no dice que esté roto | QA |
| QA-03 | Buscar (en el documento y en la carpeta) no tiene campo de texto visible, ni «n de m», ni anterior/siguiente | QA |
| US-01 | Tildar una tarea deja el apunte «sin guardar» sin ningún aviso | usuaria |
| US-05 | En el editor se ven `#`, `- ` y `**`; un `#` quedó suelto al aplicar Título 1 en una nota vacía y un icono con aspecto de lista insertó una cita | usuaria |

## Hallazgos S3 principales

Avisos redundantes o en jerga («presupuesto de memoria»), teclado y foco
(Tab, F6, foco casi invisible), tooltips lejos del puntero, el botón de la
paleta («>_», nombre cambiante, ícono ≡ repetido), íconos de la segunda barra
sin nombre (el índice se encuentra por prueba y error), contraste de íconos
(2,0 a 4,1:1) y de texto secundario (por debajo de lo que pide N5), la pestaña
«Notas que enlazan aquí» desbordada, conteos distintos entre Leer y Editar,
columna pegada a la izquierda con la ventana maximizada, resultado de búsqueda
cortado que no lleva a la frase, y las pestañas que quedan en un panel dividido
desaparecen de la barra.

## Heurísticas de Nielsen (QA)

Promedio **2,9 → 3,6**. En la ronda 1 las más bajas eran Mundo real (2),
Consistencia (2) y Errores (2); hoy son 4, 3 y 4. La más baja de la ronda 2 es
3 (Consistencia y Reconocer antes que recordar); la más alta, Control y
libertad (4,5). El detalle por heurística está en el informe de la QA.

## No se pudo verificar

«Siempre encima», Movimiento reducido, dividir hasta cuatro paneles, fijar una
pestaña, el menú contextual de tabla, la confirmación previa a abrir enlaces
web y un cierre brusco (R21). La usuaria no abrió el PDF exportado. Quedan para
la prueba manual.

## Decisión

No se cumplen dos criterios (S2 abiertos y SUS), por lo que se agrega la fase 15
al plan y se repite la evaluación al cerrarla. La mayor parte del SUS que falta
está en el mismo lugar que los S2: ver los símbolos de Markdown al editar, no
saber si lo que hizo se guardó y no encontrar el índice y la búsqueda.
