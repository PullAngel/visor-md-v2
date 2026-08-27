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
| CommonMark aplicable | Corpus | Parcial | Pendiente | Fixture versionada más `casos_commonmark_gfm_anunciados_llegan_a_layout` cubren parser, rangos y layout de encabezados, inline, saltos, escapes, enlaces, citas, listas, tareas, código, tablas, HTML permitido e inerte; incluye ejemplos 16 y 20 de CommonMark 0.31.2; falta suite oficial compatible |
| Formato inline anidado | Integración | Verificado | Pendiente | `el_enfasis_anidado_se_acumula` y tests de rangos |
| Listas y task lists | Integración y visual | Parcial | Pendiente | Parser, layout y píxeles verificados; falta evidencia estética |
| Allowlist HTML y HTML no permitido | Seguridad e integración | Verificado | Pendiente | `br`, `kbd`, `mark`, `sub` y `sup` sin atributos llegan a layout; HTML hostil, atributos y cierres defectuosos quedan visibles e inertes |
| Profundidad limitada | Adversarial | Verificado | Pendiente | 5.000 citas, listas e inline anidado |
| Línea extensa | Adversarial | Verificado | Pendiente | Más de 16 KiB degrada a tramos UTF-8 inertes y reconstruibles |
| Barrido adversarial reproducible | Property y adversarial | Verificado | Pendiente | 128 combinaciones deterministas sin `panic`; no sustituye fuzzing formal |
| Fallback a fuente segura | Modelo y end to end | Parcial | Pendiente | Fuente completa y título de aviso verificados; falta QA visual |
| Unicode y fallback | Corpus, layout y manual | Parcial | Pendiente | Corpus árabe, devanagari, japonés, coreano y emoji llega a layout; fuentes embebidas siguen siendo latinas y falta QA visual |

## Archivos y edición

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Apertura explícita local | Integración | Parcial | Pendiente | Un mismo handle limita 16 MiB y exige UTF-8; falta QA visual y política configurable |
| Parser fuera de UI | Integración y rendimiento | Parcial | Pendiente | Hilo de trabajo entrega un único evento al hilo de ventana; falta benchmark y cancelación |
| Apertura manual UNC | Integración | Pendiente | No aplica | Política definida |
| Traversal y escape bloqueados | Seguridad | Bloqueado | Bloqueado | No hay apertura secundaria todavía; deberá pasar por VFS al existir enlaces o recursos |
| Symlinks y junctions | Seguridad | Pendiente | Pendiente | VFS no implementado |
| Guardado atómico | Integración | Bloqueado | Bloqueado | Editor pendiente |
| Rangos de fuente preservados | Integración | Parcial | Pendiente | Bloques, tramos y destinos verificados; el editor source-first no usa el rango parcial que Comrak informa para la sintaxis de enlaces |
| Parches de fuente y undo/redo | Unitario | Parcial | Pendiente | `editor::EditHistory` preserva UTF-8, revisiones, undo/redo y presupuesto de 4 MiB; `SourceEditor` fija cursor y selección Unicode. Falta interacción, IME y round-trip de guardado |
| Edición de fuente inicial | Integración y manual | Parcial | Pendiente | F2 alterna fuente/lectura; IME, Backspace, Delete, Ctrl+Z y Ctrl+Y usan parches. Falta cursor visible, selección por mouse, navegación vertical, prueba end-to-end y parsing asíncrono |
| Sintaxis desconocida preservada | Property | Parcial | Pendiente | HTML inerte y documento defectuoso histórico; falta round-trip |
| Cambios externos detectados | Unitario e integración | Parcial | Pendiente | La apertura captura tamaño y fecha de modificación; una prueba detecta cambio de tamaño. Falta comparación antes de guardar, identidad de handle y diálogo de conflicto |
| EOL, BOM y UTF-8 | Corpus | Parcial | Pendiente | Apertura limitada exige UTF-8, separa BOM UTF-8 sin volverlo contenido, conserva CRLF/LF/mixto y reconstruye bytes sin edición en pruebas; faltan parches, guardado y política de otros encodings |

## Red y recursos

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Apertura normal sin sockets | Seguridad de runtime | Pendiente | Pendiente | Sin monitor automatizado |
| Imagen remota bloqueada | Integración | Bloqueado | Bloqueado | Imágenes pendientes |
| Consentimiento remoto delimitado | End to end | Bloqueado | Bloqueado | Componente pendiente |
| Imagen local contenida y limitada | Seguridad | Bloqueado | Bloqueado | VFS e imágenes pendientes |
| Hipervínculo revela destino real | UX y phishing | Parcial | Pendiente | El hover muestra el destino declarado sin abrirlo; falta QA visual y política de clic |

## Rendering y UX

| Propiedad | Nivel | Windows | Linux | Evidencia actual |
| --- | --- | --- | --- | --- |
| Tema claro y oscuro | Visual | Parcial | Pendiente | Commit `090e9de` |
| Resize sin layout corrupto | Integración | Parcial | Pendiente | Prototipo |
| DPI y zoom | Unitario, manual y visual | Parcial | Pendiente | El layout escala cuerpo, márgenes, sangrías y marcadores a partir de `Window::scale_factor`; `la_escala_dpi_aumenta_la_tipografia_sin_cambiar_el_ancho_logico` verifica la propiedad geométrica. Falta QA en monitores con distintas escalas y el futuro zoom explícito. |
| Selección con mouse | Integración y visual | Parcial | Pendiente | Hit testing y geometría de Parley comparten el layout; autoscroll y copia unitaria probados, falta QA manual |
| Copia al portapapeles | Integración y seguridad | Parcial | Pendiente | `Ctrl+C` y `Ctrl+Shift+C` distinguen vista y fuente en pruebas de selección; falta QA con otras aplicaciones y plataformas |
| Cursor de texto sobre contenido | Manual | Parcial | Pendiente | Cambia con el mismo hit testing de selección; requiere QA visual |
| Pérdida de foco durante selección | Manual | Parcial | Pendiente | Cancela arrastre y modificadores, conserva selección; requiere QA manual |
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
