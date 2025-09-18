use crate::core::models::CosmicLaw;
use crate::physics::engine::calculate_enhanced_fitness;
use crate::physics::constants::*;
use std::error::Error;
use rand::prelude::*;
use csv;
use std::f64::consts::PI;

pub fn run_alpha_fixed_mode(alpha_denominator: u32, samples: u32) -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let mut wtr = csv::Writer::from_path("alpha_fixed_analysis.csv")?;

    wtr.write_record(&[
        "sample_id",
        "derived_e",
        "fitness",
        "complexity_level",
        "c_variant",
        "hbar_variant",
        "epsilon0_variant",
    ])?;

    println!("🔬 PREMISA 1: Análisis de la Primacía de Alpha");
    println!(
        "🎯 Fijando α = 1/{}, derivando e desde otras constantes",
        alpha_denominator
    );

    let target_alpha = 1.0 / (alpha_denominator as f64);
    let mut viable_universes = 0;
    let mut best_fitness = 0.0;
    let mut best_universe = None;

    for sample_id in 0..samples {
        let c_variant = C * rng.gen_range(0.1..10.0);
        let hbar_variant = H_BAR * rng.gen_range(0.1..10.0);
        let epsilon0_variant = EPSILON_0 * rng.gen_range(0.1..10.0);
        let derived_e =
            (target_alpha * 4.0 * PI * epsilon0_variant * hbar_variant * c_variant).sqrt();

        let test_universe = CosmicLaw {
            g: rng.gen_range(6.674e-11..6.674e-10),
            e: derived_e,
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

            spatial_curvature: rng.gen_range(-1.0..1.0),
            dimensional_ratios: [
                rng.gen_range(0.5..2.0),
                rng.gen_range(0.5..2.0),
                rng.gen_range(0.5..2.0),
                rng.gen_range(0.5..2.0),
            ],
            temporal_evolution_rate: rng.gen_range(0.1..10.0),
            dark_energy_density: rng.gen_range(0.0..2.0e-29),
            dark_matter_coupling: rng.gen_range(0.0..1.0),
        };

        let (fitness, complexity_level) = calculate_enhanced_fitness(&test_universe);

        if fitness > 0.1 {
            viable_universes += 1;

            wtr.write_record(&[
                sample_id.to_string(),
                format!("{:e}", derived_e),
                format!("{:.6}", fitness),
                complexity_level.to_string(),
                format!("{:.2e}", c_variant),
                format!("{:.2e}", hbar_variant),
                format!("{:.2e}", epsilon0_variant),
            ])?;

            if fitness > best_fitness {
                best_fitness = fitness;
                best_universe = Some(test_universe);
            }
        }

        if sample_id % (samples / 10) == 0 && sample_id > 0 {
            let progress = (sample_id as f64 / samples as f64) * 100.0;
            println!(
                "📊 Progreso: {:.0}% - {} universos viables encontrados",
                progress, viable_universes
            );
        }
    }

    wtr.flush()?;

    println!("\n🎯 === RESULTADOS DE LA PRIMACÍA DE ALPHA ===");
    println!(
        "🔢 α fijo en: 1/{} = {:.8}",
        alpha_denominator, target_alpha
    );
    println!(
        "✅ Universos viables: {} de {} ({:.2}%)",
        viable_universes,
        samples,
        (viable_universes as f64 / samples as f64) * 100.0
    );
    println!("🏆 Mejor fitness: {:.6}", best_fitness);

    if viable_universes > 0 {
        println!(
            "✨ ¡CONFIRMADO! α = 1/{} puede sustentar universos complejos",
            alpha_denominator
        );
        println!("   con múltiples combinaciones de constantes fundamentales.");

        if let Some(champion) = best_universe {
            println!("\n🥇 Universo campeón:");
            println!("   e derivada: {:.3e} C", champion.e);
            println!("   Fitness: {:.6}", best_fitness);
        }
    } else {
        println!(
            "❌ α = 1/{} no parece compatible con universos complejos",
            alpha_denominator
        );
    }

    Ok(())
}
