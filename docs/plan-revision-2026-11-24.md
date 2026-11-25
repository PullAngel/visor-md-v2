# Plan de la revisión del 24-11

Seis errores encontrados en la revisión manual de `VisorMD-v2-nuevo-3.exe`. Este
plan sigue al de [la revisión del 23-11](plan-revision-2026-11-23.md). La fase 21
(ortografía) queda **aplazada** por decisión del dueño: se retoma cuando haya una
versión casi final y se pueda medir cuánto pesa.

Las fases siguen la numeración anterior: 22 a 24. Las decisiones, desde D-57.

## Lo que dice el código (causa de cada error)

| # | Error | Causa encontrada | Dónde |
|---|---|---|---|
| 1 | Subrayar sobre un subrayado no crea nada o recolorea todo | `estudio::highlight` trata cualquier solapamiento como «cambiar el color de lo que ya existe» y no crea registro nuevo; cambia el color del subrayado **entero**, aunque la selección toque una sola letra | `src/estudio.rs:282` |
| 2 | Las marcas pisan media letra | El borde de la columna se calcula con una fórmula (`pane.x + text_left + ancho`) y no con donde termina el texto dibujado; la separación fija es 12 px | `src/main.rs:14677`, `src/marcadores.rs:13` |
| 3 | El texto de la marca se escribe en la barra de arriba | `note_edit` se pinta como un aviso en la barra (`Nota de la marca: …`); el primer clic abre la tarjeta y el segundo cae sobre otra cosa | `src/main.rs:20754`, `:11262` |
| 4 | El cursor sobre una marca con texto largo no muestra el texto | Por confirmar con una prueba: la tarjeta de ayuda existe (`margin_hover_card`), pero el dibujo solo se pide cuando algo más repinta, y puede estar saliéndose o quedando tapada | `src/main.rs:22305`, `:24404` |
| 5 | Barra, cinta y bandera se ven como punto | `draw_margin_tag` dibuja un punto fijo para **toda** forma clásica; la forma sí se guarda bien | `src/main.rs:8399` |
| 6 | Ctrl+V pega y «rehace» | No hay ninguna ruta de Rehacer en la tecla V. Lo que sí está mal: **Ctrl+Mayús+Z deshace** en Editar (ignora Mayús), y Rehacer solo existe como Ctrl+Y | `src/main.rs:17020` |

## Decisiones

- **D-57. Subrayar sobre subrayado funciona como en Word.** El color nuevo se aplica
  **solo al tramo seleccionado**. De lo ya subrayado que cae fuera de la selección
  queda lo que sobraba, con su color de antes (se parte en uno o dos pedazos). Si la
  selección contiene un subrayado entero, ese subrayado desaparece y lo reemplaza
  el nuevo. Si el color es el mismo, se unen en un solo subrayado. «Sin color» y
  «Quitar subrayado» obedecen la misma regla: quitan solo lo seleccionado.
- **D-58. Hueco entre texto y marca = ancho de una «a» del cuerpo del texto**, medido
  con la fuente y el tamaño reales (sigue el zoom). El borde del texto sale del
  texto dibujado, no de una fórmula.
- **D-59. La marca se edita en su sitio, con doble clic.** Un clic sigue abriendo la
  tarjeta; el doble clic abre el texto sobre la etiqueta misma. La barra de arriba
  deja de usarse para esto (Esc cancela, Enter guarda, igual que ahora).
  - Con espacio, la etiqueta se convierte en un cuadro del mismo ancho que crece
    **hacia abajo** con el texto (varias líneas), sin tocar nunca la columna de
    texto.
  - Con cinta o muesca (poco ancho), el cuadro de edición flota con el ancho de la
    tarjeta, a la altura de la marca.
  - El tope de 200 caracteres se mantiene y se muestra un contador.
  - Las etiquetas de abajo se corren mientras se edita.
- **D-60. Rehacer = Ctrl+Y y Ctrl+Mayús+Z, en todos los modos.** Ctrl+V solo pega.

