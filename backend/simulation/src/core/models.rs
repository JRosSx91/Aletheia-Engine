use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
pub struct CosmicNarrative {
    pub story_quality: f64,  
    pub narrative_depth: u8,         
    pub dramatic_events: Vec<String>,
    pub finale_type: String,
    pub reproducibility: f64,
}
