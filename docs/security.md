# Seguridad

## Propósito

Visor MD abre archivos que pueden provenir de Internet, repositorios, compañeros
o una IA. Aunque Markdown parezca texto, puede contener rutas, enlaces, imágenes,
HTML y estructuras diseñadas para explotar parsers o agotar recursos.

La postura es tratar cada documento como entrada hostil sin volver alarmante el
uso cotidiano.

## Propiedades de seguridad

Durante apertura y render normales debe ser cierto que:

1. el documento no ejecuta código;
2. el documento no inicia conexiones;
3. el documento no lee archivos secundarios fuera de una política explícita;
4. el documento no cambia configuración;
5. entradas patológicas tienen límites y cancelación;
6. guardar no corrompe ni reescribe contenido ajeno al cambio;
7. toda excepción sensible requiere una acción consciente;
8. los componentes conocen solo los permisos que necesitan.

Estas son propiedades, no intenciones. Deben existir tests que observen red,
filesystem, tiempo, memoria y salida.

## Conceptos importantes

### Superficie de ataque

Es el conjunto de lugares por los que una entrada puede alcanzar código y causar
un efecto. Un parser, un decodificador de imágenes, un enlace y una dependencia
nativa agregan superficies diferentes.

Reducir superficie no significa eliminar funciones útiles. Significa evitar
motores generales cuando solo se necesita una capacidad pequeña y poner límites
claros alrededor de cada entrada.

### Frontera de confianza

Es el punto donde datos menos confiables entran a una zona con más permisos.
Ejemplo: un path escrito dentro del Markdown cruza una frontera antes de llegar al
filesystem. El VFS controla esa frontera.

### Defensa en profundidad

Consiste en usar varias capas que no dependan de un único filtro. Una imagen
local, por ejemplo, necesita contención de ruta, límite de bytes, validación de
formato, límite de dimensiones y presupuesto de memoria.

### Denegación de servicio

Un archivo puede intentar consumir pila, CPU o memoria hasta bloquear la app.
Las 5.000 citas anidadas que causaron stack overflow son un ejemplo. Rust evita
muchos errores de memoria, pero no evita automáticamente trabajo ilimitado.

## Garantías no configurables

Nunca se permite:

- ejecutar JavaScript, scripts o macros procedentes de documentos;
- interpretar event handlers HTML;
- incorporar iframes o formularios activos;
- permitir que contenido cambie preferencias de seguridad;
- enviar documentos o fragmentos silenciosamente;
- ocultar una conexión;
- desactivar el validador de nodos;
- cargar plugins con código arbitrario dentro del núcleo.

Un modo avanzado no elimina estas garantías.

## Markdown y HTML

`comrak` transforma Markdown en AST. Después, el modelo propio y el validador
deciden qué comportamiento existe.

Allowlist HTML inicial:

- `br`;
- `kbd`;
- `mark`;
- `sub`;
- `sup`.

Estado actual: `br`, `kbd`, `mark`, `sub` y `sup` tienen representación nativa
en el modelo, layout y dibujo. La comparación es cerrada y no admite atributos:
una etiqueta permitida con cualquier atributo permanece como fuente inerte. Una
apertura sin cierre tampoco recibe semántica nativa; se hace visible junto a su
contenido para no esconder Markdown defectuoso.

`details` y `summary` solo pueden existir como controles nativos propios y
simples. No se crea DOM.

HTML desconocido se muestra como texto inerte o fuente escapada. No se aceptan
atributos de estilo, eventos, scripts, iframes, formularios o recursos activos.

La allowlist se prueba por tipo de nodo y atributo. Escapar una cadena no alcanza
si otra ruta puede construir el mismo comportamiento.

## Límites de recursos

Se definirán límites blandos y absolutos para:

- bytes de archivo;
- profundidad;
- cantidad de nodos;
- longitud de línea;
- tamaño de texto producido;
- tiempo de parsing y layout;
- memoria de cache;
- dimensiones y bytes de imágenes;
- cantidad de archivos indexados;
- tamaño y duración de exportación.

