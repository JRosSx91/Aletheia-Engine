//! Engine outputs split into calculated quantities and the historical ranking index.
//!
//! These are software records, not laboratory-validated QCD/QED.

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
    /// True when `get_validated_hadron_masses` took the empirical/PDG arm
    /// (`empirical_validation` or the theoretical validity cut), not inferred from mass equality.
    pub used_pdg_hadron_fallback: bool,
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
