# Barra, menús, mosaico y ventanas · diseño

Fecha: 15 de noviembre de 2026. Responde a la segunda tanda de pedidos del
propietario tras probar `VisorMD-v2-nuevo.exe`. Las decisiones salen de aquí
y se integran en [`plan-ejecucion-2026-11-14.md`](plan-ejecucion-2026-11-14.md)
como D-29 a D-37 y las fases 10, 11 y 12.

**Cómo se hizo.** Una sola revisión con cinco miradas aplicadas por turnos
sobre el código y la captura del propietario: diseño visual, interacción,
accesibilidad, ingeniería y QA. No hubo agentes separados; cada decisión
indica qué mirada la motivó.

## Pedidos del propietario

| # | Pedido | Destino |
| --- | --- | --- |
| 1 | El texto del Índice se sale de su caja | Arreglado (10.1) |
| 2 | Botón día/noche junto a la chincheta y atajo con Ctrl | D-30, 10.2 |
| 3 | Mejor manejo de pantallas divididas: orden, tamaño, intercambio, sacar a ventana y volver | D-31, D-32, D-33, fases 11 y 12 |
| 4 | Las pestañas se repiten arriba y abajo al dividir | D-31 |
| 5 | «Más acciones» tiene 69 opciones en un solo botón | D-29, fase 10 |
| 6 | Poner todas las opciones de todos los menús en una tabla y replantear | Inventario de este documento |
| 7 | Poder elegir Visor MD como programa para `.md`, `.txt` y otros | D-34, 12.5 |
| 8 | Una frase quedó cortada («al momento de pasar el cursor por encima») | Supuesto: ver D-35 |

## 1. Hallazgos

### 1.1 Texto fuera de la caja (S2)

`abbreviated_label(texto, 48)` recortaba por **cantidad de caracteres**. La
columna de carpeta mide 288 px y una fila útil, unos 260: caben unos 34
caracteres, no 48. Ocurría en todas las listas de la columna (Índice, Notas,
Búsqueda, Notas que enlazan aquí). Corregido con `fit_menu_layout`, que mide el
texto y lo recorta con «…» según el ancho real y el atajo de la fila.

### 1.2 «···» concentra todo

`APP_ACTIONS` tiene 69 acciones. La barra superior solo tiene 6 botones, pero
queda espacio libre entre el selector de modo (termina en x = 488) y los
controles de ventana. Además hay duplicados:

- `WRITING_ACTIONS` («Más formato Markdown») repite 8 acciones de Bloques,
  Insertar y Listas.
- Insertar enlace, tabla, bloque de código, línea horizontal y recuadro salen
  en tres menús distintos.
- «Cerrar panel» y «Fijar pestaña» se parecen a «Cerrar pestaña».
- Los textos mezclan tres vocabularios (workspace, bóveda, carpeta).

### 1.3 Mosaico

Lo que hay hoy (`panes.rs`, `main.rs`):

| Capacidad | Estado |
| --- | --- |
| Dividir a la derecha o abajo | Sí |
| Hasta 4 paneles | Sí |
| Cambiar el tamaño de un panel | **No** (`fraction` fijo en 0,5, tope 0,25–0,75) |
| Intercambiar paneles | **No** |
| Soltar una pestaña sobre un panel | **No** |
| Reordenar pestañas arrastrando | Sí, solo en la tira |
| Sacar una pestaña a otra ventana | **No** |
| Cabecera de panel | Nombre, modo y cierre: repite lo que ya dice la tira |

El archivo aparece dos veces en pantalla: en la tira superior y en la cabecera
del panel.

### 1.4 Asociación de archivos

No existe. `docs/roadmap.md` la deja para el instalador. Claude, y cualquier
programa que use «Abrir con», solo ofrece los programas registrados en
Windows: hasta que Visor MD escriba su registro, no aparece. Sí existe lo
necesario del otro lado: `main` abre la primera ruta de la línea de comandos.

## 2. Inventario de menús

Todas las opciones de todos los menús desplegables de hoy y dónde van.

