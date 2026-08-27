# Traspaso de espacio de trabajo y contexto profundo

Fecha de consolidacion: 25 de agosto de 2026.

Este documento conserva la fotografia historica del traspaso de Visor MD v2
desde Claude Code hacia Codex. Se revisa con menos frecuencia que `AGENTS.md`.
Su funcion es explicar de donde viene el estado actual, que se comprobo, que
queda incompleto y por que existe el plan de recuperacion.

No es una especificacion inmutable. Las decisiones operativas vigentes se
mantienen en `AGENTS.md`, los ADR y la documentacion tematica.

## Alcance de la investigacion

La auditoria inicial incluyo:

- documentacion viva de v2;
- `README.md`, `Cargo.toml` y `Cargo.lock`;
- historial, ramas, remote, status y diff;
- cambios locales y snapshots de preservacion;
- codigo y tests del prototipo;
- fuentes embebidas;
- version anterior en `../visor md`;
- artifacts de diseno recuperados;
- estado publico de Tinta;
- estado publico de ThisIs-Developer/Markdown-Viewer;
- build, tests y auditoria de dependencias en modo de inspeccion.

No se modifico codigo durante la fase de investigacion.

## Modelo mental consolidado

Visor MD v2 quiere ser la aplicacion cotidiana para trabajar con Markdown y
texto inerte en una PC. Debe abrir tan rapido y sentirse tan liviana como el Bloc
de notas, pero representar el documento con calidad editorial y ofrecer lectura,
edicion, estudio, anotacion, comparacion, exportacion y navegacion de bovedas.

El producto sirve principalmente a personas que reciben Markdown de una IA y a
estudiantes. Tambien debe resultar especialmente comodo para usuarios de
Obsidian y seguro para profesionales que abren documentacion desconocida.

Dos ejemplos de uso definidos por el propietario:

1. Una IA cruza informacion de varios PDF y entrega resumenes Markdown conectados
   con una boveda universitaria. El usuario abre el resultado de inmediato, copia
   un bloque y lo comparte por Discord.
2. Durante una clase, el usuario abre un documento, resalta lo importante y
   agrega aclaraciones mediante sintaxis Markdown. Luego guarda el archivo y lo
   continua usando desde Obsidian.

La aplicacion no incorpora IA propia. Su valor esta en trabajar excepcionalmente
bien con documentos producidos y consumidos por IA.

## Promesa de producto

Para una persona no tecnica:

> Corre tan ligero como el Bloc de notas, muestra el formato como corresponde,
> permite editar, comprobar el resultado y hacer anotaciones universitarias con
> una experiencia visual cuidada.

Para una persona tecnica:

> Abre de inmediato documentos producidos por IA, permite editarlos sin dominar
> la sintaxis, preserva un Markdown que otras herramientas pueden releer, exporta
> a formatos utiles y se integra con bovedas sin obligar a abrir Obsidian.

## Identidad y limites

Rasgos buscados:

- elegante;
- tecnico;
- poderoso;
- confiable;
- optimizado;
- visualmente memorable.

Rasgos rechazados:

- sobrecargado;
- generico;
- incomodo;
- inestable;
- de mal gusto;
- parecido a una interfaz producida sin criterio;
- reducido a un Bloc de notas con formato.

Visor MD no busca:

- incorporar un chatbot;
- ejecutar documentos;
- convertirse en IDE;
- emular completamente Obsidian;
- sumar complejidad solo porque un competidor ofrece una funcion.

## Prioridad de escenarios

Orden orientativo definido por el propietario, aunque todos son importantes:

1. doble clic, apertura inmediata y lectura comoda;
2. correccion rapida y guardado;
3. estudio de documentos largos;
4. apertura de respuestas producidas por IA;
5. varios documentos abiertos;
6. copia limpia de fragmentos;
7. comparacion entre fuente y resultado;
8. creacion desde cero;
9. navegacion por notas conectadas;
10. busqueda en muchas notas.

Primera apertura en lectura. La aplicacion recuerda localmente el ultimo modo de
cada documento.

## Estudio

Funciones valoradas:

- resaltado;
- preguntas y respuestas;
- ocultar contenido para practicar memoria;
- marcar contenido como entendido, dudoso o pendiente;
- resumir secciones;
- relacionar documentos;
- listas de conceptos;
- exportacion hacia otras herramientas.

Se descartan notas al margen como sistema independiente. Se prefiere usar
sintaxis Markdown u Obsidian comoda y portable. Para datos imposibles de expresar
limpiamente se aceptan archivos auxiliares. No se desea sintaxis propietaria.

## Compatibilidad con Obsidian

La direccion actual es ser un buen ciudadano dentro de bovedas existentes.

Esencial:

- wikilinks;
- backlinks;
- callouts;
- busqueda de carpeta;
- indice y estructura;
- navegacion rapida;
- uso de bovedas sin modificarlas inesperadamente.

Deseable:

- etiquetas;
- frontmatter;
- adjuntos;
- grafo;
- referencias a encabezados o bloques.

Un segundo cerebro completo seria otro producto o una expansion aprobada por
separado.

## Ayudas para IA

Prioridad:

- copiar el Markdown original de un bloque;
- dividir documentos largos en fragmentos apropiados;
- comparar versiones;
- generar un Markdown listo para adjuntar;
- preparar copias para Discord, correo y plataformas similares.

La estimacion aproximada de tokens es interesante si no agrega dependencias,
tamano ni complejidad significativos. De lo contrario queda para el futuro.

## Otros formatos

Ademas de Markdown, se desea reconocer texto inerte como:

- `.txt`;
- `.json`;
- `.yaml`;
- `.toml`;
- `.csv`;
- archivos de codigo.

Se muestran como texto y nunca se ejecutan. Visor MD no intenta convertirse en
un editor de programacion.

El guardado debe preservar partes no editadas, incluida sintaxis desconocida, en
la medida tecnicamente posible.

## Exportacion

Prioridades:

1. PDF visualmente fiel;
2. DOCX para universidad y trabajo;
3. copia preparada para Discord, correo y otras plataformas.

HTML autonomo, texto plano e impresion son deseables. PDF y DOCX pueden requerir
componentes adicionales. Debe medirse su impacto y evitar inflar el nucleo si
pueden aislarse.

## Tamano y rendimiento

Presupuesto acordado:

- menos de 6 MB es un resultado extraordinario;
- aproximadamente 7 MB es el objetivo normal;
- menos de 8 MB es el limite deseado;
- superar 8 MB requiere explicacion y aprobacion.

La seguridad, estabilidad y las funciones esenciales prevalecen sobre el numero.

Presupuestos historicos documentados:

- ventana visible en menos de 150 ms cuando resulte viable;
- primer contenido util en menos de 400 ms;
- trabajo por frame proporcional al contenido visible.

Claude midio aproximadamente 120 ms al ejecutar directamente el binario release.
El minuto observado por el propietario al usar `cargo run --release` correspondia
principalmente a compilacion con LTO, no a arranque de la aplicacion.

## Direccion visual

Decisiones recuperadas de los artifacts:

- paleta Papel y tinta;
- verdes distintivos;
- ventana sin borde;
- iconos suaves;
- contraste editorial;
- Newsreader para lectura;
- Sora para interfaz;
- JetBrains Mono para codigo;
- superficie plana;
- elevacion suave para overlays;
- animaciones breves y fluidas.

El tablero de movimiento recuperado propone, aproximadamente:

- cambio de pestana de 180 ms;
- transicion lectura y edicion de 180 ms;
- menus cercanos a 140 ms;
- rebote reservado para confirmacion de copia;
- el texto principal no se desplaza como adorno.

Los ZIP, PNG y MHTML de `Artifac opciones de diseño` son referencias, no codigo
de produccion.

## Seguridad de producto

La seguridad debe ser casi invisible durante el uso normal. Cuando bloquea algo,
se muestra un aviso discreto con una explicacion sencilla y detalles tecnicos
opcionales.

Configuracion avanzada permitida en principio:

- mostrar imagenes remotas despues de confirmacion;
- cargar imagenes locales relativas;
- abrir enlaces web tras un clic, sin confirmacion repetitiva;
- seguir manualmente enlaces a otros archivos;
- abrir manualmente rutas UNC;
- confiar temporalmente en una boveda;
- elevar limites blandos de tamano.

