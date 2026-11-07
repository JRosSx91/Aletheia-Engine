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
use crate::physics::alpha::{si_alpha, AlphaTrace, EmParameters};
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

fn close(a: f64, b: f64, rel: f64, abs: f64) {
    let diff = (a - b).abs();
    let scale = a.abs().max(b.abs());
    assert!(
        diff <= abs || diff <= rel * scale.max(f64::MIN_POSITIVE),
        "{a} vs {b}: abs_diff={diff}, rel_diff={}",
        diff / scale.max(f64::MIN_POSITIVE)
    );
}

/// Iso-Alpha A/B/C. B rescales only MKSA (λ=4). C changes only `alpha_s` (+5 %).
const ISO_LAMBDA_EM: f64 = 4.0;
const ISO_ALPHA_S_FACTOR: f64 = 1.05;

struct IsoAlphaEngines {
    a: AdvancedPhysicsEngine,
    b: AdvancedPhysicsEngine,
    c: AdvancedPhysicsEngine,
    laws_a: crate::core::models::CosmicLaw,
    laws_b: crate::core::models::CosmicLaw,
    laws_c: crate::core::models::CosmicLaw,
}

fn iso_alpha_engines() -> IsoAlphaEngines {
    let laws_a = create_reference_universe();
    let mut laws_b = laws_a.clone();
    laws_b.e = laws_a.e * ISO_LAMBDA_EM.sqrt();
    let mut laws_c = laws_a.clone();
    laws_c.alpha_s = laws_a.alpha_s * ISO_ALPHA_S_FACTOR;
    IsoAlphaEngines {
        a: AdvancedPhysicsEngine::new(laws_a.clone()),
        b: AdvancedPhysicsEngine::new_with_em(
            laws_b.clone(),
            EmParameters::si().with_scaled_epsilon0(ISO_LAMBDA_EM),
        ),
        c: AdvancedPhysicsEngine::new(laws_c.clone()),
        laws_a,
        laws_b,
        laws_c,
    }
}

/// `get_validated_hadron_masses` uses PDG if `empirical_validation` is on, or if
/// theoretical (m_p, m_n) fail `m_p > 0 && m_n > m_p && m_p <= 2e-26 && m_n <= 2e-26`.
/// Equality of the two tuples is not enough: assert the gate predicates and that
/// validated is not the empirical substitution.
fn assert_theoretical_hadron_branch(engine: &AdvancedPhysicsEngine) {
    assert!(
        !cfg!(feature = "empirical_validation"),
        "Iso-Alpha tests must not run with empirical_validation (forced PDG)"
    );
    let theoretical = engine.get_theoretical_hadron_masses();
    let validated = engine.get_validated_hadron_masses();
    let empirical = engine.get_empirical_hadron_masses();
    let (m_p, m_n, _) = theoretical;
    assert!(m_p > 0.0, "theoretical m_p <= 0 would take the PDG branch");
    assert!(
        m_n > m_p,
        "theoretical m_n <= m_p would take the PDG branch"
    );
    assert!(
        m_p <= 2e-26,
        "theoretical m_p > 2e-26 would take the PDG branch"
    );
    assert!(
        m_n <= 2e-26,
        "theoretical m_n > 2e-26 would take the PDG branch"
    );
    assert_eq!(
        validated, theoretical,
        "validated masses must be the theoretical tuple, not a replaced copy"
    );
    assert_ne!(
        validated, empirical,
        "validated masses match PDG; fallback substitution is active"
    );
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
    let engine_alpha = si_alpha(laws.e);
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
        let engine_alpha = si_alpha(laws.e);
        assert_eq!(engine.alpha, engine_alpha);
        max_rel_err = max_rel_err.max((engine_alpha - target).abs() / target);
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
    let engine_alpha = si_alpha(laws.e);
    assert!(
        (engine_alpha - target).abs() > 1e-6,
        "engine α coinciding with the target would hide the historical inversion bug"
    );
    let trace = AlphaTrace::from_variants(target, derived_e, e_v, h_v, c_v);
    assert_eq!(trace.alpha_target, Some(target));
    assert!(trace.alpha_derived.is_some());
    assert_eq!(trace.alpha_engine, engine_alpha);
    assert_ne!(trace.alpha_derived, Some(trace.alpha_engine));
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
    let hadrons = AdvancedPhysicsEngine::new(laws.clone()).get_theoretical_hadron_masses();
    laws.scramble_fitness_ignored_fields();
    assert_eq!(calculate_enhanced_fitness(&laws), baseline);
    let hadrons_after = AdvancedPhysicsEngine::new(laws).get_theoretical_hadron_masses();
    assert_eq!(hadrons, hadrons_after);
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

#[test]
fn si_alpha_matches_historical_expression_and_engine() {
    let laws = create_reference_universe();
    let historical = laws.e.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C);
    let extracted = si_alpha(laws.e);
    let engine = AdvancedPhysicsEngine::new(laws.clone());
    assert_eq!(extracted, historical);
    assert_eq!(engine.alpha, historical);
    assert_eq!(
        crate::physics::alpha::alpha_from_em_parameters(laws.e, EPSILON_0, H_BAR, C),
        historical
    );
}

