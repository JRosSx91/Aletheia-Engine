"""Schema and loader tests for Harmonic-137 CSV helpers. No cosmology claims."""

from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

import pandas as pd

from find_adam import find_adam
from harmonic_csv import (
    CsvSchemaError,
    EVALUATION_COLUMNS,
    EVO01_INDIVIDUAL_COLUMNS,
    EVO01_METRIC_COLUMNS,
    EXP03_GATE_COLUMNS,
    RUNNING_COMPARE_COLUMNS,
    load_alpha_fixed,
    load_evaluations,
    load_evo01_individuals,
    load_evo01_metrics,
    load_exp03_gates,
    load_geometric,
    load_library,
    load_map,
    load_running_compare,
    reject_stub_csv,
)
from plot_evo01 import plot_evo01
from plot_alphas_bifurcation import SCAN_COLUMNS, plot_alphas_bifurcation
from plot_alphas_propagation import PROPAGATION_COLUMNS, plot_alphas_propagation
from plot_running_compare import plot_running_compare
from plot_exp03_gates import plot_exp03_gates
from plot_observables import plot_observables
from plot_alpha_fixed import plot_alpha_fixed
from plot_geometric import plot_geometric
from plot_library import plot_library
from plot_map import plot_map

MAP_HEADER = "fitness,complexity_level,type,alpha,mass_up_quark,mass_down_quark,spatial_curvature,temporal_rate"
MAP_ROW = "0.390607,1,Químico,407.837442,3.02e-30,5.87e-30,0.6302,0.2596"


def _write(tmp: Path, name: str, text: str) -> Path:
    path = tmp / name
    path.write_text(text, encoding="utf-8")
    return path


