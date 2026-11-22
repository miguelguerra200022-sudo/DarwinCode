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
    assert_eq!(get_pi_digit(99), Some(9));
    assert_eq!(get_pi_digit(100), None);
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
