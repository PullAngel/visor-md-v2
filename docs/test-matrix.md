# Matriz de pruebas

Estado de esta matriz: diseño inicial. Se completará con identificadores de tests
y evidencia durante la recuperación.

Estados permitidos:

- `Pendiente`: todavía no existe evidencia suficiente.
- `Parcial`: existe una prueba, pero no cubre toda la propiedad.
- `Verificado`: evidencia automatizada o manual reproducible.
- `Bloqueado`: depende de una capacidad aún no implementada.

## Núcleo Markdown

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| CommonMark aplicable | Corpus | Parcial | Pendiente | Tests unitarios parciales |
| Formato inline anidado | Integración | Parcial | Pendiente | Working tree heredado |
| Listas y task lists | Integración y visual | Parcial | Pendiente | Source no compilable |
| HTML no permitido queda inerte | Seguridad | Pendiente | Pendiente | Política documentada |
| Profundidad limitada | Adversarial | Parcial | Pendiente | Caso de 5.000 citas |
| Fallback a fuente segura | End to end | Pendiente | Pendiente | No implementado |
| Unicode y fallback | Corpus y manual | Pendiente | Pendiente | Fuentes latinas parciales |

## Archivos y edición

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Apertura explícita local | Integración | Parcial | Pendiente | Prototipo Sprint 0 |
| Apertura manual UNC | Integración | Pendiente | No aplica | Política definida |
| Traversal y escape bloqueados | Seguridad | Pendiente | Pendiente | VFS no implementado |
| Symlinks y junctions | Seguridad | Pendiente | Pendiente | VFS no implementado |
| Guardado atómico | Integración | Bloqueado | Bloqueado | Editor pendiente |
| Sintaxis desconocida preservada | Property | Bloqueado | Bloqueado | Modelo pendiente |
| Cambios externos detectados | End to end | Bloqueado | Bloqueado | Editor pendiente |
| EOL, BOM y UTF-8 | Corpus | Bloqueado | Bloqueado | Política pendiente de implementar |

## Red y recursos

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Apertura normal sin sockets | Seguridad de runtime | Pendiente | Pendiente | Sin monitor automatizado |
| Imagen remota bloqueada | Integración | Bloqueado | Bloqueado | Imágenes pendientes |
| Consentimiento remoto delimitado | End to end | Bloqueado | Bloqueado | Componente pendiente |
| Imagen local contenida y limitada | Seguridad | Bloqueado | Bloqueado | VFS e imágenes pendientes |
| Hipervínculo revela destino real | UX y phishing | Bloqueado | Bloqueado | Enlaces pendientes |

## Rendering y UX

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Tema claro y oscuro | Visual | Parcial | Pendiente | Commit `090e9de` |
| Resize sin layout corrupto | Integración | Parcial | Pendiente | Prototipo |
| DPI y zoom | Manual y visual | Pendiente | Pendiente | Sin evidencia |
| Selección con mouse | End to end | Bloqueado | Bloqueado | No implementado |
| Selección con teclado | Accesibilidad | Bloqueado | Bloqueado | No implementado |
| Menú contextual | End to end | Bloqueado | Bloqueado | Referencia en v1 |
| IME | Manual | Bloqueado | Bloqueado | Editor pendiente |
| Lector de pantalla | Accesibilidad | Bloqueado | Bloqueado | Estrategia pendiente |
| Reduce motion | Manual y unitario | Bloqueado | Bloqueado | Chrome pendiente |

## Rendimiento

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Ventana visible | Benchmark | Verificado en Sprint 0 | Pendiente | Cerca de 120 ms reportados |
| Primer contenido normal | Benchmark | Verificado en Sprint 0 | Pendiente | `budget.md` |
| Documento de 5 MB | Benchmark | Verificado en Sprint 0 | Pendiente | `budget.md` |
| Scroll proporcional a visible | Benchmark | Parcial | Pendiente | Riesgo O(n) conocido |
| Memoria estable | Benchmark | Parcial | Pendiente | Medición inicial |
| Binario menor de 8 MB | Release | Verificado en Sprint 0 | Pendiente | Prototipo debajo del límite |

## Cadena de suministro

| Propiedad | Nivel | Estado | Evidencia actual |
| --- | --- | --- | --- |
| Advisories conocidos | Audit | Parcial | Sin vulnerabilidades, un crate no mantenido |
| Licencias compatibles | Legal | Parcial | Falta inventario completo |
| SBOM reproducible | Release | Pendiente | No implementado |
| Dependencias C y `unsafe` conocidas | Audit | Parcial | Diferencias Windows y Linux |
| Fuentes reproducibles | Supply chain | Parcial | Falta cerrar subset y hashes |

## Regla de actualización

Una fila pasa a `Verificado` solo si enlaza a un test, comando, artifact o
registro reproducible. Una afirmación verbal o un binario antiguo no alcanza.
