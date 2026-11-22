use darwin_core::genome::GenomeGraph;
use darwin_core::types::{DataType, OpCode, Value};
use darwin_core::verifier::verify_invariants;

#[test]
fn test_genome_creation_and_topological_sort() {
    let mut graph = GenomeGraph::new(DataType::Float);
    let x = graph.add_input("x", DataType::Float);
    let two = graph.add_constant(Value::Float(2.0));
    let mult = graph.add_operator(OpCode::Multiply, vec![x, two], DataType::Float).unwrap();
    let one = graph.add_constant(Value::Float(1.0));
    let add = graph.add_operator(OpCode::Add, vec![mult, one], DataType::Float).unwrap();
    graph.set_output(add).unwrap();

    assert!(graph.is_valid());
    assert_eq!(graph.node_count(), 5);
    assert!(verify_invariants(&graph).is_ok());

    let sorted = graph.topological_sort().unwrap();
    assert_eq!(sorted.len(), 5);
    // Verificar que los padres aparezcan antes que los hijos en el orden topológico
    let pos_mult = sorted.iter().position(|&id| id == mult).unwrap();
    let pos_add = sorted.iter().position(|&id| id == add).unwrap();
    assert!(pos_mult < pos_add);
}

#[test]
fn test_type_mismatch_rejected() {
    let mut graph = GenomeGraph::new(DataType::Float);
    let b = graph.add_constant(Value::Boolean(true));
    let f = graph.add_constant(Value::Float(3.0));

    // Intentar sumar un Boolean y un Float debe fallar
    let res = graph.add_operator(OpCode::Add, vec![b, f], DataType::Float);
    assert!(res.is_err());
}
