mod core;
mod physics;
mod sim_modes;
mod utils;

use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Simulador Cosmológico 'El Armónico 137' - The Cosmic Library"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Map {
        #[arg(short, long, default_value_t = 1_000_000)]
        universes: u64,
        /// Optional StdRng seed. Omit for historical thread_rng behaviour.
        #[arg(long)]
        rng_seed: Option<u64>,
        /// Optional EngineEvaluation CSV. Does not change landscape_data_enhanced.csv.
        #[arg(long)]
        observables_csv: Option<PathBuf>,
    },
    /// Experimental H-03 optimizer (internal fitness). Not cosmological selection.
    #[command(after_help = "T = --generations is the number of transitions: T=30 yields generations 0..=30.\nThere is no --seed genome file; use --rng-seed. CSV/JSON via --output-dir; Plotly remains plot_evo01.py.")]
    Evolve {
        /// Individuals per arm. Not `map --universes`.
        #[arg(short = 'p', long, default_value_t = 100)]
        population: u32,
        /// Transition count T (required). Live generations = T+1.
        #[arg(short, long, required = true)]
        generations: u32,
        #[arg(long, default_value_t = 0.05)]
        mutation_sigma: f64,
        #[arg(long, required = true)]
        rng_seed: u64,
        #[arg(long, required = true)]
        output_dir: PathBuf,
        #[arg(long, default_value_t = false)]
        force: bool,
    },
    Harmonic {
        #[arg(short, long, default_value_t = 10000)]
        samples: u32,
    },
    Library {
        #[arg(short, long, default_value_t = 1)]
        min_alpha: u32,
        #[arg(short, long, default_value_t = 1000)]
        max_alpha: u32,
        #[arg(short, long, default_value_t = 100)]
        samples_per_book: u32,
        /// Optional StdRng seed. Omit for historical thread_rng behaviour.
        #[arg(long)]
        rng_seed: Option<u64>,
    },
    AlphaFixed {
        #[arg(short, long, default_value_t = 137)]
        alpha_denominator: u32,
        #[arg(short, long, default_value_t = 10000)]
        samples: u32,
        /// Optional StdRng seed. Omit for historical thread_rng behaviour.
        #[arg(long)]
        rng_seed: Option<u64>,
    },
    Geometric {
        #[arg(short, long, default_value_t = 5000)]
        samples: u32,
        /// Optional StdRng seed. Omit for historical thread_rng behaviour.
        #[arg(long)]
        rng_seed: Option<u64>,
    },
    Validate,
    Stream,
    Lattice,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Map {
            universes,
            rng_seed,
            observables_csv,
        } => sim_modes::map::run_mapping_mode(
            *universes,
            *rng_seed,
            observables_csv.as_deref(),
        )?,
        Commands::Evolve {
            population,
            generations,
            mutation_sigma,
            rng_seed,
            output_dir,
            force,
        } => {
            sim_modes::evolve::run_evolutionary_mode(&sim_modes::evolve::EvolveRequest {
                population: *population,
                generations: *generations,
                mutation_sigma: *mutation_sigma,
                rng_seed: *rng_seed,
                output_dir: output_dir.clone(),
                force: *force,
            })?;
        }
        Commands::Harmonic { samples } => sim_modes::harmonic::run_harmonic_mode(*samples)?,
        Commands::Library {
            min_alpha,
            max_alpha,
            samples_per_book,
            rng_seed,
        } => sim_modes::library::run_library_mode(
            *min_alpha,
            *max_alpha,
            *samples_per_book,
            *rng_seed,
        )?,
        Commands::AlphaFixed {
            alpha_denominator,
            samples,
            rng_seed,
        } => sim_modes::alpha_fixed::run_alpha_fixed_mode(*alpha_denominator, *samples, *rng_seed)?,
        Commands::Geometric { samples, rng_seed } => {
            sim_modes::geometric::run_geometric_mode(*samples, *rng_seed)?
        }
        Commands::Validate => sim_modes::validate::run_scientific_validation_mode()?,
        Commands::Stream => sim_modes::stream::run_streaming_mode()?,
        Commands::Lattice => sim_modes::stream::run_lattice_mode()?,
    }

    Ok(())
}

