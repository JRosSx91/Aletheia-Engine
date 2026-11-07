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
    load_alpha_fixed,
    load_evaluations,
    load_geometric,
    load_library,
    load_map,
    reject_stub_csv,
)
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


if __name__ == "__main__":
    unittest.main()
