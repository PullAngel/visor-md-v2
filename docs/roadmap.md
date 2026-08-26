# Roadmap

Última revisión: 25 de agosto de 2026.

El roadmap ordena dependencias y criterios de salida. No promete fechas. Una
etapa solo se cierra con evidencia y el producto debe quedar en un punto útil al
final de cada hito.

## Reglas

- `main` conserva estados estables.
- El trabajo ocurre en ramas separadas.
- No se inicia una etapa sobre una base rota.
- Seguridad, accesibilidad y documentación acompañan cada etapa.
- Una función reconocida por el parser no está terminada hasta llegar a UX y
  tests.
- Todo cambio de dependencia mide tamaño y superficie.
- El alcance de v2.0 puede entregarse mediante previews sin llamarlas estables.

## Estado

| Etapa | Estado | Resultado |
| --- | --- | --- |
| Sprint 0 | Cerrado con deuda documental | Prototipo nativo medido |
| Recuperación | Activa | Working tree heredado estable |
| Sprint 1 | Parcial | Lector mínimo profesional |
| Validación base | Pendiente | Gates y evidencia reproducible |
| Lector completo | Pendiente | Markdown cotidiano completo |
| Editor básico | Pendiente | Edición y guardado fiel |
| Chrome | Pendiente | Ventana, pestañas y comandos |
| Workspace | Pendiente | Carpetas, búsqueda e índice |
| Obsidian | Pendiente | Wikilinks, backlinks y callouts |
| Estudio | Pendiente | Herramientas portables |
| Exportación | Pendiente | PDF, DOCX y copias preparadas |
| Distribución | Pendiente | Windows y Linux profesionales |

## Sprint 0: viabilidad nativa

### Objetivo

Demostrar que Rust y una pila de dibujo nativa podían cumplir tamaño, apertura y
scroll sin WebView.

### Resultado real

- ventana y superficie por software;
- parsing y layout básicos;
- virtualización inicial;
- mediciones con documento normal y grande;
- tema y fuentes incorporados en el último commit estable;
- binario ampliamente debajo del límite actual.

### Deuda que no se oculta

- reproducibilidad incompleta;
- Linux no medido con la misma profundidad;
- auditoría de `unsafe` y C parcial;
- fuente y toolchain no fijados completamente;
- accesibilidad no demostrada.

Sprint 0 no se reabre, pero esa deuda entra en Validación base.

## Etapa 1: recuperación del working tree

### Objetivo

Preservar lo valioso del trabajo interrumpido, eliminar estados parciales y
establecer una base compilable que no bloquee edición futura.

### Trabajo

1. Registrar baseline, hashes, toolchain y diff.
2. Clasificar temas, fuentes, inline, bloques, límites, markers y tests.
3. Recuperar compilación para caracterizar comportamiento.
4. Definir el contrato mínimo del modelo documental con rangos y semántica.
5. Trasladar las funciones válidas al modelo correcto.
6. Cerrar proceso, licencias y cobertura de fuentes.
7. Separar commits revisables sin integrar snapshots.

### Criterios de salida

- source actual compilable;
- tests reconstruidos y verdes;
- `Marker` conectado de parser a dibujo;
- casillas verificadas en parsing, layout y rendering;
- límite de anidamiento y fallback probados;
- modelo preparado para selección y edición;
- fuentes reproducibles y licenciadas;
- ningún cambio heredado perdido sin decisión registrada;
- diff dividido en commits coherentes.

## Etapa 2: cierre de Sprint 1

### Objetivo

Entregar un lector mínimo profesional, seguro y usable que sirva de base al
editor.

### Trabajo

- separar módulos mínimos;
- modelo documental con rangos;
- CommonMark aplicable;
- extensiones GFM ya aprobadas y completas;
- allowlist HTML semántica;
- límites y vista segura;
- parsing fuera del camino crítico de UI;
- layout, alturas, scroll, resize y DPI correctos;
- Unicode y fallback;
- selección, copia y navegación por teclado;
- menú contextual inicial;
- accesibilidad mínima demostrada;
- diseño Papel y tinta preservado;
- casos valiosos portados desde v1.

### Criterios de salida

- corpus CommonMark acordado verde;
- cada sintaxis declarada llega a rendering;
- enlaces y bloques conservan semántica y origen;
- entradas patológicas no bloquean ni desbordan;
- modo seguro visible y útil;
- apertura normal sin red ni acceso secundario arbitrario;
- selección y copia utilizables;
- resize, zoom y DPI sin corrupción;
- trabajo por frame principalmente proporcional a contenido visible;
- release medido contra menos de 8 MB;
- documentación y matriz sincronizadas.

## Etapa 3: validación base

### Objetivo

Convertir seguridad, calidad y rendimiento en propiedades demostrables antes de
ampliar el producto.

### Trabajo

- CI Windows MSVC y Linux;
- format, clippy, tests y release;
- CommonMark oficial y corpus del proyecto;
- fuzzing de parser, modelo y rutas;
- property tests;
- VFS, UNC, traversal, symlinks y junctions;
- monitor de sockets;
- benchmarks versionados;
- auditoría de dependencias, licencias y `unsafe`;
- SBOM;
- matriz visual y de accesibilidad;
- threat model y ADR actualizados.

### Criterios de salida

- gates verdes en Windows y Linux;
- cero red durante apertura y render normales;
- contención de archivos demostrada;
- campaña de fuzzing sin hallazgos abiertos críticos;
- SBOM reproducible;
- advisories resueltos o aceptados explícitamente;
- benchmarks repetibles con umbrales;
- código, tests y documentación alineados.

