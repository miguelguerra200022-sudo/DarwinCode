# Pi Triad Discovery Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the Pi Triad mathematical discovery engine in DarwinCode (Recurrent Continued Fractions, Galperin Chaotic Billiards, and Non-Linear Decimal Spigot), purge legacy challenges, and dedicate the live engine to autonomous Pi exploration.

**Architecture:** Pure Rust implementation across three core modules in `darwin_core` (`pi_continued_fractions.rs`, `pi_galperin_billiards.rs`, `pi_bbp_spigot.rs`), integrated with `cli/src/main.rs` challenge runner, HTTP API in `server.rs`, and MCP tools in `mcp_tools.rs`.

**Tech Stack:** Rust 2021, Cargo, Tokio, Serde, JSON-RPC (MCP), SSE/HTTP.

**Spec:** `docs/superpowers/specs/2026-11-22-pi-triad-discovery-design.md`

## Global Constraints
- Target directory: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target`.
- Zero-panic guarantees: all arithmetic operations must use safe guards against division by zero and NaNs.
- Pure Rust, zero external C library dependencies.
- Maintain existing 66 passing tests while adding comprehensive new test suites.

## Review Focus
- Division by zero in continued fraction depth evaluation when a denominator $B_k = 0$.
- Infinite loop in Galperin billiard collisions when block velocities become equal ($v_1 = v_2$).
- Precision loss in large mass ratios $M/m = 10^8$ in event-driven physics.
- Out-of-bounds index lookup in decimal digit dataset ($n \ge 100$).
- Thread-safe rotation in `main.rs` without deadlock on `target_challenge`.

---

### Task 1: Recurrent Continued Fractions for $\pi$ (`core/src/pi_continued_fractions.rs`)

**Files:**
- Create: `core/src/pi_continued_fractions.rs`
- Test: `core/tests/pi_continued_fractions_test.rs`

**Interfaces:**
- Produces:
  - `pub struct ContinuedFractionStep { pub a: f64, pub b: f64 }`
  - `pub fn evaluate_euler_wallis(steps: &[ContinuedFractionStep], b0: f64) -> (f64, f64)`: returns `(convergent_value, abs_error)`.
  - `pub fn stern_brouncker_steps(depth: usize) -> (f64, Vec<ContinuedFractionStep>)`
  - `pub fn ramanujan_pi_cf_steps(depth: usize) -> (f64, Vec<ContinuedFractionStep>)`

- [ ] **Step 1: Write the failing test**
  Create `core/tests/pi_continued_fractions_test.rs` asserting convergence of Stern-Brouncker ($\pi = 4 / (1 + 1^2/(2 + 3^2/(2 + ...)))$) and Ramanujan's continued fraction to within $10^{-4}$ of $\pi$.

- [ ] **Step 2: Run test to verify it fails**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --test pi_continued_fractions_test`
  Expected: FAIL (module not found).

- [ ] **Step 3: Implement `core/src/pi_continued_fractions.rs`**
  Implement Euler-Wallis recurrence $A_k = b_k A_{k-1} + a_k A_{k-2}$, $B_k = b_k B_{k-1} + a_k B_{k-2}$ with scaling to prevent overflow, and generators for Stern-Brouncker and Ramanujan continued fractions.

- [ ] **Step 4: Run test to verify it passes**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --test pi_continued_fractions_test`
  Expected: PASS.

- [ ] **Step 5: Commit**
  ```bash
  git add core/src/pi_continued_fractions.rs core/tests/pi_continued_fractions_test.rs
  git commit -m "feat(core): implement recurrent continued fraction engine for Pi"
  ```

---

### Task 2: Galperin Chaotic Billiards & Collision Engine (`core/src/pi_galperin_billiards.rs`)

**Files:**
- Create: `core/src/pi_galperin_billiards.rs`
- Test: `core/tests/pi_galperin_billiards_test.rs`

**Interfaces:**
- Produces:
  - `pub struct BilliardState { pub x1: f64, pub v1: f64, pub x2: f64, pub v2: f64, pub m1: f64, pub m2: f64 }`
  - `pub struct GalperinResult { pub collisions: u64, pub wall_collisions: u64, pub block_collisions: u64, pub energy_error: f64, pub estimated_pi: f64 }`
  - `pub fn simulate_galperin(mass_ratio: f64, max_collisions: u64) -> GalperinResult`

- [ ] **Step 1: Write the failing test**
  Create `core/tests/pi_galperin_billiards_test.rs` asserting:
  - $M/m = 1 \implies 3$ collisions ($\pi \approx 3$).
  - $M/m = 100 \implies 31$ collisions ($\pi \approx 3.1$).
  - $M/m = 10000 \implies 314$ collisions ($\pi \approx 3.14$).
  - Energy conservation: $\Delta E / E_0 < 10^{-12}$.

- [ ] **Step 2: Run test to verify it fails**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --test pi_galperin_billiards_test`
  Expected: FAIL.

- [ ] **Step 3: Implement `core/src/pi_galperin_billiards.rs`**
  Implement the exact event-driven collision solver computing $\Delta t_{\text{wall}}$ and $\Delta t_{\text{blocks}}$, advancing simulation time to next collision event, and performing elastic momentum exchanges until separation.

