# AGENTS.md

## Proposito

Visor MD v2 es una aplicacion nativa para leer y editar Markdown y otros
archivos de texto inerte. Debe sentirse tan inmediata como el Bloc de notas,
mostrar Markdown con calidad editorial y permitir editar, estudiar, anotar,
copiar, compartir y trabajar con documentos producidos por IA o guardados en
bovedas de Obsidian.

Los usuarios principales son:

1. personas que trabajan habitualmente con documentos producidos por IA;
2. estudiantes;
3. usuarios de Obsidian y otros segundos cerebros;
4. profesionales que abren documentacion y archivos desconocidos.

Visor MD no es un navegador, un IDE, un chatbot ni un reemplazo completo de
Obsidian.

## Prioridades no negociables

En caso de conflicto, evaluar explicitamente:

1. seguridad y ausencia de perdida de datos;
2. correccion y estabilidad;
3. velocidad de apertura y respuesta;
4. bajo consumo de recursos;
5. experiencia de lectura y edicion;
6. accesibilidad y Unicode;
7. superficie de ataque y dependencias reducidas;
8. tamano del binario;
9. mantenibilidad y calidad visual.

El tamano tiene tres referencias:

- ideal extraordinario: menos de 6 MB;
- objetivo normal: aproximadamente 7 MB;
- limite deseado: menos de 8 MB.

Superar 8 MB requiere medicion, explicacion y aprobacion. No reducir seguridad,
estabilidad, accesibilidad, Unicode o funciones esenciales solo para alcanzar
una cifra.

## Relacion con el propietario

El propietario dirige el producto y no necesita dominar los detalles internos de
Rust, rendering, ciberseguridad o QA para tomar decisiones de producto. Estudia
redes y ciberseguridad y quiere aprender durante el desarrollo. Las explicaciones
de seguridad y QA deben conservar rigor y detalle, pero presentarse en lenguaje
natural, definir la jerga y conectar cada concepto con un riesgo, una prueba o un
ejemplo concreto de Visor MD. No ocultar informacion tecnica util por asumir que
puede resultar compleja; explicarla por capas y comprobar las decisiones desde
sus consecuencias practicas.

Los agentes deben:

- explicar decisiones importantes en lenguaje sencillo;
- definir terminos tecnicos cuando ayude a aprender;
- separar hechos medidos, inferencias y preferencias;
- cuestionar de forma directa decisiones que contradigan la filosofia;
- ofrecer alternativas cuando haya varias opciones razonables;
- indicar si una decision es facil o dificil de revertir;
- trabajar con autonomia dentro de un plan aprobado;
- detenerse ante ambiguedades de producto, seguridad, diseno, alcance o
  arquitectura dificil de revertir;
- solicitar QA manual solo cuando la percepcion humana aporte evidencia real.

Durante el trabajo, dar actualizaciones breves y un cierre verificable. En modo
autonomo, avanzar hasta completar el bloque aprobado o encontrar un bloqueante
real. La autonomia no autoriza cambios silenciosos de alcance, publicacion,
arquitectura o politicas de seguridad.

## Fuentes de autoridad

Antes de modificar archivos:

1. leer este archivo;
2. inspeccionar rama, HEAD, `git status` y `git diff`;
3. leer la documentacion y ADR relacionados;
4. comprobar la implementacion y los tests;
5. informar contradicciones relevantes.

La documentacion puede estar desactualizada y el codigo puede estar incompleto.
No asumir que uno de los dos es correcto sin comprobarlo.

Las decisiones explicitas del propietario prevalecen sobre documentos antiguos,
pero se deben cuestionar si crean riesgos o contradicen los principios del
producto. Las decisiones arquitectonicas duraderas deben registrarse mediante un
ADR practico.

La fotografia detallada del traspaso inicial esta en
`docs/workspace-handoff.md`. Es evidencia historica y se consulta cuando haga
falta reconstruir decisiones, no como sustituto de este archivo.

## Identidad del producto

Visor MD debe transmitir:

- elegancia;
- capacidad tecnica;
- potencia;
- confianza;
- rapidez;
- cuidado visual.

