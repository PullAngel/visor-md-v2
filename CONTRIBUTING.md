# Contribuir a Visor MD v2

> **Contribuciones de código: cerradas por ahora.** Se aceptan issues con
> reportes, ideas y feedback de uso, pero todavía no se reciben pull requests
> con código. El proyecto está bajo la
> [PolyForm Noncommercial License 1.0.0](LICENSE) y no tiene un acuerdo de
> contribución; mientras no lo tenga, el código lo escribe solo su autor. Si esto
> cambia, se anunciará aquí antes de aceptar el primer aporte.

El proyecto está en desarrollo activo y todavía no tiene una release estable.
Las contribuciones deben preservar su dirección de seguridad, fidelidad de datos,
tamaño y UX. El [dashboard](docs/project-dashboard.md) resume capacidades y deuda.

## Antes de empezar

Leer:

1. [`AGENTS.md`](AGENTS.md);
2. [`docs/status.md`](docs/status.md);
3. [`docs/product.md`](docs/product.md);
4. la documentación específica del cambio.

Abrir un issue o conversación antes de una dependencia, cambio arquitectónico o
función que altere producto, seguridad, diseño o alcance.

## Ramas y preservación

- `main` es la rama principal de desarrollo; no implica una release estable.
- Los pull requests externos con código no se aceptan por ahora. El workspace
  del propietario trabaja directamente en `main` según el acuerdo vigente.
- La rama `archive/claude-pre-codex` es histórica y se conserva intacta.
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
cargo fmt -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked --release
```

Aplicar los gates específicos de [`docs/testing.md`](docs/testing.md). Un test
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