El usuario avanzado puede aumentar límites blandos, especialmente el tamaño del
archivo. Los límites absolutos y la cancelación permanecen.

Al superar un límite de render enriquecido:

1. cancelar trabajo pendiente;
2. liberar estado parcial;
3. abrir fuente inerte cuando sea seguro;
4. mostrar un aviso discreto;
5. ofrecer detalles técnicos opcionales.

Rechazar completamente solo si ni la vista inerte puede construirse dentro del
presupuesto.

El lector actual aplica un tope de 16 KiB por línea al render enriquecido. Una
línea mayor no llega al parser ni al layout como un bloque único: se muestra como
texto inerte, dividido en tramos UTF-8 válidos que conservan los rangos de la
fuente. Es una defensa de disponibilidad, no una reescritura del archivo.

## Filesystem y VFS

### Archivo principal

Un archivo elegido explícitamente puede estar en disco local o UNC. La app lo
trata como entrada, no como autorización para explorar su entorno.

La implementación actual abre el archivo principal una vez, consulta sus
metadatos desde ese mismo handle y limita su lectura a 16 MiB de UTF-8 válido.
Detecta BOM UTF-8 y el patrón de EOL (`LF`, `CRLF` o mixto) sin normalizar el
texto. Es un límite operativo temporal, no una afirmación de que 16 MiB sea el
techo final del producto: una preferencia avanzada podrá elevar el límite blando
solo cuando el modo seguro y sus presupuestos de memoria estén medidos. Un
archivo que lo supera hoy se rechaza con una explicación, no se parsea
parcialmente.

Esto reduce una carrera TOCTOU común: comprobar por ruta y luego leer esa ruta
podría validar un archivo y abrir otro si un proceso local lo reemplaza entre
ambas operaciones. La contención de recursos secundarios, symlinks y junctions
todavía no existe porque Visor MD aún no abre recursos secundarios.

La sesión retiene además tamaño y fecha de modificación observados al abrir. Es
una señal de conflicto previa al guardado, no una prueba criptográfica ni una
identidad de handle: un guardado seguro volverá a validar el destino y hará el
reemplazo atómico en la misma frontera de filesystem.

Solo las extensiones Markdown (`.md`, `.markdown`, `.mdown`, `.mkdn`) pasan al
parser. `.txt`, JSON, YAML, TOML, CSV y código se conservan como texto inerte:
la aplicación no intenta ejecutar, compilar ni tratar su sintaxis como una
capacidad.

### Recursos secundarios

El contenido no puede cargar automáticamente:

- rutas UNC;
- rutas de dispositivo;
- `file://`;
- rutas absolutas;
- streams alternativos NTFS;
- destinos fuera del espacio permitido;
- symlinks o junctions que escapen de ese espacio.

El hover de enlaces clasifica y etiqueta destinos web, correo, archivos
relativos y formatos bloqueados. Esta clasificación no abre ni resuelve nada;
una futura acción de clic deberá volver a aplicar la política correspondiente.

Las rutas relativas locales pueden resolverse mediante VFS y límites. Seguir un
enlace a otro archivo requiere una acción explícita.

## Portapapeles

El portapapeles es una frontera entre Visor MD y otras aplicaciones. `Ctrl+C`
copia la selección visible y `Ctrl+Shift+C` copia la fuente Markdown de los
bloques seleccionados. `Ctrl+V` y la acción visible de pegar pueden leer texto
solamente en ese instante y colocarlo en el editor fuente activo. No existe
observador, lectura en segundo plano, historial, acceso a imágenes ni envío de
su contenido.

La copia de fuente se limita a bloques completos porque una selección visual no
equivale necesariamente a los bytes de Markdown que la produjeron. Esto evita
fabricar sintaxis incorrecta. Un error de portapapeles se comunica sin mostrar
ni registrar contenido del documento.

### TOCTOU

TOCTOU significa comprobar algo y usarlo después, cuando pudo cambiar. Un atacante
podría reemplazar un archivo o symlink entre la validación y la lectura.