Leyenda de destino: **Archivo**, **Editar**, **Vista**, **Carpeta**,
**Formato**, **Párrafo**, **Insertar**, **Estudio**, **Copiar**. «Solo
paleta» son las que únicamente se encuentran buscando.

### 2.1 Paleta «···» (`APP_ACTIONS`, 69)

| Acción | Atajo | Destino |
| --- | --- | --- |
| Nuevo documento | Ctrl+N | Archivo (y botón) |
| Abrir documento | Ctrl+O | Archivo (y botón) |
| Guardar | Ctrl+S | Archivo (y botón) |
| Guardar como | Ctrl+Shift+S | Archivo |
| Exportar a PDF | — | Archivo |
| Cerrar pestaña | Ctrl+W | Archivo |
| Abrir carpeta | Ctrl+Shift+O | Archivo y Carpeta |
| Alternar lectura y edición | F2 | Vista y selector de modo |
| Comparar fuente y vista | F3 | Vista y selector de modo |
| Alternar disposición de comparación | — | Vista |
| Dividir a la derecha con otra pestaña | — | Vista |
| Dividir abajo con otra pestaña | — | Vista |
| Cerrar panel sin cerrar la pestaña | — | Vista («Quitar del mosaico») |
| Fijar o soltar pestaña | — | Vista |
| Cambiar tema día o noche | T (se retira) | Vista y botón; atajo nuevo Ctrl+Shift+D |
| Movimiento reducido | — | Vista |
| Ventana siempre encima | — | Vista y botón |
| Índice flotante | — | Vista |
| Buscar en documento | Ctrl+F | Editar y botón |
| Cortar | Ctrl+X | Editar |
| Insertar símbolo o emoji | — | Insertar |
| Insertar enlace | Ctrl+K | Insertar |
| Insertar enlace a otra nota | — | Insertar |
| Insertar tabla | — | Insertar |
| Insertar bloque de código | — | Insertar |
| Insertar recuadro | — | Insertar |
| Insertar línea horizontal | — | Insertar |
| Negrita | Ctrl+B | Formato |
| Cursiva | Ctrl+I | Formato |
| Tachado | — | Formato |
| Código en línea | — | Formato |
| Resaltar | — | Formato |
| Título 1 a Título 6 (6 acciones) | — | Párrafo |
| Lista con viñetas | — | Párrafo |
| Lista numerada | — | Párrafo |
| Tarea | — | Párrafo |
| Cita | — | Párrafo |
| Plegar o desplegar sección | — | Párrafo y barra de lectura |
| Pregunta de estudio | — | Estudio |
| Resumen de estudio | — | Estudio |
| Lista de conceptos | — | Estudio |
| Marcar como entendido | — | Estudio |
| Marcar como dudoso | — | Estudio |
| Marcar como pendiente | — | Estudio |
| Preparar resumen desde selección | — | Estudio |
| Preparar conceptos desde selección | — | Estudio |
| Preparar fragmentos para IA | — | Estudio |
| Copiar para Discord o correo | — | Copiar |
| Copiar tabla para hoja de cálculo | — | Copiar |
| Notas de la carpeta | Ctrl+Shift+T | Carpeta |
| Buscar en carpeta | Ctrl+Shift+F | Carpeta |
| Índice del documento | Ctrl+Shift+L | Carpeta y botón de lectura |
| Notas que enlazan aquí | Ctrl+Shift+B | Carpeta |
| Revisar enlaces a otras notas | — | Carpeta |
| Actualizar índice de carpeta | Ctrl+Shift+I | Carpeta |
| Cancelar actualización | — | Carpeta |
| Abrir recuperación | Ctrl+Shift+R | Carpeta |
| Recuperación local activada | — | Carpeta |

Los grupos `BlockTools`, `ListTools`, `InsertTools`, `StudyTools`,
`WritingTools`, `WorkspaceHub` y `CommandPalette` no son acciones: son los
menús mismos. Desaparecen como filas de la paleta.

**Nuevas** (no existen hoy): Deshacer y Rehacer (Editar), Copiar y Pegar
(Editar; tarea 3.13), Mover a ventana nueva y Mover a otra ventana (Vista),
Hacer de Visor MD el programa para `.md` y `.txt` (Archivo).

