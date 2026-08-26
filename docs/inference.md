# Inferencia local: investigación histórica

## Estado

Esta idea fue reemplazada por el ADR-21. Visor MD no incorpora un modelo de IA
local o remoto.

Se conserva el documento porque explica una alternativa evaluada y por qué se
descartó. No pertenece al roadmap activo.

## Problema que intentaba resolver

La propuesta original buscaba generar preguntas, resúmenes o tarjetas de estudio
sin enviar documentos a un servicio remoto.

Se consideraron dos caminos:

- conectar con un runtime local ya instalado;
- ofrecer un componente descargable separado del binario principal.

Ambos evitaban inflar directamente el núcleo, pero agregaban distribución,
compatibilidad, permisos, modelos grandes y una nueva superficie de entrada.

## Motivo de la decisión actual

El usuario principal ya trabaja con herramientas de IA. Visor MD aporta más valor
si prepara y conserva buenos documentos que si duplica esas herramientas.

La decisión actual permite:

- copiar Markdown estructurado;
- fragmentar documentos largos;
- comparar versiones;
- preparar archivos para adjuntar;
- estimar longitud si resulta casi gratuito.

No se ejecuta inferencia y no se envía contenido.

## Condición para reabrir

Solo se reevalúa mediante una decisión de producto explícita, con threat model,
privacidad, distribución, tamaño, mantenimiento y una razón que no pueda
resolverse mejor mediante interoperabilidad.
