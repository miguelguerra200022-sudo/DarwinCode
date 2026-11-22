use darwin_cli::server::{run_server, DiscoveryRecord, ServerState, TelemetryPacket};
use darwin_core::engine::{EvolutionEngine, PopulationConfig};
use darwin_core::types::{DataType, Value};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, Mutex};

struct ChallengeSpec {
    id: &'static str,
    name: &'static str,
    inputs: &'static [&'static str],
    dataset: Vec<(Vec<Value>, Value)>,
}

fn get_challenge(id: &str) -> ChallengeSpec {
    match id {
        "pi_billiards" => ChallengeSpec {
            id: "pi_billiards",
            name: "Billares Caóticos de Galperin (Colisiones de Pi: ⌊π · 10^N⌋)",
            inputs: &["n"],
            dataset: vec![
                (vec![Value::Float(0.0)], Value::Float(3.0)),
                (vec![Value::Float(1.0)], Value::Float(31.0)),
                (vec![Value::Float(2.0)], Value::Float(314.0)),
                (vec![Value::Float(3.0)], Value::Float(3141.0)),
                (vec![Value::Float(4.0)], Value::Float(31415.0)),
            ],
        },
        "pi_spigot" => ChallengeSpec {
            id: "pi_spigot",
            name: "Extractor Modular BBP Decimal de Pi (d_n ∈ {0..9})",
            inputs: &["n"],
            dataset: darwin_core::pi_bbp_spigot::pi_digit_dataset(20),
        },
        _ => {
            // "pi_cf" - Fracciones Continuas Recurrentes de Pi
            let (_b0, steps) = darwin_core::pi_continued_fractions::ramanujan_pi_cf_steps(10);
            let dataset: Vec<(Vec<Value>, Value)> = (1..=8)
                .map(|k| {
                    let (cf_val, _) = darwin_core::pi_continued_fractions::evaluate_euler_wallis(
                        &steps[0..k],
                        1.0,
                        4.0 / std::f64::consts::PI,
                    );
                    (
                        vec![Value::Float(k as f64)],
                        Value::Float(4.0 / cf_val),
                    )
                })
                .collect();
            ChallengeSpec {
                id: "pi_cf",
                name: "Fracciones Continuas Recurrentes de Pi (Euler-Wallis S_k -> π)",
                inputs: &["k"],
                dataset,
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Modo 1: Co-procesador MCP stdio para Agentes IA (Claude Code, Antigravity, Codex, etc.)
    if args.iter().any(|a| a == "--mcp") {
        if let Err(e) = darwin_cli::mcp::run_mcp_server().await {
            eprintln!("Error en servidor MCP: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Modo 2: Síntesis directa por línea de comandos (Headless CLI)
    if args.iter().any(|a| a == "synthesize") {
        let default_ds = vec![
            darwin_cli::mcp_tools::DataPoint { inputs: vec![-3.0], expected: 10.0 },
            darwin_cli::mcp_tools::DataPoint { inputs: vec![-2.0], expected: 5.0 },
            darwin_cli::mcp_tools::DataPoint { inputs: vec![-1.0], expected: 2.0 },
            darwin_cli::mcp_tools::DataPoint { inputs: vec![0.0], expected: 1.0 },
            darwin_cli::mcp_tools::DataPoint { inputs: vec![1.0], expected: 2.0 },
            darwin_cli::mcp_tools::DataPoint { inputs: vec![2.0], expected: 5.0 },
            darwin_cli::mcp_tools::DataPoint { inputs: vec![3.0], expected: 10.0 },
        ];
        let p = darwin_cli::mcp_tools::SynthesizeParams {
            dataset: default_ds,
            input_names: Some(vec!["x".to_string()]),
            max_generations: Some(1000),
            target_accuracy: Some(0.999),
            population_size: Some(100),
            enable_sota: Some(true),
            enable_constant_optimization: Some(true),
            enable_symmetry_prepass: Some(true),
            allowed_ops: None,
        };
        match darwin_cli::mcp_tools::execute_synthesize(p) {
            Ok(res) => {
                println!("{}", res.markdown);
                return;
            }
            Err(e) => {
                eprintln!("Error en síntesis CLI: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Modo 3: Servidor Web Cockpit 3D interactivo en tiempo real
    println!("============================================================");
    println!("🧬 DARWINCODE v0.2.0 - El Ecosistema Digital Vivo");
    println!("   Motor de Computación Evolutiva Abierta (OEE) y Formal");
    println!("   Modo Web Cockpit 3D + Telemetría SSE");
    println!("============================================================");

    let is_running = Arc::new(AtomicBool::new(true));
    let delay_ms = Arc::new(AtomicU64::new(0));
    let target_challenge = Arc::new(Mutex::new("pi_cf".to_string()));
    let hall_of_fame = Arc::new(Mutex::new(Vec::<DiscoveryRecord>::new()));
    let (tx, _) = broadcast::channel::<String>(100);

    let server_state = Arc::new(ServerState {
        is_running: is_running.clone(),
        delay_ms: delay_ms.clone(),
        target_challenge: target_challenge.clone(),
        hall_of_fame: hall_of_fame.clone(),
        tx: tx.clone(),
    });

    let mut port = 8080;
    if let Some(pos) = args.iter().position(|a| a == "--port") {
        if let Some(val) = args.get(pos + 1) {
            if let Ok(p) = val.parse::<u16>() {
                port = p;
            }
        }
    }

    tokio::spawn(run_server(port, server_state.clone()));

    let config = PopulationConfig {
        population_size: 100,
        elite_ratio: 0.05,
        mutation_rate: 0.75,
        gas_limit: 1200,
    };

    let pop_size = config.population_size;
    let mut current_challenge_id = "pi_cf".to_string();
    let mut active_spec = get_challenge(&current_challenge_id);
    let mut engine = EvolutionEngine::new(config, DataType::Float, active_spec.inputs);

    println!("Iniciando motor evolutivo sin restricciones...");
    println!("Desafío activo: {}", active_spec.name);
    println!("Presiona Ctrl+C en cualquier momento para detener.\n");

    let mut last_broadcast = Instant::now();
    let mut last_terminal_log = Instant::now();
    let mut evals_in_window = 0u64;

    loop {
        // Verificar si el usuario cambió el desafío desde la Web
        {
            let req_id = {
                let lock = target_challenge.lock().await;
                lock.clone()
            };
            if req_id != current_challenge_id {
                current_challenge_id = req_id;
                active_spec = get_challenge(&current_challenge_id);
                engine.reset_with_inputs(active_spec.inputs);
                println!("\n🔄 CAMBIO DE DESAFÍO SOLICITADO: {}", active_spec.name);
            }
        }

        if is_running.load(Ordering::SeqCst) {
            let dataset_refs: Vec<(&[Value], Value)> = active_spec
                .dataset
                .iter()
                .map(|(inps, out)| (inps.as_slice(), out.clone()))
                .collect();

            engine.step_with_dataset(&dataset_refs);
            evals_in_window += (pop_size * dataset_refs.len()) as u64;

            let stats = engine.stats();
            let formula = engine.best_genome.to_expression_string();
            let precision = (stats.best_fitness * 100.0).min(100.0);

            // Registro automático en el Hall of Fame cuando se alcanza la precisión máxima (o >= 99.5%)
            if stats.best_fitness >= 0.995 || precision >= 99.5 {
                let mut fame = hall_of_fame.lock().await;
                if !fame.iter().any(|r| r.challenge_id == active_spec.id) {
                    println!("\n🏆 ¡LEY MATEMÁTICA CONQUISTADA AL {:.1}%!", precision);
                    println!("   Reto: {}", active_spec.name);
                    println!("   Fórmula Descubierta: {}", formula);
                    println!("   Generación: {} | Nodos: {}\n", stats.generation, stats.best_nodes_count);

                    fame.push(DiscoveryRecord {
                        challenge_id: active_spec.id.to_string(),
                        challenge_name: active_spec.name.to_string(),
                        formula: formula.clone(),
                        generation: stats.generation,
                        accuracy: precision,
                        node_count: stats.best_nodes_count,
                    });
                }
                drop(fame);

                // Auto-rotación al siguiente desafío científico para demostrar aprendizaje continuo en vivo
                if stats.generation >= 150 {
                    let all_challenges = ["pi_cf", "pi_billiards", "pi_spigot"];
                    let curr_idx = all_challenges.iter().position(|&c| c == current_challenge_id).unwrap_or(0);
                    let next_idx = (curr_idx + 1) % all_challenges.len();
                    let next_id = all_challenges[next_idx];
                    current_challenge_id = next_id.to_string();
                    *target_challenge.lock().await = next_id.to_string();
                    active_spec = get_challenge(&current_challenge_id);
                    engine.reset_with_inputs(active_spec.inputs);
                    println!("\n🔄 [EVOLUCIÓN EN VIVO] Desafío completado. Rotando al siguiente enigma: {}", active_spec.name);
                }
            } else if stats.generation >= 2000 {
                // Rotación por límite de generaciones para evitar estancamiento en mesetas locales
                let all_challenges = ["pi_cf", "pi_billiards", "pi_spigot"];
                let curr_idx = all_challenges.iter().position(|&c| c == current_challenge_id).unwrap_or(0);
                let next_idx = (curr_idx + 1) % all_challenges.len();
                let next_id = all_challenges[next_idx];
                current_challenge_id = next_id.to_string();
                *target_challenge.lock().await = next_id.to_string();
                active_spec = get_challenge(&current_challenge_id);
                engine.reset_with_inputs(active_spec.inputs);
                println!("\n🔄 [EVOLUCIÓN EN VIVO] Exploración completada. Rotando al siguiente enigma: {}", active_spec.name);
            }

            let now = Instant::now();

            // Emitir telemetría al navegador a ~30 Hz
            if now.duration_since(last_broadcast) >= Duration::from_millis(33) {
                let spatial_layout = engine.best_genome.compute_spatial_layout();
                let voxels = engine.best_genome.generate_voxel_representation();
                let fame_snapshot = {
                    let lock = hall_of_fame.lock().await;
                    lock.clone()
                };

                let packet = TelemetryPacket {
                    stats: stats.clone(),
                    best_graph: engine.best_genome.clone(),
                    formula: formula.clone(),
                    spatial_layout,
                    voxels,
                    is_running: true,
                    current_challenge: active_spec.id.to_string(),
                    current_challenge_name: active_spec.name.to_string(),
                    hall_of_fame: fame_snapshot,
                    delay_ms: delay_ms.load(Ordering::Relaxed),
                };
                if let Ok(json) = serde_json::to_string(&packet) {
                    let _ = tx.send(json);
                }
                last_broadcast = now;
            }

            // Mostrar resumen en terminal cada 2 segundos
            if now.duration_since(last_terminal_log) >= Duration::from_secs(2) {
                let gen_per_sec = (evals_in_window as f64) / 2.0;
                println!(
                    "Gen {:>5} | Precisión: {:>5.1}% (Fit: {:>6.4}) | ADN: {:<25} | Eval/s: {:>7.0}",
                    stats.generation, precision, stats.best_fitness, formula, gen_per_sec
                );
                evals_in_window = 0;
                last_terminal_log = now;
            }

            // Aplicar velocidad regulable (cámara lenta o máxima potencia)
            let sleep_ms = delay_ms.load(Ordering::Relaxed);
            if sleep_ms > 0 {
                tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
            }
        } else {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        tokio::task::yield_now().await;
    }
}
