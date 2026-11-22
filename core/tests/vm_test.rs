use darwin_core::genome::GenomeGraph;
use darwin_core::types::{DataType, OpCode, Value};
use darwin_core::vm::{evaluate_graph, VmError};

#[test]
fn test_vm_arithmetic_computation() {
    // f(x) = (x * 2.0) + 1.0
    let mut graph = GenomeGraph::new(DataType::Float);
    let x = graph.add_input("x", DataType::Float);
    let two = graph.add_constant(Value::Float(2.0));
    let mult = graph.add_operator(OpCode::Multiply, vec![x, two], DataType::Float).unwrap();
    let one = graph.add_constant(Value::Float(1.0));
    let add = graph.add_operator(OpCode::Add, vec![mult, one], DataType::Float).unwrap();
    graph.set_output(add).unwrap();

    let output = evaluate_graph(&graph, &[Value::Float(4.0)], 100).unwrap();
    assert_eq!(output, Value::Float(9.0));
}

#[test]
fn test_vm_gas_exhaustion_terminates_gracefully() {
    let mut graph = GenomeGraph::new(DataType::Float);
    let x = graph.add_input("x", DataType::Float);
    let two = graph.add_constant(Value::Float(2.0));
    let mult = graph.add_operator(OpCode::Multiply, vec![x, two], DataType::Float).unwrap();
    graph.set_output(mult).unwrap();

    // Con gas_limit insuficiente (1 instrucción)
    let res = evaluate_graph(&graph, &[Value::Float(4.0)], 1);
    assert_eq!(res, Err(VmError::GasExhausted));
}

#[test]
fn test_vm_branching_logic() {
    // if x < 5.0 then 100.0 else 0.0
    let mut graph = GenomeGraph::new(DataType::Float);
    let x = graph.add_input("x", DataType::Float);
    let five = graph.add_constant(Value::Float(5.0));
    let cond = graph.add_operator(OpCode::LessThan, vec![x, five], DataType::Boolean).unwrap();
    let val_true = graph.add_constant(Value::Float(100.0));
    let val_false = graph.add_constant(Value::Float(0.0));
    let if_node = graph.add_operator(OpCode::IfElse, vec![cond, val_true, val_false], DataType::Float).unwrap();
    graph.set_output(if_node).unwrap();

    let out1 = evaluate_graph(&graph, &[Value::Float(3.0)], 100).unwrap();
    assert_eq!(out1, Value::Float(100.0));

    let out2 = evaluate_graph(&graph, &[Value::Float(8.0)], 100).unwrap();
    assert_eq!(out2, Value::Float(0.0));
}