### 2.2 Menús de grupo de la barra de edición

| Menú actual | Contenido actual | Destino |
| --- | --- | --- |
| Bloques (10) | H1 a H6, cita, código, regla, recuadro | Título 1–6 y Cita → Párrafo; código, regla, recuadro → Insertar |
| Listas (3) | viñetas, numerada, tarea | Párrafo |
| Insertar (7) | enlace, enlace a nota, tabla, recuadro, código, regla, símbolo | Insertar, sin cambio |
| Estudio (9) | 6 insertar + 3 preparar | Estudio, con encabezados «Insertar» y «Desde la selección» |
| Más (16) | 8 repetidas + 7 formato + orientación | **Se elimina**; la orientación va a Vista |

### 2.3 Menú contextual (`context_actions`)

| Modo | Hoy | Nuevo |
| --- | --- | --- |
| Lectura | Copiar texto, Markdown, para Discord; tarea; tabla | Marcar tarea; Copiar ▸ (texto, Markdown, plataforma, tabla); Buscar; Dividir; Mover a ventana nueva; Fijar; Cerrar |
| Edición | Pegar, Negrita, Cursiva, Enlace, Tarea + 5 grupos | Cortar, Copiar, Pegar, Deshacer; Formato ▸; Párrafo ▸; Insertar ▸; Estudio ▸; enlace y tabla según el objeto bajo el puntero |

### 2.4 Selector de división y del hub

El selector «Dividir con…» muestra «Nuevo documento», «Abrir archivo» y las
pestañas. Queda igual. El hub (`WORKSPACE_HUB_ACTIONS`, 8) pasa a ser el menú
Carpeta.

## 3. Barra superior

### 3.1 Distribución

```text
Fila 1 (28 px)
  + 📂 💾 🔍 | Leer · Editar · Comparar | Archivo ▾  Vista ▾  Carpeta ▾  ⌘ | ☾ 📌 – □ ✕

Fila 2 (contextual)
  Lectura:  Índice · Plegar sección · Copiar ▾
  Edición:  B I S ` ▮ | Párrafo ▾  Tarea  Cita  Enlace  Código  Tabla | Insertar ▾  Estudio ▾