#[cfg(test)]
mod cli_parse_tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn rng_seed_is_optional_and_distinct_from_evolve_seed() {
        let map = Cli::try_parse_from([
            "rust-universe",
            "map",
            "--universes",
            "3",
            "--rng-seed",
            "137137",
        ])
        .unwrap();
        match map.command {
            Commands::Map {
                universes,
                rng_seed,
                observables_csv,
            } => {
                assert_eq!(universes, 3);
                assert_eq!(rng_seed, Some(137137));
                assert!(observables_csv.is_none());
            }
            _ => panic!("expected map"),
        }

        let unseeded =
            Cli::try_parse_from(["rust-universe", "geometric", "--samples", "2"]).unwrap();
        match unseeded.command {
            Commands::Geometric { rng_seed, .. } => assert_eq!(rng_seed, None),
            _ => panic!("expected geometric"),
        }

        let evolve = Cli::try_parse_from([
            "rust-universe",
            "evolve",
            "--generations",
            "30",
            "--rng-seed",
            "137137",
            "--output-dir",
            "out",
        ])
        .unwrap();
        match evolve.command {
            Commands::Evolve {
                population,
                generations,
                mutation_sigma,
                rng_seed,
                force,
                ..
            } => {
                assert_eq!(population, 100);
                assert_eq!(generations, 30);
                assert_eq!(mutation_sigma.to_bits(), 0.05f64.to_bits());
                assert_eq!(rng_seed, 137137);
                assert!(!force);
            }
            _ => panic!("expected evolve"),
        }
    }

    #[test]
    fn rejects_invalid_numeric_flags() {
        assert!(Cli::try_parse_from(["rust-universe", "map", "--universes", "nope"]).is_err());
        assert!(Cli::try_parse_from(["rust-universe", "alpha-fixed", "--samples", "-4"]).is_err());
        assert!(Cli::try_parse_from(["rust-universe", "map", "--rng-seed", "1.5"]).is_err());
        assert!(Cli::try_parse_from(["rust-universe", "evolve"]).is_err());
        assert!(Cli::try_parse_from([
            "rust-universe",
            "evolve",
            "--rng-seed",
            "1",
            "--output-dir",
            "o"
        ])
        .is_err());
        assert!(Cli::try_parse_from([
            "rust-universe",
            "evolve",
            "--generations",
            "30",
            "--output-dir",
            "o"
        ])
        .is_err());
        assert!(Cli::try_parse_from([
            "rust-universe",
            "evolve",
            "--generations",
            "30",
            "--rng-seed",
            "1"
        ])
        .is_err());
        assert!(Cli::try_parse_from([
            "rust-universe",
            "evolve",
            "--seed",
            "genome.json",
            "--generations",
            "30",
            "--rng-seed",
            "1",
            "--output-dir",
            "o"
        ])
        .is_err());
        assert!(Cli::try_parse_from([
            "rust-universe",
            "evolve",
            "--universes",
            "10",
            "--generations",
            "30",
            "--rng-seed",
            "1",
            "--output-dir",
            "o"
        ])
        .is_err());
    }

    #[test]
    fn observables_csv_flag_is_optional() {
        let parsed = Cli::try_parse_from([
            "rust-universe",
            "map",
            "--universes",
            "2",
            "--rng-seed",
            "137137",
            "--observables-csv",
            "engine_evaluations.csv",
        ])
        .unwrap();
        match parsed.command {
            Commands::Map {
                observables_csv, ..
            } => {
                assert_eq!(
                    observables_csv.as_deref(),
                    Some(std::path::Path::new("engine_evaluations.csv"))
                );
            }
            _ => panic!("expected map"),
        }
    }
}
