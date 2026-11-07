use crate::core::models::CosmicLaw;
use crate::physics::alpha::si_alpha;
use crate::physics::engine::calculate_enhanced_fitness;
use crate::utils::{analyze_universe_type, HarmonicRng};
use csv;
use rand::Rng;
use std::error::Error;
use std::path::Path;

pub const MAP_CSV_FILENAME: &str = "landscape_data_enhanced.csv";
pub const MAP_CSV_HEADERS: [&str; 8] = [
    "fitness",
    "complexity_level",
    "type",
    "alpha",
    "mass_up_quark",
    "mass_down_quark",
    "spatial_curvature",
    "temporal_rate",
];

/// Historical loop is inclusive: `0..=num_universes` draws `num_universes + 1` samples.
pub fn run_mapping_mode(num_universes: u64, rng_seed: Option<u64>) -> Result<(), Box<dyn Error>> {
    run_mapping_mode_to_path(num_universes, rng_seed, Path::new(MAP_CSV_FILENAME)).map(|_| ())
}

pub fn run_mapping_mode_to_path(
    num_universes: u64,
    rng_seed: Option<u64>,
    path: &Path,
) -> Result<u64, Box<dyn Error>> {
    let mut rng = HarmonicRng::new(rng_seed);
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(MAP_CSV_HEADERS)?;

    println!(
        "🗺️ Iniciando mapeo mejorado de {} universos...",
        num_universes
    );

    let mut records_written = 0u64;
    for i in 0..=num_universes {
        let random_laws = sample_mapped_universe(&mut rng);

        let (fitness, level) = calculate_enhanced_fitness(&random_laws);
        let alpha = si_alpha(random_laws.e);

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
            records_written += 1;
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
    Ok(records_written)
}

/// RNG consumption order is the field order below. Do not reorder.
pub(crate) fn sample_mapped_universe<R: Rng>(rng: &mut R) -> CosmicLaw {
    CosmicLaw {
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
    }
}
