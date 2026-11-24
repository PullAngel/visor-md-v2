# Barra de estado y escritura rápida · 24 de noviembre de 2026

Medición de la tarea 17 de [`plan-revision-2026-11-23.md`](../plan-revision-2026-11-23.md).
Se repite con:

```bash
cargo test --release medicion_de_la_barra -- --ignored --nocapture
```

Máquina de desarrollo (Windows 10, perfil `release`). Los tiempos son de una
sola corrida: sirven para comparar antes y después, no como cifras absolutas.

## Qué se encontró

El contador de palabras de la barra costaba poco (un recuento cada tecla) pero
**no era lo que frenaba la escritura**. Lo que crecía con el tamaño del
documento era la actualización de los bloques de la fuente en cada tecla:
copiaba y revalidaba todos los bloques que quedaban después del cursor.

## Resultados

| Medida | Antes | Después |
| --- | --- | --- |
| Actualizar los bloques tras una tecla, documento de 100 KB | 7,0 ms | 0,016 ms |
| Lo mismo con 1 MB | 74,7 ms | 0,22 ms |
| Lo mismo con 4 MB | 387 ms | no medido otra vez (crece como el de 1 MB) |
| 1000 teclas seguidas (edición y barra), 100 KB | 8,3 ms por tecla | 1,1 ms por tecla |
| 1000 teclas seguidas (edición y barra), 1 MB | 75,9 ms por tecla | 0,35 ms por tecla |
| Recontar palabras de 1 MB | 14 ms (copiando a `String`) | 9 ms (recorriendo el buffer) |
| Columna del cursor en una línea de 5 MB | recorría la línea entera cada cuadro | menos de 0,001 ms |

El dibujo de un cuadro completo (80 ms en esta máquina, a 1200×800) no depende
del tamaño del documento, y la medición de arriba no lo incluye.

## Cambios

- **Bloques en sitio.** `refresh_safe_buffer_blocks_in_place` reemplaza solo
  los bloques de las líneas tocadas y corre las posiciones de los siguientes,
  sin clonarlos ni revalidarlos uno por uno. Una prueba compara el resultado
  con reconstruir todo, tras 160 ediciones al azar.
- **Contador sin copias.** `DocumentMetrics::from_buffer` recorre los tramos del
  buffer, sin pasarlo a un `String`, con un camino rápido para ASCII. Da lo
  mismo que contar la fuente entera (prueba con texto que cruza varios tramos,
  caracteres de control y espacios Unicode).
- **Espera en documentos grandes.** Hasta 256 KB se cuenta en cada cambio. Por
  encima, a lo sumo cada 250 ms, y la espera crece con el tamaño (5 MB: cada
  1,25 s), de modo que el recuento ocupa menos de un 4 % del tiempo. Se muestra
  el último valor y se agenda un redibujo para ponerlo al día.
- **Columna del cursor.** Se calcula con la posición del carácter en el buffer
  (`chars_between`), no copiando el texto de la línea. Pegar 5 MB en una línea
  ya no hace más lenta la barra.
- **Barra en dos partes.** Los números (palabras, caracteres, línea y columna)
  van a la izquierda de una parte fija (modo y estado) anclada al borde. Al
  crecer un número, solo se mueve el texto de los números, nunca el resto.

## Pegar un bloque grande

Pegar 5 MB en una sola línea: 25 a 37 ms el pegado, y 50 ms el recuento de la
barra, una vez. No queda ninguna operación que dependa del largo de la línea.
