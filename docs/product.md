# Producto

## Definición

Visor MD v2 es una aplicación nativa para leer, editar y estudiar Markdown y
otros archivos de texto inerte. Combina apertura inmediata, presentación
editorial, edición fiel, integración con bóvedas y seguridad por construcción.

No es solo un visor. La edición, el guardado y el trabajo cotidiano con texto son
parte del producto.

## Usuarios

### Principales

1. Personas que reciben, corrigen y reutilizan documentos producidos por IA.
2. Estudiantes que leen y enriquecen apuntes durante clases o sesiones de
   estudio.

### Secundarios

3. Usuarios de Obsidian y segundos cerebros.
4. Profesionales técnicos que abren documentación y texto desconocido.

El producto debe ser fácil para alguien que no domina Markdown y suficientemente
predecible para quien depende de su sintaxis y estructura.

## Promesa

Visor MD debe abrir un `.md` con la inmediatez de una aplicación de texto simple,
mostrarlo como un documento terminado y permitir corregirlo sin romper lo que
otra herramienta o una IA necesita leer después.

Debe sentirse elegante, técnico, poderoso y confiable. No debe sentirse
sobrecargado, genérico, frágil ni como un Bloc de notas con formato superficial.

## Experiencias prioritarias

1. Doble clic, apertura inmediata y lectura cómoda.
2. Corrección rápida y guardado fiel.
3. Estudio de un documento largo.
4. Apertura de una respuesta producida por IA.
5. Varios documentos abiertos.
6. Copia limpia de fragmentos.
7. Comparación entre fuente y resultado.
8. Creación de documentos.
9. Navegación por notas conectadas.
10. Búsqueda en carpetas y bóvedas.

El orden ayuda a resolver dependencias. No convierte las últimas experiencias en
opcionales.

## Modos de documento

### Lectura

Modo inicial para un documento abierto por primera vez. Prioriza tipografía,
ancho de lectura, índice, enlaces, selección y navegación.

### Fuente

Edición Markdown directa con ayudas discretas. No intenta ocultar la sintaxis ni
reescribirla automáticamente.

### Dividida

Fuente y resultado visibles a la vez, con posición sincronizada cuando el modelo
documental lo permita de forma correcta.

### Edición en vivo

Objetivo posterior y de mayor complejidad. No bloquea el editor básico ni la
vista dividida.

La aplicación recuerda localmente el último modo usado para cada archivo. No
escribe preferencias dentro del documento.

## Edición y fidelidad

El usuario puede modificar el documento sin ser experto en sintaxis. Las ayudas
deben producir Markdown comprensible para Obsidian, GitHub, editores y modelos de
IA.

Reglas:

- preservar contenido no editado;
- conservar sintaxis desconocida en la medida técnicamente posible;
- no reformatear el archivo completo al guardar;
- no normalizar EOL, BOM o espacios silenciosamente;
- guardar de forma atómica;
- detectar cambios externos;
- hacer visible todo riesgo de pérdida.

## Estudio

Visor MD aprovecha el propio documento antes de crear sistemas paralelos.

Funciones buscadas:

- resaltado portable;
- preguntas y respuestas;
- contenido ocultable para practicar memoria;
- estados entendido, dudoso o pendiente;
- resúmenes;
- listas de conceptos;
- relaciones entre documentos;
- exportación a otras herramientas.

Usar sintaxis compatible con Obsidian cuando exista. Los datos que no encajen en
Markdown pueden vivir en sidecars documentados. No crear sintaxis exclusiva de
Visor MD.

## Trabajo con IA

Visor MD no incorpora un modelo de IA.

Ayudas previstas:

- copiar el Markdown original de un bloque;
- dividir documentos largos en fragmentos;
- comparar versiones;
- generar archivos listos para adjuntar;
- preparar copias para Discord, correo u otras plataformas;
- estimar tokens solo si el coste es insignificante.

El producto optimiza el formato y el flujo, no reemplaza a la herramienta de IA.

## Obsidian y segundo cerebro

Visor MD es un buen ciudadano dentro de bóvedas existentes.

Esencial:

- wikilinks;
- backlinks;
- callouts;
- búsqueda de carpeta;
- índice;
- navegación rápida;
- apertura sin migración ni cambios inesperados.

Deseable:

- etiquetas;
- frontmatter;
- adjuntos;
- referencias a encabezados o bloques;
- grafo visual.

No pretende reemplazar Obsidian. Convertirlo en un segundo cerebro completo
requeriría una decisión de producto separada.

## Otros archivos de texto

Reconocer `.txt`, `.json`, `.yaml`, `.toml`, `.csv`, archivos de código y otros
formatos textuales seguros. Se muestran como texto inerte y nunca se ejecutan.
Puede existir edición básica, pero no inteligencia de IDE, compilación o
ejecución.

## Exportación

Prioridades:

1. PDF visualmente fiel.
2. DOCX para universidad y trabajo.
3. Copia preparada para Discord, correo y otras plataformas.

HTML autónomo, texto plano e impresión son deseables. Los componentes pesados se
aislarán cuando ayude a conservar el núcleo pequeño.

## Seguridad percibida

La seguridad es silenciosa en el uso normal. Cuando bloquea algo:

- muestra un aviso discreto;
- explica brevemente qué ocurrió;
- ofrece detalles técnicos opcionales;
- no responsabiliza al usuario por contenido hostil.

La configuración avanzada puede ofrecer excepciones delimitadas, pero nunca
ejecución de scripts, cambios ordenados por documentos o conexiones ocultas.

## Alcance de v2.0

La versión estable debe entregar:

- lector Markdown profesional;
- editor básico y vista dividida;
- guardado fiel;
- chrome, pestañas, búsqueda y menú contextual;
- workspace de carpetas;
- compatibilidad esencial con Obsidian;
- herramientas de estudio priorizadas;
- exportación PDF y una estrategia cerrada para DOCX;
- instalación Windows y Linux;
- threat model, QA, benchmarks, SBOM y documentación de release.

Las funciones se entregarán mediante hitos internos utilizables. La etiqueta
v2.0 no justifica construir todo simultáneamente.

## Fuera de alcance actual

- IA propia o chatbot embebido;
- ejecución de HTML o JavaScript;
- plugins con código arbitrario;
- emulación completa de Obsidian;
- IDE o compilador;
- motores remotos de diagramas;
- cifrado de una bóveda secreta;
- colaboración en tiempo real.

## Criterio de éxito

Visor MD cumple su propósito cuando:

- se convierte en la aplicación predeterminada para Markdown y texto seguro;
- se usa habitualmente para estudiar;
- da confianza al abrir documentos desconocidos;
- se recomienda por su diseño;
- otras personas lo adoptan;
- el repositorio demuestra dirección de producto, ciberseguridad y QA reales.
