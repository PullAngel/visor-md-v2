# QA manual del Sprint 1

## Plegado de secciones pendiente de validación

1. Abrir `tests/fixtures/sprint1-visual.md` en lectura.
2. Pulsar el triángulo a la izquierda de un encabezado y confirmar que su
   contenido desaparece hasta el siguiente encabezado del mismo nivel.
3. Volver a pulsarlo y confirmar que reaparecen texto, listas y selección.
4. Plegar una sección, abrir el índice con `Ctrl+Shift+L` y elegir un encabezado
   hijo: sus ancestros deben desplegarse y el destino quedar enfocado.
5. Cambiar de pestaña y volver: el plegado de cada documento debe mantenerse
   independiente. Entrar en edición debe mostrar siempre la fuente completa.

## Selección por pestaña y marcas de búsqueda

1. Seleccionar texto en lectura, abrir otra pestaña y volver: la selección debe
   reaparecer en el documento correcto.
2. En una pestaña de edición, mover el cursor, cambiar de pestaña y volver: el
   caret debe verse de inmediato en la posición anterior.
3. Pulsar `Ctrl+F`, buscar una palabra repetida y comprobar marcas verdes
   discretas en el borde derecho. Enter debe recorrer resultados sin confundir
   las marcas con una barra de desplazamiento interactiva. La coincidencia
   activa debe quedar resaltada, incluida una palabra con acentos o `ñ`.

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
- Pasar el mouse sobre un enlace debe mostrar una banda efímera con su tipo y
  destino antes de abrirlo. El mismo aviso debe aparecer al enfocarlo con Tab;
  al alejar el mouse o pulsar Escape, desaparece. Verificar que una ruta
  bloqueada solo se anuncia y sigue sin acceder a archivos o red.
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
- En edición, seleccionar texto Unicode y probar `Ctrl+B`, `Ctrl+I` y
  `Ctrl+K`. Deben crear respectivamente negrita, cursiva y enlace, conservar
  seleccionada la parte útil para seguir escribiendo y revertirse con una sola
  pulsación de `Ctrl+Z`. Sin selección, negrita y cursiva dejan `texto`
  seleccionado; un enlace deja `texto` o `https://` listo para reemplazar.
- Abrir el menú contextual en edición y comprobar Negrita, Cursiva, Enlace,
  Encabezado H2 y Lista. H2 y Lista solo anteponen `## ` o `- ` a la línea
  actual; no deben cambiar las líneas vecinas ni normalizar CRLF.
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
nativos y la tabla completa se copia como TSV desde el menú o la paleta;
selección parcial de celdas queda pendiente. En ambos casos, el
contenido permanece inerte y la fuente se conserva sin activar recursos.

## Chrome sin borde de Windows

1. En Windows, abrir el ejecutable release y comprobar que no aparece la barra
   de título decorada del sistema, pero sí tres controles en el extremo superior
   derecho: minimizar, maximizar o restaurar y cerrar.
2. Pulsar minimizar, restaurar y maximizar. La ventana debe conservar documento,
   pestañas y estado sin guardar. El botón Cerrar debe seguir mostrando la
   protección de cambios sin guardar cuando corresponda.
3. Arrastrar el espacio libre de la franja superior. Debe mover la ventana, pero
   no al iniciar el gesto sobre una acción de la barra ni un control de ventana.
4. Llevar el cursor a cada borde y esquina. Debe anunciar la dirección de resize
   con el cursor apropiado y permitir redimensionar sin pasar por debajo de
   640 × 480 puntos lógicos.
5. En Linux u otra plataforma, comprobar el fallback: deben conservarse los
   controles nativos equivalentes y la aplicación debe seguir pudiendo moverse,
   redimensionarse, minimizarse, maximizarse y cerrarse.

## Barra de acciones y pestañas

1. Comprobar que Nuevo, Abrir, Guardar, Editar o Leer, Buscar y Más aparecen en
   la franja superior sin taparse en una ventana normal.
   En edición, comprobar que la misma franja cambia a Guardar, Leer, Negrita,
   Cursiva, H2, Lista, Enlace y Más. Volver a lectura debe restaurar el conjunto
   de lectura, sin dejar acciones de formato marcadas como activas.
