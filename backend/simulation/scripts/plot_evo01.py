#!/usr/bin/env python3
"""HAR-EVO-01 Plotly figures. Optimizer dynamics, not habitability."""

from __future__ import annotations

import argparse
from pathlib import Path

import pandas as pd
import plotly.graph_objects as go
from plotly.subplots import make_subplots

from harmonic_csv import load_evo01_individuals, load_evo01_metrics
from plot_common import ensure_dir, save_html

CAPTION = (
    "HAR-EVO-01 · índice interno · semilla base 137137 · "
    "no es habitabilidad ni selección cosmológica"
)


def plot_evo01(individuals_csv: Path, metrics_csv: Path, output_dir: Path) -> dict[str, Path]:
    output_dir = ensure_dir(output_dir)
    ind = load_evo01_individuals(individuals_csv)
    met = load_evo01_metrics(metrics_csv)
    paths: dict[str, Path] = {}

    fig1 = go.Figure()
    for arm, color in (("selection", "#4C78A8"), ("control", "#F58518")):
        sub = met[met["arm"] == arm].sort_values("generation")
        fig1.add_trace(
            go.Scatter(
                x=sub["generation"],
                y=sub["best_fitness"],
                name=f"{arm} mejor",
                mode="lines",
                line=dict(color=color, width=2),
                hovertemplate="gen=%{x}<br>mejor=%{y:.4f}<extra>" + arm + "</extra>",
            )
        )
        fig1.add_trace(
            go.Scatter(
                x=sub["generation"],
                y=sub["median_fitness"],
                name=f"{arm} mediana",
                mode="lines",
                line=dict(color=color, width=1, dash="dash"),
                hovertemplate="gen=%{x}<br>mediana=%{y:.4f}<extra>" + arm + "</extra>",
            )
        )
    fig1.update_layout(
        title="Fitness por generación (selección vs control)",
        xaxis_title="generación",
        yaxis_title="fitness (índice 0–1)",
        annotations=[dict(text=CAPTION, x=0, y=1.12, xref="paper", yref="paper", showarrow=False)],
    )
    paths["fitness"] = save_html(fig1, output_dir / "evo01_fitness_sel_vs_ctrl.html")

    fig2 = make_subplots(
        rows=3,
        cols=1,
        shared_xaxes=True,
        subplot_titles=("α (adimensional SI)", "α_s(μ_ref)", "γ = G/G_N (cociente de escala)"),
    )
    gene_cols = [("alpha", 1), ("alpha_s", 2), ("gamma", 3)]
    for arm, color in (("selection", "#4C78A8"), ("control", "#F58518")):
        sub = ind[ind["arm"] == arm]
        for col, row in gene_cols:
            grouped = sub.groupby("generation")[col]
            med = grouped.median()
            best_idx = sub.groupby("generation")["fitness"].idxmax()
            best = sub.loc[best_idx]
            fig2.add_trace(
                go.Scatter(
                    x=med.index,
                    y=med.values,
                    name=f"{arm} mediana {col}",
                    mode="lines",
                    line=dict(color=color, dash="dash", width=1),
                    legendgroup=arm,
                    hovertemplate="gen=%{x}<br>mediana=" + col + "=%{y:.5g}<extra>" + arm + "</extra>",
                ),
                row=row,
                col=1,
            )
            fig2.add_trace(
                go.Scatter(
                    x=best["generation"],
                    y=best[col],
                    name=f"{arm} mejor {col}",
                    mode="lines",
                    line=dict(color=color, width=2),
                    legendgroup=arm,
                    customdata=best[["universe_id", "fitness"]],
                    hovertemplate="gen=%{x}<br>"
                    + col
                    + "=%{y:.5g}<br>id=%{customdata[0]}<br>fit=%{customdata[1]:.4f}<extra>"
                    + arm
                    + "</extra>",
                ),
                row=row,
                col=1,
            )
    fig2.update_layout(
        title="Evolución de genes (mediana poblacional y mejor fitness)",
        height=800,
        annotations=[dict(text=CAPTION, x=0, y=1.04, xref="paper", yref="paper", showarrow=False)],
    )
    fig2.update_xaxes(title_text="generación", row=3, col=1)
    paths["genes"] = save_html(fig2, output_dir / "evo01_genes.html")

    fig3 = make_subplots(rows=1, cols=2, subplot_titles=("selection", "control"))
    colors = ["#4C78A8", "#54A24B", "#F58518", "#E45756", "#B279A2", "#72B7B2"]
    for col_i, arm in enumerate(("selection", "control"), start=1):
        sub = met[met["arm"] == arm].sort_values("generation")
        for lv, color in enumerate(colors):
            fig3.add_trace(
                go.Bar(
                    x=sub["generation"],
                    y=sub[f"frac_level_{lv}"],
                    name=f"nivel {lv}",
                    marker_color=color,
                    legendgroup=f"n{lv}",
                    showlegend=col_i == 1,
                    hovertemplate="gen=%{x}<br>nivel "
                    + str(lv)
                    + "<br>fracción=%{y:.3f}<extra>"
                    + arm
                    + "</extra>",
                ),
                row=1,
                col=col_i,
            )
    fig3.update_layout(
        title="Distribución de complexity_level por generación",
        barmode="stack",
        annotations=[dict(text=CAPTION, x=0, y=1.12, xref="paper", yref="paper", showarrow=False)],
    )
    fig3.update_xaxes(title_text="generación")
    fig3.update_yaxes(title_text="fracción", range=[0, 1])
    paths["levels"] = save_html(fig3, output_dir / "evo01_levels.html")

    fig4 = make_subplots(rows=2, cols=1, shared_xaxes=True, subplot_titles=("diversidad log [0,1]³", "fracción cerca de bordes"))
    for arm, color in (("selection", "#4C78A8"), ("control", "#F58518")):
        sub = met[met["arm"] == arm].sort_values("generation")
        fig4.add_trace(
            go.Scatter(
                x=sub["generation"],
                y=sub["diversity"],
                name=f"{arm} diversidad",
                mode="lines",
                line=dict(color=color),
                hovertemplate="gen=%{x}<br>diversidad=%{y:.4f}<extra>" + arm + "</extra>",
            ),
            row=1,
            col=1,
        )
        fig4.add_trace(
            go.Scatter(
                x=sub["generation"],
                y=sub["frac_near_border"],
                name=f"{arm} borde",
                mode="lines",
                line=dict(color=color),
                hovertemplate="gen=%{x}<br>fracción borde=%{y:.3f}<extra>" + arm + "</extra>",
            ),
            row=2,
            col=1,
        )
    fig4.update_layout(
        title="Diversidad genética y concentración en límites de la caja",
        height=640,
        annotations=[dict(text=CAPTION, x=0, y=1.06, xref="paper", yref="paper", showarrow=False)],
    )
    fig4.update_xaxes(title_text="generación", row=2, col=1)
    paths["diversity"] = save_html(fig4, output_dir / "evo01_diversity_bounds.html")

    sel = ind[ind["arm"] == "selection"]
    last = sel["generation"].max()
    top = sel[sel["generation"] == last].nlargest(12, "fitness")
    wanted: set[int] = set(int(x) for x in top["universe_id"].tolist())
    by_id = sel.set_index("universe_id", drop=False)
    for uid in list(wanted):
        cur = uid
        seen = set()
        while cur in by_id.index and cur not in seen:
            seen.add(cur)
            wanted.add(cur)
            parent = by_id.loc[cur, "parent_id"]
            if pd.isna(parent):
                break
            cur = int(parent)
    nodes = sel[sel["universe_id"].isin(wanted)].copy()
    fig5 = go.Figure()
    for _, row in nodes.iterrows():
        if pd.notna(row["parent_id"]) and int(row["parent_id"]) in wanted:
            parent = by_id.loc[int(row["parent_id"])]
            fig5.add_trace(
                go.Scatter(
                    x=[parent["slot"], row["slot"]],
                    y=[parent["generation"], row["generation"]],
                    mode="lines",
                    line=dict(color="#bbbbbb", width=1),
                    hoverinfo="skip",
                    showlegend=False,
                )
            )
    fig5.add_trace(
        go.Scatter(
            x=nodes["slot"],
            y=nodes["generation"],
            mode="markers",
            marker=dict(
                size=9,
                color=nodes["fitness"],
                colorscale="Viridis",
                colorbar=dict(title="fitness"),
            ),
            customdata=nodes[["universe_id", "alpha", "alpha_s", "gamma", "fitness"]],
            hovertemplate=(
                "id=%{customdata[0]}<br>slot=%{x}<br>gen=%{y}"
                "<br>α=%{customdata[1]:.5g}<br>α_s=%{customdata[2]:.5g}"
                "<br>γ=%{customdata[3]:.5g}<br>fit=%{customdata[4]:.4f}<extra></extra>"
            ),
            name="linaje destacados",
        )
    )
    fig5.update_layout(
        title="Genealogía de los 12 mayores fitness finales (brazo selección)",
        xaxis_title="slot",
        yaxis_title="generación",
        annotations=[dict(text=CAPTION, x=0, y=1.12, xref="paper", yref="paper", showarrow=False)],
    )
    paths["genealogy"] = save_html(fig5, output_dir / "evo01_genealogy.html")
    return paths


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--individuals", type=Path, required=True)
    p.add_argument("--metrics", type=Path, required=True)
    p.add_argument("--out", type=Path, required=True)
    args = p.parse_args()
    plot_evo01(args.individuals, args.metrics, args.out)


if __name__ == "__main__":
    main()
