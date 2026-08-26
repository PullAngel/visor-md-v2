# Fuentes embebidas

## Estado

La identidad usa Newsreader, Sora y JetBrains Mono. El commit estable contiene
una primera generación de tres fuentes. El working tree heredado regeneró esos
archivos y agregó Newsreader Italic, pero el pipeline exacto todavía debe
reconstruirse y automatizarse antes de aceptar los binarios definitivos.

No ejecutar nuevamente un subset a mano y sobrescribir estos archivos sin
preservar la evidencia actual.

## Archivos del working tree auditado

| Archivo | Uso | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `Sora.ttf` | Interfaz | 78.072 | `B7A8C6B38D6FE8C7C5F885D18723FA1C74A55642CA616957A1FD04B7215ED458` |
| `Newsreader.ttf` | Documento regular | 246.852 | `569B6D439BE49457D7B6206FC882409E3AC7B63B09BDE7110E185FAEFAC15C5D` |
| `Newsreader-Italic.ttf` | Documento cursivo | 273.700 | `22EBF5B00E350863EE3810B0B2D04EADDEE8B7D020EC63810B2F538A109EA207` |
| `JetBrainsMono.ttf` | Código | 95.708 | `299571DBB072C0FF6982F7A07E3DD769E3A1ED7573CD444EAC27095CB017A138` |

Total local: 694.332 bytes.

Los hashes documentan la evidencia heredada. No certifican todavía que el
pipeline sea reproducible o que estos sean los archivos finales.

## Procedencia reconstruida

- Google Fonts como fuente de descarga.
- Familias bajo SIL Open Font License 1.1.
- Subset realizado con fonttools.
- Cobertura latina y puntuación general.
- Fuentes variables conservando `fvar`.
- Primera generación sin `STAT`.
- Segunda generación conservando `STAT`.
- Newsreader Italic agregado para cursiva real.

`ttx -l` confirma que las dos variantes Newsreader locales contienen `STAT`,
`fvar`, `GPOS`, `GSUB` y `gvar`.

## Por qué se embeben

- apariencia consistente sin instalación del sistema;
- arranque offline;
- control de fallback y métricas;
- identidad editorial estable;
- no depender de una CDN.

Las fuentes embebidas cubren la identidad principal. Unicode fuera del subset
debe usar fallback del sistema. El objetivo no es incluir CJK completo dentro de
8 MB.

## Licencias

SIL OFL permite embeber, modificar y redistribuir las fuentes bajo sus
condiciones. Antes de cerrar la recuperación deben versionarse notices o copias
de licencia suficientes para la distribución y la SBOM.

No afirmar que una licencia guardada solo en una carpeta local `raw/` acompaña al
producto. La evidencia necesaria debe existir en el repositorio o en el paquete
de release.

## Pipeline requerido

La recuperación debe producir un script versionado que:

1. fije URLs o commits de origen;
2. verifique hashes de las fuentes completas;
3. registre versión de fonttools;
4. aplique una lista Unicode revisable;
5. conserve features de layout necesarias;
6. conserve IDs de nombre usados por fontique;
7. conserve `STAT` y ejes variables necesarios;
8. genere regular e italic correctos;
9. produzca hashes y tamaños de salida;
10. compruebe licencia y notices;
11. ejecute una prueba de registro y selección de estilo;
12. compruebe fallback para glifos no embebidos.

El script debe fallar si cambia inesperadamente una tabla, familia interna,
estilo, hash de entrada o cobertura.

## Casillas y símbolos

Newsreader no contiene los glifos de casilla requeridos. Las task list checkboxes
se dibujan con `tiny-skia` y no justifican agregar una fuente de símbolos.

Otros símbolos deben decidirse por el mismo criterio: fallback seguro o dibujo
simple antes de ampliar cobertura sin medir.

## Criterios de aceptación

- build selecciona Newsreader regular e italic correctamente;
- pesos variables funcionan;
- Sora y JetBrains Mono se resuelven por nombre;
- tests cubren caracteres españoles, puntuación, combinación y fallback;
- notices acompañan el producto;
- otra máquina puede reproducir archivos con los mismos hashes;
- tamaño y tiempo de registro quedan medidos.
