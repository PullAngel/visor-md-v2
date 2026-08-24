# Conectividad: Obsidian y GitHub

La idea central (ADR-6): un segundo cerebro basado en Markdown **ya es una
carpeta de archivos**. Conectarse a él no es integrarse con una API, es
entender esa carpeta mejor que un explorador de archivos común. Es la conexión
más potente y a la vez la más barata y la más segura —sin tokens, sin red, sin
superficie nueva.

## Obsidian

Una bóveda de Obsidian es una carpeta con archivos `.md` y una subcarpeta
`.obsidian/` de configuración. Visor MD v2 la abre como workspace y entiende lo
que la hace una bóveda y no una carpeta cualquiera:

- **Wikilinks `[[nota]]` y `[[nota|alias]]`.** Se resuelven contra la bóveda,
  se navegan con un clic, y se avisa visualmente cuando apuntan a una nota que
  no existe (un "enlace roto", que en Obsidian es una señal de trabajo
  pendiente, no un error).
- **Enlaces a encabezados y bloques** (`[[nota#encabezado]]`, `[[nota^bloque]]`).
- **Embeds `![[nota]]`**: mostrar el contenido de otra nota embebido. En la
  v2.0 puede empezar como un enlace destacado; el embed real es incremental.
- **Callouts** (`> [!info]`, `> [!warning]`, ...): son casi idénticos a las
  alertas de GitHub que la v1 ya renderiza. Reutilizan el mismo mecanismo.
- **Se respeta `.obsidian/`**: no se toca, no se rompe. Visor MD v2 es una
  ventana de solo-lectura-y-anotación sobre la bóveda, no un segundo dueño de
  su configuración.

**Lo que NO hace:** no reemplaza a Obsidian, no sincroniza, no toca los
plugins de Obsidian. Es la ventana rápida y liviana para *leer y anotar* dentro
de una bóveda sin abrir la app pesada. Alguien puede tener Obsidian para su
trabajo profundo y Visor MD v2 como el visor predeterminado de Windows para
abrir cualquier `.md` suelto de esa bóveda al instante.

**Por qué esto es un diferenciador real:** ninguno de los diez proyectos
estudiados entiende wikilinks. Es la función que hace que un usuario de
Obsidian adopte Visor MD v2 sin fricción —abre sus notas y *funcionan*— y es
barata de construir porque es solo resolución de nombres contra una carpeta.

## GitHub

No un cliente de la API de GitHub —eso sería red, tokens y superficie nueva—
sino entender un **repositorio ya clonado** en disco:

- **GFM fiel**: tablas, tareas, alertas, autolinks, tal como se ven en
  github.com. La v1 ya apunta a esto; la v2 lo mantiene.
- **Enlaces relativos correctos**: un `[ver la guía](../docs/guia.md)` dentro
  de un repo navega al archivo correcto, entendiendo la raíz del repo.
- **Detectar la raíz del repo** (buscar `.git/`) para resolver enlaces desde la
  raíz (`/docs/x.md`) igual que lo haría GitHub.
- **Renderizar el README** de una carpeta automáticamente al abrirla, como hace
  github.com al entrar a un directorio.

**Alternativa más ambiciosa, descartada por ahora:** abrir directamente desde
una URL de GitHub (clonar o descargar el `.md`). Se descarta porque introduce
red y la posibilidad de descargar contenido no confiable de forma automática
—exactamente lo que la política de seguridad evita. Si algún día se hace, sería
opt-in explícito y con el mismo tratamiento de contenido hostil que todo lo
demás.

## Lo que ambas conexiones comparten

- **Cero red.** Todo es lectura de carpetas locales.
- **Cero configuración de credenciales.** No hay login, no hay token.
- **Solo lectura y anotación**, no gestión: Visor MD v2 no se vuelve el dueño
  de tu bóveda ni de tu repo. Se mete adentro, con respeto, y se va.

Esa es la diferencia entre "conectar con tu segundo cerebro" y "pedirte que te
mudes al nuestro". La v2 hace lo primero.
