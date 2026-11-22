use darwin_core::pi_galperin_billiards::simulate_galperin;

#[test]
fn test_galperin_mass_ratio_1() {
    // M/m = 1 (100^0) -> exactly 3 collisions (first digit of pi)
    let res = simulate_galperin(1.0, 1000);
    assert_eq!(res.collisions, 3);
    assert_eq!(res.estimated_pi, 3.0);
    assert!(res.energy_error < 1e-12);
}

#[test]
fn test_galperin_mass_ratio_100() {
    // M/m = 100 (100^1) -> exactly 31 collisions (first two digits 3.1)
    let res = simulate_galperin(100.0, 1000);
    assert_eq!(res.collisions, 31);
    assert_eq!(res.estimated_pi, 3.1);
    assert!(res.energy_error < 1e-12);
}

#[test]
fn test_galperin_mass_ratio_10000() {
    // M/m = 10000 (100^2) -> exactly 314 collisions (first three digits 3.14)
    let res = simulate_galperin(10000.0, 1000);
    assert_eq!(res.collisions, 314);
    assert_eq!(res.estimated_pi, 3.14);
    assert!(res.energy_error < 1e-12);
}

#[test]
fn test_energy_conservation_and_safety() {
    let res = simulate_galperin(1000000.0, 5000);
    // 100^3 -> 3141 collisions
    assert_eq!(res.collisions, 3141);
    assert_eq!(res.estimated_pi, 3.141);
    assert!(res.energy_error < 1e-10);
}
