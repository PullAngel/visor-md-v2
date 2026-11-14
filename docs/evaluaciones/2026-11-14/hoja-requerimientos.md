# Visor MD · Hoja de requerimientos para QA (versión de prueba)

**Producto.** Visor MD es una aplicación de escritorio para leer y editar
documentos Markdown (`.md`), pensada para estudiar y tomar apuntes.
Prioridades: es liviana, segura por defecto y tiene una lectura cómoda.

**Estilo visual declarado.** Se llama «Papel y tinta». Tiene:
- Tema de noche (verde muy oscuro) y tema de día (papel crema).
- Un único color de acento verde.
- Ventana sin borde de Windows, con controles propios: fijar encima,
  minimizar, maximizar y cerrar.
- Dentro del documento todo es plano. Solo flotan, con una sombra suave, los
  menús, diálogos y avisos.

**Público.** Estudiantes y personas que escriben apuntes. No se asume que
conozcan la sintaxis de Markdown.

## Requerimientos funcionales

| ID | Requerimiento |
| --- | --- |
| R1 | Abrir un archivo `.md` existente y verlo formateado (vista **Leer**). |
| R2 | Crear un documento nuevo, escribirlo y guardarlo con nombre (**Guardar** / **Guardar como**). |
| R3 | Tres modos por documento: **Leer** (formateado), **Editar** (texto fuente con números de línea) y **Comparar** (fuente y vista lado a lado, sincronizadas). |
| R4 | Varias pestañas en la misma ventana: abrir, cambiar, cerrar, reordenar, fijar. Cada pestaña recuerda su posición de lectura. |
| R5 | Dividir la ventana en paneles (derecha / abajo, hasta 4) para ver dos documentos a la vez. |
| R6 | Ayudas de formato sin saber Markdown: negrita, cursiva, tachado, código, resaltado, títulos, listas con viñetas, numeradas y de tareas, citas, enlaces, bloques de código, tablas, símbolos y emojis. |
| R7 | En listas, Enter continúa la lista; Enter en un elemento vacío la termina. |
| R8 | Bloques de código con botón **Copiar** que copia solo el código y confirma visualmente. |
| R9 | Casillas de tareas que se marcan con un clic en la vista Leer. |
| R10 | Plegar y desplegar secciones desde su título. |
| R11 | **Índice** del documento para saltar entre títulos; índice flotante opcional en ventanas anchas. |
| R12 | **Buscar** dentro del documento, con resultados resaltados y recorrido entre coincidencias. |
| R13 | **Carpeta de trabajo**: elegir una carpeta de apuntes, ver sus archivos, buscar en todos y ver qué notas enlazan a la actual (Backlinks). Los enlaces entre notas `[[nota]]` funcionan. |
| R14 | Enlaces web: antes de abrir se muestra el destino real. Los enlaces rotos se distinguen. |
| R15 | **Paleta de acciones** (botón «···») que reúne todas las acciones y se filtra escribiendo. |
| R16 | Menú contextual (clic derecho) con acciones según lo que se toca: texto, tarea, tabla o pestaña. |
| R17 | Cambiar entre tema de día y de noche. Opción de reducir movimiento. |
| R18 | Fijar la ventana encima de las demás (chincheta junto a minimizar), útil para estudiar. |
| R19 | Exportar el documento a PDF. |
| R20 | Al cerrar con cambios sin guardar aparece un diálogo propio, claro, sin preguntas de «Sí/No» ambiguas; Esc siempre elige la opción segura. |
| R21 | Recuperación automática de lo escrito si la app se cierra de golpe (se puede desactivar). |
| R22 | Barra de estado con palabras, caracteres, línea y columna, modo y estado de guardado. |
| R23 | Uso con teclado: atajos comunes (Ctrl+N, Ctrl+O, Ctrl+S, Ctrl+F, Ctrl+Z…) y foco visible. |

## Requerimientos no funcionales y de experiencia

| ID | Requerimiento |
| --- | --- |
| N1 | Ventana mínima 640×480 sin controles ocultos ni encimados. Debe verse bien también maximizada. |
| N2 | Todo lo interactivo debe parecer interactivo: hover visible y respuesta al pulsar. |
| N3 | Un botón que abre un panel o menú lo cierra con un segundo clic, y muestra que está abierto. |
| N4 | Las listas y menús largos se pueden recorrer con la rueda del mouse y con el teclado. |
| N5 | Contraste suficiente para lectura en ambos temas (texto normal ≥ 7:1, secundario ≥ 4,5:1). |
| N6 | Los textos de la interfaz están en español, son breves y coherentes. La interfaz no se llena de texto. |
| N7 | Avisos breves que confirman acciones, sin tapar contenido importante ni repetir lo obvio. |
| N8 | Coherencia: un mismo tipo de control se ve y se comporta igual en toda la app. |
