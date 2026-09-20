# Implementación del workspace Papel + Tinta

Fecha: 20 de septiembre de 2026.

Este documento convierte el concepto C de
[`design-proposals`](design-proposals/README.md) en un contrato ejecutable. No
reemplaza [`design.md`](design.md): organiza la implementación, las pruebas y la
evidencia necesarias para alcanzarlo sin improvisar durante cada cambio.

## Resultado acordado

Visor MD usa el workspace refinado del concepto C. El estado normal sigue siendo
un documento grande. Cuando la persona abre navegación o una bóveda aparece una
columna completa a la izquierda; cuando divide, cada hoja conserva documento,
modo y foco propios.

Cada tema muestra la misma arquitectura y los mismos estados:

- **Día:** papel marfil con matiz sepia perceptible, tinta marrón casi negra y
  acento verde de hoja impresa. No usa blanco frío ni verde azulado.
- **Noche:** bosque profundo, tinta marfil y verde menta contenido. No usa negro
  puro ni acentos de neón.

El mockup es referencia de jerarquía, no una orden de copiar texto defectuoso ni
funciones que Visor MD no tenga.

## Requisitos trazables

| ID | Requisito | Evidencia de cierre |
| --- | --- | --- |
| UI-01 | Tema día sepia y tema noche bosque comparten roles y contraste suficiente. | Pruebas de tokens, captura día/noche y contraste de texto. |
| UI-02 | La primera franja contiene identidad, archivo, búsqueda, workspace y ventana. | Geometría y hit testing usan la misma descripción. |
| UI-03 | Pestañas de documentos viven en una franja propia y muestran foco, suciedad y cierre. | Pruebas de selección, orden y cierre; QA en ventana estrecha. |
| UI-04 | Leer, Editar y Comparar son locales a cada panel, nunca estado global visual. | Dos documentos pueden mostrar modos distintos simultáneamente. |
| UI-05 | La segunda franja cambia con el panel activo y ofrece formato o navegación sin duplicar acciones globales. | Catálogo de acciones y disponibilidad probado. |
| UI-06 | Archivos, Índice, Buscar y Backlinks comparten una columna izquierda de altura completa, abierta solo por acción explícita. | Abrir y cerrar cambia el viewport; clics nunca atraviesan la columna. |
| UI-07 | Menús, diálogos y avisos comparten radio, borde, elevación y cierre por Escape o clic exterior. | Pruebas de lifecycle y QA visual. |
| UI-08 | Un documento domina la pantalla; dividir es explícito, admite hasta cuatro archivos y conserva foco y scroll aislados. | Regresiones existentes más QA de divisiones. |
| UI-09 | Movimiento afecta indicadores y superficies, nunca texto ni geometría de lectura; reducir movimiento lo vuelve instantáneo. | Pruebas de duración y QA perceptivo. |
| UI-10 | La ventana mínima degrada acciones a menús sin crear controles invisibles o clicables. | Pruebas a 640 × 480 y QA de DPI. |
| UI-11 | Edición ofrece un kit Markdown amplio y organizado, no una hilera mínima de botones ni una paleta plana. | Acciones directas, menús por categoría, teclado y contexto invocan el mismo catálogo probado. |

## Mapa de componentes

La composición se resuelve siempre en este orden:

1. chrome de ventana y acciones globales;
2. pestañas y destino de apertura;
3. columna de workspace opcional;
4. árbol de paneles documentales;
5. cabecera y modo local de cada panel;
6. barra contextual del panel enfocado;
7. documento o editor;
8. estado inferior;
9. capas flotantes.

Cada capa obtiene un rectángulo de una única geometría de ventana. Dibujo, foco,
hit testing, scroll y accesibilidad no pueden mantener copias divergentes.

## Secuencia lineal

### Fase 1. Contrato y tokens

- fijar los colores corregidos y sus roles;
- comprobar contraste de texto, texto tenue y acento;
- eliminar descripciones históricas incompatibles con el concepto elegido;
- capturar una línea base antes de cambios estructurales.

**Salida:** paleta y jerarquía sin ambigüedad; ninguna función cambia todavía.

### Fase 2. Geometría compartida

- modelar barras, pestañas, columna, viewport, paneles y estado;
- migrar dibujo e interacción a esa geometría por partes;
- reservar espacio real para la columna y las cabeceras de panel;
- conservar el tamaño mínimo y el chrome accesible de plataforma.

**Salida:** ningún elemento se superpone ni recibe clic fuera de lo dibujado.

### Fase 3. Workspace y navegación

- convertir los paneles cortos de Archivos, Índice, Buscar y Backlinks en una
  única columna completa;
- conservar consulta, selección, scroll, estados vacío/límite y navegación VFS;
- hacer visible el destino activo y permitir cerrar la columna sin perder el
  documento.

**Salida:** el workspace elegido se reconoce en lectura, edición y divisiones.

### Fase 4. Panel documental

- mover título, modo local y acciones de división a la cabecera de cada hoja;
- asegurar modos distintos entre hojas;
- separar barra contextual de acciones globales;
- construir el kit de escritura en grupos claros: texto, estructura, listas,
  inserción, estudio y segundo cerebro;
- mantener visibles negrita, cursiva, resaltado, listas, tarea, enlace, cita,
  código y tabla; usar menús con nombre para variantes como niveles de título,
  tipos de callout, enlaces de bóveda, bloques de estudio y formatos de copia;
- adaptar el menú contextual a selección, cursor, enlace, tarea o bloque, sin
  duplicar una lista fija de acciones irrelevantes;
- completar degradación de controles por ancho.

**Salida:** pestaña, hoja activa y modo actual son inequívocos.

### Fase 5. Componentes y estados

- unificar radios, bordes, sombras, hover, foco, activo, deshabilitado y error;
- pulir editor, tablas, código, callouts, tareas y placeholders seguros;
- añadir transiciones breves con una ruta común de movimiento reducido.

**Salida:** chrome y documento parecen un solo producto, no subsistemas unidos.

### Fase 6. Cierre visual y funcional

- probar día/noche, lectura, edición, comparación, columna, dos y cuatro hojas;
- verificar ventana inicial, mínima, amplia y DPI alto;
- revisar teclado, IME, foco, contraste, menús y cierre protegido;
- actualizar manual QA, dashboard, roadmap y capturas de referencia.

**Salida:** QA humano decide matices; las propiedades estructurales ya están
automatizadas y el release mantiene el presupuesto.

## Ciclo de trabajo eficiente

Cada fase usa el mismo bucle:

1. elegir el siguiente requisito no satisfecho;
2. inspeccionar solo sus funciones, pruebas y documentación normativa;
3. hacer una edición localizada y una regresión pequeña;
4. ejecutar `cargo check` y las pruebas relacionadas;
5. encadenar el siguiente requisito si la base sigue sana;
6. ejecutar suite, Clippy, release y captura una sola vez al cerrar la fase;
7. versionar y publicar el bloque coherente.

No se repite un build release si solo cambió documentación o si ningún cambio
posterior pudo invalidarlo. El QA humano pendiente se registra y no bloquea las
fases independientes.

## Fuera de este rediseño

- IA embebida, sincronización, grafos y seguridad decorativa;
- cambiar el modelo de archivos, VFS o guardado por razones visuales;
- copiar funciones ficticias de Stitch;
- reescribir el renderer o agregar una dependencia de UI sin evidencia de que
  la arquitectura nativa actual no pueda cumplir el contrato.
