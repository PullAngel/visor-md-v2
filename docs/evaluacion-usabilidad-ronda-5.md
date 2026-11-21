# Evaluación de usabilidad, ronda 5 (21 de noviembre de 2026)

Build del commit `6746f23`. Informes:
[`evaluaciones/2026-11-21-ronda-5/informe-qa.md`](evaluaciones/2026-11-21-ronda-5/informe-qa.md) y
[`evaluaciones/2026-11-21-ronda-5/informe-usuario.md`](evaluaciones/2026-11-21-ronda-5/informe-usuario.md).

| Criterio | Meta | Ronda 3 | Ronda 4 | Ronda 5 | ¿Se cumple? |
| --- | --- | --- | --- | --- | --- |
| S1 | 0 | 0 | 0 | **0** | Sí |
| S2 | 0 | 5 | 3 | **3** (1 QA, 2 usuaria) | No |
| SUS | ≥ 68 | 75 | 75 | **67,5** | No, por 0,5 |
| Tareas sin dificultad | ≥ 80 % | 75 % | 93 % | **81 %** (13 de 16) | Sí |
| Nielsen | ≥ 3,5, mínima 3 | 3,6 | 3,6 | **4,0** | Sí |
| Calificación UX (QA) | — | 7,3 | 7,5 | **7,5** | — |

El SUS de la usuaria oscila entre 67,5 y 75 con la misma versión: es una sola
persona simulada por ronda, así que el umbral de 68 cae dentro del ruido.

## S2 y qué se hizo después

| Hallazgo | Respuesta |
| --- | --- |
| QA-01: con muchas pestañas o a 640 px los nombres quedan ilegibles; todas las nuevas se llaman «sin título» | Los documentos nuevos se numeran («sin título 2»…), y el cálculo de caracteres por pestaña reserva más lugar para la marca y la ✕ |
| US-01: la negrita sin selección se rompe con Enter | Con el rótulo «texto» de un formato recién puesto seleccionado, Enter sale del formato (cursor tras el cierre) en lugar de reemplazar el rótulo; prueba `enter_con_el_rotulo_de_un_formato_seleccionado_sale_del_formato_sin_partirlo` |
| US-02: la lupa busca solo en el documento | El campo de búsqueda lo dice y recuerda Ctrl+Shift+F para toda la carpeta |

Otros S3 atendidos: el mensaje de «Abrir recuperación» ya no contradice lo que se
ve («No hay copias de una sesión anterior… lo que escribís ahora ya se copia
solo»).

## Quedan (S3)

Negrita sin selección expone Markdown; hover de Copiar poco visible; índice
flotante desborda su tarjeta y no aparece con paneles divididos; enlace roto sin
carpeta se ve igual que uno válido; Leer, Editar y Comparar desaparecen de los
paneles a 640 px; error de apertura como párrafo; menú Estudio de más de 20
entradas; búsqueda en carpeta con título y fragmento cortado.
