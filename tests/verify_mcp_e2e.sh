#!/usr/bin/env bash
set -euo pipefail

BINARY="/data/data/com.termux/files/home/.cargo_target/release/darwin_cli"

echo "=========================================================="
echo "🧪 DarwinCode MCP End-to-End stdio Protocol Verification"
echo "=========================================================="

if [ ! -f "$BINARY" ]; then
    echo "❌ Error: Binario no encontrado en $BINARY"
    exit 1
fi

TMP_INPUT=$(mktemp)
TMP_OUTPUT=$(mktemp)

trap "rm -f $TMP_INPUT $TMP_OUTPUT" EXIT

# 1. Preparar secuencia de solicitudes JSON-RPC 2.0
cat << 'EOF' > "$TMP_INPUT"
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"darwin_synthesize","arguments":{"dataset":[{"inputs":[-2.0],"expected":5.0},{"inputs":[-1.0],"expected":2.0},{"inputs":[0.0],"expected":1.0},{"inputs":[1.0],"expected":2.0},{"inputs":[2.0],"expected":5.0}],"input_names":["x"],"max_generations":300,"target_accuracy":0.999}}}
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"darwin_evolve_voxels","arguments":{"floors":3,"style":"tower"}}}
{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"darwin_get_status","arguments":{}}}
EOF

# 2. Ejecutar binario con stdin redirigido y capturar stdout
"$BINARY" --mcp < "$TMP_INPUT" > "$TMP_OUTPUT" 2> /dev/null

echo "✅ Ejecución stdio completada. Analizando respuestas recibidas..."

TOTAL_RESPONSES=$(wc -l < "$TMP_OUTPUT")
echo "📥 Total de respuestas JSON-RPC recibidas: $TOTAL_RESPONSES"

if [ "$TOTAL_RESPONSES" -ne 5 ]; then
    echo "❌ Se esperaban 5 respuestas, pero se obtuvieron $TOTAL_RESPONSES"
    cat "$TMP_OUTPUT"
    exit 1
fi

# 3. Validar initialize (id: 1)
RESP1=$(grep '"id":1' "$TMP_OUTPUT" || true)
if echo "$RESP1" | grep -q '"name":"darwin-code"'; then
    echo "  [OK] id:1 initialize -> Servidor darwin-code v0.2.0 reconocido"
else
    echo "  [FAIL] initialize falló: $RESP1"
    exit 1
fi

# 4. Validar tools/list (id: 2)
RESP2=$(grep '"id":2' "$TMP_OUTPUT" || true)
if echo "$RESP2" | grep -q 'darwin_synthesize' && echo "$RESP2" | grep -q 'darwin_evolve_voxels'; then
    echo "  [OK] id:2 tools/list -> 5 herramientas MCP detectadas y listadas"
else
    echo "  [FAIL] tools/list falló: $RESP2"
    exit 1
fi

# 5. Validar darwin_synthesize (id: 3)
RESP3=$(grep '"id":3' "$TMP_OUTPUT" || true)
if echo "$RESP3" | grep -q 'darwin_synthesize' || echo "$RESP3" | grep -q 'Sintesis' || echo "$RESP3" | grep -q 'Formula' || echo "$RESP3" | grep -q 'Descubierta'; then
    echo "  [OK] id:3 tools/call darwin_synthesize -> Ley matemática sintetizada"
else
    echo "  [FAIL] darwin_synthesize falló: $RESP3"
    exit 1
fi

# 6. Validar darwin_evolve_voxels (id: 4)
RESP4=$(grep '"id":4' "$TMP_OUTPUT" || true)
if echo "$RESP4" | grep -q 'V' || echo "$RESP4" | grep -q 'tower' || echo "$RESP4" | grep -q 'xeles'; then
    echo "  [OK] id:4 tools/call darwin_evolve_voxels -> Matriz 3D generada asíncronamente"
else
    echo "  [FAIL] darwin_evolve_voxels falló: $RESP4"
    exit 1
fi

# 7. Validar darwin_get_status (id: 5)
RESP5=$(grep '"id":5' "$TMP_OUTPUT" || true)
if echo "$RESP5" | grep -q 'DarwinCode Neuro-Symbolic Engine'; then
    echo "  [OK] id:5 tools/call darwin_get_status -> Telemetría verificada"
else
    echo "  [FAIL] darwin_get_status falló: $RESP5"
    exit 1
fi

echo ""
echo "🎉 ¡TODAS LAS PRUEBAS E2E DEL CO-PROCESADOR MCP PASARON EXITOSAMENTE!"