Cuando el riesgo lo justifique, VFS debe abrir y validar identidad sobre el mismo
handle o volver a comprobar identidad antes del uso. La implementación depende de
la plataforma y necesita tests específicos.

## Workspace y confianza temporal

Confiar en una bóveda permite acceder a archivos locales dentro de una raíz
delimitada para índice, navegación y recursos relativos.

La confianza:

- tiene alcance y duración visibles;
- se puede revocar;
- no se hereda a rutas externas;
- no activa red;
- no ejecuta contenido;
- no elimina límites absolutos;
- no convierte archivos en inocuos.

Un índice es derivado y regenerable. No almacena secretos innecesarios ni se
convierte en autoridad sobre el filesystem.

## Enlaces y phishing

Una acción explícita sobre `http`, `https` o `mailto:` puede delegarse al
sistema sin una confirmación repetitiva. El lector actual usa Enter sobre un
enlace enfocado con Tab. La delegación no usa shell y solo recibe el destino ya
clasificado; no hay prefetch ni navegación embebida.

Controles:

- apariencia inequívoca de hipervínculo, con azul convencional;
- destino real visible antes de abrir;
- dominio Unicode normalizado o explicado cuando pueda confundir;
- esquema permitido explícitamente;
- nada de prefetch;
- nada de navegación embebida;
- ninguna URL controla texto de seguridad de la app.

Phishing significa engañar al usuario para que crea que abre un destino distinto.
El color azul ayuda a reconocer un enlace, pero no demuestra que su destino sea
legítimo.

## Imágenes locales

Antes de decodificar:

1. VFS resuelve y contiene la ruta;
2. se limita tamaño en bytes;
3. se identifica formato real;
4. se leen dimensiones con presupuesto;
5. se calcula memoria descomprimida;
6. se cancela si supera límites.

Una imagen comprimida pequeña puede expandirse a cientos de MB. El límite debe
considerar dimensiones y memoria, no solo tamaño de archivo.

## Imágenes remotas

Bloqueadas por defecto. Un placeholder discreto informa el bloqueo.

Después de consentimiento, la implementación deberá aislar la capacidad de red y
aplicar:

- `https` por defecto;
- límites de redirecciones;
- bloqueo de esquemas no previstos;
- política contra destinos locales y redes privadas;
- timeout;
- límite de bytes;
- tipo y dimensiones;
- sin cookies, credenciales o referrer del documento;
- cache explícito y borrable;
- indicación de que el servidor conocerá la IP pública.

Bloquear redes privadas evita que una URL maliciosa use la PC del usuario para
consultar routers o servicios internos. Este riesgo se conoce como SSRF cuando un
componente realiza solicitudes a destinos elegidos por un atacante.

## Edición y guardado

Amenazas principales:

- archivo truncado por fallo;
- escritura en destino equivocado;
- pérdida de sintaxis desconocida;
- conflicto con cambios externos;
- reemplazo de archivo entre validación y guardado;
- permisos alterados;
- recuperación presentada como guardado real.

Controles:

- parches sobre rangos;
- el buffer fuente es la autoridad; el AST y la vista nunca reescriben Markdown;
- preservación de bytes no editados;
- archivo temporal en el mismo filesystem;
- reemplazo atómico;
- identidad y revisión;
- estado sucio visible;
- diálogo de conflicto;
- backup o recuperación separado cuando corresponda;
- tests con fallos simulados.

No hay autoguardado por defecto.

La primera implementación de Guardar compara además los bytes completos de la
versión abierta con el destino justo antes del reemplazo atómico. Esto detecta
ediciones externas que una fecha de modificación o un tamaño iguales podrían
ocultar. La comprobación reduce el riesgo TOCTOU, pero no elimina una carrera
del filesystem entre esa lectura y el reemplazo; la identidad específica de
handle y los tests por plataforma siguen siendo trabajo pendiente.

El historial de edición conserva solo los fragmentos retirados e insertados y
tiene un presupuesto de 4 MiB por documento. Si se llena, se descartan pasos de
undo más antiguos, no el texto actual ni cambios pendientes. Es una defensa de
memoria: un historial ilimitado permitiría que un documento grande o una sesión
de pegados agotara recursos aunque el archivo de entrada estuviera limitado.

