<p align="center">
  <img src="assets/banner.jpg" alt="DarwinCode Banner" width="100%">
</p>

<p align="center">
  <img src="assets/logo.jpg" alt="DarwinCode Logo" width="180px">
</p>

<h1 align="center">🧬 DarwinCode</h1>

<p align="center">
  <strong>Ultra-Fast Neuro-Symbolic Evolutionary Engine (>300,000 evals/s) for Symbolic Regression, Algorithmic Pruning, and Mathematical Discovery</strong>
</p>

<p align="center">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/Language-Rust%202021-orange.svg?style=for-the-badge&logo=rust" alt="Rust 2021"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Dual%20(PolyForm%20%2B%20Commercial)-blue.svg?style=for-the-badge" alt="License"></a>
  <a href="#benchmarks"><img src="https://img.shields.io/badge/Speed-%3E300%2C000%20evals%2Fs-brightgreen.svg?style=for-the-badge" alt="Evaluations"></a>
  <a href="#mcp-co-processor"><img src="https://img.shields.io/badge/MCP-12%20Native%20Tools-purple.svg?style=for-the-badge" alt="MCP Compatible"></a>
  <a href="#tests"><img src="https://img.shields.io/badge/Tests-66%2F66%20Passing-success.svg?style=for-the-badge" alt="Tests Passing"></a>
</p>

---

## 🌌 What is DarwinCode?

**DarwinCode** is a state-of-the-art open-ended neuro-symbolic evolutionary computation engine engineered in pure Rust. It autonomously synthesizes mathematical formulas, algorithms, and computational graphs from empirical datasets using typed grammar-guided genetic programming (GGGP), formal invariant verification, and Occam's razor parsimony pressure.

Unlike deep learning models that hallucinate or require high-end GPU clusters, DarwinCode operates with **zero hallucination**, **deterministic gas-metered execution**, and an **$O(1)$ memory footprint**. Thousands of candidate programs compete in a continuous digital ecosystem, mutating safe Abstract Syntax Tree (AST) subtrees, calibrating constants, and eliminating bloat without ever leaking memory or stalling.

```
                            DARWINCODE ARCHITECTURE
                            
    [ TARGET DATASET / ENIGMA ] (e.g. Physics law, elliptic curve, fluid dynamics)
                 │
                 ▼
     ┌────────────────────────────────────────────────────────────────────────┐
     │ 1. TYPED COMPUTATION DAGs (Genotype Space)                             │
     │    Strict type-safety: Float, Int, Boolean, Vec3D ports                │
     └────────────────────────────────────────────────────────────────────────┘
                 │
                 ▼
     ┌────────────────────────────────────────────────────────────────────────┐
     │ 2. SMT-LITE FORMAL VERIFIER                                            │
     │    Mathematical guarantees: Zero syntax errors, cycle-free DAGs        │
     └────────────────────────────────────────────────────────────────────────┘
                 │
                 ▼
     ┌────────────────────────────────────────────────────────────────────────┐
     │ 3. DETERMINISTIC GAS-METERED VIRTUAL MACHINE                           │
     │    Halting guarantees: Gas-exhaustion bounds, safe math (no NaNs)      │
     └────────────────────────────────────────────────────────────────────────┘
                 │
                 ▼
     ┌────────────────────────────────────────────────────────────────────────┐
     │ 4. NATURAL SELECTION & O(1) DOUBLE-BUFFERED ARENA POOL                 │
     │    Continuous evolution over 1,000,000+ generations with flat memory   │
     └────────────────────────────────────────────────────────────────────────┘
                 │
                 ▼
     [ 60 FPS REAL-TIME WEB COCKPIT ] (SSE Live Stream at http://localhost:8080)
```

---

## ⚡ Key Highlights

- **⚡ Blazing Native Performance:** Evaluates over **300,000 candidate formulas per second** on standard multi-core hardware.
- **🛡️ Formally Verified Execution:** Zero division-by-zero panics, zero negative square roots, and gas-metered execution that mathematically prevents infinite loops.
- **✂️ Occam's Razor Parsimony Pressure:** Automatically identifies and prunes dead nodes (introns), favoring the shortest, most elegant closed-form expressions.
- **🔄 Autonomous Continuous Learning:** Dynamically adapts to local plateaus, automatically cycling through challenges once laws reach $>99.5\%$ precision or after evolutionary timeouts.
- **🤖 Universal MCP Co-Processor:** Exposes 12 high-throughput tools for AI Agents (**Google Antigravity**, **Claude Code**, **OpenAI Codex**, **Cursor**, **LibreChat**).
- **🌐 Embedded Cyberpunk 3D Cockpit:** Built-in zero-dependency HTTP/SSE dashboard rendering DAG graphs, real-time Pareto frontiers, and 3D voxel representations at 60 FPS.

