#!/usr/bin/env python3
"""HAR-EXP-01 Plotly overlay of map samples vs a 1-parameter α_s scan.

The figure annotates the engine's `denominator > 0.1` branch in
`running_alpha_s` at μ=1 GeV. It does not claim a physical QCD transition.
"""

from __future__ import annotations

import argparse
import math
from pathlib import Path

import pandas as pd
import plotly.graph_objects as go

from harmonic_csv import load_evaluations
from plot_common import ensure_dir, save_html

SCAN_COLUMNS = [
    "alpha_s_reference",
    "running_denominator",
    "used_running_formula",
    "running_alpha_s_1gev",
    "theoretical_proton_mass_kg",
    "theoretical_neutron_mass_kg",
    "theoretical_np_mass_diff_kg",
    "used_pdg_hadron_fallback",
]

HADRONIC_MU_GEV = 1.0
MU_REF_GEV = 91.1876
BETA_0_NF3 = 9.0
DENOM_CUT = 0.1


def hadronic_running_denominator(alpha_s_ref: float) -> float:
    log_ratio = math.log(HADRONIC_MU_GEV / MU_REF_GEV)
    return 1.0 + (BETA_0_NF3 * alpha_s_ref / (2.0 * math.pi)) * log_ratio


def alpha_s_at_denominator_cut() -> float:
    log_ratio = math.log(HADRONIC_MU_GEV / MU_REF_GEV)
    coef = (BETA_0_NF3 / (2.0 * math.pi)) * log_ratio
    return (DENOM_CUT - 1.0) / coef


def load_alpha_s_scan(path: Path) -> pd.DataFrame:
    frame = pd.read_csv(path)
    missing = [c for c in SCAN_COLUMNS if c not in frame.columns]
    if missing:
        raise ValueError(f"Faltan columnas del barrido: {missing}")
    return frame


def classify_running(alpha_s_ref: pd.Series) -> pd.Series:
    denom = alpha_s_ref.map(hadronic_running_denominator)
    return denom > DENOM_CUT


def plot_alphas_bifurcation(
    evaluations_csv: Path,
    scan_csv: Path,
    output_html: Path,
) -> Path:
    samples = load_evaluations(evaluations_csv)
    scan = load_alpha_s_scan(scan_csv)
    samples = samples.copy()
    samples["running_denominator"] = samples["alpha_s_reference"].map(
        hadronic_running_denominator
    )
    samples["used_running_formula"] = classify_running(samples["alpha_s_reference"])
    cut = alpha_s_at_denominator_cut()
    running = samples[samples["used_running_formula"]]
    cutoff = samples[~samples["used_running_formula"]]
    scan_run = scan[scan["used_running_formula"] == 1]
    scan_cut = scan[scan["used_running_formula"] == 0]

    fig = go.Figure()
    hover_s = (
        "sample_id=%{customdata[0]}<br>"
        "α_s_ref=%{x:.6g}<br>"
        "Δm=%{y:.6e} kg<br>"
        "α_si=%{customdata[1]:.6e}<br>"
        "m_p=%{customdata[2]:.6e} kg<br>"
        "m_n=%{customdata[3]:.6e} kg<br>"
        "PDG fallback=%{customdata[4]}<br>"
        "fitness=%{customdata[5]:.6f}<br>"
        "level=%{customdata[6]}<br>"
        "g_si=%{customdata[7]:.6e}<br>"
        "e_si=%{customdata[8]:.6e}<br>"
        "denom=%{customdata[9]:.6g}"
    )
    hover_scan = (
        "barrido α_s=%{x:.6g}<br>"
        "Δm=%{y:.6e} kg<br>"
        "m_p=%{customdata[0]:.6e} kg<br>"
        "m_n=%{customdata[1]:.6e} kg<br>"
        "denom=%{customdata[2]:.6g}<br>"
        "PDG fallback=%{customdata[3]}"
    )

    def sample_custom(frame: pd.DataFrame):
        return list(
            zip(
                frame["sample_id"],
                frame["alpha_si"],
                frame["theoretical_proton_mass_kg"],
                frame["theoretical_neutron_mass_kg"],
                frame["used_pdg_hadron_fallback"],
                frame["fitness"],
                frame["complexity_level"],
                frame["g_si"],
                frame["e_si"],
                frame["running_denominator"],
            )
        )

    def scan_custom(frame: pd.DataFrame):
        return list(
            zip(
                frame["theoretical_proton_mass_kg"],
                frame["theoretical_neutron_mass_kg"],
                frame["running_denominator"],
                frame["used_pdg_hadron_fallback"],
            )
        )

    fig.add_trace(
        go.Scatter(
            x=scan_run["alpha_s_reference"],
            y=scan_run["theoretical_np_mass_diff_kg"],
            mode="lines",
            name="barrido: denom > 0.1 (fórmula 1-loop)",
            line=dict(color="#4cc9f0", width=2),
            customdata=scan_custom(scan_run),
            hovertemplate=hover_scan,
        )
    )
    fig.add_trace(
        go.Scatter(
            x=scan_cut["alpha_s_reference"],
            y=scan_cut["theoretical_np_mass_diff_kg"],
            mode="lines",
            name="barrido: denom ≤ 0.1 (devuelve α_s_reference)",
            line=dict(color="#f72585", width=2),
            customdata=scan_custom(scan_cut),
            hovertemplate=hover_scan,
        )
    )
    fig.add_trace(
        go.Scatter(
            x=running["alpha_s_reference"],
            y=running["theoretical_np_mass_diff_kg"],
            mode="markers",
            name="muestras: denom > 0.1",
            marker=dict(color="#90e0ef", size=8, symbol="circle"),
            customdata=sample_custom(running),
            hovertemplate=hover_s,
        )
    )
    fig.add_trace(
        go.Scatter(
            x=cutoff["alpha_s_reference"],
            y=cutoff["theoretical_np_mass_diff_kg"],
            mode="markers",
            name="muestras: denom ≤ 0.1",
            marker=dict(color="#ff9e00", size=7, symbol="diamond"),
            customdata=sample_custom(cutoff),
            hovertemplate=hover_s,
        )
    )
    fig.add_vline(
        x=cut,
        line_dash="dash",
        line_color="#e9ecef",
        annotation_text=f"denom=0.1 (α_s≈{cut:.5f})",
        annotation_position="top",
    )
    fig.update_layout(
        template="plotly_dark",
        title=(
            "HAR-EXP-01 · α_s de referencia vs Δm teórico · "
            "rama = condición `denominator > 0.1` en running_alpha_s(1 GeV)"
        ),
        xaxis_title="α_s_reference (entrada del CosmicLaw)",
        yaxis_title="theoretical_np_mass_diff_kg",
        xaxis=dict(range=[0.10, 0.18]),
        yaxis=dict(rangemode="tozero"),
        legend=dict(orientation="v"),
        annotations=[
            dict(
                text=(
                    "Zoom por defecto: zona de la bifurcación. "
                    "Las etiquetas describen ramas del código, no transiciones QCD. "
                    "El barrido fija el universo de referencia y varía solo α_s."
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
    output_html = Path(output_html)
    ensure_dir(output_html.parent)
    return save_html(fig, output_html)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Figura HAR-EXP-01: muestras map + barrido α_s sobre Δm."
    )
    parser.add_argument("--evaluations", type=Path, required=True)
    parser.add_argument("--scan", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    path = plot_alphas_bifurcation(args.evaluations, args.scan, args.output)
    print(path)


if __name__ == "__main__":
    main()
