# Fuentes embebidas

## Estado

La identidad usa Newsreader, Sora y JetBrains Mono. El commit estable contiene
una primera generación de tres fuentes. El working tree heredado regeneró esos
archivos y agregó Newsreader Italic. La recuperación reconstruyó el proceso y
reprodujo los cuatro binarios byte por byte.

No ejecutar un subset a mano. Usar `scripts/subset-fonts.py`, que valida entradas
y salidas antes de permitir un reemplazo explícito.

## Archivos del working tree auditado

| Archivo | Uso | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `Sora.ttf` | Interfaz | 78.072 | `B7A8C6B38D6FE8C7C5F885D18723FA1C74A55642CA616957A1FD04B7215ED458` |
| `Newsreader.ttf` | Documento regular | 246.852 | `569B6D439BE49457D7B6206FC882409E3AC7B63B09BDE7110E185FAEFAC15C5D` |
| `Newsreader-Italic.ttf` | Documento cursivo | 273.700 | `22EBF5B00E350863EE3810B0B2D04EADDEE8B7D020EC63810B2F538A109EA207` |
| `JetBrainsMono.ttf` | Código | 95.708 | `299571DBB072C0FF6982F7A07E3DD769E3A1ED7573CD444EAC27095CB017A138` |

Total local: 694.332 bytes.

Los cuatro hashes fueron reproducidos con el script versionado.

## Procedencia verificada

- Google Fonts, commit
  `6a003b5eb672dc8bf5bff5937cf5863f8b175445`.
- Familias bajo SIL Open Font License 1.1.
- Subset realizado con fonttools 4.63.0.
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
condiciones. Los avisos de copyright y el texto completo que acompaña a los
subconjuntos están en [`LICENSE.txt`](LICENSE.txt).

## Pipeline reproducible

[`../../scripts/subset-fonts.py`](../../scripts/subset-fonts.py):

- fija commit y hashes de las fuentes completas;
- exige una versión conocida de fonttools;
- conserva las features, nombres, `STAT` y ejes variables necesarios;
- verifica los hashes de salida y los archivos versionados;
- trabaja en un directorio temporal de forma predeterminada;
- solo reemplaza artefactos mediante `--write` explícito.

Verificación sin modificar el repositorio:

```powershell
python scripts/subset-fonts.py
```

La reproducción descarga entradas de build. La aplicación final no las descarga
ni realiza conexiones para cargar tipografía.

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

La reproducción de hashes y los notices están cerrados. La selección visual,
fallback y cobertura completa siguen en la lista de QA del Sprint 1.