Nunca debe sentirse sobrecargado, generico, inestable, incomodo, de mal gusto o
como un Bloc de notas con formato superficial.

La interfaz usa divulgacion progresiva: acciones esenciales visibles y funciones
menos frecuentes mediante menu contextual, menu principal, paleta de comandos y
paneles que aparecen cuando se necesitan.

Al abrir un documento por primera vez, iniciar en modo lectura. Recordar
localmente el ultimo modo usado para cada documento. El cambio a edicion debe ser
rapido y evidente, sin modificar el archivo para recordar preferencias.

## Diseno

Direccion visual aprobada:

- concepto Papel y tinta;
- armonias verdes reconocibles;
- ventana sin borde;
- iconografia suave;
- contraste tipografico editorial;
- contenido principalmente plano;
- elevacion suave solo para elementos superpuestos;
- animaciones breves que expliquen cambios de estado;
- respeto por la preferencia de reducir movimiento.

Familias tipograficas previstas:

- Newsreader para lectura editorial;
- Sora para interfaz;
- JetBrains Mono para codigo.

Las fuentes embebidas deben tener licencia, procedencia y proceso de subset
documentados y reproducibles. Conservar fallback para Unicode no incluido en el
subset. No sacrificar cobertura esencial para ahorrar unos KB.

Los archivos de `Artifac opciones de diseño` son referencias visuales. No copiar
su runtime web ni incorporarlos automaticamente al producto.

## Formatos y compatibilidad

Markdown es el formato principal. Objetivos:

- CommonMark;
- extensiones GFM elegidas explicitamente;
- formatos habituales de GitHub y Obsidian;
- Unicode general;
- degradacion segura de sintaxis desconocida.

Reconocer tambien `.txt`, `.json`, `.yaml`, `.toml`, `.csv`, archivos de codigo y
otros textos inertes. Mostrarlos como texto y no intentar convertirse en un IDE,
ejecutarlos ni inferir comportamientos activos. Una edicion basica futura no debe
incorporar inteligencia de compilacion o ejecucion.

Preservar exactamente, en la medida tecnicamente posible, las partes del archivo
que el usuario no edito, incluso si contienen sintaxis desconocida. No
normalizar, reformatear o reescribir silenciosamente un documento al guardarlo.

No declarar una sintaxis como soportada solo porque el parser la reconoce. Debe
existir soporte coherente en modelo, layout, rendering, interaccion y tests.

## Modelo documental

El modelo canonico no puede ser una lista destructiva de fragmentos preparados
solo para dibujar. Debe conservar, cuando corresponda:

- semantica Markdown;
- rangos de fuente;
- estructura padre e hijo;
- texto y destino de enlaces;
- idioma de bloques de codigo;
- marcadores y estado de tareas;
- estructura y alineacion de tablas;
- identificadores de encabezados;
- texto original necesario para guardar sin perdida;
- diagnosticos y degradaciones.

El layout y los comandos de dibujo son representaciones derivadas y
descartables. No consolidar un modelo que impida seleccion precisa, edicion,
anotaciones, comparacion, sincronizacion fuente y vista o guardado fiel.

## Estudio y anotaciones

Preferir sintaxis Markdown u Obsidian portable para:

- resaltados;
- preguntas y respuestas;
- contenido ocultable;
- estados entendido, dudoso o pendiente;
- resumenes;
- relaciones entre documentos;
- listas de conceptos.

Usar sintaxis compatible con Obsidian cuando exista. Para anotaciones que no
puedan expresarse limpiamente, usar archivos auxiliares bien definidos. Evitar
crear sintaxis exclusiva de Visor MD.

No implementar un sistema separado de notas al margen cuando el mismo resultado
pueda representarse de forma comoda y portable en Markdown.

## Obsidian y espacios de trabajo

Visor MD debe ser un buen ciudadano dentro de bovedas existentes y no
modificarlas inesperadamente.

Funciones esenciales a largo plazo:

- wikilinks;
- backlinks;
- callouts;
- busqueda en carpetas;
- indice y estructura del documento;
- navegacion rapida entre notas;
- compatibilidad con bovedas existentes sin migracion.

Funciones deseables:

- etiquetas;
- frontmatter;
- adjuntos;
- grafo visual;
- referencias a encabezados o bloques.

