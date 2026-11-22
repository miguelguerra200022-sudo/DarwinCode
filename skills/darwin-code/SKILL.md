---
name: darwin-code
description: Co-procesador neuro-simbólico y motor evolutivo ultra-rápido (>300.000 eval/s). Úsalo cuando necesites descubrir fórmulas matemáticas cerradas a partir de datos empíricos, optimizar algoritmos eliminando código muerto/bloat (Navaja de Ockham), generar vóxeles estructurales 3D o evaluar código de forma segura en un entorno aislado con gasómetro.
---

# DarwinCode: Co-Procesador Neuro-Simbólico para Agentes IA

DarwinCode es una extensión de alta velocidad implementada en Rust que complementa a los agentes de IA (Google Antigravity, Claude Code, Codex). Provee búsqueda genética ultra-rápida, evaluación gas-metered y poda de introns con rigor matemático.

## ¿Por qué usar DarwinCode como brazo del Agente?

| Problema del LLM | Solución de DarwinCode |
|------------------|------------------------|
| Alucinación en interpolaciones numéricas no lineales | Evaluación determinista de más de 300.000 circuitos DAG por segundo |
| Búsqueda ciega en optimización combinatoria | Búsqueda evolutiva abierta (OEE) con torneo y presión de parsimonia |
| Riesgo de bucles infinitos en ejecución de código | Máquina virtual determinista con cuota estricta de gas |
| Expresiones excesivamente complejas ("bloat") | Poda formal de nodos muertos y simplificación por Navaja de Ockham |

---

## Herramientas Disponibles

### 1. `darwin_synthesize`
Descubre la ley matemática cerrada que gobierna una serie de datos experimentales.

**Ejemplo de uso:**
```json
{
  "dataset": [
    {"inputs": [-3.0], "expected": 10.0},
    {"inputs": [-2.0], "expected": 5.0},
    {"inputs": [-1.0], "expected": 2.0},
    {"inputs": [0.0], "expected": 1.0},
    {"inputs": [1.0], "expected": 2.0},
    {"inputs": [2.0], "expected": 5.0},
    {"inputs": [3.0], "expected": 10.0}
  ],
  "input_names": ["x"],
  "max_generations": 1000,
  "target_accuracy": 0.999
}
```
**Resultado retornado:**
- Fórmula: `((x * x) + 1.00)` ($x^2 + 1$)
- Precisión: `100.0%`
- Nodos mínimos: `4`

---

### 2. `darwin_optimize_code`
Toma una expresión o circuito DAG y lo simplifica al mínimo número de nodos matemáticos posibles sin alterar su salida.

**Ejemplo de uso:**
```json
{
  "expression": "((x * x) + (2.0 - 1.0))",
  "input_names": ["x"]
}
```

---

### 3. `darwin_evolve_voxels`
Genera una estructura volumétrica o plano arquitectónico 3D en formato de coordenadas `[x, y, z, block_type]`.

**Ejemplo de uso:**
```json
{
  "floors": 4,
  "style": "tower",
  "max_blocks": 250
}
```

---

### 4. `darwin_safe_eval`
Evalúa puntos de prueba sobre la VM de DarwinCode con cuota de gas para garantizar terminación sin efectos secundarios.

---

## Modo de Ejecución del Servidor

DarwinCode corre como servidor local MCP sobre `stdio`:
```bash
/data/data/com.termux/files/home/.cargo_target/release/darwin_cli --mcp
```
O con interfaz web interactiva en puerto 8080:
```bash
/data/data/com.termux/files/home/.cargo_target/release/darwin_cli
```
Visita `http://localhost:8080` para ver la evolución genética en vivo en 3D.