Garantias que no se pueden desactivar:

- nunca ejecutar scripts;
- nunca permitir que un documento cambie la seguridad;
- nunca enviar contenido silenciosamente;
- nunca ocultar conexiones;
- nunca interpretar eventos HTML.

Los hipervinculos deben distinguirse claramente, con preferencia del propietario
por el azul convencional. El color no basta contra phishing. Se debe mostrar el
destino real, controlar esquemas y delegar la navegacion al sistema.

Confiar en una boveda amplia acceso local, no permisos de ejecucion o red.

Elevar el limite de archivos no elimina topes absolutos. Archivos demasiado
costosos se muestran como fuente inerte o se rechazan si ni eso es seguro.

### Politica HTML acordada

Allowlist inicial:

- `br`;
- `kbd`;
- `mark`;
- `sub`;
- `sup`.

`details` y `summary` solo como componentes nativos si no complican demasiado el
renderer. HTML desconocido se muestra como texto. Se rechazan scripts, iframes,
estilos inline, eventos, formularios y recursos activos.

### Politica de rutas acordada

Un documento elegido explicitamente por el usuario puede estar en una ruta local
o UNC. El contenido de ese documento no puede obligar a la aplicacion a tocar
otros lugares.

No se siguen automaticamente:

- UNC internas;
- `file://`;
- rutas absolutas;
- recursos externos;
- enlaces a disco.

Las acciones manuales pueden existir bajo politicas claras y limites.

## Git al momento de la auditoria

Rama actual:

`codex/sprint-1-recovery`

HEAD, `main` y `origin/main`:

`090e9de Sprint 1: temas claro/oscuro y tipografia embebida`

La rama fue creada manualmente por el propietario despues de que Claude Code se
interrumpiera. Tambien se creo un backup completo fuera del repositorio.

Cambios tracked heredados:

- `assets/fonts/JetBrainsMono.ttf`;
- `assets/fonts/Newsreader.ttf`;
- `assets/fonts/Sora.ttf`;
- `src/main.rs`.

Sin seguimiento al momento de la auditoria:

- `assets/fonts/Newsreader-Italic.ttf`;
- `claude-working-tree.diff`;
- `claude-working-tree-status.txt`;
- `Artifac opciones de diseno/`.

No habia cambios staged. Los snapshots fueron creados por el propietario como
evidencia y no son parte normal del producto. El diff se genero desde PowerShell
y quedo en UTF-16.

## Secuencia reconstruida de Claude Code

### Trabajo consolidado en `090e9de`

- tema claro y oscuro;
- deteccion del tema del sistema;
- alternancia manual con `T`;
- fuentes reales de Google Fonts;
- licencia SIL OFL comprobada;
- subset latino con `fonttools`;
- fuentes embebidas en el binario;
- documentacion inicial de tipografia;
- medicion de tamano y arranque.

Claude informo que las tres fuentes iniciales quedaron en aproximadamente 409,8
KB y que el binario crecio de cerca de 2,14 MB a 2,54 MB.

### Trabajo local posterior

- representacion de bloques con tramos de estilo;
- negrita;
- cursiva;
- formatos anidados;
- codigo inline;
- tachado;
- decoraciones;
- listas;
- blockquotes;
- reglas horizontales;
- task lists;
- nuevos tests;
- limites para Markdown patologico.

Una prueba con aproximadamente 5.000 citas anidadas provoco stack overflow. Se
agregaron topes a dos recorridos recursivos y una prueba explicita. Claude obtuvo
17 tests verdes en un estado intermedio.

En la inspeccion visual posterior detecto:

- perdida de la tabla `STAT` durante el subset;
- ausencia de Newsreader Italic;
- decoraciones no dibujadas;
- glifos de casillas inexistentes en Newsreader.

Regenero las fuentes conservando `STAT`, agrego Newsreader Italic y decidio
dibujar casillas mediante `tiny-skia`.

La ultima accion declarada fue conectar `Marker` entre `Block`, cache de layout y
dibujo.

## Estado real del codigo heredado

El working tree inspeccionado no compilaba. Habia tres errores de tipos:

