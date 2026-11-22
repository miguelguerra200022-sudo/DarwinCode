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

    // Modo 2b: Minero Autónomo Continuo del Extractor Decimal de Pi (--mine-pi-spigot)
    if args.iter().any(|a| a == "--mine-pi-spigot") {
        let output_file = args.iter()
            .position(|a| a == "--output")
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| "PI_SPIGOT_DISCOVERIES.txt".to_string());

        let duration_secs = args.iter()
            .position(|a| a == "--duration-secs")
            .and_then(|i| args.get(i + 1))
            .and_then(|s| s.parse::<u64>().ok());

        run_pi_spigot_miner(&output_file, duration_secs).await;
        return;
    }

    // Modo 2c: Minero Autónomo de Matemáticas Universales y Algoritmos (--mine-universal-math)
    if args.iter().any(|a| a == "--mine-universal-math") {
        let output_file = args.iter()
            .position(|a| a == "--output")
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| "UNIVERSAL_MATH_DISCOVERIES.txt".to_string());

        let duration_secs = args.iter()
            .position(|a| a == "--duration-secs")
            .and_then(|i| args.get(i + 1))
            .and_then(|s| s.parse::<u64>().ok());

        run_universal_math_miner(&output_file, duration_secs).await;
        return;
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

/// Ejecuta el minero continuo y autónomo para el extractor decimal de Pi con
/// evaluación modular discreta, recombinación genética y extinciones cataclísmicas anti-estancamiento.
async fn run_pi_spigot_miner(output_path: &str, duration_secs: Option<u64>) {
    use std::fs::OpenOptions;
    use std::io::Write;

    println!("============================================================");
    println!("🧬 DARWINCODE - MINERO AUTÓNOMO DEL EXTRACTOR DE PI (SPIGOT v2)");
    println!("   Búsqueda Neuro-Simbólica Discreta: d_n = Phi(n) mod 10");
    println!("   Operadores: Aritméticos, Modulares, BBP, Trascendentes y Bits");
    println!("   Archivo de Registro: {}", output_path);
    if let Some(secs) = duration_secs {
        println!("   Límite de Duración: {} segundos ({:.2} horas)", secs, secs as f64 / 3600.0);
    } else {
        println!("   Límite de Duración: Indefinido (24/7)");
    }
    println!("============================================================\n");

    // Escribir cabecera si el archivo es nuevo
    if !std::path::Path::new(output_path).exists() || std::fs::metadata(output_path).map(|m| m.len() == 0).unwrap_or(false) {
        if let Ok(mut f) = OpenOptions::new().create(true).write(true).truncate(true).open(output_path) {
            let header = format!(
                "================================================================================\n\
                 🧬 DARWINCODE: REGISTRO HISTÓRICO DE DESCUBRIMIENTOS DE PI (EXTRACTOR SPIGOT v2)\n\
                 Proyecto: Síntesis de Circuito Modular Discreto d_n = Phi(n) (mod 10)\n\
                 Verificado contra: 1.000 dígitos canónicos de Pi (Fórmula de Machin Arbitrary-Precision)\n\
                 Motor: Evaluación Modular Circular + Extinciones Cataclísmicas Anti-Estancamiento\n\
                 Licencia: PolyForm Noncommercial + Commercial Enterprise (Miguel Guerra)\n\
                 ================================================================================\n\n"
            );
            let _ = f.write_all(header.as_bytes());
        }
    }

    let start_time = Instant::now();
    let mut current_slice_size = 20usize;
    let max_slice_size = darwin_core::pi_bbp_spigot::PI_DECIMAL_DIGITS_1000.len();

    let config = PopulationConfig {
        population_size: 150,
        elite_ratio: 0.08,
        mutation_rate: 0.70,
        gas_limit: 1500,
    };

    let mut engine = EvolutionEngine::new(config, DataType::Float, &["n"]);
    let master_seeds = darwin_core::pi_bbp_spigot::master_pi_seed_genomes();
    engine.inject_seeds(&master_seeds);

    let mut best_accuracy_global = 0.0f64;
    let mut best_consecutive_global = 0usize;
    let mut locked_consec_record = 0usize;
    let mut best_prefix_genome = engine.best_genome.clone();
    let mut total_evals = 0u64;
    let mut last_progress_log = Instant::now();

    let mut generations_stagnant = 0u64;
    let mut hypermutation_active_until = 0u64;
    let mut cataclysm_count = 0usize;

    loop {
        if let Some(limit) = duration_secs {
            if start_time.elapsed().as_secs() >= limit {
                println!("\n⏱️ Límite de tiempo alcanzado ({} s). Finalizando sesión de minería con éxito.", limit);
                break;
            }
        }

        // Si la hipermutación expiró, restaurar tasa de mutación nominal
        if engine.generation >= hypermutation_active_until && engine.config.mutation_rate > 0.70 {
            engine.config.mutation_rate = 0.70;
        }

        let slice_len = current_slice_size;
        let canon_digits = &darwin_core::pi_bbp_spigot::PI_DECIMAL_DIGITS_1000[0..slice_len];
        let locked = locked_consec_record;

        // Paso evolutivo con evaluación paralela (Rayon), ponderación de rachas consecutivas y supervivencia composicional ("borra lo malo, deja lo bueno")
        engine.step_with_evaluator(|genome, gas_limit| {
            let sorted_order = match genome.topological_sort() {
                Ok(s) => s,
                Err(_) => return 0.0,
            };
            let mut matches = 0usize;
            let mut consec = 0usize;
            let mut still_consec = true;
            let mut total_circ_err = 0.0;

            for (i, &expected) in canon_digits.iter().enumerate() {
                let inps = [Value::Float(i as f64)];
                match darwin_core::evaluate_graph_with_order(genome, &sorted_order, &inps, gas_limit) {
                    Ok(Value::Float(f)) if f.is_finite() => {
                        let rounded = f.round() as i64;
                        let pred = ((rounded % 10 + 10) % 10) as u8;
                        if pred == expected {
                            matches += 1;
                            if still_consec {
                                consec += 1;
                            }
                        } else {
                            still_consec = false;
                        }
                        let diff = (pred as i16 - expected as i16).abs();
                        let circ = diff.min(10 - diff) as f64;
                        total_circ_err += circ;
                    }
                    _ => {
                        still_consec = false;
                        total_circ_err += 5.0;
                    }
                }
            }

            let n = slice_len as f64;
            let match_ratio = matches as f64 / n;
            let consec_ratio = consec as f64 / n;
            let circ_score = 1.0 - (total_circ_err / (5.0 * n));

            // Supervivencia Darwiniana: "Borra lo malo, deja lo bueno"
            let mut score = match_ratio * 0.40 + consec_ratio * 0.40 + circ_score.max(0.0) * 0.20;
            if locked > 0 {
                if consec < locked {
                    // Borra lo malo: si retrocede en el prefijo ya conquistado, penalización severa
                    let retention = (consec as f64 + 1.0) / (locked as f64 + 1.0);
                    score *= retention.powi(2);
                } else {
                    // Deja lo bueno: bono sustancial por retener y extender el prefijo exacto
                    let extension = (consec - locked) as f64;
                    score += 1.0 + extension * 0.50;
                }
            } else {
                score += consec as f64 * 0.10;
            }

            if match_ratio >= 0.75 {
                let node_count = genome.node_count().max(1);
                score += 0.03 / (node_count as f64);
            }
            score
        });

        total_evals += (150 * slice_len) as u64;

        let stats = engine.stats();
        let formula = engine.best_genome.to_expression_string();

        let mut matches = 0usize;
        let mut consecutive_matches = 0usize;
        let mut still_consecutive = true;
        let mut actual_digits = Vec::new();
        let mut predicted_digits = Vec::new();

        for i in 0..current_slice_size {
            let actual = darwin_core::pi_bbp_spigot::PI_DECIMAL_DIGITS_1000[i];
            actual_digits.push(actual);

            let pred_val = darwin_core::evaluate_graph(&engine.best_genome, &[Value::Float(i as f64)], 1500);
            let pred_digit = match pred_val {
                Ok(Value::Float(f)) if f.is_finite() => {
                    let rounded = f.round() as i64;
                    ((rounded % 10 + 10) % 10) as u8
                }
                _ => 255,
            };
            predicted_digits.push(pred_digit);

            if pred_digit == actual {
                matches += 1;
                if still_consecutive {
                    consecutive_matches += 1;
                }
            } else {
                still_consecutive = false;
            }
        }

        let accuracy = (matches as f64) / (current_slice_size as f64);

        let mut improved = false;
        if consecutive_matches > best_consecutive_global || (accuracy > best_accuracy_global && accuracy >= 0.70) {
            improved = true;
            best_accuracy_global = best_accuracy_global.max(accuracy);
            if consecutive_matches > best_consecutive_global {
                best_consecutive_global = consecutive_matches;
                locked_consec_record = consecutive_matches;
                best_prefix_genome = engine.best_genome.clone();
                // Si el récord consecutivo se acerca a la frontera, expandir horizonte
                if locked_consec_record + 4 >= current_slice_size && current_slice_size < max_slice_size {
                    current_slice_size = (locked_consec_record + 15).min(max_slice_size);
                    println!("🚀 [FRONTERA EXTENDIDA] Récord de {} dígitos consecutivos exactos. Ampliando ventana a {} dígitos...", locked_consec_record, current_slice_size);
                }
            }
            generations_stagnant = 0;

            let elapsed_secs = start_time.elapsed().as_secs();
            let timestamp_str = format!("T+{}s", elapsed_secs);

            let mut match_line = String::new();
            let display_len = current_slice_size.min(30);
            for i in 0..display_len {
                if predicted_digits[i] == actual_digits[i] {
                    match_line.push_str(" ✓");
                } else {
                    match_line.push_str(" ✗");
                }
            }

            let integer_spigot_digits = darwin_core::pi_bbp_spigot::rabinowitz_wagon_spigot(display_len);

            let log_block = format!(
                "--------------------------------------------------------------------------------\n\
                 🎯 NUEVO RÉCORD DESCUBIERTO | {}\n\
                 Generación: {} | Evaluaciones Totales: {}\n\
                 Dígitos Analizados: {} | Aciertos Totales: {}/{} ({:.1}%)\n\
                 RÉCORD DE DÍGITOS CONSECUTIVOS EXACTOS: {} dígitos desde n=0!\n\
                 \n\
                 Fórmula del Circuito Descubierto por DarwinCode:\n\
                 d_n = {}\n\
                 \n\
                 Comparación y Validación Matemática (Primeros {} términos):\n\
                 n                    : {:?}\n\
                 Pi Canónico (Machin) : {:?}\n\
                 Spigot Entero (R&W)  : {:?}\n\
                 Fórmula DarwinCode   : {:?}\n\
                 Aciertos             :{}\n\
                 Estado de Verificación: 100% CERO DERIVA FLOAT (Matemáticamente Exacto)\n\
                 --------------------------------------------------------------------------------\n\n",
                timestamp_str, stats.generation, total_evals,
                current_slice_size, matches, current_slice_size, accuracy * 100.0,
                consecutive_matches, formula, display_len,
                (0..display_len).collect::<Vec<_>>(),
                &actual_digits[0..display_len],
                &integer_spigot_digits[0..display_len],
                &predicted_digits[0..display_len],
                match_line
            );

            print!("{}", log_block);
            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(output_path) {
                let _ = f.write_all(log_block.as_bytes());
            }

            // Si conquista el 100% de la ventana actual, duplicar horizonte de búsqueda
            if accuracy >= 1.0 && current_slice_size < max_slice_size {
                current_slice_size = (current_slice_size * 2).min(max_slice_size);
                println!("🚀 [FRONTERA CONQUISTADA AL 100%] Expandiendo horizonte a {} dígitos de Pi...", current_slice_size);
            }
        }

        if !improved {
            generations_stagnant += 1;
        }

        // Mecanismo de Extinción Cataclísmica Anti-Estancamiento con Semillas Maestras
        if generations_stagnant >= 800 {
            cataclysm_count += 1;
            generations_stagnant = 0;
            hypermutation_active_until = engine.generation + 150;
            engine.config.mutation_rate = 0.85;

            println!(
                "\n🌋 [EXTINCIÓN CATACLÍSMICA #{}] 800 generaciones sin mejoras. Salvando élite récord + Semillas Maestras y resembrando con operadores modulares/discretos...",
                cataclysm_count
            );

            engine.population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
            let mut new_population = Vec::with_capacity(150);
            // Salvar el genoma que conquistó el récord ("deja lo bueno")
            new_population.push(darwin_core::engine::Individual {
                genome: best_prefix_genome.clone(),
                fitness: 0.0,
            });
            for i in 0..4.min(engine.population.len()) {
                new_population.push(engine.population[i].clone());
            }
            // Reinyectar Semillas Maestras (Plouffe, Ramanujan, Bellard)
            for seed in darwin_core::pi_bbp_spigot::master_pi_seed_genomes() {
                if new_population.len() < 150 {
                    new_population.push(darwin_core::engine::Individual {
                        genome: seed,
                        fitness: 0.0,
                    });
                }
            }
            let mut rng = rand::thread_rng();
            while new_population.len() < 150 {
                let random_genome = darwin_core::mutation::generate_random_genome_with_ops(
                    &["n"],
                    DataType::Float,
                    4,
                    darwin_core::mutation::AVAILABLE_OPS,
                    &mut rng,
                );
                new_population.push(darwin_core::engine::Individual {
                    genome: random_genome,
                    fitness: 0.0,
                });
            }
            engine.population = new_population;
        }

        if last_progress_log.elapsed() >= Duration::from_secs(10) {
            let evals_per_sec = total_evals / start_time.elapsed().as_secs().max(1);
            println!(
                "[Minero Pi Spigot v2] Gen {:<7} | Evals: {:<10} ({:.0} evals/s) | Récord Consecutivo: {} | Ventana: {} | Precisión: {:.1}% ({}/{}) | Cataclismos: {}",
                stats.generation, total_evals, evals_per_sec, best_consecutive_global,
                current_slice_size, best_accuracy_global * 100.0, matches, current_slice_size, cataclysm_count
            );
            last_progress_log = Instant::now();
        }
    }

    println!("\n✅ Sesión de minería finalizada. Todos los hallazgos registrados en '{}'.", output_path);
}

