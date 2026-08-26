# Registros de benchmark

Esta carpeta conserva reportes generados, no números copiados a mano. Cada
archivo debe identificar commit, toolchain, plataforma, corpus, ejecutable,
muestras crudas y condiciones no controladas.

Generación:

```powershell
.\scripts\benchmark-startup.ps1 `
  -Document .\docs\architecture.md `
  -Runs 10 `
  -OutputPath .\docs\benchmarks\AAAA-MM-DD-windows.json
```

Una serie local no reemplaza CI ni una prueba en arranque frío. Los reportes se
versionan cuando sirven como checkpoint de una decisión o regresión.

Reportes actuales:

- [`2026-08-26-windows.json`](2026-08-26-windows.json): recuperación del Sprint
  1, diez aperturas y 240 cuadros de scroll.

La mediana de una cantidad par de muestras promedia los dos valores centrales.
P95 usa nearest rank y el máximo se conserva por separado, de modo que una
muestra lenta no desaparece por la elección del resumen estadístico.
