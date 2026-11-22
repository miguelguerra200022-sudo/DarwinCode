//! Non-Linear Decimal Digit Extractor for Pi
//!
//! Provides canonical decimal digit sequences of Pi and dataset generators for
//! evolutionary discrete circuit synthesis d_n = Phi(n) mod 10.

use crate::types::Value;

/// The first 100 decimal digits of Pi starting with 3 (index 0)
pub const PI_DECIMAL_DIGITS_100: [u8; 100] = [
    3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4,
    6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5, 0, 2, 8, 8, 4, 1, 9, 7,
    1, 6, 9, 3, 9, 9, 3, 7, 5, 1, 0, 5, 8, 2, 0, 9, 7, 4, 9, 4,
    4, 5, 9, 2, 3, 0, 7, 8, 1, 6, 4, 0, 6, 2, 8, 6, 2, 0, 8, 9,
    9, 8, 6, 2, 8, 0, 3, 4, 8, 2, 5, 3, 4, 2, 1, 1, 7, 0, 6, 9,
];

/// Returns the digit of Pi at 0-indexed position (0 -> 3, 1 -> 1, 2 -> 4...)
pub fn get_pi_digit(index: usize) -> Option<u8> {
    PI_DECIMAL_DIGITS_100.get(index).copied()
}

/// Evaluates extraction accuracy over a test slice of digits [0..count]
pub fn evaluate_digit_accuracy<F>(mut extractor: F, count: usize) -> f64
where
    F: FnMut(usize) -> u8,
{
    let limit = count.min(PI_DECIMAL_DIGITS_100.len());
    if limit == 0 {
        return 0.0;
    }
    let mut matches = 0usize;
    for (i, &expected) in PI_DECIMAL_DIGITS_100[0..limit].iter().enumerate() {
        if extractor(i) == expected {
            matches += 1;
        }
    }
    (matches as f64) / (limit as f64)
}

/// Generates dataset for DarwinCode evolutionary synthesis: inputs: [n], expected: d_n
pub fn pi_digit_dataset(count: usize) -> Vec<(Vec<Value>, Value)> {
    let limit = count.min(PI_DECIMAL_DIGITS_100.len());
    PI_DECIMAL_DIGITS_100[0..limit]
        .iter()
        .enumerate()
        .map(|(i, &d)| {
            (
                vec![Value::Float(i as f64)],
                Value::Float(d as f64),
            )
        })
        .collect()
}
