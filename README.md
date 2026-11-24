# Visor MD v2

Lector y editor nativo de Markdown para estudiar, trabajar con documentos
producidos por IA y recorrer notas de Obsidian. Offline, con edición fiel de la
fuente y controles explícitos sobre archivos y recursos externos.

[![Validación Windows y Linux](https://github.com/PullAngel/visor-md-v2/actions/workflows/ci.yml/badge.svg)](https://github.com/PullAngel/visor-md-v2/actions/workflows/ci.yml)
[![Licencia GPL-3.0-only](https://img.shields.io/badge/licencia-GPL--3.0--only-blue)](LICENSE)

[Estado](docs/project-dashboard.md) · [Arquitectura](docs/architecture.md) ·
[Modelo de amenazas](docs/threat-model.md) · [Pruebas](docs/testing.md) ·
[Roadmap](docs/roadmap.md)

## Qué puedes hacer hoy

- Leer Markdown con encabezados, listas, tareas, tablas, citas, código y formato
  inline; alternar entre lectura, fuente editable y comparación fuente/vista.
- Escribir con ayudas de formato reversibles y menús de bloques, encabezados
  H1–H6, listas, enlaces, tablas y estudio.
- Mantener varias pestañas y dividir una ventana entre hasta cuatro documentos
  distintos, con historial y estado de guardado por documento.
- Guardar conservando UTF-8, BOM y saltos de línea; detectar conflictos externos
  y recuperar ediciones locales tras un cierre inesperado.
- Buscar y navegar una carpeta autorizada: árbol de notas, índice, wikilinks,
  backlinks y callouts, sin escribir en `.obsidian`.
- Preparar preguntas, resúmenes, conceptos y fragmentos mediante Markdown
  portable. Estas ayudas funcionan localmente; la aplicación no incorpora IA.

**Estado: en desarrollo, sin release estable.** La base es funcional, pero sigue
pendiente el QA manual sistemático, la accesibilidad completa y la validación
de plataformas. Ventanas independientes, comparación entre versiones,
exportación PDF/DOCX y paquetes de distribución todavía están pendientes.
El [catálogo de funciones](docs/features.md) distingue lo implementado, lo
parcial y lo planificado.

## Seguridad con evidencia

Los documentos se consideran entradas no confiables. Las decisiones se pueden
revisar desde el riesgo hasta el código y las pruebas:

| Riesgo | Control implementado | Dónde revisarlo |
| --- | --- | --- |
| Contenido activo y recursos remotos | Render nativo sin WebView, DOM ni JavaScript; apertura y render sin red implícita; HTML desconocido visible e inerte. | [Política de contenido](docs/security.md), [modelo de amenazas](docs/threat-model.md) |
| Acceso a archivos desde una nota | Raíz de carpeta concedida explícitamente y validación VFS de los destinos secundarios; rechazo de escapes y rutas no autorizadas. | [VFS](src/vfs.rs), [conectividad](docs/connectivity.md), [bóveda adversarial](tests/fixtures/obsidian-vault/hostil.md) |
| Pérdida o corrupción de documentos | Ediciones sobre la fuente, guardado atómico, comprobación de cambios externos y recuperación explícita. | [Archivos](src/files.rs), [editor](src/editor.rs), [matriz de pruebas](docs/test-matrix.md) |
| Agotamiento de recursos | Límites de entrada, profundidad, bloques e imágenes; degradación visible a texto inerte e índice de carpeta acotado y cancelable. | [Límites](src/limits.rs), [workspace](src/workspace.rs), [presupuesto](docs/budget.md) |
| Dependencias y procedencia | Lockfile versionado, SBOM CycloneDX regenerable y registro de revisiones de dependencias. | [SBOM](sbom.cdx.json), [cadena de suministro](docs/dependencies.md) |

Estos controles no equivalen a una auditoría independiente ni a una garantía
de seguridad. Entre los riesgos abiertos están las carreras entre comprobación
y uso de rutas (TOCTOU) y la validación específica por plataforma. La
recuperación guarda copias temporales **sin cifrar** en el perfil local y puede
desactivarse. Consulta [seguridad](docs/security.md) y [riesgos y deuda](docs/project-dashboard.md)
para evaluar los límites actuales. Los reportes sensibles siguen [SECURITY.md](SECURITY.md).

## Probar desde el código

En Windows se necesita Rust estable con toolchain MSVC y las herramientas de
compilación de Visual C++. Desde PowerShell:

```powershell
git clone https://github.com/PullAngel/visor-md-v2.git
cd visor-md-v2
cargo build --locked --release
& .\target\release\visor-md.exe .\tests\fixtures\sprint1-visual.md
```

En Linux, los paquetes nativos de compilación y los comandos validados están en
el [workflow de CI](.github/workflows/ci.yml). La ejecución visual en Linux aún
requiere QA de plataforma.

`F2` alterna lectura y edición; `F3` compara fuente y vista;
`Ctrl+Shift+P` abre las acciones disponibles. Para evaluar guardado y recuperación,
usa copias de tus documentos y el [checklist de QA](docs/manual-qa-v2.md).

La compilación release optimiza tamaño con LTO y puede tardar: no representa el
tiempo de apertura del ejecutable ya compilado. El build Windows del checkpoint
`90ca7e3`, compilado el 14 de octubre de 2026, mide **3,41 MiB**; el objetivo
deseado es menos de 8 MB. La medición, hash y límites de comparación están en
[presupuesto](docs/budget.md).

## Cómo se verifica

La última suite registrada para `90ca7e3` contiene **248 pruebas** y Clippy
estricto en verde. Incluye regresiones de Unicode/CRLF, undo/redo, conflictos de
archivos, resultados asíncronos antiguos, rutas hostiles y límites de recursos.
El estado actual de los jobs de Windows y Linux se consulta en
[Actions](https://github.com/PullAngel/visor-md-v2/actions/workflows/ci.yml).

Para reproducir el gate local de Windows:

```powershell
.\scripts\check.ps1
```

Comprueba formato, Clippy, pruebas, SBOM, enlaces de documentación y build
release. La [estrategia de QA](docs/testing.md) separa esa evidencia de las
pruebas manuales de interacción, DPI, accesibilidad y compatibilidad. Fuzzing y
auditoría de release siguen pendientes; no se presentan como controles cerrados.

La pila es Rust, `winit`, `softbuffer`, `tiny-skia`, `parley`, `swash` y `comrak`.
Las decisiones y sus compromisos están en [arquitectura](docs/architecture.md)
y [ADRs](docs/decisions.md). La [documentación](docs/README.md) ofrece rutas de
lectura por tema.

## Autoría y dirección

Proyecto personal de **PullAngel**, estudiante de redes y
ciberseguridad. Su aportación central es la dirección de producto: definir
necesidades, priorizar seguridad y UX, evaluar alternativas y revisar el
comportamiento mediante QA manual.

La implementación se desarrolla con asistencia de IA. La aportación humana se
evalúa a través de requisitos, decisiones justificadas, revisión de riesgos y
feedback de uso. El repositorio conserva regresiones y mediciones revisables
para acompañar ese trabajo y el aprendizaje de seguridad aplicada.

## Contribuir

Consulta [CONTRIBUTING.md](CONTRIBUTING.md) para proponer cambios y reproducir
las verificaciones. Licencia [GNU GPL-3.0-only](LICENSE); las fuentes embebidas
conservan sus [licencias y procedencia](assets/fonts/README.md).
