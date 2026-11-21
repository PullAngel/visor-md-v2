# Evaluación de usabilidad, ronda 4 (21 de noviembre de 2026)

Misma evaluación, build del commit `ac303c6` (la ronda 3 más 15.12 a 15.15).
Informes completos:
[`evaluaciones/2026-11-21-ronda-4/informe-qa.md`](evaluaciones/2026-11-21-ronda-4/informe-qa.md) y
[`evaluaciones/2026-11-21-ronda-4/informe-usuario.md`](evaluaciones/2026-11-21-ronda-4/informe-usuario.md).

## Resultado frente a los criterios

| Criterio | Meta | Ronda 1 | Ronda 2 | Ronda 3 | Ronda 4 | ¿Se cumple? |
| --- | --- | --- | --- | --- | --- | --- |
| Hallazgos S1 | 0 | 1 grupo | 0 | 0 | **0** | Sí |
| Hallazgos S2 | 0 | 8 grupos | 5 | 5 | **3** (QA; la usuaria 0) | No |
| SUS | ≥ 68 | 35 | 67,5 | 75 | **75** | Sí |
| Tareas sin dificultad | ≥ 80 % | 40 % | 80 % | 75 % | **93 %** (14 de 15) | **Sí** |
| Nielsen, promedio | ≥ 3,5, ninguna < 3 | 2,9 | 3,6 | 3,6 | **3,6**, mínima 3 | Sí |
| Calificación UX (QA) | — | 6 | 7 | 7,3 | **7,5** | — |

Cuatro de los cinco criterios se cumplen. Falta uno: **0 hallazgos S2**. La
usuaria no reportó ninguno; los tres son de la QA.

## Los tres S2 de la QA y qué se hizo

| Id | Hallazgo | Estado |
| --- | --- | --- |
| QA-01 | La negrita sin selección inserta `**texto**` pegado al `#` y rompe el título | **Corregido**: el formato nuevo ya no se pega a un carácter vecino; prueba `la_negrita_sin_seleccion_no_se_pega_al_caracter_vecino_y_no_rompe_el_titulo` |
| QA-02 | Los tooltips, incluido el destino del enlace, salen lejos del puntero y tapan contenido | **Corregido** el del enlace: se dibuja pegado al puntero (y al pie solo con el foco de teclado). Los tooltips de la barra siguen bajo su botón |
| QA-03 | Íconos ambiguos: dos «H» y la paleta como «>_» | **Corregido** el de resaltar (ya no es una «H»); la paleta mantiene «>_» a propósito (línea de comandos) y su nombre es siempre «Buscar una acción» |

## Hallazgos de la usuaria (todos S3 o S4)

Símbolos crudos al editar y tabla como texto con barras; el `#` de «Título 1» en
la línea equivocada (puede ser la herramienta); la lupa busca solo en el
documento; sin guardado automático y el aviso es un punto pequeño; a 640 px con
vista dividida la tabla se superpone. Lo mejor: el índice, Copiar, las tareas,
Comparar, la chincheta, el subrayado y los diálogos.

## Hallazgos S3 de la QA que quedan

Contadores distintos entre modos (decisión de la fase 3: Editar cuenta lo
escrito); interruptores con textos inconsistentes; avisos que alarman («la
búsqueda cubre solo parte del contenido»); búsqueda que mezcla consulta,
contador e instrucciones; hover y pulsado poco visibles y «✓ Copiado» breve;
a 640×480 se pierden textos y se truncan pestañas, y maximizada el contenido
queda pegado a la izquierda; Comparar no sincroniza el plegado.

## Decisión

Los S2 de la QA están corregidos tras la ronda; se repite la evaluación para
confirmarlo (ronda 5). Mientras tanto la fase 15 sigue con los S3.
