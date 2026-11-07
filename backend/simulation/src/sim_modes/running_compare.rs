//! Paired A/B/D running comparison (HAR-MODEL-01B). Test/experiment helper, not CLI.

use super::map::sample_mapped_universe;
use crate::physics::engine::{
    AdvancedPhysicsEngine, RunningKind, SMOOTH_EPS_DEFAULT, SMOOTH_EPS_SENSITIVITY,
};
use crate::physics::observables::{evaluation_csv_record, EVALUATION_CSV_HEADERS};
use crate::utils::HarmonicRng;
use csv::Writer;
use std::error::Error;
use std::path::Path;

pub(crate) const MODEL01B_SEED: u64 = 137_137;
pub(crate) const MODEL01B_UNIVERSES: u64 = 400;

pub(crate) const RUNNING_COMPARE_EXTRA_HEADERS: [&str; 4] = [
    "variant",
    "epsilon",
    "running_denominator",
    "running_alpha_s_1gev",
];

#[derive(Clone, Copy)]
pub(crate) struct VariantSpec {
    pub variant: &'static str,
    pub kind: RunningKind,
    pub epsilon: Option<f64>,
    pub filename: &'static str,
}

pub(crate) fn variant_specs() -> [VariantSpec; 5] {
    [
        VariantSpec {
            variant: "A",
            kind: RunningKind::Historical,
            epsilon: None,
            filename: "variant_A_historical.csv",
        },
        VariantSpec {
            variant: "B",
            kind: RunningKind::smooth(SMOOTH_EPS_DEFAULT),
            epsilon: Some(SMOOTH_EPS_DEFAULT),
            filename: "variant_B_smooth_eps0p02.csv",
        },
        VariantSpec {
            variant: "D",
            kind: RunningKind::Constant,
            epsilon: None,
            filename: "variant_D_constant.csv",
        },
        VariantSpec {
            variant: "B",
            kind: RunningKind::smooth(SMOOTH_EPS_SENSITIVITY[0]),
            epsilon: Some(SMOOTH_EPS_SENSITIVITY[0]),
            filename: "variant_B_smooth_eps0p005.csv",
        },
        VariantSpec {
            variant: "B",
            kind: RunningKind::smooth(SMOOTH_EPS_SENSITIVITY[2]),
            epsilon: Some(SMOOTH_EPS_SENSITIVITY[2]),
            filename: "variant_B_smooth_eps0p05.csv",
        },
    ]
}

pub(crate) fn running_compare_headers() -> Vec<String> {
    let mut h = vec![
        EVALUATION_CSV_HEADERS[0].to_string(),
        EVALUATION_CSV_HEADERS[1].to_string(),
    ];
    h.extend(RUNNING_COMPARE_EXTRA_HEADERS.iter().map(|s| s.to_string()));
    h.extend(
        EVALUATION_CSV_HEADERS
            .iter()
            .skip(2)
            .map(|s| s.to_string()),
    );
    h
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

/// Sample VIS-01 once, then evaluate every variant on the same `CosmicLaw`s.
pub(crate) fn write_paired_running_csvs(dir: &Path) -> Result<u64, Box<dyn Error>> {
    std::fs::create_dir_all(dir)?;
    let mut rng = HarmonicRng::new(Some(MODEL01B_SEED));
    let mut samples = Vec::with_capacity((MODEL01B_UNIVERSES + 1) as usize);
    for i in 0..=MODEL01B_UNIVERSES {
        samples.push((i, sample_mapped_universe(&mut rng)));
    }

    let headers = running_compare_headers();
    for spec in variant_specs() {
        let path = dir.join(spec.filename);
        let mut wtr = Writer::from_path(&path)?;
        wtr.write_record(&headers)?;
        for (sample_id, laws) in &samples {
            let engine = AdvancedPhysicsEngine::new(laws.clone()).with_running_kind(spec.kind);
            let denom = engine.running_denominator(1.0);
            let run = engine.running_alpha_s(1.0);
            let ev = engine.evaluate();
            let ev_rec = evaluation_csv_record(*sample_id, Some(MODEL01B_SEED), laws, &ev);
            let eps = spec
                .epsilon
                .map(csv_f64)
                .unwrap_or_default();
            let mut row = vec![
                ev_rec[0].clone(),
                ev_rec[1].clone(),
                spec.variant.to_string(),
                eps,
                csv_f64(denom),
                csv_f64(run),
            ];
            row.extend(ev_rec.into_iter().skip(2));
            wtr.write_record(&row)?;
        }
        wtr.flush()?;
    }
    Ok(samples.len() as u64)
}
