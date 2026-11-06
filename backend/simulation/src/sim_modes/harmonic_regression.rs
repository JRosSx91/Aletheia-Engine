//! Regression tests for Harmonic-137 scanners.
//!
//! These lock software behaviour (RNG, CSV schema, formula snapshots, stubs).
//! They do **not** require the heuristic fitness to match laboratory observables.

use super::alpha_fixed::{
    run_alpha_fixed_mode_to_path, sample_alpha_fixed_universe, ALPHA_FIXED_CSV_HEADERS,
};
use super::evolve::run_evolutionary_mode;
use super::geometric::{
    run_geometric_mode_to_path, sample_geometric_universe, GEOMETRIC_CSV_HEADERS,
};
use super::harmonic::run_harmonic_mode;
use super::library::{run_library_mode_to_path, LibraryExplorer, LIBRARY_CSV_HEADERS};
use super::map::{run_mapping_mode_to_path, sample_mapped_universe, MAP_CSV_HEADERS};
use super::validate::{create_reference_universe, run_scientific_validation_mode};
use crate::physics::constants::*;
use crate::physics::engine::{calculate_enhanced_fitness, AdvancedPhysicsEngine};
use crate::utils::HarmonicRng;
use std::f64::consts::PI;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const SEED_A: u64 = 137_137;
const SEED_B: u64 = 42;
static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

fn temp_csv(label: &str) -> PathBuf {
    let n = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("har02-{}-{}-{}", label, std::process::id(), n));
    fs::create_dir_all(&dir).expect("temp dir");
    dir.join(format!("{label}.csv"))
}

fn csv_headers(path: &Path) -> Vec<String> {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn csv_data_rows(path: &Path) -> Vec<Vec<String>> {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').map(|s| s.to_string()).collect())
        .collect()
}

fn assert_finite_csv(path: &Path) {
    let text = fs::read_to_string(path).unwrap();
    for (i, line) in text.lines().enumerate().skip(1) {
        for cell in line.split(',') {
            let cell = cell.trim();
            if cell.is_empty() {
                continue;
            }
            if let Ok(v) = cell.parse::<f64>() {
                assert!(
                    v.is_finite(),
                    "non-finite value {v} in {path:?} line {}",
                    i + 1
                );
            }
        }
    }
}

#[test]
fn same_seed_replays_mapped_universes_and_scores() {
    let mut a = HarmonicRng::new(Some(SEED_A));
    let mut b = HarmonicRng::new(Some(SEED_A));
    for _ in 0..16 {
        let ua = sample_mapped_universe(&mut a);
        let ub = sample_mapped_universe(&mut b);
        assert_eq!(ua, ub);
        assert_eq!(
            calculate_enhanced_fitness(&ua),
            calculate_enhanced_fitness(&ub)
        );
    }
}

#[test]
fn different_seeds_produce_different_mapped_sequences() {
    let mut a = HarmonicRng::new(Some(SEED_A));
    let mut b = HarmonicRng::new(Some(SEED_B));
    let ua = sample_mapped_universe(&mut a);
    let ub = sample_mapped_universe(&mut b);
    assert_ne!(ua, ub);
}

#[test]
fn map_csv_is_deterministic_for_fixed_seed() {
    let p1 = temp_csv("map-a");
    let p2 = temp_csv("map-b");
    let n1 = run_mapping_mode_to_path(50, Some(SEED_A), &p1).unwrap();
    let n2 = run_mapping_mode_to_path(50, Some(SEED_A), &p2).unwrap();
    assert_eq!(n1, n2);
    assert_eq!(fs::read(&p1).unwrap(), fs::read(&p2).unwrap());
    assert_eq!(csv_headers(&p1), MAP_CSV_HEADERS);
    // Inclusive historical loop: 0..=50 → 51 draws. Filter fitness > 0.01.
    assert_eq!(n1, csv_data_rows(&p1).len() as u64);
    assert!(n1 <= 51);
    assert_finite_csv(&p1);
}

#[test]
fn map_alpha_column_is_reciprocal_of_si_alpha() {
    let mut rng = HarmonicRng::new(Some(SEED_A));
    let laws = sample_mapped_universe(&mut rng);
    let engine_alpha = laws.e.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C);
    let recorded = format!("{:.6}", 1.0 / engine_alpha);
    let expected = format!("{:.6}", 4.0 * PI * EPSILON_0 * H_BAR * C / laws.e.powi(2));
    assert_eq!(recorded, expected);
    assert!(engine_alpha.is_finite() && engine_alpha > 0.0);
}

