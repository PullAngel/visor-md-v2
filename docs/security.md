# Seguridad

Documento maestro de seguridad de Visor MD v2. `threat-model.md` describe el
modelo de amenaza; este documento es el trabajo de blindaje: qué se investigó,
qué se decidió, y qué se sacrifica.

Está escrito para poder hablar con criterio del tema sin volver a razonarlo:
cada decisión trae el ataque concreto que la motiva.

---

## 1 · El supuesto

Todo `.md` es contenido ajeno y potencialmente hostil hasta demostrar lo
contrario. Llega de un repositorio, de un conversor de PDF, de una IA, de un
adjunto. Puede estar diseñado para atacar, o simplemente roto por un conversor
defectuoso. Los dos casos importan: **un parser que se cae con basura es tan
inaceptable como uno que ejecuta código**.

## 2 · Lo que ganamos al dejar el motor web

La v1 dedicaba la mitad de su trabajo de seguridad a contener un motor de
scripts: DOMPurify, CSP, allowlist de protocolos, aislar Mermaid. Sin ese
motor, esa mitad desaparece: no se contiene lo que no está.

| Clase de ataque | En la v1 | En la v2 nativa |
| --- | --- | --- |
| XSS / ejecución de script | Contenida por sanitización | **No existe**: no hay intérprete |
| Fuga de red por CSS, `srcset`, SVG | Contenida, y costó tres hallazgos encontrarla | **No existe**: no hay CSS ni HTML activos |
| DOM clobbering | Contenida congelando referencias | **No existe**: no hay DOM |
| Robo de credenciales por ruta UNC | Corregida en v1.1.0 | Se hereda la corrección, reforzada |

## 3 · Lo que ganamos al elegir Rust

Esta es la parte que un enfoque nativo ingenuo pasa por alto. Cambiar de un
motor web a código nativo **cambia la clase de vulnerabilidad, no la elimina**:
se pasa de XSS a corrupción de memoria. Un lector nativo en C o C++ que parsea
entrada no confiable es terreno clásico de desbordamientos: el CVE-2026-5525 de
Notepad++, un desbordamiento en el manejo de rutas por arrastre, es exactamente
eso.

Rust elimina esa clase entera en el código seguro. **Pero no es magia y hay que
decirlo con precisión:**

- La garantía vale para el código `safe`. Un crate con bloques `unsafe` puede
  reintroducir corrupción de memoria, y la base de datos RUSTSEC registra
  varios casos reales: uso después de liberar, exposición de memoria sin
  inicializar, desbordamientos de montículo en APIs que se presentan como
  seguras.
- Las dependencias con más `unsafe` en esta pila son justo las de bajo nivel:
  rasterización de glifos, decodificación de imágenes, dibujo 2D.

### Lo que encontró la primera auditoría real (Sprint 0)

No es teoría: al compilar el prototipo y mirar el árbol de dependencias
aparecieron dos cosas que valen como advertencia permanente.

**1. Habíamos enlazado C sin querer.** Las funciones por defecto de `comrak`
traen `syntect-onig`, que usa **Oniguruma**, una librería de expresiones
regulares escrita en C, vía `onig_sys`. Nadie la pidió. Entró porque nadie
miró qué arrastraba una dependencia por defecto. Un motor de expresiones
regulares en C procesando entrada no confiable es exactamente la clase de
superficie que el ADR-2 existe para evitar, y estaba adentro del binario que
se presentaba como "sin C".

Se sacó con `default-features = false`. El árbol pasó de 144 a **96 crates**,
**ninguno en C**, y el binario bajó 536 KB. Ver ADR-14.

**La lección, que vale más que el arreglo:** la tesis de seguridad no se
sostiene sola por elegir Rust. Se sostiene revisando qué se enlaza. Toda
dependencia nueva entra con `default-features = false` y se audita con
`cargo tree` antes de agregarla.

**2. Un crate sin mantenimiento.** `cargo audit` da cero vulnerabilidades, pero
avisa que `ttf-parser` 0.25.1 está **sin mantenimiento**
(RUSTSEC-2026-0192). Entra de forma transitiva por la pila de fuentes. No es
un fallo hoy, pero es riesgo acumulado: si mañana aparece un problema en un
crate que nadie mantiene, no va a haber parche y hay que reemplazarlo con
apuro. Se revisa en el Sprint 1, al tocar la capa de fuentes.

**Mitigaciones concretas:**

- `cargo audit` y `cargo deny` en cada compilación, con la construcción fallando
  ante un advisory abierto. No es opcional ni "cuando nos acordemos".
