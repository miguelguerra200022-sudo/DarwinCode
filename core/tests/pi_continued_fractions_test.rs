use darwin_core::pi_continued_fractions::{
    evaluate_euler_wallis, ramanujan_pi_cf_steps, stern_brouncker_steps, ContinuedFractionStep,
};
use std::f64::consts::PI;

#[test]
fn test_euler_wallis_recurrence_golden_ratio() {
    // 1 + 1/(1 + 1/(1 + ...)) -> Golden Ratio phi = 1.6180339887...
    let steps: Vec<ContinuedFractionStep> = (0..20)
        .map(|_| ContinuedFractionStep { a: 1.0, b: 1.0 })
        .collect();
    let (val, _err) = evaluate_euler_wallis(&steps, 1.0, 1.618033988749895);
    let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
    assert!((val - phi).abs() < 1e-4);
}

#[test]
fn test_stern_brouncker_pi_convergence() {
    // 4 / pi = 1 + 1^2 / (2 + 3^2 / (2 + 5^2 / (2 + ...)))
    let (b0, steps) = stern_brouncker_steps(100);
    // The continued fraction approximates 4 / pi, so pi = 4 / val
    let (cf_val, _err) = evaluate_euler_wallis(&steps, b0, 4.0 / PI);
    let estimated_pi = 4.0 / cf_val;
    assert!((estimated_pi - PI).abs() < 0.05);
}

#[test]
fn test_ramanujan_pi_cf_convergence() {
    // 4 / pi = 1 + 1^2 / (3 + 2^2 / (5 + 3^2 / (7 + ...)))
    let (b0, steps) = ramanujan_pi_cf_steps(50);
    let (cf_val, _err) = evaluate_euler_wallis(&steps, b0, 4.0 / PI);
    let estimated_pi = 4.0 / cf_val;
    assert!((estimated_pi - PI).abs() < 0.02);
}
