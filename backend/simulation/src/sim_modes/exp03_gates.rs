//! HAR-EXP-03 census and 1-parameter scans. Test/experiment helper, not CLI.

use super::map::sample_mapped_universe;
use crate::core::models::CosmicLaw;
use crate::physics::engine::{AdvancedPhysicsEngine, RunningKind};
use crate::physics::observables::{evaluation_csv_record, EVALUATION_CSV_HEADERS};
use crate::utils::HarmonicRng;
use csv::Writer;
use std::error::Error;
use std::path::Path;

pub(crate) const EXP03_SEED: u64 = 137_137;
pub(crate) const EXP03_UNIVERSES: u64 = 400;
pub(crate) const EXP03_ID_STELLAR_A: u64 = 189;
pub(crate) const EXP03_ID_D_LEVEL5_A: u64 = 219;
pub(crate) const EXP03_ID_D_LEVEL5_B: u64 = 351;

/// Same window as `stellar_formation_details` (1 Myr / 100 Gyr with 365.25-day years).
pub(crate) const STELLAR_MIN_LIFETIME_S: f64 = 1e6 * 365.25 * 24.0 * 3600.0;
pub(crate) const STELLAR_MAX_LIFETIME_S: f64 = 1e11 * 365.25 * 24.0 * 3600.0;

pub(crate) const EXP03_EXTRA_HEADERS: [&str; 6] = [
    "variant",
    "jeans_diagnostic_kg",
    "stellar_lifetime_s",
    "stellar_lifetime_years",
    "lifetime_branch",
    "scan_axis",
];

#[derive(Clone, Copy)]
pub(crate) struct VariantSpec {
    pub variant: &'static str,
    pub kind: RunningKind,
    pub filename: &'static str,
}

pub(crate) fn vis01_variant_specs() -> [VariantSpec; 2] {
    [
        VariantSpec {
            variant: "A",
            kind: RunningKind::Historical,
            filename: "vis01_gates_A.csv",
        },
        VariantSpec {
            variant: "D",
            kind: RunningKind::Constant,
            filename: "vis01_gates_D.csv",
        },
    ]
}

fn csv_f64(x: f64) -> String {
    if x.is_nan() {
        "nan".to_string()
    } else if x.is_infinite() {
        if x.is_sign_positive() {
            "inf".to_string()
        } else {
            "-inf".to_string()
        }
    } else {
        format!("{:.16e}", x)
    }
}

pub(crate) fn lifetime_branch(tau_s: f64) -> &'static str {
    if tau_s < STELLAR_MIN_LIFETIME_S {
        "below_1myr"
    } else if tau_s > STELLAR_MAX_LIFETIME_S {
        "above_100gyr"
    } else {
        "in_window"
    }
}

pub(crate) fn sample_vis01() -> Vec<(u64, CosmicLaw)> {
    let mut rng = HarmonicRng::new(Some(EXP03_SEED));
    let mut samples = Vec::with_capacity((EXP03_UNIVERSES + 1) as usize);
    for i in 0..=EXP03_UNIVERSES {
        samples.push((i, sample_mapped_universe(&mut rng)));
    }
    samples
}

pub(crate) fn vis01_headers() -> Vec<String> {
    let mut h = vec![
        EVALUATION_CSV_HEADERS[0].to_string(),
        EVALUATION_CSV_HEADERS[1].to_string(),
    ];
    h.extend(EXP03_EXTRA_HEADERS.iter().map(|s| s.to_string()));
    h.extend(
        EVALUATION_CSV_HEADERS
            .iter()
            .skip(2)
            .map(|s| s.to_string()),
    );
    h
}

