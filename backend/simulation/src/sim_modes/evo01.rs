//! HAR-EVO-01 experimental optimizer. Called from the `evolve` CLI wrapper.
//! Fitness is an internal index, not habitability. Motor A only.

use crate::core::models::CosmicLaw;
use crate::physics::alpha::si_alpha;
use crate::physics::constants::{C, EPSILON_0, G_GRAVITATIONAL, H_BAR};
use crate::physics::engine::AdvancedPhysicsEngine;
use crate::sim_modes::validate::create_reference_universe;
use csv::Writer;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::error::Error;
use std::f64::consts::PI;
use std::path::Path;

pub(crate) const EVO01_TOURNAMENT_K: usize = 2;
#[cfg(test)]
pub(crate) const EVO01_BASE_SEED: u64 = 137_137;
#[cfg(test)]
pub(crate) const EVO01_SIGMA: f64 = 0.05;
#[cfg(test)]
pub(crate) const EVO01_PRIMARY_N: u32 = 100;
#[cfg(test)]
pub(crate) const EVO01_PRIMARY_T: u32 = 30;
#[cfg(test)]
pub(crate) const EVO01_SMOKE_N: u32 = 8;
#[cfg(test)]
pub(crate) const EVO01_SMOKE_T: u32 = 5;

pub(crate) const MAP_G_LO: f64 = 6.674e-11;
pub(crate) const MAP_G_HI: f64 = 6.674e-10;
pub(crate) const MAP_E_LO: f64 = 0.5e-19;
pub(crate) const MAP_E_HI: f64 = 2.5e-19;
pub(crate) const ALPHA_S_LO: f64 = 0.05;
pub(crate) const ALPHA_S_HI: f64 = 2.0;

const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
const BORDER_U: f64 = 0.02;
const LN_MOVE_EPS: f64 = 1e-15;

pub(crate) fn alpha_lo() -> f64 {
    si_alpha(MAP_E_LO)
}
pub(crate) fn alpha_hi() -> f64 {
    si_alpha(MAP_E_HI)
}
pub(crate) fn gamma_lo() -> f64 {
    MAP_G_LO / G_GRAVITATIONAL
}
pub(crate) fn gamma_hi() -> f64 {
    MAP_G_HI / G_GRAVITATIONAL
}

/// Inverse of `si_alpha` under global SI vacuum constants.
pub(crate) fn e_from_alpha(alpha: f64) -> f64 {
    (alpha * 4.0 * PI * EPSILON_0 * H_BAR * C).sqrt()
}

pub(crate) fn splitmix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub(crate) fn derive_seed(base: u64, stream: u64) -> u64 {
    splitmix64(base.wrapping_add(stream.wrapping_mul(GOLDEN_GAMMA)))
}

