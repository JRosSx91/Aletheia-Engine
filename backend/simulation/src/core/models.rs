use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CosmicLaw {
    #[allow(non_snake_case)]
    pub g: f64,
    pub e: f64,
    pub alpha_s: f64,
    pub alpha_w: f64,

    pub mass_up_quark: f64,
    pub mass_down_quark: f64,
    pub mass_electron: f64,

    pub mass_charm_quark: f64,
    pub mass_strange_quark: f64,
    pub mass_muon: f64,

    pub mass_top_quark: f64,
    pub mass_bottom_quark: f64,
    pub mass_tauon: f64,

    pub spatial_curvature: f64,
    pub dimensional_ratios: [f64; 4],
    pub temporal_evolution_rate: f64,

    pub dark_energy_density: f64,
    pub dark_matter_coupling: f64,
}

/// How `calculate_enhanced_fitness` consumes `CosmicLaw` today (implementation contract).
///
/// Not a physical classification and not a duplicate of the struct. CSV/narrative
/// may still record ignored fields. Limitations: `g` is both a continuous input
/// (Jeans, Chandrasekhar) and a binary cosmological cut; saturations can hide
/// an otherwise direct dependence; `mass_up_quark` / `mass_down_quark` are passed
/// into hadron helpers and then replaced by fixed chiral masses.
///
/// Direct (formula, can move the score on an open set): `g`, `e`, `alpha_s`.
/// Gate / threshold only: `mass_electron` (β-stability cut), `dark_energy_density`
/// (Hubble 0/1). `g` also participates in that 0/1 cut.
/// Ignored by fitness: `alpha_w`, heavy quark and lepton masses, `spatial_curvature`,
/// `dimensional_ratios`, `temporal_evolution_rate`, `dark_matter_coupling`, and
/// u/d masses in practice.
/// CSV / narrative only (subset of ignored): map writes u/d, curvature, `temporal_rate`;
/// geometric writes curvature and ratios; alpha-fixed writes EM variants that never
/// enter `CosmicLaw`.
impl CosmicLaw {
    /// Overwrite every field the historical fitness path does not read.
    #[cfg(test)]
    pub fn scramble_fitness_ignored_fields(&mut self) {
        self.alpha_w = 0.42;
        self.mass_up_quark = 9.9e-20;
        self.mass_down_quark = 8.8e-20;
        self.mass_strange_quark = 7.7;
        self.mass_charm_quark = 6.6;
        self.mass_top_quark = 5.5;
        self.mass_bottom_quark = 4.4;
        self.mass_muon = 3.3;
        self.mass_tauon = 2.2;
        self.spatial_curvature = -9.0;
        self.dimensional_ratios = [9.0, 8.0, 7.0, 6.0];
        self.temporal_evolution_rate = 123.0;
        self.dark_matter_coupling = 0.0;
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CosmicNarrative {
    pub story_quality: f64,
    pub narrative_depth: u8,
    pub dramatic_events: Vec<String>,
    pub finale_type: String,
    pub reproducibility: f64,
}
