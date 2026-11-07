//! HAR-EVO-01 software tests. Not habitability and not Smolin cosmology.

use super::evo01::{
    alpha_hi, alpha_lo, control_parent, derive_seed, diversity_of, e_from_alpha, exclusive_clamp,
    gamma_hi, gamma_lo, run_both_pops, tournament_winner, universe_id, write_evo01_run, Arm,
    EvoConfig, EvalStatus, Individual, ALPHA_S_HI, ALPHA_S_LO, EVO01_BASE_SEED, EVO01_SIGMA,
    INDIVIDUAL_HEADERS, MAP_E_HI, MAP_E_LO, MAP_G_HI, MAP_G_LO,
};
use super::map::sample_mapped_universe;
use crate::physics::alpha::si_alpha;
use crate::physics::constants::G_GRAVITATIONAL;
use crate::physics::engine::{AdvancedPhysicsEngine, RunningKind};
use crate::sim_modes::validate::create_reference_universe;
use crate::utils::HarmonicRng;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

struct TempTree(PathBuf);

impl TempTree {
    fn new(label: &str) -> Self {
        let n = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "har-evo01-{}-{}-{}",
            label,
            std::process::id(),
            n
        ));
        fs::create_dir_all(&dir).expect("temp dir");
        Self(dir)
    }
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempTree {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn dummy_ind(id: u64, slot: u32, fit: f64, status: EvalStatus) -> Individual {
    Individual {
        universe_id: id,
        parent_id: None,
        slot,
        generation: 0,
        genome: super::evo01::Genome {
            alpha: si_alpha(1.6e-19),
            alpha_s: 0.12,
            gamma: 1.5,
        },
        e_si: 1.6e-19,
        g_si: 1.5 * G_GRAVITATIONAL,
        fitness: fit,
        complexity_level: 0,
        eval_status: status,
        mutation_effective: 0.0,
        clamped_genes: String::new(),
        discard_reason: String::new(),
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
fn alpha_e_roundtrip_and_gamma_g() {
    let e = 1.602176634e-19;
    let a = si_alpha(e);
    let e2 = e_from_alpha(a);
    assert!((si_alpha(e2) - a).abs() <= 4.0 * f64::EPSILON * a.max(1e-16));
    let g = 3.0 * G_GRAVITATIONAL;
    let gamma = g / G_GRAVITATIONAL;
    assert_eq!((gamma * G_GRAVITATIONAL).to_bits(), g.to_bits());
    assert!(alpha_lo() < a && a < alpha_hi());
    assert!(gamma_lo() < 1.0 + 1e-6);
    assert!(gamma_hi() > 9.9);
}

#[test]
fn exclusive_clamp_never_equals_hi() {
    let hi = ALPHA_S_HI;
    let lo = ALPHA_S_LO;
    let (y, c) = exclusive_clamp(hi, lo, hi);
    assert!(c);
    assert!(y < hi);
    assert_eq!(y.to_bits(), hi.next_down().to_bits());
    let (z, c2) = exclusive_clamp(lo - 1e-6, lo, hi);
    assert!(c2);
    assert_eq!(z.to_bits(), lo.to_bits());
    let (inf, c3) = exclusive_clamp(f64::INFINITY, lo, hi);
    assert!(!c3);
    assert!(inf.is_infinite());
}

#[test]
fn tournament_rejects_nonfinite_and_prefers_higher_finite() {
    let ok_low = dummy_ind(10, 0, 0.0, EvalStatus::Ok);
    let ok_hi = dummy_ind(11, 1, 0.4, EvalStatus::Ok);
    let bad = dummy_ind(12, 2, 0.9, EvalStatus::Skipped);
    let nan = dummy_ind(13, 3, f64::NAN, EvalStatus::NonfiniteEval);
    assert_eq!(
        tournament_winner(&ok_low, &ok_hi).universe_id,
        ok_hi.universe_id
    );
    assert_eq!(tournament_winner(&ok_low, &bad).universe_id, ok_low.universe_id);
    assert_eq!(tournament_winner(&nan, &ok_hi).universe_id, ok_hi.universe_id);
    assert_eq!(tournament_winner(&bad, &nan).universe_id, bad.universe_id);
}

#[test]
fn control_parent_ignores_fitness() {
    let a = dummy_ind(1, 0, 0.99, EvalStatus::Ok);
    let b = dummy_ind(2, 1, 0.01, EvalStatus::Ok);
    let pop = vec![a, b];
    assert_eq!(control_parent(&pop, 1).universe_id, 2);
    assert_eq!(control_parent(&pop, 1).fitness, 0.01);
}

#[test]
fn diversity_zero_for_clones() {
    let a = dummy_ind(1, 0, 0.1, EvalStatus::Ok);
    let mut b = dummy_ind(2, 1, 0.2, EvalStatus::Ok);
    b.genome = a.genome.clone();
    assert_eq!(diversity_of(&[a.clone(), b]), 0.0);
}

#[test]
fn new_remains_historical() {
    let laws = create_reference_universe();
    let a = AdvancedPhysicsEngine::new(laws.clone());
    let hist = AdvancedPhysicsEngine::new(laws).with_running_kind(RunningKind::Historical);
    assert_eq!(a.evaluate(), hist.evaluate());
}

#[test]
fn smoke_determinism_shared_gen0_box_and_parents() {
    let cfg = EvoConfig::smoke();
    let (sel_a, ctrl_a) = run_both_pops(cfg);
    let (sel_b, ctrl_b) = run_both_pops(cfg);
    assert_eq!(sel_a.len(), (cfg.transitions + 1) as usize);
    assert_eq!(ctrl_a.len(), sel_a.len());
    for g in 0..sel_a.len() {
        assert_eq!(sel_a[g].len(), cfg.n as usize);
        assert_eq!(ctrl_a[g].len(), cfg.n as usize);
        for s in 0..cfg.n as usize {
            let ia = &sel_a[g][s];
            let ib = &sel_b[g][s];
            assert_eq!(ia.genome.alpha.to_bits(), ib.genome.alpha.to_bits());
            assert_eq!(ia.fitness.to_bits(), ib.fitness.to_bits());
            assert_eq!(ia.universe_id, universe_id(Arm::Selection, g as u32, s as u32));
            assert!(ia.genome.alpha >= alpha_lo() && ia.genome.alpha < alpha_hi());
            assert!(ia.genome.alpha_s >= ALPHA_S_LO && ia.genome.alpha_s < ALPHA_S_HI);
            assert!(ia.genome.gamma >= gamma_lo() && ia.genome.gamma < gamma_hi());
            assert!(ia.e_si >= MAP_E_LO && ia.e_si < MAP_E_HI);
            assert!(ia.g_si >= MAP_G_LO && ia.g_si < MAP_G_HI);
            if g == 0 {
                assert!(ia.parent_id.is_none());
                assert_eq!(ia.genome.alpha.to_bits(), ctrl_a[0][s].genome.alpha.to_bits());
                assert_eq!(
                    ia.genome.alpha_s.to_bits(),
                    ctrl_a[0][s].genome.alpha_s.to_bits()
                );
                assert_eq!(ia.genome.gamma.to_bits(), ctrl_a[0][s].genome.gamma.to_bits());
            } else {
                let p = ia.parent_id.expect("parent");
                assert!(sel_a[g - 1].iter().any(|x| x.universe_id == p));
                let cp = ctrl_a[g][s].parent_id.expect("ctrl parent");
                assert_eq!(cp, ctrl_a[g - 1][s].universe_id);
            }
        }
    }
    assert_eq!(sel_a.len(), sel_b.len());
    let _ = ctrl_b;
}

#[test]
fn csv_replay_and_schema() {
    let d1 = TempTree::new("csv-a");
    let d2 = TempTree::new("csv-b");
    let cfg = EvoConfig::smoke();
    write_evo01_run(d1.path(), cfg).unwrap();
    write_evo01_run(d2.path(), cfg).unwrap();
    let a = fs::read(d1.path().join("evo01_individuals.csv")).unwrap();
    let b = fs::read(d2.path().join("evo01_individuals.csv")).unwrap();
    assert_eq!(a, b);
    let header = fs::read_to_string(d1.path().join("evo01_individuals.csv"))
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .to_string();
    assert_eq!(header, INDIVIDUAL_HEADERS.join(","));
    let rows = csv_rows(&d1.path().join("evo01_individuals.csv"));
    assert_eq!(rows.len(), (cfg.n * (cfg.transitions + 1) * 2) as usize);
    assert!(d1.path().join("evo01_metrics.csv").is_file());
    assert!(d1.path().join("evo01_summary.json").is_file());
}

#[test]
fn map_sampler_still_uniform_e_not_alpha() {
    let mut rng = HarmonicRng::new(Some(EVO01_BASE_SEED));
    let laws = sample_mapped_universe(&mut rng);
    assert!(laws.e >= MAP_E_LO && laws.e < MAP_E_HI);
}

#[test]
fn derive_seed_is_stable_splitmix() {
    assert_ne!(derive_seed(EVO01_BASE_SEED, 0), derive_seed(EVO01_BASE_SEED, 1));
    assert_eq!(
        derive_seed(EVO01_BASE_SEED, 0),
        derive_seed(EVO01_BASE_SEED, 0)
    );
}

#[test]
fn sigma_is_approved() {
    assert_eq!(EVO01_SIGMA.to_bits(), 0.05f64.to_bits());
}

#[test]
fn universe_id_has_no_collisions_in_supported_box() {
    let mut seen = std::collections::HashSet::new();
    for arm in [Arm::Selection, Arm::Control] {
        for gen in 0u32..=100 {
            for slot in 0u32..200 {
                assert!(seen.insert(universe_id(arm, gen, slot)));
            }
        }
    }
    assert_eq!(seen.len(), 2 * 101 * 200);
}

#[test]
fn primary_replay_tempdirs_genealogy_and_metrics() {
    let d1 = TempTree::new("primary-a");
    let d2 = TempTree::new("primary-b");
    let cfg = EvoConfig::primary();
    let s1 = write_evo01_run(d1.path(), cfg).unwrap();
    let s2 = write_evo01_run(d2.path(), cfg).unwrap();
    assert_eq!(s1.evaluations_per_arm, 3100);
    assert_eq!(s2.evaluations_per_arm, 3100);
    for name in [
        "evo01_individuals.csv",
        "evo01_metrics.csv",
        "evo01_summary.json",
    ] {
        let a = fs::read(d1.path().join(name)).unwrap();
        let b = fs::read(d2.path().join(name)).unwrap();
        assert_eq!(a, b, "{name} replay mismatch");
    }
    let rows = csv_rows(&d1.path().join("evo01_individuals.csv"));
    assert_eq!(rows.len(), 6200);
    validate_individual_rows(&rows, cfg.n, cfg.transitions);
    assert_approx_metrics(&csv_rows(&d1.path().join("evo01_metrics.csv")));
    if let Ok(out) = std::env::var("HAR_EVO01_OUT") {
        let dest = PathBuf::from(out);
        fs::create_dir_all(&dest).unwrap();
        for name in [
            "evo01_individuals.csv",
            "evo01_metrics.csv",
            "evo01_summary.json",
        ] {
            fs::copy(d1.path().join(name), dest.join(name)).unwrap();
        }
    }
    assert!((s1.last_best_fitness_selection - 1.0).abs() < 1e-12);
    assert!((s1.last_median_fitness_selection - 0.661).abs() < 0.002);
    assert!((s1.last_best_fitness_control - 0.649).abs() < 0.002);
    assert!((s1.last_median_fitness_control - 0.326).abs() < 0.002);
}

fn col(name: &str) -> usize {
    INDIVIDUAL_HEADERS
        .iter()
        .position(|h| *h == name)
        .unwrap_or_else(|| panic!("{name}"))
}

fn validate_individual_rows(rows: &[Vec<String>], n: u32, t: u32) {
    let i_arm = col("arm");
    let i_gen = col("generation");
    let i_uid = col("universe_id");
    let i_pid = col("parent_id");
    let i_slot = col("slot");
    let i_alpha = col("alpha");
    let i_as = col("alpha_s");
    let i_g = col("gamma");
    let i_fit = col("fitness");
    let i_lv = col("complexity_level");
    let i_st = col("eval_status");
    let mut ids = std::collections::HashSet::new();
    let mut by_key = std::collections::HashMap::new();
    let mut sel_g0 = Vec::new();
    let mut ctrl_g0 = Vec::new();
    let mut gamma_hi_n = 0u32;
    let mut as_near = 0u32;
    let mut lv5 = 0u32;
    let mut sel_last = 0u32;
    for row in rows {
        let uid: u64 = row[i_uid].parse().unwrap();
        assert!(ids.insert(uid), "duplicate universe_id {uid}");
        let gen: u32 = row[i_gen].parse().unwrap();
        let slot: u32 = row[i_slot].parse().unwrap();
        let arm = row[i_arm].as_str();
        by_key.insert((arm.to_string(), gen, slot), uid);
        assert!(row[i_st] == "ok");
        let fit: f64 = row[i_fit].parse().unwrap();
        assert!(fit.is_finite());
        let alpha: f64 = row[i_alpha].parse().unwrap();
        let alpha_s: f64 = row[i_as].parse().unwrap();
        let gamma: f64 = row[i_g].parse().unwrap();
        assert!(alpha >= alpha_lo() && alpha < alpha_hi());
        assert!(alpha_s >= ALPHA_S_LO && alpha_s < ALPHA_S_HI);
        assert!(gamma >= gamma_lo() && gamma < gamma_hi());
        if gen == 0 {
            assert!(row[i_pid].is_empty());
            if arm == "selection" {
                sel_g0.push((alpha.to_bits(), alpha_s.to_bits(), gamma.to_bits()));
            } else {
                ctrl_g0.push((alpha.to_bits(), alpha_s.to_bits(), gamma.to_bits()));
            }
        } else {
            let pid: u64 = row[i_pid].parse().unwrap();
            assert!(
                rows.iter().any(|r| r[i_uid] == pid.to_string()
                    && r[i_arm] == arm
                    && r[i_gen].parse::<u32>().unwrap() == gen - 1),
                "missing parent {pid}"
            );
            if arm == "control" {
                assert_eq!(pid, *by_key.get(&("control".into(), gen - 1, slot)).unwrap());
            }
        }
        if arm == "selection" && gen == t {
            sel_last += 1;
            if gamma > 9.0 {
                gamma_hi_n += 1;
            }
            if (alpha_s - 0.118).abs() < 0.02 {
                as_near += 1;
            }
            if row[i_lv] == "5" {
                lv5 += 1;
            }
        }
    }
    assert_eq!(sel_g0, ctrl_g0);
    assert_eq!(sel_last, n);
    assert!((gamma_hi_n as f64 / n as f64 - 0.91).abs() < 0.015);
    assert!((as_near as f64 / n as f64 - 0.87).abs() < 0.015);
    assert!((lv5 as f64 / n as f64 - 0.23).abs() < 0.015);
    assert_eq!(ids.len(), (n * (t + 1) * 2) as usize);
}

fn assert_approx_metrics(rows: &[Vec<String>]) {
    let headers = [
        "run_id",
        "arm",
        "generation",
        "best_fitness",
        "median_fitness",
        "q1_fitness",
        "q3_fitness",
        "iqr_fitness",
        "frac_level_0",
        "frac_level_1",
        "frac_level_2",
        "frac_level_3",
        "frac_level_4",
        "frac_level_5",
        "diversity",
        "mean_dist_to_bounds",
        "frac_near_border",
        "frac_mutations_clamped",
        "mutation_rate_effective",
        "parent_concentration",
    ];
    let find = |name: &str| headers.iter().position(|h| *h == name).unwrap();
    let i_arm = find("arm");
    let i_gen = find("generation");
    let i_best = find("best_fitness");
    let i_med = find("median_fitness");
    let i_div = find("diversity");
    let mut g0_best = None;
    let mut g0_med = None;
    let mut g0_div = None;
    for row in rows {
        let gen: u32 = row[i_gen].parse().unwrap();
        let arm = row[i_arm].as_str();
        let best: f64 = row[i_best].parse().unwrap();
        let med: f64 = row[i_med].parse().unwrap();
        let div: f64 = row[i_div].parse().unwrap();
        if gen == 0 {
            if g0_best.is_none() {
                g0_best = Some(best);
                g0_med = Some(med);
                g0_div = Some(div);
            }
            assert!((best - 0.6435).abs() < 5e-4);
            assert!((med - 0.3094).abs() < 5e-4);
            assert!((div - 0.51).abs() < 0.02);
            if arm == "control" {
                assert_eq!(best.to_bits(), g0_best.unwrap().to_bits());
                assert_eq!(med.to_bits(), g0_med.unwrap().to_bits());
            }
        }
        if gen == 30 && arm == "selection" {
            assert!((best - 1.0).abs() < 1e-12);
            assert!((med - 0.661).abs() < 0.002);
            assert!((div - 0.12).abs() < 0.02);
        }
        if gen == 30 && arm == "control" {
            assert!((best - 0.649).abs() < 0.002);
            assert!((med - 0.326).abs() < 0.002);
        }
    }
    let _ = g0_div;
}
