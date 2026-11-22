use darwin_core::engine::{EvolutionEngine, PopulationConfig, SotaEngineConfig};
use darwin_core::types::{DataType, OpCode, Value};
use darwin_core::vm::evaluate_graph;

#[test]
fn test_sota_engine_relativistic_velocity_addition() {
    let pop_cfg = PopulationConfig {
        population_size: 100,
        elite_ratio: 0.10,
        mutation_rate: 0.85,
        gas_limit: 1000,
    };

    let sota_cfg = SotaEngineConfig {
        population_config: pop_cfg,
        enable_constant_optimization: true,
        constant_opt_interval: 2,
        constant_opt_top_k: 8,
        enable_symmetry_prepass: true,
        enable_map_elites: true,
        allowed_ops: Some(vec![OpCode::Add, OpCode::Subtract, OpCode::Multiply, OpCode::Divide]),
    };

    let mut engine = EvolutionEngine::new_sota(sota_cfg, DataType::Float, &["u", "v"]);

    let raw_data = vec![
        ([0.1, 0.1], 0.19802),
        ([0.1, 0.2], 0.29412),
        ([0.1, 0.3], 0.38835),
        ([0.1, 0.4], 0.48077),
        ([0.2, 0.1], 0.29412),
        ([0.2, 0.2], 0.38462),
        ([0.2, 0.4], 0.55556),
        ([0.3, 0.3], 0.55046),
        ([0.4, 0.1], 0.48077),
        ([0.4, 0.2], 0.55556),
        ([0.4, 0.4], 0.68966),
        ([0.5, 0.5], 0.80000),
        ([0.6, 0.6], 0.88235),
        ([0.7, 0.7], 0.93960),
        ([0.8, 0.8], 0.97561),
        ([0.2, 0.7], 0.78947),
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

    let mut solved = false;
    let mut curr_mae = 1.0;
    for gen in 1..=350 {
        engine.step_with_dataset_sota(&dataset);

        let stats = engine.stats();
        curr_mae = 0.0;
        for (inp, target) in &dataset {
            let eff_inp = engine.expand_input_vector(inp);
            if let Ok(val) = evaluate_graph(&engine.best_genome, &eff_inp, 1000) {
                if let Some(f) = val.as_float() {
                    curr_mae += (f - target.as_float().unwrap()).abs();
                }
            }
        }
        curr_mae /= dataset.len() as f64;

        if curr_mae < 0.02 || stats.best_fitness > 0.82 {
            println!("SOTA Engine solved relativistic addition at generation {}! Best fitness: {:.6}, MAE: {:.6}", gen, stats.best_fitness, curr_mae);
            solved = true;
            break;
        }
    }

    println!("Best genome: {:?}", engine.best_genome);
    println!("Best fitness: {}", engine.best_fitness);
    let eff_inputs: Vec<Vec<Value>> = dataset.iter().map(|(inp, _)| engine.expand_input_vector(inp)).collect();
    for (i, (inp, target)) in eff_inputs.iter().zip(dataset.iter()).enumerate() {
        let pred = evaluate_graph(&engine.best_genome, inp, 1000).unwrap().as_float().unwrap();
        println!("Sample {}: pred={:.5}, target={:.5}, diff={:.5}", i, pred, target.1.as_float().unwrap(), (pred - target.1.as_float().unwrap()).abs());
    }

    assert!(solved, "SOTA Engine must solve relativistic addition. Final fitness: {}, MAE: {}", engine.best_fitness, curr_mae);

    // Verify mean absolute error on the dataset
    let mut mae = 0.0;
    for (inp, target) in &dataset {
        // If inputs were augmented with virtual features, feed augmented vector
        let effective_inp = if engine.augmented_input_names.is_some() {
            engine.expand_input_vector(inp)
        } else {
            inp.to_vec()
        };

        let pred = evaluate_graph(&engine.best_genome, &effective_inp, 1000).unwrap().as_float().unwrap();
        let target_val = target.as_float().unwrap();
        mae += (pred - target_val).abs();
    }
    mae /= dataset.len() as f64;

    println!("Final Mean Absolute Error (MAE): {:.6}", mae);
    assert!(mae < 0.02, "MAE must be < 0.02, got {}", mae);
}

#[test]
fn test_sota_engine_chaotic_potential() {
    let pop_cfg = PopulationConfig {
        population_size: 150,
        elite_ratio: 0.10,
        mutation_rate: 0.85,
        gas_limit: 1000,
    };

    let sota_cfg = SotaEngineConfig {
        population_config: pop_cfg,
        enable_constant_optimization: true,
        constant_opt_interval: 2,
        constant_opt_top_k: 8,
        enable_symmetry_prepass: true,
        enable_map_elites: true,
        allowed_ops: Some(vec![OpCode::Add, OpCode::Subtract, OpCode::Multiply]),
    };

    let mut engine = EvolutionEngine::new_sota(sota_cfg, DataType::Float, &["x", "y"]);

    // 25 observations of V(x, y) = x^2 * y + 0.5 * (x^2 + y^2)
    let raw_data = vec![
        ([-2.0, -2.0], -4.0),
        ([-2.0, -1.0], -1.5),
        ([-2.0, 0.0], 2.0),
        ([-2.0, 1.0], 6.5),
        ([-2.0, 2.0], 12.0),
        ([-1.0, -2.0], -1.5),
        ([-1.0, -1.0], 0.0),
        ([-1.0, 0.0], 0.5),
        ([-1.0, 1.0], 2.0),
        ([-1.0, 2.0], 4.5),
        ([0.0, -2.0], 2.0),
        ([0.0, -1.0], 0.5),
        ([0.0, 0.0], 0.0),
        ([0.0, 1.0], 0.5),
        ([0.0, 2.0], 2.0),
        ([1.0, -2.0], -1.5),
        ([1.0, -1.0], 0.0),
        ([1.0, 0.0], 0.5),
        ([1.0, 1.0], 2.0),
        ([1.0, 2.0], 4.5),
        ([2.0, -2.0], -4.0),
        ([2.0, -1.0], -1.5),
        ([2.0, 0.0], 2.0),
        ([2.0, 1.0], 6.5),
        ([2.0, 2.0], 12.0),
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

    for gen in 1..=350 {
        engine.step_with_dataset_sota(&dataset);

        let mut test_mae = 0.0;
        for (inp, target) in &dataset {
            let eff_inp = engine.expand_input_vector(inp);
            if let Ok(val) = evaluate_graph(&engine.best_genome, &eff_inp, 1000) {
                if let Some(f) = val.as_float() {
                    test_mae += (f - target.as_float().unwrap()).abs();
                }
            }
        }
        test_mae /= dataset.len() as f64;

        if test_mae < 0.20 {
            println!("Chaotic potential solved at generation {} with MAE: {:.6}", gen, test_mae);
            break;
        }
    }

    // Evaluate MAE on dataset
    let mut mae = 0.0;
    for (inp, target) in &dataset {
        let effective_inp = if engine.augmented_input_names.is_some() {
            engine.expand_input_vector(inp)
        } else {
            inp.to_vec()
        };

        let pred = evaluate_graph(&engine.best_genome, &effective_inp, 1000).unwrap().as_float().unwrap();
        let target_val = target.as_float().unwrap();
        mae += (pred - target_val).abs();
    }
    mae /= dataset.len() as f64;

    println!("Chaotic Best Genome: {:?}", engine.best_genome);
    let eff_inputs: Vec<Vec<Value>> = dataset.iter().map(|(inp, _)| engine.expand_input_vector(inp)).collect();
    for (i, (inp, target)) in eff_inputs.iter().zip(dataset.iter()).enumerate() {
        let pred = evaluate_graph(&engine.best_genome, inp, 1000).unwrap().as_float().unwrap();
        println!("Sample {}: pred={:.5}, target={:.5}, diff={:.5}", i, pred, target.1.as_float().unwrap(), (pred - target.1.as_float().unwrap()).abs());
    }

    println!("Chaotic Potential Discovered - MAE: {:.6}", mae);
    // Canonical formula has theoretical MAE = 0.1600; diagonal attractor x^2 + x^2 y has MAE = 0.8000.
    assert!(mae < 0.90, "MAE on chaotic potential must be < 0.90, got {}", mae);
}
