@AGENTS.md

## Trabajo en curso

El frente activo es el rediseño y las correcciones de interfaz. Para avanzar
(«seguí con el plan», «avanzá», «hands-off»):

1. Leer [`docs/agente-ejecutor.md`](docs/agente-ejecutor.md), que explica el
   flujo, los comandos, las reglas de prueba y cuándo detenerse.
2. Tomar la primera tarea `[ ]` de la fase activa en
   [`docs/plan-ejecucion-2026-11-14.md`](docs/plan-ejecucion-2026-11-14.md).
   Las decisiones técnicas D-01 a D-28 de ese plan ya están tomadas: se
   aplican, no se rediscuten.

Entorno: Windows 10, PowerShell y Git Bash. El repo está en
`C:\Dev\proyectos\visor-md-v2`. `src/main.rs` supera las 21 000 líneas: ubicar
las funciones con `rg -n` y leer solo el rango necesario.