```

- La fila 1 deja cuatro botones directos (Nuevo, Abrir, Guardar, Buscar). El
  selector de modo pasa de x = 260 a x = 180.
- Los menús Archivo, Vista y Carpeta salen a la derecha del selector. El último
  botón («⌘») es la paleta: se sigue abriendo con Ctrl+Shift+P y sirve para
  **buscar** cualquier acción; ya no es el menú de todo.
- Cada botón de menú abre su lista **debajo de sí mismo**. Con un menú abierto,
  pasar el puntero sobre otro botón de menú abre el otro (como una barra de
  menús de escritorio). Pulsar de nuevo el mismo botón lo cierra (D-04).
- Teclado: F10 abre Archivo; ← y → cambian de menú; ↑ y ↓ recorren; Enter
  elige; Esc cierra y devuelve el foco al documento.
- Editar, Formato, Párrafo, Insertar y Estudio viven en la fila 2 (contextual)
  porque solo tienen sentido al escribir. En lectura, la fila 2 trae Índice,
  Plegar sección y Copiar.

### 3.2 Adaptación por ancho

Un solo cálculo, `top_bar_layout(window_width, mode)`, reparte el espacio por
prioridad. Si algo no cabe pasa al menú «≡» sin desaparecer.

| Prioridad | Elemento | Ancho | Si no cabe |
| --- | --- | --- | --- |
| 1 | Controles de ventana (cerrar, maximizar, minimizar, chincheta) | 4 × 36 | Nunca se quita |
| 2 | Selector de modo | 236 | Nunca se quita |
| 3 | Nuevo, Abrir, Guardar, Buscar | 4 × 40 | Nunca se quita |
| 4 | Botón día/noche | 36 | Se quita por debajo de 640 px (queda en Vista y en Ctrl+Shift+D) |
| 5 | Archivo, Vista, Carpeta con rótulo | ~72 cada uno | Pasan a íconos con ▾ |
| 6 | Íconos con ▾ (34 px) | 3 × 34 | Pasan a un solo «≡» |
| 7 | «≡» (40 px), menú único con tres secciones | 40 | — |

A 640 px: 12 + 160 (directos) + 8 + 236 (modo) = 416; «≡» 40 → 456; controles
5 × 36 = 180 → empiezan en 460. Cabe con 4 px de aire. A 860 px o más hay
rótulos completos. Una prueba recorre los anchos de 640 a 2000 px y exige que
ningún elemento se solape ni salga de la ventana (como ya hacen las pruebas de
la barra contextual).

### 3.3 Menú con secciones (D-36)

Las filas de los menús ya son listas con rueda, barra de posición y hover
(fase 3). Se agrega una capa:

```text
enum MenuEntry {
    Item(AppAction),
    Heading(&'static str),   // «Modo», «Mosaico», «Apariencia»
}
```

- Los encabezados hacen de separadores y no se seleccionan: las flechas los
  saltan. No hay `Separator` porque todas las filas miden lo mismo.
- Las alternancias muestran «✓» a la izquierda cuando están activas (en lugar
  del texto «activado ✓» al final).
- El atajo va a la derecha (ya existe).
- Una acción nunca aparece dos veces en el mismo menú.

### 3.4 Contenido de cada menú

**Archivo**: Nuevo documento · Abrir documento · Abrir carpeta de apuntes ·
Guardar · Guardar como · Exportar a PDF · Cerrar pestaña · Hacer de Visor MD el
programa para .md y .txt…

**Editar** (solo con edición o comparación; en lectura se muestra atenuado con
«Cambiá a Editar»): Deshacer · Rehacer · Cortar · Copiar · Pegar · Buscar en
documento.

**Vista**: Modo (Leer, Editar, Comparar, Disposición de la comparación) ·
Mosaico (Dividir a la derecha, Dividir abajo, Quitar del mosaico, Mover a
ventana nueva, Mover a otra ventana ▸) · Pestaña (Fijar o soltar) ·
Apariencia (Tema día o noche ✓, Movimiento reducido ✓, Ventana encima ✓,
Índice flotante ✓).

**Carpeta**: Notas de la carpeta · Buscar en carpeta · Índice del documento ·
Notas que enlazan aquí · (separador) Actualizar · Cancelar actualización ·
Revisar enlaces a otras notas · (separador) Abrir recuperación · Recuperación
local ✓ · Abrir otra carpeta.

**Párrafo**, **Insertar**, **Estudio**, **Copiar**: ver 2.1 y 2.2.

## 4. Botón día/noche (D-30)

- Posición: primero de los controles de ventana, a la izquierda de la
  chincheta. Orden final: ☾ 📌 – □ ✕.
- Ícono: luna en el tema de día (lleva a la noche) y sol en el de noche. Se
  dibuja con trazos, igual que los otros controles, con hover redondeado (D-14).
- Tooltip: «Cambiar a tema de noche · Ctrl+Shift+D» y su espejo.
- Atajo: **Ctrl+Shift+D** (D de «día»). No choca con ningún atajo de la
  aplicación ni usa AltGr, que en teclados en español es Ctrl+Alt.
- La tecla T sola se retira: hoy cambia el tema en lectura y por eso deja de
  servir si hay un campo de texto, el Índice o el modo de escritura abiertos.
  `AppAction::ToggleTheme` pasa a decir «Cambiar tema día o noche · Ctrl+Shift+D».
- La tecla se resuelve **antes** que la escritura en campos y atajos de lista,
  igual que Ctrl+Shift+T, de modo que funciona con cualquier panel abierto.

## 5. Mosaico y pestañas

### 5.1 Principio (D-31)

**Cada archivo abierto aparece una sola vez en pantalla.**

| Estado | Tira superior | Cabecera de panel |
| --- | --- | --- |
| Un solo panel | Todas las pestañas, la activa resaltada | No hay |
| Mosaico (2 a 4 paneles) | Solo las pestañas que **no** están en un panel (archivos «guardados para después») y «+» | Es la pestaña del archivo: nombre, indicador de cambios, selector de modo y botón «Quitar del mosaico» |

Así desaparece la repetición y la jerarquía se entiende: lo que ves en un
panel está en el panel; lo demás, arriba.

- El botón de la cabecera usa un ícono distinto del ✕ de la pestaña (flecha
  hacia la tira) y el tooltip «Quitar del mosaico · el archivo sigue abierto
  arriba». Cerrar el archivo es Ctrl+W o el menú.
- El panel enfocado tiene un filete de acento de 2 px arriba y su cabecera con
  el verde de pestaña activa (`palette.tab`); los demás, `tab_idle`.
- Si la tira queda vacía en mosaico, muestra solo «+» y la indicación «Arrastrá
  una cabecera hasta aquí para quitarla del mosaico».

### 5.2 Operaciones (D-32)

| Operación | Gesto | Detalle |
| --- | --- | --- |
| Cambiar tamaño | Arrastrar el divisor | Franja de 6 px con cursor de redimensionar; límites 15 %–85 %; doble clic vuelve a 50 %; el mínimo útil de un panel es 240 px |
| Cambiar tamaño con teclado | Alt+Shift+← → ↑ ↓ | Pasos de 5 % |
| Intercambiar de lugar | Arrastrar una cabecera sobre otra | Aparece un fantasma de la cabecera y el panel destino se resalta; al soltar se intercambian los documentos y el desplazamiento de cada uno |
| Reordenar dentro del mosaico | Igual que intercambiar | Con 3 o 4 paneles se intercambian de a dos |
| Poner un archivo en el mosaico | Arrastrar una pestaña de la tira hasta un panel | Soltar al centro reemplaza (el archivo reemplazado vuelve a la tira); soltar en un borde (25 % exterior) divide hacia ese lado |
| Quitar del mosaico | Arrastrar la cabecera hasta la tira, o el botón | El panel se cierra y el archivo vuelve a la tira |
| Dividir | Menú Vista, clic derecho, o arrastrar al borde | Con vista previa translúcida de la zona |
| Cerrar el último panel | — | El mosaico termina y vuelve la tira completa |

Todas las zonas de soltado se dibujan con un rectángulo de acento al 18 % y
borde de acento; se cancelan con Esc o soltando fuera de cualquier zona.
Con Reducir movimiento no hay animación del fantasma, solo el resalte.

### 5.3 Estado y reglas

- `DocumentPaneTree::Split.fraction` se mantiene; se amplía el rango válido a
  0,15–0,85 (`normalized_pane_fraction`).
- Nuevo `DocumentPaneTree::swap_panes(a, b)` y `set_fraction_at(path, value)`
  con prueba de que conservan el desplazamiento por panel y el conteo de
  hojas.
- Cerrar una pestaña conserva la división: se quitan solo las hojas de ese
  documento (tarea 5.5 existente).
- Un panel nuevo abre en Leer (tarea 5.6 existente).
- La cabecera de panel usa el mismo componente de selector de modo que la
  barra, a escala 0,85 (tarea 5.7 existente, ahora dentro de 11.1).

## 6. Ventanas (D-33)

### 6.1 Decisión técnica

**Un proceso por ventana**, coordinados por un registro en disco. La
alternativa, varias ventanas dentro de un proceso, obliga a partir `App` (22 000
líneas, un solo `softbuffer::Surface`) y deja a todas las ventanas caer
juntas si una falla. El proceso por ventana ya aísla fallos, usa lo que hay y
resuelve también la apertura desde el Explorador.

- **Registro**: `%LOCALAPPDATA%\VisorMD\ventanas\<pid>.json` con `pid`, título,
  rectángulo de pantalla y hora de latido (se actualiza cada 2 s). Un archivo
  con latido de más de 6 s se ignora y se borra.
- **Buzón**: `%LOCALAPPDATA%\VisorMD\buzon\<pid>\<n>.json`. Un mensaje es
  `{ "abrir": ["ruta", …], "recuperacion": "ruta" }`. La ventana destino lo lee
  al recibir el foco o con un temporizador de 500 ms (sin hilo por mensaje) y lo
  borra al procesarlo.
- **Documentos sin guardar**: se transfieren por una **copia de recuperación**
  (módulo `recovery.rs` ya existente) que el destino abre como pestaña sucia
  con su nombre original. El origen la cierra solo cuando el destino confirma
  con un mensaje de vuelta.
- Seguridad: el buzón y el registro viven en el perfil de la persona usuaria;
  las rutas se tratan como una apertura manual más (sin seguir enlaces ni
  UNC inesperados, ver `threat-model.md`). Nunca se ejecuta contenido.

### 6.2 Gestos

| Gesto | Resultado |
| --- | --- |
| Arrastrar una pestaña (tira o cabecera de panel) y soltarla fuera de la ventana | Abre una ventana nueva en el punto de soltado con ese archivo y lo quita de la actual |
| Soltarla sobre la tira o un panel de **otra** ventana de Visor MD | Se une a esa ventana; con la tira, como pestaña; con un panel, reemplaza o divide según la zona |
| Vista ▸ Mover a ventana nueva | Igual que arrastrar fuera, sin mouse |
| Vista ▸ Mover a otra ventana ▸ (lista de ventanas por título) | Envía la pestaña a esa ventana |
| Doble clic en una cabecera de panel | Convierte ese panel en ventana |
| Última pestaña de una ventana movida | La ventana vacía se cierra sola |

Durante el arrastre fuera de la ventana el cursor lleva un fantasma de la
pestaña. Se decide el destino con `GetCursorPos` y los rectángulos del
registro. Si la apertura falla, la pestaña sigue en su ventana de origen y se
muestra un aviso de error.

### 6.3 Archivos abiertos desde fuera (Explorador, Claude)

Ajuste **«Abrir archivos externos»**, en el diálogo de la asociación (7.2):
*en la ventana abierta* (por defecto) o *en una ventana nueva*. Si hay una
ventana abierta y el ajuste es el primero, el proceso nuevo deja el mensaje en
el buzón, activa la ventana y sale en menos de 100 ms. Se aceptan **varias
rutas** en la línea de comandos: cada una abre una pestaña (hoy solo se abre la
primera).

## 7. Asociación de archivos (D-34)

### 7.1 Por qué hoy aparece otro programa

La lista «Abrir con» de Windows se arma con los programas **registrados** para
cada extensión. Visor MD nunca se registró, así que no figura. Windows 10 y 11
tampoco dejan que un programa se declare predeterminado por su cuenta: puede
registrarse y **pedir** que la persona lo elija.

### 7.2 Qué hace Visor MD

Menú Archivo ▸ «Hacer de Visor MD el programa para .md y .txt…» abre un
diálogo:

```text
Abrir con Visor MD
  [x] Markdown (.md, .markdown, .mdown, .mkdn)
  [x] Texto (.txt)
  [ ] Otros que Visor MD sabe abrir (.json, .yaml, .yml, .toml, .csv)