#[test]
fn library_writes_one_row_per_denominator() {
    let path = temp_csv("library");
    let n = run_library_mode_to_path(135, 139, 3, Some(SEED_A), &path).unwrap();
    assert_eq!(n, 5);
    assert_eq!(csv_headers(&path), LIBRARY_CSV_HEADERS);
    assert_eq!(csv_data_rows(&path).len(), 5);
    let rows = csv_data_rows(&path);
    let denoms: Vec<&str> = rows.iter().map(|r| r[0].as_str()).collect();
    assert_eq!(denoms, ["135", "136", "137", "138", "139"]);
    assert_finite_csv(&path);
}

#[test]
fn library_inverted_range_writes_headers_only() {
    let path = temp_csv("library-inv");
    let n = run_library_mode_to_path(140, 135, 8, Some(SEED_A), &path).unwrap();
    assert_eq!(n, 0);
    assert_eq!(csv_headers(&path), LIBRARY_CSV_HEADERS);
    assert!(csv_data_rows(&path).is_empty());
}

#[test]
fn library_derived_e_matches_sampled_constants_not_engine_alpha() {
    let explorer = LibraryExplorer::new(137, 137);
    let mut rng = HarmonicRng::new(Some(SEED_A));
    let target = 1.0 / 137.0;
    let mut max_rel_err = 0.0_f64;
    for _ in 0..8 {
        let laws = explorer.generate_universe_at_alpha(target, &mut rng);
        let engine = AdvancedPhysicsEngine::new(laws.clone());
        let si_alpha = laws.e.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C);
        assert!((engine.alpha - si_alpha).abs() < 1e-18);
        max_rel_err = max_rel_err.max((si_alpha - target).abs() / target);
    }
    assert!(
        max_rel_err > 1e-3,
        "library still inverts α with variant c/ħ/ε₀; engine α must not collapse to 1/137"
    );
}

#[test]
fn alpha_fixed_same_seed_same_csv() {
    let p1 = temp_csv("af-a");
    let p2 = temp_csv("af-b");
    let n1 = run_alpha_fixed_mode_to_path(137, 40, Some(SEED_A), &p1).unwrap();
    let n2 = run_alpha_fixed_mode_to_path(137, 40, Some(SEED_A), &p2).unwrap();
    assert_eq!(n1, n2);
    assert_eq!(fs::read(&p1).unwrap(), fs::read(&p2).unwrap());
    assert_eq!(csv_headers(&p1), ALPHA_FIXED_CSV_HEADERS);
    assert_finite_csv(&p1);
}

#[test]
fn alpha_fixed_derived_e_inverts_variants_not_si_constants() {
    let mut rng = HarmonicRng::new(Some(SEED_A));
    let target = 1.0 / 137.0;
    let (laws, c_v, h_v, e_v, derived_e) = sample_alpha_fixed_universe(&mut rng, target);
    let reconstructed = derived_e.powi(2) / (4.0 * PI * e_v * h_v * c_v);
    assert!((reconstructed - target).abs() / target < 1e-12);
    let engine_alpha = laws.e.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C);
    assert!(
        (engine_alpha - target).abs() > 1e-6,
        "engine α coinciding with the target would hide the historical inversion bug"
    );
}

#[test]
fn alpha_fixed_samples_below_ten_do_not_panic() {
    let path = temp_csv("af-small");
    run_alpha_fixed_mode_to_path(137, 7, Some(SEED_A), &path).unwrap();
    let empty = temp_csv("af-zero");
    let n = run_alpha_fixed_mode_to_path(137, 0, Some(SEED_A), &empty).unwrap();
    assert_eq!(n, 0);
    assert_eq!(csv_headers(&empty), ALPHA_FIXED_CSV_HEADERS);
    assert!(csv_data_rows(&empty).is_empty());
}