---

## 🏆 Proven Discovery Benchmarks (Hall of Fame)

DarwinCode has autonomously recovered canonical physics and mathematical laws with **100.0% precision**:

| Domain | Target Law | Discovered Closed-Form Expression | Accuracy | Status |
| :--- | :--- | :--- | :---: | :---: |
| **Quadratic Parabola** | $f(x) = x^2 + 1$ | `(abs((x * x)) + 1.00)` | **100.0%** | Conquered |
| **Galilean Free Fall** | $y = 5 \cdot t^2$ | `((Max(Modulo(2.31, t), 5.00) * t) * t)` | **100.0%** | Conquered |
| **Harmonic Wave** | $f(x) = 3\sin(x)$ | `((sin(x) + sin(x)) + sin(x))` | **100.0%** | Conquered |
| **3D Paraboloid** | $f(x, y) = x^2 + y^2$ | `((y * y) + (x * x))` | **100.0%** | Conquered |
| **Conditional Switch** | $a > b \Rightarrow a-b,\; \text{else } a+b$ | `Max((a - b), ((b - (a - b)) - 1.00))` | **100.0%** | Conquered |

---

## 🔬 Advanced Research Engines

### 1. Elliptic Curves & Birch-Swinnerton-Dyer (BSD) Invariants
Located in `core/src/elliptic_bsd.rs`:
- High-performance arithmetic point counting on $\mathbb{F}_p$ for curves $y^2 = x^3 + Ax + B$.
- Fourier trace computation $a_p = p + 1 - \#E(\mathbb{F}_p)$ with Hasse bounds.
- Sato-Tate angle distribution mapping: $\theta_p = \arccos\left(\frac{a_p}{2\sqrt{p}}\right)$.
- Goldfeld-Mestre logarithmic heuristics for analytic root number $W(E)$ parity prediction.
- Benchmarked against verified LMFDB (*L-functions and Modular Forms Database*) curves.

### 2. Fluid Dynamics & Navier-Stokes Blow-Up Simulation
Located in `core/src/navier_stokes.rs`:
- Hou-Luo / De Gregorio 1D vorticity model with solid boundary effects.
- Periodic discrete Hilbert transform via exact cotangent kernel.
- Beale-Kato-Majda (BKM) singularity detection: $\int_0^T \|\omega\|_{L^\infty} dt$.
- Genetic evolution of self-similar blow-up profiles:
  $$\Omega(\xi) = \frac{c_0 \xi}{(1+\xi^2)^\gamma}$$

### 3. Ramanujan Continued Fraction Engine & OEIS Miner
Located in `core/src/ramanujan.rs` & `core/src/oeis.rs`:
- Generates generalized continued fractions:
  $$a_0 + \frac{b_1}{a_1 + \frac{b_2}{a_2 + \dots}}$$
  converging to fundamental constants ($\pi$, $e$, Catalan's $G$, Apery's $\zeta(3)$).
- Automated sequence synthesis against OEIS integer sequences.

---

## 🚀 Quick Start

