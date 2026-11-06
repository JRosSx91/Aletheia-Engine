use rand::rngs::{StdRng, ThreadRng};
use rand::{RngCore, SeedableRng};

/// RNG for Harmonic-137 scanners.
///
/// * `Some(seed)` — `rand::rngs::StdRng` (`ChaCha12` in `rand` 0.8), seeded
///   with [`SeedableRng::seed_from_u64`]. One generator per CLI invocation.
/// * `None` — `thread_rng()`, same stochastic source as the historical CLI.
///
/// Do not create additional RNGs inside sampling loops.
pub enum HarmonicRng {
    Seeded(Box<StdRng>),
    Thread(ThreadRng),
}

impl HarmonicRng {
    pub fn new(seed: Option<u64>) -> Self {
        match seed {
            Some(seed) => HarmonicRng::Seeded(Box::new(StdRng::seed_from_u64(seed))),
            None => HarmonicRng::Thread(rand::thread_rng()),
        }
    }
}

impl RngCore for HarmonicRng {
    fn next_u32(&mut self) -> u32 {
        match self {
            HarmonicRng::Seeded(rng) => rng.next_u32(),
            HarmonicRng::Thread(rng) => rng.next_u32(),
        }
    }

    fn next_u64(&mut self) -> u64 {
        match self {
            HarmonicRng::Seeded(rng) => rng.next_u64(),
            HarmonicRng::Thread(rng) => rng.next_u64(),
        }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        match self {
            HarmonicRng::Seeded(rng) => rng.fill_bytes(dest),
            HarmonicRng::Thread(rng) => rng.fill_bytes(dest),
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        match self {
            HarmonicRng::Seeded(rng) => rng.try_fill_bytes(dest),
            HarmonicRng::Thread(rng) => rng.try_fill_bytes(dest),
        }
    }
}
