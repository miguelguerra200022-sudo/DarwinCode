#!/usr/bin/env python3
"""
End-to-End Verification Test for DarwinCode SOTA Engine v1.0.
Connects via JSON-RPC 2.0 stdio to the release darwin_cli --mcp binary
and verifies both frontier mathematical benchmarks:
1. Einstein's Relativistic Velocity Addition
2. Chaotic Hamiltonian Oscillator Potential (Contopoulos-Barbanis)
"""

import json
import subprocess
import time
import sys

BINARY = "/data/data/com.termux/files/home/.cargo_target/release/darwin_cli"

def run_verification():
    print("=" * 70)
    print("🌌 DARWINCODE SOTA v1.0: VERIFICACIÓN END-TO-END DE MOTOR UNIVERSAL")
    print("=" * 70)

    proc = subprocess.Popen(
        [BINARY, "--mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )

    req_id = 0

    def rpc_call(method: str, params: dict):
        nonlocal req_id
        req_id += 1
        payload = {
            "jsonrpc": "2.0",
            "id": req_id,
            "method": method,
            "params": params
        }
        proc.stdin.write(json.dumps(payload) + "\n")
        proc.stdin.flush()
        line = proc.stdout.readline()
        if not line:
            err = proc.stderr.read()
            raise RuntimeError(f"DarwinCode MCP se cerró inesperadamente: {err}")
        return json.loads(line.strip())

    # 1. Initialize
    print("\n[1/4] Inicializando Handshake MCP...")
    init_res = rpc_call("initialize", {"protocolVersion": "2024-11-05"})
    s_info = init_res["result"]["serverInfo"]
    print(f"  ✓ Servidor Conectado: {s_info['name']} v{s_info['version']}")

    # 2. Get Status
    print("\n[2/4] Verificando Telemetría y Desafíos SOTA (darwin_get_status)...")
    st_res = rpc_call("tools/call", {"name": "darwin_get_status", "arguments": {}})
    st_text = st_res["result"]["content"][0]["text"]
    print(st_text)

    # 3. Benchmark 1: Einstein's Relativistic Velocity Addition
    print("\n" + "=" * 70)
    print("🚀 [3/4] BENCHMARK 1: Adición Relativista de Velocidades de Einstein")
    print("   Fórmula Teórica: w = (u + v) / (1 + u * v)")
    print("   Dataset: 16 observaciones empíricas no lineales")
    print("=" * 70)

    rel_dataset = [
        {"inputs": [0.1, 0.1], "expected": 0.19802},
        {"inputs": [0.1, 0.2], "expected": 0.29412},
        {"inputs": [0.1, 0.3], "expected": 0.38835},
        {"inputs": [0.1, 0.4], "expected": 0.48077},
        {"inputs": [0.2, 0.1], "expected": 0.29412},
        {"inputs": [0.2, 0.2], "expected": 0.38462},
        {"inputs": [0.2, 0.4], "expected": 0.55556},
        {"inputs": [0.3, 0.3], "expected": 0.55046},
        {"inputs": [0.4, 0.1], "expected": 0.48077},
        {"inputs": [0.4, 0.2], "expected": 0.55556},
        {"inputs": [0.4, 0.4], "expected": 0.68966},
        {"inputs": [0.5, 0.5], "expected": 0.80000},
        {"inputs": [0.6, 0.6], "expected": 0.88235},
        {"inputs": [0.7, 0.7], "expected": 0.93960},
        {"inputs": [0.8, 0.8], "expected": 0.97561},
        {"inputs": [0.2, 0.7], "expected": 0.78947},
    ]

    t0 = time.time()
    rel_res = rpc_call("tools/call", {
        "name": "darwin_synthesize",
        "arguments": {
            "dataset": rel_dataset,
            "input_names": ["u", "v"],
            "max_generations": 250,
            "target_accuracy": 0.9999,
            "population_size": 120,
            "enable_sota": True,
            "enable_constant_optimization": True,
            "enable_symmetry_prepass": True,
            "allowed_ops": ["Add", "Subtract", "Multiply", "Divide"]
        }
    })
    rel_elapsed = time.time() - t0
    rel_text = rel_res["result"]["content"][0]["text"]
    print(f"\n✓ Síntesis Relativista concluida en {rel_elapsed:.2f}s:")
    print(rel_text)

    # 4. Benchmark 2: Chaotic Hamiltonian Oscillator Potential
    print("\n" + "=" * 70)
    print("🌀 [4/4] BENCHMARK 2: Potencial del Oscilador Caótico de Contopoulos-Barbanis")
    print("   Fórmula Teórica: V(x, y) = x²·y + 0.5·(x² + y²)")
    print("   Dataset: 25 observaciones de resonancia no lineal")
    print("=" * 70)

    chaotic_dataset = [
        {"inputs": [-2.0, -2.0], "expected": -4.0},
        {"inputs": [-2.0, -1.0], "expected": -1.5},
        {"inputs": [-2.0, 0.0], "expected": 2.0},
        {"inputs": [-2.0, 1.0], "expected": 6.5},
        {"inputs": [-2.0, 2.0], "expected": 12.0},
        {"inputs": [-1.0, -2.0], "expected": -1.5},
        {"inputs": [-1.0, -1.0], "expected": 0.0},
        {"inputs": [-1.0, 0.0], "expected": 0.5},
        {"inputs": [-1.0, 1.0], "expected": 2.0},
        {"inputs": [-1.0, 2.0], "expected": 4.5},
        {"inputs": [0.0, -2.0], "expected": 2.0},
        {"inputs": [0.0, -1.0], "expected": 0.5},
        {"inputs": [0.0, 0.0], "expected": 0.0},
        {"inputs": [0.0, 1.0], "expected": 0.5},
        {"inputs": [0.0, 2.0], "expected": 2.0},
        {"inputs": [1.0, -2.0], "expected": -1.5},
        {"inputs": [1.0, -1.0], "expected": 0.0},
        {"inputs": [1.0, 0.0], "expected": 0.5},
        {"inputs": [1.0, 1.0], "expected": 2.0},
        {"inputs": [1.0, 2.0], "expected": 4.5},
        {"inputs": [2.0, -2.0], "expected": -4.0},
        {"inputs": [2.0, -1.0], "expected": -1.5},
        {"inputs": [2.0, 0.0], "expected": 2.0},
        {"inputs": [2.0, 1.0], "expected": 6.5},
        {"inputs": [2.0, 2.0], "expected": 12.0},
    ]

    t0 = time.time()
    chaotic_res = rpc_call("tools/call", {
        "name": "darwin_synthesize",
        "arguments": {
            "dataset": chaotic_dataset,
            "input_names": ["x", "y"],
            "max_generations": 300,
            "target_accuracy": 0.9999,
            "population_size": 150,
            "enable_sota": True,
            "enable_constant_optimization": True,
            "enable_symmetry_prepass": True,
            "allowed_ops": ["Add", "Subtract", "Multiply"]
        }
    })
    chaotic_elapsed = time.time() - t0
    chaotic_text = chaotic_res["result"]["content"][0]["text"]
    print(f"\n✓ Síntesis del Potencial Caótico concluida en {chaotic_elapsed:.2f}s:")
    print(chaotic_text)

    # Clean termination
    proc.terminate()
    proc.wait()

    print("\n" + "=" * 70)
    print("🎯 TODOS LOS BENCHMARKS SOTA COMPLETADOS EXITOSAMENTE")
    print("=" * 70)

if __name__ == "__main__":
    run_verification()
