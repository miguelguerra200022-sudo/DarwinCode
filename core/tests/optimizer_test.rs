use darwin_core::genome::graph::GenomeGraph;
use darwin_core::optimizer::optimize_graph_constants;
use darwin_core::types::{DataType, OpCode, Value};
use darwin_core::vm::evaluate_graph;

#[test]
fn test_levenberg_marquardt_relativistic_constants() {
    let mut graph = GenomeGraph::new(DataType::Float);

    // Inputs: u, v
    let u = graph.add_input("u", DataType::Float);
    let v = graph.add_input("v", DataType::Float);

    // Initial uncalibrated constants (imperfect values)
    let c0 = graph.add_constant(Value::Float(0.5));
    let c1 = graph.add_constant(Value::Float(0.5));
    let c2 = graph.add_constant(Value::Float(2.0));
    let c3 = graph.add_constant(Value::Float(0.2));

    // Numerator: c0 * u + c1 * v
    let term1 = graph.add_operator(OpCode::Multiply, vec![c0, u], DataType::Float).unwrap();
    let term2 = graph.add_operator(OpCode::Multiply, vec![c1, v], DataType::Float).unwrap();
    let num = graph.add_operator(OpCode::Add, vec![term1, term2], DataType::Float).unwrap();

    // Denominator: c2 + c3 * (u * v)
    let uv = graph.add_operator(OpCode::Multiply, vec![u, v], DataType::Float).unwrap();
    let term3 = graph.add_operator(OpCode::Multiply, vec![c3, uv], DataType::Float).unwrap();
    let den = graph.add_operator(OpCode::Add, vec![c2, term3], DataType::Float).unwrap();

    // Output: num / den
    let out = graph.add_operator(OpCode::Divide, vec![num, den], DataType::Float).unwrap();
    graph.set_output(out).unwrap();

    // Relativistic dataset: w = (u + v) / (1 + u * v)
    let raw_data = vec![
        ([0.1, 0.1], 0.19802),
        ([0.1, 0.2], 0.29412),
        ([0.1, 0.3], 0.38835),
        ([0.2, 0.2], 0.38462),
        ([0.2, 0.4], 0.55556),
        ([0.5, 0.5], 0.80000),
        ([0.6, 0.6], 0.88235),
        ([0.8, 0.8], 0.97561),
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

    // Check initial MSE before optimization
    let mut initial_mse = 0.0;
    for (inp, target) in &dataset {
        let pred = evaluate_graph(&graph, inp, 1000).unwrap().as_float().unwrap();
        let diff = pred - target.as_float().unwrap();
        initial_mse += diff * diff;
    }
    initial_mse /= dataset.len() as f64;
    assert!(initial_mse > 0.01, "Initial MSE should be noticeable: {}", initial_mse);

    // Run continuous optimization (Levenberg-Marquardt)
    let final_mse = optimize_graph_constants(&mut graph, &dataset, 40);

    println!("Initial MSE: {:.6}, Optimized Final MSE: {:.8}", initial_mse, final_mse);
    assert!(final_mse < 1e-4, "Optimized MSE must be < 1e-4, got: {}", final_mse);

    // Verify all points match expected closely
    for (inp, target) in &dataset {
        let pred = evaluate_graph(&graph, inp, 1000).unwrap().as_float().unwrap();
        let diff = (pred - target.as_float().unwrap()).abs();
        assert!(diff < 0.01, "Discrepancy at point: pred={}, target={}, diff={}", pred, target.as_float().unwrap(), diff);
    }
}
