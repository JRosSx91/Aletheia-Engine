"""Load and validate Harmonic-137 CSV files without changing source values.

The Rust `map` writer stores **1/α** (SI definition, using the global
constants c, ħ, ε₀) in a column historically named `alpha`. This module
never silently renames that column in the returned frame; it only adds
explicit derived copies when requested.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

import pandas as pd

MAP_FILENAME = "landscape_data_enhanced.csv"
LIBRARY_FILENAME = "cosmic_library.csv"
ALPHA_FIXED_FILENAME = "alpha_fixed_analysis.csv"
GEOMETRIC_FILENAME = "geometric_analysis.csv"
EVALUATION_FILENAME = "engine_evaluations.csv"

# Column `alpha` is the reciprocal of the SI fine-structure constant.
MAP_COLUMNS = [
    "fitness",
    "complexity_level",
    "type",
    "alpha",
    "mass_up_quark",
    "mass_down_quark",
    "spatial_curvature",
    "temporal_rate",
]
MAP_NUMERIC = [
    "fitness",
    "complexity_level",
    "alpha",
    "mass_up_quark",
    "mass_down_quark",
    "spatial_curvature",
    "temporal_rate",
]
MAP_UNUSED_BY_FITNESS = ["spatial_curvature", "temporal_rate"]

LIBRARY_COLUMNS = [
    "alpha_denominator",
    "story_quality",
    "narrative_depth",
    "finale_type",
    "reproducibility",
    "dramatic_events_count",
    "theoretical_proton_mass_kg",
    "theoretical_mass_diff_mev",
    "jeans_mass_kg",
    "stellar_lifetime_years",
    "stellar_score",
]
LIBRARY_NUMERIC = [
    "alpha_denominator",
    "story_quality",
    "narrative_depth",
    "reproducibility",
    "dramatic_events_count",
    "theoretical_proton_mass_kg",
    "theoretical_mass_diff_mev",
    "jeans_mass_kg",
    "stellar_lifetime_years",
    "stellar_score",
]

ALPHA_FIXED_COLUMNS = [
    "sample_id",
    "derived_e",
    "fitness",
    "complexity_level",
    "c_variant",
    "hbar_variant",
    "epsilon0_variant",
]
ALPHA_FIXED_UNUSED_BY_ENGINE = ["c_variant", "hbar_variant", "epsilon0_variant"]

GEOMETRIC_COLUMNS = [
    "sample_id",
    "spatial_curvature",
    "dim_ratio_1",
    "dim_ratio_2",
    "dim_ratio_3",
    "dim_ratio_4",
    "temporal_rate",
    "fitness",
    "complexity_level",
]
GEOMETRIC_UNUSED_BY_FITNESS = [
    "spatial_curvature",
    "dim_ratio_1",
    "dim_ratio_2",
    "dim_ratio_3",
    "dim_ratio_4",
    "temporal_rate",
]

# HAR-MODEL-01B experimental CSVs. Not landscape_data_enhanced.csv / engine_evaluations.csv.
RUNNING_COMPARE_COLUMNS = [
    "sample_id",
    "rng_seed",
    "variant",
    "epsilon",
    "running_denominator",
    "running_alpha_s_1gev",
    "g_si",
    "e_si",
    "alpha_si",
    "inverse_alpha_si",
    "alpha_s_reference",
    "alpha_s_mu_ref_gev",
    "theoretical_proton_mass_kg",
    "theoretical_neutron_mass_kg",
    "theoretical_np_mass_diff_kg",
    "theoretical_pion_mass_kg",
    "validated_proton_mass_kg",
    "validated_neutron_mass_kg",
    "validated_pion_mass_kg",
    "used_pdg_hadron_fallback",
    "hubble_time_s",
    "jeans_mass_kg",
    "coulomb_barrier_j",
    "chandrasekhar_mass_kg",
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
]
RUNNING_COMPARE_NUMERIC = [c for c in RUNNING_COMPARE_COLUMNS if c != "variant"]

# Optional EngineEvaluation export. `alpha_si` is α; never named `alpha` (map's 1/α).
EVALUATION_COLUMNS = [
    "sample_id",
    "rng_seed",
    "g_si",
    "e_si",
    "alpha_si",
    "inverse_alpha_si",
    "alpha_s_reference",
    "alpha_s_mu_ref_gev",
    "theoretical_proton_mass_kg",
    "theoretical_neutron_mass_kg",
    "theoretical_np_mass_diff_kg",
    "theoretical_pion_mass_kg",
    "validated_proton_mass_kg",
    "validated_neutron_mass_kg",
    "validated_pion_mass_kg",
    "used_pdg_hadron_fallback",
    "hubble_time_s",
    "jeans_mass_kg",
    "coulomb_barrier_j",
    "chandrasekhar_mass_kg",
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
]
EVALUATION_NUMERIC = EVALUATION_COLUMNS
EVALUATION_OPTIONAL = [
    "hubble_time_s",
    "jeans_mass_kg",
    "coulomb_barrier_j",
    "chandrasekhar_mass_kg",
    "cosmology_term",
    "nucleosynthesis_term",
    "stellar_term",
    "heavy_element_term",
    "black_hole_term",
]
EVALUATION_UNITS = {
    "g_si": "m³ kg⁻¹ s⁻² (CosmicLaw.g sampled)",
    "e_si": "C (CosmicLaw.e sampled)",
    "alpha_si": "1 (SI MKSA low-energy α)",
    "inverse_alpha_si": "1 (1/α_si)",
    "alpha_s_reference": "1 (α_s at μ_ref)",
    "alpha_s_mu_ref_gev": "GeV",
    "theoretical_proton_mass_kg": "kg",
    "theoretical_neutron_mass_kg": "kg",
    "theoretical_np_mass_diff_kg": "kg",
    "theoretical_pion_mass_kg": "kg",
    "validated_proton_mass_kg": "kg",
    "validated_neutron_mass_kg": "kg",
    "validated_pion_mass_kg": "kg",
    "hubble_time_s": "s",
    "jeans_mass_kg": "kg",
    "coulomb_barrier_j": "J",
    "chandrasekhar_mass_kg": "kg",
    "fitness": "1 (heuristic index)",
}


class CsvSchemaError(ValueError):
    """CSV exists but does not match the Harmonic-137 writer schema."""


@dataclass(frozen=True)
class Schema:
    name: str
    columns: tuple[str, ...]
    numeric: tuple[str, ...]
    unused_by_fitness: tuple[str, ...] = ()
    notes: str = ""


SCHEMA_MAP = Schema(
    name="map",
    columns=tuple(MAP_COLUMNS),
    numeric=tuple(MAP_NUMERIC),
    unused_by_fitness=tuple(MAP_UNUSED_BY_FITNESS),
    notes=(
        "Column `alpha` stores 1/α from e²/(4πε₀ħc) with SI constants, "
        "formatted to 6 decimals. It is not α itself."
    ),
)
SCHEMA_LIBRARY = Schema(
    name="library",
    columns=tuple(LIBRARY_COLUMNS),
    numeric=tuple(LIBRARY_NUMERIC),
    notes=(
        "`alpha_denominator` indexes the intended book α=1/N. The physics "
        "engine still recomputes α from derived e and SI constants."
    ),
)
SCHEMA_ALPHA_FIXED = Schema(
    name="alpha-fixed",
    columns=tuple(ALPHA_FIXED_COLUMNS),
    numeric=tuple(ALPHA_FIXED_COLUMNS),
    unused_by_fitness=tuple(ALPHA_FIXED_UNUSED_BY_ENGINE),
    notes=(
        "c_variant, hbar_variant and epsilon0_variant are used only to "
        "derive e. AdvancedPhysicsEngine ignores them when computing α."
    ),
)
SCHEMA_GEOMETRIC = Schema(
    name="geometric",
    columns=tuple(GEOMETRIC_COLUMNS),
    numeric=tuple(GEOMETRIC_COLUMNS),
    unused_by_fitness=tuple(GEOMETRIC_UNUSED_BY_FITNESS),
    notes="Geometric columns are sampled but do not enter calculate_enhanced_fitness.",
)
SCHEMA_RUNNING_COMPARE = Schema(
    name="running-compare",
    columns=tuple(RUNNING_COMPARE_COLUMNS),
    numeric=tuple(RUNNING_COMPARE_NUMERIC),
    notes=(
        "HAR-MODEL-01B experimental export. variant is A/B/D; epsilon empty for A and D. "
        "Empty optional stage cells are None, not zero. Not a VIS-01 historical file."
    ),
)
SCHEMA_EVALUATION = Schema(
    name="engine-evaluation",
    columns=tuple(EVALUATION_COLUMNS),
    numeric=tuple(EVALUATION_NUMERIC),
    notes=(
        "g_si and e_si are sampled CosmicLaw inputs. "
        "alpha_si is engine α. inverse_alpha_si is 1/α_si. "
        "This file is not landscape_data_enhanced.csv; that file still names 1/α as `alpha`. "
        "Empty cells are stages that did not run (None), not zeros. "
        "Units: g m³ kg⁻¹ s⁻²; e C; masses kg; Hubble s; Coulomb J; α_s scale GeV."
    ),
)


def _missing(expected: Iterable[str], actual: Iterable[str]) -> list[str]:
    present = set(actual)
    return [c for c in expected if c not in present]


def load_csv(path: Path | str, schema: Schema, *, allow_empty: bool = True) -> pd.DataFrame:
    path = Path(path)
    if not path.is_file():
        raise FileNotFoundError(
            f"No se encontró {path}. Genera el CSV con el modo `{schema.name}` "
            f"del binario rust-universe (no sobrescribas CSV históricos)."
        )

    try:
        frame = pd.read_csv(path)
    except pd.errors.EmptyDataError as exc:
        raise CsvSchemaError(f"{path} está vacío (sin cabecera).") from exc

    missing = _missing(schema.columns, frame.columns)
    if missing:
        raise CsvSchemaError(
            f"{path} no coincide con el esquema `{schema.name}`. "
            f"Faltan columnas: {missing}. Encontradas: {list(frame.columns)}. "
            f"{schema.notes}"
        )

    for column in schema.numeric:
        converted = pd.to_numeric(frame[column], errors="coerce")
        if frame[column].notna().any() and converted.isna().all():
            raise CsvSchemaError(
                f"La columna `{column}` de {path} no es numérica."
            )
        frame[column] = converted

    if frame.empty and not allow_empty:
        raise CsvSchemaError(
            f"{path} solo tiene cabecera (0 filas). El modo `{schema.name}` "
            "no produjo registros con el filtro de fitness vigente."
        )

    return frame


def add_map_alpha_columns(frame: pd.DataFrame) -> pd.DataFrame:
    """Return a copy with explicit 1/α and reconstructed α. Source `alpha` unchanged."""
    out = frame.copy()
    out["inverse_alpha_stored"] = out["alpha"]
    out["alpha_si_reconstructed"] = 1.0 / out["alpha"]
    return out


def load_map(path: Path | str, *, allow_empty: bool = True) -> pd.DataFrame:
    frame = load_csv(path, SCHEMA_MAP, allow_empty=allow_empty)
    return add_map_alpha_columns(frame)


def load_library(path: Path | str, *, allow_empty: bool = True) -> pd.DataFrame:
    return load_csv(path, SCHEMA_LIBRARY, allow_empty=allow_empty)


def load_alpha_fixed(path: Path | str, *, allow_empty: bool = True) -> pd.DataFrame:
    return load_csv(path, SCHEMA_ALPHA_FIXED, allow_empty=allow_empty)


def load_geometric(path: Path | str, *, allow_empty: bool = True) -> pd.DataFrame:
    return load_csv(path, SCHEMA_GEOMETRIC, allow_empty=allow_empty)


def load_evaluations(path: Path | str, *, allow_empty: bool = True) -> pd.DataFrame:
    """Load EngineEvaluation CSV. Empty optional fields stay NaN; they are not filled with 0."""
    return load_csv(path, SCHEMA_EVALUATION, allow_empty=allow_empty)


def load_running_compare(path: Path | str, *, allow_empty: bool = True) -> pd.DataFrame:
    """Load a HAR-MODEL-01B variant CSV. Empty epsilon / stage cells stay NaN."""
    return load_csv(path, SCHEMA_RUNNING_COMPARE, allow_empty=allow_empty)


def reject_stub_csv(kind: str) -> None:
    raise FileNotFoundError(
        f"El modo `{kind}` es un stub: el CLI no escribe CSV. "
        "HAR-03 no fabrica harmonic_results.csv ni evolution_data.csv."
    )
