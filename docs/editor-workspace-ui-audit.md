# Auditoría UX de edición, índice y bóvedas

Actualizada: 2026-09-01. Esta auditoría revisa la experiencia observable y las
fronteras implementadas de edición, índice documental y carpetas de trabajo.
Complementa [`design-audit.md`](design-audit.md): aquel documento trata la
coherencia visual global; este trata los flujos de trabajo que una persona debe
poder descubrir, entender y repetir sin convertir Visor MD en un IDE.

## Veredicto breve

La base de seguridad e integridad de estas funciones está por delante de su
experiencia de uso. Eso es una buena prioridad: el editor conserva la fuente y
el workspace no recibe permisos por el mero hecho de estar indexado. Sin
embargo, varias capacidades ya implementadas se perciben como comandos internos
porque viven casi exclusivamente en atajos, la paleta o avisos transitorios.

La corrección no es añadir una barra lateral permanente ni más botones. Debe
ser una capa pequeña de navegación contextual y paneles con mejor jerarquía,
estado y descubribilidad.

## Evidencia revisada

- código de `src/main.rs`, `src/editor.rs`, `src/document.rs`, `src/vfs.rs` e
  `src/workspace.rs`;
- especificaciones de producto, diseño, seguridad, arquitectura y funciones;
- lectura y captura de la paleta de acciones y del índice del documento en el
  binario release actual;
- pruebas automáticas existentes de editor, rutas, índice, backlinks y paneles.

No se usó una bóveda personal. Falta todavía una bóveda de fixture versionada,
pequeña y anonimizada para validar visualmente los flujos completos de árbol,
búsqueda, wikilinks, backlinks y límites de indexación.

## Lo que conviene conservar

| Decisión | Por qué es correcta |
| --- | --- |
| Edición orientada a fuente | El Markdown desconocido conserva su texto; la vista no se convierte en una reescritura destructiva. |
| Guardado atómico, detección de cambios externos y recuperación explícita | Protegen el activo principal: las notas de la persona. Un fallo no debe destruir el original ni sobrescribir una versión externa. |
| Carpeta elegida explícitamente y VFS contenida | La VFS es la frontera que vuelve a comprobar que cada destino siga dentro de la raíz concedida. Indexar una nota nunca le concede permiso para abrir otra ruta. |
| Índice en memoria, limitado y cancelable | Evita una base de datos persistente, contenido obsoleto y escrituras inesperadas en una bóveda de Obsidian. |
| Wikilinks ambiguos bloqueados | Elegir una nota por orden sería cómodo pero incorrecto: podría abrir la nota equivocada. Pedir una ruta más precisa protege integridad y comprensión. |
| Paneles mutuamente excluyentes | Conservan espacio de lectura y aplican divulgación progresiva; no debe convertirse en una columna fija de herramientas. |

## Hallazgos de edición

| Estado real | Riesgo de UX | Corrección prioritaria |
| --- | --- | --- |
| Hay comandos para negrita, cursiva, enlace, encabezado y lista, por teclado, menú contextual y paleta. | La barra de edición muestra demasiados rótulos seguidos y el menú contextual mezcla edición, archivos, modos y acciones globales. Parece una lista técnica, no una ayuda según el contexto. | Separar acciones de selección de acciones globales. El menú contextual debe mostrar solo lo aplicable; la paleta conserva el catálogo completo. |
| Fuente, lectura y vista dividida están diferenciadas. | Falta una indicación visual compacta y persistente de qué modo está activo y qué puede hacerse allí. | Reforzar el modo actual en barra de estado y toolbar con una sola acción primaria, sin repetir controles. |
| Undo/redo, selección Unicode, pegado explícito, BOM/EOL y guardado fiable existen. | El éxito o rechazo se comunica casi siempre en un aviso breve; un conflicto, recuperación o guardado bloqueado merece una explicación recuperable. | Añadir estado de documento con severidad y una acción breve de detalles, sin diálogos intrusivos para éxitos normales. |
| Copiar Markdown y TSV existen. | `Copiar TSV` solo aparece al abrir el menú sobre una tabla; el resto del menú conserva las operaciones aplicables al modo. | Mantener esta regla al añadir acciones puntuales y ofrecer explicación breve cuando una acción global se rechace. |

No se recomienda añadir numeración de líneas, completado de código o resaltado
de sintaxis general: serían señales de IDE y no resuelven el problema principal
de escribir Markdown con comodidad.

## Hallazgos de índice y navegación