#[test]
fn alpha_trace_does_not_substitute_target_for_engine() {
    let mut rng = HarmonicRng::new(Some(SEED_A));
    let mapped = sample_mapped_universe(&mut rng);
    let map_trace = AlphaTrace::engine_only(mapped.e);
    assert!(map_trace.alpha_target.is_none());
    assert!(map_trace.alpha_derived.is_none());
    assert_eq!(map_trace.alpha_engine, si_alpha(mapped.e));

    let target = 1.0 / 137.0;
    let library_trace = AlphaTrace::target_without_variants(target, mapped.e);
    assert_eq!(library_trace.alpha_target, Some(target));
    assert!(
        library_trace.alpha_derived.is_none(),
        "discarded construction constants must stay unavailable"
    );
    assert_eq!(library_trace.alpha_engine, si_alpha(mapped.e));
    assert_ne!(library_trace.alpha_target, Some(library_trace.alpha_engine));
}

#[test]
fn mass_electron_gate_trips_only_past_threshold() {
    let mut laws = create_reference_universe();
    let baseline = calculate_enhanced_fitness(&laws);
    laws.mass_electron *= 1.01;
    assert_eq!(
        calculate_enhanced_fitness(&laws),
        baseline,
        "small m_e change below the β-cut must not move the score"
    );
    laws.mass_electron = 1.0e-27;
    let (fitness, level) = calculate_enhanced_fitness(&laws);
    assert_eq!(fitness, 0.0);
    assert_eq!(level, 0);
}

#[test]
fn dark_energy_density_is_a_hubble_threshold() {
    let mut laws = create_reference_universe();
    let baseline = calculate_enhanced_fitness(&laws);
    laws.dark_energy_density *= 2.0;
    assert_eq!(
        calculate_enhanced_fitness(&laws),
        baseline,
        "ρ_Λ remains on the cosmological plateau for a factor-of-two nudge at reference"
    );
    laws.dark_energy_density = 1.0;
    let (fitness, level) = calculate_enhanced_fitness(&laws);
    assert_eq!(fitness, 0.0);
    assert_eq!(level, 0);
}

#[test]
fn alpha_s_moves_fitness_via_hadron_mass_difference() {
    let mut laws = create_reference_universe();
    let baseline = calculate_enhanced_fitness(&laws);
    laws.alpha_s = 0.1181 * 1.05;
    assert_ne!(
        calculate_enhanced_fitness(&laws),
        baseline,
        "α_s enters BBN through theoretical Δm; a 5% shift at reference must be visible"
    );
}

