#!/usr/bin/env python3
"""HAR-EXP-02: how the running_alpha_s(1 GeV) cut reaches masses and fitness.

Panels describe engine outputs. They are not a QCD phase diagram.
"""

from __future__ import annotations

import argparse
from pathlib import Path

import plotly.graph_objects as go
from plotly.subplots import make_subplots

from plot_alphas_bifurcation import (
    SCAN_COLUMNS,
    alpha_s_at_denominator_cut,
    load_alpha_s_scan,
)
from plot_common import ensure_dir, save_html

PROPAGATION_COLUMNS = SCAN_COLUMNS + [
    "theoretical_pion_mass_kg",
    "validated_proton_mass_kg",
    "nucleon_stability_gate_passed",
    "cosmology_term",
    "nucleosynthesis_term",
    "stellar_term",
    "heavy_element_term",
    "black_hole_term",
    "level_bonus_applied",
    "complexity_level",
    "fitness_uncapped",
    "fitness",
    "jeans_mass_kg",
    "fitness_was_capped",
]


def plot_alphas_propagation(scan_csv: Path, output_html: Path) -> Path:
    frame = load_alpha_s_scan(scan_csv)
    missing = [c for c in PROPAGATION_COLUMNS if c not in frame.columns]
    if missing:
        raise ValueError(f"Faltan columnas HAR-EXP-02: {missing}")
    frame = frame.sort_values("alpha_s_reference")
    cut = alpha_s_at_denominator_cut()
    run = frame[frame["used_running_formula"] == 1]
    off = frame[frame["used_running_formula"] == 0]

    hover = (
        "α_s_ref=%{x:.6g}<br>"
        "y=%{y:.6g}<br>"
        "α_s(1 GeV)=%{customdata[0]:.6g}<br>"
        "denom=%{customdata[1]:.6g}<br>"
        "PDG=%{customdata[2]}<br>"
        "nucleon_gate=%{customdata[3]}<br>"
        "level=%{customdata[4]}<br>"
        "fitness=%{customdata[5]:.6f}"
    )

    def custom(part):
        return list(
            zip(
                part["running_alpha_s_1gev"],
                part["running_denominator"],
                part["used_pdg_hadron_fallback"],
                part["nucleon_stability_gate_passed"],
                part["complexity_level"],
                part["fitness"],
            )
        )

    fig = make_subplots(
        rows=2,
        cols=2,
        subplot_titles=(
            "α_s(1 GeV) vs α_s de referencia",
            "m_p teórica (kg) vs α_s",
            "Δm teórico (kg) vs α_s",
            "fitness y términos heurísticos (adimensionales)",
        ),
        vertical_spacing=0.14,
        horizontal_spacing=0.10,
    )

    def add_pair(row, col, ycol, run_name, off_name):
        fig.add_trace(
            go.Scatter(
                x=run["alpha_s_reference"],
                y=run[ycol],
                mode="lines",
                name=run_name,
                line=dict(color="#4cc9f0", width=2),
                customdata=custom(run),
                hovertemplate=hover,
                legendgroup="run",
                showlegend=row == 1 and col == 1,
            ),
            row=row,
            col=col,
        )
        fig.add_trace(
            go.Scatter(
                x=off["alpha_s_reference"],
                y=off[ycol],
                mode="lines",
                name=off_name,
                line=dict(color="#f72585", width=2),
                customdata=custom(off),
                hovertemplate=hover,
                legendgroup="off",
                showlegend=row == 1 and col == 1,
            ),
            row=row,
            col=col,
        )

    add_pair(1, 1, "running_alpha_s_1gev", "denom > 0.1", "denom ≤ 0.1")
    add_pair(1, 2, "theoretical_proton_mass_kg", "m_p running", "m_p corte")
    add_pair(2, 1, "theoretical_np_mass_diff_kg", "Δm running", "Δm corte")

    heuristic = [
        ("fitness", "#f8f9fa", "fitness"),
        ("nucleosynthesis_term", "#90e0ef", "nucleosynthesis_term"),
        ("stellar_term", "#ffd166", "stellar_term"),
        ("heavy_element_term", "#06d6a0", "heavy_element_term"),
        ("cosmology_term", "#adb5bd", "cosmology_term"),
    ]
    for ycol, color, name in heuristic:
        fig.add_trace(
            go.Scatter(
                x=frame["alpha_s_reference"],
                y=frame[ycol],
                mode="lines",
                name=name,
                line=dict(color=color, width=2),
                hovertemplate=f"{name}=%{{y:.6g}}<br>α_s=%{{x:.6g}}",
                legendgroup=name,
            ),
            row=2,
            col=2,
        )

    for row, col in ((1, 1), (1, 2), (2, 1), (2, 2)):
        fig.add_vline(
            x=cut,
            line_dash="dash",
            line_color="#e9ecef",
            row=row,
            col=col,
        )

    zoom = dict(range=[0.10, 0.18])
    fig.update_xaxes(title_text="α_s_reference", range=zoom["range"], row=1, col=1)
    fig.update_xaxes(title_text="α_s_reference", range=zoom["range"], row=1, col=2)
    fig.update_xaxes(title_text="α_s_reference", range=zoom["range"], row=2, col=1)
    fig.update_xaxes(title_text="α_s_reference", range=zoom["range"], row=2, col=2)
    fig.update_yaxes(title_text="α_s(1 GeV)", row=1, col=1)
    fig.update_yaxes(title_text="kg", row=1, col=2)
    fig.update_yaxes(title_text="kg", row=2, col=1)
    fig.update_yaxes(title_text="índice 0–1", range=[-0.05, 1.05], row=2, col=2)
    fig.update_layout(
        template="plotly_dark",
        title=(
            "HAR-EXP-02 · propagación del corte denominator>0.1 "
            f"(α_s≈{cut:.5f}) · no es una transición QCD"
        ),
        legend=dict(orientation="h", y=-0.12),
        margin=dict(b=110, t=80),
        annotations=[
            dict(
                text=(
                    "Zoom por defecto: entorno del umbral. "
                    "El panel 4 solo agrupa puntuaciones adimensionales. "
                    "El 1-loop no es una predicción cuantitativa en acoplamiento fuerte."
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
    )
    output_html = Path(output_html)
    ensure_dir(output_html.parent)
    return save_html(fig, output_html)


def main() -> None:
    parser = argparse.ArgumentParser(description="Figura HAR-EXP-02: propagación del corte α_s.")
    parser.add_argument("--scan", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    print(plot_alphas_propagation(args.scan, args.output))


if __name__ == "__main__":
    main()
