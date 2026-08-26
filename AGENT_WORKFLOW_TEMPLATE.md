# Plantilla de instrucciones para agentes de desarrollo

Copiar este archivo al repositorio nuevo y adaptarlo antes de empezar cambios
grandes. Renombrarlo a `AGENTS.md` solo cuando sus reglas ya representen el
proyecto; mientras sea plantilla, conservar este nombre para que no se aplique
accidentalmente como instrucción activa.

## Propósito y alcance

`[NOMBRE_DEL_PROYECTO]` es `[DESCRIPCIÓN BREVE DEL PRODUCTO O SISTEMA]`.

Usuarios o personas afectadas: `[USUARIOS]`.

No es: `[LÍMITES DE ALCANCE, POR EJEMPLO IDE, NAVEGADOR, ERP]`.

## Prioridades

Ordenar explícitamente los criterios de decisión:

1. `[SEGURIDAD, DATOS, CUMPLIMIENTO O CONFIABILIDAD]`;
2. `[CORRECCIÓN Y ESTABILIDAD]`;
3. `[RENDIMIENTO O COSTE]`;
4. `[EXPERIENCIA, ACCESIBILIDAD O DISEÑO]`;
5. `[MANTENIBILIDAD]`.

Presupuestos o límites medibles: `[TAMAÑO, LATENCIA, MEMORIA, COSTE, SLO]`.
Indicar qué propiedades nunca se sacrifican para mejorar una métrica.

## Relación con el propietario

El propietario decide producto y prioridades. Los agentes deben explicar riesgos
y decisiones importantes en lenguaje natural, distinguir hechos de inferencias,
definir jerga útil y señalar decisiones difíciles de revertir.

Resolver ambigüedades reversibles con una opción conservadora y dejar registro.
Detenerse ante decisiones que afecten datos, seguridad, producto, arquitectura,
compatibilidad, dependencias estructurales o UX fundamental.

## Invariantes del proyecto

Escribir solo reglas que casi cualquier cambio debe respetar:

- `[EJEMPLO: no perder ni reescribir datos silenciosamente]`;
- `[EJEMPLO: no realizar red implícita]`;
- `[EJEMPLO: no ejecutar contenido controlado por usuarios]`;
- `[EJEMPLO: accesibilidad mínima]`;
- `[EJEMPLO: compatibilidad o formato persistente]`.

Los detalles deben vivir en documentación especializada, no repetirse aquí.

## Economía de implementación

Preferir la solución correcta más pequeña para el requisito actual.

1. comprobar si ya existe la capacidad;
2. reutilizar código, stdlib o dependencias presentes cuando sea adecuado;
3. evitar abstracciones, configuraciones y extensibilidad hipotética;
4. no agregar dependencias sin necesidad e impacto justificados;
5. eliminar o simplificar código cuando eso cumpla mejor el requisito.

La simplicidad no justifica menor seguridad, pérdida de datos, errores ocultos,
accesibilidad deficiente o comportamiento implícito.

## Fuentes de autoridad

Antes de cambiar código:

1. leer estas instrucciones;
2. revisar rama, HEAD, `git status` y diff relevante;
3. clasificar el riesgo del cambio;
4. leer documentación, ADR y pruebas del dominio afectado;
5. comprobar implementación real antes de asumir que la documentación está al día.

Mapa a completar:

| Dominio | Documento normativo | Tests o evidencia |
| --- | --- | --- |
| Producto y UX | `[RUTA]` | `[RUTA]` |
| Arquitectura | `[RUTA]` | `[RUTA]` |
| Seguridad y datos | `[RUTA]` | `[RUTA]` |
| Rendimiento | `[RUTA]` | `[RUTA]` |
| Dependencias | `[RUTA]` | `[RUTA]` |
| QA y releases | `[RUTA]` | `[RUTA]` |

## Verificación proporcional al riesgo

### Nivel 1: cambio normal

Ejemplos: documentación, UI aislada, bug local o refactor pequeño.

- formatter y lint/typecheck relevante;
- pruebas directamente afectadas;
- revisión manual cuando aporte evidencia;
- no ejecutar auditorías o benchmarks globales sin motivo.

### Nivel 2: cambio sensible

Ejemplos: entrada no confiable, autenticación, autorización, filesystem, red,
persistencia, migraciones, codificación, dependencias, rendimiento crítico o
datos personales.

- Nivel 1;
- identificar las propiedades afectadas;
- pruebas negativas y de regresión;
- medir impacto relevante;
- actualizar documentación de seguridad, arquitectura o datos.

### Nivel 3: release, milestone o cambio estructural

- suite completa;
- auditoría de dependencias y licencias cuando aplique;
- benchmark y pruebas de integración o migración;
- revisión de riesgos residuales, observabilidad y documentación de release.

## Git y preservación

- trabajar en la rama acordada; proteger la rama estable;
- preservar cambios ajenos y distinguirlos de los propios;
- no usar reset destructivo, borrados amplios ni sobrescrituras sin autorización;
- commits pequeños, coherentes y verificables;
- no mezclar refactor, función, dependencia y cambio de política en un commit.

## Documentación y decisiones

- actualizar la fuente normativa cuando cambie comportamiento;
- crear ADR solo para arquitectura, seguridad, formatos persistentes,
  compatibilidad o dependencias estructurales;
- evitar documentación duplicada; enlazar a la autoridad del tema;
- registrar deuda aceptada con motivo y condición de revisión.

## Criterio de finalización

Una tarea termina cuando cumple su objetivo, aplica la verificación proporcional,
no deja estados a medio conectar, conserva trabajo ajeno, actualiza documentación
afectada y deja un estado Git entendible.

Para completar esta plantilla antes de activarla, definir:

- qué datos son sensibles o irreemplazables;
- qué entradas no son confiables;
- qué recursos externos o permisos existen;
- cuáles son los límites de rendimiento y coste;
- qué plataformas y compatibilidades se prometen;
- dónde viven los tests, ADR, threat model y roadmap;
- quién puede aprobar cambios de seguridad, arquitectura y publicación.
