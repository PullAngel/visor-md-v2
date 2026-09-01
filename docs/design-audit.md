# Auditoría visual: retorno a Papel + Tinta

Fecha: 1 de septiembre de 2026. Este documento convierte el feedback visual en
criterios de trabajo. No sustituye [`design.md`](design.md), que conserva la
autoridad sobre la identidad del producto.

## Evidencia

La captura reproducible de v2 con `tests/fixtures/sprint1-visual.md` mostró que
la tipografía editorial y la sintaxis ya tienen una base útil, pero el chrome se
percibe como texto técnico sobre un lienzo oscuro. El bloque de código repetía
el botón Copiar por línea y el contraste entre documento, barra y estado era
insuficiente. La referencia de v1 aporta una composición coherente, aunque no
su verde saturado ni su densidad de controles.

## Desviaciones confirmadas

1. La barra usa rótulos provisionales donde el sistema aprobado pide iconos
   suaves y estados sobrios.
2. Las superficies nocturnas no comunican con claridad fondo, base, elevado y
   flotante.
3. Pestañas y estado pueden perder presencia visual al competir con el viewport.
4. Código, tablas, citas y paneles necesitan reglas compartidas de espaciado y
   borde para verse como parte de un mismo producto.
5. La composición aún no ha pasado QA visual sistemático en día, noche, edición,
   dividida, ventanas estrechas y DPI alto.

## Plan de corrección

### A. Fundaciones y defectos visibles

- corregir controles repetidos y clipping;
- reservar chrome y viewport con una única geometría;
- establecer cuatro superficies y separadores consistentes;
- añadir pruebas de hit testing y geometría a cada defecto real.

### B. Chrome editorial

- reemplazar rótulos de acciones primarias por iconos suaves dibujados de forma
  nativa, manteniendo texto accesible en foco, menú y paleta;
- equilibrar jerarquía de título, acciones, pestañas y estado;
- aplicar hover, foco y activo sin convertir los botones en bloques verdes.

### C. Documento y edición

- recalibrar márgenes, ancho de lectura, ritmo vertical y bloques enriquecidos;
- unificar código, tablas, citas, callouts y paneles bajo la escala de
  superficies;
- hacer que fuente y vista dividida conserven identidad editorial y separación
  clara sin competir con el texto.

### D. Validación visual

- producir capturas reproducibles para lectura día/noche, edición, dividida,
  tablas, código, panel y ventana mínima;
- revisar contraste, foco, clipping, tamaño de objetivos y continuidad con
  `design.md`;
- registrar QA humano pendiente antes de declarar cerrado el sprint visual.

## Criterio de salida

Una persona debe reconocer una aplicación editorial antes de leer sus botones:
el documento domina, el chrome acompaña, el verde guía y ninguna capa se
confunde con otra. Las capturas y QA deben demostrarlo, no solo afirmarlo.