#[test]
fn charge_sensitivity_is_observable_on_sampled_laws_not_every_perturbation() {
    let mut rng = HarmonicRng::new(Some(SEED_A));
    let mut found = false;
    for _ in 0..256 {
        let laws = sample_mapped_universe(&mut rng);
        let baseline = calculate_enhanced_fitness(&laws);
        let mut scaled = laws.clone();
        scaled.e *= 8.0;
        if calculate_enhanced_fitness(&scaled) != baseline {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "expected at least one seed-137137 map sample where scaling e changes fitness"
    );
}

#[test]
fn gravity_sensitivity_is_observable_away_from_the_cosmological_cut() {
    let mut rng = HarmonicRng::new(Some(SEED_A));
    let mut found = false;
    for _ in 0..256 {
        let laws = sample_mapped_universe(&mut rng);
        let baseline = calculate_enhanced_fitness(&laws);
        if baseline.0 == 0.0 {
            continue;
        }
        let mut scaled = laws.clone();
        scaled.g *= 3.0;
        if calculate_enhanced_fitness(&scaled) != baseline {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "expected at least one viable map sample where scaling g changes fitness"
    );
}

#[test]
fn gravity_moves_chandrasekhar_even_if_fitness_stays_saturated() {
    let mut laws = create_reference_universe();
    let engine0 = AdvancedPhysicsEngine::new(laws.clone());
    let (fitness0, _) = calculate_enhanced_fitness(&laws);
    let m_ch0 = engine0.chandrasekhar_mass();
    let jeans0 = engine0.calculate_jeans_mass();

    laws.g *= 1.05;
    let engine1 = AdvancedPhysicsEngine::new(laws.clone());
    let (fitness1, _) = calculate_enhanced_fitness(&laws);
    let m_ch1 = engine1.chandrasekhar_mass();
    let jeans1 = engine1.calculate_jeans_mass();

    assert_ne!(m_ch0, m_ch1);
    assert_ne!(jeans0, jeans1);
    // The score may stay put: several terms saturate at 0 or 1. Dependence
    // of Chandrasekhar/Jeans is mathematical; fitness sensitivity is numerical.
    let _fitness_may_saturate = (fitness0, fitness1);
}

#[test]
fn reference_stays_below_heavy_element_stage() {
    let (_fitness, level) = calculate_enhanced_fitness(&create_reference_universe());
    assert_eq!(
        level, 1,
        "reference universe never reaches heavy_element_creation; e in Coulomb is dormant there"
    );
}

#[test]
fn si_em_override_matches_historical_new_bitwise() {
    let laws = create_reference_universe();
    let historical = AdvancedPhysicsEngine::new(laws.clone());
    let via_em = AdvancedPhysicsEngine::new_with_em(laws.clone(), EmParameters::si());
    assert_eq!(historical.alpha, si_alpha(laws.e));
    assert_eq!(via_em.alpha, historical.alpha);
    assert_eq!(
        via_em.coulomb_barrier_energy(),
        historical.coulomb_barrier_energy()
    );
    assert_eq!(
        via_em.compute_enhanced_fitness(),
        historical.compute_enhanced_fitness()
    );
    assert_eq!(
        calculate_enhanced_fitness(&laws),
        historical.compute_enhanced_fitness()
    );
}

#[test]
fn iso_alpha_ab_lambda_4_is_equivalent_representation() {
    const LAMBDA: f64 = 4.0;
    const REL: f64 = 2.0e-15;
    const ABS_ALPHA: f64 = 1.0e-18;
    const ABS_COULOMB: f64 = 1.0e-28;

    let laws_a = create_reference_universe();
    let mut laws_b = laws_a.clone();
    laws_b.e = laws_a.e * LAMBDA.sqrt();

    let engine_a = AdvancedPhysicsEngine::new(laws_a.clone());
    let engine_b = AdvancedPhysicsEngine::new_with_em(
        laws_b.clone(),
        EmParameters::si().with_scaled_epsilon0(LAMBDA),
    );
    let engine_b_si = AdvancedPhysicsEngine::new(laws_b.clone());

    assert!(
        (engine_b_si.alpha - engine_a.alpha).abs() / engine_a.alpha > 1.0e-6,
        "without EM context, scaling only e must look like a different SI α"
    );

    close(engine_a.alpha, engine_b.alpha, REL, ABS_ALPHA);
    close(
        laws_a.e.powi(2) / EPSILON_0,
        laws_b.e.powi(2) / (EPSILON_0 * LAMBDA),
        REL,
        ABS_COULOMB,
    );
    close(
        engine_a.coulomb_barrier_energy(),
        engine_b.coulomb_barrier_energy(),
        REL,
        ABS_COULOMB,
    );
    close(
        engine_a.heavy_element_creation(),
        engine_b.heavy_element_creation(),
        REL,
        1.0e-18,
    );

    let (mp_a, mn_a, mpi_a) = engine_a.get_theoretical_hadron_masses();
    let (mp_b, mn_b, mpi_b) = engine_b.get_theoretical_hadron_masses();
    assert_eq!((mp_a, mn_a, mpi_a), (mp_b, mn_b, mpi_b));
    assert_eq!(engine_a.laws.alpha_s, engine_b.laws.alpha_s);
    close(
        mp_a / laws_a.mass_electron,
        mp_b / laws_b.mass_electron,
        REL,
        1.0e-12,
    );

    close(
        engine_a.primordial_nucleosynthesis_success(),
        engine_b.primordial_nucleosynthesis_success(),
        REL,
        1.0e-18,
    );
    assert_eq!(
        engine_a.stellar_formation_epoch(),
        engine_b.stellar_formation_epoch()
    );

    let fit_a = engine_a.compute_enhanced_fitness();
    let fit_b = engine_b.compute_enhanced_fitness();
    assert_eq!(fit_a.1, 1);
    assert_eq!(fit_a.1, fit_b.1);
    close(fit_a.0, fit_b.0, REL, 1.0e-18);
}

#[test]
fn iso_alpha_ab_coulomb_is_compared_without_fitness_gates() {
    const LAMBDA: f64 = 4.0;
    let mut rng = HarmonicRng::new(Some(SEED_A));
    let first = sample_mapped_universe(&mut rng);
    let mut laws_b = first.clone();
    laws_b.e = first.e * LAMBDA.sqrt();
    let engine_a = AdvancedPhysicsEngine::new(first);
    let engine_b =
        AdvancedPhysicsEngine::new_with_em(laws_b, EmParameters::si().with_scaled_epsilon0(LAMBDA));
    close(
        engine_a.coulomb_barrier_energy(),
        engine_b.coulomb_barrier_energy(),
        2.0e-15,
        1.0e-28,
    );
    close(
        engine_a.heavy_element_creation(),
        engine_b.heavy_element_creation(),
        2.0e-15,
        1.0e-18,
    );
}

#[test]
fn iso_alpha_c_same_alpha_different_alpha_s() {
    const REL: f64 = 2.0e-15;
    let iso = iso_alpha_engines();

    assert_eq!(iso.laws_a.e, iso.laws_c.e);
    assert_eq!(iso.a.alpha, iso.c.alpha);
    close(iso.a.alpha, si_alpha(iso.laws_a.e), REL, 1.0e-18);
    assert_eq!(
        iso.a.coulomb_barrier_energy(),
        iso.c.coulomb_barrier_energy()
    );

    assert_ne!(iso.laws_a.alpha_s, iso.laws_c.alpha_s);
    close(
        iso.laws_c.alpha_s / iso.laws_a.alpha_s,
        ISO_ALPHA_S_FACTOR,
        REL,
        1.0e-18,
    );
    let mut c_restored = iso.laws_c.clone();
    c_restored.alpha_s = iso.laws_a.alpha_s;
    assert_eq!(
        iso.laws_a, c_restored,
        "case C must differ from A only in alpha_s"
    );

    assert_theoretical_hadron_branch(&iso.a);
    assert_theoretical_hadron_branch(&iso.c);

    let run_a = iso.a.running_alpha_s(1.0);
    let run_c = iso.c.running_alpha_s(1.0);
    let bind_a = iso.a.realistic_qcd_binding("proton");
    let bind_c = iso.c.realistic_qcd_binding("proton");
    let (mp_a, mn_a, _) = iso.a.get_theoretical_hadron_masses();
    let (mp_c, mn_c, _) = iso.c.get_theoretical_hadron_masses();
    let dm_a = mn_a - mp_a;
    let dm_c = mn_c - mp_c;

    assert_ne!(run_a, run_c);
    assert_ne!(bind_a, bind_c);
    assert_ne!((mp_a, mn_a), (mp_c, mn_c));
    assert_ne!(dm_a, dm_c);
    assert_ne!(
        iso.a.primordial_nucleosynthesis_success(),
        iso.c.primordial_nucleosynthesis_success()
    );

    let (fit_a, level_a) = iso.a.compute_enhanced_fitness();
    let (fit_c, level_c) = iso.c.compute_enhanced_fitness();
    assert_eq!(level_a, 1, "reference stays below the Coulomb fitness gate");
    assert_eq!(level_c, 1, "case C must not be forced through later gates");
    let _fitness_is_not_habitability = (fit_a, fit_c);
}

#[test]
fn iso_alpha_abc_invariants_and_which_observables_move() {
    const REL: f64 = 2.0e-15;
    let iso = iso_alpha_engines();

    assert_eq!(iso.a.alpha, iso.c.alpha);
    close(iso.a.alpha, iso.b.alpha, REL, 1.0e-18);
    close(iso.b.alpha, iso.c.alpha, REL, 1.0e-18);
    close(
        iso.a.coulomb_barrier_energy(),
        iso.b.coulomb_barrier_energy(),
        REL,
        1.0e-28,
    );
    assert_eq!(
        iso.a.coulomb_barrier_energy(),
        iso.c.coulomb_barrier_energy()
    );
    assert_eq!(iso.laws_a.alpha_s, iso.laws_b.alpha_s);
    assert_ne!(iso.laws_a.alpha_s, iso.laws_c.alpha_s);

    let th_a = iso.a.get_theoretical_hadron_masses();
    let th_b = iso.b.get_theoretical_hadron_masses();
    let th_c = iso.c.get_theoretical_hadron_masses();
    assert_eq!(
        th_a, th_b,
        "EM-only rescaling must not move theoretical hadrons"
    );
    assert_ne!(th_a, th_c, "alpha_s must move theoretical hadrons");
    assert_theoretical_hadron_branch(&iso.a);
    assert_theoretical_hadron_branch(&iso.b);
    assert_theoretical_hadron_branch(&iso.c);

    assert_eq!(iso.a.running_alpha_s(1.0), iso.b.running_alpha_s(1.0));
    assert_ne!(iso.a.running_alpha_s(1.0), iso.c.running_alpha_s(1.0));
}
