use darwin_cli::mcp_protocol::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, ToolCallResult,
};
use serde_json::json;

#[test]
fn test_parse_initialize_request() {
    let raw = r#"{
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "claude-code",
                "version": "1.0.0"
            }
        }
    }"#;

    let req: JsonRpcRequest = serde_json::from_str(raw).expect("Failed to parse request");
    assert_eq!(req.jsonrpc, "2.0");
    assert_eq!(req.id, Some(json!(1)));
    assert_eq!(req.method, "initialize");
    assert!(req.params.is_some());
}

#[test]
fn test_create_initialize_response() {
    let resp = JsonRpcResponse::success(
        Some(json!(1)),
        json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "darwin-code",
                "version": "0.2.0"
            }
        }),
    );

    let serialized = serde_json::to_string(&resp).expect("Failed to serialize");
    assert!(serialized.contains(r#""jsonrpc":"2.0""#));
    assert!(serialized.contains(r#""darwin-code""#));
}

#[test]
fn test_create_error_response() {
    let err = JsonRpcError::method_not_found("Unknown method 'foobar'");
    let resp = JsonRpcResponse::error(Some(json!(42)), err);

    let serialized = serde_json::to_string(&resp).expect("Failed to serialize");
    assert!(serialized.contains(r#""code":-32601"#));
    assert!(serialized.contains("Unknown method 'foobar'"));
}

#[test]
fn test_tool_call_result_formatting() {
    let res = ToolCallResult::text("Result: x^2 + 1", false);
    let val = serde_json::to_value(&res).expect("to_value failed");
    assert_eq!(val["isError"], false);
    assert_eq!(val["content"][0]["type"], "text");
    assert_eq!(val["content"][0]["text"], "Result: x^2 + 1");
}
