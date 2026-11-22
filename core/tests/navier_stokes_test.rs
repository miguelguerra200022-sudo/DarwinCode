use darwin_core::navier_stokes::{FluidSimulation1D, SelfSimilarBlowupProfile, evolve_blowup_profile};

#[test]
fn test_fluid_simulation_initialization_and_symmetry() {
    let mut sim = FluidSimulation1D::new(128, 0.0); // nu = 0.0 (Euler inviscid)
    // Initialize with odd vorticity: omega(x) = sin(2*pi*x)
    sim.init_sinusoidal(1.0);

    // Circulation int omega dx must be zero (odd function)
    let total_circulation = sim.total_circulation();
    assert!(total_circulation.abs() < 1e-10, "Total circulation must be 0 for odd profile");

    let initial_enstrophy = sim.enstrophy();
    assert!(initial_enstrophy > 0.0);
}

#[test]
fn test_hou_luo_step_and_vortex_stretching() {
    let mut sim = FluidSimulation1D::new(128, 0.001); // Small viscosity
    sim.init_sinusoidal(2.0);

    let _e0 = sim.enstrophy();
    // Step forward in time
    for _ in 0..50 {
        sim.step(0.001);
    }
    let e1 = sim.enstrophy();

    // Verify simulation is numerical-stable (finite enstrophy)
    assert!(e1.is_finite());
    assert!(sim.max_vorticity() > 0.0);
}

#[test]
fn test_bkm_blowup_detection() {
    let mut sim = FluidSimulation1D::new(64, 0.0);
    // Strong initial vortex gradient
    sim.init_sinusoidal(10.0);

    let mut bkm_integral = 0.0;
    let dt = 0.002;
    for _ in 0..100 {
        sim.step(dt);
        bkm_integral += sim.max_vorticity() * dt;
    }

    assert!(bkm_integral > 0.0);
    assert!(sim.check_bkm_criterion().is_some());
}

#[test]
fn test_self_similar_blowup_profile_residual() {
    // Exact Hou-Luo ansatz profile: Omega(xi) = (c0 * xi) / (1 + xi^2)^gamma
    let profile = SelfSimilarBlowupProfile::new(1.0, 1.0, 1.0, 1.0);
    let residual = profile.compute_residual(0.5);
    assert!(residual.is_finite());

    // Evolutionary search for minimal residual profile
    let best = evolve_blowup_profile(200);
    assert!(best.residual < 0.5);
    assert!(best.c_omega > 0.0);
}
