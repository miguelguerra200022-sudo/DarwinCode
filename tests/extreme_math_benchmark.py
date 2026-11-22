#!/usr/bin/env python3
"""
BENCHMARK MATEMÁTICO EXTREMO PARA DARWINCODE MCP
Problemas de frontera científica y física teórica:
1. Ley de Adición Relativista de Velocidades de Einstein-Poincaré: w = (u + v) / (1 + u·v)
2. Momento Relativista de Lorentz (Dilatación Temporal): p = (m · v) / sqrt(1 - v²)
3. Potencial Caótico Galáctico de Hénon-Heiles: V(x, y) = 0.5·(x² + y²) + x²·y
"""

import subprocess
import json
import math
import time

BINARY = "/data/data/com.termux/files/home/.cargo_target/release/darwin_cli"

def run_benchmark():
    print("=" * 70)
    print("🌌 DESAFÍO MATEMÁTICO EXTREMO: DARWINCODE MCP CO-PROCESADOR")
    print("   Prueba de Alta Dificultad para Inteligencia Artificial y Humanos")
    print("=" * 70)

    proc = subprocess.Popen(
        [BINARY, "--mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )

    def send_rpc(method: str, params: dict, req_id: int):
        payload = {"jsonrpc": "2.0", "id": req_id, "method": method, "params": params}
        proc.stdin.write(json.dumps(payload) + "\n")
        proc.stdin.flush()
        line = proc.stdout.readline()
        return json.loads(line.strip())

    # Handshake
    send_rpc("initialize", {"protocolVersion": "2024-11-05"}, req_id=1)

    # -------------------------------------------------------------
    # RETO 1: ADICIÓN RELATIVISTA DE VELOCIDADES DE EINSTEIN
    # Formula real: w(u, v) = (u + v) / (1 + u * v)
    # Rango: u, v en [0.1, 0.9] (fracción de la velocidad de la luz c)
    # -------------------------------------------------------------
    print("\n" + "#" * 70)
    print("🧪 RETO 1: Cinemática Relativista de Einstein (Adición de Velocidades)")
    print("   Ecuación Teórica: w(u, v) = (u + v) / (1 + u·v)")
    print("   Complejidad: Fracción racional no lineal con acoplamiento cruzado")
    print("#" * 70)

    u_vals = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]
    v_vals = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]
    dataset_relativity = []
    
    for u in u_vals:
        for v in v_vals:
            w = (u + v) / (1.0 + u * v)
            dataset_relativity.append({
                "inputs": [round(u, 2), round(v, 2)],
                "expected": round(w, 5)
            })

    print(f"📊 Dataset generado: {len(dataset_relativity)} puntos experimentales relativistas.")
    print("   Ejemplos:")
    for pt in dataset_relativity[:4]:
        print(f"     u={pt['inputs'][0]}, v={pt['inputs'][1]} -> w={pt['expected']}")

    print("\n🧬 Iniciando evolución genética abierta en DarwinCode...")
    t0 = time.time()
    resp1 = send_rpc("tools/call", {
        "name": "darwin_synthesize",
        "arguments": {
            "dataset": dataset_relativity,
            "input_names": ["u", "v"],
            "max_generations": 3000,
            "target_accuracy": 0.9999,
            "population_size": 200
        }
    }, req_id=2)
    elapsed1 = time.time() - t0

    content1 = resp1.get("result", {}).get("content", [{}])[0].get("text", "")
    print(f"⏱️ Tiempo transcurrido: {elapsed1:.2f} s")
    print(content1)

    # -------------------------------------------------------------
    # RETO 2: EL POTENCIAL CAÓTICO DE HÉNON-HEILES (Astrofísica)
    # Formula real: V(x, y) = 0.5 * (x^2 + y^2) + x^2 * y
    # -------------------------------------------------------------
    print("\n" + "#" * 70)
    print("🌌 RETO 2: Dinámica Caótica Galáctica (Potencial de Hénon-Heiles)")
    print("   Ecuación Teórica: V(x, y) = 0.5·(x² + y²) + x²·y")
    print("   Complejidad: Caos determinista, términos cuadráticos y cúbicos acoplados")
    print("#" * 70)

    dataset_chaos = []
    for x in [-2.0, -1.0, 0.0, 1.0, 2.0]:
        for y in [-2.0, -1.0, 0.0, 1.0, 2.0]:
            v = 0.5 * (x**2 + y**2) + (x**2) * y
            dataset_chaos.append({
                "inputs": [x, y],
                "expected": v
            })

    print(f"📊 Dataset generado: {len(dataset_chaos)} observaciones orbitales.")
    t0 = time.time()
    resp2 = send_rpc("tools/call", {
        "name": "darwin_synthesize",
        "arguments": {
            "dataset": dataset_chaos,
            "input_names": ["x", "y"],
            "max_generations": 3000,
            "target_accuracy": 0.9999,
            "population_size": 200
        }
    }, req_id=3)
    elapsed2 = time.time() - t0

    content2 = resp2.get("result", {}).get("content", [{}])[0].get("text", "")
    print(f"⏱️ Tiempo transcurrido: {elapsed2:.2f} s")
    print(content2)

    proc.terminate()
    print("\n" + "=" * 70)
    print("🏁 Benchmark Extremo Finalizado con Éxito.")
    print("=" * 70)

if __name__ == "__main__":
    run_benchmark()
