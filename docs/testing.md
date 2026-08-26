# Estrategia de QA

QA significa aseguramiento de calidad. No consiste solamente en buscar errores al
final. En Visor MD implica definir qué debe ser cierto, construir evidencia y
detener una entrega cuando esa evidencia no alcanza.

## Objetivos

La estrategia debe demostrar:

- que Markdown válido se interpreta correctamente;
- que entrada hostil no ejecuta código ni agota recursos sin límite;
- que abrir y guardar no destruye contenido;
- que la UI sigue siendo rápida y utilizable;
- que Windows y Linux se comportan de forma compatible;
- que las dependencias y licencias son conocidas;
- que una función visible también funciona con teclado, DPI y errores reales.

## Pirámide de pruebas

### Unitarias

Prueban funciones pequeñas y rápidas: políticas de rutas, límites, conversión del
AST, marcadores, rangos y operaciones de edición.

Son útiles para localizar un fallo, pero no demuestran por sí solas que la
aplicación completa funciona.

### Integración

Prueban varias capas juntas: abrir, parsear, construir el modelo, maquetar y
producir comandos de dibujo. También cubren VFS, guardado y exportadores.

### Corpus

Un corpus es una colección versionada de entradas y resultados esperados.

Se usarán:

- ejemplos oficiales CommonMark aplicables;
- extensiones GFM y Obsidian elegidas;
- documentos reales anonimizados;
- casos históricos de v1;
- entradas patológicas y adversariales.

### Property testing

En lugar de comprobar solo ejemplos concretos, genera muchas variantes y verifica
propiedades. Ejemplo: editar un rango no debe modificar bytes fuera de ese rango.

### Fuzzing

Un fuzzer genera entradas inesperadas continuamente. Busca panic, bloqueos,
consumo excesivo y estados imposibles. No reemplaza casos diseñados a mano.

Campañas prioritarias:

- parser y conversión de AST;
- límites de profundidad;
- rangos y edición;
- rutas, wikilinks y VFS;
- decodificación y dimensiones de imágenes;
- importación y exportación.

### End to end

Prueban recorridos visibles completos, como abrir, cambiar a edición, modificar,
guardar, cerrar y volver a abrir. Deben mantenerse pocas y centradas en flujos
críticos para evitar una suite lenta y frágil.

### QA manual

Se reserva para percepción y entornos difíciles de automatizar:

- calidad tipográfica;
- animaciones;
- selección y menú contextual;
- lector de pantalla;
- IME;
- alto contraste;
- varios DPI y monitores;
- sensación de arranque y scroll.

Cada sesión manual usa una lista corta, registra plataforma y deja resultado. No
se sustituye una prueba automatizable por memoria humana.

## Seguridad dentro de QA

Una prueba de seguridad debe verificar la propiedad relevante.

Ejemplos:

- No basta con que una URL no se vea. Se monitorean sockets para demostrar que no
  hubo conexión.
- No basta con que `..` sea rechazado como texto. Se prueban symlinks, junctions,
  cambios de archivo y rutas UNC.
- No basta con que 5.000 citas no produzcan panic. Se mide cancelación, tiempo,
  memoria y entrada al modo seguro.
- No basta con escapar `<script>`. Se comprueba que ningún nodo HTML no permitido
  llegue al renderer como comportamiento activo.

Esta diferencia es importante en ciberseguridad: probar apariencia comprueba lo
que se observa; probar una propiedad comprueba lo que el sistema puede hacer.

## Gates por cambio

Todo cambio usa las gates proporcionales a su riesgo.

Base mínima:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
```

Además:

- cambios Markdown: corpus y patologías;
- filesystem: matriz de rutas y guardado;
- rendering: regresión visual, DPI y selección;
- dependencias: audit, deny, licencias, SBOM y tamaño;
- rendimiento: benchmark antes y después;
- seguridad: caso positivo, negativo y forma de evasión.

## Evidencia de release

Una release candidata necesita:

- commit y toolchain identificados;
- CI verde en Windows y Linux;
- tests unitarios, integración y corpus;
- campaña de fuzzing registrada;
- matriz manual completada;
- benchmark reproducible;
- SBOM y notices;
- advisories resueltos o aceptados;
- threat model y documentación sincronizados;
- lista explícita de riesgos residuales.

## Tratamiento de fallos

Un test que descubre un defecto real no se elimina ni se debilita para recuperar
el color verde. Primero se determina si la especificación, el test o el código es
incorrecto. Toda regresión importante debe dejar una prueba que falle antes del
arreglo y pase después.

Los tests antiguos se recompilan. Un ejecutable anterior no demuestra el estado
del working tree actual.
