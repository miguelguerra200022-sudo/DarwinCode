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
    assert_eq!(tools.len(), 12);

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