  Abrir archivos externos:  (•) en la ventana abierta   ( ) en una ventana nueva

  [Registrar y elegir en Windows]   [Quitar el registro]   [Cancelar]
```

«Registrar y elegir en Windows» escribe, **sin permisos de administrador**
(solo `HKEY_CURRENT_USER`), y luego abre la pantalla de aplicaciones
predeterminadas de Windows con `ms-settings:defaultapps`. El diálogo explica en
una línea qué falta: «Windows te pide confirmar la elección: buscá Visor MD».

Claves (todas bajo `HKCU\Software`):

| Clave | Valor |
| --- | --- |
| `Classes\VisorMD.Documento` | `(Predeterminado)` = «Documento de Visor MD» |
| `Classes\VisorMD.Documento\DefaultIcon` | `"<exe>",0` |
| `Classes\VisorMD.Documento\shell\open\command` | `"<exe>" "%1"` |
| `Classes\<.ext>\OpenWithProgids` | valor `VisorMD.Documento` (vacío) |
| `Classes\Applications\visor-md.exe\shell\open\command` | `"<exe>" "%1"` |
| `Classes\Applications\visor-md.exe\SupportedTypes\<.ext>` | vacío |
| `VisorMD\Capabilities` | `ApplicationName`, `ApplicationDescription` |
| `VisorMD\Capabilities\FileAssociations\<.ext>` | `VisorMD.Documento` |
| `RegisteredApplications` | `VisorMD` = `Software\VisorMD\Capabilities` |

Después se llama a `SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, …)` para
que el Explorador lo note. «Quitar el registro» borra esas claves y nada más.

Se invoca `%SystemRoot%\System32\reg.exe` con argumentos fijos, sin shell y sin
dependencias nuevas. Las rutas se citan y se validan (sin comillas ni
caracteres de control) antes de armar el comando. Linux queda para el paquete
(`.desktop` con `MimeType=text/markdown;text/plain;` y `xdg-mime default`); lo
mismo vale para el instalador de Windows, que reutiliza este registro.

### 7.3 Primer arranque

La pantalla de bienvenida (D-27) agrega una línea con un enlace: «¿Querés
abrir tus .md con Visor MD? Configurarlo». Una sola vez; se recuerda con
`Settings`.

## 8. Decisiones nuevas

Se copian al plan como D-29 a D-37.

| ID | Decisión |
| --- | --- |
| D-29 | La paleta «···» deja de ser el menú de todo. Los menús son Archivo, Vista, Carpeta (siempre) y Editar, Formato, Párrafo, Insertar, Estudio, Copiar (contextuales). «Más formato Markdown» se elimina. Toda `AppAction` está en al menos un menú o marcada «solo paleta»; una prueba lo exige |
| D-30 | Botón día/noche como primer control de ventana; atajo Ctrl+Shift+D; la tecla T sola deja de cambiar el tema |
| D-31 | Cada archivo aparece una sola vez: con mosaico, la tira solo lista lo que no está en un panel y la cabecera de panel es la pestaña del archivo |
| D-32 | Mosaico: divisor arrastrable (15–85 %, doble clic 50 %), intercambio por arrastre, zonas de soltado, teclado Alt+Shift+flechas |
| D-33 | Ventanas: un proceso por ventana, registro y buzón en `%LOCALAPPDATA%\VisorMD`, transferencia de lo sin guardar por copia de recuperación |
| D-34 | Asociación: registro por usuario con `reg.exe` más `ms-settings:defaultapps`, diálogo propio, ajuste «Abrir archivos externos», varias rutas por línea de comandos |
| D-35 | Texto recortado por ancho medido; toda fila recortada muestra el texto completo en un tooltip (usa D-15) al pasar el puntero. Supuesto sobre la frase cortada del propietario; confirmar |
| D-36 | Componente de menú con encabezados, separadores y marca ✓ (`MenuEntry`) |
| D-37 | Barra superior adaptable: `top_bar_layout` reparte por prioridad; lo que no cabe pasa a «≡» |

## 9. Riesgos

| Riesgo | Mitigación |
| --- | --- |
| La barra nueva rompe pruebas de geometría y referencias | Se hace en una fase propia con una prueba de solapes de 640 a 2000 px y referencias regeneradas a propósito |
| Arrastrar fuera de la ventana depende de la captura del puntero | winit sigue entregando `CursorMoved` mientras el botón esté pulsado; si el sistema corta, se usa `GetCursorPos` en el soltado |
| Perder un documento sin guardar al moverlo | El origen cierra la pestaña solo tras la confirmación del destino; la copia de recuperación queda hasta entonces |
| Registro de Windows mal escrito | Se escribe y se **lee de vuelta** para verificar; «Quitar» es simétrico; la prueba usa una raíz de prueba (`HKCU\Software\VisorMDPrueba`) y no el registro real |
| Latidos de ventanas muertas | Se ignoran tras 6 s y se borran al arrancar |
| El mosaico oculta pestañas y confunde | La tira muestra el texto «n abiertas en el mosaico» como indicación; las cabeceras llevan el verde de pestaña |