- un punto seguia entregando `Option<String>` donde el nuevo modelo esperaba
  `Marker`;
- dos pinceles ya no eran opcionales, pero el codigo intentaba usar `unwrap_or`.

El binario de tests existente pasaba 17 pruebas, pero habia sido compilado antes
de las ultimas modificaciones. Por tanto, no demuestra que el source actual
estuviera verde.

El diff de `src/main.rs` tenia aproximadamente 666 inserciones y 45 eliminaciones
sobre un archivo que alcanzo cerca de 1.662 lineas.

## Evaluacion de las decisiones heredadas

Decisiones valiosas que conviene conservar:

- Rust y UI nativa sin WebView;
- dependencias por defecto desactivadas cuando resulta practico;
- fuentes embebidas;
- tema y direccion tipografica;
- limites de profundidad;
- prueba especifica del stack overflow;
- casillas dibujadas sin agregar otra fuente;
- foco en binario pequeno;
- nucleo offline;
- threat model y presupuestos documentados.

Decisiones o estados que requieren revision:

- modelo `Block` y `Span` demasiado destructivo;
- perdida de rangos de fuente y semantica;
- `main.rs` monolitico;
- parsing sin la separacion prevista del hilo de UI;
- acceso directo a archivos en vez de VFS;
- virtualizacion con recorridos lineales por frame;
- alturas estimadas sin correccion suficiente;
- scroll con limites aproximados;
- falta de validacion DPI;
- accesibilidad, seleccion e IME aun no demostradas;
- proceso de fuentes no automatizado;
- documentacion adelantada respecto del codigo.

Resolver solo los tres errores de compilacion no vuelve correcta la arquitectura.
Las funciones recuperadas deben trasladarse a un modelo que preserve informacion
para edicion, seleccion, enlaces, anotaciones y guardado fiel.

## Arquitectura documentada y arquitectura implementada

La documentacion describe una direccion con:

- VFS;
- validador central;
- parser fuera del hilo de UI;
- limites de recursos;
- virtualizacion;
- componentes opcionales aislados;
- conectividad cerrada por defecto.

El prototipo implementado en la auditoria tenia:

- lectura directa mediante filesystem;
- parsing y preparacion sincronicos;
- modelo simplificado;
- rendering artesanal en un monolito;
- pruebas unitarias concentradas en parsing;
- sin CI, fuzzing, SBOM o matriz automatizada completa.

Esta diferencia no invalida Sprint 0. Significa que el prototipo probo viabilidad,
no que la arquitectura objetivo ya exista.

## Dependencias

Dependencias directas al momento de la auditoria:

- `comrak 0.54.0`, sin default features;
- `parley 0.11.1`;
- `softbuffer 0.4.8`;
- `swash 0.2.10`;
- `tiny-skia 0.12.0`, sin PNG;
- `winit 0.30.13`.

El grafo inspeccionado tenia aproximadamente 101 paquetes unicos activos en
Windows y 144 en Linux. El lockfile contenia alrededor de 271 paquetes porque
incluye alternativas por plataforma y resolucion.

`cargo audit` no encontro vulnerabilidades conocidas en ese momento. Advirtio
que `ttf-parser 0.25.1` estaba marcado como no mantenido mediante
`RUSTSEC-2026-0192`. Es deuda de suministro, no una vulnerabilidad demostrada.

Linux incorporaba `yeslogic-fontconfig-sys`, por lo que la afirmacion de ausencia
total de C solo estaba demostrada para el camino Windows medido.

No existian aun CI, `cargo deny`, fuzzing formal, SBOM ni `AGENTS.md`.

## Fuentes

La conversacion recuperada establece:

- origen en Google Fonts;
- licencia SIL OFL;
- uso de `fonttools` para subset;
- subset latino inicial;
- posterior conservacion de tabla `STAT`;
- adicion deliberada de Newsreader Italic;
- casillas dibujadas porque Newsreader no trae esos glifos.

Queda por reconstruir y automatizar el comando exacto de subset, hashes de
entrada, versiones de herramienta, notices y comprobacion de cobertura Unicode.

