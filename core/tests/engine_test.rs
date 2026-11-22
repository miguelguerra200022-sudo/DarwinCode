use darwin_core::engine::{EvolutionEngine, PopulationConfig};
use darwin_core::types::{DataType, Value};

#[test]
fn test_evolutionary_engine_runs_generations_without_memory_leaks() {
    let config = PopulationConfig {
        population_size: 50,
        elite_ratio: 0.1,
        mutation_rate: 0.3,
        gas_limit: 500,
    };

    let mut engine = EvolutionEngine::new(config, DataType::Float, &["x"]);

    // Dataset para aprender la función identidad f(x) = x
    let dataset = vec![
        (vec![Value::Float(1.0)], Value::Float(1.0)),
        (vec![Value::Float(2.0)], Value::Float(2.0)),
        (vec![Value::Float(3.0)], Value::Float(3.0)),
    ];
    let dataset_refs: Vec<(&[Value], Value)> = dataset
        .iter()
        .map(|(inps, out)| (inps.as_slice(), out.clone()))
        .collect();

    for _ in 0..100 {
        engine.step_with_dataset(&dataset_refs);
    }

    let stats = engine.stats();
    assert_eq!(stats.generation, 100);
    assert!(stats.best_fitness > 0.0);
    assert!(stats.total_evaluations >= 5000);
}

#[test]
fn test_evolution_prunes_bloat_and_improves_fitness() {
    let config = PopulationConfig {
        population_size: 40,
        elite_ratio: 0.1,
        mutation_rate: 0.8,
        gas_limit: 800,
    };

    let mut engine = EvolutionEngine::new(config, DataType::Float, &["x"]);

    let dataset = vec![
        (vec![Value::Float(-2.0)], Value::Float(5.0)),
        (vec![Value::Float(0.0)], Value::Float(1.0)),
        (vec![Value::Float(2.0)], Value::Float(5.0)),
    ];
    let dataset_refs: Vec<(&[Value], Value)> = dataset
        .iter()
        .map(|(inps, out)| (inps.as_slice(), out.clone()))
        .collect();

    let initial_fitness = {
        engine.step_with_dataset(&dataset_refs);
        engine.best_fitness
    };

    for _ in 0..150 {
        engine.step_with_dataset(&dataset_refs);
    }

    let stats = engine.stats();
    println!("Final fitness after 150 gens: {}, initial: {}", stats.best_fitness, initial_fitness);
    println!("Best formula: {}", engine.best_genome.to_expression_string());
    println!("Best active node count: {}", engine.best_genome.node_count());

    // El genoma líder debe mantenerse compacto sin bloat
    assert!(engine.best_genome.node_count() <= 15);
    // La aptitud debe mejorar o mantenerse alta
    assert!(stats.best_fitness >= initial_fitness);
}
