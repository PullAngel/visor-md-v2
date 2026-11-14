# Evaluación de usabilidad reproducible

Herramientas para repetir la evaluación del 14 de noviembre de 2026
([`docs/evaluacion-usabilidad-2026-11-14.md`](../../docs/evaluacion-usabilidad-2026-11-14.md)):
una QA de UI/UX y una usuaria novata, cada una con su propia copia de la app.
No se mueven el mouse ni el teclado reales, y los diálogos de Windows no
aparecen en pantalla.

## Contenido

- `ui.ps1`: maneja una copia de la app por mensajes de ventana. Tiene
  comandos `launch`, `shot`, `move`, `click`, `dblclick`, `rclick`, `drag`,
  `wheel`, `key`, `type`, `answer`, `resize` y `close`. La ayuda está en la
  cabecera del archivo y en el prompt de cada perfil, que se describe en la
  evaluación.
- `build-evaluacion.patch`: tres cambios **solo para la build de evaluación**.
  Nunca se aplican en `main`.
  1. Los diálogos de archivo leen la respuesta de
     `%LOCALAPPDATA%\eval-dialogo.txt`, que se escribe con `ui.ps1 -Cmd answer`.
  2. Con `VISOR_EVAL=1`, el puntero simulado sobrevive a `CursorLeft`.
  3. Un clic sin puntero conocido no redimensiona la ventana. Este cambio
     también entra en `main` como corrección en la fase 1 del plan.
- `apuntes/`: carpeta de notas de muestra usada por los dos perfiles.
- La hoja de requerimientos de la QA está en
  [`docs/evaluaciones/2026-11-14/hoja-requerimientos.md`](../../docs/evaluaciones/2026-11-14/hoja-requerimientos.md).

## Preparación

```powershell
git worktree add --detach ..\visor-md-v2-eval HEAD
cd ..\visor-md-v2-eval
git apply ..\visor-md-v2\scripts\evaluacion\build-evaluacion.patch
cargo build --release
```

Después, en una carpeta de trabajo **fuera del repo**:

- copiar `ui.ps1`;
- crear una subcarpeta por perfil (`qa\`, `usuario\`) con el ejecutable
  renombrado como `qa.exe` o `usuario.exe`, una copia de `apuntes\` y una
  carpeta `capturas\`.

`ui.ps1` crea un perfil limpio por copia en `<perfil>\perfil`.

## Límites conocidos

- Ctrl, Alt y Shift combinados no llegan a la app.
- `type` no escribe mayúsculas ni vocales con tilde.
- Si se pulsa un enlace web, la app abre el navegador real del sistema.
