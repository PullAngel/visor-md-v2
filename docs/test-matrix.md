# Matriz de pruebas

Estado de esta matriz: recuperación activa. Los nombres de tests indicados viven
en `src/main.rs` hasta que el proyecto se divida en módulos.

Estados permitidos:

- `Pendiente`: todavía no existe evidencia suficiente.
- `Parcial`: existe una prueba, pero no cubre toda la propiedad.
- `Verificado`: evidencia automatizada o manual reproducible.
- `Bloqueado`: depende de una capacidad aún no implementada.

## Núcleo Markdown

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| CommonMark aplicable | Corpus | Parcial | Pendiente | Corpus versionado del lector cubre encabezados, inline, saltos, escapes, entidades, links, citas, listas, código, tablas e imágenes anunciadas; falta suite oficial seleccionada |
| Formato inline anidado | Integración | Verificado | Pendiente | `el_enfasis_anidado_se_acumula` y tests de rangos |
| Listas y task lists | Integración y visual | Parcial | Pendiente | Parser, layout y píxeles verificados; falta evidencia estética |
| Allowlist HTML y HTML no permitido | Seguridad e integración | Verificado | Pendiente | `br`, `kbd`, `mark`, `sub` y `sup` sin atributos llegan a layout; HTML hostil, atributos y cierres defectuosos quedan visibles e inertes |
| Profundidad limitada | Adversarial | Verificado | Pendiente | 5.000 citas, listas e inline anidado |
| Fallback a fuente segura | Modelo y end to end | Parcial | Pendiente | Fuente completa y título de aviso verificados; falta QA visual |
| Unicode y fallback | Corpus y manual | Pendiente | Pendiente | Fuentes latinas parciales |

## Archivos y edición

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Apertura explícita local | Integración | Parcial | Pendiente | Prototipo Sprint 0 |
| Apertura manual UNC | Integración | Pendiente | No aplica | Política definida |
| Traversal y escape bloqueados | Seguridad | Pendiente | Pendiente | VFS no implementado |
| Symlinks y junctions | Seguridad | Pendiente | Pendiente | VFS no implementado |
| Guardado atómico | Integración | Bloqueado | Bloqueado | Editor pendiente |
| Rangos de fuente preservados | Integración | Parcial | Pendiente | Bloques, tramos y destinos verificados |
| Sintaxis desconocida preservada | Property | Parcial | Pendiente | HTML inerte y documento defectuoso histórico; falta round-trip |
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
| Selección con mouse | Integración y visual | Parcial | Pendiente | Hit testing y geometría de Parley comparten el layout; autoscroll probado, pero falta QA manual, selección general y copia |
| Cursor de texto sobre contenido | Manual | Parcial | Pendiente | Cambia con el mismo hit testing de selección; requiere QA visual |
| Selección con teclado | Accesibilidad | Parcial | Pendiente | Flechas verticales y horizontales, Shift+flechas, Ctrl+A y Escape funcionan; falta foco y atajos de línea completos |
| Menú contextual | End to end | Bloqueado | Bloqueado | Referencia en v1 |
| IME | Manual | Bloqueado | Bloqueado | Editor pendiente |
| Lector de pantalla | Accesibilidad | Bloqueado | Bloqueado | Estrategia pendiente |
| Reduce motion | Manual y unitario | Bloqueado | Bloqueado | Chrome pendiente |

## Rendimiento

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Ventana visible | Benchmark | Verificado | Pendiente | Mediana 89 ms y P95 600 ms en diez ejecuciones |
| Primer contenido normal | Benchmark | Verificado | Pendiente | Mediana 102,5 ms y P95 612 ms; muestras versionadas |
| Documento de 5 MB | Benchmark | Verificado en Sprint 0 | Pendiente | `budget.md` |
| Scroll proporcional a visible | Unitario y benchmark | Parcial | Pendiente | Rango visible por búsqueda binaria; 4,4 ms medidos |
| Memoria estable | Benchmark | Parcial | Pendiente | Medición inicial |
| Binario menor de 8 MB | Release | Verificado | Pendiente | 2.996.736 bytes en `6176a82` |

## Cadena de suministro

| Propiedad | Nivel | Estado | Evidencia actual |
| --- | --- | --- | --- |
| Advisories conocidos | Audit | Verificado el 2026-08-26 | Cero vulnerabilidades, un crate no mantenido |
| Licencias compatibles | Legal | Parcial | SBOM completo; falta revisión de compatibilidad y notices |
| SBOM reproducible | Release | Parcial | Generador CycloneDX desde metadata bloqueada; falta validador independiente |
| Dependencias C y `unsafe` conocidas | Audit | Parcial | Diferencias Windows y Linux |
| Fuentes reproducibles | Supply chain | Verificado | Script reproduce cuatro hashes y licencia versionada |

## Regla de actualización

Una fila pasa a `Verificado` solo si enlaza a un test, comando, artifact o
registro reproducible. Una afirmación verbal o un binario antiguo no alcanza.