class LoadMapTests(unittest.TestCase):
    def test_valid_schema_and_inverse_alpha(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            path = _write(tmp, "map.csv", MAP_HEADER + "\n" + MAP_ROW + "\n")
            frame = load_map(path)
            self.assertEqual(len(frame), 1)
            self.assertAlmostEqual(frame.loc[0, "inverse_alpha_stored"], 407.837442)
            reconstructed = 1.0 / 407.837442
            self.assertAlmostEqual(frame.loc[0, "alpha_si_reconstructed"], reconstructed)
            self.assertNotAlmostEqual(frame.loc[0, "alpha"], reconstructed)
            self.assertGreater(frame.loc[0, "alpha"], 1.0)

    def test_rejects_legacy_landscape_data_name_schema(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            path = _write(
                tmp,
                "landscape_data.csv",
                "fitness,winning_gen,mass_up_quark,mass_down_quark\n0.1,1,1e-30,2e-30\n",
            )
            with self.assertRaises(CsvSchemaError) as ctx:
                load_map(path)
            self.assertIn("alpha", str(ctx.exception))
            self.assertIn("Faltan", str(ctx.exception))

    def test_missing_file(self) -> None:
        with self.assertRaises(FileNotFoundError):
            load_map(Path("definitely-missing-map.csv"))

    def test_empty_header_only(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            path = _write(tmp, "empty.csv", MAP_HEADER + "\n")
            frame = load_map(path, allow_empty=True)
            self.assertTrue(frame.empty)
            with self.assertRaises(CsvSchemaError):
                load_map(path, allow_empty=False)

    def test_figures_from_tiny_map(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            csv_path = _write(tmp, "map.csv", MAP_HEADER + "\n" + MAP_ROW + "\n")
            out = tmp / "fig"
            paths = plot_map(csv_path, out)
            self.assertTrue(any(p.name == "map_fitness_hist.png" for p in paths))
            self.assertTrue(any(p.suffix == ".html" for p in paths))
            for path in paths:
                self.assertGreater(path.stat().st_size, 0)


class LoadOtherModesTests(unittest.TestCase):
    def test_library_schema(self) -> None:
        header = (
            "alpha_denominator,story_quality,narrative_depth,finale_type,reproducibility,"
            "dramatic_events_count,theoretical_proton_mass_kg,theoretical_mass_diff_mev,"
            "jeans_mass_kg,stellar_lifetime_years,stellar_score"
        )
        row = "137,0.55,1,Chemical Whispers,1.0,2,9e-28,1.2,1e32,1e5,0.0"
        with tempfile.TemporaryDirectory() as raw:
            path = _write(Path(raw), "lib.csv", header + "\n" + row + "\n")
            frame = load_library(path)
            self.assertEqual(int(frame.loc[0, "alpha_denominator"]), 137)

    def test_alpha_fixed_and_geometric_figures(self) -> None:
        af_header = "sample_id,derived_e,fitness,complexity_level,c_variant,hbar_variant,epsilon0_variant"
        af_row = "0,6.8e-19,0.35,1,9.43e8,2.34e-34,2.32e-11"
        geo_header = (
            "sample_id,spatial_curvature,dim_ratio_1,dim_ratio_2,dim_ratio_3,"
            "dim_ratio_4,temporal_rate,fitness,complexity_level"
        )
        geo_row = "0,1.26,4.77,0.17,0.95,1.59,26.8,0.39,1"
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            af = _write(tmp, "af.csv", af_header + "\n" + af_row + "\n")
            geo = _write(tmp, "geo.csv", geo_header + "\n" + geo_row + "\n")
            self.assertGreater(len(plot_alpha_fixed(af, tmp / "af")), 0)
            self.assertGreater(len(plot_geometric(geo, tmp / "geo")), 0)
            lib_header = (
                "alpha_denominator,story_quality,narrative_depth,finale_type,reproducibility,"
                "dramatic_events_count,theoretical_proton_mass_kg,theoretical_mass_diff_mev,"
                "jeans_mass_kg,stellar_lifetime_years,stellar_score"
            )
            lib_row = "137,0.55,1,Chemical Whispers,1.0,2,9e-28,1.2,1e32,1e5,0.0"
            lib = _write(tmp, "lib.csv", lib_header + "\n" + lib_row + "\n")
            self.assertGreater(len(plot_library(lib, tmp / "lib")), 0)

    def test_stubs_do_not_invent_files(self) -> None:
        with self.assertRaises(FileNotFoundError) as ctx:
            reject_stub_csv("harmonic")
        self.assertIn("stub", str(ctx.exception))


class FindAdamTests(unittest.TestCase):
    def test_does_not_inject_placeholder_constants(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            path = _write(tmp, "map.csv", MAP_HEADER + "\n" + MAP_ROW + "\n")
            out = tmp / "adam.json"
            record = find_adam(path, out)
            self.assertNotIn("G", record)
            self.assertNotIn("e", record)
            payload = json.loads(out.read_text(encoding="utf-8"))
            self.assertIn("1/α", payload["note"])


EVAL_HEADER = ",".join(EVALUATION_COLUMNS)
EVAL_ROW = (
    "0,137137,6.6743000000000000e-11,1.6021766340000000e-19,7.2973525737492600e-03,1.3703599918000000e+02,1.1810000000000000e-01,"
    "9.1187600000000000e+01,8.9223830337127490e-28,8.9457787358688820e-28,"
    "2.3395702156133200e-30,6.0000000000000000e-28,8.9223830337127490e-28,"
    "8.9457787358688820e-28,6.0000000000000000e-28,0,1.2340000000000000e+17,"
    "1.0000000000000000e+32,,,1,1.0000000000000000e+00,9.7775732521619770e-01,"
    "0.0000000000000000e+00,,,0,1,3.9443933130404940e-01,3.9443933130404940e-01"
)
EVAL_NONE_ROW = (
    "3,137137,6.6743000000000000e-11,1.6021766340000000e-19,7.2973525737492600e-03,1.3703599918000000e+02,1.1810000000000000e-01,"
    "9.1187600000000000e+01,8.9223830337127490e-28,8.9457787358688820e-28,"
    "2.3395702156133200e-30,6.0000000000000000e-28,1.6726219236900000e-27,"
    "1.6749274980400000e-27,2.4883541700000000e-28,1,,,,,0,,,,,,0,0,"
    "0.0000000000000000e+00,0.0000000000000000e+00"
)
EVAL_NAN_ROW = (
    "4,137137,6.6743000000000000e-11,1.6021766340000000e-19,nan,inf,1.1810000000000000e-01,9.1187600000000000e+01,"
    "8.9223830337127490e-28,8.9457787358688820e-28,2.3395702156133200e-30,"
    "6.0000000000000000e-28,8.9223830337127490e-28,8.9457787358688820e-28,"
    "6.0000000000000000e-28,0,1.2340000000000000e+17,1.0000000000000000e+32,,,"
    "1,1.0000000000000000e+00,9.7775732521619770e-01,0.0000000000000000e+00,,,"
    "0,1,3.9443933130404940e-01,3.9443933130404940e-01"
)


class EvaluationCsvTests(unittest.TestCase):
    def test_none_stays_nan_not_zero(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            path = _write(Path(raw), "ev.csv", EVAL_HEADER + "\n" + EVAL_NONE_ROW + "\n")
            frame = load_evaluations(path)
            self.assertEqual(len(frame), 1)
            self.assertTrue(pd.isna(frame.loc[0, "hubble_time_s"]))
            self.assertTrue(pd.isna(frame.loc[0, "jeans_mass_kg"]))
            self.assertTrue(pd.isna(frame.loc[0, "coulomb_barrier_j"]))
            self.assertTrue(pd.isna(frame.loc[0, "chandrasekhar_mass_kg"]))
            self.assertTrue(pd.isna(frame.loc[0, "stellar_term"]))
            self.assertEqual(frame.loc[0, "used_pdg_hadron_fallback"], 1)
            self.assertEqual(frame.loc[0, "fitness"], 0.0)
            self.assertNotEqual(frame.loc[0, "hubble_time_s"], 0.0)

    def test_zero_stellar_is_not_none(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            path = _write(Path(raw), "ev.csv", EVAL_HEADER + "\n" + EVAL_ROW + "\n")
            frame = load_evaluations(path)
            self.assertEqual(frame.loc[0, "stellar_term"], 0.0)
            self.assertFalse(pd.isna(frame.loc[0, "stellar_term"]))
            self.assertTrue(pd.isna(frame.loc[0, "coulomb_barrier_j"]))
            self.assertEqual(frame.loc[0, "sample_id"], 0)
            self.assertEqual(frame.loc[0, "rng_seed"], 137137)
            self.assertAlmostEqual(frame.loc[0, "g_si"], 6.6743e-11)
            self.assertAlmostEqual(frame.loc[0, "e_si"], 1.602176634e-19)

    def test_nonfinite_tokens(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            path = _write(Path(raw), "ev.csv", EVAL_HEADER + "\n" + EVAL_NAN_ROW + "\n")
            frame = load_evaluations(path)
            self.assertTrue(pd.isna(frame.loc[0, "alpha_si"]))
            self.assertTrue(float(frame.loc[0, "inverse_alpha_si"]) == float("inf"))

    def test_rejects_map_schema(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            path = _write(Path(raw), "map.csv", MAP_HEADER + "\n" + MAP_ROW + "\n")
            with self.assertRaises(CsvSchemaError):
                load_evaluations(path)

    def test_html_figures_and_sample_card(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            path = _write(
                tmp,
                "ev.csv",
                EVAL_HEADER + "\n" + EVAL_ROW + "\n" + EVAL_NONE_ROW + "\n",
            )
            out = tmp / "fig"
            paths = plot_observables(path, out, seed_label="137137", sample_id=3)
            names = {p.name for p in paths}
            self.assertIn("obs_fitness_hist.html", names)
            self.assertIn("obs_complexity_hist.html", names)
            self.assertIn("obs_alpha_vs_proton.html", names)
            self.assertIn("obs_alphas_vs_npdiff.html", names)
            self.assertIn("obs_scatter_matrix.html", names)
            self.assertIn("obs_sample_card.html", names)
            self.assertIn("obs_jeans.html", names)
            card = (out / "obs_sample_card.html").read_text(encoding="utf-8")
            self.assertIn("sample_id=3", card)
            for item in paths:
                self.assertGreater(item.stat().st_size, 0)


SCAN_HEADER = ",".join(SCAN_COLUMNS)
SCAN_ROW_RUN = (
    "1.2000000000000000e-01,2.2447200000000000e-01,1,5.3450000000000000e-01,"
    "8.9000000000000000e-28,8.9200000000000000e-28,2.4000000000000000e-30,0"
)
SCAN_ROW_CUT = (
    "1.5000000000000000e-01,-3.0000000000000000e-02,0,1.5000000000000000e-01,"
    "8.8000000000000000e-28,8.8200000000000000e-28,2.2000000000000000e-30,0"
)


class AlphasBifurcationPlotTests(unittest.TestCase):
    def test_overlay_html_mentions_code_branch_not_qcd_transition(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            ev = _write(
                tmp,
                "ev.csv",
                EVAL_HEADER + "\n" + EVAL_ROW + "\n" + EVAL_NONE_ROW + "\n",
            )
            scan = _write(
                tmp,
                "scan.csv",
                SCAN_HEADER + "\n" + SCAN_ROW_RUN + "\n" + SCAN_ROW_CUT + "\n",
            )
            out = tmp / "exp01_alphas_bifurcation.html"
            path = plot_alphas_bifurcation(ev, scan, out)
            text = path.read_text(encoding="utf-8")
            self.assertGreater(path.stat().st_size, 0)
            self.assertIn("running_alpha_s", text)
            self.assertTrue("denom \\u003e 0.1" in text or "denom > 0.1" in text)
            self.assertNotIn("obs_alphas_vs_npdiff.html", text)


PROP_HEADER = ",".join(PROPAGATION_COLUMNS)
PROP_ROW_RUN = (
    SCAN_ROW_RUN
    + ",6.0000000000000000e-28,8.9000000000000000e-28,1,1.0000000000000000e+00,"
    "9.0000000000000000e-01,1.0000000000000000e+00,1.0000000000000000e-02,,"
    "0,1,3.9000000000000000e-01,3.9000000000000000e-01,1.0000000000000000e+32,0"
)
PROP_ROW_CUT = (
    SCAN_ROW_CUT
    + ",6.0000000000000000e-28,8.8000000000000000e-28,1,1.0000000000000000e+00,"
    "8.0000000000000000e-01,1.0000000000000000e+00,8.0000000000000000e-03,,"
    "0,1,3.8000000000000000e-01,3.8000000000000000e-01,1.0000000000000000e+32,0"
)


class AlphasPropagationPlotTests(unittest.TestCase):
    def test_four_panels_mark_code_threshold_not_qcd_transition(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            scan = _write(
                tmp,
                "scan.csv",
                PROP_HEADER + "\n" + PROP_ROW_RUN + "\n" + PROP_ROW_CUT + "\n",
            )
            out = tmp / "exp02_qcd_propagation.html"
            path = plot_alphas_propagation(scan, out)
            text = path.read_text(encoding="utf-8")
            self.assertGreater(path.stat().st_size, 0)
            self.assertIn("HAR-EXP-02", text)
            self.assertTrue("denominator" in text)
            self.assertIn("nucleosynthesis_term", text)
            self.assertNotIn("obs_alphas_vs_npdiff.html", text)


RC_HEADER = ",".join(RUNNING_COMPARE_COLUMNS)


def _rc_row(eval_row: str, variant: str, eps: str, denom: str, run: str, sample: str) -> str:
    parts = eval_row.split(",")
    parts[0] = sample
    return ",".join([parts[0], parts[1], variant, eps, denom, run] + parts[2:])


class RunningComparePlotTests(unittest.TestCase):
    def test_loader_keeps_empty_epsilon_and_none_stages(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            row = _rc_row(EVAL_NONE_ROW, "A", "", "1.0000000000000000e-01", "1.1810000000000000e-01", "0")
            path = _write(tmp, "A.csv", RC_HEADER + "\n" + row + "\n")
            frame = load_running_compare(path)
            self.assertEqual(frame.loc[0, "variant"], "A")
            self.assertTrue(pd.isna(frame.loc[0, "epsilon"]))
            self.assertTrue(pd.isna(frame.loc[0, "stellar_term"]))
            self.assertNotEqual(frame.loc[0, "stellar_term"], 0)

    def test_five_figures_caption_not_qcd(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)

            def write_variant(name: str, variant: str, eps: str, run: str, fit_suffix: str) -> Path:
                r0 = _rc_row(EVAL_ROW, variant, eps, "2.0000000000000000e-01", run, "0")
                r1 = _rc_row(EVAL_NONE_ROW, variant, eps, "5.0000000000000000e-02", run, "1")
                r0 = r0.rsplit(",", 1)[0] + "," + fit_suffix
                return _write(tmp, name, RC_HEADER + "\n" + r0 + "\n" + r1 + "\n")

            a = write_variant("variant_A_historical.csv", "A", "", "5.9000000000000000e-01", "3.9443933130404940e-01")
            b = write_variant("variant_B_smooth_eps0p02.csv", "B", "2.0000000000000000e-02", "6.0000000000000000e-01", "4.0000000000000000e-01")
            d = write_variant("variant_D_constant.csv", "D", "", "1.1810000000000000e-01", "3.5000000000000000e-01")
            blo = write_variant("variant_B_smooth_eps0p005.csv", "B", "5.0000000000000000e-03", "5.9500000000000000e-01", "3.9600000000000000e-01")
            bhi = write_variant("variant_B_smooth_eps0p05.csv", "B", "5.0000000000000000e-02", "6.2000000000000000e-01", "4.1000000000000000e-01")
            out = tmp / "figs"
            paths = plot_running_compare(a, b, d, blo, bhi, out, expected_n=2)
            self.assertEqual(len(paths), 5)
            text = paths["alphas"].read_text(encoding="utf-8")
            self.assertIn("HAR-MODEL-01B", text)
            self.assertIn("sample_id", text)
            self.assertNotIn("QCD de laboratorio", text)
            self.assertTrue((out / "paired_summary.json").is_file())


E3_HEADER = ",".join(EXP03_GATE_COLUMNS)


def _e3_row(eval_row: str, sample: str, variant: str, level: str, stellar: str, heavy: str, jeans: str) -> str:
    parts = eval_row.split(",")
    parts[0] = sample
    extras = [variant, jeans, "1.0000000000000000e+15", "3.1688087814028950e+07", "in_window", "vis01"]
    body = parts[2:]
    # complexity_level is index -2 of evaluation body (last two are fitness_uncapped, fitness)
    # evaluation: after seed, 28 fields; complexity_level is field 26 of full eval = body[24]
    return ",".join([parts[0], parts[1]] + extras + body)


class Exp03PlotTests(unittest.TestCase):
    def test_loader_keeps_none_heavy(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            row = _e3_row(EVAL_NONE_ROW, "219", "A", "1", "0", "", "8e31")
            path = _write(tmp, "a.csv", E3_HEADER + "\n" + row + "\n")
            frame = load_exp03_gates(path)
            self.assertEqual(frame.loc[0, "variant"], "A")
            self.assertTrue(pd.isna(frame.loc[0, "heavy_element_term"]))
            self.assertEqual(frame.loc[0, "lifetime_branch"], "in_window")

    def test_five_html_not_habitability(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)

            def two(name: str, variant: str, st: str, hv: str, lv_a: str, lv_b: str) -> Path:
                r0 = _e3_row(EVAL_ROW, "219", variant, lv_a, st, hv, "7e31")
                r1 = _e3_row(EVAL_ROW, "351", variant, lv_b, st, hv, "7e31")
                return _write(tmp, name, E3_HEADER + "\n" + r0 + "\n" + r1 + "\n")

            a = two("vis01_gates_A.csv", "A", "0.0", "", "1", "1")
            d = two("vis01_gates_D.csv", "D", "1.0", "1.0", "5", "5")
            scan = two("scan.csv", "A", "1.0", "0.0", "2", "2")
            out = tmp / "figs"
            paths = plot_exp03_gates(a, d, scan, scan, scan, scan, scan, out)
            self.assertGreaterEqual(len(paths), 5)
            text = paths["levels"].read_text(encoding="utf-8")
            self.assertIn("HAR-EXP-03", text)
            ad = paths["ad"].read_text(encoding="utf-8")
            self.assertTrue("219" in ad)
            self.assertNotIn("habitabilidad física demostrada", text)


E1I = ",".join(EVO01_INDIVIDUAL_COLUMNS)
E1M = ",".join(EVO01_METRIC_COLUMNS)


def _e1_ind(arm: str, gen: str, uid: str, parent: str, slot: str, fit: str, level: str) -> str:
    return ",".join(
        [
            "evo01-n2-t1-base137137",
            arm,
            gen,
            uid,
            parent,
            slot,
            "7.2970000000000000e-03",
            "1.1800000000000000e-01",
            "2.0000000000000000e+00",
            "1.6020000000000000e-19",
            "1.3340000000000000e-10",
            fit,
            level,
            "ok",
            "1.0000000000000000e+00",
            "",
            "",
        ]
    )


def _e1_met(arm: str, gen: str, best: str, med: str, div: str, border: str) -> str:
    return ",".join(
        [
            "evo01-n2-t1-base137137",
            arm,
            gen,
            best,
            med,
            "1.0000000000000000e-01",
            "4.0000000000000000e-01",
            "3.0000000000000000e-01",
            "0.0000000000000000e+00",
            "5.0000000000000000e-01",
            "5.0000000000000000e-01",
            "0.0000000000000000e+00",
            "0.0000000000000000e+00",
            "0.0000000000000000e+00",
            div,
            "2.0000000000000000e-01",
            border,
            "0.0000000000000000e+00",
            "1.0000000000000000e+00",
            "5.0000000000000000e-01",
        ]
    )


class Evo01PlotTests(unittest.TestCase):
    def test_loader_keeps_empty_parent_id(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            rows = [
                _e1_ind("selection", "0", "0", "", "0", "0.2", "1"),
                _e1_ind("selection", "0", "1", "", "1", "0.3", "1"),
            ]
            path = _write(tmp, "ind.csv", E1I + "\n" + "\n".join(rows) + "\n")
            frame = load_evo01_individuals(path)
            self.assertTrue(pd.isna(frame.loc[0, "parent_id"]))
            self.assertEqual(frame.loc[0, "arm"], "selection")

    def test_five_html_not_habitability(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            inds = [
                _e1_ind("selection", "0", "0", "", "0", "0.20", "1"),
                _e1_ind("selection", "0", "1", "", "1", "0.30", "1"),
                _e1_ind("selection", "1", "268435456", "0", "0", "0.40", "2"),
                _e1_ind("selection", "1", "268435457", "1", "1", "0.50", "2"),
                _e1_ind("control", "0", "72057594037927936", "", "0", "0.20", "1"),
                _e1_ind("control", "0", "72057594037927937", "", "1", "0.30", "1"),
                _e1_ind("control", "1", "72339069014638592", "72057594037927936", "0", "0.25", "1"),
                _e1_ind("control", "1", "72339069014638593", "72057594037927937", "1", "0.28", "1"),
            ]
            mets = [
                _e1_met("selection", "0", "0.30", "0.25", "0.10", "0.00"),
                _e1_met("selection", "1", "0.50", "0.45", "0.08", "0.20"),
                _e1_met("control", "0", "0.30", "0.25", "0.10", "0.00"),
                _e1_met("control", "1", "0.28", "0.26", "0.09", "0.10"),
            ]
            ind_p = _write(tmp, "evo01_individuals.csv", E1I + "\n" + "\n".join(inds) + "\n")
            met_p = _write(tmp, "evo01_metrics.csv", E1M + "\n" + "\n".join(mets) + "\n")
            out = tmp / "figs"
            paths = plot_evo01(ind_p, met_p, out)
            self.assertEqual(len(paths), 5)
            text = paths["fitness"].read_text(encoding="utf-8")
            self.assertIn("HAR-EVO-01", text)
            self.assertIn("selection", text)
            self.assertNotIn("habitabilidad física demostrada", text)
            self.assertIn("alpha_s", paths["genes"].read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
