# Conectividad con segundos cerebros

La idea central: un segundo cerebro basado en Markdown **ya es una carpeta de
archivos**. Conectarse a él no es integrarse con una API, es entender esa
carpeta mejor que un explorador de archivos. Es la conexión más potente y a la
vez la más barata y la más segura: sin tokens, sin red, sin superficie nueva.

## El formato común

Casi todas las herramientas de segundo cerebro convergieron en lo mismo:
archivos `.md` planos, en una carpeta, con enlaces entre ellos. Las diferencias
son de dialecto, no de fondo.

| Herramienta | Formato | Qué la distingue |
| --- | --- | --- |
| **Obsidian** | `.md` + `.obsidian/` | Wikilinks `[[nota]]`, callouts, `==resaltado==`, frontmatter YAML |
| **Logseq** | `.md` en `pages/` y `journals/` | Todo es una lista anidada; cada viñeta es un bloque con identidad |
| **Foam** | `.md` en un repo | Wikilinks sobre VS Code; es Obsidian sin la app |
| **Dendron** | `.md` con nombres jerárquicos | La jerarquía está en el nombre: `proyecto.tema.subtema.md` |
| **Zettlr / Joplin** | `.md` + adjuntos | Markdown estándar, sin dialecto propio |
| **Repo de GitHub** | `.md` + `.git/` | Enlaces relativos, GFM |

**La conclusión práctica:** soportar bien **wikilinks + frontmatter + callouts +
`==resaltado==` + enlaces relativos** cubre de una vez Obsidian, Foam, Dendron,
Zettlr, Joplin y cualquier repo. No hay que integrar con cada herramienta: hay
que hablar bien el dialecto común.

Logseq es el único que pide algo extra (su modelo de bloques) y por eso queda
como soporte de segunda: sus archivos se leen perfecto como Markdown normal,
solo que sin entender las referencias a nivel de bloque.

## Obsidian

- **Wikilinks** `[[nota]]` y `[[nota|alias]]`: se resuelven contra el índice de
  la bóveda, se navegan con un clic, y se marcan visualmente cuando apuntan a
  una nota que no existe: en Obsidian eso es trabajo pendiente, no un error.
- **Enlaces a encabezados y bloques**: `[[nota#encabezado]]`, `[[nota^bloque]]`.
- **Embeds** `![[nota]]`: en la v2.0 como enlace destacado; el embed real es
  incremental, con tope de profundidad y detección de ciclos.
- **Callouts** `> [!info]`: casi idénticos a las alertas de GitHub que la v1 ya
  renderiza. Mismo mecanismo.
- **`==resaltado==`**: nativo de Obsidian y la base de nuestro resaltado
  incrustado. Ver `vision.md`.
- **Frontmatter YAML**: se oculta como en GitHub, y sus campos alimentan el
  índice.
- **`.obsidian/` no se toca.** Visor MD v2 es una ventana sobre la bóveda, no un
  segundo dueño de su configuración.

## GitHub

No un cliente de la API (eso sería red y tokens) sino entender un repositorio
**ya clonado**:

- GFM fiel: tablas, tareas, alertas, autolinks, como en github.com.
- Enlaces relativos correctos, resolviendo contra la raíz del repo.
- Detección de la raíz buscando `.git/`, para resolver rutas absolutas del repo
  (`/docs/x.md`) igual que lo haría GitHub.
- README renderizado automáticamente al abrir una carpeta.

**Descartado por ahora:** abrir directo desde una URL de GitHub. Introduce red y
descarga automática de contenido no confiable, justo lo que la política evita.

## Guardar en un servicio de sincronización

Preguntaste por guardar directamente en un drive. La respuesta es que **ya
funciona sin que hagamos nada**: una carpeta de OneDrive, Dropbox o Google Drive
sincronizada es, para el sistema operativo, una carpeta normal. Abrir una bóveda
que vive ahí no requiere ninguna integración.

Lo que **no** vamos a hacer es hablar con la API de esos servicios: eso sí sería
red, tokens y una superficie de ataque nueva por una comodidad menor.

Lo que sí hay que cuidar, y va al roadmap como caso de prueba: un archivo que
cambia por debajo mientras lo tenés abierto porque el cliente de sincronización
lo bajó. La detección de cambios externos de la v1 ya cubre el caso; hay que
probarlo específicamente contra una carpeta sincronizada de verdad.

## Puerta a IA local

Pediste una puerta segura a IA local, sin que agregue complejidad ni vectores
innecesarios. El diseño que cumple las dos cosas:

- La IA corre en **su propio proceso**, sin acceso al sistema de archivos.
- El núcleo le manda **texto** y recibe **texto**. Nada más. No hay rutas, no
  hay comandos, no hay archivos cruzando esa frontera.
- El componente **no tiene cliente HTTP**. No puede llamar a una API remota
  aunque quisiera.
- Es **opt-in y por documento**: la IA no toca una nota hasta que se lo pedís en
  esa nota.
- El camino más barato y el que se evalúa primero: **hablar con un Ollama que ya
  tengas instalado**, por loopback. Cero peso agregado, cero modelo que
  mantener, y el principio local se respeta porque Ollama corre en tu máquina.

Con esas cinco reglas, la superficie que agrega es un socket local hablando
texto plano con un proceso sin privilegios. Es de las integraciones más seguras
posibles. El detalle está en `inference.md`.

## Lo que todas las conexiones comparten

- **Cero red.** Todo es lectura de carpetas locales.
- **Cero credenciales.** No hay login, no hay token.
- **Lectura y anotación, no gestión.** Visor MD v2 no se vuelve el dueño de tu
  bóveda ni de tu repo. Se mete adentro, con respeto, y se va.