## Fase 22 · Subrayado por tramos (error 1) — la más importante

El error toca datos del usuario, por eso va primero.

1. Reescribir `estudio::highlight` según D-57: para cada subrayado que se cruza,
   calcular el resto izquierdo y el derecho, crearlos como registros nuevos con el
   color y los extras del original (cada uno con su cita y su contexto propios) y
   crear el del tramo. Unir cuando el color coincide y los tramos se tocan.
2. Hacer lo mismo en `remove_highlights` (quitar solo lo seleccionado).
3. Mantener el aviso útil: «Subrayado en amarillo», «2 subrayados partidos»…
4. Pruebas (cada caso del informe):
   - subrayar un bloque que contiene a uno ya subrayado, mismo color y otro color;
   - una sola letra de un subrayado con otro color: quedan tres tramos;
   - varios subrayados previos dentro de un bloque grande;
   - selección que cruza el borde de un subrayado;
   - quitar solo una parte;
   - el archivo `.md` guardado y vuelto a abrir conserva los tramos y el texto no cambia.
5. Comprobar en pantalla (Leer, Editar y Comparar) con capturas de prueba.

## Fase 23 · Marcas del margen (errores 5, 2, 4 y 3)

Orden: del más simple al que cambia la interacción.

1. **Error 5.** Dibujar la forma real en la etiqueta: barra, cinta, punto y bandera,
   cada una reconocible a 22 px de alto. Prueba visual con las nueve formas.
2. **Error 2.** Tomar el borde derecho del texto dibujado de las líneas visibles
   (máximo), y la separación igual al ancho de una «a». La misma cuenta la usan el
   dibujo, el clic y el ancho disponible (`marcadores::available_width`). Prueba:
   ninguna etiqueta se superpone con el texto, con distintos anchos y zoom.
3. **Error 4.** Reproducirlo primero con una prueba (marca con nota de 200
   caracteres, cursor encima). Pedir el repintado al mover el cursor sobre una
   etiqueta, ajustar la tarjeta para que el texto largo se vea completo (ancho
   fijo, varias líneas, alto máximo con «…» al final si pasa de 10 líneas) y que
   no quede fuera de la ventana ni tapada por otra etiqueta.
4. **Error 3.** Doble clic según D-59:
   - detectar el doble clic sobre la misma etiqueta dentro del tiempo del sistema;
   - modo de edición dentro de la etiqueta, con el cursor visible, ajuste de línea,
     Enter guarda, Esc cancela, clic fuera guarda;
   - el texto se escribe con teclado e IME (acentos) y se puede pegar;
   - retirar `note_overlay` de la barra de arriba; el atajo y la fila «Texto de la
     marca…» de la tarjeta abren esta misma edición.
5. Re-bendecir las referencias visuales que cambien a propósito.

## Fase 24 · Rehacer y pegar (error 6)

1. Ctrl+Mayús+Z → rehacer en Editar y Comparar; Ctrl+Y se mantiene (D-60).
2. Prueba de teclas: Ctrl+V solo pega (el historial no cambia), Ctrl+Z deshace,
   Ctrl+Y y Ctrl+Mayús+Z rehacen, en Leer, Editar y Comparar.
3. Actualizar la ayuda de «Rehacer» a «Rehacer · Ctrl+Y / Ctrl+Mayús+Z».
4. Si con esto el dueño sigue viendo que Ctrl+V rehace, pedirle los pasos exactos
   (modo, qué había copiado, si pasó con el teclado o con un menú).

## Verificación de todo

- `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`,
  `cargo test visual`, `scripts/check-docs.ps1`.
- Ejecutable de prueba nuevo, sin pisar los anteriores.
- Lista para probar a mano: los seis casos del informe, tal como los describió.
- Un commit por fase, con la nota de lo que cambió y de cualquier desvío.

## Fuera de este plan

- Ortografía en español (fase 21): aplazada hasta tener una versión casi final.
- Nombre y logo: sigue en su propia sesión.
