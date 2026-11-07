#!/usr/bin/env python3
"""Exploratory Plotly views of the optional EngineEvaluation CSV.

These figures describe software outputs. They are not laboratory QCD/QED
and do not claim causal physics beyond couplings already in the engine.
"""

from __future__ import annotations

import argparse
from pathlib import Path

import pandas as pd
import plotly.express as px
import plotly.graph_objects as go

from harmonic_csv import EVALUATION_FILENAME, SCHEMA_EVALUATION, load_evaluations
from plot_common import CAPTION_FITNESS, ensure_dir, save_html

CAPTION_NONE = (
    "Celdas vacías = etapa no ejecutada (None). No se sustituyen por cero."
)
CAPTION_ALPHA = (
    "`alpha_si` es α del motor (adimensional). `inverse_alpha_si` es 1/α. "
    "No es la columna histórica `alpha` del CSV map."
)
CAPTION_CORR = (
    "La matriz muestra covariación numérica entre columnas exportadas. "
    "No implica que el motor acoplue causalmente todas las pares."
)


def _seed_label(frame: pd.DataFrame, override: str | None) -> str:
    if override:
        return override
    seeds = frame["rng_seed"].dropna().unique()
    if len(seeds) == 1:
        return str(int(seeds[0])) if float(seeds[0]).is_integer() else str(seeds[0])
    return "unspecified"


