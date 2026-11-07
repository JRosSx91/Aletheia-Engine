//! Historical SI fine-structure constant and diagnostic traces.
//!
//! `si_alpha` is the expression used by `AdvancedPhysicsEngine::new`.
//! It is not a running coupling and does not accept variant \(c\), \(\hbar\), or \(\varepsilon_0\).

use crate::physics::constants::{C, EPSILON_0, H_BAR};
use std::f64::consts::PI;

/// Low-energy QED α with global SI vacuum constants: \(e^2 / (4\pi\varepsilon_0\hbar c)\).
///
/// Operand order matches the historical engine line. Do not reorder.
pub fn si_alpha(e: f64) -> f64 {
    e.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C)
}

/// Vacuum constants for the MKSA α / Coulomb pair. Default is historical SI.
/// Charge `e` stays on `CosmicLaw`; this type does not replace it.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmParameters {
    pub epsilon_0: f64,
    pub hbar: f64,
    pub c: f64,
}

#[cfg(test)]
impl EmParameters {
    pub fn si() -> Self {
        Self {
            epsilon_0: EPSILON_0,
            hbar: H_BAR,
            c: C,
        }
    }

    /// Scale ε₀ only. Pair with `e' = e √λ` to keep α and `e²/ε₀` invariant. Leaves ħ and c unchanged.
    pub fn with_scaled_epsilon0(self, lambda: f64) -> Self {
        Self {
            epsilon_0: self.epsilon_0 * lambda,
            ..self
        }
    }
}

/// α reconstructed from construction-time electromagnetic parameters.
///
/// Same algebra as `si_alpha`, but the vacuum constants are arguments.
/// When those arguments are the global SI values, the result must match `si_alpha(e)`.
#[cfg(test)]
pub fn alpha_from_em_parameters(e: f64, epsilon_0: f64, hbar: f64, c: f64) -> f64 {
    e.powi(2) / (4.0 * PI * epsilon_0 * hbar * c)
}

/// Three-legged α diagnostic. Missing legs stay `None`; they are never filled from another leg.
#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub struct AlphaTrace {
    /// Value requested by the experiment (e.g. \(1/N\)). `None` if the mode has no target.
    pub alpha_target: Option<f64>,
    /// \(e^2/(4\pi\varepsilon_0^\mathrm{var}\hbar^\mathrm{var}c^\mathrm{var})\). `None` if variants were discarded.
    pub alpha_derived: Option<f64>,
    /// Value the historical engine evaluates: `si_alpha(e)`.
    pub alpha_engine: f64,
}

#[cfg(test)]
impl AlphaTrace {
    /// Map-like sampling: no target, no construction variants.
    pub fn engine_only(e: f64) -> Self {
        Self {
            alpha_target: None,
            alpha_derived: None,
            alpha_engine: si_alpha(e),
        }
    }

    /// Target known but construction vacuum constants were not retained (library path).
    pub fn target_without_variants(alpha_target: f64, e: f64) -> Self {
        Self {
            alpha_target: Some(alpha_target),
            alpha_derived: None,
            alpha_engine: si_alpha(e),
        }
    }

    /// Full construction record (alpha-fixed path).
    pub fn from_variants(
        alpha_target: f64,
        e: f64,
        epsilon0_variant: f64,
        hbar_variant: f64,
        c_variant: f64,
    ) -> Self {
        Self {
            alpha_target: Some(alpha_target),
            alpha_derived: Some(alpha_from_em_parameters(
                e,
                epsilon0_variant,
                hbar_variant,
                c_variant,
            )),
            alpha_engine: si_alpha(e),
        }
    }
}
