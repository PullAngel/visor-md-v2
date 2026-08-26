"""Reproduce y verifica las fuentes embebidas de Visor MD.

Requiere Python 3 y fonttools 4.63.0. Sin argumentos trabaja en una carpeta
temporal y no modifica el repositorio. ``--write`` reemplaza los cuatro
artefactos solo después de verificar hashes de entrada y salida.
"""

from __future__ import annotations

import argparse
import hashlib
import os
import subprocess
import sys
import tempfile
import urllib.parse
import urllib.request
from dataclasses import dataclass
from pathlib import Path

import fontTools


GOOGLE_FONTS_COMMIT = "6a003b5eb672dc8bf5bff5937cf5863f8b175445"
FONTTOOLS_VERSION = "4.63.0"
UNICODES = (
    "U+0000-00FF,U+0131,U+0152-0153,U+02BB-02BC,U+02C6,U+02DA,U+02DC,"
    "U+0300-0301,U+0303,U+0304,U+0308,U+0309,U+0323,U+0329,U+2000-206F,"
    "U+2074,U+20AC,U+2122,U+2190-2193,U+2212,U+2215,U+FEFF,U+FFFD"
)


@dataclass(frozen=True)
class FontJob:
    source: str
    output: str
    source_size: int
    source_sha256: str
    output_sha256: str


JOBS = (
    FontJob(
        "ofl/sora/Sora[wght].ttf",
        "Sora.ttf",
        111_400,
        "84FF7096AE3EC6C8BE47D906D1A0BA4DE7F2CE78C615275C77301964A316E16C",
        "B7A8C6B38D6FE8C7C5F885D18723FA1C74A55642CA616957A1FD04B7215ED458",
    ),
    FontJob(
        "ofl/newsreader/Newsreader[opsz,wght].ttf",
        "Newsreader.ttf",
        451_664,
        "8A08D13F8A6C0D51BE379A60AF84F945F65369A67E509EE3C3BDCC421254D7C1",
        "569B6D439BE49457D7B6206FC882409E3AC7B63B09BDE7110E185FAEFAC15C5D",
    ),
    FontJob(
        "ofl/newsreader/Newsreader-Italic[opsz,wght].ttf",
        "Newsreader-Italic.ttf",
        495_684,
        "796668611F80B64D5ADF182FDE3B6F29ED83B4E7CBEC7B96937E84AC01364792",
        "22EBF5B00E350863EE3810B0B2D04EADDEE8B7D020EC63810B2F538A109EA207",
    ),
    FontJob(
        "ofl/jetbrainsmono/JetBrainsMono[wght].ttf",
        "JetBrainsMono.ttf",
        187_208,
        "48715A42EC242C21E9F02692891E147D022299A52E48D5E413E1A942193FFEDA",
        "299571DBB072C0FF6982F7A07E3DD769E3A1ED7573CD444EAC27095CB017A138",
    ),
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest().upper()


def source_url(path: str) -> str:
    encoded = urllib.parse.quote(path, safe="/")
    return (
        "https://raw.githubusercontent.com/google/fonts/"
        f"{GOOGLE_FONTS_COMMIT}/{encoded}"
    )


def download(job: FontJob, destination: Path) -> None:
    request = urllib.request.Request(
        source_url(job.source),
        headers={"User-Agent": "visor-md-font-reproducer/1"},
    )
    with urllib.request.urlopen(request, timeout=60) as response:
        data = response.read(job.source_size + 1)
    if len(data) != job.source_size:
        raise RuntimeError(
            f"tamaño de entrada inesperado para {job.source}: {len(data)}"
        )
    destination.write_bytes(data)
    actual = sha256(destination)
    if actual != job.source_sha256:
        raise RuntimeError(
            f"hash de entrada inesperado para {job.source}: {actual}"
        )


def subset(source: Path, destination: Path) -> None:
    subprocess.run(
        [
            sys.executable,
            "-m",
            "fontTools.subset",
            os.fspath(source),
            f"--output-file={destination}",
            f"--unicodes={UNICODES}",
            "--layout-features=*",
            "--name-IDs=1,2,4,6,16,17",
        ],
        check=True,
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--write",
        action="store_true",
        help="reemplaza assets/fonts solo si todos los hashes coinciden",
    )
    args = parser.parse_args()

    if fontTools.__version__ != FONTTOOLS_VERSION:
        raise RuntimeError(
            f"se requiere fonttools {FONTTOOLS_VERSION}; se encontró "
            f"{fontTools.__version__}"
        )

    repo = Path(__file__).resolve().parents[1]
    assets = repo / "assets" / "fonts"
    generated: list[tuple[FontJob, Path]] = []

    with tempfile.TemporaryDirectory(prefix="visor-md-fonts-") as raw_temp:
        temporary = Path(raw_temp)
        for index, job in enumerate(JOBS):
            source = temporary / f"source-{index}.ttf"
            output = temporary / job.output
            download(job, source)
            subset(source, output)
            actual = sha256(output)
            if actual != job.output_sha256:
                raise RuntimeError(
                    f"salida no reproducible para {job.output}: {actual}"
                )
            generated.append((job, output))

        for job, output in generated:
            checked_in = assets / job.output
            if checked_in.exists() and sha256(checked_in) != job.output_sha256:
                raise RuntimeError(
                    f"el archivo versionado no coincide: {checked_in}"
                )
            if args.write:
                replacement = assets / f".{job.output}.new"
                replacement.write_bytes(output.read_bytes())
                os.replace(replacement, checked_in)
            print(f"OK {job.output} {job.output_sha256}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