fn diagnostic_row(
    sample_id: u64,
    seed: u64,
    variant: &str,
    scan_axis: &str,
    laws: &CosmicLaw,
    kind: RunningKind,
) -> Vec<String> {
    let engine = AdvancedPhysicsEngine::new(laws.clone()).with_running_kind(kind);
    let jeans = engine.calculate_jeans_mass();
    let tau_s = engine.main_sequence_lifetime(jeans);
    let tau_yr = if tau_s.is_finite() {
        tau_s / (365.25 * 24.0 * 3600.0)
    } else {
        f64::NAN
    };
    let ev = engine.evaluate();
    let ev_rec = evaluation_csv_record(sample_id, Some(seed), laws, &ev);
    let mut row = vec![
        ev_rec[0].clone(),
        ev_rec[1].clone(),
        variant.to_string(),
        csv_f64(jeans),
        csv_f64(tau_s),
        csv_f64(tau_yr),
        lifetime_branch(tau_s).to_string(),
        scan_axis.to_string(),
    ];
    row.extend(ev_rec.into_iter().skip(2));
    row
}

/// One VIS-01 draw, then A and D on the same laws. Extra Jeans/lifetime are diagnostics.
pub(crate) fn write_vis01_gate_csvs(dir: &Path) -> Result<u64, Box<dyn Error>> {
    std::fs::create_dir_all(dir)?;
    let samples = sample_vis01();
    let headers = vis01_headers();
    for spec in vis01_variant_specs() {
        let mut wtr = Writer::from_path(dir.join(spec.filename))?;
        wtr.write_record(&headers)?;
        for (id, laws) in &samples {
            wtr.write_record(&diagnostic_row(
                *id,
                EXP03_SEED,
                spec.variant,
                "vis01",
                laws,
                spec.kind,
            ))?;
        }
        wtr.flush()?;
    }
    Ok(samples.len() as u64)
}

fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![start];
    }
    (0..n)
        .map(|i| start + (end - start) * (i as f64) / ((n - 1) as f64))
        .collect()
}

pub(crate) fn vis01_laws(sample_id: u64) -> CosmicLaw {
    sample_vis01()
        .into_iter()
        .find(|(i, _)| *i == sample_id)
        .expect("sample_id in VIS-01")
        .1
}

/// 1-parameter scans on a frozen VIS-01 law. Historical running (A) only.
pub(crate) fn write_sensitivity_csvs(dir: &Path) -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(dir)?;
    let headers = vis01_headers();
    let base_189 = vis01_laws(EXP03_ID_STELLAR_A);
    let base_219 = vis01_laws(EXP03_ID_D_LEVEL5_A);

    let jobs: [(&str, CosmicLaw, &str, Vec<f64>); 6] = [
        (
            "scan_189_alpha_s.csv",
            base_189.clone(),
            "alpha_s",
            {
                let mut v = linspace(0.05, 0.50, 91);
                v.extend(linspace(0.08, 0.16, 81));
                v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                v.dedup_by(|a, b| (*a - *b).abs() < 1e-15);
                v
            },
        ),
        (
            "scan_189_e.csv",
            base_189.clone(),
            "e",
            linspace(base_189.e * 0.25, base_189.e * 2.0, 81),
        ),
        (
            "scan_189_g.csv",
            base_189.clone(),
            "g",
            linspace(6.674e-11, 6.674e-10, 91),
        ),
        (
            "scan_219_alpha_s.csv",
            base_219.clone(),
            "alpha_s",
            {
                let mut v = linspace(0.05, 0.50, 91);
                v.extend(linspace(0.08, 0.16, 81));
                v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                v.dedup_by(|a, b| (*a - *b).abs() < 1e-15);
                v
            },
        ),
        (
            "scan_219_e.csv",
            base_219.clone(),
            "e",
            linspace(base_219.e * 0.25, base_219.e * 2.0, 81),
        ),
        (
            "scan_219_g.csv",
            base_219,
            "g",
            linspace(6.674e-11, 6.674e-10, 91),
        ),
    ];

    for (filename, base, axis, values) in jobs {
        let mut wtr = Writer::from_path(dir.join(filename))?;
        wtr.write_record(&headers)?;
        for (k, x) in values.into_iter().enumerate() {
            let mut laws = base.clone();
            match axis {
                "alpha_s" => laws.alpha_s = x,
                "e" => laws.e = x,
                "g" => laws.g = x,
                _ => unreachable!(),
            }
            wtr.write_record(&diagnostic_row(
                k as u64,
                EXP03_SEED,
                "A",
                axis,
                &laws,
                RunningKind::Historical,
            ))?;
        }
        wtr.flush()?;
    }
    Ok(())
}
