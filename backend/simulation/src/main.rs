mod core;
mod physics;
mod sim_modes;
mod utils;

use clap::{Parser, Subcommand};
use std::error::Error;

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
    },
    Evolve {
        #[arg(short, long)]
        seed: String,
        #[arg(short, long, default_value_t = 500)]
        generations: u32,
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
        } => sim_modes::map::run_mapping_mode(*universes, *rng_seed)?,
        Commands::Evolve { seed, generations } => {
            sim_modes::evolve::run_evolutionary_mode(seed, *generations)?
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
            } => {
                assert_eq!(universes, 3);
                assert_eq!(rng_seed, Some(137137));
            }
            _ => panic!("expected map"),
        }

        let unseeded =
            Cli::try_parse_from(["rust-universe", "geometric", "--samples", "2"]).unwrap();
        match unseeded.command {
            Commands::Geometric { rng_seed, .. } => assert_eq!(rng_seed, None),
            _ => panic!("expected geometric"),
        }

        let evolve =
            Cli::try_parse_from(["rust-universe", "evolve", "--seed", "genome.json"]).unwrap();
        match evolve.command {
            Commands::Evolve { seed, generations } => {
                assert_eq!(seed, "genome.json");
                assert_eq!(generations, 500);
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
    }
}
