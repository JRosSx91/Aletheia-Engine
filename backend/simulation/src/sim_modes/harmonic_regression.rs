//! Regression tests for Harmonic-137 scanners.
//!
//! These lock software behaviour (RNG, CSV schema, formula snapshots, stubs).
//! They do **not** require the heuristic fitness to match laboratory observables.

use super::alpha_fixed::{
    run_alpha_fixed_mode_to_path, sample_alpha_fixed_universe, ALPHA_FIXED_CSV_HEADERS,
};
use super::geometric::{
    run_geometric_mode_to_path, sample_geometric_universe, GEOMETRIC_CSV_HEADERS,
};
use super::harmonic::run_harmonic_mode;
use super::library::{run_library_mode_to_path, LibraryExplorer, LIBRARY_CSV_HEADERS};
use super::map::{
    run_mapping_mode_to_path, run_mapping_mode_to_paths, sample_mapped_universe, MAP_CSV_HEADERS,
};
use super::validate::{create_reference_universe, run_scientific_validation_mode};
use crate::physics::alpha::{si_alpha, AlphaTrace, EmParameters};
use crate::physics::constants::*;
use crate::physics::engine::{
    calculate_enhanced_fitness, hadron_masses_need_pdg, AdvancedPhysicsEngine,
};
use crate::physics::observables::EVALUATION_CSV_HEADERS;
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
fn optional_evaluation_csv_does_not_change_historical_map() {
    let hist_a = temp_csv("map-hist-a");
    let hist_b = temp_csv("map-hist-b");
    let obs = temp_csv("map-obs");
    let n_a = run_mapping_mode_to_path(20, Some(SEED_A), &hist_a).unwrap();
    let n_b = run_mapping_mode_to_paths(20, Some(SEED_A), &hist_b, Some(&obs)).unwrap();
    assert_eq!(n_a, n_b);
    assert_eq!(fs::read(&hist_a).unwrap(), fs::read(&hist_b).unwrap());
    assert_eq!(csv_headers(&obs), EVALUATION_CSV_HEADERS);
    let rows = csv_data_rows(&obs);
    assert_eq!(rows.len(), 21);
    assert_eq!(rows[0][0], "0");
    assert_eq!(rows[20][0], "20");
    let ids: Vec<&str> = rows.iter().map(|r| r[0].as_str()).collect();
    let mut sorted = ids.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "sample_id must be unique");
    assert!(rows.iter().all(|r| r[1] == "137137"));
    let mut rng = HarmonicRng::new(Some(SEED_A));
    let first = sample_mapped_universe(&mut rng);
    let ev = AdvancedPhysicsEngine::new(first.clone()).evaluate();
    let parsed_g: f64 = rows[0][2].parse().unwrap();
    let parsed_e: f64 = rows[0][3].parse().unwrap();
    assert_eq!(parsed_g, first.g);
    assert_eq!(parsed_e, first.e);
    let parsed_fitness: f64 = rows[0][29].parse().unwrap();
    assert_eq!(parsed_fitness, ev.heuristic.fitness);
    assert_eq!(rows[0][27], ev.heuristic.complexity_level.to_string());
    if ev.observables.jeans_mass_kg.is_none() {
        assert!(rows[0][17].is_empty());
    }
    if ev.observables.coulomb_barrier_j.is_none() {
        assert!(rows[0][18].is_empty());
    }
    if ev.observables.chandrasekhar_mass_kg.is_none() {
        assert!(rows[0][19].is_empty());
    }
}

#[test]
fn evaluation_csv_keeps_none_empty_on_nucleon_gate() {
    let mut laws = create_reference_universe();
    laws.mass_electron = 1.0e-27;
    let ev = AdvancedPhysicsEngine::new(laws.clone()).evaluate();
    let row = crate::physics::observables::evaluation_csv_record(7, Some(SEED_A), &laws, &ev);
    assert_eq!(row[0], "7");
    assert_eq!(row[2].parse::<f64>().unwrap(), laws.g);
    assert_eq!(row[3].parse::<f64>().unwrap(), laws.e);
    assert!(
        row[16].is_empty(),
        "hubble must be empty when cosmology did not run"
    );
    assert!(row[17].is_empty());
    assert!(row[18].is_empty());
    assert!(row[19].is_empty());
    assert_eq!(row[21], "");
    assert_eq!(row[22], "");
    assert_eq!(row[23], "");
    assert_eq!(row[29].parse::<f64>().unwrap(), 0.0);
}

