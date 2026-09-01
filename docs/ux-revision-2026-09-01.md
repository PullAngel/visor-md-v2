# Revisión UX posterior al QA manual

Fecha: 1 de septiembre de 2026. Esta nota convierte el feedback de uso en un
plan de producto. Complementa `design.md`; no autoriza a convertir Visor MD en
un IDE ni en una réplica de Obsidian.

## Hallazgos confirmados

- Un menú contextual abierto no se cierra al pulsar fuera; debe cerrarse con
  Escape, clic fuera y al cambiar de panel.
- Falta cortar (`Ctrl+X` y menú contextual), una operación básica del editor.
- Backspace no repite al mantenerse pulsado; hay que revisar también Delete,
  flechas y selección repetida sin insertar texto de control.
- La vista dividida asimétrica no conserva una correspondencia visual suficiente
  entre fuente y resultado. Debe volver a una división centrada o sincronizar
  posiciones antes de priorizar ancho editorial.
- El toggle lectura/edición no tiene jerarquía visual ni forma reconocible de
  conmutador. Tema día/noche tampoco es descubrible aunque existe el atajo `T`.
- El índice, carpeta y navegación de bóveda están demasiado ligados a lectura;
  deben seguir disponibles en edición sin competir con el formato.
- Las tareas de lectura necesitan una revisión funcional: el clic debe cambiar
  únicamente `[ ]` y `[x]`, conservar undo/redo y actualizar la vista.
- Menús, diálogos y avisos flotantes no comparten aún radio, borde, sombra,
  foco y cierre consistentes.

## Segunda revisión técnica

- La acción de tareas ya existe, pero su área de clic o refresco no coincide con
  el resultado percibido: es una regresión de interacción, no una función que
  pueda declararse terminada por tener código.
- **Corregido:** el editor admite repetición de Backspace, Delete y navegación;
  los atajos, diálogos y cambios de modo siguen siendo de un único disparo.
  Enter permanece deliberadamente individual hasta revisar agrupación de undo.
- El tema alterna con `T`, pero no figura como acción visible y puede ser
  absorbido por la entrada de fuente. La solución es una acción del catálogo,
  no otro atajo oculto.
- El menú contextual se descarta al pulsar, pero paneles y overlays no tienen
  una política única de cierre al hacer clic fuera. Ese estado debe centralizarse
  antes de añadir desplegables.
- El split 42/58 fue una corrección visual válida en abstracto, pero el QA
  muestra que empeora la comparación. La fidelidad de correspondencia prevalece
  sobre ese reparto de ancho.

## Decisiones de jerarquía

1. La barra de edición será un kit Markdown completo, pero en dos niveles:
   acciones universales visibles (texto, bloques, listas, enlaces, tarea,
   cita, código, tabla, imagen segura) y variantes en desplegables o `Más`.
2. El menú contextual se calcula por situación: selección, cursor vacío,
   enlace, tabla, tarea o pestaña. Nunca muestra una lista fija de acciones
   inútiles.
3. Estudio, preparación para IA y segundo cerebro se agrupan en un único panel
   plegable de "Herramientas", no en la barra de formato. Primero se implementa
   Markdown portable; funciones propias o sidecars requieren decisión aparte.
4. Índice, búsqueda, carpeta, backlinks y wikilinks viven bajo un único punto
   "Espacio de trabajo" disponible en lectura y edición. La VFS continúa siendo
   la única ruta de apertura.
5. Una barra lateral izquierda solo se habilita como rail plegable para
   navegación/contexto. No se convierte en columna fija ni duplica paneles.

## Plan secuencial

### A. Corrección de interacción y fidelidad

- Cierre de overlays, cortar, repetición de teclado, tareas clicables y revisión
  de otras operaciones básicas omitidas (deshacer/rehacer, seleccionar todo,
  pegar explícito, abrir/guardar/cerrar).
- División centrada, scroll y correspondencia de bloque; si no se puede
  sincronizar con fidelidad, alternar lectura/fuente es preferible a fingir una
  comparación útil.
- Regresiones para cada defecto y QA manual focalizado.

**Salida:** edición cotidiana no sorprende ni pierde datos; las tareas y la
vista dividida reflejan exactamente la fuente.

### B. Sistema visual y chrome

- Toggle lectura/edición con estado claro; selector de tema visible dentro de
  `Más` y paleta, conservando `T`.
- Radio moderado en controles flotantes y bloques adecuados; elevación suave,
  borde y foco unificados para menús, paneles, confirmaciones y diálogos.
- Mejor uso de la zona superior derecha con acciones de estado, no con botones
  permanentes nuevos. Animación solo donde no desplace texto y con reducción de
  movimiento respetada.

**Salida:** el chrome se entiende sin memorizar atajos y las capas flotantes se
sienten parte del mismo producto.

### C. Kit de escritura y toma de notas

- Formato inline, encabezados, listas/tareas, cita, código, tabla, enlaces,
  wikilinks, callouts, resaltado y recursos locales seguros.
- Operaciones contextuales para selección y cursor; insertar fecha, símbolos,
  bloques de estudio y fragmentos preparados para IA quedan bajo menús o panel.
- Herramientas de estudio portables: pregunta/respuesta, ocultación, estados,
  conceptos y resúmenes, priorizadas tras validar sintaxis interoperable.

**Salida:** escribir Markdown para universidad, IA u Obsidian no exige recordar
sintaxis frecuente y no crea formatos exclusivos por defecto.

### D. Espacio de trabajo y ventanas

- Unificar navegación de bóveda en panel/rail plegable disponible en ambos
  modos; mostrar raíz, estado de índice, búsqueda, árbol, backlinks y límites.
- Pestañas reordenables y arrastrables antes de crear otra ventana.
- Separación horizontal/vertical y ventanas nuevas son un programa aparte:
  necesitan definir propiedad de documentos, cierre protegido, recuperación,
  foco, DPI y comportamiento entre monitores. No se implementarán como un
  atajo visual sobre el estado actual.

**Salida:** una bóveda se entiende sin experiencia previa de Obsidian; varias
pestañas no pierden cambios ni confunden la sesión.

## Gates

- pruebas de teclado repetido, cortar, menú fuera, tareas y división;
- QA visual de día/noche, mínimo, DPI, lectura, edición y dividida;
- no añadir dependencia o permiso sin revisión de tamaño, licencia y seguridad;
- registrar medidas release antes de cerrar el programa UX.
