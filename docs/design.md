# Diseño visual

Sistema visual aprobado para construir y evaluar. Las decisiones de identidad son
estables; medidas, contrastes e interacción pueden ajustarse cuando QA,
accesibilidad o pruebas de uso aporten evidencia mejor.

## Identidad: Papel + Tinta

Casi monocromo. El verde aparece **solo como hilo**: pestaña activa, enlace,
atajos, filete de una alerta, estado activo de un control. Todo lo demás es
tinta sobre papel. El documento es siempre lo más brillante de la pantalla; la
interfaz se calla.

### Paleta

| Rol | Noche | Día |
| --- | --- | --- |
| Fondo | `#0C0F0D` | `#EBFADC` |
| Superficie (código) | `#121513` | `#F7FDEF` |
| Borde | `#1D2320` | `#D6E5C6` |
| Texto | `#E9E9E4` | `#132A0A` |
| Tenue | `#8B918C` | `#5A6B4F` |
| Acento | `#5FD08A` | `#2E9E5B` |

El acento cambia de tono entre temas para mantener el contraste: el verde claro
que funciona sobre negro se lava sobre papel.

## Jerarquía de superficies: barra, menús y configuración

Regla explícita para decidir dónde va cada función nueva, nacida de comparar
los dos extremos de la investigación: ThisIs-Developer pone todo a la vista
siempre y se ve sobrecargado; Tinta, en el otro extremo, deja tan poco visible
que a Angel le pareció "una terminal", sin calidez de interfaz. Visor MD v2 no
quiere ninguno de los dos.

Cuatro niveles, del más visible al más escondido:

1. **Barra de formato.** Solo lo que se usa en casi todo documento: negrita,
   listas, encabezados, enlace, imagen. Si una función no pasa ese filtro, no
   entra acá aunque sea barata.
2. **Menús desplegables de la propia barra.** Variantes de una acción ya
   presente se agregan como segunda opción de su desplegable, no como botón
   nuevo. Ejemplos ya decididos: insertar enlace de referencia (junto al
   wikilink), insertar bloque de terminal (junto al bloque de código),
   insertar fecha y hora (en el menú de símbolos y entidades).
3. **Menú contextual (click derecho).** Todo lo que actúa sobre una selección
   o un elemento puntual y no necesita descubrirse por curioseo: cambiar
   mayúsculas y minúsculas de la selección, fijar una pestaña, las acciones
   de la papelera.
4. **Paleta de comandos y paneles contextuales.** La paleta permite encontrar
   acciones sin memorizar su ubicación. Los paneles aparecen para índice,
   búsqueda, workspace, detalles o comparación y se cierran sin ocupar espacio
   permanente.
5. **Configuración avanzada.** Funciones que cambian el comportamiento por
   defecto del documento y que, si estuvieran activas para todos, generarían
   falsos positivos o sorpresas. Ejemplo decidido: las referencias de archivo
   en texto plano quedan apagadas por defecto y se activan acá, porque
   convertir cualquier ruta mencionada al pasar en un enlace no pedido rompe
   la lectura de quien no lo esperaba. Cuando se activan, se pintan del color
   de hipervínculo para que quede claro que son un enlace.

**El criterio para dudar:** si hay que preguntarse "¿esto va en la barra?", ya
la respuesta es no. La barra es para lo que casi todo el mundo usa en cada
documento; todo lo demás vive un nivel más adentro, disponible pero sin gritar.

## Ventana: sin borde

Sin líneas de contorno. La jerarquía sale del salto de color entre superficies.
Radio de 11 px, sombra difusa para despegar del escritorio.

**La condición que esto impone**, y por eso importa: el salto de fondo a
superficie tiene que ser de al menos 6 % de luminosidad, o todo se aplana. Es
justo lo que le faltaba al modo noche de la v1. La paleta de arriba ya cumple.

## Iconos: A · Suave

Trazo 1.5, terminaciones y uniones redondeadas, rejilla de 24.

**Corrección respecto de la v1:** los botones de barra **no llevan fondo verde
en reposo**. En la v1 todos lo llevan y compiten con el documento. Estados:

| Estado | Tratamiento |
| --- | --- |
| Reposo | Sin fondo, icono en tono tenue |
| Hover | Fondo blanco al 5 %, icono en tono texto |
| Activo | Fondo del acento al 14 %, borde del acento al 35 %, icono en acento |
| Inactivo | Icono al 25 % de opacidad, sin fondo |

La barra superior provisional de acciones de archivo aplica la misma gramática:
no dibuja una hilera de botones sólidos en reposo, usa superficie solo al pasar
el mouse y reserva el acento para la acción de modo activa. Sus rótulos se
reemplazarán por iconos accesibles cuando el sistema definitivo de iconos esté
integrado.