No ampliar accidentalmente el producto hasta convertirlo en un segundo cerebro
completo. Una ambicion de ese tipo requiere una decision de producto separada.

## Trabajo con IA

Visor MD no incorpora IA propia y no depende de un servicio de IA. Debe ser
excelente para documentos producidos, enviados o releidos por IA.

Ayudas priorizadas:

- copiar el Markdown original de un bloque;
- dividir documentos largos en fragmentos apropiados;
- comparar una version anterior con una nueva;
- generar un Markdown listo para adjuntar;
- preparar copias para Discord, correo u otras plataformas.

Una estimacion aproximada de tokens solo se incorpora si su coste de desarrollo,
dependencias y tamano es insignificante. En caso contrario queda como funcion
futura.

El README puede explicar de forma honesta el flujo asistido por IA y la direccion
de producto humana. Evitar metadiscurso sobre IA en codigo, comentarios, tests y
commits.

No usar emojis decorativos, frases genericas, muletillas de IA ni comentarios
que narren lo evidente. Los comentarios deben ser humanos, concretos y
funcionales. Mantener el estilo documental del repositorio y evitar rayas largas
como recurso de redaccion.

## Exportacion

Prioridades de producto:

1. PDF visualmente fiel;
2. DOCX util para universidad o trabajo;
3. copia preparada para Discord, correo y plataformas similares.

HTML autonomo, texto plano e impresion directa son deseables. Los exportadores
pesados deben evaluarse como componentes opcionales o aislados para no inflar el
nucleo ni su superficie de ataque.

## Seguridad

Tratar documentos, nombres, rutas, enlaces, imagenes y metadatos como entradas
potencialmente hostiles.

### Red

El uso normal del nucleo no realiza conexiones:

- sin telemetria;
- sin analytics;
- sin actualizaciones silenciosas;
- sin fuentes o dependencias remotas;
- sin prefetch;
- sin carga automatica de imagenes remotas.

Un enlace `http` o `https` puede abrirse en el navegador del sistema solo tras un
clic explicito. Mostrarlo con apariencia inequívoca de hipervinculo y revelar su
destino real de forma legible. El color azul por si solo no protege contra
phishing. Normalizar y mostrar dominios confusos, rechazar esquemas peligrosos y
no seguir el enlace dentro de un motor web propio.

Una imagen remota puede cargarse solo despues de consentimiento claro. Si se
implementa, aislar la capacidad de red del nucleo y aplicar limites de protocolo,
redireccion, tiempo, tipo, dimensiones y bytes. No enviar cookies, credenciales,
rutas locales ni contenido del documento. Explicar que la carga revela al menos
la direccion de red del usuario al servidor remoto.

Las pruebas deben verificar ausencia de conexiones durante apertura y render
normal.

### Rutas y archivos

El archivo principal puede abrirse desde una ruta elegida explicitamente por el
usuario, incluida una ruta UNC abierta manualmente.

El documento no puede provocar por si mismo acceso arbitrario:

- no seguir automaticamente UNC;
- no resolver automaticamente `file://`;
- no cargar rutas absolutas indicadas por contenido;
- no escapar mediante `..`, symlinks o junctions;
- no cargar recursos secundarios desde UNC;
- no abrir automaticamente enlaces o archivos locales.

Las imagenes locales relativas pueden cargarse tras aplicar la politica de VFS,
contencion y limites. Seguir enlaces a otros archivos requiere una accion
explicita.

Confiar temporalmente en una boveda solo puede ampliar el acceso local dentro de
esa boveda. No habilita scripts, red silenciosa ni cambios de configuracion.

Centralizar el acceso a disco en una capa VFS o politica. Evitar lecturas
dispersas desde parser, renderer, exportadores o componentes opcionales. Aplicar
canonicalizacion, contencion y proteccion ante cambios de identidad del archivo
cuando corresponda.

### HTML

Visor MD no ejecuta HTML.

Allowlist semantica inicial:

- `br`;
- `kbd`;
- `mark`;
- `sub`;
- `sup`.

`details` y `summary` solo pueden incorporarse como componentes nativos propios,
sin DOM, scripts o recursos remotos, si su complejidad resulta razonable.

