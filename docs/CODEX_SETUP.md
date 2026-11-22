# Integración de DarwinCode con OpenAI Codex, Cursor & LibreChat

DarwinCode implementa la especificación estándar **Model Context Protocol (MCP)** mediante transporte `stdio` (JSON-RPC 2.0). Cualquier cliente o agente compatible puede consumirlo.

---

## 1. Conexión en Cursor IDE

En Cursor:
1. Abre **Settings** -> **Features** -> **MCP Servers**.
2. Haz clic en **Add New MCP Server**.
3. Configuración:
   - **Name:** `darwin-code`
   - **Type:** `stdio`
   - **Command:** `/data/data/com.termux/files/home/.cargo_target/release/darwin_cli --mcp`
4. Guarda y activa las herramientas.

---

## 2. Conexión en LibreChat o Clientes WebMCP

En tu archivo `librechat.yaml`:

```yaml
mcpServers:
  darwin-code:
    type: stdio
    command: /data/data/com.termux/files/home/.cargo_target/release/darwin_cli
    args:
      - "--mcp"
```

---

## 3. Uso en Scripts de Python con Codex / OpenAI SDK

```python
import subprocess
import json

def call_darwin_tool(tool_name: str, arguments: dict):
    process = subprocess.Popen(
        ["/data/data/com.termux/files/home/.cargo_target/release/darwin_cli", "--mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # 1. Enviar Initialize
    init_req = {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}
    process.stdin.write(json.dumps(init_req) + "\n")
    process.stdin.flush()
    _ = process.stdout.readline()

    # 2. Enviar Tool Call
    call_req = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {"name": tool_name, "arguments": arguments}
    }
    process.stdin.write(json.dumps(call_req) + "\n")
    process.stdin.flush()
    res_line = process.stdout.readline()
    
    process.terminate()
    return json.loads(res_line)

# Ejemplo de uso:
resultado = call_darwin_tool("darwin_synthesize", {
    "dataset": [
        {"inputs": [-2.0], "expected": 5.0},
        {"inputs": [0.0], "expected": 1.0},
        {"inputs": [2.0], "expected": 5.0}
    ]
})
print(resultado["result"]["content"][0]["text"])
```
