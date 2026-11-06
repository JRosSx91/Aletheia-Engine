#!/usr/bin/env python3
"""Plots for geometric_analysis.csv. Geometry is sampled, not scored."""

from __future__ import annotations

import argparse
from pathlib import Path

import matplotlib.pyplot as plt
import plotly.express as px

from harmonic_csv import GEOMETRIC_FILENAME, GEOMETRIC_UNUSED_BY_FITNESS, load_geometric
from plot_common import CAPTION_FITNESS, apply_dark_mpl, empty_message, ensure_dir, save_html, save_png

WARNING = (
    "spatial_curvature, dimensional_ratios y temporal_rate se escriben en el CSV "
    "pero calculate_enhanced_fitness no los usa. Un patrón aquí no es un efecto 5D."
)


def plot_geometric(input_csv: Path, output_dir: Path, *, seed_label: str | None = None) -> list[Path]:
    frame = load_geometric(input_csv)
    output_dir = ensure_dir(output_dir)
    written: list[Path] = []
    if frame.empty:
        (output_dir / "README.txt").write_text(empty_message("geometric"), encoding="utf-8")
        return written

    subtitle = f"n={len(frame)}"
    if seed_label:
        subtitle += f" · rng-seed {seed_label}"

    apply_dark_mpl()
    fig, axes = plt.subplots(1, 2, figsize=(12, 5))
    axes[0].scatter(frame["spatial_curvature"], frame["fitness"], s=16, alpha=0.75, c="#6ea8fe")
    axes[0].set_xlabel("spatial_curvature (muestreada, ignorada)")
    axes[0].set_ylabel("fitness (heurística)")
    axes[0].set_title("Curvatura vs puntuación")
    axes[1].scatter(frame["temporal_rate"], frame["fitness"], s=16, alpha=0.75, c="#f0c674")
    axes[1].set_xlabel("temporal_rate (muestreada, ignorada)")
    axes[1].set_ylabel("fitness (heurística)")
    axes[1].set_title("Ritmo temporal vs puntuación")
    fig.suptitle(f"Parámetros geométricos no puntúan ({subtitle})")
    fig.text(0.01, 0.01, WARNING, fontsize=8)
    written.append(save_png(fig, output_dir / "geometric_unused_params.png"))

    interactive = px.scatter_matrix(
        frame,
        dimensions=GEOMETRIC_UNUSED_BY_FITNESS + ["fitness"],
        color="complexity_level",
        title=f"Matriz de parámetros geométricos (ignorados) y fitness · {subtitle}",
    )
    interactive.update_layout(
        template="plotly_dark",
        annotations=[
            dict(
                text=WARNING + " " + CAPTION_FITNESS,
                showarrow=False,
                xref="paper",
                yref="paper",
                x=0,
                y=-0.12,
                font=dict(size=11),
                align="left",
            )
        ],
        margin=dict(b=80),
    )
    written.append(save_html(interactive, output_dir / "geometric_scatter_matrix.html"))
    return written


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, default=Path(GEOMETRIC_FILENAME))
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--rng-seed", default=None)
    args = parser.parse_args()
    for path in plot_geometric(args.input, args.output_dir, seed_label=args.rng_seed):
        print(path)


if __name__ == "__main__":
    main()
