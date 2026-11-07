//! Mathematical tests for RunningKind A/B/D. Software continuity, not QCD validation.

use super::map::{run_mapping_mode_to_paths, sample_mapped_universe};
use super::running_compare::{
    running_compare_headers, variant_specs, write_paired_running_csvs, MODEL01B_SEED,
    MODEL01B_UNIVERSES,
};
use super::validate::create_reference_universe;
use crate::physics::engine::{
    smooth_alpha_s_dd, smooth_alpha_s_from_denominator, AdvancedPhysicsEngine, RunningKind,
    RUNNING_DENOM_THRESHOLD, SMOOTH_EPS_DEFAULT, SMOOTH_EPS_SENSITIVITY,
};
use crate::utils::HarmonicRng;
use std::f64::consts::PI;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

fn temp_dir(label: &str) -> PathBuf {
    let n = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "har-model01b-{}-{}-{}",
        label,
        std::process::id(),
        n
    ));
    fs::create_dir_all(&dir).expect("temp dir");
    dir
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

fn hadronic_d_from_alpha(alpha_s_ref: f64) -> f64 {
    let beta_0 = 9.0;
    let log_ratio = (1.0_f64 / 91.1876).ln();
    1.0 + (beta_0 * alpha_s_ref / (2.0 * PI)) * log_ratio
}

fn alpha_from_hadronic_d(d: f64) -> f64 {
    let beta_0 = 9.0;
    let log_ratio = (1.0_f64 / 91.1876).ln();
    let coef = (beta_0 / (2.0 * PI)) * log_ratio;
    (d - 1.0) / coef
}

#[test]
fn new_is_historical_running_kind() {
    let laws = create_reference_universe();
    let a = AdvancedPhysicsEngine::new(laws.clone());
    let tagged = AdvancedPhysicsEngine::new(laws.clone()).with_running_kind(RunningKind::Historical);
    assert_eq!(
        a.running_alpha_s(1.0).to_bits(),
        tagged.running_alpha_s(1.0).to_bits()
    );
    assert_eq!(a.evaluate(), tagged.evaluate());
    let smooth = AdvancedPhysicsEngine::new(laws)
        .with_running_kind(RunningKind::smooth(SMOOTH_EPS_DEFAULT));
    let d = hadronic_d_from_alpha(a.alpha_s_reference);
    if (d - RUNNING_DENOM_THRESHOLD).abs() < SMOOTH_EPS_DEFAULT {
        assert_ne!(a.running_alpha_s(1.0).to_bits(), smooth.running_alpha_s(1.0).to_bits());
    }
}

#[test]
fn historical_matches_inline_cutoff_formula() {
    let mut laws = create_reference_universe();
    for alpha in [0.05, 0.1181, 0.1390, 0.1395, 0.5, 1.9] {
        laws.alpha_s = alpha;
        let engine = AdvancedPhysicsEngine::new(laws.clone());
        let d = engine.running_denominator(1.0);
        let expected = if d > 0.1 { alpha / d } else { alpha };
        assert_eq!(engine.running_alpha_s(1.0).to_bits(), expected.to_bits());
        close(d, hadronic_d_from_alpha(alpha), 1e-14, 1e-18);
    }
}

#[test]
fn constant_returns_alpha_s_ref_for_any_mu() {
    let mut laws = create_reference_universe();
    laws.alpha_s = 0.1181;
    let engine =
        AdvancedPhysicsEngine::new(laws).with_running_kind(RunningKind::Constant);
    for mu in [0.2, 0.3255, 1.0, 2.0, 91.1876] {
        assert_eq!(engine.running_alpha_s(mu).to_bits(), 0.1181_f64.to_bits());
    }
}

