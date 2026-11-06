"""Schema and loader tests for Harmonic-137 CSV helpers. No cosmology claims."""

from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from find_adam import find_adam
from harmonic_csv import (
    CsvSchemaError,
    load_alpha_fixed,
    load_geometric,
    load_library,
    load_map,
    reject_stub_csv,
)
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


if __name__ == "__main__":
    unittest.main()
