#!/usr/bin/env python3
"""Compare two JSON genomes if the user supplies them. The CLI does not emit these files."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

import pandas as pd


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--champion", type=Path, required=True)
    parser.add_argument("--reference", type=Path, required=True)
    args = parser.parse_args()

    missing = [str(p) for p in (args.champion, args.reference) if not p.is_file()]
    if missing:
        print(
            "Archivos ausentes: "
            + ", ".join(missing)
            + ". El CLI actual no genera champion_genome.json ni our_universe.json. "
            "No se inventan genomas.",
            file=sys.stderr,
        )
        sys.exit(2)

    champion = pd.Series(json.loads(args.champion.read_text(encoding="utf-8")), name="champion")
    reference = pd.Series(json.loads(args.reference.read_text(encoding="utf-8")), name="reference")
    table = pd.concat([champion, reference], axis=1)
    print(table.to_string())
    print(
        "\nSin interpretación cosmológica: esta tabla solo confronta JSON suministrados por el usuario."
    )


if __name__ == "__main__":
    main()
