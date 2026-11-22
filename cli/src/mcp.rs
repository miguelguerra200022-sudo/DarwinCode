use crate::mcp_protocol::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, ToolCallResult, ToolDefinition,
};
use crate::mcp_tools::{
    execute_bsd_analysis, execute_evolve_cppn, execute_evolve_voxels, execute_generate_city,
    execute_get_status, execute_navier_stokes_blowup, execute_oeis_discover, execute_optimize_code,
    execute_ramanujan_solve, execute_safe_eval, execute_srsd_hard_solve, execute_synthesize,
    BsdParams, CityParams, CppnParams, NavierStokesParams, OeisParams, OptimizeParams,
    RamanujanParams, SafeEvalParams, SrsdParams, SynthesizeParams, VoxelParams,
};
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

/// Retorna la lista oficial de herramientas exportadas por DarwinCode.
pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "darwin_synthesize".to_string(),
            description: "Descubre leyes matemáticas, físicas o algoritmos cerrados a partir de un conjunto de datos numéricos usando evolución genética abierta y poda de introns con Navaja de Ockham.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "dataset": {
                        "type": "array",
                        "description": "Lista de puntos de prueba con entradas numéricas y el valor esperado",
                        "items": {
                            "type": "object",
                            "properties": {
                                "inputs": {
                                    "type": "array",
                                    "items": { "type": "number" },
                                    "description": "Valores de entrada f64 [x0, x1, ...]"
                                },
                                "expected": {
                                    "type": "number",
                                    "description": "Resultado f64 esperado"
                                }
                            },
                            "required": ["inputs", "expected"]
                        }
                    },
                    "input_names": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Nombres simbólicos para las variables (ej: ['x'] o ['x', 'y'])"
                    },
                    "max_generations": {
                        "type": "integer",
                        "description": "Límite máximo de generaciones (por defecto 1000)"
                    },
                    "target_accuracy": {
                        "type": "number",
                        "description": "Precisión mínima para convergencia temprana entre 0.0 y 1.0 (por defecto 0.999)"
                    },
                    "population_size": {
                        "type": "integer",
                        "description": "Tamaño de la población de genomas en el ecosistema (por defecto 100)"
                    },
                    "enable_sota": {
                        "type": "boolean",
                        "description": "Activa el pipeline SOTA (AI Feynman, Levenberg-Marquardt y MAP-Elites)"
                    },
                    "enable_constant_optimization": {
                        "type": "boolean",
                        "description": "Habilita la optimización continua de constantes por mínimos cuadrados amortiguados"
                    },
                    "enable_symmetry_prepass": {
                        "type": "boolean",
                        "description": "Habilita la detección de simetrías de permutación y paridad"
                    },
                    "allowed_ops": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Operadores permitidos (ej: ['Add', 'Subtract', 'Multiply', 'Divide'])"
                    }
                },
                "required": ["dataset"]
            }),
        },
        ToolDefinition {
            name: "darwin_optimize_code".to_string(),
            description: "Simplifica fórmulas o algoritmos eliminando código muerto y complejidad innecesaria mediante presión de parsimonia evolutiva.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "Expresión simbólica o código a optimizar"
                    },
                    "dataset": {
                        "type": "array",
                        "description": "Puntos de prueba para verificar equivalencia semántica"
                    },
                    "input_names": {
                        "type": "array",
                        "items": { "type": "string" }
                    },
                    "max_generations": {
                        "type": "integer",
                        "description": "Límite de generaciones para la optimización (por defecto 500)"
                    }
                }
            }),
        },
        ToolDefinition {
            name: "darwin_evolve_voxels".to_string(),
            description: "Genera planos arquitectónicos, morfológicos y criaturas en 3D (vóxeles [x, y, z, tipo]).".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "floors": {
                        "type": "integer",
                        "description": "Número de niveles o pisos estructurales (por defecto 3)"
                    },
                    "style": {
                        "type": "string",
                        "enum": ["tower", "pyramid", "bridge", "dog", "canine", "animal"],
                        "description": "Estilo morfológico o criatura (tower, pyramid, bridge, dog, canine)"
                    },
                    "max_blocks": {
                        "type": "integer",
                        "description": "Cantidad máxima de bloques vóxel a ubicar (por defecto 300)"
                    }
                }
            }),
        },
        ToolDefinition {
            name: "darwin_evolve_cppn".to_string(),
            description: "Evoluciona morfologías 3D orgánicas y estructuras continuas autónomas mediante redes neuronales generativas CPPN-NEAT (selección artificial y simetría bilateral).".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "generations": {
                        "type": "integer",
                        "description": "Número de generaciones de selección natural a simular (por defecto 15)"
                    },
                    "population_size": {
                        "type": "integer",
                        "description": "Tamaño de la población de genomas en competencia (por defecto 30)"
                    },
                    "resolution": {
                        "type": "integer",
                        "description": "Resolución de la cuadrícula tridimensional de muestreo (por defecto 14)"
                    },
                    "seed": {
                        "type": "integer",
                        "description": "Semilla estocástica para reproducibilidad genética"
                    }
                }
            }),
        },
        ToolDefinition {
            name: "darwin_generate_city".to_string(),
            description: "Genera una metrópolis 3D autónoma viva con rascacielos, tráfico vehicular, peatones y semáforos interactivos.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "grid_size": {
                        "type": "integer",
                        "description": "Dimensión de la cuadrícula de distritos (por defecto 5)"
                    },
                    "num_vehicles": {
                        "type": "integer",
                        "description": "Número de vehículos autónomos en circulación (por defecto 50)"
                    },
                    "num_pedestrians": {
                        "type": "integer",
                        "description": "Número de ciudadanos autónomos en aceras y cruces (por defecto 70)"
                    },
                    "theme": {
                        "type": "string",
                        "enum": ["metropolis", "cyberpunk", "modern"],
                        "description": "Tema visual y arquitectónico de la metrópolis"
                    }
                }
            }),
        },
        ToolDefinition {
            name: "darwin_safe_eval".to_string(),
            description: "Ejecuta de forma determinista y segura puntos de datos en la máquina virtual de DarwinCode con medidor de gas para prevenir cuelgues o bucles infinitos.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "dataset": {
                        "type": "array",
                        "description": "Puntos de datos a evaluar de forma aislada",
                        "items": {
                            "type": "object",
                            "properties": {
                                "inputs": { "type": "array", "items": { "type": "number" } },
                                "expected": { "type": "number" }
                            },
                            "required": ["inputs", "expected"]
                        }
                    },
                    "gas_limit": {
                        "type": "integer",
                        "description": "Límite de instrucciones de gas (por defecto 2000)"
                    }
                },
                "required": ["dataset"]
            }),
        },
        ToolDefinition {
            name: "darwin_ramanujan_solve".to_string(),
            description: "Evalúa conjeturas de fracciones continuas generalizadas (GCF) de la Ramanujan Machine con convergencia de alta precisión para constantes fundamentales (Catalan G, Apery zeta(3), pi, e).".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "constant": {
                        "type": "string",
                        "description": "Constante objetivo a evaluar: 'catalan', 'apery', 'pi', o 'e'"
                    },
                    "max_depth": {
                        "type": "integer",
                        "description": "Profundidad máxima de capas fraccionarias para evaluar mediante el algoritmo de Lentz (5 a 50)"
                    }
                }
            }),
        },
        ToolDefinition {
            name: "darwin_srsd_hard_solve".to_string(),
            description: "Resuelve el benchmark de regresión simbólica científica SRSD-Hard, eliminando variables trampa espurias (dummy noise) con poda de Ockham e identificando la fórmula relativista exacta con NED = 0.0.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "num_samples": {
                        "type": "integer",
                        "description": "Número de muestras de datos físicos a sintetizar (20 a 200)"
                    },
                    "noise_level": {
                        "type": "number",
                        "description": "Nivel de ruido gaussiano añadido a la señal física (0.0 a 0.5)"
                    }
                }
            }),
        },
        ToolDefinition {
            name: "darwin_oeis_discover".to_string(),
            description: "Descubre ecuaciones de recurrencia polinomiales racionales en secuencias enteras de la OEIS y predice términos ocultos con verificación matemática exacta.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "sequence_id": {
                        "type": "string",
                        "description": "Identificador OEIS: 'A000108' (Catalan) o 'A005268' (Fishburn & Roberts)"
                    },
                    "num_training_terms": {
                        "type": "integer",
                        "description": "Número de términos conocidos suministrados para el entrenamiento (4 a 15)"
                    }
                }
            }),
        },
        ToolDefinition {
            name: "darwin_bsd_analysis".to_string(),
            description: "Analiza curvas elípticas sobre F_p y evalúa la conjetura del milenio de Birch y Swinnerton-Dyer (BSD), trazas de Frobenius a_p, ángulos Sato-Tate y paridad del rango algebraico.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "curve_a": { "type": "integer", "description": "Coeficiente A de la curva Weierstrass y^2 = x^3 + Ax + B" },
                    "curve_b": { "type": "integer", "description": "Coeficiente B de la curva Weierstrass y^2 = x^3 + Ax + B" }
                }
            }),
        },
        ToolDefinition {
            name: "darwin_navier_stokes_blowup".to_string(),
            description: "Simula el modelo de singularidad y blow-up de Hou-Luo / De Gregorio para las ecuaciones 3D de Navier-Stokes/Euler, calculando enstrofía, criterio Beale-Kato-Majda y perfiles autosimilares.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "grid_size": { "type": "integer", "description": "Número de puntos de malla espacial (por defecto 128)" },
                    "viscosity": { "type": "number", "description": "Viscosidad cinemática nu (0.0 para Euler invíscido)" },
                    "steps": { "type": "integer", "description": "Pasos temporales de evolución (por defecto 100)" },
                    "dt": { "type": "number", "description": "Paso temporal dt (por defecto 0.001)" }
                }
            }),
        },
        ToolDefinition {
            name: "darwin_get_status".to_string(),
            description: "Obtiene la telemetría, versión, herramientas disponibles y estado del motor neuro-simbólico DarwinCode.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
    ]
}