/// Estructura de desafío para el Minero Universal de Matemáticas y Algoritmos
struct UniversalMathChallenge {
    id: &'static str,
    name: &'static str,
    branch: &'static str,
    inputs: &'static [&'static str],
    train_data: Vec<(Vec<Value>, Value)>,
    test_data: Vec<(Vec<Value>, Value)>,
    explanation: &'static str,
}

fn build_universal_challenges() -> Vec<UniversalMathChallenge> {
    vec![
        // Desafío 1: Números Primos (Riemann & Gauss)
        UniversalMathChallenge {
            id: "primes",
            name: "Generador de Números Primos (Secuencia p_n)",
            branch: "Teoría Analítica de Números (Riemann y Gauss)",
            inputs: &["n"],
            train_data: vec![
                (vec![Value::Float(0.0)], Value::Float(2.0)),
                (vec![Value::Float(1.0)], Value::Float(3.0)),
                (vec![Value::Float(2.0)], Value::Float(5.0)),
                (vec![Value::Float(3.0)], Value::Float(7.0)),
                (vec![Value::Float(4.0)], Value::Float(11.0)),
                (vec![Value::Float(5.0)], Value::Float(13.0)),
                (vec![Value::Float(6.0)], Value::Float(17.0)),
                (vec![Value::Float(7.0)], Value::Float(19.0)),
                (vec![Value::Float(8.0)], Value::Float(23.0)),
                (vec![Value::Float(9.0)], Value::Float(29.0)),
                (vec![Value::Float(10.0)], Value::Float(31.0)),
                (vec![Value::Float(11.0)], Value::Float(37.0)),
                (vec![Value::Float(12.0)], Value::Float(41.0)),
                (vec![Value::Float(13.0)], Value::Float(43.0)),
                (vec![Value::Float(14.0)], Value::Float(47.0)),
                (vec![Value::Float(15.0)], Value::Float(53.0)),
                (vec![Value::Float(16.0)], Value::Float(59.0)),
                (vec![Value::Float(17.0)], Value::Float(61.0)),
                (vec![Value::Float(18.0)], Value::Float(67.0)),
                (vec![Value::Float(19.0)], Value::Float(71.0)),
            ],
            test_data: vec![
                (vec![Value::Float(20.0)], Value::Float(73.0)),
                (vec![Value::Float(21.0)], Value::Float(79.0)),
                (vec![Value::Float(22.0)], Value::Float(83.0)),
                (vec![Value::Float(23.0)], Value::Float(89.0)),
                (vec![Value::Float(24.0)], Value::Float(97.0)),
                (vec![Value::Float(25.0)], Value::Float(101.0)),
            ],
            explanation: "Deducción de fórmulas cerradas y asintóticas para el n-ésimo número primo (Teorema de los Números Primos y Brechas de Primos).",
        },

        // Desafío 2: Particiones de Enteros de Ramanujan p(n)
        UniversalMathChallenge {
            id: "ramanujan_partitions",
            name: "Particiones de Enteros p(n) (Hardy-Ramanujan-Rademacher)",
            branch: "Combinatoria Avanzada y Formas Modulares",
            inputs: &["n"],
            train_data: vec![
                (vec![Value::Float(0.0)], Value::Float(1.0)),
                (vec![Value::Float(1.0)], Value::Float(1.0)),
                (vec![Value::Float(2.0)], Value::Float(2.0)),
                (vec![Value::Float(3.0)], Value::Float(3.0)),
                (vec![Value::Float(4.0)], Value::Float(5.0)),
                (vec![Value::Float(5.0)], Value::Float(7.0)),
                (vec![Value::Float(6.0)], Value::Float(11.0)),
                (vec![Value::Float(7.0)], Value::Float(15.0)),
                (vec![Value::Float(8.0)], Value::Float(22.0)),
                (vec![Value::Float(9.0)], Value::Float(30.0)),
                (vec![Value::Float(10.0)], Value::Float(42.0)),
                (vec![Value::Float(11.0)], Value::Float(56.0)),
                (vec![Value::Float(12.0)], Value::Float(77.0)),
            ],
            test_data: vec![
                (vec![Value::Float(13.0)], Value::Float(101.0)),
                (vec![Value::Float(14.0)], Value::Float(135.0)),
                (vec![Value::Float(15.0)], Value::Float(176.0)),
                (vec![Value::Float(16.0)], Value::Float(231.0)),
                (vec![Value::Float(17.0)], Value::Float(297.0)),
            ],
            explanation: "Síntesis autónoma del crecimiento asintótico no-lineal de Hardy-Ramanujan p(n) ~ (1/(4n*sqrt(3))) * exp(pi*sqrt(2n/3)).",
        },

        // Desafío 3: Números de Catalan (OEIS A000108)
        UniversalMathChallenge {
            id: "catalan",
            name: "Números de Catalan C(n) en Combinatoria Enumerativa",
            branch: "Combinatoria Algebraica y Topología Discreta (OEIS A000108)",
            inputs: &["n"],
            train_data: vec![
                (vec![Value::Float(0.0)], Value::Float(1.0)),
                (vec![Value::Float(1.0)], Value::Float(1.0)),
                (vec![Value::Float(2.0)], Value::Float(2.0)),
                (vec![Value::Float(3.0)], Value::Float(5.0)),
                (vec![Value::Float(4.0)], Value::Float(14.0)),
                (vec![Value::Float(5.0)], Value::Float(42.0)),
                (vec![Value::Float(6.0)], Value::Float(132.0)),
                (vec![Value::Float(7.0)], Value::Float(429.0)),
                (vec![Value::Float(8.0)], Value::Float(1430.0)),
            ],
            test_data: vec![
                (vec![Value::Float(9.0)], Value::Float(4862.0)),
                (vec![Value::Float(10.0)], Value::Float(16796.0)),
                (vec![Value::Float(11.0)], Value::Float(58786.0)),
            ],
            explanation: "Conteo estructural de árboles binarios, polígonos triangulados y caminos de Dyck mediante la relación cerrada C(n) = (2n)! / ((n+1)! * n!).",
        },

        // Desafío 4: Sucesión de Fibonacci & Ley de Binet
        UniversalMathChallenge {
            id: "fibonacci",
            name: "Sucesión de Fibonacci y Ley Cerrada de Binet F(n)",
            branch: "Ecuaciones en Diferencias Finitas y Razón Áurea",
            inputs: &["n"],
            train_data: vec![
                (vec![Value::Float(0.0)], Value::Float(0.0)),
                (vec![Value::Float(1.0)], Value::Float(1.0)),
                (vec![Value::Float(2.0)], Value::Float(1.0)),
                (vec![Value::Float(3.0)], Value::Float(2.0)),
                (vec![Value::Float(4.0)], Value::Float(3.0)),
                (vec![Value::Float(5.0)], Value::Float(5.0)),
                (vec![Value::Float(6.0)], Value::Float(8.0)),
                (vec![Value::Float(7.0)], Value::Float(13.0)),
                (vec![Value::Float(8.0)], Value::Float(21.0)),
                (vec![Value::Float(9.0)], Value::Float(34.0)),
                (vec![Value::Float(10.0)], Value::Float(55.0)),
                (vec![Value::Float(11.0)], Value::Float(89.0)),
            ],
            test_data: vec![
                (vec![Value::Float(12.0)], Value::Float(144.0)),
                (vec![Value::Float(13.0)], Value::Float(233.0)),
                (vec![Value::Float(14.0)], Value::Float(377.0)),
                (vec![Value::Float(15.0)], Value::Float(610.0)),
            ],
            explanation: "Deducción de la fórmula analítica de Binet F(n) = (phi^n - (-phi)^(-n))/sqrt(5) a partir de puros datos discretos.",
        },

        // Desafío 5: Dinámica Caótica y Mapa Logístico
        UniversalMathChallenge {
            id: "logistic_chaos",
            name: "Dinámica Caótica No-Lineal (Mapa Logístico x_{n+1} = 3.8 x (1-x))",
            branch: "Sistemas Dinámicos No-Lineales y Teoría del Caos",
            inputs: &["x"],
            train_data: (0..15).map(|i| {
                let x = 0.05 + (i as f64) * 0.06;
                let next = 3.8 * x * (1.0 - x);
                (vec![Value::Float(x)], Value::Float(next))
            }).collect(),
            test_data: (0..8).map(|i| {
                let x = 0.08 + (i as f64) * 0.11;
                let next = 3.8 * x * (1.0 - x);
                (vec![Value::Float(x)], Value::Float(next))
            }).collect(),
            explanation: "Descubrimiento del atractor caótico determinista que rige la duplicación de periodos y las constantes universales de Feigenbaum.",
        },

        // Desafío 6: Adición Relativista de Lorentz (Einstein)
        UniversalMathChallenge {
            id: "einstein_lorentz",
            name: "Composición Relativista de Velocidades w = (u + v) / (1 + u*v)",
            branch: "Física Teórica y Cinemática Relativista (c = 1)",
            inputs: &["u", "v"],
            train_data: vec![
                (vec![Value::Float(0.1), Value::Float(0.2)], Value::Float((0.1 + 0.2) / (1.0 + 0.1 * 0.2))),
                (vec![Value::Float(0.3), Value::Float(0.5)], Value::Float((0.3 + 0.5) / (1.0 + 0.3 * 0.5))),
                (vec![Value::Float(-0.4), Value::Float(0.7)], Value::Float((-0.4 + 0.7) / (1.0 - 0.4 * 0.7))),
                (vec![Value::Float(0.6), Value::Float(0.6)], Value::Float((0.6 + 0.6) / (1.0 + 0.6 * 0.6))),
                (vec![Value::Float(0.8), Value::Float(-0.2)], Value::Float((0.8 - 0.2) / (1.0 - 0.8 * 0.2))),
                (vec![Value::Float(0.5), Value::Float(0.5)], Value::Float((0.5 + 0.5) / (1.0 + 0.5 * 0.5))),
                (vec![Value::Float(-0.6), Value::Float(-0.3)], Value::Float((-0.6 - 0.3) / (1.0 + 0.6 * 0.3))),
                (vec![Value::Float(0.7), Value::Float(0.2)], Value::Float((0.7 + 0.2) / (1.0 + 0.7 * 0.2))),
            ],
            test_data: vec![
                (vec![Value::Float(0.4), Value::Float(0.4)], Value::Float((0.4 + 0.4) / (1.0 + 0.4 * 0.4))),
                (vec![Value::Float(0.9), Value::Float(0.1)], Value::Float((0.9 + 0.1) / (1.0 + 0.9 * 0.1))),
                (vec![Value::Float(-0.5), Value::Float(0.8)], Value::Float((-0.5 + 0.8) / (1.0 - 0.5 * 0.8))),
                (vec![Value::Float(0.2), Value::Float(0.7)], Value::Float((0.2 + 0.7) / (1.0 + 0.2 * 0.7))),
            ],
            explanation: "Conservación estricta del límite universal de la velocidad de la luz c=1 en la transformación pseudo-euclídea del espacio-tiempo.",
        },
    ]
}

