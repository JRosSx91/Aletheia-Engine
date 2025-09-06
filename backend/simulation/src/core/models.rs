use serde::{Deserialize, Serialize};

// pub struct ParticleBlueprint {
    // name: &'static str,
    // spin: f64,
    // charge_fraction: (i32, i32), // (numerator, denominator) for fractional charges
// }

// lazy_static::lazy_static! {
    // static ref UP_QUARK: ParticleBlueprint = ParticleBlueprint {
       // name: "Up", spin: 0.5, charge_fraction: (2, 3)
    // };
    // static ref DOWN_QUARK: ParticleBlueprint = ParticleBlueprint {
       // name: "Down", spin: 0.5, charge_fraction: (-1, 3)
    // };
    // static ref ELECTRON: ParticleBlueprint = ParticleBlueprint {
       // name: "Electron", spin: 0.5, charge_fraction: (-1, 1)
    // };
// }

// --- LEVEL 2: GENOMA CÓSMICO EXPANDIDO ---
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CosmicLaw {
    #[allow(non_snake_case)]
    pub g: f64,
    pub e: f64,
    pub alpha_s: f64,
    pub alpha_w: f64,

    // Masas de partículas (Generación 1)
    pub mass_up_quark: f64,
    pub mass_down_quark: f64,
    pub mass_electron: f64,

    // Masas de partículas (Generación 2)
    pub mass_charm_quark: f64,
    pub mass_strange_quark: f64,
    pub mass_muon: f64,

    // Masas de partículas (Generación 3)
    pub mass_top_quark: f64,
    pub mass_bottom_quark: f64,
    pub mass_tauon: f64,

    // NUEVO: Parámetros geométricos 5D
    pub spatial_curvature: f64,
    pub dimensional_ratios: [f64; 4],
    pub temporal_evolution_rate: f64,

    // NUEVO: Parámetros cosmológicos
    pub dark_energy_density: f64,
    pub dark_matter_coupling: f64,
}

// --- NIVEL 3: NARRATIVA CÓSMICA ---
#[derive(Debug, Clone, Serialize)]
pub struct CosmicNarrative {
    pub story_quality: f64,           // Qué tan "interesante" es la historia
    pub narrative_depth: u8,          // Cuántos capítulos de complejidad
    pub dramatic_events: Vec<String>, // Los momentos clave de la historia
    pub finale_type: String,          // Cómo termina la historia
    pub reproducibility: f64,         // Potencial para generar secuelas (agujeros negros)
}
