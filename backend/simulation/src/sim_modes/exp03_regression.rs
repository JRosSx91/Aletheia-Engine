//! HAR-EXP-03 gate census tests. Software traces, not habitability claims.

use super::exp03_gates::{
    lifetime_branch, sample_vis01, vis01_headers, vis01_laws, write_sensitivity_csvs,
    write_vis01_gate_csvs, EXP03_ID_D_LEVEL5_A, EXP03_ID_D_LEVEL5_B, EXP03_ID_STELLAR_A,
    EXP03_SEED, EXP03_UNIVERSES, STELLAR_MAX_LIFETIME_S, STELLAR_MIN_LIFETIME_S,
};
use super::map::sample_mapped_universe;
use crate::physics::engine::{AdvancedPhysicsEngine, RunningKind};
use crate::utils::HarmonicRng;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

fn temp_dir(label: &str) -> PathBuf {
    let n = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "har-exp03-{}-{}-{}",
        label,
        std::process::id(),
        n
    ));
    fs::create_dir_all(&dir).expect("temp dir");
    dir
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

fn col(headers: &[String], name: &str) -> usize {
    headers.iter().position(|h| h == name).expect(name)
}

#[test]
fn vis01_single_draw_feeds_a_and_d() {
    let samples = sample_vis01();
    assert_eq!(samples.len(), (EXP03_UNIVERSES + 1) as usize);
    let mut rng = HarmonicRng::new(Some(EXP03_SEED));
    for (_id, laws) in &samples {
        let replayed = sample_mapped_universe(&mut rng);
        assert_eq!(laws.g.to_bits(), replayed.g.to_bits());
        assert_eq!(laws.e.to_bits(), replayed.e.to_bits());
        assert_eq!(laws.alpha_s.to_bits(), replayed.alpha_s.to_bits());
    }
}

#[test]
fn vis01_a_level_distribution_and_stellar_passers() {
    let dir = temp_dir("census");
    let n = write_vis01_gate_csvs(&dir).unwrap();
    assert_eq!(n, 401);
    let headers = vis01_headers();
    let rows = csv_rows(&dir.join("vis01_gates_A.csv"));
    assert_eq!(rows.len(), 401);
    let i_level = col(&headers, "complexity_level");
    let i_st = col(&headers, "stellar_term");
    let i_hv = col(&headers, "heavy_element_term");
    let i_bh = col(&headers, "black_hole_term");
    let i_id = col(&headers, "sample_id");
    let mut levels = [0u32; 6];
    let mut stellar_gt = 0u32;
    let mut heavy_eval = 0u32;
    let mut bh_eval = 0u32;
    let mut passers = Vec::new();
    for row in &rows {
        let lv: usize = row[i_level].parse().unwrap();
        levels[lv] += 1;
        if !row[i_st].is_empty() {
            let st: f64 = row[i_st].parse().unwrap();
            if st > 0.4 {
                stellar_gt += 1;
                passers.push(row[i_id].parse::<u64>().unwrap());
            }
        }
        if !row[i_hv].is_empty() {
            heavy_eval += 1;
        }
        if !row[i_bh].is_empty() {
            bh_eval += 1;
        }
    }
    assert_eq!(levels, [94, 299, 8, 0, 0, 0]);
    assert_eq!(stellar_gt, 8);
    assert_eq!(heavy_eval, 8);
    assert_eq!(bh_eval, 0);
    assert_eq!(passers, vec![175, 189, 200, 227, 236, 242, 298, 373]);

    if let Ok(out) = std::env::var("HAR_EXP03_OUT") {
        let dest = PathBuf::from(out);
        fs::create_dir_all(&dest).unwrap();
        fs::copy(dir.join("vis01_gates_A.csv"), dest.join("vis01_gates_A.csv")).unwrap();
        fs::copy(dir.join("vis01_gates_D.csv"), dest.join("vis01_gates_D.csv")).unwrap();
    }
}

