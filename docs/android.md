# Visor MD para Android: preparación

Estado: **futuro, no iniciado**. Este documento fija la dirección para que el
trabajo de escritorio no cierre caminos y para que, cuando llegue el momento, la
versión móvil sea el mismo producto con el mismo nombre, no una reescritura.

## Qué sería Visor MD móvil

Una app simple y segura para **leer y editar ligeramente** Markdown en el
teléfono o la tablet:

- abrir un `.md` desde el gestor de archivos, una descarga o "Compartir con";
- lectura editorial con los mismos temas Papel + Tinta;
- edición de fuente básica, guardado atómico y detección de conflictos;
- copiar un bloque de código, buscar en el documento y ver el índice.

Quedan fuera de la primera versión móvil: bóvedas completas, divisiones,
fijar encima y la paleta de comandos. Se agregan solo con evidencia de uso.

## Por qué la arquitectura actual ya ayuda

| Pieza | Situación | Portabilidad |
| --- | --- | --- |
| `comrak`, `ropey`, `parley`, `swash`, `tiny-skia` | Rust puro, sin C ni WebView | Compilan para Android sin cambios |
| `winit` | Soporta Android mediante `android-activity` | La ventana y el bucle de eventos se reutilizan |
| `softbuffer` | Soporta Android | El framebuffer por software se reutiliza |
| `src/editor.rs`, `src/limits.rs`, `src/dialog.rs` | Sin APIs de plataforma | Se reutilizan tal cual |
| `rfd` (diálogos de archivo) | Sin soporte Android | Se reemplaza por el selector del sistema (SAF) |
| `arboard` (portapapeles) | Sin soporte Android | Se reemplaza por `ClipboardManager` vía JNI |
| `std::fs` directo en `files.rs` y `vfs.rs` | Android entrega `content://`, no rutas | Necesita una capa de almacenamiento |

## Seguridad móvil por diseño

- **Sin permiso `INTERNET` en el manifiesto.** Es la garantía más fuerte
  posible de que la app no se conecta: el sistema operativo lo impide.
- Sin permisos de almacenamiento amplio: solo los documentos que la persona
  elige mediante el selector del sistema (Storage Access Framework).
- Mismo modelo de contenido inerte: ningún documento ejecuta nada, HTML visible
  como texto, límites de tamaño y anidamiento compartidos con escritorio.
- Firma de release con clave propia, guardada fuera del repositorio.

## Preparación que ya conviene hacer en escritorio

Cada punto mejora también la versión de escritorio; ninguno agrega código
móvil todavía.

1. **Núcleo sin plataforma.** Extraer de `main.rs` el modelo documental
   (`Block`, `parse_blocks`, inline, límites, mediciones) a módulos que no
   importen `winit`, `rfd`, `arboard` ni `std::fs`. A futuro serán un crate
   `visor-core` compartido.
2. **Almacenamiento como capacidad.** Que abrir y guardar pasen por una
   interfaz pequeña (leer bytes, escribir de forma atómica, identidad del
   documento) implementada hoy con `std::fs` y mañana con `content://`.
3. **Escala de interfaz.** Las medidas del chrome son píxeles fijos; en un
   teléfono deben multiplicarse por la densidad. Pasar las constantes de UI por
   un factor de escala corrige además el DPI alto en escritorio.
4. **Entrada por intención.** Mantener que botones, menús y atajos invoquen el
   mismo catálogo de `AppAction`; el toque y los gestos se agregan como otra
   fuente de las mismas acciones.
5. **Portapapeles y diálogos detrás de funciones propias.** Ya existen
   `write_text_to_clipboard` y `file_dialog`; conservarlos como único punto de
   contacto con la plataforma.

## Dificultades conocidas

- **Teclado en pantalla e IME.** Es lo más delicado: hay que mostrar y ocultar
  el teclado y recibir composición de texto. `winit` lo cubre parcialmente.
- **Ciclo de vida.** Android puede pausar o destruir la app en cualquier
  momento: la recuperación local debe escribirse al pausar.
- **Distribución.** Google Play exige cuenta de desarrollador (pago único) y
  verificación de identidad; F-Droid es gratuito y exige compilación
  reproducible desde el código fuente, algo que ya buscamos.

## Orden sugerido cuando se active

1. Prototipo que abre un `.md` incluido en la app y lo muestra en lectura.
2. Abrir desde el selector del sistema y desde "Compartir con".
3. Edición de fuente con teclado en pantalla y guardado atómico.
4. QA en dos teléfonos reales y una tablet, tema claro y oscuro.
5. Publicación en F-Droid y, si conviene, Google Play.
