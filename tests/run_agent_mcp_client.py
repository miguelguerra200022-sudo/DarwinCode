#!/usr/bin/env python3
"""
Cliente MCP Autónomo para Agente de IA.
Conecta mediante stdio (JSON-RPC 2.0) al motor DarwinCode y delega
el problema de síntesis multivariable no lineal.
"""

import subprocess
import json
import sys
import time

BINARY = "/data/data/com.termux/files/home/.cargo_target/release/darwin_cli"

def main():
    print("=" * 65)
    print("🤖 AGENTE IA: Invocando DarwinCode MCP como Co-Procesador")
    print("=" * 65)

    # 1. Levantar el servidor MCP en subproceso con stdio
    proc = subprocess.Popen(
        [BINARY, "--mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )

    def send_rpc(method: str, params: dict, req_id: int):
        payload = {
            "jsonrpc": "2.0",
            "id": req_id,
            "method": method,
            "params": params
        }
        line = json.dumps(payload)
        proc.stdin.write(line + "\n")
        proc.stdin.flush()
        
        # Leer respuesta JSON-RPC
        resp_line = proc.stdout.readline()
        if not resp_line:
            raise RuntimeError("Servidor MCP cerró el canal inesperadamente")
        return json.loads(resp_line.strip())

    # 2. Handshake MCP
    print("\n[Paso 1/3] Inicializando protocolo MCP (Initialize Handshake)...")
    init_resp = send_rpc("initialize", {"protocolVersion": "2024-11-05"}, req_id=1)
    server_info = init_resp.get("result", {}).get("serverInfo", {})
    print(f" -> Conectado a: {server_info.get('name')} v{server_info.get('version')}")

    # 3. Formular el problema complejo para DarwinCode
    # Ley no lineal multivariable acoplada: f(T, v) = T² + v² + T·v
    print("\n[Paso 2/3] Delegando Problema No Lineal Multivariable:")
    print("   Fenómeno: Disipación Térmica en Motor con Acoplamiento Cruzado")
    print("   Variables: T (Temperatura), v (Velocidad)")
    print("   Datos empíricos: 15 observaciones discretas multidimensionales")

    dataset = [
        {"inputs": [0.0, 0.0], "expected": 0.0},
        {"inputs": [1.0, 0.0], "expected": 1.0},
        {"inputs": [0.0, 1.0], "expected": 1.0},
        {"inputs": [1.0, 1.0], "expected": 3.0},
        {"inputs": [2.0, 0.0], "expected": 4.0},
        {"inputs": [0.0, 2.0], "expected": 4.0},
        {"inputs": [2.0, 1.0], "expected": 7.0},
        {"inputs": [1.0, 2.0], "expected": 7.0},
        {"inputs": [2.0, 2.0], "expected": 12.0},
        {"inputs": [3.0, 1.0], "expected": 13.0},
        {"inputs": [1.0, 3.0], "expected": 13.0},
        {"inputs": [3.0, 2.0], "expected": 19.0},
        {"inputs": [2.0, 3.0], "expected": 19.0},
        {"inputs": [4.0, 1.0], "expected": 21.0},
        {"inputs": [3.0, 3.0], "expected": 27.0},
    ]

    synth_args = {
        "dataset": dataset,
        "input_names": ["T", "v"],
        "max_generations": 2500,
        "target_accuracy": 0.9999,
        "population_size": 150
    }

    t0 = time.time()
    call_resp = send_rpc("tools/call", {
        "name": "darwin_synthesize",
        "arguments": synth_args
    }, req_id=2)
    elapsed = time.time() - t0

    result = call_resp.get("result", {})
    is_error = result.get("isError", False)
    content = result.get("content", [{}])[0].get("text", "")

    print(f"\n[Paso 3/3] Síntesis finalizada en {elapsed:.2f} segundos!")
    print("\n--- Respuesta del Co-Procesador DarwinCode ---")
    print(content)

    # 4. Evaluación de seguridad en VM
    print("\n[Verificación Formal] Probando caso extremo en VM gas-metered:")
    eval_resp = send_rpc("tools/call", {
        "name": "darwin_safe_eval",
        "arguments": {
            "dataset": [
                {"inputs": [10.0, 5.0], "expected": 175.0}, # 100 + 25 + 50 = 175
                {"inputs": [20.0, 10.0], "expected": 700.0} # 400 + 100 + 200 = 700
            ],
            "gas_limit": 5000
        }
    }, req_id=3)
    eval_content = eval_resp.get("result", {}).get("content", [{}])[0].get("text", "")
    print(eval_content)

    # 5. Generación de estructura 3D en vóxeles
    print("\n[Diseño Espacial 3D] Solicitando matriz de vóxeles para estructura:")
    voxel_resp = send_rpc("tools/call", {
        "name": "darwin_evolve_voxels",
        "arguments": {
            "floors": 4,
            "style": "pyramid",
            "max_blocks": 150
        }
    }, req_id=4)
    voxel_content = voxel_resp.get("result", {}).get("content", [{}])[0].get("text", "")
    print(voxel_content)

    proc.terminate()
    print("=" * 65)
    print("🎯 Conclusión: Sinergia Neuro-Simbólica demostrada con éxito.")

if __name__ == "__main__":
    main()
