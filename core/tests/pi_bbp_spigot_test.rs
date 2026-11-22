use darwin_core::pi_bbp_spigot::{
    evaluate_digit_accuracy, get_pi_digit, pi_digit_dataset, PI_DECIMAL_DIGITS_100,
};

#[test]
fn test_first_digits_of_pi() {
    assert_eq!(PI_DECIMAL_DIGITS_100[0], 3);
    assert_eq!(PI_DECIMAL_DIGITS_100[1], 1);
    assert_eq!(PI_DECIMAL_DIGITS_100[2], 4);
    assert_eq!(PI_DECIMAL_DIGITS_100[3], 1);
    assert_eq!(PI_DECIMAL_DIGITS_100[4], 5);
    assert_eq!(PI_DECIMAL_DIGITS_100[5], 9);
    assert_eq!(PI_DECIMAL_DIGITS_100[6], 2);
    assert_eq!(PI_DECIMAL_DIGITS_100[7], 6);
    assert_eq!(PI_DECIMAL_DIGITS_100[8], 5);
    assert_eq!(PI_DECIMAL_DIGITS_100[9], 3);
    assert_eq!(get_pi_digit(0), Some(3));
    assert_eq!(get_pi_digit(99), Some(7));
    assert_eq!(get_pi_digit(999), Some(8));
    assert_eq!(get_pi_digit(1000), None);
}

#[test]
fn test_dataset_generation() {
    let ds = pi_digit_dataset(20);
    assert_eq!(ds.len(), 20);
    assert_eq!(ds[0].0[0].as_float(), Some(0.0));
    assert_eq!(ds[0].1.as_float(), Some(3.0));
    assert_eq!(ds[2].0[0].as_float(), Some(2.0));
    assert_eq!(ds[2].1.as_float(), Some(4.0));
}

#[test]
fn test_extractor_accuracy() {
    // Perfect extractor matches all digits
    let acc = evaluate_digit_accuracy(|n| PI_DECIMAL_DIGITS_100[n], 50);
    assert_eq!(acc, 1.0);

    // Wrong extractor
    let wrong = evaluate_digit_accuracy(|_| 0, 50);
    assert!(wrong < 0.2);
}

#[test]
fn test_rabinowitz_wagon_integer_spigot() {
    use darwin_core::pi_bbp_spigot::rabinowitz_wagon_spigot;
    let digits = rabinowitz_wagon_spigot(50);
    assert_eq!(digits.len(), 50);
    for i in 0..50 {
        assert_eq!(digits[i], PI_DECIMAL_DIGITS_100[i], "Mismatch at digit {}", i);
    }
}

#[test]
fn test_master_pi_seed_genomes_validity() {
    use darwin_core::pi_bbp_spigot::master_pi_seed_genomes;
    use darwin_core::verifier::verify_invariants;

    let seeds = master_pi_seed_genomes();
    assert!(!seeds.is_empty());
    for seed in &seeds {
        assert!(verify_invariants(seed).is_ok());
    }
}