#[test]
fn infrared_cut_preserved_for_a_and_b() {
    let mut laws = create_reference_universe();
    laws.alpha_s = 0.12;
    let mu_ir = 0.2;
    assert!(mu_ir <= 0.217 * 1.5);
    let a = AdvancedPhysicsEngine::new(laws.clone());
    let b = AdvancedPhysicsEngine::new(laws.clone())
        .with_running_kind(RunningKind::smooth(SMOOTH_EPS_DEFAULT));
    assert_eq!(a.running_alpha_s(mu_ir).to_bits(), 0.12_f64.to_bits());
    assert_eq!(b.running_alpha_s(mu_ir).to_bits(), 0.12_f64.to_bits());
    assert_ne!(a.running_alpha_s(1.0), a.alpha_s_reference);
}

#[test]
fn smooth_matches_branches_outside_band() {
    let alpha = 0.12;
    for &eps in &SMOOTH_EPS_SENSITIVITY {
        let lo = RUNNING_DENOM_THRESHOLD - eps;
        let hi = RUNNING_DENOM_THRESHOLD + eps;
        close(
            smooth_alpha_s_from_denominator(alpha, lo - 1e-3, eps),
            alpha,
            0.0,
            0.0,
        );
        close(
            smooth_alpha_s_from_denominator(alpha, hi + 1e-3, eps),
            alpha / (hi + 1e-3),
            1e-14,
            1e-18,
        );
        close(
            smooth_alpha_s_from_denominator(alpha, -2.0, eps),
            alpha,
            0.0,
            0.0,
        );
    }
}

#[test]
fn smooth_c0_at_band_edges() {
    let alpha = 0.1181;
    for &eps in &SMOOTH_EPS_SENSITIVITY {
        let lo = RUNNING_DENOM_THRESHOLD - eps;
        let hi = RUNNING_DENOM_THRESHOLD + eps;
        close(
            smooth_alpha_s_from_denominator(alpha, lo, eps),
            alpha,
            1e-15,
            1e-18,
        );
        close(
            smooth_alpha_s_from_denominator(alpha, hi, eps),
            alpha / hi,
            1e-15,
            1e-18,
        );
    }
}

#[test]
fn smooth_c1_matches_analytic_and_finite_differences() {
    let alpha = 0.1181;
    let h = 1e-8;
    for &eps in &SMOOTH_EPS_SENSITIVITY {
        let lo = RUNNING_DENOM_THRESHOLD - eps;
        let hi = RUNNING_DENOM_THRESHOLD + eps;
        close(smooth_alpha_s_dd(alpha, lo, eps), 0.0, 0.0, 0.0);
        close(
            smooth_alpha_s_dd(alpha, hi, eps),
            -alpha / (hi * hi),
            1e-14,
            1e-18,
        );

        // At d=lo the first derivative is 0 but f'' is large (s ~ t²). A one-sided
        // difference is O(h), so it is not a C¹ counterexample; check it shrinks with h.
        let fd_lo = |step: f64| {
            (smooth_alpha_s_from_denominator(alpha, lo + step, eps)
                - smooth_alpha_s_from_denominator(alpha, lo, eps))
                / step
        };
        assert!(
            fd_lo(1e-8).abs() < fd_lo(1e-6).abs(),
            "right difference at lo should shrink with h (C¹ vs noisy FD)"
        );

        let analytic_hi = -alpha / (hi * hi);
        let fd_hi_right = (smooth_alpha_s_from_denominator(alpha, hi + h, eps)
            - smooth_alpha_s_from_denominator(alpha, hi, eps))
            / h;
        close(fd_hi_right, analytic_hi, 1e-4, 1e-5);
        let d_in_hi = hi - 0.02 * (hi - lo);
        let fd_in_hi = (smooth_alpha_s_from_denominator(alpha, d_in_hi + h, eps)
            - smooth_alpha_s_from_denominator(alpha, d_in_hi - h, eps))
            / (2.0 * h);
        close(fd_in_hi, smooth_alpha_s_dd(alpha, d_in_hi, eps), 1e-4, 1e-5);

        for k in 1..9 {
            let d = lo + (hi - lo) * (k as f64) / 9.0;
            let analytic = smooth_alpha_s_dd(alpha, d, eps);
            let fd = (smooth_alpha_s_from_denominator(alpha, d + h, eps)
                - smooth_alpha_s_from_denominator(alpha, d - h, eps))
                / (2.0 * h);
            close(fd, analytic, 1e-4, 1e-5);
        }
    }
}

