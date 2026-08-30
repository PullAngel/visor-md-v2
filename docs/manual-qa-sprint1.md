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

## Carpeta de trabajo

- La barra inferior debe indicar Lectura o Edición, guardado o sin guardar, y
  si hay carpeta activa. Tras un aviso de actualización de carpeta debe decir
  que requiere actualización, sin tapar el contenido ni convertirse en panel.

- Con `Ctrl+Shift+O`, elegir una carpeta de pruebas que tenga dos o más notas
  Markdown. `Ctrl+Shift+F` debe buscar solo dentro de esa carpeta; las flechas
  cambian la coincidencia y Enter abre la elegida.
- Con esa misma carpeta, `Ctrl+Shift+T` debe recorrer las rutas indexadas en
  orden estable. Escape no abre nada; Enter abre únicamente la nota elegida y
  los cambios sin guardar impiden la navegación. Repetirlo desde modo edición:
  el atajo debe estar disponible y conservar esa misma protección.
- Crear o modificar una nota desde otro editor y pulsar `Ctrl+Shift+I`. La
  actualización debe ser visible en la búsqueda posterior, sin crear archivos
  auxiliares en la carpeta ni tocar `.obsidian`.
- Después de modificar el contenido de la carpeta desde otro programa, volver
  a enfocar Visor MD. Si la marca del directorio cambió, debe avisar de forma
  discreta que `Ctrl+Shift+I` actualiza el índice. Es una ayuda visible, no un
  watcher ni una promesa de detectar todos los cambios de cada filesystem.
- Abrir una nota que reciba enlaces desde dos o más notas de prueba y pulsar
  `Ctrl+Shift+B`. Debe verse la ruta del backlink seleccionado; flechas cambia
  la selección, Escape no abre nada y Enter abre solo una nota dentro de la
  carpeta autorizada.

## Índice del documento

- Con un documento que tenga encabezados de varios niveles, `Ctrl+Shift+L`
  muestra el encabezado actual de la lista; las flechas cambian la selección,
  Enter desplaza la lectura al encabezado y Escape no cambia la posición.
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
- Tras editar, el título muestra `*` junto al nombre del documento; desaparece
  solamente después de un guardado confirmado.
- Pulsar `F2` otra vez vuelve a lectura y muestra el contenido modificado, sin
  superponer bloques aunque se haya pegado o escrito una cantidad grande de
  texto. `Ctrl+Z`, `Ctrl+Y` y `Ctrl+Shift+Z` desde lectura actualizan la vista
  y conservan la posibilidad de volver a entrar en edición.
- Para comprobar un conflicto de guardado, abrir una copia de la fixture,
  editarla en Visor MD, modificarla también con un editor externo y pulsar
  `Ctrl+S`. Confirmar que el archivo externo no se sobrescribe y que el diálogo
  explica las tres acciones. Probar Cancelar; para Recargar, verificar que la
  recuperación queda disponible con `Ctrl+Shift+R`; para Guardar una copia,
  elegir un destino nuevo y verificar que el original externo no cambia.
- Editar un documento, intentar cerrar la ventana y confirmar que aparece el
  aviso de cambios sin guardar. Elegir seguir editando y comprobar que nada se
  pierde; repetir, elegir cerrar y usar `Ctrl+Shift+R` tras reiniciar para abrir
  la recuperación como documento sin destino.
- Abrir o crear tres documentos, modificar al menos dos y recorrerlos con
  `Ctrl+PageUp` y `Ctrl+PageDown`. Confirmar que fuente, undo/redo, título y
  estado sucio pertenecen al documento correcto y que cada pestaña de la barra
  inferior abre el archivo indicado al hacer clic. Verificar también que el
  scroll vuelve a su posición anterior. La `x` de cada pestaña y `Ctrl+W` deben
  proteger solo la
  pestaña activa; cerrar la ventana debe informar el total de documentos con
  cambios y no cerrar si alguna recuperación no puede escribirse.
- Pulsar `Ctrl+Shift+P`, recorrer las acciones con flechas o Tab y ejecutar con
  Enter. Confirmar que Nuevo, Abrir, Guardar, alternar modo, búsqueda, carpeta e
  índice producen el mismo resultado que sus atajos indicados; Escape debe
  cerrar la paleta sin ejecutar nada ni insertar texto en modo edición.
- Con un documento abierto, usar `Ctrl+Shift+R` cuando exista una recuperación.
  Debe aparecer en una pestaña nueva, marcada sin guardar y sin reemplazar ni
  modificar el documento que estaba activo.
- Desde la paleta elegir activar o desactivar recuperación. Al desactivar debe
  aparecer una advertencia y la opción segura debe mantenerla activa. Si se
  confirma desactivar, reiniciar y comprobar que la preferencia persiste; el
  cierre con cambios debe advertir que no existe recuperación, no bloquearse ni
  afirmar que guardó una copia.
- Abrir Índice, Notas y Backlinks desde la paleta. Confirmar que el panel muestra
  varias filas, que flechas conservan visible la elección en listas largas,
  Enter o un clic navegan al destino correcto y Escape devuelve todo el ancho
  a la lectura. Repetir un par de acciones de la paleta mediante clic.
- Abrir el menú contextual con y sin selección en lectura y edición. Copiar no
  debe inventar contenido si no hay selección; Pegar solo aparece en edición.
  Buscar, alternar modo, Guardar y Guardar como deben coincidir con la paleta y
  sus atajos.
- Crear un documento vacío y pulsar `Ctrl+S`: debe abrir Guardar como. Con otra
  pestaña modificada y sin guardar, abrir un wikilink, backlink o nota de la
  carpeta; el destino debe aparecer en una pestaña nueva y la edición anterior
  debe conservar su `*`, fuente e historial.
- Abrir una nota que ya está en otra pestaña mediante diálogo, wikilink o panel.
  Debe activar la pestaña existente, conservar su lugar en la barra y aplicar
  el encabezado solicitado sin crear una copia duplicada.
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

Los bloques de código ahora muestran una acción nativa `Copiar` que copia su
fuente de forma explícita. Las tablas ya se representan con celdas y bordes
nativos; selección y copia TSV de celdas quedan pendientes. En ambos casos, el
contenido permanece inerte y la fuente se conserva sin activar recursos.

## Barra de acciones y pestañas

1. Comprobar que Nuevo, Abrir, Guardar, Editar o Leer, Buscar y Más aparecen en
   la franja superior sin taparse en una ventana normal.
2. Activar cada acción con mouse y comprobar que coincide con su atajo.
3. Abrir dos documentos, alternarlos desde la barra inferior y comprobar que el
   asterisco de cambios permanece asociado al documento correcto.
4. Reducir el ancho de la ventana y confirmar que las acciones que ya no caben
   se omiten sin texto cortado ni controles parcialmente clicables.
5. Con una carpeta abierta, usar Buscar en carpeta y escribir una consulta con
   varios resultados. Recorrerlos con flechas y abrir uno con mouse; la nota
   debe abrirse en su pestaña sin modificar la carpeta ni salir de su raíz.
6. Modificar y guardar una nota grande; antes de que termine, cambiar a otra
   pestaña. La segunda debe seguir operable y no debe perder su asterisco ni
   aparecer como guardada cuando termine la escritura de la primera.