2. Activar cada acción con mouse y comprobar que coincide con su atajo.
3. Abrir dos documentos, alternarlos desde la barra inferior y comprobar que el
   asterisco de cambios permanece asociado al documento correcto.
   Abrir el menú contextual o la paleta, fijar una de las pestañas y comprobar
   que aparece `•`; intentar cerrarla debe avisar sin cerrar ni perder cambios.
   Liberarla debe restaurar el cierre protegido normal.
4. Intentar reducir la ventana por debajo de 640 × 480 puntos lógicos. Debe
   detenerse en ese mínimo y conservar todos los controles de la barra visibles
   y clicables.
5. Con una carpeta abierta, usar Buscar en carpeta y escribir una consulta con
   varios resultados. Recorrerlos con flechas y abrir uno con mouse; la nota
   debe abrirse en su pestaña sin modificar la carpeta ni salir de su raíz.
6. Modificar y guardar una nota grande; antes de que termine, cambiar a otra
   pestaña. La segunda debe seguir operable y no debe perder su asterisco ni
   aparecer como guardada cuando termine la escritura de la primera.
7. Abrir Más o pulsar `Ctrl+Shift+P`, escribir `guardar` y verificar que solo
   aparezcan las acciones relacionadas. Borrar la consulta debe restaurar el
   catálogo completo; una consulta inexistente no debe ejecutar nada.
8. Pulsar `F6`, recorrer la barra con flechas y Tab y comprobar que el foco sea
   visible. Enter o Espacio deben ejecutar la acción enfocada y Escape debe
   devolver el teclado al documento.
9. Abrir un documento grande y cambiar inmediatamente a otra pestaña. La barra
   de estado debe indicar la operación solo al volver a la pestaña que carga;
   al terminar, su contenido debe aparecer allí y no en la pestaña activa.

## Vista dividida del editor

1. Abrir `tests/fixtures/sprint1-visual.md` y pulsar `F3`.
2. Confirmar que la fuente aparece a la izquierda sin botones repetidos sobre
   cada línea y que la lectura aparece a la derecha con estilos Markdown.
3. Escribir varias palabras seguidas en la mitad izquierda. La derecha puede
   tardar una fracción de segundo, pero debe terminar mostrando la última
   revisión, sin volver atrás ni mover el cursor de edición.
4. Desplazar el documento y alternar `F3`, `F2` y pestañas. No debe mezclarse el
   contenido de dos documentos ni perderse el indicador de cambios.
5. Repetir con ventana estrecha y, si está disponible, otro DPI. Registrar
   recortes, superposición o texto ilegible; este gate visual sigue pendiente.
6. Cerrar y volver a abrir el mismo archivo después de dejarlo en lectura,
   edición y vista dividida. Debe recuperar cada modo. Un archivo que nunca se
   abrió debe iniciar en lectura y el archivo de preferencias no debe contener
   la ruta ni el nombre del documento en claro.

## PNG local con confirmación

1. Dentro de una carpeta de trabajo, crear una subcarpeta con una nota y un PNG.
   Referenciarlo desde la nota con una ruta relativa a esa subcarpeta.
2. Abrir la carpeta y la nota. El documento debe mostrar primero un placeholder;
   no debe leer ni mostrar la imagen automáticamente.
3. Pulsar el placeholder o enfocarlo con Tab y Enter. Confirmar el diálogo. La
   imagen debe aparecer centrada, sin ampliarse por encima de su tamaño y sin
   deformarse. Escape o un clic deben cerrarla.
4. Repetir rechazando el diálogo: no debe cargar ni recordar el permiso.
5. Probar una URL remota, una ruta absoluta, `..`, una extensión distinta y un
   PNG fuera de la raíz mediante junction o symlink. Todos deben permanecer
   bloqueados, mostrar una explicación breve y no abrir navegador ni red.
