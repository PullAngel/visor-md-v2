# QA manual del Sprint 1

Esta lista comprueba propiedades visuales y de interacción que las pruebas de
píxeles no pueden juzgar por sí solas. No convierte una impresión informal en
evidencia: se registra plataforma, escala, commit y resultado.

## Preparación

```powershell
.\scripts\check.ps1
& ".\target\release\visor-md.exe" ".\tests\fixtures\sprint1-visual.md"
```

Registrar antes de comenzar:

- commit;
- versión de Windows o distribución Linux;
- resolución y escala de pantalla;
- tema del sistema;
- fecha y persona que revisa.

## Lectura y composición

- El título, cuerpo y código usan familias distinguibles y legibles.
- Negrita, cursiva, combinación anidada y tachado se distinguen sin deformar la
  altura de línea.
- `kbd`, `mark`, subíndice y superíndice se distinguen sin desplazar ni tapar
  el texto vecino; una etiqueta HTML con atributo se ve como fuente inerte.
- Los caracteres árabes, devanagari, japoneses, coreanos y emoji no desaparecen
  ni se convierten en cuadros vacíos; registrar fuente fallback visible y si la
  dirección de escritura árabe conserva un orden legible.
- Las líneas largas ajustan sin cortar glifos ni salir del margen.
- La tabla presenta una celda por columna, sin caracteres `|` visibles dentro
  del contenido, con bordes tenues, encabezado distinguible y alineaciones
  izquierda, centro y derecha cuando la sintaxis GFM las declara. Probar al
  menos una celda suficientemente larga para ajustarse en varias líneas.

## Estructuras

- Las listas numeradas mantienen su número y las viñetas quedan alineadas.
- Una línea envuelta dentro de una lista comienza bajo el texto, no bajo el
  marcador.
- Las tareas abiertas y completadas se distinguen en ambos temas. Un clic sobre
  la casilla cambia solo `[ ]`/`[x]` de la fuente, marca el documento como
  modificado y `Ctrl+Z` revierte ese cambio desde lectura.
- Las citas anidadas muestran profundidad sin consumir un ancho excesivo.
- La regla horizontal es visible pero no domina la página.
- HTML no permitido aparece como fuente inerte y no cambia la interfaz.

## Ventana e interacción

- La primera apertura comienza en modo lectura.
- Un documento que excede un límite muestra una banda superior discreta de
  "Modo seguro" y conserva su fuente visible e inerte; la banda no tapa la
  primera línea del documento.
- La tecla `T` alterna tema sin parpadeo ni pérdida de posición.
- Redimensionar angosto, mediano y maximizado no rompe layout ni scroll. Repetir
  al mover la ventana entre escalas de pantalla distintas si hay más de un
  monitor; el cuerpo, los márgenes, las sangrías y las casillas deben conservar
  el mismo tamaño lógico percibido. Registrar aparte si un control flotante
  (banda de modo seguro o menú contextual) se percibe desproporcionado.
- La rueda llega al principio y al final sin zonas inaccesibles.
- Texto, enlaces, decoración y casillas conservan contraste suficiente.
- La sensación de scroll es estable, sin saltos perceptibles.
- Arrastrar cerca de un borde desplaza el documento de manera controlable y
  conserva la selección; dentro de la pantalla, esta coincide con los glifos y
  sus líneas envueltas.
- El cursor de mouse cambia a texto sobre contenido seleccionable y vuelve al
  cursor habitual al salir de él.
- Un clic muestra un cursor fino; las flechas lo desplazan dentro del bloque,
  incluidas las líneas envueltas. Shift+flechas extiende la selección y Escape
  elimina el cursor o selección.
- Inicio y Fin llevan el cursor al borde del bloque; Ctrl+Inicio y Ctrl+Fin al
  comienzo y final del documento. Shift conserva el ancla para seleccionar.
- RePág y AvPág desplazan aproximadamente una pantalla sin perder la selección.
- Tab y Shift+Tab recorren enlaces en ambos sentidos, hacen visible el destino
  real y dejan una señal de foco distinguible del hover.
- Enter sobre un enlace web o de correo enfocado lo delega al navegador o
  aplicación del sistema. Un enlace relativo avisa que requiere VFS; una ruta
  bloqueada no se abre.
- Ctrl+A abarca todo el documento; `Ctrl+C` pega en otra aplicación el texto
  visible conservando viñetas, numeración, casillas y citas cuando los bloques
  entraron completos. `Ctrl+Shift+C` pega el Markdown original de los bloques
  seleccionados, incluso si la selección visual solo tomó una parte de ellos.
- Con texto seleccionado, click derecho abre un menú propio con solo "Copiar
  texto" y "Copiar Markdown original". Probar ambas acciones en otra
  aplicación; sin selección, el menú no aparece ni hace otra operación.
- Si se inicia una selección y el cursor o foco salen de la ventana, el
  arrastre se detiene sin borrar la selección ya conseguida.

## Edición de fuente inicial

- Pulsar `F2` muestra la fuente Markdown como texto inerte; escribir texto con
  teclado o IME, Backspace, Delete, Ctrl+Z y Ctrl+Y no debe cerrar la ventana.
- Pulsar `F2` otra vez vuelve a lectura y muestra el contenido modificado, sin
  superponer bloques aunque se haya pegado o escrito una cantidad grande de
  texto. `Ctrl+Z` desde lectura también actualiza la vista y conserva la
  posibilidad de volver a entrar en edición.
- Para comprobar un conflicto de guardado, abrir una copia de la fixture,
  editarla en Visor MD, modificarla también con un editor externo y pulsar
  `Ctrl+S`. Confirmar que el archivo externo no se sobrescribe y que el diálogo
  explica las tres acciones. Probar Cancelar; para Recargar, verificar que la
  recuperación queda disponible con `Ctrl+Shift+R`; para Guardar una copia,
  elegir un destino nuevo y verificar que el original externo no cambia.
- Esta comprobación no autoriza guardar: cursor visible, selección por mouse,
  navegación vertical y guardado siguen fuera de esta etapa.

## Registro

Para cada defecto anotar:

1. sección de la fixture;
2. resultado esperado y observado;
3. tema, resolución y escala;
4. captura si el problema es visual;
5. si se reproduce después de cerrar y volver a abrir.

La revisión queda `Pendiente` hasta completar toda la lista. Un defecto se
convierte en test automatizado cuando su propiedad puede comprobarse sin juicio
estético.

## Ronda del 27 de agosto de 2026

La primera ejecución de esta lista encontró defectos reales en negrita, emoji,
escritura de teclado, profundidad visual de citas y copia estructurada. Se
corrigieron en el siguiente bloque de trabajo con regresiones automáticas para
la clave de caché de fuentes variables, composición RGBA de fallback, sangría
de citas y copia legible. Falta repetir esta lista con el ejecutable release
posterior a esas correcciones antes de cerrar Sprint A.

La acción visible de copiar un bloque de código sigue registrada como trabajo de
"Lector completo". Las tablas ya se representan con celdas y bordes nativos;
selección y copia TSV de celdas quedan pendientes. En ambos casos, el contenido
permanece inerte y la fuente se conserva sin activar recursos.
