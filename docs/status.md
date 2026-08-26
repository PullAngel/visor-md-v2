# Estado actual

Última revisión: 25 de agosto de 2026.

## Resumen

Visor MD v2 tiene un prototipo nativo medido y un bloque parcial de Sprint 1.
El último commit estable es `090e9de`. El working tree local contiene trabajo
heredado de Claude Code que quedó interrumpido durante el cableado de task list
markers.

El source local actual no compila. La recuperación debe preservar las funciones
válidas, corregir el modelo documental y volver a establecer una base verde antes
de continuar el roadmap.

## Git

- Rama activa de recuperación: `codex/sprint-1-recovery`.
- `main` y `origin/main`: `090e9de` durante la auditoría inicial.
- El código y las fuentes heredadas permanecen sin commit.
- Los snapshots de preservación no son parte del producto.
- La documentación nueva se publica separada del código incompleto.

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

## Trabajo local parcial

- tramos inline con estilo;
- negrita, cursiva y anidamiento;
- tachado y decoraciones;
- listas, blockquotes y reglas horizontales;
- task lists;
- límites de recursión;
- pruebas adicionales.

Este bloque no está terminado ni validado. El binario de tests que reporta 17
pruebas verdes es anterior a las últimas modificaciones.

## Fallos conocidos inmediatos

- incompatibilidad entre un `Option<String>` heredado y el nuevo `Marker`;
- dos usos de `unwrap_or` sobre pinceles que ya no son opcionales;
- modelo documental que pierde rangos y semántica necesaria;
- `main.rs` monolítico;
- acceso directo a filesystem sin VFS;
- parsing en el camino de UI;
- virtualización y alturas todavía aproximadas;
- proceso de fuentes no completamente reproducible;
- ausencia de CI, fuzzing, SBOM y gates multiplataforma.

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

- compila el source actual;
- los tests se reconstruyen y pasan;
- `Marker` está conectado de parser a dibujo;
- el stack overflow tiene límite y fallback verificables;
- el modelo conserva la información necesaria para edición;
- las fuentes tienen procedencia y reproducción cerradas;
- el diff puede revisarse en commits pequeños;
- no se perdió trabajo heredado sin decisión explícita.
