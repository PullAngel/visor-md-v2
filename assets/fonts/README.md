# Fuentes embebidas

Tres familias variables, subconjunto latino, ~410 KB en total. La decisión de
diseño está en [`docs/design.md`](../../docs/design.md) ("Contraste
editorial"); esto documenta de dónde salen y cómo reproducir el subconjunto.

| Archivo | Familia | Uso | Licencia |
| --- | --- | --- | --- |
| `Sora.ttf` | Sora | Interfaz (aún sin chrome que la use) | SIL Open Font License 1.1 |
| `Newsreader.ttf` | Newsreader | Documento | SIL Open Font License 1.1 |
| `JetBrainsMono.ttf` | JetBrains Mono | Código | SIL Open Font License 1.1 |

Las tres son fuentes variables completas de [Google Fonts](https://github.com/google/fonts)
(`ofl/sora`, `ofl/newsreader`, `ofl/jetbrainsmono`), recortadas al subconjunto
latino con [fontTools](https://github.com/fonttools/fonttools). La licencia
completa de cada una queda archivada en `raw/` (no versionado, ver abajo) y su
texto es el mismo `OFL.txt` de siempre: permite embeber, modificar y
redistribuir, con la sola condición de no vender la fuente suelta con su
nombre original sin el permiso del autor.

## Cómo se generaron

```bash
# 1. Bajar la fuente variable completa (ejemplo con Sora)
curl -o raw/Sora.ttf \
  "https://raw.githubusercontent.com/google/fonts/main/ofl/sora/Sora%5Bwght%5D.ttf"

# 2. Recortar al subconjunto latino, conservando fvar (variable) y los
#    name IDs que fontique necesita para reconocer la familia
pip install fonttools

UNICODES="U+0000-00FF,U+0131,U+0152-0153,U+02BB-02BC,U+02C6,U+02DA,U+02DC,\
U+0300-0301,U+0303,U+0304,U+0308,U+0309,U+0323,U+0329,U+2000-206F,U+2074,\
U+20AC,U+2122,U+2191,U+2193,U+2212,U+2215,U+FEFF,U+FFFD"

python -m fontTools.subset raw/Sora.ttf \
  --output-file=Sora.ttf \
  --unicodes="$UNICODES" \
  --layout-features='*' \
  --name-IDs=1,2,4,6,16,17 \
  --drop-tables+=STAT
```

Mismo procedimiento para `newsreader` (`Newsreader%5Bopsz%2Cwght%5D.ttf`) y
`jetbrainsmono` (`JetBrainsMono%5Bwght%5D.ttf`).

**Por qué se conservan esos `name-IDs` y no se vacía la tabla entera:**
`Collection::register_fonts` de `fontique` (usado en `src/main.rs`,
`register_embedded_fonts`) identifica la familia leyendo el nombre desde el
propio archivo. Vaciar la tabla de nombres deja la fuente registrada pero
irreconocible por nombre, y el `FontFamily::List` del código cae en silencio
al genérico del sistema.

**Por qué se descarta `STAT`:** es metadata de presentación de los ejes
variables (para menús de selección de estilo en editores), que esta app no
usa. `fvar` (los ejes en sí, necesarios para variar el peso) se conserva.

## Por qué no está `raw/` en el repositorio

Las fuentes originales completas pesan ~750 KB combinadas, cubren miles de
glifos que este proyecto no necesita (cirílico, griego, la mayoría de CJK) y
son regenerables desde Google Fonts en cualquier momento. Versionarlas
duplicaría peso en el historial de git sin necesidad; el comando de arriba las
reconstruye igual.