- [ ] **Step 4: Run test to verify it passes**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --test pi_galperin_billiards_test`
  Expected: PASS.

- [ ] **Step 5: Commit**
  ```bash
  git add core/src/pi_galperin_billiards.rs core/tests/pi_galperin_billiards_test.rs
  git commit -m "feat(core): implement Galperin chaotic billiard collision simulator for Pi"
  ```

---

### Task 3: Non-Linear Decimal Digit Extractor (`core/src/pi_bbp_spigot.rs`)

**Files:**
- Create: `core/src/pi_bbp_spigot.rs`
- Test: `core/tests/pi_bbp_spigot_test.rs`

**Interfaces:**
- Produces:
  - `pub const PI_DECIMAL_DIGITS_100: &[u8; 100]`
  - `pub fn get_pi_digit(index: usize) -> Option<u8>`
  - `pub fn evaluate_digit_accuracy<F>(mut extractor: F, count: usize) -> f64 where F: FnMut(usize) -> u8`
  - `pub fn pi_digit_dataset(count: usize) -> Vec<(Vec<darwin_core::types::Value>, darwin_core::types::Value)>`

- [ ] **Step 1: Write the failing test**
  Create `core/tests/pi_bbp_spigot_test.rs` asserting first 10 digits are `[3, 1, 4, 1, 5, 9, 2, 6, 5, 3]` and accuracy function works.

- [ ] **Step 2: Run test to verify it fails**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --test pi_bbp_spigot_test`
  Expected: FAIL.

- [ ] **Step 3: Implement `core/src/pi_bbp_spigot.rs`**
  Embed canonical 100 digits of $\pi$ and implement dataset generator converting $(n) \to d_n$ for DarwinCode evolutionary graphs.

- [ ] **Step 4: Run test to verify it passes**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --test pi_bbp_spigot_test`
  Expected: PASS.

- [ ] **Step 5: Commit**
  ```bash
  git add core/src/pi_bbp_spigot.rs core/tests/pi_bbp_spigot_test.rs
  git commit -m "feat(core): implement decimal Pi digit spigot and dataset generator"
  ```

---

### Task 4: Export Modules in `core/src/lib.rs`

**Files:**
- Modify: `core/src/lib.rs`

- [ ] **Step 1: Add module declarations**
  Add `pub mod pi_continued_fractions;`, `pub mod pi_galperin_billiards;`, `pub mod pi_bbp_spigot;`.
- [ ] **Step 2: Run all core tests**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test -p darwin_core`
  Expected: PASS (all tests pass).
- [ ] **Step 3: Commit**
  ```bash
  git add core/src/lib.rs
  git commit -m "feat(core): export Pi Triad discovery modules"
  ```

---

### Task 5: Refactor `cli/src/main.rs` to Focus 100% on the Pi Triad

**Files:**
- Modify: `cli/src/main.rs`

- [ ] **Step 1: Replace legacy challenges with the Pi Triad**
  Remove `parabola`, `gravity`, `wave`, `surface3d`, `logic_gate`.
  Define new `ChallengeSpec` entries:
  - `"pi_cf"`: `inputs: &["k"]`, target: Stern-Brouncker / Ramanujan continued fraction convergence values.
  - `"pi_billiards"`: `inputs: &["log_mass"]`, target: $\lfloor \pi \cdot 10^N \rfloor$.
  - `"pi_spigot"`: `inputs: &["n"]`, target: decimal digits of $\pi$.
  Set active rotation to: `["pi_cf", "pi_billiards", "pi_spigot"]`.
- [ ] **Step 2: Compile & verify `darwin_cli`**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo check --bin darwin_cli`
  Expected: PASS.
- [ ] **Step 3: Commit**
  ```bash
  git add cli/src/main.rs
  git commit -m "feat(cli): dedicate live challenge loop exclusively to Pi Triad"
  ```

---

### Task 6: Add Pi Triad HTTP API & MCP Tools (`cli/src/server.rs`, `cli/src/mcp_tools.rs`, `cli/src/mcp.rs`)

**Files:**
- Modify: `cli/src/mcp_tools.rs`
- Modify: `cli/src/mcp.rs`
- Modify: `cli/src/server.rs`
- Test: `cli/tests/mcp_stdio_test.rs`

- [ ] **Step 1: Add MCP Tool handlers in `mcp_tools.rs`**
  - `darwin_pi_continued_fraction`: depth, sequence type.
  - `darwin_pi_billiards_simulate`: mass_ratio or N.
  - `darwin_pi_spigot_extract`: count of digits to verify.
- [ ] **Step 2: Expose tools in `mcp.rs` and routes in `server.rs`**
  - Add routes: `/api/challenge/pi_cf`, `/api/challenge/pi_billiards`, `/api/challenge/pi_spigot`.
- [ ] **Step 3: Update `mcp_stdio_test.rs` tool count assertion**
  Update assertion from 12 to 15 tools and verify all pass.
- [ ] **Step 4: Verify test passes**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test -p darwin_cli`
  Expected: PASS.
- [ ] **Step 5: Commit**
  ```bash
  git add cli/src/mcp_tools.rs cli/src/mcp.rs cli/src/server.rs cli/tests/mcp_stdio_test.rs
  git commit -m "feat(cli): expose Pi Triad tools via MCP and HTTP API"
  ```

---

### Task 7: Full System Verification, Release Build & Live Execution

**Files:**
- Test all: `cargo test --all`
- Build release: `cargo build --release --bin darwin_cli`

- [ ] **Step 1: Run complete workspace test suite**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --all`
  Expected: 100% passing tests.
- [ ] **Step 2: Build release binary**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo build --release --bin darwin_cli`
- [ ] **Step 3: Launch live `darwin_cli` in background**
  Run `/data/data/com.termux/files/home/.cargo_target/release/darwin_cli`.
  Verify port 8080 rotates through `pi_cf`, `pi_billiards`, and `pi_spigot`.
- [ ] **Step 4: Commit and push updates to GitHub**
  ```bash
  git push origin main
  ```
