#!/usr/bin/env python3
"""Plots for cosmic_library.csv. Book 137 is marked, not privileged a priori."""

from __future__ import annotations

import argparse
from pathlib import Path

import matplotlib.pyplot as plt
import plotly.express as px

from harmonic_csv import LIBRARY_FILENAME, load_library
from plot_common import CAPTION_FITNESS, apply_dark_mpl, empty_message, ensure_dir, save_html, save_png

BOOK_137 = 137


def plot_library(input_csv: Path, output_dir: Path, *, seed_label: str | None = None) -> list[Path]:
    frame = load_library(input_csv)
    output_dir = ensure_dir(output_dir)
    written: list[Path] = []
    if frame.empty:
        (output_dir / "README.txt").write_text(empty_message("library"), encoding="utf-8")
        return written

    subtitle = f"n={len(frame)}"
    if seed_label:
        subtitle += f" · rng-seed {seed_label}"

    apply_dark_mpl()
    colors = ["#f0c674" if n == BOOK_137 else "#6ea8fe" for n in frame["alpha_denominator"]]
    fig, ax = plt.subplots(figsize=(9, 5))
    ax.bar(frame["alpha_denominator"].astype(int).astype(str), frame["story_quality"], color=colors)
    ax.axhline(0.8, color="#ff6b6b", ls="--", lw=1, label="umbral literario interno 0.8 («obra maestra»)")
    ax.set_xlabel("denominador N del libro (intención α = 1/N; no es α del motor)")
    ax.set_ylabel("story_quality (fitness × bonus narrativo, cap 2)")
    ax.set_title(f"Biblioteca cósmica · calidad narrativa interna ({subtitle})")
    ax.legend()
    fig.text(
        0.01,
        0.01,
        "Amarillo = libro 137 (marcado para localizarlo, no porque el código lo privilegie). "
        + CAPTION_FITNESS,
        fontsize=8,
    )
    written.append(save_png(fig, output_dir / "library_story_quality.png"))

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.scatter(frame["alpha_denominator"], frame["stellar_score"], c="#8bd5ca", s=40)
    ax.set_xlabel("N (denominador)")
    ax.set_ylabel("stellar_score (heurística 0–1)")
    ax.set_title(f"stellar_score por libro ({subtitle})")
    fig.text(
        0.01,
        0.01,
        "stellar_score usa Jeans + vida MS con T y ρ fijos. 0 no significa «sin estrellas observadas».",
        fontsize=8,
    )
    written.append(save_png(fig, output_dir / "library_stellar_score.png"))

    interactive = px.scatter(
        frame,
        x="alpha_denominator",
        y="story_quality",
        color="finale_type",
        size="dramatic_events_count",
        hover_data={
            "narrative_depth": True,
            "reproducibility": ":.4f",
            "theoretical_proton_mass_kg": ":.4e",
            "stellar_score": ":.4f",
        },
        title=f"Libros de la biblioteca · métricas internas · {subtitle}",
        labels={
            "alpha_denominator": "N (libro α=1/N, intención de muestreo)",
            "story_quality": "story_quality (heurística)",
        },
    )
    interactive.add_vline(
        x=BOOK_137,
        line_dash="dash",
        line_color="#f0c674",
        annotation_text="N=137 (marcado)",
    )
    interactive.update_layout(template="plotly_dark")
    written.append(save_html(interactive, output_dir / "library_interactive.html"))
    return written


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, default=Path(LIBRARY_FILENAME))
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--rng-seed", default=None)
    args = parser.parse_args()
    for path in plot_library(args.input, args.output_dir, seed_label=args.rng_seed):
        print(path)


if __name__ == "__main__":
    main()
