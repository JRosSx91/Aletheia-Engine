#!/usr/bin/env python3
"""HAR-MODEL-01B Plotly comparison of running variants A/B/D.

Figures are engine outputs under three algorithmic recipes. They are not
a claim that Smooth B is laboratory QCD.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import pandas as pd
import plotly.graph_objects as go

from harmonic_csv import load_running_compare
from plot_common import ensure_dir, save_html

CUT_ALPHA_S = 0.1392266
CAPTION = (
    "HAR-MODEL-01B · modelo experimental · semilla 137137 · "
    "B no es QCD validada · D es control negativo"
)
HOVER = ["sample_id", "alpha_s_reference", "running_denominator", "fitness", "complexity_level"]


def _empty_none_not_zero(frame: pd.DataFrame, column: str) -> None:
    if column in frame.columns and frame[column].isna().any():
        pass


def paired_frame(control: pd.DataFrame, other: pd.DataFrame, suffix: str) -> pd.DataFrame:
    keep = [
        "sample_id",
        "alpha_s_reference",
        "running_denominator",
        "running_alpha_s_1gev",
        "theoretical_proton_mass_kg",
        "theoretical_neutron_mass_kg",
        "theoretical_np_mass_diff_kg",
        "nucleosynthesis_term",
        "stellar_term",
        "heavy_element_term",
        "fitness",
        "complexity_level",
        "nucleon_stability_gate_passed",
        "used_pdg_hadron_fallback",
    ]
    a = control[keep].rename(columns={c: f"{c}_A" for c in keep if c != "sample_id"})
    a = a.rename(columns={"alpha_s_reference_A": "alpha_s_reference"})
    b = other[keep].rename(columns={c: f"{c}_{suffix}" for c in keep if c != "sample_id"})
    merged = a.merge(b, on="sample_id", how="inner")
    if len(merged) != len(control) or len(merged) != len(other):
        raise ValueError("sample_id sets do not match; comparison is not paired")
    return merged


def _delta(merged: pd.DataFrame, field: str, suffix: str) -> pd.Series:
    return merged[f"{field}_{suffix}"] - merged[f"{field}_A"]


def summarize_pair(merged: pd.DataFrame, suffix: str, eps: float | None) -> dict:
    fit = _delta(merged, "fitness", suffix)
    mp = _delta(merged, "theoretical_proton_mass_kg", suffix)
    mn = _delta(merged, "theoretical_neutron_mass_kg", suffix)
    dm = _delta(merged, "theoretical_np_mass_diff_kg", suffix)
    bbn = _delta(merged, "nucleosynthesis_term", suffix)
    level_chg = merged[f"complexity_level_{suffix}"] != merged["complexity_level_A"]
    nucleon_chg = (
        merged[f"nucleon_stability_gate_passed_{suffix}"]
        != merged["nucleon_stability_gate_passed_A"]
    )
    stellar_none_a = merged["stellar_term_A"].isna()
    stellar_none_v = merged[f"stellar_term_{suffix}"].isna()
    stellar_gate = stellar_none_a != stellar_none_v
    num = fit.abs() > 1e-15
    relevant = level_chg | nucleon_chg | stellar_gate | (fit.abs() > 1e-6) | (mp.abs() > 1e-40)
    region = (merged["alpha_s_reference"] - CUT_ALPHA_S).abs() < 0.01

    def quantiles(s: pd.Series) -> dict:
        s = s.dropna()
        if s.empty:
            return {"q50": None, "q90": None, "max_abs": None}
        return {
            "q50": float(s.abs().quantile(0.5)),
            "q90": float(s.abs().quantile(0.9)),
            "max_abs": float(s.abs().max()),
        }

    return {
        "variant": suffix,
        "epsilon": eps,
        "n": int(len(merged)),
        "n_numeric_change": int(num.sum()),
        "n_unchanged_bits": int((~num & ~level_chg).sum()),
        "n_relevant_change": int(relevant.sum()),
        "n_level_change": int(level_chg.sum()),
        "n_nucleon_gate_change": int(nucleon_chg.sum()),
        "n_stellar_gate_change": int(stellar_gate.sum()),
        "delta_fitness": quantiles(fit),
        "delta_m_p": quantiles(mp),
        "delta_m_n": quantiles(mn),
        "delta_np_diff": quantiles(dm),
        "delta_bbn": quantiles(bbn.dropna()),
        "near_cut_0.1392266_pm_0.01": {
            "n": int(region.sum()),
            "n_relevant": int((relevant & region).sum()),
            "delta_fitness": quantiles(fit[region]),
        },
    }


def plot_running_compare(
    csv_a: Path,
    csv_b: Path,
    csv_d: Path,
    csv_b_lo: Path,
    csv_b_hi: Path,
    output_dir: Path,
    expected_n: int | None = 401,
) -> dict[str, Path]:
    output_dir = ensure_dir(output_dir)
    a = load_running_compare(csv_a)
    b = load_running_compare(csv_b)
    d = load_running_compare(csv_d)
    b_lo = load_running_compare(csv_b_lo)
    b_hi = load_running_compare(csv_b_hi)
    for frame in (a, b, d, b_lo, b_hi):
        _empty_none_not_zero(frame, "stellar_term")
        if expected_n is not None and len(frame) != expected_n:
            raise ValueError(f"expected {expected_n} paired samples, got {len(frame)}")

    pb = paired_frame(a, b, "B")
    pd_ = paired_frame(a, d, "D")
    plo = paired_frame(a, b_lo, "Blo")
    phi = paired_frame(a, b_hi, "Bhi")

    summary = {
        "caption": CAPTION,
        "B_eps0p02": summarize_pair(pb, "B", 0.02),
        "D": summarize_pair(pd_, "D", None),
        "B_eps0p005": summarize_pair(plo, "Blo", 0.005),
        "B_eps0p05": summarize_pair(phi, "Bhi", 0.05),
    }
    (output_dir / "paired_summary.json").write_text(
        json.dumps(summary, indent=2), encoding="utf-8"
    )

    paths: dict[str, Path] = {}

    fig1 = go.Figure()
    for name, frame, color in (
        ("A historical", a, "#4C78A8"),
        ("B ε=0.02", b, "#F58518"),
        ("D constant", d, "#54A24B"),
    ):
        fig1.add_trace(
            go.Scatter(
                x=frame["alpha_s_reference"],
                y=frame["running_alpha_s_1gev"],
                mode="markers",
                name=name,
                marker=dict(size=7, color=color, opacity=0.75),
                customdata=frame[HOVER],
                hovertemplate=(
                    "sample_id=%{customdata[0]}<br>α_s_ref=%{x:.6f}"
                    "<br>α_s(1 GeV)=%{y:.6f}<br>denom=%{customdata[2]:.4f}"
                    "<extra>" + name + "</extra>"
                ),
            )
        )
    fig1.add_vline(x=CUT_ALPHA_S, line_dash="dot", annotation_text="denom=0.1 (A)")
    fig1.update_layout(
        title="α_s(1 GeV) por variante (modelo experimental, no QCD)",
        xaxis_title="α_s_ref",
        yaxis_title="α_s(1 GeV)",
        annotations=[dict(text=CAPTION, x=0, y=1.12, xref="paper", yref="paper", showarrow=False)],
    )
    paths["alphas"] = save_html(fig1, output_dir / "cmp_alphas_1gev.html")

    fig2 = go.Figure()
    fig2.add_trace(
        go.Scatter(
            x=pb["fitness_A"],
            y=pb["fitness_B"],
            mode="markers",
            name="B vs A",
            marker=dict(size=7, color="#F58518"),
            customdata=pb[["sample_id", "alpha_s_reference"]],
            hovertemplate="sample_id=%{customdata[0]}<br>α_s_ref=%{customdata[1]:.6f}"
            "<br>fit_A=%{x:.6f}<br>fit_B=%{y:.6f}<extra></extra>",
        )
    )
    lo = float(min(pb["fitness_A"].min(), pb["fitness_B"].min()))
    hi = float(max(pb["fitness_A"].max(), pb["fitness_B"].max()))
    fig2.add_trace(
        go.Scatter(
            x=[lo, hi],
            y=[lo, hi],
            mode="lines",
            name="identidad",
            line=dict(color="#888", dash="dash"),
        )
    )
    fig2.update_layout(
        title="Fitness A frente a B ε=0.02 (índice interno)",
        xaxis_title="fitness A",
        yaxis_title="fitness B",
    )
    paths["fitness_ab"] = save_html(fig2, output_dir / "cmp_fitness_A_vs_B.html")

    fig3 = go.Figure()
    fig3.add_trace(
        go.Scatter(
            x=pb["alpha_s_reference"],
            y=_delta(pb, "fitness", "B"),
            mode="markers",
            name="B−A",
            marker=dict(size=7, color="#F58518"),
            customdata=pb[["sample_id"]],
            hovertemplate="sample_id=%{customdata[0]}<br>α_s_ref=%{x:.6f}"
            "<br>Δfitness=%{y:.6e}<extra>B−A</extra>",
        )
    )
    fig3.add_trace(
        go.Scatter(
            x=pd_["alpha_s_reference"],
            y=_delta(pd_, "fitness", "D"),
            mode="markers",
            name="D−A",
            marker=dict(size=7, color="#54A24B", opacity=0.6),
            customdata=pd_[["sample_id"]],
            hovertemplate="sample_id=%{customdata[0]}<br>α_s_ref=%{x:.6f}"
            "<br>Δfitness=%{y:.6e}<extra>D−A</extra>",
        )
    )
    fig3.add_vline(x=CUT_ALPHA_S, line_dash="dot")
    fig3.update_layout(
        title="Diferencia pareada de fitness frente a α_s_ref",
        xaxis_title="α_s_ref",
        yaxis_title="Δ fitness (variante − A)",
    )
    paths["dfit"] = save_html(fig3, output_dir / "cmp_delta_fitness.html")

    fig4 = go.Figure()
    level_b = (pb["complexity_level_B"] != pb["complexity_level_A"]).astype(int)
    fig4.add_trace(
        go.Scatter(
            x=pb["alpha_s_reference"],
            y=level_b,
            mode="markers",
            name="cambio de nivel B vs A",
            marker=dict(size=8, color="#E45756"),
            customdata=pb[["sample_id", "complexity_level_A", "complexity_level_B"]],
            hovertemplate="sample_id=%{customdata[0]}<br>nivel A=%{customdata[1]}"
            "<br>nivel B=%{customdata[2]}<extra></extra>",
        )
    )
    fig4.update_layout(
        title="Cambios de nivel de complejidad (1 = distinto de A)",
        xaxis_title="α_s_ref",
        yaxis_title="cambio de nivel",
        yaxis=dict(tickmode="array", tickvals=[0, 1]),
    )
    paths["level"] = save_html(fig4, output_dir / "cmp_level_gates.html")

    fig5 = go.Figure()
    for name, frame, suf, color in (
        ("ε=0.005", plo, "Blo", "#B279A2"),
        ("ε=0.02", pb, "B", "#F58518"),
        ("ε=0.05", phi, "Bhi", "#72B7B2"),
    ):
        fig5.add_trace(
            go.Scatter(
                x=frame["alpha_s_reference"],
                y=_delta(frame, "fitness", suf),
                mode="markers",
                name=name,
                marker=dict(size=7, color=color, opacity=0.7),
                customdata=frame[["sample_id"]],
                hovertemplate="sample_id=%{customdata[0]}<br>α_s_ref=%{x:.6f}"
                "<br>Δfitness=%{y:.6e}<extra>" + name + "</extra>",
            )
        )
    fig5.add_vline(x=CUT_ALPHA_S, line_dash="dot")
    fig5.update_layout(
        title="Sensibilidad a ε (Δfitness B−A); ε no se ajustó al fitness",
        xaxis_title="α_s_ref",
        yaxis_title="Δ fitness (B − A)",
    )
    paths["eps"] = save_html(fig5, output_dir / "cmp_epsilon_sensitivity.html")
    return paths


def main() -> None:
    parser = argparse.ArgumentParser(description="HAR-MODEL-01B paired Plotly figures")
    parser.add_argument("--dir", type=Path, required=True, help="Folder with variant_*.csv")
    parser.add_argument("--out", type=Path, default=None)
    args = parser.parse_args()
    out = args.out or args.dir
    plot_running_compare(
        args.dir / "variant_A_historical.csv",
        args.dir / "variant_B_smooth_eps0p02.csv",
        args.dir / "variant_D_constant.csv",
        args.dir / "variant_B_smooth_eps0p005.csv",
        args.dir / "variant_B_smooth_eps0p05.csv",
        out,
    )


if __name__ == "__main__":
    main()
