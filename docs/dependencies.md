# Dependencias y cadena de suministro

Este documento registra checkpoints verificables del grafo de dependencias. No
reemplaza una SBOM ni convierte una auditoría puntual en garantía permanente.

## Cambio en curso: editor, guardado y diálogos nativos

La primera parte del Sprint B incorpora tres dependencias directas, aún en
validación de cadena de suministro y presupuesto release:

- `atomicwrites 0.4.4` crea el temporal junto al destino, sincroniza el archivo
  y lo reemplaza atómicamente. Visor MD además compara los bytes completos de
  la versión abierta antes de invocarlo; por tanto un conflicto externo no se
  sobrescribe por accidente.
- `ropey 1.6.1` será el buffer UTF-8 del editor. Sus operaciones se expresan
  en límites de caracteres y están diseñadas para ediciones no contiguas sin
  desplazar un documento completo en cada pulsación.
- `rfd 0.17.2` se reserva para los diálogos nativos de abrir, guardar y elegir
  carpeta. No recibe contenido de documentos ni otorga permisos secundarios.

Antes de cerrar Sprint B se registrarán las licencias y transitivas reales de
este lockfile, `cargo audit`, SBOM, build Windows/Linux y delta de tamaño. Si
una dependencia no supera esos gates, no se sustituye silenciosamente por una
escritura no atómica ni por una UI que exponga rutas arbitrarias.

### Medición inicial del 27 de agosto de 2026

Con `atomicwrites`, `rfd` y `ropey` resueltos en el lockfile, el build release
Windows produjo `visor-md.exe` de 3.144.704 bytes (3,00 MiB). `cargo audit`
revisó 286 paquetes: cero vulnerabilidades conocidas y la misma advertencia
permitida `RUSTSEC-2026-0192` de `ttf-parser` transitivo ya explicada abajo.
Faltan SBOM regenerado, licencias por target y evidencia Linux antes de cerrar
la validación de estas dependencias.

## Cambio pendiente de validar: portapapeles de texto

La rama de recuperación incorpora `arboard 3.6.1` con
`default-features = false`. Es una dependencia directa, publicada bajo
`MIT OR Apache-2.0`; su API usada por Visor MD es `Clipboard::new`, `set_text`
y `get_text` tras un atajo o acción de menú explícita de la persona.

Las funciones por defecto de `arboard` incluirían imágenes y sus decodificadores.
Se mantienen desactivadas: Visor MD no lee ni escribe imágenes al portapapeles.
El lockfile agrega ocho paquetes para atender los portapapeles de las plataformas
soportadas, incluidos `clipboard-win` en Windows y bibliotecas Objective-C para
macOS futuro. El `cargo audit` posterior inspeccionó 279 paquetes: no encontró
vulnerabilidades y mantuvo solo la advertencia previa de `ttf-parser` no
mantenido. `sbom.cdx.json` fue regenerado con el nuevo grafo. Falta revisión de
licencias y QA por target antes de elevar la cadena completa a estado verificado.

En X11 y Wayland la aplicación puede ser propietaria del texto copiado mientras
vive. Por ello la instancia de portapapeles se mantiene en el estado de la app
después de una copia, pero no se consulta ni se transmite su contenido. La única
lectura permitida es el texto solicitado por `Ctrl+V` o la acción de pegar; no
hay observador, historial ni lectura en segundo plano.

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
- revisión de compatibilidad y notices de licencias;
- validación independiente del SBOM;
- inventario de `unsafe`, C y C++ por target;
- auditoría Windows y Linux en CI;
- criterio de caducidad para aceptaciones temporales.

Cada release candidata debe repetir la auditoría contra una base RustSec actual y
registrar commit, fecha, target y resultado.

## SBOM

`scripts/generate-sbom.ps1` genera `sbom.cdx.json` en formato CycloneDX 1.6 a
partir del grafo resuelto por `cargo metadata --locked`. Incluye todos los
paquetes de `Cargo.lock`, también los específicos de otras plataformas, y sus
relaciones. El archivo no lleva fecha, rutas locales ni identificadores
aleatorios, por lo que dos ejecuciones sobre el mismo lockfile producen el mismo
contenido.

```powershell
.\scripts\generate-sbom.ps1
```

Cargo conserva dos expresiones históricas de licencia con `/`. El generador las
normaliza a su significado SPDX con `OR`; no cambia ni selecciona una licencia.
El SBOM permite localizar componentes y versiones, pero no reemplaza `cargo
audit`, revisión de licencias ni análisis de código nativo.
