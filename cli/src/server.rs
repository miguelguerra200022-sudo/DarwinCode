use crate::mcp_tools::{
    execute_bsd_analysis, execute_navier_stokes_blowup, execute_oeis_discover,
    execute_pi_billiards, execute_pi_cf, execute_pi_spigot, execute_ramanujan_solve,
    execute_srsd_hard_solve, BsdParams, NavierStokesParams, OeisParams, PiBilliardsParams,
    PiCfParams, PiSpigotParams, RamanujanParams, SrsdParams,
};
use darwin_core::engine::EngineStats;
use darwin_core::genome::GenomeGraph;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryRecord {
    pub challenge_id: String,
    pub challenge_name: String,
    pub formula: String,
    pub generation: u64,
    pub accuracy: f64,
    pub node_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct TelemetryPacket {
    pub stats: EngineStats,
    pub best_graph: GenomeGraph,
    pub formula: String,
    pub spatial_layout: Vec<darwin_core::genome::NodePosition3D>,
    pub voxels: Vec<[i32; 4]>,
    pub is_running: bool,
    pub current_challenge: String,
    pub current_challenge_name: String,
    pub hall_of_fame: Vec<DiscoveryRecord>,
    pub delay_ms: u64,
}

pub struct ServerState {
    pub is_running: Arc<AtomicBool>,
    pub delay_ms: Arc<AtomicU64>,
    pub target_challenge: Arc<Mutex<String>>,
    pub hall_of_fame: Arc<Mutex<Vec<DiscoveryRecord>>>,
    pub tx: broadcast::Sender<String>,
}

pub async fn run_server(port: u16, state: Arc<ServerState>) {
    let addr = format!("0.0.0.0:{}", port);
    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error al vincular el servidor en {}: {}", addr, e);
            return;
        }
    };

    println!("🌐 Cockpit Visual de DarwinCode disponible en: http://localhost:{}", port);
    println!("   (Puedes abrir este enlace en el navegador de tu móvil)");

    loop {
        let (mut socket, _) = match listener.accept().await {
            Ok(s) => s,
            Err(_) => continue,
        };

        let state_clone = state.clone();

        tokio::spawn(async move {
            let mut buf = [0u8; 4096];
            let n = match socket.read(&mut buf).await {
                Ok(n) if n > 0 => n,
                _ => return,
            };

            let req_str = String::from_utf8_lossy(&buf[..n]);
            let first_line = req_str.lines().next().unwrap_or("");
            let parts: Vec<&str> = first_line.split_whitespace().collect();

            if parts.len() < 2 {
                return;
            }

            let method = parts[0];
            let raw_path = parts[1];
            let path = raw_path.split('?').next().unwrap_or(raw_path);

            if method == "GET" || method == "HEAD" {
                match path {
                    "/sandbox" | "/sandbox.html" => {
                        let content = match std::fs::read_to_string("web/sandbox.html") {
                            Ok(c) => c,
                            Err(_) => include_str!("../../web/sandbox.html").to_string(),
                        };
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/" | "/index.html" => {
                        let content = include_str!("../../web/index.html");
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/style.css" => {
                        let content = include_str!("../../web/style.css");
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/css\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/app.js" => {
                        let content = include_str!("../../web/app.js");
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/renderer_3d.js" => {
                        let content = include_str!("../../web/renderer_3d.js");
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/three.min.js" => {
                        let content = include_str!("../../web/three.min.js");
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/dog" | "/dog.html" => {
                        let content = match std::fs::read_to_string("web/dog_3d_viewer.html") {
                            Ok(c) => c,
                            Err(_) => include_str!("../../web/dog_3d_viewer.html").to_string(),
                        };
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/dog.obj" => {
                        let content = match std::fs::read_to_string("web/dog.obj") {
                            Ok(c) => c,
                            Err(_) => "".to_string(),
                        };
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Disposition: attachment; filename=\"dog.obj\"\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/city" | "/city.html" => {
                        let content = match std::fs::read_to_string("web/city.html") {
                            Ok(c) => c,
                            Err(_) => include_str!("../../web/city.html").to_string(),
                        };
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/city_engine.js" => {
                        let content = match std::fs::read_to_string("web/city_engine.js") {
                            Ok(c) => c,
                            Err(_) => include_str!("../../web/city_engine.js").to_string(),
                        };
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/sdf" | "/sdf.html" | "/sdf_viewer.html" => {
                        let content = match std::fs::read_to_string("web/sdf_viewer.html") {
                            Ok(c) => c,
                            Err(_) => include_str!("../../web/sdf_viewer.html").to_string(),
                        };
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/cppn" | "/cppn.html" | "/cppn_viewer.html" => {
                        let content = match std::fs::read_to_string("web/cppn_viewer.html") {
                            Ok(c) => c,
                            Err(_) => include_str!("../../web/cppn_viewer.html").to_string(),
                        };
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/evolved_cppn.obj" => {
                        let content = match std::fs::read_to_string("web/evolved_cppn.obj") {
                            Ok(c) => c,
                            Err(_) => "".to_string(),
                        };
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Disposition: attachment; filename=\"evolved_cppn.obj\"\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/evolved_cppn.json" => {
                        let content = match std::fs::read_to_string("web/evolved_cppn.json") {
                            Ok(c) => c,
                            Err(_) => "{}".to_string(),
                        };
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            content.len(), content
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/events" => {
                        let header = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\nAccess-Control-Allow-Origin: *\r\n\r\n";
                        if socket.write_all(header.as_bytes()).await.is_err() {
                            return;
                        }

                        let mut rx = state_clone.tx.subscribe();
                        while let Ok(msg) = rx.recv().await {
                            let sse_data = format!("data: {}\n\n", msg);
                            if socket.write_all(sse_data.as_bytes()).await.is_err() {
                                break;
                            }
                        }
                    }
                    "/api/fame" => {
                        let fame = state_clone.hall_of_fame.lock().await;
                        let body = serde_json::to_string(&*fame).unwrap_or_else(|_| "[]".to_string());
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            body.len(), body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    _ => {
                        let not_found = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                        let _ = socket.write_all(not_found.as_bytes()).await;
                    }
                }
            } else if method == "POST" {
                match path {
                    "/api/toggle" => {
                        let current = state_clone.is_running.load(Ordering::SeqCst);
                        state_clone.is_running.store(!current, Ordering::SeqCst);
                        let status = !current;
                        let body = format!("{{\"running\": {}}}", status);
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            body.len(), body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/api/challenge" => {
                        // Buscar el cuerpo JSON
                        if let Some(body_start) = req_str.find("\r\n\r\n") {
                            let body_str = &req_str[body_start + 4..];
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str) {
                                if let Some(cid) = val.get("challenge_id").and_then(|v| v.as_str()) {
                                    let mut lock = state_clone.target_challenge.lock().await;
                                    *lock = cid.to_string();
                                }
                            }
                        }
                        let res_body = "{\"status\": \"ok\"}";
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            res_body.len(), res_body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/api/speed" => {
                        if let Some(body_start) = req_str.find("\r\n\r\n") {
                            let body_str = &req_str[body_start + 4..];
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str) {
                                if let Some(ms) = val.get("delay_ms").and_then(|v| v.as_u64()) {
                                    state_clone.delay_ms.store(ms, Ordering::SeqCst);
                                }
                            }
                        }
                        let res_body = "{\"status\": \"ok\"}";
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            res_body.len(), res_body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/api/challenge/ramanujan" => {
                        let mut params = RamanujanParams { constant: None, max_depth: None };
                        if let Some(body_start) = req_str.find("\r\n\r\n") {
                            let body_str = &req_str[body_start + 4..];
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str) {
                                params.constant = val.get("constant").and_then(|v| v.as_str()).map(|s| s.to_string());
                                params.max_depth = val.get("depth").and_then(|v| v.as_u64()).map(|d| d as usize);
                            }
                        }
                        let res_obj = execute_ramanujan_solve(params).unwrap_or_else(|_| {
                            execute_ramanujan_solve(RamanujanParams {
                                constant: Some("catalan".to_string()),
                                max_depth: Some(25),
                            }).unwrap()
                        });
                        let body = serde_json::to_string(&res_obj).unwrap_or_else(|_| "{}".to_string());
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            body.len(), body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/api/challenge/srsd" => {
                        let mut params = SrsdParams { num_samples: None, noise_level: None };
                        if let Some(body_start) = req_str.find("\r\n\r\n") {
                            let body_str = &req_str[body_start + 4..];
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str) {
                                params.num_samples = val.get("samples").and_then(|v| v.as_u64()).map(|s| s as usize);
                                params.noise_level = val.get("noise").and_then(|v| v.as_f64());
                            }
                        }
                        let res_obj = execute_srsd_hard_solve(params).unwrap_or_else(|_| {
                            execute_srsd_hard_solve(SrsdParams {
                                num_samples: Some(50),
                                noise_level: Some(0.0),
                            }).unwrap()
                        });
                        let body = serde_json::to_string(&res_obj).unwrap_or_else(|_| "{}".to_string());
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            body.len(), body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/api/challenge/oeis" => {
                        let mut params = OeisParams { sequence_id: None, num_training_terms: None };
                        if let Some(body_start) = req_str.find("\r\n\r\n") {
                            let body_str = &req_str[body_start + 4..];
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str) {
                                params.sequence_id = val.get("sequence_id").and_then(|v| v.as_str()).map(|s| s.to_string());
                                params.num_training_terms = val.get("training_terms").and_then(|v| v.as_u64()).map(|t| t as usize);
                            }
                        }
                        let res_obj = execute_oeis_discover(params).unwrap_or_else(|_| {
                            execute_oeis_discover(OeisParams {
                                sequence_id: Some("A000108".to_string()),
                                num_training_terms: Some(6),
                            }).unwrap()
                        });
                        let body = serde_json::to_string(&res_obj).unwrap_or_else(|_| "{}".to_string());
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            body.len(), body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/api/challenge/bsd" => {
                        let mut params = BsdParams { curve_a: None, curve_b: None };
                        if let Some(body_start) = req_str.find("\r\n\r\n") {
                            let body_str = &req_str[body_start + 4..];
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str) {
                                params.curve_a = val.get("curve_a").and_then(|v| v.as_i64());
                                params.curve_b = val.get("curve_b").and_then(|v| v.as_i64());
                            }
                        }
                        let res_obj = execute_bsd_analysis(params).unwrap_or_else(|_| {
                            execute_bsd_analysis(BsdParams { curve_a: Some(-1), curve_b: Some(0) }).unwrap()
                        });
                        let body = serde_json::to_string(&res_obj).unwrap_or_else(|_| "{}".to_string());
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            body.len(), body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/api/challenge/navier_stokes" => {
                        let mut params = NavierStokesParams { grid_size: None, viscosity: None, steps: None, dt: None };
                        if let Some(body_start) = req_str.find("\r\n\r\n") {
                            let body_str = &req_str[body_start + 4..];
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str) {
                                params.grid_size = val.get("grid_size").and_then(|v| v.as_u64()).map(|s| s as usize);
                                params.viscosity = val.get("viscosity").and_then(|v| v.as_f64());
                                params.steps = val.get("steps").and_then(|v| v.as_u64()).map(|s| s as usize);
                                params.dt = val.get("dt").and_then(|v| v.as_f64());
                            }
                        }
                        let res_obj = execute_navier_stokes_blowup(params).unwrap_or_else(|_| {
                            execute_navier_stokes_blowup(NavierStokesParams { grid_size: Some(64), viscosity: Some(0.0), steps: Some(50), dt: Some(0.001) }).unwrap()
                        });
                        let body = serde_json::to_string(&res_obj).unwrap_or_else(|_| "{}".to_string());
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            body.len(), body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/api/challenge/pi_cf" => {
                        let mut params = PiCfParams { depth: None, sequence_type: None };
                        if let Some(body_start) = req_str.find("\r\n\r\n") {
                            let body_str = &req_str[body_start + 4..];
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str) {
                                params.depth = val.get("depth").and_then(|v| v.as_u64()).map(|d| d as usize);
                                params.sequence_type = val.get("sequence_type").and_then(|v| v.as_str()).map(|s| s.to_string());
                            }
                        }
                        let res_obj = execute_pi_cf(params).unwrap_or_else(|_| {
                            execute_pi_cf(PiCfParams { depth: Some(25), sequence_type: Some("ramanujan".to_string()) }).unwrap()
                        });
                        let body = serde_json::to_string(&res_obj).unwrap_or_else(|_| "{}".to_string());
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            body.len(), body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/api/challenge/pi_billiards" => {
                        let mut params = PiBilliardsParams { mass_ratio: None, log_mass: None, max_collisions: None };
                        if let Some(body_start) = req_str.find("\r\n\r\n") {
                            let body_str = &req_str[body_start + 4..];
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str) {
                                params.mass_ratio = val.get("mass_ratio").and_then(|v| v.as_f64());
                                params.log_mass = val.get("log_mass").and_then(|v| v.as_u64()).map(|n| n as u32);
                                params.max_collisions = val.get("max_collisions").and_then(|v| v.as_u64());
                            }
                        }
                        let res_obj = execute_pi_billiards(params).unwrap_or_else(|_| {
                            execute_pi_billiards(PiBilliardsParams { mass_ratio: Some(10000.0), log_mass: None, max_collisions: Some(100_000) }).unwrap()
                        });
                        let body = serde_json::to_string(&res_obj).unwrap_or_else(|_| "{}".to_string());
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            body.len(), body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    "/api/challenge/pi_spigot" => {
                        let mut params = PiSpigotParams { num_digits: None };
                        if let Some(body_start) = req_str.find("\r\n\r\n") {
                            let body_str = &req_str[body_start + 4..];
                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body_str) {
                                params.num_digits = val.get("num_digits").and_then(|v| v.as_u64()).map(|n| n as usize);
                            }
                        }
                        let res_obj = execute_pi_spigot(params).unwrap_or_else(|_| {
                            execute_pi_spigot(PiSpigotParams { num_digits: Some(20) }).unwrap()
                        });
                        let body = serde_json::to_string(&res_obj).unwrap_or_else(|_| "{}".to_string());
                        let res = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{}",
                            body.len(), body
                        );
                        let _ = socket.write_all(res.as_bytes()).await;
                    }
                    _ => {
                        let not_found = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                        let _ = socket.write_all(not_found.as_bytes()).await;
                    }
                }
            }
        });
    }
}