## Etapa 4: lector completo

### Alcance

- tablas;
- footnotes y autolinks;
- enlaces y destinos visibles;
- imágenes locales seguras;
- bloques de código y resaltado medido;
- índice filtrable;
- búsqueda con marcas de scroll;
- plegado de secciones;
- contador de palabras y lectura;
- copiar tablas como TSV;
- link peek seguro.

### Criterios de salida

- documentos GitHub y Obsidian elegidos se leen sin pérdida importante;
- imágenes pasan VFS y límites;
- tablas son seleccionables y copiables;
- navegación por teclado completa;
- documentos grandes mantienen presupuesto;
- toda sintaxis soportada tiene corpus y UX de error.

## Etapa 5: editor básico y guardado

Esta etapa se adelanta respecto del roadmap histórico. Guardar fielmente condiciona
workspace, anotaciones y Obsidian, por lo que descubrirlo tarde sería costoso.

### Alcance

- editor fuente;
- vista dividida;
- mapeo fuente y render;
- undo y redo;
- ayudas Markdown discretas;
- crear, abrir, guardar y guardar como;
- guardado atómico;
- cambios externos y conflictos;
- EOL, BOM y UTF-8;
- preservación de sintaxis desconocida;
- recuperación ante cierre inesperado.

### Criterios de salida

- round-trip probado con contenido conocido y desconocido;
- ningún guardado parcial ante fallo simulado;
- IME, selección y portapapeles verificados;
- edición de documentos grandes permanece responsiva;
- Obsidian y GitHub releen los archivos de prueba sin cambios inesperados.

## Etapa 6: chrome, pestañas y comandos

### Alcance

- ventana sin borde;
- pestañas, fijado y estado sucio;
- varios documentos;
- menú principal y contextual;
- paleta de comandos;
- paneles progresivos;
- búsqueda global de comandos;
- siempre encima si supera QA de plataforma;
- animaciones y reduce motion.

### Criterios de salida

- acciones esenciales descubribles sin atajos;
- profundidad disponible sin sobrecarga visual;
- cierre nunca pierde cambios;
- teclado y lector de pantalla cubren comandos;
- artifacts aprobados usados como referencia visual.

## Etapa 7: workspace

### Alcance

- abrir carpeta;
- árbol de archivos;
- índice incremental;
- búsqueda en carpeta;
- navegación rápida;
- papelera recuperable;
- confianza temporal delimitada;
- detección de cambios externos.

### Criterios de salida

- una carpeta grande no bloquea UI;
- índice respeta VFS, ignores y límites;
- borrar es recuperable;
- confiar no habilita red o ejecución;
- cambios externos se reflejan de manera predecible.

## Etapa 8: Obsidian y GitHub

### Alcance

- wikilinks;
- backlinks;
- callouts;
- etiquetas y frontmatter en lectura;
- referencias existentes a encabezados o bloques;
- rutas relativas compatibles;
- comportamiento Git amistoso, sin reescrituras masivas.

### Criterios de salida

- bóvedas de prueba abren sin migración;
- backlinks se actualizan incrementalmente;
- enlaces no escapan de la política;
- guardar no genera ruido innecesario en Git;
- sintaxis no soportada se preserva.

## Etapa 9: estudio y trabajo con IA

### Alcance

- resaltado portable;
- sidecars solo para estado no portable;
- preguntas y respuestas;
- ocultar respuestas;
- estados de aprendizaje;
- listas de conceptos;
- repaso espaciado simple;
- copiar Markdown de un bloque;
- fragmentar documentos;
- comparar versiones;
- preparar adjuntos y copias.

### Criterios de salida

- Obsidian interpreta las anotaciones portables;
- sidecars tienen versión, recuperación y sincronización;
- fragmentar no rompe bloques estructurales;
- comparar no modifica archivos;
- no hay modelo de IA ni conexión implícita.

## Etapa 10: exportación

### Prioridad

1. PDF visualmente fiel.
2. DOCX compatible con universidad y trabajo.
3. Copia preparada para plataformas.

### Criterios de salida

- exportar contenido hostil no ejecuta recursos;
- PDF conserva tipografía, enlaces y paginado acordados;
- estrategia DOCX tiene compatibilidad y coste medidos;
- componentes pesados quedan aislados cuando conviene;
- exportación no altera el documento fuente.

## Etapa 11: distribución

### Windows

- MSVC;
- instalador y desinstalador;
- asociación de archivos;
- firma cuando sea viable;
- integración de tema y accesibilidad;
- paquete, SBOM y notices.

### Linux

- build y paquetes elegidos;
- asociación MIME;
- fontconfig y dependencias nativas documentadas;
- Wayland y X11 según soporte real;
- accesibilidad y DPI;
- paquete, SBOM y notices.

### Criterios de salida v2.0

- recorridos principales estables en Windows y Linux;
- instaladores reproducibles;
- datos del usuario protegidos;
- threat model revisado;
- matriz de release completa;
- tamaño, memoria y arranque publicados;
- riesgos residuales documentados;
- aprobación manual del producto.

## Después de v2.0

Solo se evalúan con evidencia de uso:

- edición en vivo;
- matemática opcional;
- Mermaid nativo;
- corrector descargable;
- grafo;
- plugins declarativos sin código arbitrario;
- macOS;
- funciones de estudio adicionales.

Ver [`future.md`](future.md).
