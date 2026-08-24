# Comparación y decisión

## Matriz

| Criterio | A — Híbrida endurecida | B — Nativa con escalamiento | C — Workspace y confianza auditable |
| --- | --- | --- | --- |
| Arranque | Mejora percibida real; sin cambio en el piso de WebView2 | El único que puede acercarse a Tinta en el caso común | Sin mejora |
| Peso | Sin cambio relevante frente a la v1 | Menor en el caso común; sin cambio en el escalado | Sin cambio |
| Separación de procesos (seguridad) | Sí — la mejora más concreta de las tres | Sí, pero solo en el camino WebView2 | No |
| Confianza auditable | No incluida por defecto | No incluida por defecto | Sí — única que la tiene |
| Fidelidad de render | Completa, sin duplicar trabajo | Requiere mantener dos renderizadores | Completa, sin duplicar trabajo |
| Riesgo de ejecución | Medio | Alto | Bajo |
| Responde a la crítica de Tinta | Parcialmente (arranque percibido) | Sí, de raíz | No |
| Workspace y edición estructural | Sí | Sí (no es el foco, pero no lo excluye) | Sí — es el foco |

## La decisión no es elegir una

Ninguna de las tres, tomada sola, cumple todos los requisitos de
`02-requisitos.md` sin ceder en algo importante. Elegir una a secas sería
repetir el error que el pedido original pidió evitar: enamorarse de la
primera idea. La versión definitiva combina piezas de las tres, con una
apuesta puesta a prueba antes de comprometerse con ella.

## Recomendación

**Arquitectura base: Propuesta A**, la híbrida endurecida en dos procesos.
Es la que de verdad mejora la seguridad de la v1 de forma estructural (la
lección de Moji), con el riesgo de ejecución más razonable de las tres que
sí tocan la arquitectura de render.

**Se le suma, sin costo arquitectónico extra, la pieza central de la
Propuesta C**: la bitácora de confianza auditable en vez del interruptor
global de "carpetas de confianza", y el aislamiento de los motores de
diagramas en un contexto de navegación separado. Ninguna de las dos
requiere tocar el contrato IPC ni el proceso de render — se construyen
sobre la base de A sin fricción.

**La Propuesta B no se descarta — se pospone a evidencia.** Es la única
que responde de raíz a la comparación con Tinta, pero comprometerse con
ella sin datos reales sería la clase de decisión que este documento
existe para evitar. En vez de eso: un prototipo acotado, aislado del resto
del trabajo, que renderice en nativo solo encabezados, párrafos, listas y
tablas —sin Mermaid, sin KaTeX, sin HTML— y mida arranque y peso reales
contra la Propuesta A ya construida. Si la diferencia justifica mantener
dos renderizadores para siempre, se retoma como una fase posterior. Si no,
la Propuesta A con las piezas de C ya es, por sí sola, una mejora real y
medible sobre la v1 en los tres ejes que importaban: seguridad, producto y
percepción de velocidad.

## Qué significa esto en una frase

La v2 no promete ganarle a Tinta en la métrica que Tinta eligió. Promete
igualar o superar la seguridad de la v1 de forma estructural, no solo
declarativa, y ponerse a la altura de la profundidad de producto de
ThisIs-Developer e idea-multimarkdown — con la puerta abierta, y no
cerrada, a perseguir el rendimiento nativo si un prototipo real demuestra
que vale el costo de mantenerlo.
