# DarwinCode v1.0 SOTA Universal Discovery & Optimization Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform DarwinCode into a State-of-the-Art Neuro-Symbolic Optimization and Discovery Engine capable of autonomously solving extreme non-linear mathematics (relativistic addition, chaotic potentials) in seconds, utilizing continuous constant optimization (Levenberg-Marquardt), automatic symmetry detection (AI Feynman), and Quality-Diversity (MAP-Elites).

**Architecture:**
- `darwin_core::optimizer`: Damped Gauss-Newton / Levenberg-Marquardt continuous optimizer for float constants in DAGs.
- `darwin_core::symmetry`: Analytical detector for permutation invariance ($f(u,v) = f(v,u)$), parity ($f(-x) = f(x)$), and automatic basis expansion ($S = u+v, P = uv$).
- `darwin_core::map_elites`: Multi-dimensional phenotypic archive classifying individuals by complexity and non-linearity to preserve evolutionary stepping stones.
- `darwin_core::engine`: Unified evolution loop integrating constant tuning, MAP-Elites, and Occam's razor parsimony.
- `darwin_cli::mcp_tools`: Exposing enhanced neuro-symbolic tools via MCP JSON-RPC 2.0.

**Tech Stack:** Rust (2021 edition), zero-dependency linear algebra for bare-metal performance, Tokio async stdio MCP server.

**Spec:** `/sdcard/Antigravity/Universo/DarwinCode/docs/superpowers/plans/2026-11-20-darwin-universal-sota.md`

## Global Constraints
- Target platform: aarch64 Android/Termux & Linux x86_64.
- Cargo target directory: `/data/data/com.termux/files/home/.cargo_target`.
- Zero heavy C/BLAS external dependencies for linear algebra to ensure frictionless instant compilation on Termux.
- Complete documentation integrity with docstrings and mathematical rationale.

## Review Focus
1. Singular matrices in Levenberg-Marquardt normal equations: Damping parameter $\lambda$ must prevent division by zero or NaN propagation.
2. Symmetries with noisy floating-point data: Permutation and parity tests must use configurable tolerance $\epsilon \sim 10^{-4}$.
3. Graph without constants: Constant optimizer must gracefully return Ok without mutating graphs that only use variables.
4. MAP-Elites bin boundaries: Out-of-bounds descriptors must clamp to the edge bins without panic.
5. Extreme math benchmarks: Relativistic velocity addition and chaotic potential must converge with MAE $< 10^{-3}$ in under 5 seconds.

---

### Task 1: Continuous Constant Optimizer (Levenberg-Marquardt)

**Files:**
- Create: `core/src/optimizer/mod.rs`
- Create: `core/src/optimizer/continuous.rs`
- Modify: `core/src/lib.rs`
- Test: `core/tests/optimizer_test.rs`

**Interfaces:**
- Consumes: `GenomeGraph`, `evaluate_graph_with_order`, `Value`, `NodeType`
- Produces: `optimize_graph_constants(graph: &mut GenomeGraph, dataset: &[(&[Value], Value)], max_iters: usize) -> f64`

- [ ] **Step 1: Write failing test for constant optimization in `core/tests/optimizer_test.rs`**
  Construct a graph $\frac{c_0 u + c_1 v}{c_2 + c_3 uv}$ with initial constants all set to $0.1$. Run `optimize_graph_constants` against the relativistic dataset and assert that constants converge to $[1.0, 1.0, 1.0, 1.0]$ with MSE $< 10^{-6}$.
- [ ] **Step 2: Run test to verify it fails**
  Run: `cargo test --test optimizer_test`
  Expected: FAIL (module `optimizer` not found).
- [ ] **Step 3: Implement `continuous.rs` and `mod.rs`**
  Implement numerical Jacobian estimation via finite differences, $K \times K$ normal equation solver $(J^T J + \lambda \text{diag}(J^T J)) \Delta c = -J^T r$ with Cholesky/Gaussian pivoting, and adaptive $\lambda$ damping.
- [ ] **Step 4: Run test to verify it passes**
  Run: `cargo test --test optimizer_test`
  Expected: PASS.
- [ ] **Step 5: Commit**
  Run: `git commit -am "feat(core): add Levenberg-Marquardt continuous constant optimizer"`

---

### Task 2: Symmetry & Invariant Detector (AI Feynman)

**Files:**
- Create: `core/src/symmetry/mod.rs`
- Create: `core/src/symmetry/detector.rs`
- Modify: `core/src/lib.rs`
- Test: `core/tests/symmetry_test.rs`

**Interfaces:**
- Consumes: `Value`, dataset slices
- Produces: `detect_symmetries(dataset: &[(&[Value], Value)], num_inputs: usize) -> SymmetryReport`, `augment_dataset_with_symmetries(dataset, &SymmetryReport) -> (AugmentedDataset, Vec<String>)`

