# Visión

## El problema

Un `.md` en Windows tiene dos destinos malos y ninguno bueno. El Bloc de notas
lo muestra como texto plano. Un editor de código o una app de segundo cerebro
(Obsidian, Logseq) lo abren bien, pero son pesados, tardan en arrancar y están
pensados para *vivir adentro de ellos*, no para el gesto simple de "abrí este
archivo y déjame leerlo".

Falta el equivalente a hacer doble clic en un PDF: algo que se abra al
instante, muestre el documento tal como fue pensado, y desaparezca cuando
terminaste. La v1 de Visor MD resolvió eso sobre WebView2. Funciona, pero pesa
30 MB en disco y arranca en 3-4 segundos, porque levanta el motor de Edge
entero para mostrar un archivo de texto.

## La apuesta de la v2

Lo mismo, pero **nativo**: sin motor web, sin navegador empaquetado, por
debajo de 7 MB, con un arranque que no se siente. La referencia de que esto es
posible es Tinta (1,9 MB, <100 ms, C++ nativo). La diferencia es que la v2 no
copia a Tinta: le suma el modelo de seguridad de la v1, la profundidad de
producto de los mejores readers, y una conexión de primera clase con los
segundos cerebros que la gente ya usa.

## Para quién

- **Quien recibe `.md` de terceros** — de un repo, de un conversor de PDF, de
  una IA — y quiere abrirlos sin que el archivo pueda hacer nada raro.
- **Quien estudia y toma notas** en Markdown, y quiere leerlas, revisarlas y
  repasarlas sin abrir una app de 300 MB para cada gesto.
- **Quien ya tiene un segundo cerebro** (una bóveda de Obsidian, un repo de
  documentación en GitHub) y quiere una ventana rápida, liviana y segura para
  *leer y anotar* dentro de él, sin reemplazarlo.

## Los cuatro valores, en orden

1. **Seguro por construcción.** No "sanitizamos bien"; directamente no hay
   motor de scripts que ejecutar, y el lenguaje elegido elimina de raíz los
   fallos de memoria que son el otro gran vector en lectores nativos.
2. **Liviano de verdad.** <7 MB es un requisito, no un deseo. Todo lo que no
   entre en ese presupuesto se pospone o se hace opcional.
3. **Cómodo y potente.** Workspace persistente, wikilinks, repaso para
   estudio, edición estructural. No un juguete: una herramienta de trabajo.
4. **Conectado, no encerrado.** Habla el formato de Obsidian y de GitHub. No
   pide que migres a su formato; se mete en el tuyo.

## Qué NO quiere ser

- No es un reemplazo de Obsidian ni de Logseq. Es la ventana rápida que se
  abre *sobre* la bóveda de ellos.
- No es una nube ni un servicio. Todo local, todo tuyo.
- No es un IDE. No compite con VS Code por ser el lugar donde vivís.

## El criterio que zanja las discusiones

Cuando una función quiera entrar y no esté claro si vale la pena, se la mide
contra estas dos preguntas: **¿cabe en el presupuesto de 7 MB?** y **¿puede
un documento hostil abusar de ella?** Si engorda el binario o abre una
superficie de ataque, la respuesta por defecto es no.