#[test]
fn smooth_positive_and_finite_on_domain() {
    let ds = [-20.0, -1.0, 0.0, 1e-3, 0.05, 0.08, 0.1, 0.12, 0.5, 1.0, 2.0];
    for &eps in &SMOOTH_EPS_SENSITIVITY {
        for &alpha in &[0.05, 0.1181, 0.1392266, 2.0] {
            for &d in &ds {
                if d >= RUNNING_DENOM_THRESHOLD + eps && d == 0.0 {
                    continue;
                }
                if d.abs() < 1e-18 && d > RUNNING_DENOM_THRESHOLD - eps {
                    continue;
                }
                let y = smooth_alpha_s_from_denominator(alpha, d, eps);
                assert!(y.is_finite(), "α={alpha} d={d} ε={eps} -> {y}");
                assert!(y > 0.0, "α={alpha} d={d} ε={eps} -> {y}");
            }
        }
    }
}

#[test]
fn smooth_limit_small_eps_matches_historical_away_from_cut() {
    let alpha_loop = 0.08;
    let d_loop = hadronic_d_from_alpha(alpha_loop);
    assert!(d_loop > 0.1 + 0.05);
    let hist = alpha_loop / d_loop;
    for &eps in &[1e-6, 1e-4, 0.005] {
        close(
            smooth_alpha_s_from_denominator(alpha_loop, d_loop, eps),
            hist,
            1e-14,
            1e-18,
        );
    }
    let alpha_cut = 0.5;
    let d_cut = hadronic_d_from_alpha(alpha_cut);
    assert!(d_cut < 0.1 - 0.05);
    for &eps in &[1e-6, 1e-4, 0.005] {
        close(
            smooth_alpha_s_from_denominator(alpha_cut, d_cut, eps),
            alpha_cut,
            0.0,
            0.0,
        );
    }
}

#[test]
fn engine_smooth_agrees_with_denominator_formula() {
    let mut laws = create_reference_universe();
    for &eps in &SMOOTH_EPS_SENSITIVITY {
        for &d_target in &[0.05, 0.08, 0.1, 0.12, 0.4] {
            laws.alpha_s = alpha_from_hadronic_d(d_target);
            let engine = AdvancedPhysicsEngine::new(laws.clone())
                .with_running_kind(RunningKind::smooth(eps));
            let d = engine.running_denominator(1.0);
            close(d, d_target, 1e-12, 1e-15);
            close(
                engine.running_alpha_s(1.0),
                smooth_alpha_s_from_denominator(laws.alpha_s, d, eps),
                1e-14,
                1e-18,
            );
        }
    }
}

fn csv_rows(path: &Path) -> Vec<Vec<String>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').map(|s| s.to_string()).collect())
        .collect()
}

