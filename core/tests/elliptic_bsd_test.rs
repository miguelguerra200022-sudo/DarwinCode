use darwin_core::elliptic_bsd::{EllipticCurve, count_points_fp, fourier_trace_ap, root_number_estimate};

#[test]
fn test_elliptic_curve_discriminant() {
    // E: y^2 = x^3 - x (A = -1, B = 0)
    // Delta = -16 * (4*(-1)^3 + 27*(0)^2) = -16 * (-4) = 64
    let curve = EllipticCurve::new(-1, 0).expect("Valid curve");
    assert_eq!(curve.discriminant(), 64);

    // Singular curve: y^2 = x^3 (A = 0, B = 0) -> Delta = 0 (Must fail)
    assert!(EllipticCurve::new(0, 0).is_err());
}

#[test]
fn test_fourier_traces_ap() {
    // E: y^2 = x^3 - x
    // p = 3: a_3 = 0
    assert_eq!(fourier_trace_ap(-1, 0, 3), 0);
    // p = 5: a_5 = -2
    assert_eq!(fourier_trace_ap(-1, 0, 5), -2);
    // Point count on F_5 is 8
    assert_eq!(count_points_fp(-1, 0, 5), 8);
}

#[test]
fn test_sato_tate_angles() {
    let curve = EllipticCurve::new(-1, 0).unwrap();
    let theta_5 = curve.sato_tate_angle(5);
    // a_5 / (2*sqrt(5)) = -2 / 4.472135... = -0.4472135...
    // theta = arccos(-0.4472135) approx 2.0344 rad
    assert!((theta_5 - 2.03444).abs() < 1e-3);
}

#[test]
fn test_canonical_lmfdb_curves_and_bsd() {
    // Curve 1: y^2 = x^3 - x (rank 0, W(E) = +1)
    let c0 = EllipticCurve::new(-1, 0).unwrap();
    assert_eq!(root_number_estimate(&c0), 1);

    // Curve 2: y^2 = x^3 - 2 (rank 1, rational point (3, 5), W(E) = -1)
    let c1 = EllipticCurve::new(0, -2).unwrap();
    assert_eq!(root_number_estimate(&c1), -1);

    // Parity conjecture: (-1)^r == W(E)
    // For rank 0: (-1)^0 = +1 == W(E)
    // For rank 1: (-1)^1 = -1 == W(E)
}
