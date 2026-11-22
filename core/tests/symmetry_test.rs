use darwin_core::symmetry::{detect_symmetries, augment_dataset_with_symmetries};
use darwin_core::types::Value;

#[test]
fn test_symmetry_detection_relativistic_data() {
    // Relativistic dataset with inputs [u, v]
    let raw_data = vec![
        ([0.1, 0.2], 0.29412),
        ([0.2, 0.1], 0.29412),
        ([0.1, 0.4], 0.48077),
        ([0.4, 0.1], 0.48077),
        ([0.2, 0.4], 0.55556),
        ([0.4, 0.2], 0.55556),
        ([0.5, 0.5], 0.80000),
    ];

    let dataset_inputs: Vec<Vec<Value>> = raw_data
        .iter()
        .map(|(inp, _)| vec![Value::Float(inp[0]), Value::Float(inp[1])])
        .collect();

    let dataset: Vec<(&[Value], Value)> = dataset_inputs
        .iter()
        .zip(raw_data.iter())
        .map(|(inp_vec, (_, target))| (inp_vec.as_slice(), Value::Float(*target)))
        .collect();

    let report = detect_symmetries(&dataset, 2, 1e-4);

    // Assert permutation symmetry between input 0 and input 1
    assert!(report.is_symmetric_pair(0, 1), "Should detect permutation symmetry between u and v");

    // Augment dataset with basis invariants (S = u + v, P = u * v)
    let (augmented_data, names) = augment_dataset_with_symmetries(&dataset, &["u", "v"], &report);

    assert!(names.contains(&"u_plus_v".to_string()));
    assert!(names.contains(&"u_mul_v".to_string()));

    // For first sample: u=0.1, v=0.2 => u_plus_v=0.3, u_mul_v=0.02
    let first_row = &augmented_data[0].0;
    assert_eq!(first_row.len(), 4); // u, v, u_plus_v, u_mul_v
    assert!((first_row[2].as_float().unwrap() - 0.3).abs() < 1e-5);
    assert!((first_row[3].as_float().unwrap() - 0.02).abs() < 1e-5);
}

#[test]
fn test_parity_detection_chaotic_potential() {
    // Grid with x in [-2, 2], y in [-1, 1], V = x^2 y + 0.5(x^2 + y^2)
    // Parity: V(-x, y) = V(x, y) => x is even
    let raw_data = vec![
        ([-2.0, 1.0], 6.5),
        ([2.0, 1.0], 6.5),
        ([-1.0, 1.0], 2.0),
        ([1.0, 1.0], 2.0),
        ([-2.0, 0.0], 2.0),
        ([2.0, 0.0], 2.0),
        ([-1.0, 0.0], 0.5),
        ([1.0, 0.0], 0.5),
    ];

    let dataset_inputs: Vec<Vec<Value>> = raw_data
        .iter()
        .map(|(inp, _)| vec![Value::Float(inp[0]), Value::Float(inp[1])])
        .collect();

    let dataset: Vec<(&[Value], Value)> = dataset_inputs
        .iter()
        .zip(raw_data.iter())
        .map(|(inp_vec, (_, target))| (inp_vec.as_slice(), Value::Float(*target)))
        .collect();

    let report = detect_symmetries(&dataset, 2, 1e-4);

    assert!(report.is_even_variable(0), "Variable x (index 0) must be detected as even");
    assert!(!report.is_even_variable(1), "Variable y (index 1) is NOT even due to cubic x^2 y term");

    let (augmented_data, names) = augment_dataset_with_symmetries(&dataset, &["x", "y"], &report);
    assert!(names.contains(&"sq_x".to_string()));

    let first_row = &augmented_data[0].0;
    // x=-2.0, y=1.0 => sq_x=4.0
    assert!((first_row[2].as_float().unwrap() - 4.0).abs() < 1e-5);
}