- **Ninguna dependencia en C.** Verificado en el Sprint 0 y a verificar en cada
  dependencia nueva.
- **Presupuesto de `unsafe` en código propio: cero.** `#![forbid(unsafe_code)]`
  en todos los módulos salvo la capa de integración con el sistema operativo,
  donde llamar a la API de Windows lo exige. Ese módulo se revisa aparte y a
  mano.
- Árbol de dependencias mínimo. Cada crate es código de terceros en el binario;
  además ayuda al presupuesto de tamaño. Los dos objetivos empujan igual.
- Auditoría explícita de qué crates traen `unsafe` y cuánto, con `cargo geiger`,
  antes de fijar la pila en la Fase 0.

## 4 · Superficies de ataque, una por una

### 4.1 · El parser de Markdown

**Ataque:** documento con anidamiento patológico, tabla de un millón de celdas,
enlace de referencia recursivo, o entrada que dispara un caso cuadrático.

**Defensas:**
- Topes duros: profundidad de anidamiento, cantidad de nodos, largo de línea.
  Se calibran contra el corpus real, no a ojo.
- Parseo en hilo aparte con cancelación: si un documento excede su presupuesto
  de tiempo, se corta y se muestra lo que se alcanzó a parsear, con aviso.
- **Fuzzing continuo** con `cargo-fuzz` sobre el parser. Barato en Rust y
  encuentra lo que nadie imagina. La v1 no podía hacerlo con la misma facilidad
  sobre su pipeline de JavaScript.

### 4.2 · Rutas de archivo

**Ataque:** el documento pide un recurso por una ruta que se sale de donde
corresponde. Es el vector que en la v1 produjo la fuga de credenciales.

**Defensas** (heredadas y reforzadas):
- Canonizar **antes** de validar. Validar la cadena original es inútil frente a
  un junction o un enlace simbólico.
