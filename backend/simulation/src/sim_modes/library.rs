use crate::core::models::{CosmicNarrative, CosmicLaw};
use crate::physics::engine::{AdvancedPhysicsEngine, calculate_enhanced_fitness};
use crate::physics::constants::*;
use std::f64::consts::PI;
use std::error::Error;
use std::collections::HashMap;
use rand::prelude::*;
use csv;

pub struct LibraryExplorer {
    // shelf_range: (u32, u32),
    books_scanned: HashMap<u32, CosmicNarrative>,
    genre_classification: HashMap<String, Vec<u32>>,
}

impl LibraryExplorer {
    pub fn new(_min_alpha_denom: u32, _max_alpha_denom: u32) -> Self {
        Self {
            // shelf_range: (min_alpha_denom, max_alpha_denom),
            books_scanned: HashMap::new(),
            genre_classification: HashMap::new(),
        }
    }

    pub fn scan_book(
        &mut self,
        alpha_denominator: u32,
        samples: u32,
        rng: &mut impl Rng,
    ) -> CosmicNarrative {
        let target_alpha = 1.0 / (alpha_denominator as f64);
        let mut best_narrative = CosmicNarrative {
            story_quality: 0.0,
            narrative_depth: 0,
            dramatic_events: Vec::new(),
            finale_type: "Silence".to_string(),
            reproducibility: 0.0,
        };

        for _ in 0..samples {
            let universe = self.generate_universe_at_alpha(target_alpha, rng);
            let narrative = self.analyze_cosmic_narrative(&universe);

            if narrative.story_quality > best_narrative.story_quality {
                best_narrative = narrative;
            }
        }

        self.books_scanned
            .insert(alpha_denominator, best_narrative.clone());
        best_narrative
    }

    pub fn generate_universe_at_alpha(&self, target_alpha: f64, rng: &mut impl Rng) -> CosmicLaw {
        // PREMISA 1: La Primacía de Alpha - derivamos e desde α fijo
        let random_c = C * rng.gen_range(0.5..2.0);
        let random_hbar = H_BAR * rng.gen_range(0.5..2.0);
        let random_epsilon0 = EPSILON_0 * rng.gen_range(0.5..2.0);
        let derived_e = (target_alpha * 4.0 * PI * random_epsilon0 * random_hbar * random_c).sqrt();

        CosmicLaw {
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

            // PREMISA 6: Parámetros geométricos
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
        }
    }

    pub fn analyze_cosmic_narrative(&self, laws: &CosmicLaw) -> CosmicNarrative {
        let engine = AdvancedPhysicsEngine::new(laws.clone());
        let (fitness, complexity_level) = calculate_enhanced_fitness(laws);

        let mut dramatic_events = Vec::new();
        let mut story_quality = fitness;

        // Analizar los momentos dramáticos del universo
        if engine.primordial_nucleosynthesis_success() > 0.5 {
            dramatic_events.push("The Great Nucleosynthesis".to_string());
        }

        if engine.stellar_formation_epoch() > 0.5 {
            dramatic_events.push("The Age of Stars Begins".to_string());
        }

        if engine.heavy_element_creation() > 0.5 {
            dramatic_events.push("The Chemical Revolution".to_string());
        }

        if engine.black_hole_formation_potential() > 0.6 {
            dramatic_events.push("The Portal Makers Awaken".to_string());
        }

        // Calcular calidad de historia basada en variedad de eventos
        story_quality *= 1.0 + dramatic_events.len() as f64 * 0.2;

        let finale_type = match complexity_level {
            0 => "Stillborn Silence".to_string(),
            1 => "Chemical Whispers".to_string(),
            2 => "Nuclear Fire".to_string(),
            3 => "Stellar Symphony".to_string(),
            4 => "Complex Choreography".to_string(),
            5 => "Reproductive Renaissance".to_string(),
            _ => "Unknown Epic".to_string(),
        };

        CosmicNarrative {
            story_quality: story_quality.min(2.0), // Cap at 2.0 for exceptional stories
            narrative_depth: complexity_level,
            dramatic_events,
            finale_type,
            reproducibility: engine.black_hole_formation_potential(),
        }
    }