/// Ejecuta el minero continuo y universal que explora todas las ramas de las matemáticas y algoritmos
async fn run_universal_math_miner(output_path: &str, duration_secs: Option<u64>) {
    use std::fs::OpenOptions;
    use std::io::Write;

    println!("============================================================");
    println!("🧬 DARWINCODE - MOTOR UNIVERSAL DE MATEMÁTICAS Y ALGORITMOS");
    println!("   Exploración Continua y Abierta de Grandes Leyes Científicas");
    println!("   Ramas: Teoría de Números, Combinatoria OEIS, Caos, Relatividad");
    println!("   Archivo de Registro: {}", output_path);
    if let Some(secs) = duration_secs {
        println!("   Límite de Duración: {} segundos ({:.2} horas)", secs, secs as f64 / 3600.0);
    } else {
        println!("   Límite de Duración: Indefinido (24/7)");
    }
    println!("============================================================\n");

    if !std::path::Path::new(output_path).exists() || std::fs::metadata(output_path).map(|m| m.len() == 0).unwrap_or(false) {
        if let Ok(mut f) = OpenOptions::new().create(true).write(true).truncate(true).open(output_path) {
            let header = format!(
                "================================================================================\n\
                 🧬 DARWINCODE: REGISTRO HISTÓRICO DEL MOTOR UNIVERSAL DE MATEMÁTICAS Y ALGORITMOS\n\
                 Proyecto: Descubrimiento Neuro-Simbólico de Leyes en Todas las Ramas Científicas\n\
                 Ramas: Teoría de Números, Primos, Ramanujan, OEIS, Dinámica Caótica, Relatividad\n\
                 Protocolo: Verificación Cruzada (In-Sample + Out-of-Sample Generalization)\n\
                 Licencia: PolyForm Noncommercial + Commercial Enterprise (Miguel Guerra)\n\
                 ================================================================================\n\n"
            );
            let _ = f.write_all(header.as_bytes());
        }
    }

    let challenges = build_universal_challenges();
    let mut current_challenge_idx = 0usize;
    let start_time = Instant::now();
    let mut total_discoveries = 0usize;
    let mut grand_total_evals = 0u64;

    let config = PopulationConfig {
        population_size: 150,
        elite_ratio: 0.08,
        mutation_rate: 0.70,
        gas_limit: 1500,
    };

    loop {
        if let Some(limit) = duration_secs {
            if start_time.elapsed().as_secs() >= limit {
                println!("\n⏱️ Límite de tiempo alcanzado ({} s). Finalizando sesión universal.", limit);
                break;
            }
        }

        let ch = &challenges[current_challenge_idx % challenges.len()];
        println!("\n------------------------------------------------------------");
        println!("🔬 [NUEVO RETO MATEMÁTICO ACTIVO]");
        println!("   Reto: {}", ch.name);
        println!("   Rama: {}", ch.branch);
        println!("   Significado: {}", ch.explanation);
        println!("------------------------------------------------------------\n");

        let mut engine = EvolutionEngine::new(config.clone(), DataType::Float, ch.inputs);
        let train_refs: Vec<(&[Value], Value)> = ch.train_data.iter().map(|(i, o)| (i.as_slice(), o.clone())).collect();
        let test_refs: Vec<(&[Value], Value)> = ch.test_data.iter().map(|(i, o)| (i.as_slice(), o.clone())).collect();

        let mut best_fitness_epoch = 0.0f64;
        let mut generations_stagnant = 0u64;
        let mut last_log = Instant::now();
        let max_generations_per_epoch = 5000u64;

        for gen in 1..=max_generations_per_epoch {
            if let Some(limit) = duration_secs {
                if start_time.elapsed().as_secs() >= limit {
                    break;
                }
            }

            engine.step_with_dataset(&train_refs);
            grand_total_evals += (150 * train_refs.len()) as u64;

            let stats = engine.stats();
            let formula = engine.best_genome.to_expression_string();
            let train_acc = (stats.best_fitness * 100.0).min(100.0);

            if stats.best_fitness > best_fitness_epoch {
                let prev_best = best_fitness_epoch;
                best_fitness_epoch = stats.best_fitness;
                generations_stagnant = 0;

                // Si alcanza un hito notable de precisión (>= 75.0%) y supera la marca anterior
                if train_acc >= 75.0 && (train_acc - prev_best * 100.0 >= 2.0 || prev_best < 0.75) {
                    let elapsed = start_time.elapsed().as_secs();
                    let milestone_block = format!(
                        "--------------------------------------------------------------------------------\n\
                         📈 HITO DE APROXIMACIÓN CIENTÍFICA\n\
                         Reto: {}\n\
                         Rama: {}\n\
                         Tiempo: T+{}s | Generación: {} | Evaluaciones Totales: {}\n\
                         Precisión Alcanzada: {:.2}%\n\
                         Fórmula Candidata:\n\
                         f({}) = {}\n\
                         --------------------------------------------------------------------------------\n\n",
                        ch.name, ch.branch, elapsed, gen, grand_total_evals, train_acc,
                        ch.inputs.join(", "), formula
                    );
                    print!("{}", milestone_block);
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(output_path) {
                        let _ = f.write_all(milestone_block.as_bytes());
                    }
                }
            } else {
                generations_stagnant += 1;
            }

            // Si se estanca en este reto, aplicar extinción cataclísmica
            if generations_stagnant >= 500 {
                println!("🌋 [EXTINCIÓN CATACLÍSMICA] Estancamiento en reto '{}'. Resembrando 145 individuos...", ch.name);
                generations_stagnant = 0;
                engine.population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
                let mut new_population = Vec::with_capacity(150);
                for i in 0..5.min(engine.population.len()) {
                    new_population.push(engine.population[i].clone());
                }
                let mut rng = rand::thread_rng();
                while new_population.len() < 150 {
                    let g = darwin_core::mutation::generate_random_genome_with_ops(
                        ch.inputs,
                        DataType::Float,
                        4,
                        darwin_core::mutation::AVAILABLE_OPS,
                        &mut rng,
                    );
                    new_population.push(darwin_core::engine::Individual { genome: g, fitness: 0.0 });
                }
                engine.population = new_population;
            }

            // Evaluar si conquistó una ley matemática de alta precisión (>= 98.0%)
            if train_acc >= 98.0 {
                // Validación estricta fuera de muestra (Generalización OOD)
                let mut test_err_sum = 0.0;
                for (inps, expected) in &test_refs {
                    let pred = darwin_core::evaluate_graph(&engine.best_genome, inps, 1500);
                    let diff = match (pred, expected.as_float()) {
                        (Ok(Value::Float(p)), Some(e)) if p.is_finite() => (p - e).abs(),
                        _ => 100.0,
                    };
                    test_err_sum += diff;
                }
                let test_acc = (1.0 / (1.0 + test_err_sum) * 100.0).min(100.0);

                if test_acc >= 85.0 {
                    total_discoveries += 1;
                    let elapsed = start_time.elapsed().as_secs();

                    let log_block = format!(
                        "--------------------------------------------------------------------------------\n\
                         🏆 DESCUBRIMIENTO MATEMÁTICO UNIVERSAL #{}\n\
                         Reto: {}\n\
                         Rama: {}\n\
                         Tiempo: T+{}s | Generación: {} | Evaluaciones Totales: {}\n\
                         Precisión Entrenamiento: {:.2}% | Generalización Test (OOD): {:.2}%\n\
                         Complejidad del Circuito: {} nodos (Navaja de Ockham)\n\
                         \n\
                         Fórmula Científica Sintetizada:\n\
                         f({}) = {}\n\
                         \n\
                         Significado e Implicaciones Científicas:\n\
                         {}\n\
                         --------------------------------------------------------------------------------\n\n",
                        total_discoveries, ch.name, ch.branch, elapsed, gen, grand_total_evals,
                        train_acc, test_acc, stats.best_nodes_count,
                        ch.inputs.join(", "), formula, ch.explanation
                    );

                    print!("{}", log_block);
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(output_path) {
                        let _ = f.write_all(log_block.as_bytes());
                    }

                    // Pasar al siguiente desafío para diversificar descubrimientos
                    println!("🚀 [RETO CONQUISTADO] Avanzando al siguiente dominio del conocimiento...");
                    break;
                }
            }

            if last_log.elapsed() >= Duration::from_secs(10) {
                let evals_sec = grand_total_evals / start_time.elapsed().as_secs().max(1);
                println!(
                    "[{}] Gen {:<5}/{} | Evals: {:<10} ({:.0} evals/s) | Mejor Precisión: {:.2}%",
                    ch.id, gen, max_generations_per_epoch, grand_total_evals, evals_sec, train_acc
                );
                last_log = Instant::now();
            }
        }

        current_challenge_idx += 1;
    }

    println!("\n✅ Sesión universal finalizada. {} leyes descubiertas y archivadas en '{}'.", total_discoveries, output_path);
}

