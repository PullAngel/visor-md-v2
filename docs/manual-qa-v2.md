# QA manual integral de Visor MD v2

Esta lista reúne el QA humano, end to end y adversarial que no puede quedar
demostrado solamente por las pruebas automáticas. Úsala sobre un build release
actual, marca cada resultado y conserva capturas, versión, sistema operativo y
fecha junto a cualquier fallo.

No convierte una función planificada en requisito de aprobación: los ítems que
digan **si está disponible** verifican una capacidad parcial existente. Lo que
no está implementado sigue en [`features.md`](features.md) y
[`roadmap.md`](roadmap.md).

## Registro de la sesión

- [ ] Fecha y hora:
- [ ] Persona que ejecuta:
- [ ] Commit probado: `________________`
- [ ] Sistema operativo y versión:
- [ ] Arquitectura y toolchain:
- [ ] Escala DPI y resolución:
- [ ] Tema del sistema:
- [ ] Ruta del ejecutable probado:
- [ ] Fixture o documentos usados:
- [ ] Resultado general: aprobado / aprobado con observaciones / bloqueado.
- [ ] Capturas, vídeos o logs se guardaron fuera de `target/` y se enlazaron
  desde el informe de la sesión.

## Preparación segura

- [ ] El árbol Git se inspeccionó antes de probar; no se usarán documentos
  personales como entrada para pruebas destructivas.
- [ ] Se creó una carpeta temporal de QA con copias de documentos, nunca se
  modificó una bóveda real durante esta lista.
- [ ] La carpeta temporal contiene al menos: un `.md` normal, un `.txt`, un
  Markdown con Unicode/emoji, un archivo con CRLF y BOM UTF-8, un Markdown con
  tablas y tareas, y una copia destinada a probar conflictos externos.
- [ ] Se comprobó que no hay instancias anteriores de Visor MD abiertas antes
  de evaluar el arranque.
- [ ] Se ejecutó el gate técnico reciente o se registró por qué no corresponde:
  `scripts/check.ps1`.
- [ ] Se construyó o verificó el release actual antes de probarlo.

Comando habitual desde la raíz del repositorio:

```powershell
cargo run --release -- .\tests\fixtures\sprint1-visual.md
```

Tras compilar una vez, para no medir la compilación como si fuera arranque:

```powershell
& .\target\release\visor-md.exe .\tests\fixtures\sprint1-visual.md
```

## Arranque, ventana y estado vacío

- [ ] La aplicación abre sin ventana de consola inesperada ni mensajes técnicos
  dirigidos a la persona usuaria.
- [ ] Un `.md` abierto por argumento muestra el título, el contenido y el modo
  de lectura inicial sin un parpadeo importante.
- [ ] Al abrir sin archivo, aparece un documento nuevo y el aviso explica cómo
  elegir destino al guardar.
- [ ] El título de la ventana identifica el documento activo y señala cambios
  sin guardar cuando corresponde.
- [ ] Minimizar, restaurar, maximizar y cerrar responden correctamente.
- [ ] En Windows, la ventana sin borde se puede arrastrar solo desde la zona
  prevista y redimensionar desde bordes y esquinas sin impedir botones.
- [ ] La ventana mínima no oculta controles esenciales ni deja objetivos de
  clic fuera de pantalla.
- [ ] Abrir y cerrar menús, paneles o la paleta con `Escape` devuelve el foco
  de forma predecible al documento.
- [ ] No hay cierre inesperado, congelamiento ni crecimiento visible de CPU en
  reposo durante al menos un minuto.

## Lectura editorial y Markdown

Usar `tests/fixtures/sprint1-visual.md` y luego un Markdown propio no sensible.

- [ ] Encabezados, párrafos, negrita, cursiva, combinación anidada, tachado,
  código inline y enlaces tienen jerarquía visual clara.
- [ ] Las listas con viñetas, numeradas, anidadas y con líneas ajustadas
  conservan sangría y alineación legibles.
- [ ] Las tareas muestran estados distintos y las casillas se ven completas.
- [ ] Hacer clic en una tarea disponible cambia exclusivamente `[ ]` y `[x]` de
  su fuente, marca el documento como modificado y `Ctrl+Z` lo revierte.
- [ ] Citas simples y anidadas se distinguen por profundidad, sin solaparse.
- [ ] Reglas horizontales, bloques de código y botón de copia de código se ven
  y funcionan sin seleccionar texto ajeno.
- [ ] Tablas tienen encabezado, celdas, bordes y alineación visibles; tablas
  estrechas, anchas y con Unicode no rompen el layout.
- [ ] `kbd`, `mark`, `sub`, `sup` y `br` permitidos sin atributos se presentan
  como semántica nativa; atributos permanecen visibles e inertes.
