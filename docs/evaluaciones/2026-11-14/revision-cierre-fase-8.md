# Revisión de cierre de la fase 8

Fecha: 16 de noviembre de 2026. Tarea 8.10 del plan de ejecución. Revisión
hecha sobre las imágenes de referencia de `tests/visual/referencia/` y el
código, con los criterios de WCAG 2.1 AA y una crítica de diseño. No sustituye
la prueba con tecnologías de apoyo reales (fase 9).

## Accesibilidad (WCAG 2.1 AA)

| # | Hallazgo | Criterio | Gravedad | Estado |
| --- | --- | --- | --- | --- |
| 1 | El tirador de la barra de desplazamiento medía 2,1:1 (día) y 2,5:1 (noche) contra el documento | 1.4.11 Contraste no textual | Mayor | **Corregido**: opacidad 180, ≥ 3:1 en ambos temas; la prueba `el_tirador_de_la_barra_se_distingue_del_documento` lo vigila |
| 2 | La interfaz se dibuja a mano y no publica un árbol de accesibilidad: un lector de pantalla no ve botones, pestañas ni documento | 4.1.2 Nombre, función, valor | Crítico | Abierto. Requiere integrar AccessKit; va como tarea propia después de la fase 12 |
| 3 | Los bordes de botones y paneles (`border`) tienen 1,4:1 contra el documento | 1.4.11 | Menor | Aceptado: ningún control depende solo del borde; el relleno, el texto y el foco los distinguen |
| 4 | Objetivos de la barra de unos 36 × 30 px y cabeceras de panel de 22 px | 2.5.5 (nivel AAA; 2.5.8 en WCAG 2.2) | Menor | Aceptado para escritorio con mouse; la zona activa de la barra de desplazamiento y de las casillas ya es más generosa que su dibujo |
| 5 | El foco de teclado se ve (subrayado de acento en la barra, anillo en los diálogos, fila resaltada en las listas) | 2.4.7 | Cumple | Verificar el orden completo con F6 en la fase 9 |
| 6 | Movimiento reducido: el hover y el ✓ de copiar pasan a estado final sin animar; el destello de pulsación se omite | 2.3.3 (AAA) | Cumple | Cubierto por pruebas de `UiMotion` y de `copied_look` |
| 7 | El estado de «Copiado» no depende solo del color: cambia el ícono (✓) y el texto | 1.4.1 Uso del color | Cumple | — |

### Contraste de texto

Ver la tabla de [`design.md`](../../design.md): todos los pares de texto cumplen
4,5:1 en ambos temas; el texto principal pasa de 13:1.

## Crítica de diseño

- **Funciona:** la jerarquía de la Bienvenida (título, un botón principal con
  borde de acento, dos secundarios) y el recuadro con el rótulo dentro del
  bloque; el lenguaje tenue junto a «Copiar» da contexto sin ruido.
- **Consistencia:** hover, pulsación y radio de 6 px ya son los mismos en barras,
  pestañas y filas. Quedan sin hover animado los botones de la Bienvenida
  (cambian de color al instante).
- **Oportunidades:**
  1. La tira de pestañas queda con solo el «+» cuando todos los documentos
     están en paneles (pendiente de la fase 11).
  2. El resalte del divisor junto al borde del panel enfocado se confunde con el
     contorno verde del panel (pendiente de la fase 11).
  3. En 640 px, «Notas que enlazan aquí» llena el ancho de las pestañas del panel
     lateral.
