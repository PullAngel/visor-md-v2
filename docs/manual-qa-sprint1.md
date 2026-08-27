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
- Los caracteres no latinos y emoji no desaparecen ni se convierten en cuadros
  vacíos.
- Las líneas largas ajustan sin cortar glifos ni salir del margen.
- La tabla conserva columnas comprensibles y no oculta contenido.

## Estructuras

- Las listas numeradas mantienen su número y las viñetas quedan alineadas.
- Una línea envuelta dentro de una lista comienza bajo el texto, no bajo el
  marcador.
- Las tareas abiertas y completadas se distinguen en ambos temas.
- Las citas anidadas muestran profundidad sin consumir un ancho excesivo.
- La regla horizontal es visible pero no domina la página.
- HTML no permitido aparece como fuente inerte y no cambia la interfaz.

## Ventana e interacción

- La primera apertura comienza en modo lectura.
- La tecla `T` alterna tema sin parpadeo ni pérdida de posición.
- Redimensionar angosto, mediano y maximizado no rompe layout ni scroll.
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
- Ctrl+A abarca todo el documento; `Ctrl+C` pega en otra aplicación el texto
  visible sin marcadores Markdown. `Ctrl+Shift+C` pega el Markdown original de
  los bloques seleccionados, incluso si la selección visual solo tomó una parte
  de ellos.
- Si se inicia una selección y el cursor o foco salen de la ventana, el
  arrastre se detiene sin borrar la selección ya conseguida.

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
