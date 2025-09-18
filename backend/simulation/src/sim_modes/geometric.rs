use crate::core::models::CosmicLaw;
use crate::physics::engine::calculate_enhanced_fitness;
use std::error::Error;
use std::collections::HashMap;
use rand::prelude::*;
use csv;

pub fn run_geometric_mode(samples: u32) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let mut wtr = csv::Writer::from_path("geometric_analysis.csv")?;

    wtr.write_record(&[
        "sample_id",
        "spatial_curvature",
        "dim_ratio_1",
        "dim_ratio_2",
        "dim_ratio_3",
        "dim_ratio_4",
        "temporal_rate",
        "fitness",
        "complexity_level",
    ])?;

    println!("📐 PREMISA 6: Análisis Geométrico de Universos 5D");
    println!("🌌 Explorando cómo la geometría 5D afecta la viabilidad 4D");

    let mut geometric_patterns = HashMap::new();

    for sample_id in 0..samples {
        let test_universe = CosmicLaw {
            g: rng.gen_range(6.674e-11..6.674e-10),
            e: rng.gen_range(0.5e-19..2.5e-19),
            alpha_s: rng.gen_range(0.05..2.0),
            alpha_w: rng.gen_range(1.0e-9..1.0e-4),

            mass_up_quark: rng.gen_range(1.0e-30..6.0e-30),
            mass_down_quark: rng.gen_range(1.0e-30..1.3e-29),
            mass_electron: rng.gen_range(1.0e-31..1.0e-30),

            mass_charm_quark: rng.gen_range(1.0e-29..1.0e-27),
            mass_strange_quark: rng.gen_range(1.0e-29..1.0e-28),
            mass_muon: rng.gen_range(1.0e-29..1.0e-27),

            mass_top_quark: rng.gen_range(1.0e-28..1.0e-25),
            mass_bottom_quark: rng.gen_range(1.0e-28..1.0e-27),
            mass_tauon: rng.gen_range(1.0e-28..1.0e-26),

            spatial_curvature: rng.gen_range(-2.0..2.0),
            dimensional_ratios: [
                rng.gen_range(0.1..5.0),
                rng.gen_range(0.1..5.0),
                rng.gen_range(0.1..5.0),
                rng.gen_range(0.1..5.0),
            ],
            temporal_evolution_rate: rng.gen_range(0.01..100.0),
            dark_energy_density: rng.gen_range(0.0..5.0e-29),
            dark_matter_coupling: rng.gen_range(0.0..2.0),
        };

        let (fitness, complexity_level) = calculate_enhanced_fitness(&test_universe);

        let curvature_class = if test_universe.spatial_curvature < -0.5 {
            "Hyperbolic"
        } else if test_universe.spatial_curvature > 0.5 {
            "Spherical"
        } else {
            "Flat"
        };

        *geometric_patterns
            .entry(curvature_class.to_string())
            .or_insert(0) += 1;

        if fitness > 0.1 {
            wtr.write_record(&[
                sample_id.to_string(),
                format!("{:.4}", test_universe.spatial_curvature),
                format!("{:.4}", test_universe.dimensional_ratios[0]),
                format!("{:.4}", test_universe.dimensional_ratios[1]),
                format!("{:.4}", test_universe.dimensional_ratios[2]),
                format!("{:.4}", test_universe.dimensional_ratios[3]),
                format!("{:.4}", test_universe.temporal_evolution_rate),
                format!("{:.6}", fitness),
                complexity_level.to_string(),
            ])?;
        }
    }

    wtr.flush()?;

    println!("\n📊 === ANÁLISIS DE PATRONES GEOMÉTRICOS ===");
    for (geometry, count) in geometric_patterns {
        println!("🔷 Geometría {}: {} universos", geometry, count);
    }

    println!("💫 Análisis completo guardado en 'geometric_analysis.csv'");

    Ok(())
}
