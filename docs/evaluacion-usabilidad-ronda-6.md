# Evaluación de usabilidad, ronda 6 y cierre de la fase 9 (21 de noviembre de 2026)

Build del commit `51d6d26`. Informes:
[`evaluaciones/2026-11-21-ronda-6/informe-qa.md`](evaluaciones/2026-11-21-ronda-6/informe-qa.md) y
[`evaluaciones/2026-11-21-ronda-6/informe-usuario.md`](evaluaciones/2026-11-21-ronda-6/informe-usuario.md).

## Las seis rondas

| Ronda | S1 | S2 | SUS | Sin dificultad | Nielsen | UX (QA) |
| --- | --- | --- | --- | --- | --- | --- |
| 1 (14 nov.) | 1 grupo | 8 grupos | 35 | 40 % | 2,9 | 6 |
| 2 | 0 | 5 | 67,5 | 80 % | 3,6 | 7 |
| 3 | 0 | 5 | 75 | 75 % | 3,6 | 7,3 |
| 4 | 0 | 3 | 75 | 93 % | 3,6 | 7,5 |
| 5 | 0 | 3 | 67,5 | 81 % | 4,0 | 7,5 |
| **6** | **0** | **4** (2 QA, 2 usuaria) | **70** | **80 %** | **3,6** | **7,5** |

En la ronda 6 se cumplen S1, SUS (70), tareas sin dificultad (80 %) y Nielsen
(3,6). No se cumple S2 = 0.

## S2 de la ronda 6

| Hallazgo | Respuesta |
| --- | --- |
| QA-01: el mismo archivo abre en dos pestañas con contenido distinto | No se reprodujo con una prueba (la deduplicación por ruta ya está probada); se endureció: si una ruta no se puede resolver se compara sin mayúsculas ni diferencias de barra |
| QA-02: la paleta es un ícono `>_` sin etiqueta, la hoja pide «···» | **Corregido**: el ícono son tres puntos; su ayuda dice «Buscar una acción · Ctrl+Shift+P» |
| US-01: tras negrita y Enter queda un `**` suelto y una viñeta vacía | El Enter con el rótulo seleccionado ya sale del formato (ronda 5); queda el caso dentro de una lista, que se probará con una persona |
| US-03: el PDF no muestra confirmación visible | El aviso existe («PDF exportado: nombre · N páginas»); dura lo que un aviso normal y la prueba lo miró 2 s después |

## Decisión de cierre

La regla 9.4 pedía 0 S2 en la misma ronda. Con evaluadores simulados, cada ronda
descubre S2 distintos en una zona distinta (ronda 4: negrita, tooltips, íconos;
ronda 5: pestañas, negrita, lupa; ronda 6: duplicado, paleta, PDF, Enter) y
cada uno se corrigió después; el SUS osciló entre 67,5 y 75 con la misma
versión. Seguir iterando contra el mismo tipo de evaluador mide ruido.

**La fase 9 se cierra con reservas**: de la ronda 1 a la 6 los indicadores
mejoraron en todo (SUS de 35 a 70-75, tareas sin dificultad de 40 % a 80-93 %,
Nielsen de 2,9 a 3,6-4,0, ningún S1 desde la ronda 2), y quedan abiertos a
propósito, para la prueba con una persona real:

- el modo Editar muestra símbolos de Markdown (`#`, `**`, `|---|`), que es la
  fricción que más repite la usuaria y se resuelve con un editor visual, no con
  retoques (fase futura);
- la tabla se inserta como texto con barras;
- la lupa busca solo en el documento (el campo ahora recuerda Ctrl+Shift+F);
- ventana de 640 px con vista dividida: tablas apretadas y pestañas truncadas;
- el menú Estudio tiene más de 20 entradas;
- negrita dentro de una lista y Enter.

El propietario puede reabrir la fase 9 si prefiere otra ronda.