def plot_observables(
    input_csv: Path,
    output_dir: Path,
    *,
    seed_label: str | None = None,
    sample_id: int | None = None,
) -> list[Path]:
    frame = load_evaluations(input_csv)
    output_dir = ensure_dir(output_dir)
    written: list[Path] = []
    if frame.empty:
        (output_dir / "README.txt").write_text(
            "El CSV de evaluaciones no contiene filas.", encoding="utf-8"
        )
        return written

    subtitle = f"n={len(frame)} · rng-seed {_seed_label(frame, seed_label)}"
    hover = {
        "sample_id": True,
        "complexity_level": True,
        "fitness": ":.6f",
        "alpha_si": ":.6e",
        "g_si": ":.6e",
        "e_si": ":.6e",
        "used_pdg_hadron_fallback": True,
    }

    hist = px.histogram(
        frame,
        x="fitness",
        nbins=32,
        title=f"Distribución de la puntuación interna · {subtitle}",
        labels={"fitness": "fitness (heurística)"},
    )
    hist.update_layout(template="plotly_dark")
    written.append(save_html(hist, output_dir / "obs_fitness_hist.html"))

    levels = px.histogram(
        frame,
        x="complexity_level",
        title=f"Niveles de clasificación · {subtitle}",
        labels={"complexity_level": "complexity_level (heurística)"},
    )
    levels.update_layout(template="plotly_dark", bargap=0.2)
    written.append(save_html(levels, output_dir / "obs_complexity_hist.html"))

    masses = px.scatter(
        frame,
        x="alpha_si",
        y="theoretical_proton_mass_kg",
        color="alpha_s_reference",
        hover_data=hover,
        title=f"α SI vs masa protónica teórica · {subtitle}",
        labels={
            "alpha_si": "α SI (adimensional)",
            "theoretical_proton_mass_kg": "m_p teórica (kg)",
            "alpha_s_reference": "α_s(μ_ref)",
        },
    )
    masses.update_layout(
        template="plotly_dark",
        annotations=[
            dict(
                text=f"{CAPTION_ALPHA} Las masas u/d muestreadas no entran en esta receta.",
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
    written.append(save_html(masses, output_dir / "obs_alpha_vs_proton.html"))

    npdiff = px.scatter(
        frame,
        x="alpha_s_reference",
        y="theoretical_np_mass_diff_kg",
        color="fitness",
        hover_data=hover,
        title=f"α_s de referencia vs Δm teórico n−p · {subtitle}",
        labels={
            "alpha_s_reference": "α_s(μ_ref) (adimensional)",
            "theoretical_np_mass_diff_kg": "m_n − m_p teórico (kg)",
            "fitness": "fitness (heurística)",
        },
    )
    npdiff.update_layout(template="plotly_dark")
    written.append(save_html(npdiff, output_dir / "obs_alphas_vs_npdiff.html"))

    jeans = frame.dropna(subset=["jeans_mass_kg"])
    if not jeans.empty:
        jfig = px.scatter(
            jeans,
            x="theoretical_proton_mass_kg",
            y="jeans_mass_kg",
            color="fitness",
            hover_data=hover,
            title=f"Jeans (solo etapa estelar ejecutada) · {subtitle}",
            labels={
                "theoretical_proton_mass_kg": "m_p teórica (kg)",
                "jeans_mass_kg": "masa de Jeans (kg)",
            },
        )
        jfig.update_layout(template="plotly_dark")
        written.append(save_html(jfig, output_dir / "obs_jeans.html"))

    matrix_cols = [
        "g_si",
        "alpha_si",
        "alpha_s_reference",
        "theoretical_proton_mass_kg",
        "theoretical_np_mass_diff_kg",
        "fitness",
    ]
    matrix_frame = frame.dropna(subset=matrix_cols)
    matrix = px.scatter_matrix(
        matrix_frame,
        dimensions=matrix_cols,
        color="complexity_level",
        title=f"Scatter matrix (columnas acotadas, sin NaN) · {subtitle}",
    )
    matrix.update_layout(
        template="plotly_dark",
        annotations=[
            dict(
                text=f"{CAPTION_CORR} {CAPTION_FITNESS} {CAPTION_NONE}",
                showarrow=False,
                xref="paper",
                yref="paper",
                x=0,
                y=-0.08,
                font=dict(size=11),
                align="left",
            )
        ],
        margin=dict(b=80),
    )
    written.append(save_html(matrix, output_dir / "obs_scatter_matrix.html"))

    chosen = sample_id
    if chosen is None:
        chosen = int(frame.loc[frame["fitness"].idxmax(), "sample_id"])
    card_rows = frame.loc[frame["sample_id"] == chosen]
    if card_rows.empty:
        raise ValueError(f"sample_id {chosen} no está en el CSV")
    row = card_rows.iloc[0]
    stages = [
        ("cosmology_term", row["cosmology_term"]),
        ("nucleosynthesis_term", row["nucleosynthesis_term"]),
        ("stellar_term", row["stellar_term"]),
        ("heavy_element_term", row["heavy_element_term"]),
        ("black_hole_term", row["black_hole_term"]),
        ("hubble_time_s", row["hubble_time_s"]),
        ("jeans_mass_kg", row["jeans_mass_kg"]),
        ("coulomb_barrier_j", row["coulomb_barrier_j"]),
        ("chandrasekhar_mass_kg", row["chandrasekhar_mass_kg"]),
    ]
    labels = [name for name, _ in stages]
    values = [0.0 if pd.isna(val) else float(val) for _, val in stages]
    colors = ["#555555" if pd.isna(val) else "#6ea8fe" for _, val in stages]
    bar = go.Figure(
        go.Bar(
            x=labels,
            y=values,
            marker_color=colors,
            hovertext=[
                "no evaluado" if pd.isna(val) else f"{val:.6g}" for _, val in stages
            ],
        )
    )
    bar.update_layout(
        template="plotly_dark",
        title=(
            f"Muestra sample_id={chosen} · fitness={row['fitness']:.6g} · "
            f"nivel={int(row['complexity_level'])} · "
            f"g={row['g_si']:.6g} · e={row['e_si']:.6g} · {subtitle}"
        ),
        yaxis_title="valor exportado (gris = None, no es cero)",
        annotations=[
            dict(
                text=(
                    f"{CAPTION_NONE} PDG fallback={int(row['used_pdg_hadron_fallback'])}. "
                    f"{SCHEMA_EVALUATION.notes}"
                ),
                showarrow=False,
                xref="paper",
                yref="paper",
                x=0,
                y=-0.28,
                font=dict(size=11),
                align="left",
            )
        ],
        margin=dict(b=120),
    )
    written.append(save_html(bar, output_dir / "obs_sample_card.html"))
    return written


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, default=Path(EVALUATION_FILENAME))
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--rng-seed", default=None)
    parser.add_argument("--sample-id", type=int, default=None)
    args = parser.parse_args()
    paths = plot_observables(
        args.input,
        args.output_dir,
        seed_label=args.rng_seed,
        sample_id=args.sample_id,
    )
    for path in paths:
        print(path)


if __name__ == "__main__":
    main()
