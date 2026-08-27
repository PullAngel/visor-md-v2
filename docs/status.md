# Estado actual

Última revisión: 26 de agosto de 2026.

## Resumen

Visor MD v2 tiene un prototipo nativo medido y una recuperación funcional de
Sprint 1 versionada en `codex/sprint-1-recovery`. El último commit estable de
producto en `main` sigue siendo `090e9de`.

El source heredado volvió a compilar y está verde. La recuperación conectó
el trabajo interrumpido, reforzó el modelo documental y convirtió los límites
defensivos en un fallback verificable. La tipografía ya tiene licencia y pipeline
reproducible. Falta inspección visual y revisar el contrato de round-trip antes de
considerar estabilizada la recuperación.

## Git

- Rama activa de recuperación: `codex/sprint-1-recovery`.
- `main` y `origin/main`: `090e9de`, verificado el 26 de agosto de 2026.
- Documentación base: `738faff`.
- Fuentes y reproducción: `d566518`.
- Renderer y modo seguro: `a54c9d6`.
- Los snapshots de preservación no son parte del producto.
- Ningún commit de recuperación se aplicó sobre `main`.

Consultar [`workspace-handoff.md`](workspace-handoff.md) para el inventario
exacto del traspaso.

## Implementado en el último commit estable

- ventana nativa con `winit`;
- framebuffer por software;
- parsing Markdown básico;
- layout de texto y render visible;
- virtualización inicial;
- tema claro y oscuro;
- detección del tema del sistema;
- fuentes embebidas;
- perfiles release orientados a tamaño;
- mediciones iniciales de apertura, scroll y memoria.

## Trabajo recuperado y probado

- tramos inline con estilo;
- negrita, cursiva y anidamiento;
- tachado y decoraciones;
- listas, blockquotes y reglas horizontales;
- task lists;
- límites de recursión;
- fallback completo a fuente inerte;
- rangos de fuente para bloques y tramos;
- texto UTF-8 abierto retenido durante la sesión junto a esos rangos;
- destinos de enlaces e imágenes preservados como datos;
- semántica de tablas y lenguaje de bloques de código;
- HTML desconocido visible e inerte;
- allowlist HTML nativa y sin atributos: `br`, `kbd`, `mark`, `sub` y `sup`;
- búsqueda binaria del tramo visible;
- scroll limitado por el viewport real;
- fallos de inicialización y presentación gráfica reportados sin `panic`;
- 53 pruebas unitarias y adversariales, incluido un corpus integrado de lector
  y un barrido adversarial determinista.
- copia explícita de selección: texto visible con `Ctrl+C` y Markdown fuente de
  bloques con `Ctrl+Shift+C`; falta QA manual con aplicaciones externas.
- apertura primaria desde un mismo handle, limitada a 16 MiB y UTF-8 válido;
  no habilita recursos secundarios ni navega rutas del documento.
- apertura y parsing fuera del hilo de interfaz; la ventana se crea mientras el
  modelo se prepara y no recibe un estado parcial.
- enlaces visibles con cursor y destino declarado al hover, sin resolver rutas
  ni abrir navegador o filesystem.
- un fallo de apertura asíncrona conserva la ventana y muestra un aviso simple;
  el detalle técnico queda solo en el registro local de la sesión.
- solo extensiones Markdown pasan al parser; otros textos UTF-8 se muestran de
  forma inerte, sin comportamiento de IDE ni ejecución.

Evidencia actual en Windows:

- `cargo check`: verde;
- `cargo fmt -- --check`: verde;
- `cargo clippy --all-targets -- -D warnings`: verde;
- `cargo test`: 56 de 56 pruebas verdes tras la apertura limitada;
- `scripts/check.ps1`: verde el 26 de agosto de 2026 (formato, Clippy, pruebas,
  SBOM, documentación y release);
- último release de `6176a82`: 2.996.736 bytes, 2,858 MiB;
- working tree con allowlist HTML: 3.000.320 bytes, 2,861 MiB;
- working tree con selección por bloque: 3.009.536 bytes, 2,870 MiB;
- working tree con selección de teclado: 3.010.048 bytes, 2,871 MiB;
- working tree con autoscroll de selección: 3.011.584 bytes, 2,872 MiB;
- working tree con Ctrl+A: 3.012.096 bytes, 2,873 MiB;
- working tree con navegación vertical: 3.013.120 bytes, 2,874 MiB;
- working tree con cursor contextual: 3.013.632 bytes, 2,874 MiB;
- working tree con portapapeles de texto: 3.019.264 bytes, 2,879 MiB;
- working tree con apertura primaria limitada: 3.021.312 bytes, 2,881 MiB;
- primer pintado mediano sobre diez ejecuciones: 102,5 ms;
- P95 de primer pintado: 612 ms;
- scroll automatizado: 4,4 ms por cuadro.

La serie automatizada conserva una ejecución de 612 ms y las muestras crudas.
Ejecuciones anteriores alcanzaron 388, 587, 631 y 965 ms. No se les atribuye
causa porque todavía no se controlan caché, carga y planificación del sistema.

## Pendientes inmediatos

- verificación visual de task lists, decoraciones, temas y cursiva;
- selección de ejemplos de la suite oficial CommonMark y ampliación GFM sistemática;
- separación incompleta de `main.rs`; fuentes y tema ya tienen módulos propios;
- VFS de recursos secundarios, contención de rutas y política de bóvedas;
- cancelación de parsing y política de reemplazo si cambia el documento;
- virtualización y alturas todavía aproximadas;
- cobertura y fallback tipográfico todavía sin matriz completa;
- medición de memoria pendiente de repetir tras retener la fuente de sesión;
- ausencia de CI, fuzzing, validación independiente del SBOM y gates
  multiplataforma.
- QA manual de copia, incluidos atajos, aplicaciones destino y plataforma;
- selección parcial con mouse y autoscroll, flechas y Shift+flechas en ambas
  direcciones, Ctrl+A y Escape; la copia ya tiene pruebas unitarias.

## Evidencia disponible

- mediciones de Sprint 0 en [`budget.md`](budget.md);
- decisiones históricas en [`decisions.md`](decisions.md);
- amenaza y controles en [`threat-model.md`](threat-model.md) y
  [`security.md`](security.md);
- snapshots `claude-working-tree.diff` y
  `claude-working-tree-status.txt` en el working tree local;
- backup externo creado por el propietario;
- artifacts visuales recuperados localmente.

## Próximo criterio de salida

La recuperación inicial termina cuando:

- se completa la inspección visual y no aparecen regresiones;
- se valida el modo seguro end to end, no solo en el modelo;
- el modelo recuperado recibe revisión de round-trip antes de editar;
- la selección de estilos y fallback tipográfico supera QA visual;
- el diff puede revisarse en commits pequeños;
- no se perdió trabajo heredado sin decisión explícita.

Las comprobaciones automatizables de esa lista están cubiertas. El cierre sigue
abierto por QA visual y por la revisión del contrato de round-trip; no por un
working tree roto.