#[test]
fn geometric_csv_schema_and_replay() {
    let p1 = temp_csv("geo-a");
    let p2 = temp_csv("geo-b");
    let n1 = run_geometric_mode_to_path(30, Some(SEED_A), &p1).unwrap();
    let n2 = run_geometric_mode_to_path(30, Some(SEED_A), &p2).unwrap();
    assert_eq!(n1, n2);
    assert_eq!(csv_headers(&p1), GEOMETRIC_CSV_HEADERS);
    assert_finite_csv(&p1);
    let mut a = HarmonicRng::new(Some(SEED_A));
    let mut b = HarmonicRng::new(Some(SEED_B));
    assert_ne!(
        sample_geometric_universe(&mut a),
        sample_geometric_universe(&mut b)
    );
}

#[test]
fn csv_write_fails_when_path_is_a_directory() {
    let dir = std::env::temp_dir().join(format!("har02-not-a-file-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let err = run_mapping_mode_to_path(1, Some(SEED_A), &dir).unwrap_err();
    let msg = err.to_string();
    assert!(
        !msg.is_empty(),
        "expected an I/O error from csv Writer::from_path on a directory"
    );
}

#[test]
fn unused_cosmic_law_fields_do_not_change_fitness() {
    let mut laws = create_reference_universe();
    let baseline = calculate_enhanced_fitness(&laws);
    laws.spatial_curvature = 1.5;
    laws.dimensional_ratios = [4.0, 0.2, 3.0, 1.7];
    laws.temporal_evolution_rate = 50.0;
    laws.alpha_w = 1.0e-5;
    laws.dark_matter_coupling = 0.9;
    assert_eq!(calculate_enhanced_fitness(&laws), baseline);
}

#[test]
fn extreme_gravity_fails_cosmological_gate() {
    let mut laws = create_reference_universe();
    laws.g = 1.0e20;
    let (fitness, level) = calculate_enhanced_fitness(&laws);
    assert_eq!(fitness, 0.0);
    assert_eq!(level, 0);
}

#[test]
fn fitness_is_capped_and_finite_for_sampled_laws() {
    let mut rng = HarmonicRng::new(Some(SEED_A));
    for _ in 0..64 {
        let laws = sample_mapped_universe(&mut rng);
        let (fitness, level) = calculate_enhanced_fitness(&laws);
        assert!(fitness.is_finite());
        assert!((0.0..=1.0).contains(&fitness));
        assert!(level <= 5);
    }
}

#[test]
fn theoretical_hadron_masses_match_har01_snapshot_not_pdg() {
    let engine = AdvancedPhysicsEngine::new(create_reference_universe());
    let (mp, mn, _) = engine.get_theoretical_hadron_masses();
    let mp_mev = mp / MEV_TO_KG;
    let dn_mev = (mn - mp) / MEV_TO_KG;
    // HAR-01 validate stdout: 5.005090e2 vs PDG 938.27; Δm 1.312403 vs 1.293325.
    assert!(
        (mp_mev - 500.509).abs() < 0.05,
        "theoretical proton mass drifted: {mp_mev} MeV"
    );
    assert!(
        (dn_mev - 1.312403).abs() < 0.001,
        "theoretical n-p difference drifted: {dn_mev} MeV"
    );
    assert!(
        (mp_mev - 938.272).abs() > 100.0,
        "do not treat the current hadron recipe as a PDG match"
    );
}

#[test]
fn reference_alpha_matches_codata_definition() {
    let engine = AdvancedPhysicsEngine::new(create_reference_universe());
    let expected = ELEMENTARY_CHARGE.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C);
    assert!((engine.alpha - expected).abs() / expected < 1e-12);
}

#[test]
fn validate_mode_still_fails_atomic_level() {
    let err = run_scientific_validation_mode().unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("Física atómica fundamental"),
        "validate must keep documenting the HAR-01 failure, got {msg}"
    );
}

#[test]
fn harmonic_and_evolve_remain_stubs() {
    run_harmonic_mode(10).unwrap();
    run_evolutionary_mode("genome.json", 10).unwrap();
    run_evolutionary_mode("missing-file-does-not-matter.json", 0).unwrap();
}
