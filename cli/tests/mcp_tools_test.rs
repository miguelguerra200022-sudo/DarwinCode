use darwin_cli::mcp_tools::{
    execute_evolve_voxels, execute_get_status, execute_safe_eval, execute_synthesize,
    DataPoint, SafeEvalParams, SynthesizeParams, VoxelParams,
};

#[test]
fn test_synthesize_parabola() {
    let dataset = vec![
        DataPoint { inputs: vec![-2.0], expected: 5.0 },
        DataPoint { inputs: vec![-1.0], expected: 2.0 },
        DataPoint { inputs: vec![0.0], expected: 1.0 },
        DataPoint { inputs: vec![1.0], expected: 2.0 },
        DataPoint { inputs: vec![2.0], expected: 5.0 },
    ];

    let params = SynthesizeParams {
        dataset,
        input_names: Some(vec!["x".to_string()]),
        max_generations: Some(200),
        target_accuracy: Some(0.999),
        population_size: Some(100),
        enable_sota: Some(true),
        enable_constant_optimization: Some(true),
        enable_symmetry_prepass: Some(true),
        allowed_ops: None,
    };

    let result = execute_synthesize(params).expect("Synthesis failed");
    assert!(result.accuracy >= 95.0, "Expected accuracy >= 95%, got {}", result.accuracy);
    assert!(!result.formula.is_empty());
    assert!(result.node_count > 0);
}

#[test]
fn test_safe_eval() {
    let params = SafeEvalParams {
        dataset: vec![
            DataPoint { inputs: vec![3.0], expected: 10.0 }
        ],
        expression: None,
        genome: None,
        gas_limit: Some(500),
    };

    let result = execute_safe_eval(params).expect("Safe eval failed");
    assert!(result.evaluated_points >= 1);
}

#[test]
fn test_evolve_voxels() {
    let params = VoxelParams {
        floors: Some(3),
        style: Some("tower".to_string()),
        max_blocks: Some(100),
    };

    let result = execute_evolve_voxels(params).expect("Voxel generation failed");
    assert!(!result.voxels.is_empty());
    assert!(result.total_blocks > 0);
    assert_eq!(result.dimensions.len(), 3);
}

#[test]
fn test_evolve_dog_voxels() {
    let params = VoxelParams {
        floors: None,
        style: Some("dog".to_string()),
        max_blocks: Some(300),
    };

    let result = execute_evolve_voxels(params).expect("Dog voxel generation failed");
    assert!(!result.voxels.is_empty());
    assert!(result.total_blocks >= 100);
    assert_eq!(result.dimensions, [16, 11, 5]);
    assert!(result.markdown.contains("Golden Retriever"));
    assert!(result.ascii_representation.contains("Vista Lateral"));
}

#[test]
fn test_get_status() {
    let status = execute_get_status();
    assert_eq!(status.name, "darwin-code");
    assert!(status.version.contains("1.1.0") || status.version.contains("1.0.0"));
    assert_eq!(status.tools.len(), 10);
}

#[test]
fn test_synthesize_rejects_single_point_dataset() {
    let params = SynthesizeParams {
        dataset: vec![DataPoint { inputs: vec![1.0], expected: 2.0 }],
        input_names: Some(vec!["x".to_string()]),
        max_generations: Some(10),
        target_accuracy: Some(0.99),
        population_size: Some(50),
        enable_sota: Some(false),
        enable_constant_optimization: Some(false),
        enable_symmetry_prepass: Some(false),
        allowed_ops: None,
    };

    let err = execute_synthesize(params).expect_err("Should reject single point dataset");
    assert!(err.contains("al menos 2 puntos"));
}

#[test]
fn test_optimize_code_requires_dataset() {
    let params = darwin_cli::mcp_tools::OptimizeParams {
        expression: Some("((x * x) + (2.0 - 1.0))".to_string()),
        dataset: None,
        input_names: Some(vec!["x".to_string()]),
        max_generations: Some(10),
    };

    let err = darwin_cli::mcp_tools::execute_optimize_code(params).expect_err("Should require dataset");
    assert!(err.contains("dataset"));
}

#[test]
fn test_ramanujan_mcp_solve() {
    let params = darwin_cli::mcp_tools::RamanujanParams {
        constant: Some("e".to_string()),
        max_depth: Some(15),
    };
    let res = darwin_cli::mcp_tools::execute_ramanujan_solve(params).expect("Ramanujan solve failed");
    assert!(res.matching_digits >= 12);
    assert!((res.evaluated_value - std::f64::consts::E).abs() < 1e-12);
}

