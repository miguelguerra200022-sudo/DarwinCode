use darwin_cli::mcp::handle_jsonrpc_request;
use darwin_cli::mcp_protocol::JsonRpcRequest;
use serde_json::json;

#[tokio::test]
async fn test_mcp_handle_initialize() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: Some(json!({
            "protocolVersion": "2024-11-05"
        })),
    };

    let resp = handle_jsonrpc_request(req).await.expect("Expected response");
    assert_eq!(resp.id, Some(json!(1)));
    let result = resp.result.expect("Expected result");
    assert_eq!(result["serverInfo"]["name"], "darwin-code");
    assert_eq!(result["serverInfo"]["version"], "1.0.0");
}

#[tokio::test]
async fn test_mcp_handle_tools_list() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(2)),
        method: "tools/list".to_string(),
        params: None,
    };

    let resp = handle_jsonrpc_request(req).await.expect("Expected response");
    let result = resp.result.expect("Expected result");
    let tools = result["tools"].as_array().expect("Tools must be an array");
    assert_eq!(tools.len(), 15);

    let names: Vec<&str> = tools
        .iter()
        .map(|t| t["name"].as_str().expect("name must be string"))
        .collect();
    assert!(names.contains(&"darwin_synthesize"));
    assert!(names.contains(&"darwin_optimize_code"));
    assert!(names.contains(&"darwin_evolve_voxels"));
    assert!(names.contains(&"darwin_evolve_cppn"));
    assert!(names.contains(&"darwin_generate_city"));
    assert!(names.contains(&"darwin_ramanujan_solve"));
    assert!(names.contains(&"darwin_srsd_hard_solve"));
    assert!(names.contains(&"darwin_oeis_discover"));
    assert!(names.contains(&"darwin_bsd_analysis"));
    assert!(names.contains(&"darwin_navier_stokes_blowup"));
    assert!(names.contains(&"darwin_safe_eval"));
    assert!(names.contains(&"darwin_get_status"));
    assert!(names.contains(&"darwin_pi_continued_fraction"));
    assert!(names.contains(&"darwin_pi_billiards_simulate"));
    assert!(names.contains(&"darwin_pi_spigot_extract"));
}

#[tokio::test]
async fn test_mcp_handle_tool_call_status() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(3)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "darwin_get_status",
            "arguments": {}
        })),
    };

    let resp = handle_jsonrpc_request(req).await.expect("Expected response");
    let result = resp.result.expect("Expected result");
    assert_eq!(result["isError"], false);
    let text = result["content"][0]["text"].as_str().expect("text expected");
    assert!(text.contains("darwin-code"));
}

#[tokio::test]
async fn test_mcp_handle_tool_call_pi_triad() {
    // 1. Fracción continua Pi
    let req_cf = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(10)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "darwin_pi_continued_fraction",
            "arguments": { "depth": 10, "sequence_type": "ramanujan" }
        })),
    };
    let resp_cf = handle_jsonrpc_request(req_cf).await.expect("CF resp");
    let res_cf = resp_cf.result.expect("CF result");
    assert_eq!(res_cf["isError"], false);
    let text_cf = res_cf["content"][0]["text"].as_str().unwrap();
    assert!(text_cf.contains("Fracción Continua Recurrente"));

    // 2. Billares Galperin Pi
    let req_bil = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(11)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "darwin_pi_billiards_simulate",
            "arguments": { "mass_ratio": 10000.0 }
        })),
    };
    let resp_bil = handle_jsonrpc_request(req_bil).await.expect("Billiards resp");
    let res_bil = resp_bil.result.expect("Billiards result");
    assert_eq!(res_bil["isError"], false);
    let text_bil = res_bil["content"][0]["text"].as_str().unwrap();
    assert!(text_bil.contains("314"));

    // 3. Extractor Spigot Pi
    let req_spigot = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(12)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "darwin_pi_spigot_extract",
            "arguments": { "num_digits": 10 }
        })),
    };
    let resp_spigot = handle_jsonrpc_request(req_spigot).await.expect("Spigot resp");
    let res_spigot = resp_spigot.result.expect("Spigot result");
    assert_eq!(res_spigot["isError"], false);
    let text_spigot = res_spigot["content"][0]["text"].as_str().unwrap();
    assert!(text_spigot.contains("141592653"));
}

#[tokio::test]
async fn test_mcp_handle_unknown_method() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(99)),
        method: "non_existent_method".to_string(),
        params: None,
    };

    let resp = handle_jsonrpc_request(req).await.expect("Expected response");
    let err = resp.error.expect("Expected error");
    assert_eq!(err.code, -32601);
}