pub(crate) fn stream_seed(base: u64, stream: u64, generation: u32, slot: u32) -> u64 {
    let packed = ((generation as u64) << 32) | (slot as u64);
    derive_seed(derive_seed(base, stream), packed)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Arm {
    Selection,
    Control,
}

impl Arm {
    fn code(self) -> u64 {
        match self {
            Arm::Selection => 0,
            Arm::Control => 1,
        }
    }
    fn name(self) -> &'static str {
        match self {
            Arm::Selection => "selection",
            Arm::Control => "control",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EvalStatus {
    Ok,
    Skipped,
    NonfiniteEval,
}

impl EvalStatus {
    fn as_str(self) -> &'static str {
        match self {
            EvalStatus::Ok => "ok",
            EvalStatus::Skipped => "skipped",
            EvalStatus::NonfiniteEval => "nonfinite_eval",
        }
    }
    fn can_win(self) -> bool {
        matches!(self, EvalStatus::Ok)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Genome {
    pub alpha: f64,
    pub alpha_s: f64,
    pub gamma: f64,
}

impl Genome {
    fn in_box(&self) -> bool {
        in_half_open(self.alpha, alpha_lo(), alpha_hi())
            && in_half_open(self.alpha_s, ALPHA_S_LO, ALPHA_S_HI)
            && in_half_open(self.gamma, gamma_lo(), gamma_hi())
            && self.alpha.is_finite()
            && self.alpha_s.is_finite()
            && self.gamma.is_finite()
    }

    fn log_unit(&self) -> Option<[f64; 3]> {
        Some([
            log_unit(self.alpha, alpha_lo(), alpha_hi())?,
            log_unit(self.alpha_s, ALPHA_S_LO, ALPHA_S_HI)?,
            log_unit(self.gamma, gamma_lo(), gamma_hi())?,
        ])
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Individual {
    pub universe_id: u64,
    pub parent_id: Option<u64>,
    pub slot: u32,
    pub generation: u32,
    pub genome: Genome,
    pub e_si: f64,
    pub g_si: f64,
    pub fitness: f64,
    pub complexity_level: u8,
    pub eval_status: EvalStatus,
    pub mutation_effective: f64,
    pub clamped_genes: String,
    pub discard_reason: String,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct EvoConfig {
    pub n: u32,
    pub transitions: u32,
    pub sigma: f64,
    pub base_seed: u64,
}

impl EvoConfig {
    #[cfg(test)]
    pub(crate) fn smoke() -> Self {
        Self {
            n: EVO01_SMOKE_N,
            transitions: EVO01_SMOKE_T,
            sigma: EVO01_SIGMA,
            base_seed: EVO01_BASE_SEED,
        }
    }
    #[cfg(test)]
    pub(crate) fn primary() -> Self {
        Self {
            n: EVO01_PRIMARY_N,
            transitions: EVO01_PRIMARY_T,
            sigma: EVO01_SIGMA,
            base_seed: EVO01_BASE_SEED,
        }
    }
    pub(crate) fn run_id(&self) -> String {
        format!(
            "evo01-n{}-t{}-base{}",
            self.n, self.transitions, self.base_seed
        )
    }
    fn generations(&self) -> u32 {
        self.transitions + 1
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct EvoSummary {
    pub run_id: String,
    pub n: u32,
    pub transitions: u32,
    pub generations: u32,
    pub sigma: f64,
    pub base_seed: u64,
    pub seed_init: u64,
    pub seed_select_stream: u64,
    pub seed_mutate_stream: u64,
    pub evaluations_per_arm: u64,
    pub last_best_fitness_selection: f64,
    pub last_best_fitness_control: f64,
    pub last_median_fitness_selection: f64,
    pub last_median_fitness_control: f64,
    pub last_frac_border_selection: f64,
    pub last_frac_border_control: f64,
    pub note: String,
}

pub(crate) const INDIVIDUAL_HEADERS: [&str; 17] = [
    "run_id",
    "arm",
    "generation",
    "universe_id",
    "parent_id",
    "slot",
    "alpha",
    "alpha_s",
    "gamma",
    "e_si",
    "g_si",
    "fitness",
    "complexity_level",
    "eval_status",
    "mutation_effective",
    "clamped_genes",
    "discard_reason",
];

pub(crate) const METRIC_HEADERS: [&str; 20] = [
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

fn in_half_open(x: f64, lo: f64, hi: f64) -> bool {
    x.is_finite() && x >= lo && x < hi
}

fn log_unit(theta: f64, lo: f64, hi: f64) -> Option<f64> {
    if !theta.is_finite() || theta <= 0.0 || lo <= 0.0 {
        return None;
    }
    let hi_star = hi.next_down();
    let denom = hi_star.ln() - lo.ln();
    if denom <= 0.0 {
        return None;
    }
    Some(((theta.ln() - lo.ln()) / denom).clamp(0.0, 1.0))
}

pub(crate) fn exclusive_clamp(x: f64, lo: f64, hi: f64) -> (f64, bool) {
    if !x.is_finite() {
        return (x, false);
    }
    let mut y = x;
    let mut clamped = false;
    if y < lo {
        y = lo;
        clamped = true;
    }
    if y >= hi {
        y = hi.next_down();
        clamped = true;
    }
    if y < lo {
        y = lo;
        clamped = true;
    }
    (y, clamped)
}

pub(crate) fn universe_id(arm: Arm, generation: u32, slot: u32) -> u64 {
    (arm.code() << 56) | ((generation as u64) << 28) | (slot as u64)
}

pub(crate) fn control_parent(pop: &[Individual], slot: usize) -> &Individual {
    &pop[slot]
}

pub(crate) fn tournament_winner<'a>(a: &'a Individual, b: &'a Individual) -> &'a Individual {
    match (a.eval_status.can_win(), b.eval_status.can_win()) {
        (true, false) => a,
        (false, true) => b,
        (false, false) => {
            if a.universe_id <= b.universe_id {
                a
            } else {
                b
            }
        }
        (true, true) => match a.fitness.partial_cmp(&b.fitness) {
            Some(std::cmp::Ordering::Greater) => a,
            Some(std::cmp::Ordering::Less) => b,
            _ if a.universe_id <= b.universe_id => a,
            _ => b,
        },
    }
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

fn to_laws(g: &Genome, background: &CosmicLaw) -> Option<CosmicLaw> {
    if !g.in_box() {
        return None;
    }
    let mut laws = background.clone();
    laws.alpha_s = g.alpha_s;
    laws.e = e_from_alpha(g.alpha);
    laws.g = g.gamma * G_GRAVITATIONAL;
    if !in_half_open(laws.e, MAP_E_LO, MAP_E_HI) {
        let (e, _) = exclusive_clamp(laws.e, MAP_E_LO, MAP_E_HI);
        laws.e = e;
    }
    if !in_half_open(laws.g, MAP_G_LO, MAP_G_HI) {
        let (gv, _) = exclusive_clamp(laws.g, MAP_G_LO, MAP_G_HI);
        laws.g = gv;
    }
    Some(laws)
}

fn sample_init_genome<R: Rng>(rng: &mut R) -> Genome {
    let e = rng.gen_range(MAP_E_LO..MAP_E_HI);
    let g = rng.gen_range(MAP_G_LO..MAP_G_HI);
    let alpha_s = rng.gen_range(ALPHA_S_LO..ALPHA_S_HI);
    Genome {
        alpha: si_alpha(e),
        alpha_s,
        gamma: g / G_GRAVITATIONAL,
    }
}

fn normals3(seed: u64) -> [f64; 3] {
    let mut rng = StdRng::seed_from_u64(seed);
    [
        StandardNormal.sample(&mut rng),
        StandardNormal.sample(&mut rng),
        StandardNormal.sample(&mut rng),
    ]
}

fn tournament_indices(seed: u64, n: u32) -> (usize, usize) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut idxs = [0usize; EVO01_TOURNAMENT_K];
    for slot in idxs.iter_mut() {
        *slot = rng.gen_range(0..n) as usize;
    }
    (idxs[0], idxs[1])
}

struct MutOutcome {
    genome: Genome,
    clamped: Vec<&'static str>,
    nonfinite: bool,
    pre_out: [bool; 3],
    effective: f64,
}

fn mutate(parent: &Genome, z: [f64; 3], sigma: f64) -> MutOutcome {
    let raw = [
        parent.alpha * (sigma * z[0]).exp(),
        parent.alpha_s * (sigma * z[1]).exp(),
        parent.gamma * (sigma * z[2]).exp(),
    ];
    let bounds = [
        (alpha_lo(), alpha_hi(), "alpha"),
        (ALPHA_S_LO, ALPHA_S_HI, "alpha_s"),
        (gamma_lo(), gamma_hi(), "gamma"),
    ];
    let mut genes = [0.0; 3];
    let mut clamped = Vec::new();
    let mut nonfinite = false;
    let mut pre_out = [false; 3];
    for i in 0..3 {
        let (lo, hi, name) = bounds[i];
        pre_out[i] = raw[i].is_finite() && (raw[i] < lo || raw[i] >= hi);
        if !raw[i].is_finite() {
            genes[i] = raw[i];
            nonfinite = true;
            continue;
        }
        let (y, c) = exclusive_clamp(raw[i], lo, hi);
        genes[i] = y;
        if c {
            clamped.push(name);
        }
    }
    let genome = Genome {
        alpha: genes[0],
        alpha_s: genes[1],
        gamma: genes[2],
    };
    let moved = [
        gene_moved(parent.alpha, genome.alpha),
        gene_moved(parent.alpha_s, genome.alpha_s),
        gene_moved(parent.gamma, genome.gamma),
    ];
    let effective = moved.iter().filter(|m| **m).count() as f64 / 3.0;
    MutOutcome {
        genome,
        clamped,
        nonfinite,
        pre_out,
        effective,
    }
}

fn gene_moved(old: f64, new: f64) -> bool {
    if !old.is_finite() || !new.is_finite() || old <= 0.0 || new <= 0.0 {
        return old.to_bits() != new.to_bits();
    }
    (new.ln() - old.ln()).abs() > LN_MOVE_EPS
}

fn evaluate_individual(
    arm: Arm,
    generation: u32,
    slot: u32,
    parent_id: Option<u64>,
    genome: Genome,
    background: &CosmicLaw,
    mut_eff: f64,
    clamped: &str,
    skip_nonfinite: bool,
) -> Individual {
    let uid = universe_id(arm, generation, slot);
    if skip_nonfinite || !genome.in_box() {
        return Individual {
            universe_id: uid,
            parent_id,
            slot,
            generation,
            e_si: e_from_alpha(genome.alpha),
            g_si: genome.gamma * G_GRAVITATIONAL,
            genome,
            fitness: f64::NAN,
            complexity_level: 0,
            eval_status: EvalStatus::Skipped,
            mutation_effective: mut_eff,
            clamped_genes: clamped.to_string(),
            discard_reason: "nonfinite_genome".to_string(),
        };
    }
    let laws = to_laws(&genome, background).expect("in-box genome");
    let ev = AdvancedPhysicsEngine::new(laws.clone()).evaluate();
    let fit = ev.heuristic.fitness;
    let status = if fit.is_finite() {
        EvalStatus::Ok
    } else {
        EvalStatus::NonfiniteEval
    };
    let discard = if status == EvalStatus::Ok {
        String::new()
    } else {
        "nonfinite_eval".to_string()
    };
    Individual {
        universe_id: uid,
        parent_id,
        slot,
        generation,
        genome: crate::sim_modes::evo01::Genome {
            alpha: si_alpha(laws.e),
            alpha_s: laws.alpha_s,
            gamma: laws.g / G_GRAVITATIONAL,
        },
        e_si: laws.e,
        g_si: laws.g,
        fitness: if status == EvalStatus::Ok { fit } else { f64::NAN },
        complexity_level: ev.heuristic.complexity_level,
        eval_status: status,
        mutation_effective: mut_eff,
        clamped_genes: clamped.to_string(),
        discard_reason: discard,
    }
}

fn sample_generation_zero(cfg: &EvoConfig, _background: &CosmicLaw) -> Vec<Genome> {
    let mut rng = StdRng::seed_from_u64(derive_seed(cfg.base_seed, 0));
    (0..cfg.n)
        .map(|_| sample_init_genome(&mut rng))
        .collect()
}

fn instantiate_pop(
    _cfg: &EvoConfig,
    arm: Arm,
    genomes: &[Genome],
    background: &CosmicLaw,
) -> Vec<Individual> {
    genomes
        .iter()
        .enumerate()
        .map(|(i, g)| {
            evaluate_individual(
                arm,
                0,
                i as u32,
                None,
                g.clone(),
                background,
                0.0,
                "",
                false,
            )
        })
        .collect()
}

fn step_arm(
    cfg: &EvoConfig,
    arm: Arm,
    prev: &[Individual],
    dest_gen: u32,
    background: &CosmicLaw,
) -> (Vec<Individual>, u32) {
    let n = cfg.n;
    let mut next = Vec::with_capacity(n as usize);
    let mut clamped_gene_count = 0u32;
    for slot in 0..n {
        let z = normals3(stream_seed(cfg.base_seed, 2, dest_gen, slot));
        let parent = match arm {
            Arm::Control => control_parent(prev, slot as usize),
            Arm::Selection => {
                let (ia, ib) = tournament_indices(
                    stream_seed(cfg.base_seed, 1, dest_gen, slot),
                    n,
                );
                tournament_winner(&prev[ia], &prev[ib])
            }
        };
        let outcome = mutate(&parent.genome, z, cfg.sigma);
        clamped_gene_count += outcome.pre_out.iter().filter(|b| **b).count() as u32;
        let clamped = outcome.clamped.join("|");
        next.push(evaluate_individual(
            arm,
            dest_gen,
            slot,
            Some(parent.universe_id),
            outcome.genome,
            background,
            outcome.effective,
            &clamped,
            outcome.nonfinite,
        ));
    }
    (next, clamped_gene_count)
}

#[derive(Clone)]
struct GenStats {
    best: f64,
    median: f64,
    q1: f64,
    q3: f64,
    iqr: f64,
    levels: [f64; 6],
    diversity: f64,
    mean_dist: f64,
    frac_border: f64,
    frac_clamped: f64,
    mut_rate: f64,
    parent_conc: f64,
}

fn percentile_sorted(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let idx = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

pub(crate) fn diversity_of(pop: &[Individual]) -> f64 {
    let units: Vec<[f64; 3]> = pop.iter().filter_map(|i| i.genome.log_unit()).collect();
    let m = units.len();
    if m < 2 {
        return 0.0;
    }
    let mut acc = 0.0;
    let mut n = 0u64;
    for i in 0..m {
        for j in (i + 1)..m {
            let d0 = units[i][0] - units[j][0];
            let d1 = units[i][1] - units[j][1];
            let d2 = units[i][2] - units[j][2];
            acc += (d0 * d0 + d1 * d1 + d2 * d2).sqrt();
            n += 1;
        }
    }
    acc / n as f64
}

fn summarize_generation(pop: &[Individual], clamped_genes: u32) -> GenStats {
    let mut fits: Vec<f64> = pop
        .iter()
        .filter(|i| i.eval_status == EvalStatus::Ok)
        .map(|i| i.fitness)
        .collect();
    fits.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let best = fits.last().copied().unwrap_or(f64::NAN);
    let median = percentile_sorted(&fits, 0.5);
    let q1 = percentile_sorted(&fits, 0.25);
    let q3 = percentile_sorted(&fits, 0.75);
    let n = pop.len() as f64;
    let mut levels = [0.0; 6];
    let mut border = 0.0;
    let mut dist_acc = 0.0;
    let mut dist_n = 0.0;
    let mut mut_acc = 0.0;
    for ind in pop {
        levels[ind.complexity_level as usize] += 1.0 / n;
        mut_acc += ind.mutation_effective;
        if let Some(u) = ind.genome.log_unit() {
            let d = u
                .iter()
                .map(|x| x.min(1.0 - x))
                .fold(f64::INFINITY, f64::min);
            dist_acc += d;
            dist_n += 1.0;
            if u.iter().any(|x| *x < BORDER_U || *x > 1.0 - BORDER_U) {
                border += 1.0;
            }
        }
    }
    let unique_parents = {
        let mut p: Vec<u64> = pop.iter().filter_map(|i| i.parent_id).collect();
        p.sort_unstable();
        p.dedup();
        if pop[0].generation == 0 {
            f64::NAN
        } else {
            p.len() as f64 / n
        }
    };
    GenStats {
        best,
        median,
        q1,
        q3,
        iqr: q3 - q1,
        levels,
        diversity: diversity_of(pop),
        mean_dist: if dist_n > 0.0 { dist_acc / dist_n } else { f64::NAN },
        frac_border: border / n,
        frac_clamped: if pop[0].generation == 0 {
            0.0
        } else {
            clamped_genes as f64 / (n * 3.0)
        },
        mut_rate: if pop[0].generation == 0 {
            0.0
        } else {
            mut_acc / n
        },
        parent_conc: unique_parents,
    }
}

fn write_ind_row(
    wtr: &mut Writer<std::fs::File>,
    run_id: &str,
    arm: Arm,
    ind: &Individual,
) -> Result<(), Box<dyn Error>> {
    wtr.write_record(&[
        run_id.to_string(),
        arm.name().to_string(),
        ind.generation.to_string(),
        ind.universe_id.to_string(),
        ind.parent_id.map(|p| p.to_string()).unwrap_or_default(),
        ind.slot.to_string(),
        csv_f64(ind.genome.alpha),
        csv_f64(ind.genome.alpha_s),
        csv_f64(ind.genome.gamma),
        csv_f64(ind.e_si),
        csv_f64(ind.g_si),
        csv_f64(ind.fitness),
        ind.complexity_level.to_string(),
        ind.eval_status.as_str().to_string(),
        csv_f64(ind.mutation_effective),
        ind.clamped_genes.clone(),
        ind.discard_reason.clone(),
    ])?;
    Ok(())
}

fn write_metric_row(
    wtr: &mut Writer<std::fs::File>,
    run_id: &str,
    arm: Arm,
    generation: u32,
    s: &GenStats,
) -> Result<(), Box<dyn Error>> {
    wtr.write_record(&[
        run_id.to_string(),
        arm.name().to_string(),
        generation.to_string(),
        csv_f64(s.best),
        csv_f64(s.median),
        csv_f64(s.q1),
        csv_f64(s.q3),
        csv_f64(s.iqr),
        csv_f64(s.levels[0]),
        csv_f64(s.levels[1]),
        csv_f64(s.levels[2]),
        csv_f64(s.levels[3]),
        csv_f64(s.levels[4]),
        csv_f64(s.levels[5]),
        csv_f64(s.diversity),
        csv_f64(s.mean_dist),
        csv_f64(s.frac_border),
        csv_f64(s.frac_clamped),
        csv_f64(s.mut_rate),
        csv_f64(s.parent_conc),
    ])?;
    Ok(())
}

/// Full selection-vs-control run. Writes individuals + metrics CSV and summary JSON.
pub(crate) fn write_evo01_run(dir: &Path, cfg: EvoConfig) -> Result<EvoSummary, Box<dyn Error>> {
    std::fs::create_dir_all(dir)?;
    let run_id = cfg.run_id();
    let background = create_reference_universe();
    let genomes0 = sample_generation_zero(&cfg, &background);
    let mut sel = instantiate_pop(&cfg, Arm::Selection, &genomes0, &background);
    let mut ctrl = instantiate_pop(&cfg, Arm::Control, &genomes0, &background);

    let ind_path = dir.join("evo01_individuals.csv");
    let met_path = dir.join("evo01_metrics.csv");
    let mut ind_w = Writer::from_path(&ind_path)?;
    let mut met_w = Writer::from_path(&met_path)?;
    ind_w.write_record(INDIVIDUAL_HEADERS)?;
    met_w.write_record(METRIC_HEADERS)?;

    let mut last_sel = summarize_generation(&sel, 0);
    let mut last_ctrl = summarize_generation(&ctrl, 0);
    for ind in &sel {
        write_ind_row(&mut ind_w, &run_id, Arm::Selection, ind)?;
    }
    for ind in &ctrl {
        write_ind_row(&mut ind_w, &run_id, Arm::Control, ind)?;
    }
    write_metric_row(&mut met_w, &run_id, Arm::Selection, 0, &last_sel)?;
    write_metric_row(&mut met_w, &run_id, Arm::Control, 0, &last_ctrl)?;

    for dest in 1..=cfg.transitions {
        let (n_sel, c_sel) = step_arm(&cfg, Arm::Selection, &sel, dest, &background);
        let (n_ctrl, c_ctrl) = step_arm(&cfg, Arm::Control, &ctrl, dest, &background);
        last_sel = summarize_generation(&n_sel, c_sel);
        last_ctrl = summarize_generation(&n_ctrl, c_ctrl);
        for ind in &n_sel {
            write_ind_row(&mut ind_w, &run_id, Arm::Selection, ind)?;
        }
        for ind in &n_ctrl {
            write_ind_row(&mut ind_w, &run_id, Arm::Control, ind)?;
        }
        write_metric_row(&mut met_w, &run_id, Arm::Selection, dest, &last_sel)?;
        write_metric_row(&mut met_w, &run_id, Arm::Control, dest, &last_ctrl)?;
        sel = n_sel;
        ctrl = n_ctrl;
    }

    ind_w.flush()?;
    met_w.flush()?;

    let summary = EvoSummary {
        run_id,
        n: cfg.n,
        transitions: cfg.transitions,
        generations: cfg.generations(),
        sigma: cfg.sigma,
        base_seed: cfg.base_seed,
        seed_init: derive_seed(cfg.base_seed, 0),
        seed_select_stream: derive_seed(cfg.base_seed, 1),
        seed_mutate_stream: derive_seed(cfg.base_seed, 2),
        evaluations_per_arm: cfg.n as u64 * cfg.generations() as u64,
        last_best_fitness_selection: last_sel.best,
        last_best_fitness_control: last_ctrl.best,
        last_median_fitness_selection: last_sel.median,
        last_median_fitness_control: last_ctrl.median,
        last_frac_border_selection: last_sel.frac_border,
        last_frac_border_control: last_ctrl.frac_border,
        note: "Internal fitness index. Not habitability. Not Smolin selection.".to_string(),
    };
    std::fs::write(
        dir.join("evo01_summary.json"),
        serde_json::to_string_pretty(&summary)?,
    )?;
    Ok(summary)
}

#[cfg(test)]
pub(crate) fn run_both_pops(
    cfg: EvoConfig,
) -> (Vec<Vec<Individual>>, Vec<Vec<Individual>>) {
    let background = create_reference_universe();
    let genomes0 = sample_generation_zero(&cfg, &background);
    let mut sel_hist = vec![instantiate_pop(&cfg, Arm::Selection, &genomes0, &background)];
    let mut ctrl_hist = vec![instantiate_pop(&cfg, Arm::Control, &genomes0, &background)];
    for dest in 1..=cfg.transitions {
        let (n_sel, _) = step_arm(
            &cfg,
            Arm::Selection,
            sel_hist.last().unwrap(),
            dest,
            &background,
        );
        let (n_ctrl, _) = step_arm(
            &cfg,
            Arm::Control,
            ctrl_hist.last().unwrap(),
            dest,
            &background,
        );
        sel_hist.push(n_sel);
        ctrl_hist.push(n_ctrl);
    }
    (sel_hist, ctrl_hist)
}
