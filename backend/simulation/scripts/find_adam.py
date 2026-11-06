#!/usr/bin/env python3
"""Report the highest-fitness row in a map CSV. Does not invent missing CosmicLaw fields."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from harmonic_csv import MAP_FILENAME, load_map


def find_adam(input_csv: Path, output_json: Path | None = None) -> dict:
    frame = load_map(input_csv, allow_empty=False)
    best = frame.loc[frame["fitness"].idxmax()]
    record = {
        "source_csv": str(input_csv),
        "note": (
            "Row with maximum internal fitness. Other CosmicLaw fields are not in this CSV "
            "and are not filled with laboratory placeholders. Column alpha is 1/α."
        ),
        "fitness": float(best["fitness"]),
        "complexity_level": int(best["complexity_level"]),
        "type": str(best["type"]),
        "inverse_alpha_stored": float(best["inverse_alpha_stored"]),
        "alpha_si_reconstructed": float(best["alpha_si_reconstructed"]),
        "mass_up_quark_kg": float(best["mass_up_quark"]),
        "mass_down_quark_kg": float(best["mass_down_quark"]),
        "spatial_curvature_unused_by_fitness": float(best["spatial_curvature"]),
        "temporal_rate_unused_by_fitness": float(best["temporal_rate"]),
    }
    if output_json is not None:
        output_json.parent.mkdir(parents=True, exist_ok=True)
        output_json.write_text(json.dumps(record, indent=2), encoding="utf-8")
    return record


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, default=Path(MAP_FILENAME))
    parser.add_argument("--output-json", type=Path, default=None)
    args = parser.parse_args()
    record = find_adam(args.input, args.output_json)
    print("--- Fila de mayor fitness (métrica interna) ---")
    print(json.dumps(record, indent=2))


if __name__ == "__main__":
    main()