Las tres fuentes tracked y Newsreader Italic local sumaban aproximadamente 694
KB durante la auditoria.

## Rendering y rendimiento

Riesgos observados:

- busqueda lineal de slots visibles durante frames;
- estimaciones de altura que pueden divergir del layout real;
- calculo aproximado del maximo de scroll;
- ausencia de estrategia demostrada para DPI y zoom;
- seleccion, hit testing y accesibilidad aun no construidos;
- renderer artesanal con riesgo de crecer en complejidad.

El renderer propio sigue siendo una decision razonable por tamano, control y
superficie reducida. Debe mantenerse condicionada a superar temprano pruebas de
seleccion, teclado, IME, Unicode, lectores de pantalla y edicion.

## Parsing y modelo

El modelo heredado pierde informacion necesaria para:

- destino de enlaces;
- posiciones de fuente;
- idioma de codigo;
- celdas y alineacion de tablas;
- profundidad semantica de citas;
- identificadores;
- sincronizacion fuente y vista;
- anotaciones;
- edicion sin perdida.

El modelo canonico futuro debe preservar semantica y rangos. Los objetos de
layout y dibujo deben derivarse de el.

## Tests

Durante el tramo heredado Claude llego a 17 tests verdes. El working tree final
interrumpido no los compilaba.

Faltaban como minimo:

- corpus oficial CommonMark;
- corpus amplio GFM y Obsidian;
- casos de seguridad portados desde v1;
- fuzzing;
- property tests;
- pruebas de red;
- pruebas de VFS y rutas;
- round-trip;
- guardado atomico;
- regresion visual;
- accesibilidad;
- CI Windows y Linux;
- benchmarks automatizados.

La prueba de 5.000 citas es valiosa y debe conservarse, pero el criterio no puede
ser solo evitar panic. Debe comprobar limites, cancelacion, consumo y fallback.

## Version anterior

La v1 estaba limpia en `main` durante la auditoria y tenia una aplicacion Windows
basada en WebView2 con lectura, edicion, busqueda, seleccion, copia, menu
contextual y pruebas smoke amplias.

Se usa para recuperar:

- flujos de usuario;
- atajos;
- menu contextual;
- comportamiento de seleccion y copia;
- casos de seguridad;
- compatibilidad documental;
- expectativas de UX.

No se usa como arquitectura objetivo porque WebView2 contradice el nucleo nativo
y la superficie reducida de v2.

## Referencias externas

### Tinta

Al 25 de agosto de 2026, Tinta era mucho mas amplio que la fotografia descrita en
la investigacion temprana del repo. Declaraba edicion, diagramas nativos,
exportacion, anotaciones, referencias, pestanas y navegacion por carpetas con un
binario Windows pequeno.

Leccion util: una aplicacion nativa rica puede ser compacta.

Limite de la comparacion: usa C++ y servicios de Windows. Su tamano no se compara
directamente con un binario Rust multiplataforma y su arquitectura no es la de
Visor MD.

### ThisIs-Developer/Markdown-Viewer

Su interfaz y cantidad de funciones resultan sobrecargadas para la personalidad
de Visor MD. Es util como referencia de espacios de trabajo, documentacion y
limites visibles.

Su arquitectura web, PWA y Neutralino y sus funciones con CDN, APIs y servicios
remotos amplian mucho la superficie de ataque. No es referencia arquitectonica.

Las observaciones sobre competidores son una fotografia temporal y no deben
convertirse en reglas eternas.

## Informacion no reconstruida por completo

- comando exacto y reproducible de subset de cada fuente;
- hashes de las fuentes originales;
- notices finales de terceros;
- instante exacto del ultimo source con 17 tests verdes;
- validacion real en Linux del prototipo;
- intencion de variantes descartadas del artifact interactivo;
- resultado temprano de `cargo geiger` mencionado en documentos;
- comportamiento real de accesibilidad del renderer.

## Plan de recuperacion aprobado como propuesta

No iniciar implementacion hasta recibir autorizacion explicita posterior a este
traspaso documental.

### Etapa 1: recuperar y estabilizar

Trabajo:

1. registrar baseline, hashes, toolchain y diff;
2. clasificar cambios heredados por funcion;
3. recuperar compilacion para caracterizar, no para declarar terminacion;
4. definir un modelo documental no destructivo;
5. trasladar las funciones validas;
6. revisar y automatizar fuentes;
7. separar commits coherentes cuando se autorice.

Criterios:

- compilacion desde source actual;
- tests recien compilados;
- `Marker` completamente conectado;
- casillas probadas en parser, layout y dibujo;
- limites y fallback probados;
- fuentes verificadas;
- snapshots fuera del producto;
- ningun cambio heredado perdido sin decision explicita.

### Etapa 2: cerrar Sprint 1

Trabajo:

1. separar modulos minimos;
2. introducir modelo con rangos;
3. completar CommonMark declarado;
4. implementar allowlist HTML;
5. centralizar limites y modo seguro;
6. sacar trabajo pesado del camino de UI;
7. corregir virtualizacion, alturas, scroll, resize y DPI;
8. validar Unicode y fallback;
9. incorporar seleccion, copia, teclado y menu contextual;
10. establecer accesibilidad minima;
11. mantener tema y direccion visual;
12. portar casos valiosos de v1.

Criterios:

- documentos comunes renderizados correctamente;
- cada sintaxis declarada llega hasta rendering;
- semantica y rangos preservados;
- patologias sin bloqueo ni desborde;
- fallback visible;
- ausencia de red y accesos secundarios arbitrarios;
- seleccion, copia y teclado funcionales;
- scroll correcto con resize y DPI;
- trabajo por frame principalmente visible;
- tamano release medido;
- evidencia reproducible y documentacion sincronizada.

### Etapa 3: validaciones

Trabajo:

- CI Windows MSVC y Linux;
- format, clippy, tests y release;
- CommonMark oficial;
- casos patologicos y de v1;
- fuzzing;
- rutas, UNC y VFS;
- monitor de sockets;
- benchmarks versionados;
- auditoria de dependencias, licencias y unsafe;
- SBOM;
- matriz manual visual y de accesibilidad;
- actualizacion de threat model y ADR.

Criterios:

- gates verdes en Windows y Linux;
- cero red en apertura y render normales;
- contencion de archivos demostrada;
- fuzzing sin panic, bloqueo o consumo sin limite en la campaña acordada;
- SBOM reproducible;
- advisories resueltos o aceptados;
- benchmarks repetibles con umbrales;
- codigo, tests y threat model alineados.

### Etapa 4: trabajo posterior recomendado

Orden recomendado, sujeto a aprobacion al editar el roadmap:

1. lector completo;
2. editor basico y guardado seguro;
3. chrome profesional, pestanas y comandos;
4. workspace y segundo cerebro;
5. compatibilidad Obsidian y GitHub;
6. herramientas de estudio;
7. componentes opcionales pesados;
8. distribucion Windows y Linux.

Se recomienda adelantar el editor porque el guardado fiel y la relacion entre
fuente y render condicionan workspace, anotaciones y compatibilidad futura.

## Forma de colaboracion acordada

Antes de un plan aprobado se busca interaccion amplia y preguntas de producto.
Despues:

- autonomia dentro del bloque;
- consulta ante producto, seguridad, diseno o alcance;
- alternativas antes de arquitectura dificil de revertir;
- mensajes breves durante el trabajo;
- cierre claro;
- explicaciones educativas;
- modo autonomo hasta bloqueante, ambiguedad o QA manual valioso;
- desacuerdo directo, con hechos, riesgos y alternativas.

El README debe hacer explicito de forma sobria el uso de IA y la direccion humana
del producto. No convertir comentarios, commits o codigo en propaganda o
metadiscurso sobre agentes.

## Autorización posterior

El 25 de agosto de 2026 el propietario aprobó iniciar el trabajo en una rama
separada y autorizó editar o crear lo necesario dentro de los principios
acordados.

La primera instrucción fue actualizar y publicar la documentación antes de
continuar el código. Esta nota describe ese estado histórico. Tras la validación
y autorización posterior del propietario, el antiguo `main` se preservó en
`archive/claude-pre-codex` y la historia lineal validada se avanzó a `main` sin
reescritura. Consultar `status.md` para la rama principal actual.