HTML desconocido se muestra como fuente inerte o texto escapado. No interpretar:

- scripts;
- iframes;
- estilos inline;
- manejadores de eventos;
- formularios;
- recursos remotos;
- contenido interactivo arbitrario.

### Limites y modo seguro

Definir y probar limites de tamano, profundidad, nodos, longitud de linea,
imagenes, memoria, trabajo de parsing, layout y recursión.

El usuario avanzado puede aumentar el limite blando de tamano de archivo. Deben
seguir existiendo limites absolutos de seguridad, cancelacion y fallback. No
intentar render enriquecido si hacerlo puede agotar memoria o bloquear la UI.

Si el render enriquecido supera un limite, cancelar esa ruta y ofrecer fuente
inerte con un aviso discreto. Mostrar una explicacion simple y detalles tecnicos
opcionales. Rechazar por completo solo cuando el archivo no pueda leerse o
mostrarse de forma segura.

### Configuracion avanzada

Puede permitir de forma explicita y delimitada:

- imagenes remotas confirmadas;
- imagenes locales relativas;
- apertura manual de enlaces web;
- seguimiento manual de enlaces a archivos;
- rutas UNC abiertas manualmente;
- confianza temporal en una boveda;
- limites blandos de archivo mayores.

La configuracion debe resistir phishing y explicar alcance, duracion y riesgo.
Preferir permisos por documento, carpeta o sesion antes que excepciones globales.

Nunca permitir:

- ejecucion de scripts;
- eventos HTML;
- cambios de seguridad ordenados por un documento;
- envio silencioso de contenido;
- conexiones ocultas.

### Codigo inseguro y dependencias nativas

Evitar `unsafe` propio. Si resulta imprescindible, aislarlo, documentar sus
precondiciones, probarlo y revisarlo manualmente. Revisar tambien `unsafe`, C,
C++ y bindings nativos transitivos.

## Edicion y guardado

Antes de consolidar el editor, definir y probar:

- UTF-8, BOM y codificaciones invalidas;
- finales de linea;
- guardado atomico;
- permisos y errores de escritura;
- cambios externos;
- recuperacion ante cierre o fallo;
- conflictos entre memoria y disco;
- preservacion de sintaxis desconocida.

No sobrescribir destructivamente. No normalizar contenido no editado. La vista
renderizada y la fuente deben relacionarse mediante rangos estables.

## Arquitectura

La base inicial es un prototipo nativo de Sprint 0. No confundirla con la
arquitectura final.

Direccion tecnologica actual:

- Rust;
- `winit` para ventana y eventos;
- `softbuffer` para presentar el framebuffer;
- `tiny-skia` para dibujo 2D;
- `parley` y `swash` para texto;
- `comrak` para Markdown;
- sin WebView, DOM o JavaScript.

Esta seleccion puede revisarse si evidencia reproducible demuestra que no cumple
accesibilidad, Unicode, edicion, seguridad, mantenimiento o rendimiento.

Separar progresivamente:

- modelo documental;
- parsing;
- politicas y limites;
- VFS;
- layout;
- rendering;
- estado y comandos;
- edicion y guardado;
- exportacion;
- pruebas y medicion.

No hacer una reorganizacion masiva sin tests que preserven comportamiento.

## Rendering, texto y accesibilidad

El trabajo por frame debe depender principalmente de contenido visible. Evitar
recorridos O(n) del documento completo durante cada frame.

Validar desde temprano:

- seleccion con mouse y teclado;
- copiar y seleccionar todo;
- menu contextual;
- navegacion por teclado;
- foco visible;
- IME;
- Unicode y fallback;
- texto bidireccional cuando corresponda;
- DPI y zoom;
- lectores de pantalla;
- alto contraste;
- reduccion de movimiento.

La accesibilidad no es una funcion del ultimo sprint. Si el renderer artesanal
impide alcanzar un resultado profesional, reevaluar la estrategia antes de
acumular mas funciones.

## Rendimiento y medicion

Usar builds release reproducibles y registrar:

