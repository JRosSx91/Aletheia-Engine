#!/usr/bin/env python3
"""Exploratory plots for landscape_data_enhanced.csv (map mode)."""

from __future__ import annotations

import argparse
from pathlib import Path

import matplotlib.pyplot as plt
import plotly.express as px

from harmonic_csv import MAP_FILENAME, SCHEMA_MAP, load_map
from plot_common import (
    CAPTION_FITNESS,
    CAPTION_INVERSE_ALPHA,
    apply_dark_mpl,
    empty_message,
    ensure_dir,
    save_html,
    save_png,
)


def plot_map(input_csv: Path, output_dir: Path, *, seed_label: str | None = None) -> list[Path]:
    frame = load_map(input_csv)
    output_dir = ensure_dir(output_dir)
    written: list[Path] = []
    if frame.empty:
        (output_dir / "README.txt").write_text(empty_message("map"), encoding="utf-8")
        return written

    subtitle = f"n={len(frame)}"
    if seed_label:
        subtitle += f" · rng-seed {seed_label}"

    apply_dark_mpl()
    fig, ax = plt.subplots(figsize=(8, 5))
    ax.hist(frame["fitness"], bins=24, color="#6ea8fe")
    ax.set_title(f"Distribución de la puntuación interna ({subtitle})")
    ax.set_xlabel("fitness (heurística)")
    ax.set_ylabel("universos escritos en el CSV")
    fig.text(0.01, 0.01, CAPTION_FITNESS, fontsize=8)
    written.append(save_png(fig, output_dir / "map_fitness_hist.png"))

    fig, ax = plt.subplots(figsize=(8, 6))
    sc = ax.scatter(
        frame["mass_up_quark"],
        frame["mass_down_quark"],
        c=frame["fitness"],
        cmap="viridis",
        s=18,
        alpha=0.75,
    )
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("masa quark up muestreada (kg)")
    ax.set_ylabel("masa quark down muestreada (kg)")
    ax.set_title(f"Masas muestreadas vs puntuación ({subtitle})")
    fig.colorbar(sc, ax=ax, label="fitness (heurística)")
    fig.text(
        0.01,
        0.01,
        "Las masas up/down se muestrean; el motor de hadrones usa masas quirales fijas ~310 MeV para u/d.",
        fontsize=8,
    )
    written.append(save_png(fig, output_dir / "map_quark_masses.png"))

    counts = frame["type"].value_counts()
    fig, ax = plt.subplots(figsize=(8, 5))
    ax.bar(counts.index.astype(str), counts.values, color="#8bd5ca")
    ax.set_title(f"Categorías de complexity_level ({subtitle})")
    ax.set_ylabel("conteo")
    ax.set_xlabel("etiqueta literaria de helpers.rs (no un observable)")
    written.append(save_png(fig, output_dir / "map_categories.png"))

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.scatter(
        frame["inverse_alpha_stored"],
        frame["fitness"],
        s=14,
        alpha=0.7,
        c="#f0c674",
    )
    ax.set_xlabel("columna `alpha` = 1/α almacenada (adimensional)")
    ax.set_ylabel("fitness (heurística)")
    ax.set_title(f"Recíproco almacenado vs puntuación ({subtitle})")
    fig.text(0.01, 0.01, CAPTION_INVERSE_ALPHA, fontsize=8)
    written.append(save_png(fig, output_dir / "map_inverse_alpha_vs_fitness.png"))

    hover = {
        "fitness": ":.6f",
        "inverse_alpha_stored": ":.6f",
        "alpha_si_reconstructed": ":.6e",
        "type": True,
        "complexity_level": True,
        "spatial_curvature": ":.4f",
        "temporal_rate": ":.4f",
    }
    interactive = px.scatter(
        frame,
        x="mass_up_quark",
        y="mass_down_quark",
        color="fitness",
        hover_data=hover,
        title=f"Mapa interactivo · puntuación interna · {subtitle}",
        labels={
            "mass_up_quark": "masa quark up muestreada (kg)",
            "mass_down_quark": "masa quark down muestreada (kg)",
            "fitness": "fitness (heurística)",
        },
    )
    interactive.update_layout(
        template="plotly_dark",
        annotations=[
            dict(
                text=(
                    f"{CAPTION_FITNESS} {CAPTION_INVERSE_ALPHA} "
                    f"spatial_curvature y temporal_rate se muestran en el tooltip; "
                    f"no entran en el fitness. {SCHEMA_MAP.notes}"
                ),
                showarrow=False,
                xref="paper",
                yref="paper",
                x=0,
                y=-0.18,
                font=dict(size=11),
                align="left",
            )
        ],
        margin=dict(b=90),
    )
    written.append(save_html(interactive, output_dir / "map_interactive.html"))
    return written


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, default=Path(MAP_FILENAME))
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--rng-seed", default=None)
    args = parser.parse_args()
    paths = plot_map(args.input, args.output_dir, seed_label=args.rng_seed)
    for path in paths:
        print(path)


if __name__ == "__main__":
    main()
