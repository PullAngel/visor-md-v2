# Visión

## El problema

Un `.md` en Windows tiene dos destinos malos y ninguno bueno. El Bloc de notas
lo muestra como texto plano. Un editor de código o una app de segundo cerebro
lo abren bien, pero son pesados, tardan en arrancar y están pensados para
*vivir adentro de ellos*, no para el gesto simple de "abrí este archivo".

Falta el equivalente a hacer doble clic en un PDF: algo que se abra al
instante, muestre el documento tal como fue pensado, y desaparezca cuando
terminaste. La v1 resolvió eso sobre WebView2. Funciona, pero pesa 30 MB en
disco y arranca en 3-4 segundos, porque levanta el motor de Edge entero para
mostrar un archivo de texto.

## La apuesta de la v2

Lo mismo, pero **nativo**: sin motor web, por debajo de 7 MB, con un arranque
que no se siente. La referencia de que es posible es Tinta. La diferencia es
que la v2 no la copia: le suma el modelo de seguridad de la v1, la profundidad
de producto de los mejores lectores, y una herramienta de estudio real.

## Para quién

- **Quien recibe `.md` de terceros** y quiere abrirlos sin que el archivo pueda
  hacer nada raro.
- **Quien estudia.** Este es el eje que más creció: no solo leer notas, sino
  trabajar sobre ellas —subrayar, repasar, medir— sin abrir una app de 300 MB
  para cada gesto. Ver `study-brainstorm.md`.
- **Quien ya tiene un segundo cerebro** y quiere una ventana rápida y segura
  para leer y anotar dentro de él, sin reemplazarlo.

## Los cinco valores, en orden

1. **Seguro por construcción.** No "sanitizamos bien": no hay motor de scripts
   que ejecutar, y el lenguaje elegido elimina de raíz los fallos de memoria.
2. **Liviano de verdad.** Objetivo <7 MB, ideal <6, techo duro 9,44. Todo lo
   que no entra es componente aparte o no existe.
3. **Rápido de verdad.** El arranque pesa tanto como el tamaño: es lo primero
   que se nota y lo que hace que quede como predeterminado.
4. **Útil todos los días.** Que un estudiante lo abra por gusto, no por
   obligación.
5. **Conectado, no encerrado.** Habla el formato de Obsidian, de Logseq y de un
   repo de GitHub. No pide que migres: se mete en lo tuyo.

## Portabilidad

Sin instalación obligatoria. Que se pueda descomprimir y correr desde un
pendrive, y que funcione en una VM descartable de Linux sin ceremonia.

**Windows 10/11 y las distribuciones estándar de Linux desde el primer día.**
macOS se compila y prueba en paralelo pero no se publicita hasta estar bien
probado; el razonamiento está en `architecture.md`.

## Subrayar sin romper el `.md`

Preguntaste cómo agregar resaltado sin dañar el archivo ni cómo lo interpreta
Obsidian. **Hay una respuesta buena, y es mejor de lo que esperabas.**

`==texto==` es la sintaxis de resaltado que **Obsidian ya usa de forma nativa**.
No es una invención nuestra: es un formato de facto que varios sabores de
Markdown entienden, y que en Obsidian se ve subrayado igual que acá. O sea que
incrustar el resaltado en el archivo **no rompe nada** y viaja con la nota.

Aun así el valor por defecto es **sidecar**, en un archivo paralelo:

| | Sidecar (por defecto) | Incrustado (`==texto==`) |
| --- | --- | --- |
| El `.md` queda | Intacto | Con las marcas dentro |
| En Obsidian se ve | No | Sí, resaltado |
| Si movés la nota | Se pierde si no va el sidecar | Viaja con ella |
| Si la edita otro | No molesta | Puede confundir a quien no lo espere |

Se cambia por documento, en un clic, y en los dos sentidos sin pérdida. El
sidecar es el valor por defecto porque un archivo ajeno no debería cambiar por
haberlo abierto.

## Qué NO quiere ser

- No reemplaza Obsidian ni Logseq. Es la ventana rápida que se abre *sobre* la
  bóveda de ellos.
- No es una nube ni un servicio. Todo local.
- No es un IDE.

## El criterio que zanja las discusiones

Cuando una función quiera entrar y no esté claro si vale la pena: **¿cabe en el
presupuesto?** y **¿puede un documento hostil abusar de ella?** Si engorda el
binario o abre una superficie de ataque, la respuesta por defecto es no.
