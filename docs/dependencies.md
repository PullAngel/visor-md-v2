# Dependencias y cadena de suministro

Este documento registra checkpoints verificables del grafo de dependencias. No
reemplaza una SBOM ni convierte una auditoría puntual en garantía permanente.

## Checkpoint del 26 de agosto de 2026

Entorno:

- `rustc 1.98.0`;
- `cargo-audit 0.22.2`;
- base RustSec en
  `a7bfe16948bf6f3ee25bdee4822209f87da21b80`;
- `Cargo.lock` del commit `da8616e`;
- 271 paquetes inspeccionados en el lockfile.

Resultado de `cargo audit`:

- cero vulnerabilidades conocidas;
- una advertencia de crate no mantenido;
- `cargo deny` no está instalado y su política sigue pendiente.

La advertencia es `RUSTSEC-2026-0192` para `ttf-parser 0.25.1`. No describe una
vulnerabilidad explotable concreta; informa que esa versión ya no recibe
mantenimiento. La ruta para todos los targets es:

```text
winit
└── sctk-adwaita
    └── ab_glyph
        └── owned_ttf_parser
            └── ttf-parser 0.25.1
```

Esta rama llega mediante la feature `wayland-csd-adwaita` incluida por defecto en
`winit`. No está activa en el binario Windows medido. Sí es relevante para las
decoraciones de cliente de una ventana Wayland en Linux.

## Decisión pendiente

Alternativas razonables:

1. conservarla temporalmente, monitorear RustSec y esperar migración upstream;
2. desactivar defaults de `winit` y seleccionar X11/Wayland sin CSD Adwaita;
3. retirarla cuando el chrome sin borde propio cubra mover, cerrar y redimensionar
   la ventana en Linux.

La segunda opción puede reducir superficie y deuda, pero podría dejar una ventana
Wayland sin controles utilizables. Requiere build y QA real en Linux antes de
cambiar el grafo. No se acepta la advertencia de forma permanente ni se elimina
una capacidad por intuición.

## Duplicados observados

`cargo tree -d` muestra versiones paralelas de:

- `syn 2` y `syn 3` por macros de ecosistemas distintos;
- `windows-sys 0.52` y `0.61` por `winit` y `softbuffer`;
- ramas de `phf_shared` de build y runtime.

Un duplicado no es automáticamente un defecto. Se investiga cuando aumenta el
binario, mantiene una versión vulnerable o complica la actualización. No se
fuerzan versiones transitivas sin comprobar compatibilidad.

## Gates pendientes

- política y archivo de configuración de `cargo deny`;
- inventario completo de licencias;
- SBOM reproducible por target;
- inventario de `unsafe`, C y C++ por target;
- auditoría Windows y Linux en CI;
- criterio de caducidad para aceptaciones temporales.

Cada release candidata debe repetir la auditoría contra una base RustSec actual y
registrar commit, fecha, target y resultado.
