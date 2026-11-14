# Guía del agente ejecutor

Esta guía es para el modelo que ejecuta
[`plan-ejecucion-2026-11-14.md`](plan-ejecucion-2026-11-14.md) sin
acompañamiento (por ejemplo, Sonnet 5.5). Las decisiones ya están tomadas
en ese plan (D-01 a D-28). Esta guía dice **cómo trabajar**.
[`AGENTS.md`](../AGENTS.md) sigue vigente y tiene prioridad si algo choca.

## Al empezar una sesión

1. `git -C C:\Dev\proyectos\visor-md-v2 status` y `git log --oneline -5`.
   - Si hay cambios sin confirmar que no son tuyos, no los toques: anotalo y
     detenete.
2. Abrir la tabla «Estado» del plan, identificar la fase activa y tomar su
   primera tarea `[ ]`.
3. Leer **solo** las decisiones D-xx que cita la tarea y las funciones que
   nombra. `src/main.rs` tiene más de 21 000 líneas: nunca leerlo entero.
   Ubicar con `rg -n "fn nombre" src/` y leer de 40 a 150 líneas alrededor.

## Ciclo por tarea

1. **Ubicar.** `rg -n` de cada función citada y lectura del rango.
2. **Prueba primero**, cuando la tarea es de comportamiento:
   - escribirla en el módulo `#[cfg(test)]` del archivo que corresponde;
   - comprobar que falla: `cargo test nombre_de_la_prueba`.
3. **Cambiar** con la herramienta de edición. Diffs chicos: no reformatear ni
   renombrar lo que la tarea no pide.
4. **Verificar**, en este orden y deteniéndose en el primer fallo:

   ```bash
   cargo fmt
   cargo clippy --all-targets -- -D warnings
   cargo test
   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/check-docs.ps1
   ```

5. **Visual**, si la tarea cambia el dibujo y la fase 2 ya está hecha:
   - correr `cargo test visual`;
   - si falla a propósito, `VISOR_BLESS=1 cargo test visual` y revisar con
     `kompari review tests/visual/referencia target/visual/actual`;
   - no aprobar referencias sin mirarlas.
6. **Commit y push** en `main`:
   - una tarea por commit, con un asunto en español en imperativo de tercera
     persona («Corrige…», «Agrega…»);
   - el cuerpo dice qué cambia para la persona usuaria;
   - última línea: `Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>`;
   - después, `git push origin main`.
7. **Marcar la tarea** `[x] (hash)` en el plan, en el mismo commit o en uno
   chico de documentación.

## Al cerrar una fase

1. Compilar el release:

   ```bash
   cargo build --release
   ```

2. Capturas en vivo con [`scripts/evaluacion/`](../scripts/evaluacion/README.md):
   - con la build normal: copiar `target/release/visor-md.exe` como
     `<carpeta>/prueba/prueba.exe`;
   - comandos `launch`, `click`, `shot`, etc.;
   - si la tarea necesita abrir archivos sin diálogo, usar la build de
     evaluación.

   Tomar noche y día, a 640×480 y 1100×720, de lo que cambió en la fase.
3. `design-critique` sobre las capturas, si el plugin Design está disponible.
   Corregir lo S2+ antes de cerrar y registrar lo S3 o S4 como tareas de la
   fase 8.
4. `/code-review` sobre el diff de la fase:

   ```bash
   git diff <primer-commit-de-la-fase>^..HEAD
   ```

5. Ejecutable para el propietario: copiar el release a
   `C:\Dev\archivo\visor-md-v2-ejecutable\VisorMD-v2.exe`, renombrando antes el
   anterior a `VisorMD-v2-anterior-AAAA-MM-DD.exe`.
6. Actualizar «Estado» del plan: marcar la fase hecha y activar la siguiente
   según «Secuencia».
7. Avisarle al propietario en 5 a 8 líneas qué puede probar y qué no se
   verificó.

## Reglas de seguridad al probar

- Nunca mover el mouse real ni enviar teclado real (`SendInput`,
  `SetCursorPos`). Solo `scripts/evaluacion/ui.ps1`, que usa mensajes de
  ventana.
- Solo cerrar procesos con el nombre de tu propia copia (`prueba`, `qa`,
  `usuario`); nunca `VisorMD-v2`, que puede ser del propietario.
- No pulsar enlaces web en pruebas: abren el navegador real.
- El portapapeles es el real: probar Copiar solo cuando la tarea lo pida.

## Cuándo detenerse o escalar

Parar la tarea, anotarla `[~]` con el motivo y seguir con la siguiente tarea
independiente cuando:

1. Una decisión D-xx resulta imposible tal como está escrita. Anotar por qué y
   qué alternativa conservadora se propone, **sin implementarla** si cambia
   comportamiento visible o arquitectura.
2. Después de tres intentos sustancialmente distintos, la prueba no pasa.
3. La tarea pediría una dependencia nueva. Las dependencias requieren ADR y
   aprobación del propietario.
4. El binario release supera 7,5 MB. Hoy pesa unos 3,6 MB.
5. Hay riesgo de perder datos del documento: cualquier cambio en guardado,
   recuperación o VFS que no esté cubierto por una prueba.

El turno termina solo cuando todo lo restante depende del propietario
(ver [`AGENTS.md`](../AGENTS.md#relación-con-el-propietario)).

## Trampas conocidas del código

- **Entrada de texto.** En Windows el texto llega en `KeyEvent::text`, no en
  `Ime::Commit`, salvo en composición IME. Cualquier campo nuevo usa
  `typed_text` (D-04).
- **Geometría.** El dibujo y el clic deben usar la misma función de
  rectángulos. Desde la fase 2 es `ui_target_at`. No calcular rectángulos a
  mano en un solo lado.
- **Despertar el bucle de eventos.** Todo lo animado o diferido agenda su
  próximo cuadro en `about_to_wait` con una fecha límite. No usar hilos ni
  `sleep`.
- **`render_frame` desarma `self`** en campos sueltos para el préstamo del
  pixmap (`let App { pixmap, … } = self;`). Si un campo nuevo se necesita
  dentro del dibujo, agregarlo a ese desarme.
- **CRLF.** El editor conserva el fin de línea del documento. Las inserciones
  usan el `eol` que reciben; nunca escribir `"\n"` fijo en `editor.rs`.
- **`APP_ACTIONS`** tiene un tamaño fijo y la prueba del catálogo limita su
  crecimiento. Si se agrega una acción, actualizar el tamaño del arreglo y el
  límite de la prueba, con un comentario.
- **Pruebas lentas.** `cargo test` completo tarda alrededor de un minuto.
  Durante la iteración, filtrar por nombre.

## Re-evaluación

La fase 9 repite exactamente la evaluación del 14 de noviembre:

- Prompt de la QA: [`scripts/evaluacion/prompt-qa.md`](../scripts/evaluacion/prompt-qa.md).
- Prompt de la usuaria: [`scripts/evaluacion/prompt-usuario.md`](../scripts/evaluacion/prompt-usuario.md).
- Hoja de requerimientos: [`evaluaciones/2026-11-14/hoja-requerimientos.md`](evaluaciones/2026-11-14/hoja-requerimientos.md).

Se lanzan dos subagentes en paralelo, cada uno con su copia (`qa` y
`usuario`) y un perfil vacío. En los prompts, `<EVAL>` se reemplaza por la ruta
absoluta de la carpeta de trabajo de la evaluación.
