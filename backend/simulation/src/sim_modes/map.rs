use crate::core::models::CosmicLaw;
use crate::physics::engine::calculate_enhanced_fitness;
use crate::physics::constants::*;
use crate::utils::analyze_universe_type;
use std::error::Error;
use rand::prelude::*;
use csv;
use std::f64::consts::PI;

pub fn run_mapping_mode(num_universes: u64) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let mut wtr = csv::Writer::from_path("landscape_data_enhanced.csv")?;
    wtr.write_record(&[
        "fitness",
        "complexity_level",
        "type",
        "alpha",
        "mass_up_quark",
        "mass_down_quark",
        "spatial_curvature",
        "temporal_rate",
    ])?;

    println!(
        "🗺️ Iniciando mapeo mejorado de {} universos...",
        num_universes
    );

    for i in 0..=num_universes {
        let random_laws = CosmicLaw {
            g: rng.gen_range(6.674e-11..6.674e-10),
            e: rng.gen_range(0.5e-19..2.5e-19),
            alpha_s: rng.gen_range(0.05..2.0),
            alpha_w: rng.gen_range(1.0e-9..1.0e-4),

            mass_up_quark: rng.gen_range(1.0e-30..6.0e-30),
            mass_down_quark: rng.gen_range(1.0e-30..1.3e-29),
            mass_electron: rng.gen_range(1.0e-31..1.0e-30),

            mass_charm_quark: rng.gen(),
            mass_strange_quark: rng.gen(),
            mass_muon: rng.gen(),
            mass_top_quark: rng.gen(),
            mass_bottom_quark: rng.gen(),
            mass_tauon: rng.gen(),

            spatial_curvature: rng.gen_range(-1.0..1.0),
            dimensional_ratios: [rng.gen(); 4],
            temporal_evolution_rate: rng.gen_range(0.1..10.0),
            dark_energy_density: rng.gen_range(0.0..2.0e-29),
            dark_matter_coupling: rng.gen_range(0.0..1.0),
        };

        let (fitness, level) = calculate_enhanced_fitness(&random_laws);
        let alpha = random_laws.e.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C);

        if fitness > 0.01 {
            let universe_type = analyze_universe_type(level);
            wtr.write_record(&[
                format!("{:.6}", fitness),
                level.to_string(),
                universe_type.to_string(),
                format!("{:.6}", 1.0 / alpha),
                format!("{:e}", random_laws.mass_up_quark),
                format!("{:e}", random_laws.mass_down_quark),
                format!("{:.4}", random_laws.spatial_curvature),
                format!("{:.4}", random_laws.temporal_evolution_rate),
            ])?;
        }

        if i > 0 && i % 100_000 == 0 {
            let (fitness, level) = calculate_enhanced_fitness(&random_laws);
            let universe_type = analyze_universe_type(level);
            println!(
                "Universo #{}... Fitness: {:.4}, Tipo: {}",
                i, fitness, universe_type
            );
        }
    }

    wtr.flush()?;
    println!("🎯 === MAPEO MEJORADO COMPLETADO ===");
    Ok(())
}