- [ ] **Step 1: Write failing test in `core/tests/symmetry_test.rs`**
  Test 1: Permutation symmetry on $(u, v) \implies$ detects pair $(0, 1)$ is symmetric.
  Test 2: Parity symmetry on $(x, y) \implies$ detects variable $x$ has even parity ($V(-x,y) = V(x,y)$).
- [ ] **Step 2: Run test to verify it fails**
  Run: `cargo test --test symmetry_test`
  Expected: FAIL (module `symmetry` not found).
- [ ] **Step 3: Implement `detector.rs` and dataset augmentation**
  Check pairwise permutations by pairing data points $(a, b)$ with $(b, a)$. Check sign inversions $(-x_i)$ against $(x_i)$. Synthesize basis terms $S = x_i + x_j$, $P = x_i x_j$, and $X = x_i^2$.
- [ ] **Step 4: Run test to verify it passes**
  Run: `cargo test --test symmetry_test`
  Expected: PASS.
- [ ] **Step 5: Commit**
  Run: `git commit -am "feat(core): add symmetry and invariant detector"`

---

### Task 3: Quality-Diversity MAP-Elites Archive

**Files:**
- Create: `core/src/map_elites/mod.rs`
- Create: `core/src/map_elites/archive.rs`
- Modify: `core/src/lib.rs`
- Test: `core/tests/map_elites_test.rs`

**Interfaces:**
- Consumes: `GenomeGraph`, `Individual`
- Produces: `MapElitesArchive<D1, D2>`, `add_or_replace(&mut self, ind: Individual, d1: usize, d2: usize) -> bool`, `sample_elite(&self, rng) -> Option<&GenomeGraph>`

- [ ] **Step 1: Write failing test in `core/tests/map_elites_test.rs`**
  Create archive with $5 \times 5$ bins. Insert individuals with varying node counts and fitnesses. Verify replacement only occurs when fitness is higher in the same bin.
- [ ] **Step 2: Run test to verify it fails**
  Run: `cargo test --test map_elites_test`
  Expected: FAIL.
- [ ] **Step 3: Implement `archive.rs`**
  Implement 2D grid archive with cell occupation tracking, coverage metrics, and diversity-preserving selection.
- [ ] **Step 4: Run test to verify it passes**
  Run: `cargo test --test map_elites_test`
  Expected: PASS.
- [ ] **Step 5: Commit**
  Run: `git commit -am "feat(core): add Quality-Diversity MAP-Elites archive"`

---

### Task 4: Upgrade `EvolutionEngine` with SOTA Hybrid Pipeline

**Files:**
- Modify: `core/src/engine.rs`
- Modify: `core/src/mutation/operators.rs`
- Test: `core/tests/sota_engine_test.rs`

**Interfaces:**
- Consumes: `optimize_graph_constants`, `detect_symmetries`, `MapElitesArchive`
- Produces: `EvolutionEngine::step_with_dataset_sota(&mut self, dataset: &[(&[Value], Value)], config: SotaEngineConfig)`

- [ ] **Step 1: Write failing integration test for extreme math in `core/tests/sota_engine_test.rs`**
  Test 1: Solve relativistic velocity addition in $< 100$ generations with MAE $< 10^{-4}$.
  Test 2: Solve chaotic oscillator potential with MAE $< 0.20$ (accounting for data anomalies).
- [ ] **Step 2: Run test to verify it fails**
  Run: `cargo test --test sota_engine_test`
  Expected: FAIL.
- [ ] **Step 3: Integrate hybrid loop into `EvolutionEngine`**
  Every $N$ generations, apply Levenberg-Marquardt to top individuals. Use MAP-Elites to seed crossover and mutation pools.
- [ ] **Step 4: Run test to verify it passes**
  Run: `cargo test --test sota_engine_test`
  Expected: PASS.
- [ ] **Step 5: Commit**
  Run: `git commit -am "feat(core): integrate SOTA neuro-symbolic hybrid pipeline into EvolutionEngine"`

---

### Task 5: Upgrade MCP Server Tools & End-to-End Extreme Benchmark

**Files:**
- Modify: `cli/src/mcp_tools.rs`
- Modify: `cli/src/mcp_protocol.rs`
- Update MCP Schemas in: `mcp/darwin-code/` and `~/.gemini/antigravity-cli/mcp/darwin-code/`
- Create: `tests/extreme_sota_verification.py`

- [ ] **Step 1: Add `enable_sota_optimizer`, `detect_symmetries` flags to `darwin_synthesize` in `cli/src/mcp_tools.rs`**
- [ ] **Step 2: Build release binary using cargo target directory**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo build --release`
- [ ] **Step 3: Run comprehensive verification script `extreme_sota_verification.py` via MCP JSON-RPC**
  Verify solving Einstein velocity addition and Hénon-Heiles/Contopoulos chaotic potential.
- [ ] **Step 4: Update MCP schemas and documentation**
- [ ] **Step 5: Commit and final report**
  Run: `git commit -am "feat(cli): expose SOTA neuro-symbolic optimization in MCP tools"`
