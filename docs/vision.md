# Visión

## La idea

El Markdown vive en una posición incómoda. Como fuente es fácil de transportar,
pero no siempre agradable de leer. Renderizado es cómodo, pero muchas
herramientas vuelven difícil corregirlo, pesan demasiado o introducen un motor
web con una superficie de ataque innecesaria.

Visor MD apuesta por una tercera experiencia: la inmediatez de abrir texto, la
calidad de leer un documento terminado y la confianza de editar sin perder
compatibilidad.

## La escena cotidiana

Una persona pide a una IA que cruce varios PDF y entregue resúmenes conectados en
Markdown. Hace doble clic en el resultado, lo lee de inmediato, copia una sección
para un compañero y lo guarda en su bóveda.

Otro día abre el mismo documento durante una clase. Resalta ideas, agrega
aclaraciones y marca preguntas. Al guardarlo, Obsidian y otras herramientas
siguen entendiendo el archivo.

Ese recorrido es más importante que una lista extensa de funciones.

## Personalidad

Visor MD debe sentirse:

- elegante;
- técnico;
- poderoso;
- confiable;
- sereno aun cuando ofrece profundidad.

La ventana, la tipografía, los verdes y el movimiento forman parte de la
identidad. Las herramientas visibles son las necesarias. La profundidad aparece
por contexto, comandos y paneles, no mediante una pared de controles.

## Principios

### Seguro por construcción

El documento nunca se considera confiable solo porque parece texto. El parser,
las rutas, las imágenes, las extensiones y el guardado tienen límites y pruebas.
La aplicación no ejecuta contenido.

### Instantáneo por disciplina

La rapidez no se consigue escondiendo trabajo, sino evitando dependencias y
operaciones innecesarias, virtualizando correctamente y midiendo cada regresión.

### Portable antes que propietario

Las anotaciones y ayudas producen Markdown que otras herramientas puedan leer.
Una función exclusiva solo se acepta cuando no existe una representación portable
y su valor justifica el coste.

### Profundo sin sobrecarga

Visor MD puede ser poderoso sin mostrar todo al mismo tiempo. Menú contextual,
paleta de comandos y paneles progresivos permiten mantener el documento como
protagonista.

### Ingeniería demostrable

Seguridad, calidad y rendimiento deben tener evidencia: tests, threat model,
benchmarks, ADR, fuzzing, SBOM y resultados reproducibles.

## Tamaño

Menos de 6 MB sería excepcional, alrededor de 7 MB es el objetivo y menos de 8
MB es el límite deseado. El tamaño protege la disciplina del proyecto. No está
por encima de seguridad, estabilidad, accesibilidad o funciones esenciales.

## Portabilidad

Windows y Linux forman parte de v2.0. La arquitectura se mantiene portable desde
el inicio y las gates de CI deben detectar divergencias temprano. macOS puede
evaluarse después sin contaminar el diseño actual.

## Qué no quiere ser

- un navegador de propósito general;
- un editor sobrecargado;
- un clon de Obsidian;
- un IDE;
- una aplicación con IA integrada por moda;
- un escaparate técnico que descuida al usuario;
- un producto liviano solo porque carece de funciones esenciales.

## Criterio para resolver discusiones

Entre dos alternativas válidas, elegir la que preserve mejor el conjunto:

1. seguridad y datos;
2. corrección;
3. velocidad y recursos;
4. lectura, edición y accesibilidad;
5. compatibilidad;
6. tamaño;
7. mantenibilidad;
8. identidad visual.

Si una mejora beneficia una dimensión y perjudica otra, documentar el intercambio
en lugar de presentarla como ganancia gratuita.
