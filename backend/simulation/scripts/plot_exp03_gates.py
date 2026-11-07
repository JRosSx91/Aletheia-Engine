#!/usr/bin/env python3
"""HAR-EXP-03 Plotly figures for stellar gates. Engine outputs, not habitability."""

from __future__ import annotations

import argparse
from pathlib import Path

import pandas as pd
import plotly.graph_objects as go
from plotly.subplots import make_subplots

from harmonic_csv import load_exp03_gates
from plot_common import ensure_dir, save_html

CAPTION = (
    "HAR-EXP-03 · modelo experimental · semilla 137137 · "
    "índices internos, no habitabilidad"
)
HOVER = "sample_id=%{customdata[0]}<br>α_s_ref=%{customdata[1]:.4f}<br>G=%{customdata[2]:.3e}<extra></extra>"


def _custom(frame: pd.DataFrame):
    return frame[["sample_id", "alpha_s_reference", "g_si"]]


def plot_exp03_gates(
    csv_a: Path,
    csv_d: Path,
    scan_189_as: Path,
    scan_189_e: Path,
    scan_189_g: Path,
    scan_219_as: Path,
    scan_219_g: Path,
    output_dir: Path,
) -> dict[str, Path]:
    output_dir = ensure_dir(output_dir)
    a = load_exp03_gates(csv_a)
    d = load_exp03_gates(csv_d)
    s189_as = load_exp03_gates(scan_189_as)
    s189_e = load_exp03_gates(scan_189_e)
    s189_g = load_exp03_gates(scan_189_g)
    s219_as = load_exp03_gates(scan_219_as)
    s219_g = load_exp03_gates(scan_219_g)
    paths: dict[str, Path] = {}

    fig1 = go.Figure()
    for name, frame, color in (("A historical", a, "#4C78A8"), ("D contrast", d, "#54A24B")):
        counts = frame["complexity_level"].value_counts().sort_index()
        fig1.add_trace(
            go.Bar(
                x=[str(i) for i in counts.index],
                y=counts.values,
                name=name,
                marker_color=color,
                hovertemplate="nivel %{x}<br>n=%{y}<extra>" + name + "</extra>",
            )
        )
    fig1.update_layout(
        title="Distribución de complexity_level (VIS-01, 401 muestras)",
        xaxis_title="nivel (0–5)",
        yaxis_title="número de universos",
        barmode="group",
        annotations=[dict(text=CAPTION, x=0, y=1.12, xref="paper", yref="paper", showarrow=False)],
    )
    paths["levels"] = save_html(fig1, output_dir / "exp03_levels.html")

    def funnel(frame: pd.DataFrame) -> list[int]:
        return [
            int(frame["nucleon_stability_gate_passed"].eq(1).sum()),
            int((frame["nucleosynthesis_term"] > 0.3).sum()),
            int(frame["stellar_term"].notna().sum()),
            int((frame["stellar_term"] > 0.4).sum()),
            int(frame["heavy_element_term"].notna().sum()),
            int((frame["heavy_element_term"] > 0.5).sum()),
            int(frame["black_hole_term"].notna().sum()),
        ]

    labels = [
        "nucleón ok",
        "BBN > 0.3",
        "stellar evaluado",
        "stellar > 0.4",
        "pesados evaluado",
        "pesados > 0.5",
        "BH evaluado",
    ]
    fig2 = go.Figure()
    fig2.add_trace(go.Bar(x=labels, y=funnel(a), name="A", marker_color="#4C78A8"))
    fig2.add_trace(go.Bar(x=labels, y=funnel(d), name="D", marker_color="#54A24B"))
    fig2.update_layout(
        title="Etapas evaluadas vs superadas (None ≠ 0)",
        yaxis_title="muestras",
        barmode="group",
    )
    paths["funnel"] = save_html(fig2, output_dir / "exp03_gate_funnel.html")

    fig3 = go.Figure()
    fig3.add_trace(
        go.Scatter(
            x=a["g_si"],
            y=a["alpha_s_reference"],
            mode="markers",
            name="A",
            marker=dict(
                size=8,
                color=a["complexity_level"],
                colorscale="Viridis",
                colorbar=dict(title="nivel"),
                cmin=0,
                cmax=5,
            ),
            customdata=_custom(a),
            hovertemplate=HOVER,
        )
    )
    fig3.update_layout(
        title="G frente a α_s_ref, color = nivel (A). G es SI, no un invariante adimensional.",
        xaxis_title="G (m³ kg⁻¹ s⁻²)",
        yaxis_title="α_s_ref",
    )
    paths["params"] = save_html(fig3, output_dir / "exp03_params_vs_level.html")

    fig4 = make_subplots(rows=1, cols=2, subplot_titles=("M_Jeans vs G", "τ vs G"))
    fig4.add_trace(
        go.Scatter(
            x=s189_g["g_si"],
            y=s189_g["jeans_diagnostic_kg"],
            mode="lines+markers",
            name="189 A",
            customdata=s189_g[["complexity_level", "stellar_term"]],
            hovertemplate="G=%{x:.3e}<br>M_J=%{y:.3e} kg<br>nivel=%{customdata[0]}<extra>189</extra>",
        ),
        row=1,
        col=1,
    )
    fig4.add_trace(
        go.Scatter(
            x=s219_g["g_si"],
            y=s219_g["jeans_diagnostic_kg"],
            mode="lines+markers",
            name="219 A",
            hovertemplate="G=%{x:.3e}<br>M_J=%{y:.3e} kg<extra>219</extra>",
        ),
        row=1,
        col=1,
    )
    fig4.add_trace(
        go.Scatter(
            x=s189_g["g_si"],
            y=s189_g["stellar_lifetime_years"],
            mode="lines+markers",
            name="189 τ",
            hovertemplate="G=%{x:.3e}<br>τ=%{y:.3e} yr<extra>189</extra>",
        ),
        row=1,
        col=2,
    )
    fig4.add_trace(
        go.Scatter(
            x=s219_g["g_si"],
            y=s219_g["stellar_lifetime_years"],
            mode="lines+markers",
            name="219 τ",
            hovertemplate="G=%{x:.3e}<br>τ=%{y:.3e} yr<extra>219</extra>",
        ),
        row=1,
        col=2,
    )
    fig4.update_xaxes(title_text="G (SI)", row=1, col=1)
    fig4.update_xaxes(title_text="G (SI)", row=1, col=2)
    fig4.update_yaxes(title_text="M_Jeans (kg)", type="log", row=1, col=1)
    fig4.update_yaxes(title_text="τ (años)", type="log", row=1, col=2)
    fig4.update_layout(title="Sensibilidad Jeans y lifetime: G SI a m_p y ρ fijas (no es variación adimensional)")
    paths["jeans"] = save_html(fig4, output_dir / "exp03_jeans_lifetime.html")

    pair = a[a["sample_id"].isin([219, 351])].copy()
    pair["which"] = "A"
    pd_ = d[d["sample_id"].isin([219, 351])].copy()
    pd_["which"] = "D"
    both = pd.concat([pair, pd_], ignore_index=True)
    fig5 = make_subplots(
        rows=2,
        cols=2,
        subplot_titles=("stellar_term (0–1)", "heavy_element_term (0–1)", "nivel", "τ (años)"),
    )
    for which, color in (("A", "#4C78A8"), ("D", "#54A24B")):
        sub = both[both["which"] == which]
        xs = sub["sample_id"].astype(str)
        fig5.add_trace(
            go.Bar(x=xs, y=sub["stellar_term"], name=which, marker_color=color, legendgroup=which),
            1,
            1,
        )
        fig5.add_trace(
            go.Bar(
                x=xs,
                y=sub["heavy_element_term"],
                name=which,
                marker_color=color,
                legendgroup=which,
                showlegend=False,
            ),
            1,
            2,
        )
        fig5.add_trace(
            go.Bar(
                x=xs,
                y=sub["complexity_level"],
                name=which,
                marker_color=color,
                legendgroup=which,
                showlegend=False,
            ),
            2,
            1,
        )
        fig5.add_trace(
            go.Bar(
                x=xs,
                y=sub["stellar_lifetime_years"],
                name=which,
                marker_color=color,
                legendgroup=which,
                showlegend=False,
            ),
            2,
            2,
        )
    fig5.update_layout(title="219 y 351 A vs D: mismos ids, distinto running. No es QCD.")
    paths["ad"] = save_html(fig5, output_dir / "exp03_219_351_AD.html")

    fig_h = make_subplots(rows=1, cols=2, subplot_titles=("189: heavy vs α_s_ref", "189: heavy vs α SI"))
    fig_h.add_trace(
        go.Scatter(
            x=s189_as["alpha_s_reference"],
            y=s189_as["heavy_element_term"],
            mode="markers",
            name="α_s",
            hovertemplate="α_s=%{x:.4f}<br>heavy=%{y:.3e}<extra></extra>",
        ),
        1,
        1,
    )
    fig_h.add_trace(
        go.Scatter(
            x=s189_e["alpha_si"],
            y=s189_e["heavy_element_term"],
            mode="markers",
            name="α",
            hovertemplate="α=%{x:.4f}<br>heavy=%{y:.3e}<extra></extra>",
        ),
        1,
        2,
    )
    fig_h.update_xaxes(title_text="α_s_ref", row=1, col=1)
    fig_h.update_xaxes(title_text="α (SI)", row=1, col=2)
    fig_h.update_yaxes(title_text="heavy_element_term", row=1, col=1)
    fig_h.update_layout(title="Universo 189 (A): gaussiana α_s=0.118 y barrera Coulomb ~ e². CSV deja NaN si la etapa no corre.")
    paths["heavy"] = save_html(fig_h, output_dir / "exp03_189_heavy.html")
    return paths


def main() -> None:
    parser = argparse.ArgumentParser(description="HAR-EXP-03 figures")
    parser.add_argument("--dir", type=Path, required=True)
    parser.add_argument("--out", type=Path, default=None)
    args = parser.parse_args()
    out = args.out or args.dir
    plot_exp03_gates(
        args.dir / "vis01_gates_A.csv",
        args.dir / "vis01_gates_D.csv",
        args.dir / "scan_189_alpha_s.csv",
        args.dir / "scan_189_e.csv",
        args.dir / "scan_189_g.csv",
        args.dir / "scan_219_alpha_s.csv",
        args.dir / "scan_219_g.csv",
        out,
    )


if __name__ == "__main__":
    main()