- [ ] HTML no permitido, scripts, iframes, eventos y estilos no ejecutan nada,
  no cargan recursos y permanecen como fuente visible segura.
- [ ] Callouts conocidos son legibles; callouts desconocidos no desaparecen ni
  adquieren comportamiento.
- [ ] Los títulos plegables y preguntas plegables ocultan solo la vista, nunca
  alteran el texto al guardar o volver a abrir.
- [ ] El índice, búsqueda, selección, plegado y métricas no cambian la fuente.

## Unicode, fuentes y tamaños extremos

- [ ] Español con tildes, ñ y signos de apertura se ve y se copia correctamente.
- [ ] Árabe, japonés, coreano, devanagari y mezcla de direcciones no producen
  cuadros corruptos, cortes de bytes ni bloqueos.
- [ ] Emoji se muestra mediante fallback o, si falta un glifo, se degrada de
  forma comprensible sin cambiar el texto copiado.
- [ ] Redimensionar repetidamente una ventana con Unicode no superpone líneas,
  marcadores ni selección.
- [ ] Un documento muy largo mantiene scroll, búsqueda y respuesta de teclado
  utilizables; registrar tamaño aproximado y percepción.
- [ ] Una línea excepcionalmente larga entra en vista segura o muestra el aviso
  correspondiente; no cuelga ni pierde la fuente al copiarla.
- [ ] Un documento con anidamiento profundo se degrada con aviso o permanece
  estable; no causa cierre por desbordamiento de pila.

## Tema, apariencia, DPI y movimiento

- [ ] El tema inicial sigue el sistema cuando no hay preferencia local previa.
- [ ] La acción de tema cambia entre día y noche, conserva contraste y no deja
  texto, iconos o bordes invisibles.
- [ ] La transición de tema es breve, no desplaza el contenido y termina.
- [ ] Con reducir movimiento activado, el cambio de tema es inmediato y la
  preferencia sobrevive al reinicio.
- [ ] En 100 %, 125 %, 150 % y, si es posible, 200 % DPI, tipografía, iconos,
  paneles, barras y áreas de clic siguen alineados.
- [ ] Al mover la ventana entre monitores con escalas distintas, no aparecen
  texto borroso, recortes, coordenadas de clic desplazadas ni layout corrupto.
- [ ] El foco visible de teclado se distingue del hover y del estado activo.

## Selección, portapapeles y navegación de lectura

- [ ] Arrastrar para seleccionar texto en lectura da una selección continua y
  permite cruzar varios bloques.
- [ ] Arrastrar hasta el borde hace autoscroll sin saltos excesivos.
- [ ] `Shift` más flechas, `Ctrl` más flechas, Inicio/Fin, `Ctrl+Inicio`,
  `Ctrl+Fin`, RePág, AvPág y `Ctrl+A` seleccionan lo esperable sin partir
  caracteres Unicode ni CRLF.
- [ ] `Ctrl+C` copia texto legible; listas, citas y tareas siguen siendo
  comprensibles al pegar en un editor simple.
- [ ] `Ctrl+Shift+C` copia Markdown de bloques completos y no produce sintaxis
  rota por una selección visual parcial.
- [ ] Copiar una tabla como TSV, si la acción está disponible sobre ella, pega
  columnas separadas correctamente en una hoja de cálculo o editor de texto.
- [ ] Tab recorre enlaces e imágenes confirmables en un orden predecible;
  Enter solo abre el destino enfocado mediante la política correspondiente.
- [ ] Hover y foco de enlaces revelan el destino declarado antes de abrirlo.

## Edición fuente, historial y vista dividida

- [ ] `F2` alterna lectura y edición sin perder contenido, scroll o historial.
- [ ] Se puede escribir texto normal, Unicode y pegado explícito con `Ctrl+V`.
- [ ] Backspace y Delete repetidos eliminan contenido sin insertar espacios ni
  mover visualmente el cursor en sentido contrario.
- [ ] `Ctrl+X`, `Ctrl+C`, `Ctrl+V`, `Ctrl+Z` y `Ctrl+Y` actúan sobre la selección
  o el historial esperado; no leen el portapapeles en segundo plano.
- [ ] `Ctrl+Z` desde lectura revierte el último cambio del documento y actualiza
  la vista renderizada sin perder la posición lógica del cursor al volver a editar.
- [ ] Enter conserva el comportamiento Markdown previsto y `Shift+Enter`
  produce un salto visible que también se refleja en lectura.
- [ ] Selección con mouse, teclado y la introducción mediante IME no parte
  caracteres, no duplica texto ni deja composición fantasma.
- [ ] Insertar formato desde barra, atajo, paleta y menú contextual realiza el
  mismo Markdown reversible y no modifica texto fuera de la selección.
- [ ] `F3` abre comparación fuente/vista: ambos paneles muestran la misma
  revisión, sin superposición; probar disposición lado a lado y apilada.