#[test]
fn paired_vis01_sample_once_and_a_matches_new() {
    let dir = temp_dir("paired");
    let n = write_paired_running_csvs(&dir).expect("write paired csvs");
    assert_eq!(n, MODEL01B_UNIVERSES + 1);
    let headers = running_compare_headers();
    for spec in variant_specs() {
        let path = dir.join(spec.filename);
        let text = fs::read_to_string(&path).unwrap();
        let head: Vec<&str> = text.lines().next().unwrap().split(',').collect();
        assert_eq!(head, headers.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        assert_eq!(csv_rows(&path).len(), 401);
    }

    let a_rows = csv_rows(&dir.join("variant_A_historical.csv"));
    let mut rng = HarmonicRng::new(Some(MODEL01B_SEED));
    for i in 0..=MODEL01B_UNIVERSES {
        let laws = sample_mapped_universe(&mut rng);
        let hist = AdvancedPhysicsEngine::new(laws.clone());
        let tagged = AdvancedPhysicsEngine::new(laws.clone())
            .with_running_kind(RunningKind::Historical);
        assert_eq!(hist.evaluate(), tagged.evaluate());
        assert_eq!(
            hist.running_alpha_s(1.0).to_bits(),
            tagged.running_alpha_s(1.0).to_bits()
        );
        let row = &a_rows[i as usize];
        assert_eq!(row[0], i.to_string());
        assert_eq!(row[1], MODEL01B_SEED.to_string());
        assert_eq!(row[2], "A");
        assert_eq!(row[3], "");
        let run: f64 = row[5].parse().unwrap();
        close(run, hist.running_alpha_s(1.0), 1e-15, 1e-18);
        let fit: f64 = row.last().unwrap().parse().unwrap();
        close(fit, hist.evaluate().heuristic.fitness, 1e-15, 1e-18);
    }

    if let Ok(out) = std::env::var("HAR_MODEL01B_OUT") {
        let dest = PathBuf::from(out);
        fs::create_dir_all(&dest).expect("HAR_MODEL01B_OUT");
        for spec in variant_specs() {
            fs::copy(dir.join(spec.filename), dest.join(spec.filename)).expect("copy variant csv");
        }
    }
}

#[test]
fn variant_a_csv_matches_historical_map_evaluations() {
    let dir = temp_dir("map-a");
    write_paired_running_csvs(&dir).unwrap();
    let landscape = dir.join("landscape_data_enhanced.csv");
    let obs = dir.join("engine_evaluations.csv");
    run_mapping_mode_to_paths(MODEL01B_UNIVERSES, Some(MODEL01B_SEED), &landscape, Some(&obs))
        .unwrap();
    let a_rows = csv_rows(&dir.join("variant_A_historical.csv"));
    let map_rows = csv_rows(&obs);
    assert_eq!(a_rows.len(), map_rows.len());
    for (a, m) in a_rows.iter().zip(map_rows.iter()) {
        assert_eq!(a[0], m[0]);
        assert_eq!(&a[6..], &m[2..]);
    }
}

/// Level 5 is a documented engine state (0..=5). Samples 219 and 351 jump 1→5 under D
/// because stellar_score crosses 0.4 after α_s_had drops from the 1-loop value to α_s_ref.
/// That is not a CSV mis-join: same sample_id, same CosmicLaw, stellar is 0.0 in A (stage ran).
#[test]
fn vis01_samples_219_and_351_reach_level_five_only_under_constant_running() {
    let mut rng = HarmonicRng::new(Some(MODEL01B_SEED));
    let mut found = 0u8;
    for i in 0..=MODEL01B_UNIVERSES {
        let laws = sample_mapped_universe(&mut rng);
        if i != 219 && i != 351 {
            continue;
        }
        found += 1;
        let a = AdvancedPhysicsEngine::new(laws.clone()).evaluate();
        let d = AdvancedPhysicsEngine::new(laws.clone())
            .with_running_kind(RunningKind::Constant)
            .evaluate();
        let b = AdvancedPhysicsEngine::new(laws)
            .with_running_kind(RunningKind::smooth(SMOOTH_EPS_DEFAULT))
            .evaluate();
        assert_eq!(a.heuristic.complexity_level, 1);
        assert_eq!(a.heuristic.stellar_term, Some(0.0));
        assert!(a.heuristic.heavy_element_term.is_none());
        assert!(a.heuristic.black_hole_term.is_none());
        assert!(!a.heuristic.level_bonus_applied);
        assert_eq!(b.heuristic.complexity_level, 1);
        assert_eq!(d.heuristic.complexity_level, 5);
        assert_eq!(d.heuristic.stellar_term, Some(1.0));
        assert_eq!(d.heuristic.heavy_element_term, Some(1.0));
        assert_eq!(d.heuristic.black_hole_term, Some(1.0));
        assert!(d.heuristic.level_bonus_applied);
        assert_eq!(d.heuristic.fitness, 1.0);
    }
    assert_eq!(found, 2);
}
