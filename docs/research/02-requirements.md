# Requisitos de la v2

Ninguno de estos sale de la nada: cada uno señala de qué proyecto salió y
por qué. Donde hay un número, es una meta a validar con un prototipo, no
una promesa ya cumplida.

## Rendimiento y peso

- **Arranque**: bajar de los 3-4 s actuales de la v1 a menos de 1 s hasta la
  primera pintura del documento, para el caso común. No se persigue el
  <100 ms de Tinta a costa de perder motor web — ver la discusión de
  compromiso en `03` y `04`.
- **Peso**: no superar sensiblemente los ~14 MB del zip de la v1. Cualquier
  arquitectura que exija empaquetar un Chromium propio (estilo Electron)
  queda descartada de entrada por esto — es el motivo estructural por el
  que Moji, siendo tan afín en seguridad, no es un modelo a copiar en peso.
- **Documentos grandes**: estrategia explícita para archivos de decenas de
  MB — parseo fuera del hilo de interfaz, o virtualización del renderizado.
  La v1 no tiene ninguna; Moji sí. *(Origen: Moji.)*

## Seguridad — línea base: igualar la v1, no bajarla nunca

Las cuatro propiedades de `docs/frontera-de-seguridad.md` de la v1 se
mantienen intactas y no negociables:

1. Abrir un documento no genera ninguna petición de red.
2. Nada del documento se ejecuta.
3. Un documento no puede leer archivos que no le corresponden.
4. Un documento no puede disfrazarse de la aplicación.

## Seguridad — lo que se agrega en la v2

- **Separación de proceso**: el proceso con acceso a disco y al registro de
  Windows no es el mismo proceso que renderiza el documento. Comunicados
  por un contrato de mensajes tipado y angosto, no por un puente que
  refleja métodos de Python directamente a JavaScript. *(Origen: Moji.)*
- **Principio de red extendido a diagramas**: si en el futuro se suma
  soporte para motores de diagramas más allá de Mermaid, solo se suman los
  que corran 100% local. Ninguno que mande la fuente del diagrama a un
  servicio externo, ni con aviso al usuario. *(Origen: ThisIs-Developer,
  por contraste — es justo lo que ellos sí permiten.)*
- **Un solo punto de sanitización**: se formaliza como regla arquitectónica
  que ningún renderizador, plugin o motor de diagramas inserta contenido en
  el DOM salvo pasando por el único paso de sanitización auditado. *(Origen:
  ByteMD, por el patrón correcto; EasyMDE, por el contraejemplo de hacerlo
  opcional.)*
- **Sin interruptor para apagar la sanitización**: como en la v1, la
  configuración avanzada puede ampliar a qué recursos accede un documento,
  nunca qué puede ejecutar. No se revisa esta decisión.

## Producto — usable a diario por profesionales de IT

- **Workspace persistente**: carpetas, documentos recientes y favoritos que
  sobreviven entre sesiones, no solo pestañas que se resetean al cerrar.
  *(Origen: ThisIs-Developer.)*
- **Edición estructural**: renombrar un encabezado actualiza los enlaces
  internos que apuntan a él; pegar una imagen del portapapeles la guarda
  como archivo y arma el enlace; autocompletado de enlaces a otros
  documentos del mismo workspace. *(Origen: idea-multimarkdown.)*
- **Diagramas locales**: Mermaid ya cubierto en la v1; evaluar sumar
  Markmap (mapas mentales desde encabezados) por ser igual de viable en
  local. PlantUML/Graphviz/D2 quedan fuera salvo que exista un motor que
  corra embebido sin red — no se agregan "porque los tiene la competencia".

## Lo que se preserva de la v1 sin cambios

- Licencia GPLv3 y desarrollo abierto.
- Se fija como programa predeterminado de Windows para `.md`.
- Distribución portable sin instalador obligatorio.
- Disciplina de pruebas: cualquier arquitectura nueva llega con su propia
  suite, siguiendo el mismo criterio de la v1 — afirmar propiedades
  ("no salió ninguna petición a la red"), no solo "no se cayó".

## Lo que queda fuera a propósito

- Colaboración en vivo / compartir por enlace (ThisIs-Developer lo tiene;
  rompe la propiedad 1 de la lista de seguridad si no es estrictamente
  opt-in y aislado, y no es el problema que Visor MD existe para resolver).
- Un sistema de plugins de terceros instalables en tiempo de ejecución —
  eso multiplica la superficie de confianza exactamente en el punto que la
  v2 quiere angostar. La extensibilidad interna (formalizar el patrón de
  ByteMD) no implica abrir la puerta a plugins de terceros sin auditar.