## Sidecars

Un sidecar contiene información adicional junto al documento. Se usa solo cuando
Markdown no expresa bien el estado, por ejemplo fechas de repaso.

Debe tener:

- formato versionado;
- vínculo verificable con el documento;
- escritura atómica;
- recuperación ante corrupción;
- comportamiento seguro si la fuente cambia;
- exclusión clara de secretos innecesarios.

No aplicar una anotación a otro fragmento solo porque ocupa el mismo rango después
de una edición.

## Dependencias y supply chain

Supply chain es la cadena de herramientas y bibliotecas que termina dentro del
producto. Una dependencia comprometida puede afectar el binario aunque nuestro
código sea seguro.

Antes de incorporar una dependencia:

- revisar mantenimiento y advisories;
- revisar licencia;
- inspeccionar features transitivas;
- identificar C, C++ y `unsafe`;
- buscar capacidades de red o filesystem;
- medir tamaño;
- fijar versión y registrar decisión cuando el riesgo sea relevante.

Entregables:

- `Cargo.lock`;
- `cargo audit`;
- política `cargo deny`;
- SBOM;
- notices de terceros;
- inventario de `unsafe` y código nativo;
- proceso reproducible de fuentes.

La auditoría repetida el 26 de agosto de 2026 no encontró vulnerabilidades
conocidas. `ttf-parser 0.25.1` sigue marcado como no mantenido y entra por las
decoraciones Wayland de `winit`, no por el camino Windows medido. El análisis y
las alternativas están en [`dependencies.md`](dependencies.md).

## Rust y memoria

Rust reduce use-after-free, double free y otras clases comunes de corrupción. No
evita automáticamente:

- lógica de autorización incorrecta;
- path traversal;
- agotamiento de memoria;
- algoritmos lentos;
- dependencias vulnerables;
- `unsafe` incorrecto;
- errores en bibliotecas nativas o del sistema.

Por eso la elección de Rust es una defensa importante, no una certificación.

## Privacidad y registros

No hay telemetría. Los logs de diagnóstico:

- no incluyen contenido completo por defecto;
- evitan rutas privadas cuando no son necesarias;
- distinguen errores técnicos de datos del usuario;
- se generan localmente;
- se comparten solo por acción explícita.

## Configuración avanzada

| Opción | Default | Alcance recomendado | Límite permanente |
| --- | --- | --- | --- |
| Imágenes remotas | Bloqueadas | Recurso o sesión | Sin credenciales ni red silenciosa |
| Imágenes locales relativas | Permitidas por política | Documento o bóveda | VFS y límites |
| Enlaces web | Clic explícito | Global | Esquemas permitidos y destino visible |
| Enlaces a archivos | Acción explícita | Documento o bóveda | Contención y confirmación externa |
| UNC principal | Apertura manual | Archivo | Sin recursos UNC secundarios |
| Bóveda confiable | No confiable | Temporal o persistencia explícita | Sin ejecución ni red |
| Archivo grande | Límite normal | Sesión o preferencia | Techo absoluto y fallback |

La UI debe explicar riesgo, alcance y duración sin usar patrones engañosos.

## Verificación

Consultar [`testing.md`](testing.md) y [`test-matrix.md`](test-matrix.md).

Pruebas críticas:

- sockets observados durante apertura;
- traversal con variantes de plataforma;
- symlinks y junctions;
- UNC explícita y secundaria;
- HTML conocido y desconocido;
- archivos profundos, anchos y grandes;
- imágenes comprimidas maliciosas;
- guardado interrumpido;
- conflicto externo;
- dependencias y licencias;
- phishing visual y teclado.

## Riesgo residual

No se protege contra un sistema operativo ya comprometido, un administrador local
malicioso o una biblioteca del sistema alterada. Tampoco existe seguridad
absoluta frente a vulnerabilidades desconocidas.

El objetivo es minimizar privilegios, limitar entradas, aislar capacidades y
detectar regresiones con evidencia mantenible.
