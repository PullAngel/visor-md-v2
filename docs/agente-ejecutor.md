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

## Trampas de herramientas (aprendidas ejecutando las fases 1 a 2)

- **Clippy y el código de salida.** `cargo clippy ... | tail` oculta el código
  de salida y un `&&` después comitea igual con errores. Verificar así:

  ```bash
  cargo clippy --all-targets -- -D warnings >/dev/null 2>&1; echo "clippy=$?"
  ```

  Debe imprimir `clippy=0`. El perfil de pruebas también cuenta: una función
  `#[cfg(test)]` sin usar falla con `-D warnings`.
- **Editar archivos.** La herramienta Edit exige haber leído el archivo antes.
  Para cambios de varios fragmentos, un script de Python con
  `edit(ruta, [(viejo, nuevo), ...])` que verifique que cada `viejo` es único
  antes de escribir. Escribir ese script con la herramienta Write: el shell
  interpreta las barras invertidas de un heredoc y cambia `\n` por un salto
  real dentro de las cadenas de Rust.
- **rustfmt reformatea.** Después de `cargo fmt`, una línea de prueba puede
  quedar partida en varias; los anclajes de ediciones posteriores hay que
  releerlos del archivo, no recordarlos.
- **Prueba en vivo.** La build normal borra la posición del puntero cuando
  Windows avisa que el cursor real salió de la ventana, y desde la tarea 1.3 un
  clic sin posición no hace nada. La herramienta de evaluación necesita la
  build con `scripts/evaluacion/build-evaluacion.patch`:

  ```bash
  git worktree add --detach ../visor-md-v2-eval HEAD
  cd ../visor-md-v2-eval && git apply ../visor-md-v2/scripts/evaluacion/build-evaluacion.patch
  CARGO_TARGET_DIR=../visor-md-v2/target-eval cargo build --release
  ```

  Al terminar: `git worktree remove --force ../visor-md-v2-eval` y borrar
  `target-eval`.
- **Primer arranque en vivo.** Un perfil limpio abre el diálogo de privacidad
  de la recuperación: pulsar «Entendido» (centro de la ventana) antes de usar
  la app.
- **Tipeo de la herramienta.** `type` a veces duplica o intercala letras justo
  después de un clic o de Enter. No es un defecto de la app; repetir con
  pausas.
- **Pruebas de imagen.** Si un cambio visual es deliberado:
  `VISOR_BLESS=1 cargo test visual`, mirar las imágenes nuevas con la
  herramienta de lectura de imágenes y comitearlas con el cambio. Solo
  cambian las imágenes de las escenas afectadas.
