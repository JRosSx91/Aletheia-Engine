#!/usr/bin/env python3
"""Plots for alpha_fixed_analysis.csv. Variants c/ħ/ε₀ do not enter the engine."""

from __future__ import annotations

import argparse
from pathlib import Path

import matplotlib.pyplot as plt
import plotly.express as px

from harmonic_csv import ALPHA_FIXED_FILENAME, ALPHA_FIXED_UNUSED_BY_ENGINE, load_alpha_fixed
from plot_common import CAPTION_FITNESS, apply_dark_mpl, empty_message, ensure_dir, save_html, save_png


def plot_alpha_fixed(input_csv: Path, output_dir: Path, *, seed_label: str | None = None) -> list[Path]:
    frame = load_alpha_fixed(input_csv)
    output_dir = ensure_dir(output_dir)
    written: list[Path] = []
    if frame.empty:
        (output_dir / "README.txt").write_text(empty_message("alpha-fixed"), encoding="utf-8")
        return written

    subtitle = f"n={len(frame)}"
    if seed_label:
        subtitle += f" · rng-seed {seed_label}"

    apply_dark_mpl()
    fig, ax = plt.subplots(figsize=(8, 5))
    ax.hist(frame["fitness"], bins=16, color="#6ea8fe")
    ax.set_title(f"alpha-fixed · puntuación interna ({subtitle})")
    ax.set_xlabel("fitness (heurística)")
    ax.set_ylabel("muestras escritas (filtro fitness > 0.1)")
    fig.text(
        0.01,
        0.01,
        CAPTION_FITNESS + " El print «¡CONFIRMADO!» del CLI no es un test.",
        fontsize=8,
    )
    written.append(save_png(fig, output_dir / "alpha_fixed_fitness_hist.png"))

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.scatter(frame["derived_e"], frame["fitness"], s=18, alpha=0.75, c="#f0c674")
    ax.set_xscale("log")
    ax.set_xlabel("e derivada (C) — entra en CosmicLaw.e")
    ax.set_ylabel("fitness (heurística)")
    ax.set_title(f"e derivada vs puntuación ({subtitle})")
    fig.text(
        0.01,
        0.01,
        "e se obtiene de α_objetivo × 4π ε₀_var ħ_var c_var. El motor vuelve a calcular α con c, ħ, ε₀ SI.",
        fontsize=8,
    )
    written.append(save_png(fig, output_dir / "alpha_fixed_derived_e.png"))

    melted = frame.melt(
        id_vars=["sample_id", "fitness"],
        value_vars=ALPHA_FIXED_UNUSED_BY_ENGINE,
        var_name="variant",
        value_name="sampled_value",
    )
    interactive = px.scatter(
        melted,
        x="sampled_value",
        y="fitness",
        facet_col="variant",
        title=f"Variantes muestreadas que el motor físico ignora · {subtitle}",
        labels={
            "sampled_value": "valor muestreado (SI de esa constante × factor)",
            "fitness": "fitness (heurística)",
        },
        log_x=True,
    )
    interactive.update_layout(
        template="plotly_dark",
        annotations=[
            dict(
                text=(
                    "c_variant, hbar_variant y epsilon0_variant NO se pasan a "
                    "AdvancedPhysicsEngine. Cualquier correlación con fitness no "
                    "demuestra «primacía de alpha»."
                ),
                showarrow=False,
                xref="paper",
                yref="paper",
                x=0,
                y=-0.22,
                font=dict(size=11),
                align="left",
            )
        ],
        margin=dict(b=100),
    )
    written.append(save_html(interactive, output_dir / "alpha_fixed_unused_variants.html"))
    return written


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, default=Path(ALPHA_FIXED_FILENAME))
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--rng-seed", default=None)
    args = parser.parse_args()
    for path in plot_alpha_fixed(args.input, args.output_dir, seed_label=args.rng_seed):
        print(path)


if __name__ == "__main__":
    main()