| Área | Estado real | Problema de UX | Siguiente mejora segura |
| --- | --- | --- | --- |
| Índice del documento | Lista encabezados y puede enfocar el bloque. | El panel no expresa posición, total, atajo de salida ni jerarquía suficiente en documentos largos. | Cabecera clara y pie compacto, por ejemplo `3 de 24 · Esc para cerrar`; conservar sangría por nivel. |
| Búsqueda de carpeta | Busca título, ruta, encabezados y contenido indexado; su cabecera identifica la carpeta autorizada y cada resultado muestra título seguido de ruta relativa. | Falta un fragmento o encabezado que explique por qué coincidió una nota cuyo título no contiene la consulta. | Añadir un fragmento o encabezado coincidente, sin guardar contenido fuera del índice ya permitido. |
| Árbol de notas | Es acotado, abre solo notas validadas por la VFS y permite plegar directorios. | La cabecera ahora identifica la carpeta concedida y el total indexado; aún falta una convención gráfica más clara para carpeta, nota y actualización. | Conservar la cabecera de workspace y añadir símbolos coherentes dentro del panel, no una barra lateral fija. |
| Backlinks | Se derivan del índice y el destino vuelve a validarse al abrir. Un documento sin referencias abre un panel que lo explica y permite cerrarlo con Escape. | Los estados de documento fuera de raíz o no indexado continúan siendo avisos de seguridad, porque no describen una lista vacía sino una operación bloqueada. | Mantener la distinción y añadir detalle bajo demanda solo si aparece una necesidad real. |
| Wikilinks | Soporta destino, alias y encabezado, con ambigüedad bloqueada. | Los diagnósticos seguros desaparecen pronto y no hay una vista de enlaces rotos o ambiguos. | Incorporar un diagnóstico contextual de solo lectura, limitado al documento actual, después de estabilizar la navegación base. |

## Defectos de interacción a corregir antes de ampliar funciones

1. **Corregido en la base de paneles:** la geometría de dibujo y de hit testing
   ahora procede de una única función responsive. El panel también muestra el
   rango visible y el total, por ejemplo `1–9 de 24 · Esc para cerrar`.
2. Los paneles tienen capacidad limitada, pero no exponían con claridad cuántos
   resultados quedan fuera de la ventana ni la posición de la selección. Esto
   queda resuelto para listas largas por el pie de rango; falta conservar esa
   claridad en estados vacíos y resultados enriquecidos.
3. Elegir y actualizar una carpeta está disponible mediante paleta o atajo. Es
   correcto para personas expertas, pero demasiado oculto para un flujo central
   de Obsidian. Debe quedar accesible desde `Más` o desde un único punto de
   navegación contextual, no como un nuevo conjunto de botones permanentes.
4. **Corregido en la barra inferior:** ahora prioriza modo, estado de guardado
   y actualización de carpeta. Las métricas siguen siendo locales y disponibles
   para las capacidades que las usan, pero dejaron de competir con advertencias
   de integridad o de workspace en la franja persistente.

## Secuencia de corrección propuesta

### 1. Fundamento común de paneles

- **Realizado:** centralizar rectángulo, cabecera, filas, pie y hit testing;
- **Realizado:** hacer el ancho y alto responsivos y mostrar total, selección
  y salida por teclado;
- conservar Escape, flechas y Enter, y añadir foco visible consistente con el
  sistema de iconos pendiente.

**Cierre verificable:** cada panel acepta clics solo dentro de su rectángulo
real y comunica selección/total; las pruebas cubren geometría estrecha y
navegación de listas largas.

### 2. Taxonomía de acciones y edición contextual

- **Realizado en la estructura:** separar una barra principal para archivo y
  navegación general de una segunda barra contextual para formato o bóveda;
- mover operaciones menos frecuentes a `Más` y a la paleta;
- **Realizado para tablas:** ocultar `Copiar TSV` fuera de una tabla; extender
  la misma regla gradualmente a acciones dependientes de selección o enlace;
- usar rótulos y futuras iconografías que sigan el sistema suave de diseño, con
  nombres disponibles para teclado y accesibilidad.

**Cierre verificable:** en fuente, lectura y selección de tabla, el menú no
ofrece acciones inútiles ni oculta una acción esencial; toda acción sigue
disponible desde teclado o paleta.

### 3. Navegación de bóveda visible y segura

- presentar una entrada única de navegación de carpeta desde `Más`;
- dar contexto de raíz, estado de índice, límites aplicados y actualización;
- enriquecer árbol, búsqueda y backlinks con título/ruta secundaria y estados
  vacíos específicos;
- mantener toda apertura de nota y enlace a través de VFS.

**Cierre verificable:** una persona elige una carpeta, entiende que fue
indexada sin escritura, busca una nota y abre un backlink sin recordar atajos;
las pruebas demuestran que rutas externas, UNC y enlaces ambiguos siguen
bloqueados.

### 4. Diagnóstico y QA de bóveda

- añadir fixture de bóveda anonimizada que incluya alias, encabezados, enlaces
  rotos, ambigüedad, callouts, rutas hostiles y una estructura profunda;
- ejecutar un recorrido manual visual de árbol, búsqueda, breadcrumbs/estado,
  wikilinks y backlinks;
- documentar cualquier límite mostrado a la persona en `security.md` y
  `manual-qa`.

**Cierre verificable:** el recorrido se reproduce sin tocar `.obsidian`, crear
sidecars ni seguir recursos secundarios.

## Límites deliberados

- No persistir el índice ni el contenido de la bóveda.
- No cargar imágenes o recursos remotos.
- No abrir automáticamente rutas locales, UNC, `file://` ni enlaces web.
- No usar paneles como sustituto de una interfaz de Obsidian completa.
- No agregar una función de productividad antes de hacer comprensible y segura
  la navegación ya existente.