    pub fn classify_genres(&mut self) {
        let mut genres: HashMap<String, Vec<u32>> = HashMap::new();

        for (&alpha_denom, narrative) in &self.books_scanned {
            let genre = match narrative.narrative_depth {
                0 => "Cosmic Horror",
                1 => "Minimalist Drama",
                2 => "Nuclear Thriller",
                3 => "Space Opera",
                4 => "Epic Fantasy",
                5 => "Reproductive Romance",
                _ => "Experimental Fiction",
            };

            genres
                .entry(genre.to_string())
                .or_insert(Vec::new())
                .push(alpha_denom);
        }

        self.genre_classification = genres;
    }
}

pub fn run_library_mode(
    min_alpha: u32,
    max_alpha: u32,
    samples_per_book: u32,
) -> Result<(), Box<dyn Error>> {
    let mut explorer = LibraryExplorer::new(min_alpha, max_alpha);
    let mut rng = thread_rng();
    let mut wtr = csv::Writer::from_path("cosmic_library.csv")?;

    wtr.write_record(&[
        "alpha_denominator",
        "story_quality",
        "narrative_depth",
        "finale_type",
        "reproducibility",
        "dramatic_events_count",
    ])?;

    println!("📚 Explorando la Biblioteca Cósmica...");
    println!(
        "📖 Escaneando libros desde α = 1/{} hasta α = 1/{}",
        min_alpha, max_alpha
    );

    let mut masterpieces = Vec::new();
    let total_books = max_alpha - min_alpha + 1;

    for alpha_denom in min_alpha..=max_alpha {
        print!("📑 Leyendo libro α = 1/{}... ", alpha_denom);

        let narrative = explorer.scan_book(alpha_denom, samples_per_book, &mut rng);

        wtr.write_record(&[
            alpha_denom.to_string(),
            format!("{:.6}", narrative.story_quality),
            narrative.narrative_depth.to_string(),
            narrative.finale_type.clone(),
            format!("{:.6}", narrative.reproducibility),
            narrative.dramatic_events.len().to_string(),
        ])?;

        if narrative.story_quality > 0.8 {
            masterpieces.push((alpha_denom, narrative.story_quality));
            println!("✨ ¡OBRA MAESTRA! Calidad: {:.4}", narrative.story_quality);
        } else if narrative.story_quality > 0.5 {
            println!(
                "📝 Historia interesante. Calidad: {:.4}",
                narrative.story_quality
            );
        } else {
            println!("📄 Historia simple.");
        }

        if alpha_denom % 100 == 0 {
            let progress = (alpha_denom - min_alpha) as f64 / total_books as f64 * 100.0;
            println!(
                "📊 Progreso: {:.1}% - {} obras maestras encontradas",
                progress,
                masterpieces.len()
            );
        }
    }

    wtr.flush()?;

    // Clasificar géneros
    explorer.classify_genres();

    println!("\n🎭 === ANÁLISIS LITERARIO DE LA BIBLIOTECA CÓSMICA ===");
    for (genre, books) in &explorer.genre_classification {
        println!("📚 Género '{}': {} libros", genre, books.len());
        if books.len() <= 5 {
            println!("   Títulos destacados: α = 1/{:?}", books);
        }
    }

    println!("\n⭐ === OBRAS MAESTRAS CÓSMICAS ===");
    masterpieces.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (i, (alpha_denom, quality)) in masterpieces.iter().take(10).enumerate() {
        println!("{}. α = 1/{} (Calidad: {:.4})", i + 1, alpha_denom, quality);
    }

    if masterpieces
        .iter()
        .any(|(alpha_denom, _)| *alpha_denom == 137)
    {
        println!("🌟 ¡Nuestro universo (α = 1/137) está entre las obras maestras!");
    }

    Ok(())
}