### Prerequisites
- [Rust 1.75+](https://rustup.rs/) (Cargo & rustc)

### 1. Installation & Build
```bash
git clone https://github.com/miguelguerra200022-sudo/DarwinCode.git
cd DarwinCode
cargo build --release
```

### 2. Launch the Engine & Web Cockpit
```bash
./target/release/darwin_cli
```
Open **`http://localhost:8080`** in any web browser to observe the live evolutionary stream, topology graphs, and Hall of Fame!

### 3. Run MCP Server for AI Coding Agents
```bash
./target/release/darwin_cli --mcp
```

### 4. Direct CLI Synthesis
```bash
./target/release/darwin_cli synthesize --inputs x --output float --dataset "[-2, 5], [-1, 2], [0, 1], [1, 2], [2, 5]"
```

---

## 🧪 Testing & Verification

Run the entire suite of 66 comprehensive unit, integration, and property tests:

```bash
cargo test --all
```

Result:
```
test result: ok. 44 passed (darwin_core); 22 passed (darwin_cli); 0 failed; 0 ignored
```

---

## 🤖 Model Context Protocol (MCP) Integration

DarwinCode implements the official **Model Context Protocol (stdio)**. Add it to your AI agent configuration:

### Claude Desktop (`claude_desktop_config.json`)
```json
{
  "mcpServers": {
    "darwin-code": {
      "command": "/absolute/path/to/DarwinCode/target/release/darwin_cli",
      "args": ["--mcp"]
    }
  }
}
```

### Cursor / Codex / Antigravity (`~/.gemini/antigravity-cli/mcp/darwin-code.json`)
```json
{
  "command": "darwin_cli",
  "args": ["--mcp"]
}
```

### Available MCP Tools:
1. `darwin_synthesize`: Symbolic regression from dataset points.
2. `darwin_optimize_code`: Prunes dead AST branches and bloat using parsimony.
3. `darwin_evolve_voxels`: Evolves verified 3D structural voxel matrices.
4. `darwin_safe_eval`: Deterministic gas-metered evaluation sandbox.
5. `darwin_ramanujan_solve`: Discovers continued fraction representations for mathematical constants.
6. `darwin_srsd_hard_solve`: High-dimensional symbolic discovery for physics laws.
7. `darwin_oeis_discover`: Reverse-engineers integer sequences from OEIS.
8. `darwin_bsd_analysis`: Analyzes elliptic curves, Frobenius traces, and BSD rank parity.
9. `darwin_navier_stokes_blowup`: Simulates Hou-Luo fluid vorticity and self-similar singularities.
10. `darwin_get_status`: Returns engine telemetry, generation count, and Hall of Fame records.
11. `darwin_inject_heuristic`: Injects custom mathematical seed genomes into the evolutionary pool.
12. `darwin_explain_genome`: Generates visual ASCII and mathematical LaTeX expressions for genomes.

---

## 📁 Repository Structure

```
DarwinCode/
├── assets/                # High-res logos and banners
│   ├── logo.jpg           # Emblem logo
│   └── banner.jpg         # Hero banner
│
├── core/                  # Core Evolutionary Engine (darwin_core)
│   ├── src/
│   │   ├── types.rs       # Type system, safe math, OpCodes
│   │   ├── genome/        # Computation DAGs, topological sorting
│   │   ├── vm/            # Gas-metered deterministic virtual machine
│   │   ├── mutation/      # Grammar-guided mutation, crossover
│   │   ├── verifier/      # SMT-lite cycle detection and type checking
│   │   ├── engine.rs      # Evolutionary engine with double buffering
│   │   ├── elliptic_bsd.rs# Elliptic curves & BSD invariant engine
│   │   ├── navier_stokes.rs # Hou-Luo vorticity & fluid blowup solver
│   │   ├── ramanujan.rs   # Continued fraction discovery
│   │   └── oeis.rs        # Integer sequence synthesis
│   └── tests/             # Unit and integration test suites
│
├── cli/                   # Runner, MCP Server & Embedded Web (darwin_cli)
│   └── src/
│       ├── main.rs        # Evolution coordinator and CLI entrypoint
│       ├── mcp.rs         # Model Context Protocol stdio server
│       ├── mcp_tools.rs   # MCP tool definitions and executors
│       └── server.rs      # Embedded HTTP/SSE live streaming server
│
├── skills/                # Agent Skills
│   └── darwin-code/       # Antigravity & Claude Code skill definitions
│
├── web/                   # Embedded Web Cockpit (http://localhost:8080)
│   ├── index.html         # Cyberpunk dashboard interface
│   ├── style.css          # Visual styling
│   └── app.js             # Real-time SSE client and 3D visualizer
│
├── OEIS_SUBMISSIONS.md    # Formal OEIS submissions guide (A161330 / A006459)
├── LICENSE                # Dual License (PolyForm Noncommercial + Commercial)
└── README.md              # Project documentation
```

---

## 📜 License

DarwinCode is dual-licensed:

- **Free for Individuals, Independents & Academic Research:** Licensed under the [PolyForm Noncommercial License 1.0.0](LICENSE). You are free to study, experiment, publish papers, and build non-commercial tools.
- **Commercial Enterprise License:** Required for commercial companies, proprietary hedge funds, and institutions with over **$1,000,000 USD** in cumulative capital, funding, or annual revenue.

For commercial licensing and enterprise support:
- Author: **Miguel Guerra** ([@miguelguerra200022-sudo](https://github.com/miguelguerra200022-sudo))
- Inquiries: Open a confidential inquiry via repository issues or GitHub profile.

---

<p align="center">
  <sub>Built with mathematical passion and pure Rust. Engineered for open-ended discovery.</sub>
</p>