- Rechazo incondicional de: rutas UNC (`\\servidor\recurso`), rutas de
  dispositivo (`\\?\`, `\\.\`), flujos alternativos de NTFS (`archivo.png:oculto`),
  y unidades de red mapeadas.
- Contención al árbol de la carpeta del documento, con las carpetas de
  confianza como única puerta y con registro auditable.
- **Nuevo en la v2:** verificación de que el archivo abierto es el mismo que se
  validó, comprobando el identificador de archivo del sistema. Cierra la ventana
  TOCTOU entre validar y abrir, que en la v1 quedaba como riesgo residual
  aceptado.

### 4.3 · Wikilinks e índice del workspace

Superficie nueva de la v2, y me pediste cuidado explícito acá.

**Ataques:**
- Un wikilink que resuelve fuera de la bóveda: `[[../../../.ssh/id_rsa]]`.
- Una bomba de índice: una bóveda con cientos de miles de archivos, o nombres
  de archivo diseñados para colisionar y hacer cuadrática la resolución.
- Ciclos de embeds: `A` embebe `B` que embebe `A`.
- Enlaces que apuntan a rutas absolutas o de red disfrazadas de nota.

**Defensas:**
- Un wikilink **nunca** es una ruta: es un nombre que se busca en el índice de
  la bóveda. Si no está en el índice, no existe: no se intenta abrir como
  ruta. Esto sola cierra el traversal por completo.
- El índice solo contiene archivos que ya pasaron la contención de rutas.
- Tope de profundidad de embeds y detección de ciclos por conjunto de visitados.
- Índice incremental con tope de archivos; pasado el tope, se avisa y se indexa
  lo que entra en vez de colgarse.

### 4.4 · Imágenes

**Ataque:** un `.png` malformado que explota el decodificador. Es la superficie
nativa más peligrosa que queda, porque los decodificadores de imagen son código
de bajo nivel con `unsafe`.

**Defensas:**
- Límite de dimensiones y de tamaño **antes** de decodificar, leyendo solo la
  cabecera. Una imagen de 50.000 × 50.000 se rechaza sin haberla decodificado.
- Solo formatos con decodificador en Rust puro donde exista. Si un formato
  exige un decodificador en C, se evalúa dejarlo fuera antes que aceptarlo.
- Las imágenes remotas siguen bloqueadas por defecto: la propiedad de red se
  mantiene intacta desde la v1.

### 4.5 · Anotaciones sidecar

Superficie nueva: un archivo que la app escribe y vuelve a leer.

**Ataque:** un sidecar manipulado con rutas, tamaños absurdos o referencias a
otros archivos.

**Defensas:** el sidecar se trata como entrada no confiable igual que el `.md`.
Formato simple y tipado, sin rutas dentro, sin ejecución posible. Vive junto a
la nota y no puede apuntar fuera de ella.

### 4.6 · Componentes opcionales (IA local, diagramas)

**Ataque:** el componente descargable es el eslabón débil: puede estar
adulterado, o tener su propia superficie.

**Defensas:**
- Corren en **su propio proceso**, sin acceso al sistema de archivos, hablando
  por un contrato de mensajes angosto y tipado.
- Se verifican por hash antes de cargarse.
- El núcleo funciona entero sin ellos. Si un componente falla, la app sigue.
- Para la IA local: el texto va al componente, nunca a la red. El componente no
  tiene cliente HTTP.

### 4.7 · Guardado de archivos

**Ataque:** perder trabajo del usuario, o corromper un archivo ajeno.

**Defensas:**
- Guardado atómico: archivo temporal en la misma carpeta y reemplazo. Un corte
  de luz no deja el archivo a medio escribir.
- Codificación, BOM y fin de línea originales preservados. Un archivo ajeno no
  cambia de formato por haberlo abierto.
- **Sin autoguardado por defecto** (decisión tuya): las modificaciones no tocan
  el original hasta que se guarda. La recuperación ante cierre inesperado usa un
  archivo temporal aparte, nunca el original.

## 5 · Lo que cuesta esta postura

Me pediste avisarte si algo termina en una decisión de diseño con sacrificio.
Estos son:

| Se pierde | Por qué | Alternativa que se ofrece |
| --- | --- | --- |
| **HTML arbitrario en el documento** | Sin motor de render HTML, solo se dibuja lo que la allowlist contempla | Se cubren los casos reales: `<details>`, `<kbd>`, `<mark>`, `<sub>`, `<sup>`, `<br>`. Lo demás se muestra como texto inerte, no se descarta en silencio |
| **CSS embebido en el documento** | Es un vector de fuga de red y de suplantación de la interfaz | Ninguna. Es un `no` definitivo |
| **Formatos de imagen exóticos** | Decodificadores en C con superficie de memoria | PNG, JPEG, GIF, WebP cubren el uso real |
| **Abrir directo desde una URL** | Introduce red y descarga automática de contenido no confiable | Clonar o descargar el archivo fuera y abrirlo |

**Lo que NO se sacrifica:** Mermaid. Dijiste que es de las pocas cosas por las
que subirías el límite de tamaño, y coincido: es lo que distingue un lector
técnico de un lector de texto. El plan está en `product.md`.

## 6 · Lo que NO es configurable, nunca

Se puede ampliar **a qué recursos accede** un documento. Nunca **qué puede
ejecutar**. En concreto, no hay ni habrá ajuste para:

- Desactivar el validador de nodos.
- Permitir rutas de red.
- Ejecutar HTML o CSS del documento.
- Cargar un componente opcional sin verificar su hash.

Un interruptor para cualquiera de esas sería el ajuste más atacado del
programa: bastaría con convencer al usuario de activarlo una vez.

## 7 · Cómo se demuestra

Igual que en la v1, la suite afirma **propiedades**, no ausencia de crash. El
corpus de ataque de la v1 se traslada entero y se amplía:

- [ ] Ningún `.md` del corpus produce una petición de red (se observa el
      socket, no se confía en la ausencia).
- [ ] Ninguna ruta del corpus de traversal se resuelve fuera de su carpeta.
- [ ] Ningún wikilink resuelve fuera de la bóveda.
- [ ] El corpus de conversión defectuosa renderiza entero sin panic.
- [ ] Un documento con anidamiento patológico se corta en el tope, no cuelga.
- [ ] El parser sobrevive a entrada aleatoria (fuzzing continuo).
- [ ] Una imagen con dimensiones absurdas se rechaza sin decodificarse.
- [ ] Un sidecar manipulado no produce lectura fuera de la nota.
- [ ] `cargo audit` limpio en cada compilación.

## 8 · Riesgo residual

Lo que queda, dicho sin maquillar:

- **Dependiente de crate externo.** Un fallo en `comrak`, `parley`, `tiny-skia`
  o el decodificador de imágenes. `cargo audit` lo detecta cuando se publica;
  entre la existencia del fallo y su publicación no hay defensa.
- **Dependiente del sistema operativo.** El dibujo, las fuentes y los diálogos
  nativos quedan en manos de Windows, Linux o macOS.
- **Agotar memoria** con un documento gigante sigue siendo posible pese a los
  topes. El usuario cierra la pestaña; no se pierde nada.
- **El usuario decidiendo mal.** Una carpeta de confianza agregada a la ligera
  amplía el acceso legítimamente. La bitácora auditable existe para que esa
  decisión sea visible y revocable, no para impedirla.
