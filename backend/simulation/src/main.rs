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
    /// NUEVO: Explorar la biblioteca cósmica sistemáticamente
    Library {
        #[arg(short, long, default_value_t = 1)]
        min_alpha: u32,
        #[arg(short, long, default_value_t = 1000)]
        max_alpha: u32,
        #[arg(short, long, default_value_t = 100)]
        samples_per_book: u32,
    },
    /// NUEVO: Modo de análisis con alpha fijo (Primacía de Alpha)
    AlphaFixed {
        #[arg(short, long, default_value_t = 137)]
        alpha_denominator: u32,
        #[arg(short, long, default_value_t = 10000)]
        samples: u32,
    },
    /// NUEVO: Análisis geométrico de universos
    Geometric {
        #[arg(short, long, default_value_t = 5000)]
        samples: u32,
    },
    Validate,
    Stream,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Map { universes } => {
        sim_modes::map::run_mapping_mode(*universes)?
    },
        Commands::Evolve { seed, generations } => { sim_modes::evolve::run_evolutionary_mode(seed, *generations)? },
        Commands::Harmonic { samples } => {sim_modes::harmonic::run_harmonic_mode(*samples)?},
        Commands::Library {
            min_alpha,
            max_alpha,
            samples_per_book,
        } => {sim_modes::library::run_library_mode(*min_alpha, *max_alpha, *samples_per_book)?},
        Commands::AlphaFixed {
            alpha_denominator,
            samples,
        } => {sim_modes::alpha_fixed::run_alpha_fixed_mode(*alpha_denominator, *samples)?},
        Commands::Geometric { samples } => {sim_modes::geometric::run_geometric_mode(*samples)?},
        Commands::Validate => sim_modes::validate::run_scientific_validation_mode()?,
        Commands::Stream => sim_modes::stream::run_streaming_mode()?,
    }

    Ok(())
}
