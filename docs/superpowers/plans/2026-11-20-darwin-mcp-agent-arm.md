# DarwinCode Universal MCP & AI Agent Neuro-Symbolic Arm Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform DarwinCode into a universal MCP (Model Context Protocol) server and neuro-symbolic co-processor for AI coding agents (Claude Code, Google Antigravity, Codex, Cursor), executing high-throughput genetic synthesis, code simplification, 3D voxel generation, and gas-metered sandboxed evaluation.

**Architecture:**
- JSON-RPC 2.0 stdio server running in `darwin_cli --mcp`, strictly separating JSON-RPC output (`stdout`) from telemetry and debug logs (`stderr`).
- High-level MCP tools powered directly by `darwin_core`: `darwin_synthesize`, `darwin_optimize_code`, `darwin_evolve_voxels`, `darwin_safe_eval`, and `darwin_get_status`.
- Dual output presentation (machine-readable JSON + human-readable markdown) conforming to Anthropic & Google MCP specifications.
- Registration in Antigravity MCP directory (`~/.gemini/antigravity-cli/mcp/darwin-code/`) and Antigravity Skill (`~/.gemini/config/skills/darwin-code/SKILL.md`).
- Headless CLI commands (`darwin_cli synthesize ...`) for direct shell script automation.

**Tech Stack:** Rust (Tokio, Serde, Serde-JSON), Model Context Protocol (JSON-RPC 2.0), Linux/Termux.

**Spec:** Transforming DarwinCode into an autonomous neuro-symbolic evolutionary arm for AI agents with 5 core tools, stdio transport, full test coverage, and multi-agent configuration recipes.

## Global Constraints
- Target build directory on Android/Termux must remain `/data/data/com.termux/files/home/.cargo_target` (`.cargo/config.toml`).
- Zero stdout pollution in `--mcp` mode: stdout is strictly reserved for newline-delimited JSON-RPC 2.0 responses.
- Safe execution: all evaluations must be gas-metered with configurable max cycles (prevent infinite loops or out-of-memory).
- Backward compatibility: the 3D Web Cockpit on port 8080 must continue to run when `--mcp` is not specified.

## Review Focus
1. Malformed JSON-RPC or unknown method calls must return standard JSON-RPC 2.0 error codes (`-32600`, `-32601`, `-32602`) rather than crashing the process.
2. Tool execution timeouts / generation ceilings: evolutionary tasks with unattainable precision targets must terminate cleanly at `max_generations` and return the best candidate found.
3. Empty or single-point datasets passed to `darwin_synthesize`: must gracefully validate inputs and return a descriptive error message without panicking.
4. Gas exhaustion in `darwin_safe_eval`: expressions containing cycles or deep trees must cleanly report gas limit exceeded without freezing the agent.
5. Concurrent requests on stdio: sequential line-based reader with asynchronous execution per request ID.

---

### Task 1: Rust MCP Protocol & Serialization Data Models

**Files:**
- Create: `DarwinCode/cli/src/mcp_protocol.rs`
- Modify: `DarwinCode/cli/Cargo.toml`
- Test: `DarwinCode/cli/tests/mcp_protocol_test.rs`

**Interfaces:**
- Produces:
  - `JsonRpcRequest`: `struct { jsonrpc: String, id: Option<serde_json::Value>, method: String, params: Option<serde_json::Value> }`
  - `JsonRpcResponse`: `struct { jsonrpc: String, id: Option<serde_json::Value>, result: Option<serde_json::Value>, error: Option<JsonRpcError> }`
  - `ToolDefinition`: `struct { name: String, description: String, inputSchema: serde_json::Value }`
  - `ToolCallResult`: `struct { content: Vec<ToolContent>, isError: bool }`
  - `ToolContent`: `enum { Text { text: String } }`

- [ ] **Step 1: Write test for MCP JSON-RPC serialization & deserialization**
Create `DarwinCode/cli/tests/mcp_protocol_test.rs` testing parsing of `initialize`, `tools/list`, and `tools/call`.

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test --test mcp_protocol_test`

- [ ] **Step 3: Implement `mcp_protocol.rs`**
Implement standard MCP types, request parsing, response generation, and standard JSON-RPC error codes.

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test mcp_protocol_test`
Expected: PASS.

- [ ] **Step 5: Commit**
```bash
git add cli/src/mcp_protocol.rs cli/tests/mcp_protocol_test.rs
git commit -m "feat(mcp): add JSON-RPC 2.0 MCP data models and protocol types"
```

---

### Task 2: Core Tool Executors (Synthesize, Optimize, Voxels, Safe Eval, Status)

**Files:**
- Create: `DarwinCode/cli/src/mcp_tools.rs`
- Modify: `DarwinCode/core/src/engine.rs` (if auxiliary helpers needed)
- Test: `DarwinCode/cli/tests/mcp_tools_test.rs`

**Interfaces:**
- Consumes: `darwin_core::engine::EvolutionEngine`, `darwin_core::vm::VirtualMachine`, `mcp_protocol::*`
- Produces:
  - `execute_synthesize(params: SynthesizeParams) -> Result<SynthesizeResult, String>`
  - `execute_optimize_code(params: OptimizeParams) -> Result<OptimizeResult, String>`
  - `execute_evolve_voxels(params: VoxelParams) -> Result<VoxelResult, String>`
  - `execute_safe_eval(params: SafeEvalParams) -> Result<SafeEvalResult, String>`
  - `execute_get_status() -> StatusResult`

