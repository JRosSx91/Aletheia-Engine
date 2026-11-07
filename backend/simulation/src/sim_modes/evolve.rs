//! Public `evolve` CLI wrapper around HAR-EVO-01. Not Smolin cosmology.

use super::evo01::{write_evo01_run, EvoConfig, EvoSummary};
use std::error::Error;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

pub const EVO_OUTPUT_FILES: [&str; 3] = [
    "evo01_individuals.csv",
    "evo01_metrics.csv",
    "evo01_summary.json",
];

pub const EVO_N_MIN: u32 = 2;
pub const EVO_N_MAX: u32 = 200;
pub const EVO_T_MAX: u32 = 100;
pub const EVO_SIGMA_MAX: f64 = 0.2;
pub const EVO_MAX_TOTAL_EVALS: u64 = 50_000;

#[derive(Clone, Debug)]
pub struct EvolveRequest {
    pub population: u32,
    pub generations: u32,
    pub mutation_sigma: f64,
    pub rng_seed: u64,
    pub output_dir: PathBuf,
    pub force: bool,
}

pub fn run_evolutionary_mode(req: &EvolveRequest) -> Result<EvoSummary, Box<dyn Error>> {
    validate_evolve_request(req)?;
    let dest = &req.output_dir;
    if dest.exists() && dest.is_file() {
        return Err(io_err(format!(
            "--output-dir {} is a file, not a directory",
            dest.display()
        )));
    }
    fs::create_dir_all(dest)?;
    if !dest.is_dir() {
        return Err(io_err(format!(
            "--output-dir {} is not a directory",
            dest.display()
        )));
    }

    let collisions: Vec<PathBuf> = EVO_OUTPUT_FILES
        .iter()
        .map(|n| dest.join(n))
        .filter(|p| p.exists())
        .collect();
    if !collisions.is_empty() && !req.force {
        return Err(io_err(format!(
            "refusing to overwrite {} (pass --force)",
            collisions
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }

    let staging = dest.join(format!(".evo01-staging-{}", std::process::id()));
    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    fs::create_dir_all(&staging)?;

    let cfg = EvoConfig {
        n: req.population,
        transitions: req.generations,
        sigma: req.mutation_sigma,
        base_seed: req.rng_seed,
    };
    let summary = match write_evo01_run(&staging, cfg) {
        Ok(s) => s,
        Err(e) => {
            let _ = fs::remove_dir_all(&staging);
            return Err(e);
        }
    };

    if let Err(e) = publish_staged(&staging, dest) {
        return Err(e);
    }
    let _ = fs::remove_dir_all(&staging);

    println!(
        "evolve H-03 experimental optimizer (internal fitness index, not habitability, not Smolin)"
    );
    println!(
        "T={} transitions → generations 0..={} ({} populations/arm)",
        req.generations, req.generations, req.generations + 1
    );
    println!(
        "wrote {} (N={}, evals/arm={}) to {}",
        summary.run_id,
        summary.n,
        summary.evaluations_per_arm,
        dest.display()
    );
    Ok(summary)
}

pub(crate) fn validate_evolve_request(req: &EvolveRequest) -> Result<(), Box<dyn Error>> {
    if req.population < EVO_N_MIN || req.population > EVO_N_MAX {
        return Err(io_err(format!(
            "--population must be in [{}, {}], got {}",
            EVO_N_MIN, EVO_N_MAX, req.population
        )));
    }
    if req.generations > EVO_T_MAX {
        return Err(io_err(format!(
            "--generations (transitions T) must be in [0, {}], got {}",
            EVO_T_MAX, req.generations
        )));
    }
    if !(req.mutation_sigma > 0.0 && req.mutation_sigma <= EVO_SIGMA_MAX) {
        return Err(io_err(format!(
            "--mutation-sigma must be in (0, {}], got {}",
            EVO_SIGMA_MAX, req.mutation_sigma
        )));
    }
    let total = 2u64 * u64::from(req.population) * u64::from(req.generations.saturating_add(1));
    if total > EVO_MAX_TOTAL_EVALS {
        return Err(io_err(format!(
            "evaluation budget 2×N×(T+1)={} exceeds {}",
            total, EVO_MAX_TOTAL_EVALS
        )));
    }
    Ok(())
}

fn publish_staged(staging: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    for name in EVO_OUTPUT_FILES {
        let from = staging.join(name);
        if !from.is_file() {
            return Err(io_err(format!("staging missing {name}")));
        }
    }
    for name in EVO_OUTPUT_FILES {
        let from = staging.join(name);
        let to = dest.join(name);
        if to.exists() {
            fs::remove_file(&to)?;
        }
        if fs::rename(&from, &to).is_err() {
            fs::copy(&from, &to)?;
            fs::remove_file(&from)?;
        }
    }
    Ok(())
}

fn io_err(msg: String) -> Box<dyn Error> {
    Box::new(io::Error::new(ErrorKind::InvalidInput, msg))
}

#[cfg(test)]
mod evolve_cli_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

    struct TempTree(PathBuf);
    impl TempTree {
        fn new(label: &str) -> Self {
            let n = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!(
                "har-evo02-{}-{}-{}",
                label,
                std::process::id(),
                n
            ));
            fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
    }
    impl Drop for TempTree {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn req(dir: &Path, n: u32, t: u32, force: bool) -> EvolveRequest {
        EvolveRequest {
            population: n,
            generations: t,
            mutation_sigma: 0.05,
            rng_seed: 137137,
            output_dir: dir.to_path_buf(),
            force,
        }
    }

    #[test]
    fn rejects_population_and_sigma_and_budget() {
        let dir = PathBuf::from("unused");
        assert!(validate_evolve_request(&req(&dir, 1, 5, false)).is_err());
        assert!(validate_evolve_request(&req(&dir, 201, 5, false)).is_err());
        assert!(validate_evolve_request(&EvolveRequest {
            mutation_sigma: 0.0,
            ..req(&dir, 8, 5, false)
        })
        .is_err());
        assert!(validate_evolve_request(&EvolveRequest {
            mutation_sigma: 0.21,
            ..req(&dir, 8, 5, false)
        })
        .is_err());
        assert!(validate_evolve_request(&req(&dir, 8, 101, false)).is_err());
        assert!(validate_evolve_request(&req(&dir, 8, 0, false)).is_ok());
        assert!(validate_evolve_request(&req(&dir, 200, 100, false)).is_ok());
        assert_eq!(
            2u64 * 100 * 31,
            2u64 * u64::from(100u32) * u64::from(30u32.saturating_add(1))
        );
        assert!(2u64 * 200 * 101 <= EVO_MAX_TOTAL_EVALS);
        assert!(2u64 * 200 * 126 > EVO_MAX_TOTAL_EVALS);
    }

    #[test]
    fn output_file_is_rejected() {
        let tmp = TempTree::new("file");
        let file = tmp.0.join("not-a-dir");
        fs::write(&file, b"x").unwrap();
        let err = run_evolutionary_mode(&req(&file, 8, 1, false)).unwrap_err();
        assert!(err.to_string().contains("file"));
    }

    #[test]
    fn refuses_overwrite_without_force_and_leaves_bytes() {
        let tmp = TempTree::new("ow");
        let marker = tmp.0.join("evo01_individuals.csv");
        let foreign = tmp.0.join("notes.txt");
        fs::write(&marker, b"KEEP").unwrap();
        fs::write(&foreign, b"FOREIGN").unwrap();
        let err = run_evolutionary_mode(&req(&tmp.0, 8, 1, false)).unwrap_err();
        assert!(err.to_string().contains("--force"));
        assert_eq!(fs::read(&marker).unwrap(), b"KEEP");
        assert!(!tmp.0.join("evo01_metrics.csv").exists());
        assert!(!tmp.0.join("evo01_summary.json").exists());
        assert_eq!(fs::read_to_string(&foreign).unwrap(), "FOREIGN");
        let summary = run_evolutionary_mode(&req(&tmp.0, 8, 1, true)).unwrap();
        assert_eq!(summary.n, 8);
        let body = fs::read_to_string(&marker).unwrap();
        assert!(body.starts_with("run_id,"));
        assert!(!body.starts_with("KEEP"));
        assert_eq!(fs::read_to_string(&foreign).unwrap(), "FOREIGN");
    }

    #[test]
    fn one_of_three_outputs_blocks_without_force() {
        let tmp = TempTree::new("one");
        fs::write(tmp.0.join("evo01_metrics.csv"), b"KEEP-MET").unwrap();
        let err = run_evolutionary_mode(&req(&tmp.0, 8, 1, false)).unwrap_err();
        assert!(err.to_string().contains("--force"));
        assert_eq!(
            fs::read(tmp.0.join("evo01_metrics.csv")).unwrap(),
            b"KEEP-MET"
        );
        assert!(!tmp.0.join("evo01_individuals.csv").exists());
        assert!(!tmp.0.join("evo01_summary.json").exists());
    }

    #[test]
    fn t_zero_writes_generation_zero_only() {
        let tmp = TempTree::new("t0");
        let summary = run_evolutionary_mode(&req(&tmp.0, 2, 0, false)).unwrap();
        assert_eq!(summary.generations, 1);
        assert_eq!(summary.evaluations_per_arm, 2);
        let metrics = fs::read_to_string(tmp.0.join("evo01_metrics.csv")).unwrap();
        assert_eq!(metrics.lines().count(), 3); // header + 2 arms × generation 0
        assert!(metrics.contains(",selection,0,"));
        assert!(metrics.contains(",control,0,"));
        let inds = fs::read_to_string(tmp.0.join("evo01_individuals.csv")).unwrap();
        assert_eq!(inds.lines().count(), 5); // header + 2 arms × N=2
        for line in inds.lines().skip(1) {
            let cols: Vec<&str> = line.split(',').collect();
            assert_eq!(cols[2], "0");
            assert!(cols[4].is_empty(), "gen 0 has no parent_id");
        }
    }

    #[test]
    fn nested_path_under_a_file_fails() {
        let tmp = TempTree::new("nested");
        let file = tmp.0.join("blocker");
        fs::write(&file, b"x").unwrap();
        let dest = file.join("child");
        assert!(run_evolutionary_mode(&req(&dest, 8, 1, false)).is_err());
    }

    #[test]
    fn cli_matches_runner_primary_bytes() {
        let a = TempTree::new("cli");
        let b = TempTree::new("runner");
        let summary = run_evolutionary_mode(&EvolveRequest {
            population: 100,
            generations: 30,
            mutation_sigma: 0.05,
            rng_seed: 137137,
            output_dir: a.0.clone(),
            force: false,
        })
        .unwrap();
        assert_eq!(summary.evaluations_per_arm, 3100);
        write_evo01_run(
            &b.0,
            EvoConfig {
                n: 100,
                transitions: 30,
                sigma: 0.05,
                base_seed: 137137,
            },
        )
        .unwrap();
        for name in EVO_OUTPUT_FILES {
            assert_eq!(
                fs::read(a.0.join(name)).unwrap(),
                fs::read(b.0.join(name)).unwrap(),
                "{name}"
            );
        }
    }
}