#[test]
fn evaluation_csv_keeps_pdg_flag_and_nonfinite_tokens() {
    let mut laws = create_reference_universe();
    laws.alpha_s = 1.0e6;
    let ev = AdvancedPhysicsEngine::new(laws.clone()).evaluate();
    let row = crate::physics::observables::evaluation_csv_record(0, None, &laws, &ev);
    assert!(row[1].is_empty());
    assert_eq!(row[15], "1");
    let ref_laws = create_reference_universe();
    let nan_row = crate::physics::observables::evaluation_csv_record(1, None, &ref_laws, &{
        let mut ev = AdvancedPhysicsEngine::new(ref_laws.clone()).evaluate();
        ev.observables.alpha = f64::NAN;
        ev
    });
    assert_eq!(nan_row[4], "nan");
    assert_ne!(nan_row[4], "");
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
fn harmonic_remains_stub() {
    run_harmonic_mode(10).unwrap();
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

#[test]
fn evaluate_fitness_is_bitwise_identical_to_historical_tuple() {
    let laws = create_reference_universe();
    let engine = AdvancedPhysicsEngine::new(laws.clone());
    let ev = engine.evaluate();
    assert_eq!(
        (ev.heuristic.fitness, ev.heuristic.complexity_level),
        calculate_enhanced_fitness(&laws)
    );
    assert_eq!(
        (ev.heuristic.fitness, ev.heuristic.complexity_level),
        engine.compute_enhanced_fitness()
    );

    let mut rng = HarmonicRng::new(Some(SEED_A));
    for _ in 0..16 {
        let laws = sample_mapped_universe(&mut rng);
        let engine = AdvancedPhysicsEngine::new(laws.clone());
        let ev = engine.evaluate();
        assert_eq!(
            (ev.heuristic.fitness, ev.heuristic.complexity_level),
            calculate_enhanced_fitness(&laws)
        );
    }
}

#[test]
fn reference_evaluation_does_not_invent_unreached_heuristic_stages() {
    let ev = AdvancedPhysicsEngine::new(create_reference_universe()).evaluate();
    assert!(ev.heuristic.nucleon_stability_gate_passed);
    assert_eq!(ev.heuristic.cosmology_term, Some(1.0));
    assert!(ev.heuristic.nucleosynthesis_term.is_some());
    assert!(ev.heuristic.stellar_term.is_some());
    assert_eq!(
        ev.heuristic.stellar_term,
        Some(0.0),
        "stellar was evaluated; a zero score must stay Some(0.0), not None"
    );
    assert!(ev.heuristic.heavy_element_term.is_none());
    assert!(ev.heuristic.black_hole_term.is_none());
    assert!(!ev.heuristic.level_bonus_applied);
    assert_eq!(ev.heuristic.complexity_level, 1);
    assert_eq!(ev.heuristic.fitness, ev.heuristic.fitness_uncapped.min(1.0));
    assert!(!ev.observables.used_pdg_hadron_fallback);
    assert_eq!(
        ev.observables.validated_proton_mass_kg,
        ev.observables.theoretical_proton_mass_kg
    );
    let engine = AdvancedPhysicsEngine::new(create_reference_universe());
    let (tp, tn, tpi) = engine.get_theoretical_hadron_masses();
    assert_eq!(ev.observables.theoretical_pion_mass_kg, tpi);
    assert_eq!(ev.observables.validated_pion_mass_kg, tpi);
    assert_eq!(ev.observables.theoretical_proton_mass_kg, tp);
    assert_eq!(ev.observables.theoretical_neutron_mass_kg, tn);
    assert!(ev.observables.hubble_time_s.is_some());
    assert_eq!(
        ev.observables.jeans_mass_kg,
        Some(engine.calculate_jeans_mass())
    );
    assert!(
        ev.observables.coulomb_barrier_j.is_none(),
        "heavy stage not entered: Coulomb must stay absent, not 0"
    );
    assert!(
        ev.observables.chandrasekhar_mass_kg.is_none(),
        "BH stage not entered: Chandrasekhar must stay absent even though the public method exists"
    );
    assert!(engine.chandrasekhar_mass() > 0.0);
}

#[test]
fn nucleon_gate_failure_leaves_later_heuristic_terms_none() {
    let mut laws = create_reference_universe();
    laws.mass_electron = 1.0e-27;
    let ev = AdvancedPhysicsEngine::new(laws).evaluate();
    assert!(!ev.heuristic.nucleon_stability_gate_passed);
    assert!(ev.heuristic.cosmology_term.is_none());
    assert!(ev.heuristic.nucleosynthesis_term.is_none());
    assert!(ev.heuristic.stellar_term.is_none());
    assert!(ev.heuristic.heavy_element_term.is_none());
    assert!(ev.heuristic.black_hole_term.is_none());
    assert_eq!(ev.heuristic.fitness, 0.0);
    assert_eq!(ev.heuristic.complexity_level, 0);
    assert!(!ev.heuristic.level_bonus_applied);
    assert!(ev.observables.hubble_time_s.is_none());
    assert!(ev.observables.jeans_mass_kg.is_none());
    assert!(ev.observables.coulomb_barrier_j.is_none());
    assert!(ev.observables.chandrasekhar_mass_kg.is_none());
}

#[test]
fn cosmological_cut_records_evaluated_zero_and_skips_later_stages() {
    let mut laws = create_reference_universe();
    laws.g = 1.0e20;
    let ev = AdvancedPhysicsEngine::new(laws.clone()).evaluate();
    assert!(ev.heuristic.nucleon_stability_gate_passed);
    assert_eq!(ev.heuristic.cosmology_term, Some(0.0));
    assert!(ev.heuristic.nucleosynthesis_term.is_none());
    assert!(ev.heuristic.stellar_term.is_none());
    assert!(ev.heuristic.heavy_element_term.is_none());
    assert!(ev.heuristic.black_hole_term.is_none());
    assert_eq!(ev.heuristic.fitness, 0.0);
    assert_eq!(ev.heuristic.complexity_level, 0);
    assert_eq!(
        (ev.heuristic.fitness, ev.heuristic.complexity_level),
        calculate_enhanced_fitness(&laws)
    );
    assert!(
        ev.observables.hubble_time_s.is_none(),
        "freefall cut returns before Hubble; do not invent a time"
    );
    assert!(ev.observables.jeans_mass_kg.is_none());
    assert!(ev.observables.coulomb_barrier_j.is_none());
    assert!(ev.observables.chandrasekhar_mass_kg.is_none());
}

#[test]
fn pdg_fallback_flag_follows_validation_branch_not_mass_equality() {
    let engine = AdvancedPhysicsEngine::new(create_reference_universe());
    let ev = engine.evaluate();
    let (tp, tn, _) = engine.get_theoretical_hadron_masses();
    let (ep, en, _) = engine.get_empirical_hadron_masses();
    let (vp, vn, _) = engine.get_validated_hadron_masses();
    let validity_cut = tp <= 0.0 || tn <= tp || tp > 2e-26 || tn > 2e-26;
    let expect_pdg = cfg!(feature = "empirical_validation") || validity_cut;
    assert_eq!(ev.observables.used_pdg_hadron_fallback, expect_pdg);
    assert_eq!(ev.observables.theoretical_proton_mass_kg, tp);
    assert_eq!(ev.observables.theoretical_neutron_mass_kg, tn);
    assert_eq!(ev.observables.validated_proton_mass_kg, vp);
    assert_eq!(ev.observables.validated_neutron_mass_kg, vn);
    if expect_pdg {
        assert_eq!(vp, ep);
        assert_eq!(vn, en);
    } else {
        assert_eq!(vp, tp);
        assert_eq!(vn, tn);
    }
}

#[test]
fn invalid_theoretical_hadrons_set_pdg_flag_and_keep_theory_fields() {
    if cfg!(feature = "empirical_validation") {
        return;
    }
    let mut laws = create_reference_universe();
    laws.alpha_s = 1.0e6;
    let engine = AdvancedPhysicsEngine::new(laws);
    let (tp, tn, tpi) = engine.get_theoretical_hadron_masses();
    assert!(
        tp <= 0.0 || tn <= tp || tp > 2e-26 || tn > 2e-26,
        "alpha_s=1e6 must trip the existing validity cut, got m_p={tp} m_n={tn}"
    );
    let ev = engine.evaluate();
    let (ep, en, epi) = engine.get_empirical_hadron_masses();
    assert!(ev.observables.used_pdg_hadron_fallback);
    assert_eq!(ev.observables.theoretical_proton_mass_kg, tp);
    assert_eq!(ev.observables.theoretical_neutron_mass_kg, tn);
    assert_eq!(ev.observables.validated_proton_mass_kg, ep);
    assert_eq!(ev.observables.validated_neutron_mass_kg, en);
    assert_eq!(ev.observables.theoretical_pion_mass_kg, tpi);
    assert_eq!(ev.observables.validated_pion_mass_kg, epi);
    assert_ne!(tpi, epi);
}

#[test]
fn evaluate_preserves_iso_alpha_abc_split() {
    let iso = iso_alpha_engines();
    let ea = iso.a.evaluate();
    let eb = iso.b.evaluate();
    let ec = iso.c.evaluate();
    assert_eq!(ea.observables.alpha, ec.observables.alpha);
    close(ea.observables.alpha, eb.observables.alpha, 2.0e-15, 1.0e-18);
    assert_eq!(
        ea.observables.theoretical_proton_mass_kg,
        eb.observables.theoretical_proton_mass_kg
    );
    assert_ne!(
        ea.observables.theoretical_proton_mass_kg,
        ec.observables.theoretical_proton_mass_kg
    );
    assert_eq!(
        ea.observables.alpha_s_reference,
        eb.observables.alpha_s_reference
    );
    assert_ne!(
        ea.observables.alpha_s_reference,
        ec.observables.alpha_s_reference
    );
    assert!(!ea.observables.used_pdg_hadron_fallback);
    assert!(!eb.observables.used_pdg_hadron_fallback);
    assert!(!ec.observables.used_pdg_hadron_fallback);
    assert_eq!(
        (ea.heuristic.fitness, ea.heuristic.complexity_level),
        iso.a.compute_enhanced_fitness()
    );
    assert_eq!(
        (ec.heuristic.fitness, ec.heuristic.complexity_level),
        iso.c.compute_enhanced_fitness()
    );
    assert_eq!(
        ea.observables.theoretical_pion_mass_kg,
        eb.observables.theoretical_pion_mass_kg
    );
    assert_ne!(
        ea.observables.theoretical_pion_mass_kg,
        ec.observables.theoretical_pion_mass_kg
    );
    assert_eq!(ea.observables.hubble_time_s, eb.observables.hubble_time_s);
    assert_eq!(ea.observables.hubble_time_s, ec.observables.hubble_time_s);
    assert_eq!(ea.observables.jeans_mass_kg, eb.observables.jeans_mass_kg);
    assert_ne!(ea.observables.jeans_mass_kg, ec.observables.jeans_mass_kg);
    assert!(ea.observables.coulomb_barrier_j.is_none());
    assert!(eb.observables.coulomb_barrier_j.is_none());
    assert!(ec.observables.coulomb_barrier_j.is_none());
    assert!(ea.observables.chandrasekhar_mass_kg.is_none());
    assert!(eb.observables.chandrasekhar_mass_kg.is_none());
    assert!(ec.observables.chandrasekhar_mass_kg.is_none());
    assert_eq!(ea.heuristic.stellar_term, Some(0.0));
    assert_eq!(ec.heuristic.stellar_term, Some(0.0));
}

#[test]
fn hubble_cut_records_time_but_skips_later_stages() {
    let mut laws = create_reference_universe();
    laws.dark_energy_density = 1.0;
    let ev = AdvancedPhysicsEngine::new(laws.clone()).evaluate();
    assert_eq!(ev.heuristic.cosmology_term, Some(0.0));
    assert!(ev.observables.hubble_time_s.is_some());
    let expected = (3.0 / (8.0 * PI * laws.g * laws.dark_energy_density)).sqrt();
    assert_eq!(ev.observables.hubble_time_s, Some(expected));
    assert!(ev.heuristic.nucleosynthesis_term.is_none());
    assert!(ev.observables.jeans_mass_kg.is_none());
    assert_eq!(
        (ev.heuristic.fitness, ev.heuristic.complexity_level),
        calculate_enhanced_fitness(&laws)
    );
}

#[test]
fn resolve_and_validated_hadrons_stay_equivalent() {
    let cases = [create_reference_universe(), {
        let mut laws = create_reference_universe();
        laws.alpha_s = 1.0e6;
        laws
    }];
    for laws in cases {
        let engine = AdvancedPhysicsEngine::new(laws);
        let ev = engine.evaluate();
        let validated = engine.get_validated_hadron_masses();
        assert_eq!(
            (
                ev.observables.validated_proton_mass_kg,
                ev.observables.validated_neutron_mass_kg,
                ev.observables.validated_pion_mass_kg
            ),
            validated
        );
        let theoretical = engine.get_theoretical_hadron_masses();
        assert_eq!(
            (
                ev.observables.theoretical_proton_mass_kg,
                ev.observables.theoretical_neutron_mass_kg,
                ev.observables.theoretical_pion_mass_kg
            ),
            theoretical
        );
        let (m_p, m_n, _) = theoretical;
        let expect_pdg = cfg!(feature = "empirical_validation") || hadron_masses_need_pdg(m_p, m_n);
        assert_eq!(ev.observables.used_pdg_hadron_fallback, expect_pdg);
    }
}

#[test]
fn heavy_stage_captures_coulomb_without_inventing_chandrasekhar() {
    let mut rng = HarmonicRng::new(Some(SEED_A));
    let mut found = false;
    for _ in 0..256 {
        let laws = sample_mapped_universe(&mut rng);
        let engine = AdvancedPhysicsEngine::new(laws.clone());
        let ev = engine.evaluate();
        if ev.heuristic.heavy_element_term.is_none() {
            continue;
        }
        found = true;
        assert_eq!(
            ev.observables.coulomb_barrier_j,
            Some(engine.coulomb_barrier_energy())
        );
        if ev.heuristic.black_hole_term.is_none() {
            assert!(ev.observables.chandrasekhar_mass_kg.is_none());
        } else {
            assert_eq!(
                ev.observables.chandrasekhar_mass_kg,
                Some(engine.chandrasekhar_mass())
            );
        }
        assert_eq!(
            (ev.heuristic.fitness, ev.heuristic.complexity_level),
            calculate_enhanced_fitness(&laws)
        );
        break;
    }
    assert!(
        found,
        "expected a seed-137137 map sample that enters the heavy-element stage"
    );
}

#[test]
fn hadron_pdg_predicate_covers_each_historical_condition() {
    let cap = 2e-26;
    let m_p = 1.0e-27;
    let m_n = 1.1e-27;
    assert!(
        !hadron_masses_need_pdg(m_p, m_n),
        "ordinary theoretical pair"
    );

    assert!(hadron_masses_need_pdg(0.0, m_n), "m_p <= 0 at zero");
    assert!(
        hadron_masses_need_pdg(-1.0e-31, m_n),
        "m_p <= 0 when negative"
    );
    assert!(hadron_masses_need_pdg(m_p, m_p), "m_n <= m_p when equal");
    assert!(
        hadron_masses_need_pdg(m_p, m_p - 1.0e-40),
        "m_n <= m_p when lighter"
    );
    assert!(
        hadron_masses_need_pdg(cap + 1.0e-40, cap + 2.0e-40),
        "m_p > 2e-26"
    );
    assert!(hadron_masses_need_pdg(m_p, cap + 1.0e-40), "m_n > 2e-26");
}

#[test]
fn hadron_pdg_predicate_boundaries_use_historical_operators() {
    let cap = 2e-26;
    assert!(hadron_masses_need_pdg(0.0, 1.0e-27), "m_p <= 0 includes 0");
    assert!(!hadron_masses_need_pdg(
        f64::MIN_POSITIVE,
        2.0 * f64::MIN_POSITIVE
    ));
    assert!(
        hadron_masses_need_pdg(1.0e-27, 1.0e-27),
        "m_n <= m_p includes equality"
    );
    assert!(!hadron_masses_need_pdg(1.0e-27, 1.0e-27 * 1.000001));
    assert!(
        !hadron_masses_need_pdg(cap * 0.5, cap),
        "cap uses '>' so m_n == 2e-26 is still accepted"
    );
    assert!(
        hadron_masses_need_pdg(cap, cap),
        "equal masses at the cap still fail m_n <= m_p"
    );
    assert!(hadron_masses_need_pdg(cap * 1.1, cap * 1.2));
    assert!(hadron_masses_need_pdg(1.0e-28, cap * 1.1));
}

#[test]
fn hadron_pdg_predicate_documents_ieee_nonfinite_behaviour() {
    // Historical `||` of IEEE comparisons: NaN compared with `<`, `<=`, `>` is false.
    assert!(
        !hadron_masses_need_pdg(f64::NAN, 1.0e-27),
        "NaN m_p does not take the PDG arm; documented, not a new policy"
    );
    assert!(
        !hadron_masses_need_pdg(1.0e-27, f64::NAN),
        "NaN m_n does not take the PDG arm; documented, not a new policy"
    );
    assert!(hadron_masses_need_pdg(f64::INFINITY, f64::INFINITY + 1.0));
    assert!(hadron_masses_need_pdg(f64::NEG_INFINITY, 1.0e-27));
}

#[cfg(feature = "empirical_validation")]
#[test]
fn empirical_validation_forces_pdg_even_on_reference() {
    let engine = AdvancedPhysicsEngine::new(create_reference_universe());
    let ev = engine.evaluate();
    let theoretical = engine.get_theoretical_hadron_masses();
    let empirical = engine.get_empirical_hadron_masses();
    assert!(ev.observables.used_pdg_hadron_fallback);
    assert_eq!(
        (
            ev.observables.theoretical_proton_mass_kg,
            ev.observables.theoretical_neutron_mass_kg,
            ev.observables.theoretical_pion_mass_kg
        ),
        theoretical
    );
    assert_eq!(
        (
            ev.observables.validated_proton_mass_kg,
            ev.observables.validated_neutron_mass_kg,
            ev.observables.validated_pion_mass_kg
        ),
        empirical
    );
    assert_eq!(engine.get_validated_hadron_masses(), empirical);
    assert_ne!(theoretical, empirical);
}

/// 1-loop denominator used by `running_alpha_s` at μ=1 GeV (n_f=3, μ_ref=91.1876 GeV).
fn hadronic_running_denominator(alpha_s_ref: f64) -> f64 {
    let nf = 3.0;
    let beta_0 = (11.0 * 3.0 - 2.0 * nf) / 3.0;
    let log_ratio = (1.0_f64 / 91.1876).ln();
    1.0 + (beta_0 * alpha_s_ref / (2.0 * PI)) * log_ratio
}

const ALPHA_S_SCAN_HEADERS: [&str; 22] = [
    "alpha_s_reference",
    "running_denominator",
    "used_running_formula",
    "running_alpha_s_1gev",
    "theoretical_proton_mass_kg",
    "theoretical_neutron_mass_kg",
    "theoretical_np_mass_diff_kg",
    "used_pdg_hadron_fallback",
    "theoretical_pion_mass_kg",
    "validated_proton_mass_kg",
    "nucleon_stability_gate_passed",
    "cosmology_term",
    "nucleosynthesis_term",
    "stellar_term",
    "heavy_element_term",
    "black_hole_term",
    "level_bonus_applied",
    "complexity_level",
    "fitness_uncapped",
    "fitness",
    "jeans_mass_kg",
    "fitness_was_capped",
];

fn scan_opt_f64(x: Option<f64>) -> String {
    match x {
        Some(v) => format!("{:.16e}", v),
        None => String::new(),
    }
}

fn reference_alpha_s_scan_values() -> Vec<f64> {
    let mut xs = vec![create_reference_universe().alpha_s];
    for i in 0..=400 {
        xs.push(0.05 + (0.25 - 0.05) * (i as f64) / 400.0);
    }
    for i in 0..=200 {
        xs.push(0.12 + (0.16 - 0.12) * (i as f64) / 200.0);
    }
    for i in 0..=300 {
        xs.push(0.1388 + (0.1398 - 0.1388) * (i as f64) / 300.0);
    }
    for i in 1..=80 {
        xs.push(0.25 + (2.0 - 0.25) * (i as f64) / 80.0);
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    xs.dedup_by(|a, b| (*a - *b).abs() < 1e-15);
    xs
}

fn write_reference_alpha_s_scan(path: &Path) {
    let mut body = String::from(ALPHA_S_SCAN_HEADERS.join(","));
    body.push('\n');
    for alpha_s in reference_alpha_s_scan_values() {
        let mut laws = create_reference_universe();
        laws.alpha_s = alpha_s;
        let engine = AdvancedPhysicsEngine::new(laws);
        let denom = hadronic_running_denominator(alpha_s);
        let used_running = denom > 0.1;
        let run = engine.running_alpha_s(1.0);
        let ev = engine.evaluate();
        let o = &ev.observables;
        let h = &ev.heuristic;
        body.push_str(&format!(
            "{:.16e},{:.16e},{},{:.16e},{:.16e},{:.16e},{:.16e},{},{:.16e},{:.16e},{},{},{},{},{},{},{},{},{:.16e},{:.16e},{},{}\n",
            alpha_s,
            denom,
            if used_running { 1 } else { 0 },
            run,
            o.theoretical_proton_mass_kg,
            o.theoretical_neutron_mass_kg,
            o.theoretical_np_mass_diff_kg,
            if o.used_pdg_hadron_fallback { 1 } else { 0 },
            o.theoretical_pion_mass_kg,
            o.validated_proton_mass_kg,
            if h.nucleon_stability_gate_passed { 1 } else { 0 },
            scan_opt_f64(h.cosmology_term),
            scan_opt_f64(h.nucleosynthesis_term),
            scan_opt_f64(h.stellar_term),
            scan_opt_f64(h.heavy_element_term),
            scan_opt_f64(h.black_hole_term),
            if h.level_bonus_applied { 1 } else { 0 },
            h.complexity_level,
            h.fitness_uncapped,
            h.fitness,
            scan_opt_f64(o.jeans_mass_kg),
            if (h.fitness_uncapped - h.fitness).abs() > 0.0 { 1 } else { 0 },
        ));
    }
    fs::write(path, body).expect("write alpha_s scan");
}

#[test]
fn alpha_s_running_cutoff_matches_engine_and_is_discontinuous() {
    let below = 0.1390;
    let above = 0.1395;
    let d_below = hadronic_running_denominator(below);
    let d_above = hadronic_running_denominator(above);
    assert!(d_below > 0.1);
    assert!(d_above <= 0.1);

    let mut laws = create_reference_universe();
    laws.alpha_s = below;
    let eng_b = AdvancedPhysicsEngine::new(laws.clone());
    close(eng_b.running_alpha_s(1.0), below / d_below, 1e-12, 1e-18);
    let (mp_b, mn_b, _) = eng_b.get_theoretical_hadron_masses();

    laws.alpha_s = above;
    let eng_a = AdvancedPhysicsEngine::new(laws);
    assert_eq!(eng_a.running_alpha_s(1.0), above);
    let (mp_a, mn_a, _) = eng_a.get_theoretical_hadron_masses();

    assert!(
        (mn_b - mp_b) - (mn_a - mp_a) > 1e-32,
        "Δm must jump down when the running cut returns α_s_reference"
    );
}

#[test]
fn alpha_s_scan_is_deterministic_and_matches_evaluate() {
    let p1 = temp_csv("as-scan-a");
    let p2 = temp_csv("as-scan-b");
    write_reference_alpha_s_scan(&p1);
    write_reference_alpha_s_scan(&p2);
    assert_eq!(fs::read(&p1).unwrap(), fs::read(&p2).unwrap());
    assert_eq!(csv_headers(&p1), ALPHA_S_SCAN_HEADERS);
    let laws = create_reference_universe();
    let ev = AdvancedPhysicsEngine::new(laws.clone()).evaluate();
    let mut found = false;
    for row in csv_data_rows(&p1) {
        let as_ref: f64 = row[0].parse().unwrap();
        if (as_ref - laws.alpha_s).abs() < 1e-18 {
            let dm: f64 = row[6].parse().unwrap();
            close(dm, ev.observables.theoretical_np_mass_diff_kg, 1e-12, 1e-40);
            found = true;
            break;
        }
    }
    assert!(found, "scan must include a point near the reference α_s");
    if let Ok(out) = std::env::var("HAR_EXP01_SCAN") {
        fs::copy(&p1, &out).expect("copy scan to HAR_EXP01_SCAN");
    }
    if let Ok(out) = std::env::var("HAR_EXP02_SCAN") {
        fs::copy(&p1, &out).expect("copy scan to HAR_EXP02_SCAN");
    }
}

#[test]
fn alpha_s_cutoff_propagates_to_masses_not_cosmology() {
    let mut laws = create_reference_universe();
    laws.alpha_s = 0.1390;
    let below = AdvancedPhysicsEngine::new(laws.clone()).evaluate();
    laws.alpha_s = 0.1395;
    let above = AdvancedPhysicsEngine::new(laws).evaluate();
    assert!(hadronic_running_denominator(0.1390) > 0.1);
    assert!(hadronic_running_denominator(0.1395) <= 0.1);
    assert!(!below.observables.used_pdg_hadron_fallback);
    assert!(!above.observables.used_pdg_hadron_fallback);
    let d_mp = (below.observables.theoretical_proton_mass_kg
        - above.observables.theoretical_proton_mass_kg)
        .abs();
    let d_dm = (below.observables.theoretical_np_mass_diff_kg
        - above.observables.theoretical_np_mass_diff_kg)
        .abs();
    assert!(d_mp / below.observables.theoretical_proton_mass_kg > 1e-3);
    assert!(d_dm / below.observables.theoretical_np_mass_diff_kg > 1e-3);
    assert_eq!(
        below.heuristic.cosmology_term,
        above.heuristic.cosmology_term
    );
    assert!(below.heuristic.nucleon_stability_gate_passed);
    assert!(above.heuristic.nucleon_stability_gate_passed);
    assert_eq!(below.heuristic.fitness, below.heuristic.fitness_uncapped);
    assert_eq!(above.heuristic.fitness, above.heuristic.fitness_uncapped);
}

#[test]
fn seed_137137_sample_230_stays_on_running_side() {
    let hist = temp_csv("exp01-hist");
    let obs = temp_csv("exp01-obs");
    run_mapping_mode_to_paths(400, Some(SEED_A), &hist, Some(&obs)).unwrap();
    let rows = csv_data_rows(&obs);
    assert_eq!(rows.len(), 401);
    let row = rows.iter().find(|r| r[0] == "230").expect("sample_id 230");
    let alpha_s: f64 = row[6].parse().unwrap();
    assert!(
        hadronic_running_denominator(alpha_s) > 0.1,
        "sample 230 must remain on the denom>0.1 side, got α_s={alpha_s}"
    );
    let exported_dm: f64 = row[10].parse().unwrap();
    let mut rng = HarmonicRng::new(Some(SEED_A));
    let mut laws = None;
    for _ in 0..=230 {
        laws = Some(sample_mapped_universe(&mut rng));
    }
    let ev = AdvancedPhysicsEngine::new(laws.unwrap()).evaluate();
    assert_eq!(exported_dm, ev.observables.theoretical_np_mass_diff_kg);
    let row_297 = rows.iter().find(|r| r[0] == "297").expect("sample_id 297");
    let alpha_s_297: f64 = row_297[6].parse().unwrap();
    assert!(
        hadronic_running_denominator(alpha_s_297) <= 0.1,
        "sample 297 must remain on the denom≤0.1 side, got α_s={alpha_s_297}"
    );
    assert_eq!(
        calculate_enhanced_fitness(&create_reference_universe()),
        AdvancedPhysicsEngine::new(create_reference_universe()).compute_enhanced_fitness()
    );
}