#[test]
fn vis01_d_opens_219_and_351_to_level_five_same_ids() {
    let dir = temp_dir("ad");
    write_vis01_gate_csvs(&dir).unwrap();
    let headers = vis01_headers();
    let a = csv_rows(&dir.join("vis01_gates_A.csv"));
    let d = csv_rows(&dir.join("vis01_gates_D.csv"));
    let i_id = col(&headers, "sample_id");
    let i_g = col(&headers, "g_si");
    let i_as = col(&headers, "alpha_s_reference");
    let i_lv = col(&headers, "complexity_level");
    let i_st = col(&headers, "stellar_term");
    let i_branch = col(&headers, "lifetime_branch");
    for id in [EXP03_ID_D_LEVEL5_A, EXP03_ID_D_LEVEL5_B] {
        let ra = a.iter().find(|r| r[i_id] == id.to_string()).unwrap();
        let rd = d.iter().find(|r| r[i_id] == id.to_string()).unwrap();
        assert_eq!(ra[i_g], rd[i_g]);
        assert_eq!(ra[i_as], rd[i_as]);
        assert_eq!(ra[i_lv], "1");
        assert_eq!(rd[i_lv], "5");
        let st_a: f64 = ra[i_st].parse().unwrap();
        let st_d: f64 = rd[i_st].parse().unwrap();
        assert_eq!(st_a, 0.0);
        assert!(st_d > 0.4);
        assert_eq!(ra[i_branch], "below_1myr");
        assert_eq!(rd[i_branch], "in_window");
    }
}

#[test]
fn sample_189_passes_stellar_under_historical_a() {
    let laws = vis01_laws(EXP03_ID_STELLAR_A);
    let a = AdvancedPhysicsEngine::new(laws.clone());
    let tagged = AdvancedPhysicsEngine::new(laws).with_running_kind(RunningKind::Historical);
    let ev = a.evaluate();
    assert_eq!(ev, tagged.evaluate());
    assert_eq!(ev.heuristic.complexity_level, 2);
    assert_eq!(ev.heuristic.stellar_term, Some(1.0));
    assert!(ev.heuristic.heavy_element_term.unwrap() < 0.5);
    assert!(ev.heuristic.black_hole_term.is_none());
    let jeans = a.calculate_jeans_mass();
    let tau = a.main_sequence_lifetime(jeans);
    assert_eq!(lifetime_branch(tau), "in_window");
    assert!(tau >= STELLAR_MIN_LIFETIME_S && tau <= STELLAR_MAX_LIFETIME_S);
}

#[test]
fn new_remains_historical_on_vis01_passers() {
    for id in [EXP03_ID_STELLAR_A, EXP03_ID_D_LEVEL5_A] {
        let laws = vis01_laws(id);
        let a = AdvancedPhysicsEngine::new(laws.clone());
        let hist = AdvancedPhysicsEngine::new(laws).with_running_kind(RunningKind::Historical);
        assert_eq!(
            a.running_alpha_s(1.0).to_bits(),
            hist.running_alpha_s(1.0).to_bits()
        );
        assert_eq!(a.evaluate(), hist.evaluate());
    }
}

#[test]
fn sensitivity_scans_are_deterministic_and_one_axis() {
    let d1 = temp_dir("scan-a");
    let d2 = temp_dir("scan-b");
    write_sensitivity_csvs(&d1).unwrap();
    write_sensitivity_csvs(&d2).unwrap();
    for name in [
        "scan_189_alpha_s.csv",
        "scan_189_e.csv",
        "scan_189_g.csv",
        "scan_219_alpha_s.csv",
        "scan_219_e.csv",
        "scan_219_g.csv",
    ] {
        assert_eq!(fs::read(d1.join(name)).unwrap(), fs::read(d2.join(name)).unwrap());
        let rows = csv_rows(&d1.join(name));
        assert!(rows.len() >= 80, "{name} too short");
    }
    if let Ok(out) = std::env::var("HAR_EXP03_OUT") {
        let dest = PathBuf::from(out);
        fs::create_dir_all(&dest).unwrap();
        for name in [
            "scan_189_alpha_s.csv",
            "scan_189_e.csv",
            "scan_189_g.csv",
            "scan_219_alpha_s.csv",
            "scan_219_e.csv",
            "scan_219_g.csv",
        ] {
            fs::copy(d1.join(name), dest.join(name)).unwrap();
        }
    }
}