/// Procesa una solicitud JSON-RPC individual y retorna la respuesta correspondiente.
pub async fn handle_jsonrpc_request(req: JsonRpcRequest) -> Option<JsonRpcResponse> {
    match req.method.as_str() {
        "initialize" => {
            let res = json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "darwin-code",
                    "version": "1.0.0"
                }
            });
            Some(JsonRpcResponse::success(req.id, res))
        }

        "notifications/initialized" => {
            // Notificación sin respuesta según el protocolo MCP
            None
        }

        "ping" => Some(JsonRpcResponse::success(req.id, json!({}))),

        "tools/list" => {
            let tools = get_tool_definitions();
            let res = json!({
                "tools": tools
            });
            Some(JsonRpcResponse::success(req.id, res))
        }

        "tools/call" => {
            let params = match req.params {
                Some(p) => p,
                None => {
                    return Some(JsonRpcResponse::error(
                        req.id,
                        JsonRpcError::invalid_params("Parámetros faltantes en tools/call"),
                    ));
                }
            };

            let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

            let tool_result = match tool_name {
                "darwin_synthesize" => {
                    match serde_json::from_value::<SynthesizeParams>(arguments) {
                        Ok(p) => match execute_synthesize(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en síntesis: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_optimize_code" => {
                    match serde_json::from_value::<OptimizeParams>(arguments) {
                        Ok(p) => match execute_optimize_code(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en optimización: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_evolve_voxels" => {
                    match serde_json::from_value::<VoxelParams>(arguments) {
                        Ok(p) => match execute_evolve_voxels(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en vóxeles: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_evolve_cppn" => {
                    match serde_json::from_value::<CppnParams>(arguments) {
                        Ok(p) => match execute_evolve_cppn(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en evolución CPPN: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_generate_city" => {
                    match serde_json::from_value::<CityParams>(arguments) {
                        Ok(p) => match execute_generate_city(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en generación de ciudad: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_ramanujan_solve" => {
                    match serde_json::from_value::<RamanujanParams>(arguments) {
                        Ok(p) => match execute_ramanujan_solve(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en Ramanujan Machine: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_srsd_hard_solve" => {
                    match serde_json::from_value::<SrsdParams>(arguments) {
                        Ok(p) => match execute_srsd_hard_solve(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en SRSD-Hard: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_oeis_discover" => {
                    match serde_json::from_value::<OeisParams>(arguments) {
                        Ok(p) => match execute_oeis_discover(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en descubrimiento OEIS: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_bsd_analysis" => {
                    match serde_json::from_value::<BsdParams>(arguments) {
                        Ok(p) => match execute_bsd_analysis(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en análisis BSD: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_navier_stokes_blowup" => {
                    match serde_json::from_value::<NavierStokesParams>(arguments) {
                        Ok(p) => match execute_navier_stokes_blowup(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en simulación Navier-Stokes: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_safe_eval" => {
                    match serde_json::from_value::<SafeEvalParams>(arguments) {
                        Ok(p) => match execute_safe_eval(p) {
                            Ok(res) => ToolCallResult::text(res.markdown, false),
                            Err(e) => ToolCallResult::text(format!("Error en evaluación segura: {}", e), true),
                        },
                        Err(e) => ToolCallResult::text(format!("Argumentos inválidos: {}", e), true),
                    }
                }

                "darwin_get_status" => {
                    let st = execute_get_status();
                    let md = format!(
                        "### 🧬 DarwinCode Neuro-Symbolic Engine\n\
                        - **Nombre:** `{}`\n\
                        - **Versión:** `{}`\n\
                        - **Descripción:** {}\n\
                        - **Herramientas Activas:** `{}`\n\
                        - **Desafíos Disponibles:** `{}`\n",
                        st.name,
                        st.version,
                        st.description,
                        st.tools.join(", "),
                        st.supported_challenges.join(", ")
                    );
                    ToolCallResult::text(md, false)
                }

                _ => ToolCallResult::text(format!("Herramienta '{}' no encontrada en DarwinCode", tool_name), true),
            };

            let res_json = serde_json::to_value(&tool_result).unwrap_or(json!({
                "content": [{"type": "text", "text": "Error serializando resultado"}],
                "isError": true
            }));

            Some(JsonRpcResponse::success(req.id, res_json))
        }

        _ => Some(JsonRpcResponse::error(
            req.id,
            JsonRpcError::method_not_found(format!("Método '{}' no implementado", req.method)),
        )),
    }
}

/// Ejecuta el servidor MCP de forma interactiva y asíncrona sobre stdio (stdin/stdout).
/// Todo el logging de depuración se desvía a stderr para garantizar pureza en stdout.
pub async fn run_mcp_server() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("🧬 DarwinCode MCP Server iniciado (stdio transport)");
    eprintln!("   Protocolo: MCP JSON-RPC 2.0 (2024-11-05)");
    eprintln!("   Stdout reservado exclusivamente para mensajes JSON-RPC");

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<JsonRpcResponse>(64);

    // Tarea asíncrona dedicada a escribir en stdout sin bloqueos
    let stdout_handle = tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();
        while let Some(resp) = rx.recv().await {
            if let Ok(serialized) = serde_json::to_string(&resp) {
                let _ = stdout.write_all(serialized.as_bytes()).await;
                let _ = stdout.write_all(b"\n").await;
                let _ = stdout.flush().await;
            }
        }
    });

    while let Some(line) = reader.next_line().await? {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let raw_val: Result<serde_json::Value, _> = serde_json::from_str(trimmed);
        let json_val = match raw_val {
            Ok(v) => v,
            Err(e) => {
                let err_resp = JsonRpcResponse::error(
                    None,
                    JsonRpcError::parse_error(format!("Error de sintaxis JSON: {}", e)),
                );
                let _ = tx.send(err_resp).await;
                continue;
            }
        };

        // Si es JSON válido pero no cumple la estructura de JsonRpcRequest, retornar Invalid Request (-32600)
        let id_val = json_val.get("id").cloned();
        let request: Result<JsonRpcRequest, _> = serde_json::from_value(json_val);

        match request {
            Ok(req) => {
                let req_tx = tx.clone();
                tokio::spawn(async move {
                    if let Some(resp) = handle_jsonrpc_request(req).await {
                        let _ = req_tx.send(resp).await;
                    }
                });
            }
            Err(e) => {
                let err_resp = JsonRpcResponse::error(
                    id_val,
                    JsonRpcError::invalid_request(format!("Solicitud JSON-RPC inválida: {}", e)),
                );
                let _ = tx.send(err_resp).await;
            }
        }
    }

    drop(tx);
    let _ = stdout_handle.await;

    eprintln!("🧬 DarwinCode MCP Server finalizado (EOF en stdin)");
    Ok(())
}
