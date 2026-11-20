# Evaluación de usabilidad, ronda 3 (20 de noviembre de 2026)

Repite la evaluación con los mismos prompts sobre la build del commit `8d037f2`
(la ronda 2 más las correcciones de la fase 15). Informes completos:
[`evaluaciones/2026-11-20-ronda-3/informe-qa.md`](evaluaciones/2026-11-20-ronda-3/informe-qa.md) y
[`evaluaciones/2026-11-20-ronda-3/informe-usuario.md`](evaluaciones/2026-11-20-ronda-3/informe-usuario.md).

## Resultado frente a los criterios

| Criterio | Meta | Ronda 1 | Ronda 2 | Ronda 3 | ¿Se cumple? |
| --- | --- | --- | --- | --- | --- |
| Hallazgos S1 | 0 | 1 grupo | 0 | **0** | Sí |
| Hallazgos S2 | 0 | 8 grupos | 5 | **5** (3 QA + 2 usuaria) | No |
| SUS | ≥ 68 | 35 | 67,5 | **75** | **Sí** |
| Tareas sin dificultad | ≥ 80 % | 40 % | 80 % | **75 %** (12 de 16) | No, por una tarea |
| Nielsen, promedio | ≥ 3,5 | 2,9 | 3,6 | **3,6** | Sí |
| Calificación UX (QA) | — | 6 | 7 | **7,3** | — |
| Tareas completadas | — | 14 de 15 | 15 de 15 | 14 de 16 | — |

El SUS supera el umbral por primera vez. Siguen abiertos los S2 y las tareas
sin dificultad quedan en 75 %; **no se cierra la fase 9**.

## Hallazgos S2 de la ronda 3

| Id | Hallazgo | Estado |
| --- | --- | --- |
| QA-01 | La segunda barra cambia según el modo: en Leer son paneles; en Editar y Comparar, formato. Un clic pensado para abrir el Índice aplicó negrita | Nuevo; a la fase 15 |
| QA-02 | Con la ventana dividida la barra de pestañas queda vacía | **Repetido de la ronda 2 (15.8, diferida)**; a hacer |
| QA-03 | El campo de búsqueda flotaba sobre el texto y tapaba la coincidencia activa | **Corregido** tras la ronda: ahora ocupa la segunda barra |
| US-02 | Negrita sin selección: queda `texto` seleccionado entre `**`; Enter ahí lo reemplaza y rompe la negrita | Es el comportamiento diseñado (el rótulo queda seleccionado para escribir encima); se evaluará un aviso |
| US-01 | Un `#` suelto tras aplicar título desde el menú Párrafo (el informe dice que podría ser la herramienta de escritura) | Se vuelve a probar a mano |

## Lo que mejoró desde la ronda 2

- Los 3 S2 de la QA de la ronda 2 que se atacaron (panel dividido desde
  Comparar, enlace roto, campo de búsqueda) ya no aparecen como tales, salvo el
  campo flotante, que se movió.
- La usuaria llegó al Capítulo 30 en 4 acciones (índice con ícono propio) y
  exportó el PDF sin ayuda; Copiar, Comparar, dividir, el pin y el diálogo de
  cerrar sin guardar siguen funcionando.
- Los subrayados de estudio y las marcas se usaron por primera vez en una
  evaluación: «Subrayar» exige selección y su aviso no lo explica (a la 15), y
  «Estudio» no está a la vista en Leer (se llega por clic derecho).

## Hallazgos S3 principales (a la fase 15)

Avisos repetitivos («Vista de lectura», «Pestaña cerrada»); un clic sobre otro
título de menú con uno abierto solo lo cierra; «✓ Copiado» dura menos de
0,3 s; el borde de las casillas de tarea vacías tiene contraste de 1,4:1; sin
hover en tareas ni en las flechas de plegado; un clic en un enlace entre notas
sin carpeta no responde; documento vacío en Leer es una pantalla en blanco;
foco de teclado casi invisible; íconos duplicados (dos «H», ▷ para plegar);
la tabla se inserta como texto crudo; el amarillo de estudio se ve verde
oliva en noche; el aviso «la búsqueda cubre solo parte del contenido» confunde;
los resultados de la búsqueda en carpeta siguen cortados.

## Decisión

Se agregan 15.12 a 15.15 al plan, con prioridad en la pestaña del panel
dividido (dos rondas seguidas) y en la segunda barra por modo. Se repite la
evaluación al cerrarlas.
