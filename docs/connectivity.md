# Conectividad y confianza

## Principio

Visor MD funciona offline. Abrir, leer, editar, buscar y navegar una bóveda local
no requieren cuenta ni Internet.

Conectividad tiene dos significados distintos:

1. conexión con otras herramientas mediante archivos locales;
2. conexión de red opcional y explícita para una acción concreta.

No deben mezclarse.

## Estado normal

Durante apertura y render:

- no hay cliente HTTP activo;
- no hay telemetría;
- no hay actualización silenciosa;
- no se descargan fuentes;
- no se precargan enlaces;
- las imágenes remotas quedan bloqueadas;
- el contenido no puede solicitar red.

La ausencia de red se comprueba observando sockets en tests.

## Enlaces web

Un clic explícito sobre `http` o `https` delega la apertura al navegador del
sistema. Visor MD no representa la página.

Antes del clic debe ser posible conocer el destino real. El hipervínculo se
distingue en azul, tiene estados de foco y no usa texto de seguridad controlado
por el documento.

El lector actual muestra el destino declarado en el título de ventana al pasar
por encima del enlace y cambia el cursor. Esto no resuelve, abre ni consulta el
destino: permite detectar un enlace engañoso antes de que exista una acción de
apertura.

## Imágenes remotas

Bloqueadas por defecto. El usuario puede autorizar una imagen o un alcance
limitado desde configuración avanzada.

La capacidad futura de descarga debe aislarse del núcleo y no enviar cookies,
credenciales, contenido, rutas o referrer. Se aplican timeout, límites, control
de redirects y bloqueo de destinos privados.

Mostrar una imagen remota revela al servidor al menos la IP de salida. La UI debe
explicarlo en lenguaje natural.

## Archivos locales

El documento principal puede abrirse desde cualquier ubicación elegida
explícitamente, incluida UNC manual.

Los recursos relativos y enlaces locales pasan por VFS. Un documento no obtiene
permiso sobre todo el disco por haber sido abierto.

La VFS resuelve cada enlace Markdown relativo desde la carpeta de la nota que lo
declara, no desde la raíz de la bóveda ni desde el proceso. Tanto la nota base
como el destino canonicalizado deben permanecer dentro de la raíz concedida;
una nota abierta fuera de ella no hereda acceso lateral a la bóveda activa.

## Obsidian

La integración usa el formato común del filesystem:

- `.md`;
- carpetas;
- wikilinks;
- callouts;
- frontmatter y etiquetas elegidos;
- adjuntos locales permitidos.

No requiere API, cuenta o plugin de Obsidian. Visor MD abre una bóveda existente
sin migrarla.

Confiar temporalmente en la bóveda permite indexar y navegar dentro de su raíz.
No habilita scripts, red o acceso externo.

## GitHub y Git

La compatibilidad principal ocurre mediante archivos Markdown que Git puede
versionar sin ruido.

Visor MD no necesita token de GitHub para leer o editar un checkout local. Abrir
una página de GitHub usa el navegador tras clic explícito. Clonar, pull, push y
credenciales quedan fuera del núcleo salvo una decisión futura separada.

El guardado debe evitar reformatos masivos que oculten el cambio real en un diff.

## Servicios de sincronización

OneDrive, Dropbox, Syncthing, Git u otras herramientas pueden sincronizar la
carpeta por fuera de Visor MD. La app las ve como filesystem local.

Debe tolerar:

- archivos que cambian externamente;
- archivos temporalmente no disponibles;
- conflictos y copias duplicadas;
- reemplazos de identidad;
- latencia de rutas UNC o proveedores cloud.

No afirmar que los datos permanecen en el equipo si el usuario eligió una carpeta
sincronizada. Visor MD no realiza el envío, pero el proveedor puede hacerlo.

## Sidecars

Los sidecars viajan junto al documento solo cuando el usuario y su herramienta de
sincronización incluyen esos archivos. Su formato debe ser visible, documentado y
versionable. No contienen tokens o secretos.

## Componentes opcionales

Exportadores o renderers futuros no heredan red automáticamente. Cada componente
declara:

- si toca disco;
- si toca red;
- qué datos recibe;
- qué límites aplica;
- cómo se cancela;
- qué evidencia lo prueba.

No hay componente de IA previsto para Visor MD.

## Matriz de acciones

| Acción | Red | Disco secundario | Consentimiento |
| --- | --- | --- | --- |
| Abrir Markdown | No | No automático | Archivo elegido |
| Render normal | No | Solo recursos locales permitidos | Política local |
| Abrir enlace web | Navegador externo | No | Clic |
| Mostrar imagen remota | Componente aislado | No | Confirmación |
| Abrir enlace local | No | VFS | Clic y política |
| Abrir UNC principal | Posible red de filesystem | Archivo elegido | Acción manual |
| Indexar bóveda | No | Dentro de raíz | Confianza delimitada |
| Exportar | No por defecto | Destino elegido | Acción explícita |

## Lo que nunca cambia

- un documento no activa conectividad;
- confiar en una carpeta no habilita Internet;
- no hay telemetría oculta;
- no se envía contenido a IA;
- toda conexión opcional es visible, limitada y revocable.