- [ ] **Step 1: Write integration tests for tool execution**
Test `execute_synthesize` on $x^2 + 1$ dataset, `execute_safe_eval` with gas limit, and `execute_evolve_voxels`.

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test --test mcp_tools_test`

- [ ] **Step 3: Implement `mcp_tools.rs`**
Implement all 5 tool handlers with input validation, timeout/generation limits, markdown formatting, and structured JSON results.

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test mcp_tools_test`
Expected: PASS.

- [ ] **Step 5: Commit**
```bash
git add cli/src/mcp_tools.rs cli/tests/mcp_tools_test.rs
git commit -m "feat(mcp): implement core evolutionary tool handlers"
```

---

### Task 3: MCP Stdio Server & CLI Argument Dispatcher

**Files:**
- Create: `DarwinCode/cli/src/mcp.rs`
- Modify: `DarwinCode/cli/src/main.rs`
- Test: `DarwinCode/cli/tests/mcp_stdio_test.rs`

**Interfaces:**
- Produces:
  - `run_mcp_server() -> Result<(), Box<dyn std::error::Error>>`
  - CLI flag support: `--mcp` for MCP server mode, `--port <P>` for web cockpit, `synthesize` for direct CLI execution.

- [ ] **Step 1: Write stdio integration test**
Simulate reading a newline-delimited stream of JSON-RPC requests via a test pipe and validating responses.

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test --test mcp_stdio_test`

- [ ] **Step 3: Implement `mcp.rs` and update `main.rs`**
Implement the async line-by-line stdio loop in `mcp.rs` using `tokio::io::AsyncBufReadExt` and `tokio::io::AsyncWriteExt`. Wire `args.contains(&"--mcp")` in `main.rs`. Redirect all tracing/logging to `eprintln!`.

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test mcp_stdio_test`
Expected: PASS.

- [ ] **Step 5: Verify build & release binary**
Run: `cargo build --release --bin darwin_cli`

- [ ] **Step 6: Commit**
```bash
git add cli/src/mcp.rs cli/src/main.rs cli/tests/mcp_stdio_test.rs
git commit -m "feat(cli): wire --mcp flag and stdio JSON-RPC server loop"
```

---

### Task 4: Antigravity MCP Server Registration & Schemas

**Files:**
- Create: `/data/data/com.termux/files/home/.gemini/antigravity-cli/mcp/darwin-code/darwin_synthesize.json`
- Create: `/data/data/com.termux/files/home/.gemini/antigravity-cli/mcp/darwin-code/darwin_optimize_code.json`
- Create: `/data/data/com.termux/files/home/.gemini/antigravity-cli/mcp/darwin-code/darwin_evolve_voxels.json`
- Create: `/data/data/com.termux/files/home/.gemini/antigravity-cli/mcp/darwin-code/darwin_safe_eval.json`
- Create: `/data/data/com.termux/files/home/.gemini/antigravity-cli/mcp/darwin-code/darwin_get_status.json`
- Create: `/data/data/com.termux/files/home/.gemini/antigravity-cli/mcp/darwin-code/instructions.md`

- [ ] **Step 1: Create tool schema JSONs**
Generate schemas matching standard MCP parameter specifications for each tool.

- [ ] **Step 2: Create `instructions.md`**
Document when and how AI agents should invoke DarwinCode tools (symbolic regression, algorithm optimization, parsimony de-bloating, 3D voxel assembly).

- [ ] **Step 3: Verify schema loading**
Check that schemas are well-formed JSON and follow the Antigravity MCP directory layout.

---

### Task 5: Antigravity Skill & Universal Multi-Agent Integration Guides

**Files:**
- Create: `DarwinCode/skills/darwin-code/SKILL.md`
- Create: `/data/data/com.termux/files/home/.gemini/config/skills/darwin-code/SKILL.md`
- Create: `DarwinCode/docs/CLAUDE_CODE_SETUP.md`
- Create: `DarwinCode/docs/CODEX_SETUP.md`
- Modify: `DarwinCode/README.md`

- [ ] **Step 1: Create `darwin-code` Antigravity Skill**
Write comprehensive instructions with frontmatter for Antigravity explaining neuro-symbolic synergy, tool parameters, prompt patterns, and best practices.

- [ ] **Step 2: Copy skill to `~/.gemini/config/skills/darwin-code/SKILL.md`**
Make the skill globally available to Antigravity sessions.

- [ ] **Step 3: Create agent setup guides**
Provide configuration snippets for Claude Code (`claude mcp add`), Claude Desktop (`claude_desktop_config.json`), and Codex / Cursor.

- [ ] **Step 4: Update `DarwinCode/README.md`**
Document the new MCP co-processor capabilities and usage examples.

- [ ] **Step 5: Commit**
```bash
git add skills/ docs/ README.md
git commit -m "docs: add official Antigravity skill and multi-agent setup guides"
```

---

### Task 6: End-to-End Verification & Live Demonstration

**Files:**
- Create: `DarwinCode/tests/verify_mcp_e2e.sh`

- [ ] **Step 1: Write E2E stdio test script**
Pipes `initialize`, `tools/list`, and `tools/call` for `darwin_synthesize` into `darwin_cli --mcp` and asserts valid JSON-RPC responses.

- [ ] **Step 2: Execute E2E test script**
Run: `bash DarwinCode/tests/verify_mcp_e2e.sh`
Expected: Discovers formula with 100% precision and returns valid JSON-RPC response with zero errors.

- [ ] **Step 3: Verify Web Cockpit on port 8080**
Ensure `darwin_cli` default mode still boots the web server and streams live telemetry.

- [ ] **Step 4: Final commit and status report**
```bash
git commit -m "chore: complete DarwinCode MCP agent arm implementation"
```
