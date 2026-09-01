# Visor MD v2

Visor MD v2 es una aplicación nativa, ligera y offline para leer y editar Markdown y texto inerte.

Está pensada para estudio, documentos producidos por IA y bóvedas existentes de Obsidian, sin ser navegador, IDE ni reemplazo de Obsidian.

## Estado real

Es una versión de desarrollo avanzada, no una v2.0 distribuible todavía. Ya abre, lee, edita, compara fuente y vista, guarda de forma atómica, usa pestañas y navega una carpeta autorizada. Incluye búsqueda, índice, wikilinks, backlinks, callouts, recuperación local explícita y límites para contenido patológico.

Faltan QA humano sistemático, accesibilidad completa, corpus ampliado de bóvedas, herramientas de estudio, exportación PDF/DOCX, empaquetado y validación final de plataformas. Ver [`docs/status.md`](docs/status.md) y [`docs/roadmap.md`](docs/roadmap.md).

## Principios

- Seguridad y preservación de datos antes que funciones llamativas.
- Sin WebView, DOM, JavaScript, telemetría ni red implícita.
- Markdown fuente primero: no se reformatea ni normaliza silenciosamente.
- La persona concede una carpeta; la VFS valida cada destino dentro de esa raíz.
- El documento domina la interfaz; herramientas progresivas, no un IDE permanente.
- Objetivo de tamaño: ~7 MB; límite deseado: 8 MB sin sacrificar seguridad, Unicode o estabilidad.

## Capacidades actuales

- CommonMark y GFM esencial, lectura, fuente y comparación.
- Edición Unicode con IME, selección, undo/redo, pegado explícito y ayudas Markdown reversibles.
- Guardado atómico, conflictos externos, recuperación y preservación de UTF-8, BOM y EOL.
- Pestañas, búsqueda, índice, workspace en memoria, árbol, wikilinks y backlinks sin escribir en `.obsidian`.

## Seguridad y desarrollo

El contenido no ejecuta scripts ni carga recursos remotos automáticamente. Las rutas externas, absolutas, UNC o fuera de la raíz se bloquean; entradas que exceden límites degradan a fuente inerte visible. Ver [`docs/security.md`](docs/security.md) y [`docs/threat-model.md`](docs/threat-model.md).

Requiere Rust estable con MSVC en Windows. Gate local:

```powershell
.\scripts\check.ps1
```

La pila usa Rust, `winit`, `softbuffer`, `tiny-skia`, `parley`, `swash` y `comrak`; no hay motor web. La documentación operativa empieza en [`docs/README.md`](docs/README.md).

Visor MD se desarrolla con asistencia de IA bajo dirección humana: Angel David Durán Erazo define producto, prioridades y límites. El objetivo del repositorio es demostrar producto, ciberseguridad aplicada y QA verificable.

## Contribución y licencia

Leer [`AGENTS.md`](AGENTS.md) y [`CONTRIBUTING.md`](CONTRIBUTING.md). Los reportes sensibles siguen [`SECURITY.md`](SECURITY.md). Visor MD v2 se distribuye bajo GNU GPL v3.