#[test]
fn test_srsd_mcp_solve() {
    let params = darwin_cli::mcp_tools::SrsdParams {
        num_samples: Some(40),
        noise_level: Some(0.0),
    };
    let res = darwin_cli::mcp_tools::execute_srsd_hard_solve(params).expect("SRSD solve failed");
    assert_eq!(res.normalized_edit_distance, 0.0);
    assert_eq!(res.pruned_dummy_features.len(), 4);
    assert!(res.r2_score > 0.9999);
}

#[test]
fn test_oeis_mcp_discover() {
    let params = darwin_cli::mcp_tools::OeisParams {
        sequence_id: Some("A000108".to_string()),
        num_training_terms: Some(6),
    };
    let res = darwin_cli::mcp_tools::execute_oeis_discover(params).expect("OEIS discover failed");
    assert_eq!(res.accuracy, 1.0);
    assert!(res.discovered_recurrence.unwrap().contains("4*n"));
}

#[test]
fn test_ramanujan_zeta5_and_gamma() {
    // Zeta(5)
    let p_zeta5 = darwin_cli::mcp_tools::RamanujanParams {
        constant: Some("zeta5".to_string()),
        max_depth: Some(20),
    };
    let res_z5 = darwin_cli::mcp_tools::execute_ramanujan_solve(p_zeta5).expect("Zeta5 solve failed");
    assert!(res_z5.matching_digits >= 12);
    assert!((res_z5.evaluated_value - darwin_core::ramanujan::CONST_ZETA_5).abs() < 1e-12);

    // Gamma
    let p_gamma = darwin_cli::mcp_tools::RamanujanParams {
        constant: Some("gamma".to_string()),
        max_depth: Some(20),
    };
    let res_gam = darwin_cli::mcp_tools::execute_ramanujan_solve(p_gamma).expect("Gamma solve failed");
    assert!(res_gam.matching_digits >= 12);
    assert!((res_gam.evaluated_value - darwin_core::ramanujan::CONST_GAMMA).abs() < 1e-12);
}

#[test]
fn test_oeis_lenstra_a006459_and_snowflake_a161330() {
    // Lenstra A006459
    let p_lenstra = darwin_cli::mcp_tools::OeisParams {
        sequence_id: Some("A006459".to_string()),
        num_training_terms: None,
    };
    let res_len = darwin_cli::mcp_tools::execute_oeis_discover(p_lenstra).expect("Lenstra discover failed");
    assert!(res_len.holonomic_obstruction.is_some());
    let obs = res_len.holonomic_obstruction.unwrap();
    assert!(obs.is_obstructed);
    assert_eq!(obs.nullspace_dimension, 0);
    assert!((obs.asymptotic_ratio_limit - std::f64::consts::SQRT_2).abs() < 0.06);

    // Snowflake A161330
    let p_snow = darwin_cli::mcp_tools::OeisParams {
        sequence_id: Some("A161330".to_string()),
        num_training_terms: None,
    };
    let res_snow = darwin_cli::mcp_tools::execute_oeis_discover(p_snow).expect("Snowflake discover failed");
    assert!(res_snow.modular_spectrum.is_some());
    let spec = res_snow.modular_spectrum.unwrap();
    assert!(spec.is_strictly_modular);
    assert_eq!(spec.base_modulus, 6);
}

#[test]
fn test_mcp_bsd_analysis() {
    let params = darwin_cli::mcp_tools::BsdParams {
        curve_a: Some(-1),
        curve_b: Some(0),
    };
    let res = darwin_cli::mcp_tools::execute_bsd_analysis(params).expect("BSD analysis failed");
    assert_eq!(res.discriminant, 64);
    assert_eq!(res.predicted_rank_parity, 1);
    assert!(!res.fourier_traces.is_empty());
}

#[test]
fn test_mcp_navier_stokes_blowup() {
    let params = darwin_cli::mcp_tools::NavierStokesParams {
        grid_size: Some(64),
        viscosity: Some(0.0),
        steps: Some(50),
        dt: Some(0.001),
    };
    let res = darwin_cli::mcp_tools::execute_navier_stokes_blowup(params).expect("Navier-Stokes blowup failed");
    assert!(res.final_enstrophy > 0.0);
    assert!(res.self_similar_residual < 0.5);
    assert!(!res.history.is_empty());
}


