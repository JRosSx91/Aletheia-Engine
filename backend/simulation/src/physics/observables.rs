//! Engine outputs split into calculated quantities and the historical ranking index.
//!
//! These are software records, not laboratory-validated QCD/QED.

use crate::core::models::CosmicLaw;

/// Results taken from the engine (existing fields/methods), with units noted in names.
/// Not a claim of physical accuracy.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelObservables {
    /// Low-energy SI α stored at construction: \(e^2/(4\pi\varepsilon_0\hbar c)\). Dimensionless.
    pub alpha: f64,
    /// `CosmicLaw.alpha_s`, treated as α_s(μ_ref). Dimensionless.
    pub alpha_s_reference: f64,
    /// μ_ref used by `running_alpha_s`, in GeV (`AdvancedPhysicsEngine.reference_scale`).
    pub alpha_s_mu_ref_gev: f64,
    /// From `get_theoretical_hadron_masses`. kg.
    pub theoretical_proton_mass_kg: f64,
    /// From `get_theoretical_hadron_masses`. kg.
    pub theoretical_neutron_mass_kg: f64,
    /// m_n − m_p from the theoretical tuple. kg.
    pub theoretical_np_mass_diff_kg: f64,
    /// From `get_validated_hadron_masses` (theoretical or PDG). kg.
    pub validated_proton_mass_kg: f64,
    /// From `get_validated_hadron_masses`. kg.
    pub validated_neutron_mass_kg: f64,
    /// Third element of the theoretical hadron tuple. kg.
    pub theoretical_pion_mass_kg: f64,
    /// Third element of the validated hadron tuple (theoretical or PDG). kg.
    pub validated_pion_mass_kg: f64,
    /// True when `get_validated_hadron_masses` took the empirical/PDG arm
    /// (`empirical_validation` or the theoretical validity cut), not inferred from mass equality.
    pub used_pdg_hadron_fallback: bool,
    /// `hubble_time_approx` inside cosmology, seconds. `None` if that line did not run
    /// (nucleon gate, or freefall cut returning before Hubble).
    pub hubble_time_s: Option<f64>,
    /// Jeans mass used by the stellar stage. kg. `None` if that stage was not entered.
    pub jeans_mass_kg: Option<f64>,
    /// Coulomb barrier used by the heavy-element stage. J. `None` if that stage was not entered.
    pub coulomb_barrier_j: Option<f64>,
    /// Chandrasekhar mass used by the black-hole stage. kg. `None` if that stage was not entered.
    pub chandrasekhar_mass_kg: Option<f64>,
}

/// Historical ranking pieces. `fitness` is an index, not a physical observable.
#[derive(Debug, Clone, PartialEq)]
pub struct HeuristicBreakdown {
    pub nucleon_stability_gate_passed: bool,
    /// `None` if the nucleon/electron gate returned before cosmology.
    pub cosmology_term: Option<f64>,
    /// `None` if cosmology cut returned before BBN.
    pub nucleosynthesis_term: Option<f64>,
    /// `None` if BBN score ≤ 0.3 (stage not entered).
    pub stellar_term: Option<f64>,
    /// `None` if stellar score ≤ 0.4 (stage not entered).
    pub heavy_element_term: Option<f64>,
    /// `None` if heavy score ≤ 0.5 (stage not entered).
    pub black_hole_term: Option<f64>,
    pub level_bonus_applied: bool,
    pub complexity_level: u8,
    /// Weighted sum before `.min(1.0)`.
    pub fitness_uncapped: f64,
    /// Historical `calculate_enhanced_fitness` first return value.
    pub fitness: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineEvaluation {
    pub observables: ModelObservables,
    pub heuristic: HeuristicBreakdown,
}

/// Optional exploratory export. Not the historical `map` schema.
/// Empty cells mean `None` (stage not run), never a fabricated zero.
pub const EVALUATION_CSV_HEADERS: [&str; 30] = [
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
];

fn csv_f64(x: f64) -> String {
    if x.is_nan() {
        "nan".to_string()
    } else if x.is_infinite() {
        if x.is_sign_positive() {
            "inf".to_string()
        } else {
            "-inf".to_string()
        }
    } else {
        format!("{:.16e}", x)
    }
}

fn csv_opt_f64(x: Option<f64>) -> String {
    match x {
        Some(v) => csv_f64(v),
        None => String::new(),
    }
}

fn csv_bool(x: bool) -> String {
    if x {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

/// One row for the optional evaluation CSV. `rng_seed` empty when the run was unseeded.
/// `g_si` / `e_si` are copied from the sampled `CosmicLaw`, not reconstructed from α.
pub fn evaluation_csv_record(
    sample_id: u64,
    rng_seed: Option<u64>,
    laws: &CosmicLaw,
    ev: &EngineEvaluation,
) -> Vec<String> {
    let o = &ev.observables;
    let h = &ev.heuristic;
    let inverse = if o.alpha == 0.0 {
        csv_f64(f64::INFINITY)
    } else {
        csv_f64(1.0 / o.alpha)
    };
    vec![
        sample_id.to_string(),
        rng_seed.map(|s| s.to_string()).unwrap_or_default(),
        csv_f64(laws.g),
        csv_f64(laws.e),
        csv_f64(o.alpha),
        inverse,
        csv_f64(o.alpha_s_reference),
        csv_f64(o.alpha_s_mu_ref_gev),
        csv_f64(o.theoretical_proton_mass_kg),
        csv_f64(o.theoretical_neutron_mass_kg),
        csv_f64(o.theoretical_np_mass_diff_kg),
        csv_f64(o.theoretical_pion_mass_kg),
        csv_f64(o.validated_proton_mass_kg),
        csv_f64(o.validated_neutron_mass_kg),
        csv_f64(o.validated_pion_mass_kg),
        csv_bool(o.used_pdg_hadron_fallback),
        csv_opt_f64(o.hubble_time_s),
        csv_opt_f64(o.jeans_mass_kg),
        csv_opt_f64(o.coulomb_barrier_j),
        csv_opt_f64(o.chandrasekhar_mass_kg),
        csv_bool(h.nucleon_stability_gate_passed),
        csv_opt_f64(h.cosmology_term),
        csv_opt_f64(h.nucleosynthesis_term),
        csv_opt_f64(h.stellar_term),
        csv_opt_f64(h.heavy_element_term),
        csv_opt_f64(h.black_hole_term),
        csv_bool(h.level_bonus_applied),
        h.complexity_level.to_string(),
        csv_f64(h.fitness_uncapped),
        csv_f64(h.fitness),
    ]
}
