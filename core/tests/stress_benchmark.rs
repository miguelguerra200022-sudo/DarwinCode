use darwin_core::engine::{EvolutionEngine, PopulationConfig};
use darwin_core::types::{DataType, Value};
use std::time::Instant;

#[test]
#[ignore] // Se ejecuta con cargo test --test stress_benchmark -- --ignored
fn test_extreme_100_000_generations_stress() {
    println!("Iniciando prueba de resistencia extrema: 100,000 generaciones...");
    let start = Instant::now();

    let config = PopulationConfig {
        population_size: 50,
        elite_ratio: 0.1,
        mutation_rate: 0.25,
        gas_limit: 500,
    };

    let mut engine = EvolutionEngine::new(config, DataType::Float, &["x"]);
    let dataset = vec![
        (vec![Value::Float(1.0)], Value::Float(2.0)),
        (vec![Value::Float(2.0)], Value::Float(4.0)),
        (vec![Value::Float(3.0)], Value::Float(6.0)),
    ];
    let dataset_refs: Vec<(&[Value], Value)> = dataset
        .iter()
        .map(|(inps, out)| (inps.as_slice(), out.clone()))
        .collect();

    for gen in 1..=100_000 {
        engine.step_with_dataset(&dataset_refs);
        if gen % 20_000 == 0 {
            let stats = engine.stats();
            println!(
                "Hito alcanzado: Gen {:>6} | Mejor Aptitud: {:>8.4} | Tiempo: {:?}",
                stats.generation,
                stats.best_fitness,
                start.elapsed()
            );
        }
    }

    let duration = start.elapsed();
    let stats = engine.stats();
    println!("============================================================");
    println!("Prueba de 100,000 generaciones superada con éxito.");
    println!("Tiempo total: {:?}", duration);
    println!("Generaciones/segundo: {:.0}", 100_000.0 / duration.as_secs_f64());
    println!("Aptitud final alcanzada: {:.4}", stats.best_fitness);
    println!("============================================================");

    assert_eq!(stats.generation, 100_000);
}
