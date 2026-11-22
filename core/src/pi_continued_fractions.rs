//! High-Performance Recurrent Continued Fraction Engine for Pi
//!
//! Evaluates generalized and recurrent continued fractions:
//!   S_k = b_0 + a_1 / (b_1 + a_2 / (b_2 + ... + a_k / b_k))
//! using the stabilized Euler-Wallis forward recurrence algorithm.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinuedFractionStep {
    pub a: f64,
    pub b: f64,
}

/// Evaluates a continued fraction given its partial numerators and denominators
/// Returns (convergent_value, abs_error) relative to target
pub fn evaluate_euler_wallis(
    steps: &[ContinuedFractionStep],
    b0: f64,
    target: f64,
) -> (f64, f64) {
    if steps.is_empty() {
        return (b0, (b0 - target).abs());
    }

    let mut a_prev2 = 1.0f64;
    let mut a_prev1 = b0;
    let mut b_prev2 = 0.0f64;
    let mut b_prev1 = 1.0f64;

    for step in steps {
        let a_curr = step.b * a_prev1 + step.a * a_prev2;
        let b_curr = step.b * b_prev1 + step.a * b_prev2;

        // Scaling to avoid numerical overflow while preserving ratio A/B
        if a_curr.abs() > 1e80 || b_curr.abs() > 1e80 {
            let scale = a_curr.abs().max(b_curr.abs());
            a_prev2 = a_prev1 / scale;
            a_prev1 = a_curr / scale;
            b_prev2 = b_prev1 / scale;
            b_prev1 = b_curr / scale;
        } else {
            a_prev2 = a_prev1;
            a_prev1 = a_curr;
            b_prev2 = b_prev1;
            b_prev1 = b_curr;
        }
    }

    let val = if b_prev1.abs() > 1e-15 {
        a_prev1 / b_prev1
    } else {
        a_prev1
    };

    let err = (val - target).abs();
    (val, err)
}

/// Stern-Brouncker continued fraction for 4/pi:
/// 4/pi = 1 + 1^2 / (2 + 3^2 / (2 + 5^2 / (2 + ...)))
pub fn stern_brouncker_steps(depth: usize) -> (f64, Vec<ContinuedFractionStep>) {
    let b0 = 1.0;
    let steps: Vec<ContinuedFractionStep> = (1..=depth)
        .map(|n| {
            let odd = (2 * n - 1) as f64;
            ContinuedFractionStep {
                a: odd * odd,
                b: 2.0,
            }
        })
        .collect();
    (b0, steps)
}

/// Ramanujan's continued fraction for 4/pi:
/// 4/pi = 1 + 1^2 / (3 + 2^2 / (5 + 3^2 / (7 + ...)))
pub fn ramanujan_pi_cf_steps(depth: usize) -> (f64, Vec<ContinuedFractionStep>) {
    let b0 = 1.0;
    let steps: Vec<ContinuedFractionStep> = (1..=depth)
        .map(|n| {
            let nf = n as f64;
            ContinuedFractionStep {
                a: nf * nf,
                b: 2.0 * nf + 1.0,
            }
        })
        .collect();
    (b0, steps)
}
