# Integración de DarwinCode con Claude Code & Claude Desktop

DarwinCode puede ser utilizado directamente por **Claude Code** (CLI) y **Claude Desktop** como un servidor de herramientas MCP (Model Context Protocol).

---

## 1. Conectar con Claude Code (Línea de Comandos)

Ejecuta el siguiente comando en tu terminal para registrar DarwinCode como servidor MCP local:

```bash
claude mcp add darwin-code -- /data/data/com.termux/files/home/.cargo_target/release/darwin_cli --mcp
```

Para verificar que está conectado y listar las herramientas:

```bash
claude mcp list
```

---

## 2. Conectar con Claude Desktop

Agrega la siguiente configuración a tu archivo `claude_desktop_config.json`:

- **Linux / Android Termux:** `~/.config/Claude/claude_desktop_config.json`
- **macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "darwin-code": {
      "command": "/data/data/com.termux/files/home/.cargo_target/release/darwin_cli",
      "args": ["--mcp"]
    }
  }
}
```

---

## 3. Ejemplo de Interacción en el Chat con Claude

Una vez configurado, puedes pedirle a Claude:

> *"Claude, tengo estas lecturas de un sensor de aceleración: a t=0 y=0, t=1 y=5, t=2 y=20, t=3 y=45. Usa DarwinCode para sintetizar la ley física exacta y verificarla."*

Claude invocará automáticamente `darwin_synthesize` a través de JSON-RPC, evaluará los genomas en microsegundos y responderá con la fórmula $y = 5 \cdot t^2$ demostrada y libre de alucinaciones.
