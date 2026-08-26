# Contribuir a Visor MD v2

El proyecto está en recuperación temprana. Las contribuciones deben preservar su
dirección de seguridad, tamaño y UX.

## Antes de empezar

Leer:

1. [`AGENTS.md`](AGENTS.md);
2. [`docs/status.md`](docs/status.md);
3. [`docs/product.md`](docs/product.md);
4. la documentación específica del cambio.

Abrir un issue o conversación antes de una dependencia, cambio arquitectónico o
función que altere producto, seguridad, diseño o alcance.

## Ramas y preservación

- `main` representa el estado estable.
- Usar una rama separada.
- Inspeccionar `git status` y `git diff` antes de editar.
- No descartar trabajo ajeno ni snapshots sin autorización.
- No mezclar recuperación, refactor y función nueva en un mismo commit.

## Cambios pequeños y demostrables

Una contribución debe explicar:

- problema real que resuelve;
- comportamiento esperado;
- riesgos;
- pruebas agregadas;
- impacto de tamaño o rendimiento cuando aplique;
- documentación actualizada.

Evitar comentarios narrativos, emojis decorativos y mensajes genéricos. Los
commits describen cambios del producto con lenguaje natural y concreto.

## Calidad

Base habitual:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
```

Agregar las gates específicas de [`docs/testing.md`](docs/testing.md). Un test
anterior o un binario cacheado no prueba el working tree actual.

## Dependencias

No agregar una dependencia solo por comodidad. Documentar:

- mantenimiento;
- licencia;
- features y grafo transitivo;
- `unsafe` y código nativo;
- red y filesystem;
- tamaño release antes y después;
- alternativa evaluada.

## Seguridad

Los reportes sensibles siguen [`SECURITY.md`](SECURITY.md). No incluir payloads
explotables o datos privados en issues públicos.

Una corrección de seguridad necesita una prueba de regresión que verifique la
propiedad cuando sea seguro publicarla.

## Documentación

Actualizar especificación, ADR, threat model, matriz y status cuando corresponda.
No afirmar que algo está implementado hasta que código y evidencia lo demuestren.