- sistema operativo y arquitectura;
- version y target de Rust;
- flags;
- corpus;
- commit;
- tamano;
- tiempo de ventana visible y primer contenido;
- memoria;
- comportamiento de scroll.

Presupuestos iniciales:

- ventana visible en menos de 150 ms cuando sea viable;
- primer contenido util en menos de 400 ms;
- trabajo por frame proporcional al contenido visible.

Separar siempre tiempo de compilacion y tiempo de arranque. No aceptar mediciones
manuales imposibles de repetir.

## Dependencias y cadena de suministro

Antes de agregar o actualizar una dependencia:

- justificarla;
- revisar funciones por defecto y transitivas;
- medir binario antes y despues;
- comprobar mantenimiento y advisories;
- revisar licencia;
- revisar `unsafe` y codigo nativo;
- revisar red y filesystem;
- considerar una implementacion pequena propia solo si es mas mantenible y
  segura.

Mantener SBOM y notices. Usar `cargo audit`, `cargo deny` o equivalentes. Una
advertencia se corrige o se documenta con responsable y condicion de revision.

## Tests y evidencia

Las pruebas deben comprobar propiedades, no solo ausencia de panic.

Capas esperadas:

- unit tests;
- corpus oficial CommonMark aplicable;
- casos GFM y Obsidian;
- casos historicos portados desde v1;
- entradas patologicas;
- property tests;
- fuzzing;
- pruebas de VFS y rutas;
- pruebas de ausencia de red;
- round-trip y guardado;
- layout y rendering;
- regresion visual;
- benchmarks reproducibles;
- matriz manual de UX y accesibilidad;
- Windows y Linux.

Comandos minimos, adaptados al cambio:

- `cargo fmt --check`;
- `cargo clippy --all-targets --all-features`;
- `cargo test`;
- build release relevante;
- auditoria de dependencias;
- pruebas especificas de seguridad y rendimiento.

No afirmar que los tests pasan usando un binario anterior. Deben compilarse desde
el working tree actual.

## Git y preservacion

`main` representa el ultimo estado estable. Trabajar en ramas `codex/...` o en la
rama acordada.

Antes de tocar archivos:

- comprobar rama y HEAD;
- inspeccionar status y diff;
- distinguir cambios heredados y propios;
- preservar trabajo ajeno;
- no restaurar, resetear, borrar, mover o sobrescribir sin autorizacion.

No integrar como producto:

- snapshots de recuperacion;
- backups;
- artifacts de diseno;
- binarios de medicion;
- temporales.

Los commits deben ser pequenos, coherentes y compilables. No mezclar recuperacion,
refactor, funcion nueva y actualizacion masiva de dependencias.

No cambiar dependencias, toolchain, licencia, arquitectura o politicas de
seguridad sin explicar impacto y obtener la aprobacion que corresponda.

## Documentacion

Mantener periodicamente:

- README;
- arquitectura;
- roadmap;
- threat model;
- seguridad y conectividad;
- ADR;
- presupuesto y benchmarks;
- matriz de pruebas;
- SBOM y licencias;
- registro de funciones terminadas.

La documentacion es evidencia operativa, no decoracion. Actualizarla cuando cambia
una decision o un comportamiento, no mediante grandes correcciones tardias.

El README es el lugar publico para explicar el uso profesional de IA y la
direccion de producto humana. No repetir esa narrativa como muletilla por todo el
repositorio.

## Referencias

La v1 sirve para recuperar UX, atajos, menu contextual, casos de prueba y flujos
reales. No reutilizar su arquitectura WebView como objetivo.

Tinta, Markdown Viewer y otros competidores son referencias puntuales. Sus
funciones y metricas cambian. No copiar su roadmap ni agregar funciones solo para
igualar listas.

## Criterio de finalizacion

Una tarea no esta terminada hasta que:

- compila desde el working tree actual;
- pasan los tests relevantes recien compilados;
- no quedan estructuras parcialmente conectadas;
- los limites de seguridad estan probados;
- se midio el impacto relevante;
- Windows y Linux fueron considerados;
- documentacion y comportamiento coinciden;
- la deuda aceptada esta registrada;
- el estado de Git es entendible y comunicable.

`Funciona en mi maquina` y `no hizo crash` no son criterios suficientes.
