# Futuro

Este documento contiene ideas válidas fuera del plan activo. No son promesas de
v2.0. Una idea entra al roadmap solo con caso de uso, coste, threat model, diseño
y criterio de salida.

## Principios

- El núcleo permanece pequeño y offline.
- Nada ejecuta contenido de documentos.
- Lo pesado se aísla o se ofrece como componente opcional.
- No se agrega una función solo para igualar a un competidor.
- Cada release debe ser un punto de parada útil.

## Candidatos

### Edición en vivo

Editar directamente sobre el documento renderizado. Requiere selección, cursor,
IME, actualizaciones incrementales y round-trip ya resueltos. Se evalúa después
del editor fuente y la vista dividida.

### Mermaid nativo

Diagramas sin JavaScript ni servicio remoto. Empezaría por familias de alto valor
como flowchart y secuencia. Debe medir tamaño, complejidad y corpus adversarial.

### Matemática

Componente opcional sin motor web. La compatibilidad y la accesibilidad importan
más que reconocer muchas fórmulas parcialmente.

### Corrector ortográfico

Diccionarios descargables por idioma para no inflar binario y memoria de quienes
no lo usan. Debe funcionar offline y no enviar texto.

### Grafo de notas

Visualización secundaria. Solo entra si usuarios reales demuestran que mejora la
navegación más que backlinks y búsqueda.

### Referencias de bloque creadas por Visor MD

Requieren identidad estable de fragmentos y una política compatible con Obsidian.
Navegar referencias existentes puede llegar antes.

### Plugins declarativos

Extensiones de tema, sintaxis o comandos descritas como datos. No se acepta
código arbitrario dentro del proceso. Cualquier sistema de plugins necesita un
threat model independiente.

### macOS

La arquitectura evita dependencias innecesarias de Windows, pero macOS no bloquea
v2.0. Se evalúa cuando Windows y Linux tengan releases mantenibles.

### Espacio cifrado

Una bóveda cifrada abre preguntas sobre índice, nombres, sidecars, backups,
recuperación y contraseña olvidada. No se diseña hasta poder ofrecer una solución
criptográfica completa y auditable.

### Actualizaciones firmadas

Opcionales, visibles y sin telemetría. Requieren firma, protección contra
rollback, canal de publicación y recuperación segura.

## Investigación de seguridad no prioritaria

Estas dos líneas quedan pendientes, sin urgencia, fuera del roadmap activo. No
son requisitos ni gates de v2.0 y no implican un compromiso de implementación.
La revisión futura parte de las políticas de [`security.md`](security.md), el
[`threat-model.md`](threat-model.md) y los criterios de UX de
[`design.md`](design.md); no amplía sus garantías ni sustituye los controles y
verificaciones de seguridad ya exigidos para v2.0.

### Teclado virtual compatible con Visor MD

Investigar si un teclado virtual aporta utilidad real en los flujos de escritura
de Visor MD antes de decidir si se crea y se agrega. La evaluación debe cubrir:

- **Caso de uso y alternativas:** qué tarea concreta mejora, para quién y si el
  teclado en pantalla del sistema u otra solución existente ya la resuelve.
- **Threat model y limitaciones:** identificar el atacante, sus permisos, el
  punto de captura y qué protección sería demostrable. Distinguir captura de
  pulsaciones, eventos sintéticos, pantalla, accesibilidad y lectura de memoria;
  evitar tratar todas esas vías como si fueran el mismo problema. No prometer
  protección general contra keyloggers ni frente a un sistema comprometido,
  conforme a las exclusiones del modelo vigente. La ofuscación interna no debe
  presentarse como protección de una entrada ya observada antes de llegar a la
  app.
- **Compatibilidad y UX:** edición fuente/dividida, foco, selección, atajos,
  undo/redo, Unicode, distribuciones de teclado, IME y tecnologías de asistencia
  en Windows y Linux. Valorar velocidad de escritura, errores, fatiga y claridad
  de las limitaciones sin sobrecargar la interfaz editorial.
- **Coste y evidencia:** superficie de ataque, dependencias, tamaño, recursos y
  mantenimiento frente al beneficio. Definir pruebas por vía de captura y
  plataforma antes de atribuirle una mejora de seguridad.

La salida será una decisión razonada: descartar, mantener en investigación o
proponer una implementación acotada con evidencia, riesgo residual y criterio
de salida. Una utilidad de accesibilidad o comodidad debe describirse como tal,
sin atribuirle garantías de confidencialidad no demostradas.

### Sesión futura de revisión de seguridad pragmática

Reservar una sesión para explorar mejores alternativas de seguridad sin
overengineering: partir de riesgos y flujos concretos, reutilizar controles
existentes y preferir la solución mantenible más pequeña. No es una auditoría
realizada ni una nueva etapa obligatoria del roadmap.

Agenda propuesta:

1. Recorrer apertura de documentos hostiles, edición, portapapeles, recuperación
   local, guardado, bóvedas, enlaces, exportación y distribución. Separar las
   propiedades implementadas de las previstas y de las todavía no verificadas.
2. Contrastar cada escenario con OWASP y buenas prácticas aplicables. Seleccionar
   las guías y controles pertinentes para una app nativa de escritorio, registrar
   fuente, versión o fecha y justificar lo no aplicable; no trasladar listas web
   o móviles completas ni presentar la revisión como una certificación.
3. Identificar dónde Visor MD o una app de su ecosistema podría aportar más
   seguridad y control sobre los dispositivos del usuario: permisos acotados,
   datos locales, decisiones visibles y transferencias explícitas entre apps.
   Separar lo que corresponde al núcleo, a un componente opcional y al sistema
   operativo; conservar el núcleo pequeño y offline, sin ejecución de documentos
   ni ampliaciones implícitas de privilegios o red. Evaluar también cuándo basta
   con una herramienta existente o una explicación más clara.
4. Comparar alternativas por amenaza mitigada, evidencia reproducible, riesgo
   residual, coste de UX, accesibilidad, rendimiento y mantenimiento. Diferenciar
   mitigaciones reales de security theater: controles que aparentan protección
   sin demostrar una reducción del riesgo concreto o que inducen falsa confianza.

Como resultado, dejar una lista breve de propuestas con escenario, control,
prueba, límites y decisión: descartar, investigar o proponer al roadmap. No
implementar infraestructura preventiva ni ampliar el producto durante esta
sesión; cualquier propuesta seguirá los criterios de entrada al roadmap y las
políticas de seguridad vigentes.

## Ideas descartadas actualmente

- IA propia o chatbot;
- motores remotos de diagramas;
- ejecución de HTML o JavaScript;
- colaboración online en tiempo real;
- outliner como modelo de escritura principal;
- plugins con código arbitrario;
- autoguardado silencioso como única protección.

## Riesgo de sostenibilidad

El proyecto tiene un mantenedor principal. La mejor defensa contra abandono es un
alcance disciplinado, automatización y etapas que produzcan software útil. Una
función futura no puede volver frágil lo que ya funciona.