- [ ] Editar rápido, pegar varios párrafos y volver a lectura no deja bloques
  superpuestos, líneas desaparecidas ni un render anterior.
- [ ] Abrir un `.txt`, JSON, YAML, TOML, CSV o código lo muestra como texto
  inerte y nunca intenta ejecutarlo ni convertirlo en editor de programación.

## Archivos, guardado y recuperación

- [ ] Crear documento, escribir, guardar, cerrar y reabrir preserva exactamente
  la fuente Markdown incluida sintaxis que Visor MD no renderiza.
- [ ] Guardar como crea el destino elegido y no reemplaza otro archivo sin una
  acción explícita.
- [ ] Un archivo UTF-8 con BOM se reabre con el BOM conservado pero sin mostrarlo
  como carácter de contenido.
- [ ] Un archivo CRLF conserva CRLF; uno LF conserva LF; uno mixto no se
  normaliza silenciosamente.
- [ ] Mientras una pestaña tiene cambios, abrir, cambiar de pestaña y cerrar
  otra no mezcla estados ni historial.
- [ ] Cerrar una pestaña modificada muestra una decisión clara; cancelar deja
  todo intacto, guardar persiste y descartar solo afecta esa pestaña.
- [ ] Modificar el archivo temporal desde otro editor mientras Visor MD tiene
  cambios abiertos provoca conflicto: no sobrescribe; recargar, guardar copia y
  cancelar preservan la edición local hasta elegir una acción.
- [ ] Simular un fallo de destino no destruye el archivo original ni oculta el
  error.
- [ ] Con recuperación activa, modificar un documento, esperar unos segundos y
  cerrar de forma inesperada en la copia de QA permite detectar una recuperación
  de manera explícita en el siguiente inicio.
- [ ] Tras guardar o cerrar correctamente, la recuperación temporal se elimina;
  desactivar recuperación muestra una explicación de privacidad clara.

## Pestañas, acciones y paneles

- [ ] Abrir tres documentos mantiene fuente, modo, selección, scroll, plegados,
  historial y estado de guardado separados por pestaña.
- [ ] `Ctrl+Tab`, `Ctrl+PageUp`, `Ctrl+PageDown`, mouse y pestañas fijadas
  siguen un orden estable y no cambian el documento equivocado.
- [ ] Reordenar pestañas con arrastre cambia solo la sesión, sin duplicar o
  perder documentos.
- [ ] La barra de acciones, menú contextual y `Ctrl+Shift+P` exponen acciones
  coherentes; una acción buscada tiene etiqueta comprensible y no se duplica.
- [ ] La paleta puede abrirse, buscar, elegir con teclado, cancelar y devolver
  foco correctamente.
- [ ] Índice, búsqueda de documento, árbol de carpeta, búsqueda de workspace y
  backlinks se pueden abrir y cerrar sin tapar permanentemente la lectura.
- [ ] Los paneles mantienen selección visible, permiten teclado/mouse y no
  permiten clics fuera de sus filas.
- [ ] La barra de estado explica modo, cambios, bloqueos o workspace sin mostrar
  contenido sensible ni detalles técnicos innecesarios.

## Workspace y Obsidian

Trabajar únicamente en una copia de una bóveda o en
`tests/fixtures/obsidian-vault`.

- [ ] Elegir una carpeta indexa notas dentro de la raíz explícita; no crea
  sidecars, no modifica `.obsidian` ni escribe contenido indexado.
- [ ] El árbol representa carpetas y notas; abrir una nota desde él no sale de
  la raíz autorizada.
- [ ] La búsqueda de workspace devuelve títulos/rutas correctos, no contenido
  fuera de presupuesto y sigue siendo respondiente mientras indexa.
- [ ] Cancelar un índice grande deja un estado claro; resultados viejos no
  sustituyen una raíz elegida después.
- [ ] Modificar, crear o borrar una nota externa produce señal visible y
  `Ctrl+Shift+I` reconstruye el índice de manera explícita.
- [ ] `[[nota]]`, `[[nota|alias]]` y enlaces a encabezados correctos navegan
  solo tras una acción explícita y dentro de VFS.
- [ ] Un destino ausente o ambiguo se mantiene visible, explica el problema y
  no abre una nota elegida arbitrariamente.
- [ ] Los backlinks muestran las notas que apuntan a la actual y su navegación
  vuelve a validar la ruta antes de abrirla.
- [ ] Callouts y resaltado portable de una bóveda existente se leen sin migrar,
  reformatear ni cambiar archivos no editados.

## Seguridad adversarial

No usar archivos sensibles. Registrar resultado y, ante conducta inesperada,
cerrar la prueba sin intentar explotar el fallo.

- [ ] Abrir Markdown con `<script>`, `onclick`, iframe, formulario y CSS inline
  no ejecuta código, no abre ventanas y no hace conexiones.