`F6` lleva el foco visible a esa barra; flechas o Tab lo recorren, Enter o
Espacio ejecutan y Escape vuelve al documento. Este foco no se presenta como
prueba de lector de pantalla: la semántica accesible del canvas sigue siendo un
trabajo separado.

## Tipografía: Contraste editorial

- **Interfaz:** Sora
- **Documento:** Newsreader
- **Código:** JetBrains Mono

Preguntaste si tres familias pesan. **Sí, pero entra**: en una app nativa no hay
CDN (no habría red) así que las fuentes van embebidas. Subconjunto latino de una
variable por familia. El subset inicial medido fue de unos 410 KB. El working
tree posterior, con Newsreader Italic y metadata corregida, ronda 694 KB. Sigue
dentro del presupuesto menor de 8 MB, pero el proceso debe quedar reproducible.

Vale la pena porque la interfaz geométrica y seca contra el documento cálido y
literario es lo que señala qué es aplicación y qué es contenido. Si el
presupuesto se ajusta en una medición futura, el plan de repliegue es en dos pasos:
primero bajar a dos familias (Newsreader + la sans del sistema), y solo después
pasar a Neutro suizo (Archivo + Literata), tu opción B.

### Escala del documento

| Rol | Tamaño / interlínea |
| --- | --- |
| H1 | 31 / 1.2 |
| H2 | 25 / 1.3 |
| H3 | 20 / 1.35 |
| Cuerpo | 16 / 1.65 |
| Código | 13.5 / 1.7 |

Medida de 62 a 72 caracteres por línea.

## Profundidad: plano, con elevación solo para lo que flota

**Dentro del documento: plano.** Sin sombras. Los bloques de código y las
alertas se separan por color de superficie y, en el caso de las alertas, por su
filete de acento a la izquierda.

**Lo que está literalmente por encima** (menús, diálogos, avisos) lleva
elevación, con la sombra suavizada respecto de la propuesta original:

| Capa | Sombra |
| --- | --- |
| Menú desplegable | `0 10px 24px -12px rgba(0,0,0,0.55)` |
| Diálogo modal | `0 20px 44px -18px rgba(0,0,0,0.6)` |
| Ventana | `0 18px 40px -14px rgba(0,0,0,0.75)` |

### Escala de superficies (noche)

| Nivel | Color | Uso |
| --- | --- | --- |
| 0 · fondo | `#0C0F0D` | Lienzo de la ventana |
| 1 · base | `#121513` | Área del documento |
| 2 · elevado | `#1A1F1C` | Bloques, barra |
| 3 · flotante | `#232A26` | Menús, diálogos |

## Movimiento

Como se propuso, sin cambios.

**Duraciones:** 120 ms color y hover · 180 ms pestaña y conmutador · 200 ms
cambio de tema · 240 ms panel y división.

**Curvas:** estándar `cubic-bezier(0.32, 0.72, 0, 1)` · salida
`cubic-bezier(0.16, 1, 0.3, 1)` · rebote `cubic-bezier(0.34, 1.56, 0.64, 1)`,
solo en confirmaciones.

**Reglas:** el texto nunca se mueve · solo `transform` y `opacity` · lo que sale
sale más rápido que lo que entra · se respeta `prefers-reduced-motion` · nada en
bucle salvo progreso real.

## Enlaces y estados de seguridad

- Enlace web externo o correo: azul convencional, subrayado y destino real
  visible antes de abrir.
- Enlace interno o wikilink: verde de acento, diferenciado del externo.
- Recurso bloqueado: placeholder discreto y acceso a detalles.
- Archivo o destino inexistente: tono tenue, sin fingir que la acción funcionará.
- Foco de teclado: visible incluso cuando no hay hover.

El color nunca es la única señal. Esto mejora accesibilidad y reduce phishing.

## Accesibilidad

- contraste comprobado, no solo elegido a ojo;
- zoom sin recortar controles;
- navegación completa por teclado;
- foco visible;
- targets de mouse suficientemente grandes;
- no depender solo de color;
- reducir movimiento;
- lectores de pantalla e IME considerados antes del editor;
- alto contraste del sistema probado.

La ventana sin borde no puede eliminar affordances necesarias para mover,
redimensionar, cerrar o comprender el foco.

## El lienzo

Los PNG, ZIP y MHTML recuperados en `Artifac opciones de diseño` son referencias
locales. No son runtime ni código de producción. Las decisiones vigentes se
mantienen en este documento para que el proyecto no dependa del artifact.