- [ ] Una imagen `https://...` permanece bloqueada por defecto; no hay descarga
  ni solicitud de red automática.
- [ ] Una imagen relativa PNG dentro de la bóveda muestra placeholder y solicita
  confirmación individual antes de decodificarla.
- [ ] Rutas `file://`, UNC (`\\servidor\recurso`), absolutas, de dispositivo,
  `..`, streams alternativos y enlaces que atraviesan symlinks/junctions se
  muestran como texto o diagnóstico bloqueado; nunca se abren automáticamente.
- [ ] Si se prueba una UNC elegida explícitamente mediante diálogo, registrar
  que la app solo abre ese archivo principal y no explora recursos secundarios.
- [ ] Enlaces web o mail solo delegan al sistema después de la interacción
  explícita definida; verificar que el destino visible coincide con el elegido.
- [ ] El documento no puede cambiar tema, preferencias, recuperación, permisos
  de bóveda ni política de seguridad.
- [ ] Una PNG con extensión falsa, tamaño excesivo o dimensiones excesivas se
  rechaza con mensaje comprensible sin agotar memoria.
- [ ] Durante apertura normal y render de documentos hostiles no aparecen
  conexiones salientes inesperadas en el monitor de red disponible.

## Rendimiento y estabilidad

- [ ] Medir arranque con `scripts/benchmark-startup.ps1` y conservar la salida
  cruda; no confundir el tiempo de compilación con el de apertura.
- [ ] Abrir repetidamente una fixture pequeña se siente inmediato y no degrada
  de una iteración a otra.
- [ ] Hacer scroll sostenido en documento largo no presenta congelamientos,
  consumo de memoria creciente sin límite ni cuadros persistentemente perdidos.
- [ ] Abrir una carpeta grande sintética o copia de bóveda permite cancelar,
  cambiar de pestaña y seguir leyendo mientras indexa.
- [ ] El release medido permanece por debajo de 8 MiB; anotar bytes y hash.
- [ ] Tras cerrar pestañas y paneles, la aplicación continúa respondiendo y no
  conserva contenido de documentos en mensajes, títulos o paneles equivocados.

## Accesibilidad y compatibilidad de plataforma

- [ ] Todo flujo crítico de esta lista se puede iniciar y cancelar con teclado.
- [ ] El orden de Tab es predecible; foco, selección y destino de Enter son
  visibles.
- [ ] Contraste de texto, enlaces, selección, avisos y estados deshabilitados
  sigue siendo legible en ambos temas.
- [ ] Probar con alto contraste del sistema, si está disponible; registrar
  elementos invisibles o con contraste insuficiente.
- [ ] Probar con lector de pantalla, si está disponible; registrar controles sin
  nombre, foco perdido o contenido inaccesible. Esta capacidad aún no es un
  criterio cerrado de la versión actual.
- [ ] Probar al menos Windows y Linux antes de declarar una release; registrar
  diferencias de diálogos nativos, rutas, fuentes, DPI, IME y chrome.

## Cierre, defectos y evidencia de release

- [ ] Cada defecto reproduce pasos mínimos, resultado esperado, resultado real,
  plataforma, commit y evidencia visual o log sin datos privados.
- [ ] Cada defecto de seguridad indica el recurso afectado, si hubo acceso real
  y si se evitó repetir una explotación.
- [ ] Cada fallo corregido recibe una regresión automatizable antes de cerrar el
  ticket, cuando sea técnicamente posible.
- [ ] Se ejecutó nuevamente el subconjunto automatizado afectado después de cada
  arreglo relevante.
- [ ] Para un hito o release, `scripts/check.ps1`, CI Windows/Linux, SBOM,
  advisories, benchmark reproducible, matriz de pruebas y threat model quedan
  revisados y enlazados.
- [ ] Los ítems pendientes se trasladaron a `docs/test-matrix.md` o
  `docs/roadmap.md` con alcance, riesgo y condición de cierre claros.
- [ ] La entrega no se declara lista solo porque no hubo crash: los ítems de
  seguridad, preservación de datos, accesibilidad y UX críticos tienen evidencia.

## Resumen de resultados

| Área | Aprobado | Observaciones o enlace a evidencia |
| --- | --- | --- |
| Arranque y ventana |  |  |
| Lectura y Markdown |  |  |
| Unicode, fuentes y límites |  |  |
| Tema, DPI y movimiento |  |  |
| Selección y portapapeles |  |  |
| Edición y vista dividida |  |  |
| Guardado y recuperación |  |  |
| Pestañas, acciones y paneles |  |  |
| Workspace y Obsidian |  |  |
| Seguridad adversarial |  |  |
| Rendimiento y estabilidad |  |  |
| Accesibilidad y plataformas |  |  |
