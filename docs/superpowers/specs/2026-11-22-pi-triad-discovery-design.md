# Design Specification: The Pi Triad Discovery Engine in DarwinCode

**Date:** 2026-11-22  
**Author:** Miguel Guerra & Antigravity  
**Status:** Approved  

---

## 1. Overview & Motivation

While existing research around $\pi$ relies on either static polynomial continued fractions (the *Ramanujan Machine*) or standard base-16 BBP spigot sums, DarwinCode expands into three uncharted computational territories:
1. **Recurrent Non-Polynomial Continued Fractions:** Bivariate recursive DAGs $a_n = f(n, a_{n-1}, (-1)^n)$ and $b_n = g(n, b_{n-1})$ evaluated with exact Euler-Wallis recurrences targeting super-exponential convergence to $\pi$.
2. **Galperin Chaotic Billiards & Symplectic Collision Engine:** Event-driven 1D/2D elastic collision mechanics that compute the digits and convergents of $\pi$ directly from Newtonian Hamiltonian phase-space dynamics.
3. **Non-Linear Decimal Spigot Extractor:** Synthesizing discrete functional circuits $d_n = \Phi(n) \pmod{10}$ over the decimal expansion of $\pi$.

All legacy challenges (`parabola`, `gravity`, `wave`, `surface3d`, `logic_gate`) are removed from the active evolution cycle to dedicate 100% of the engine's throughput to the Pi Triad.

---

## 2. Architecture & Components

```
                            THE PI TRIAD ENGINE
                            
    ┌──────────────────────────────────────────────────────────────────┐
    │ 1. core/src/pi_continued_fractions.rs                            │
    │    - Euler-Wallis recurrence solver                              │
    │    - Convergence rate analyzer (digits per depth step)           │
    │    - Canonical seeds: Stern-Brouncker, Euler, Ramanujan          │
    └─────────────────────────────────┬────────────────────────────────┘
                                      │
    ┌─────────────────────────────────▼────────────────────────────────┐
    │ 2. core/src/pi_galperin_billiards.rs                             │
    │    - Exact event-driven elastic collision simulator              │
    │    - Symplectic phase space coordinate transform                 │
    │    - Verification: M/m = 100^N -> collisions = floor(pi * 10^N)  │
    └─────────────────────────────────┬────────────────────────────────┘
                                      │
    ┌─────────────────────────────────▼────────────────────────────────┐
    │ 3. core/src/pi_bbp_spigot.rs                                     │
    │    - High-precision decimal pi digit table (first 100+ digits)   │
    │    - Discrete modular circuit evaluator                          │
    │    - Accuracy score over digit sequence                          │
    └─────────────────────────────────┬────────────────────────────────┘
                                      │
    ┌─────────────────────────────────▼────────────────────────────────┐
    │ 4. cli/src/main.rs & server.rs & mcp_tools.rs                    │
    │    - Active rotation: ["pi_cf", "pi_billiards", "pi_spigot"]     │
    │    - HTTP Endpoints: /api/challenge/pi_{cf, billiards, spigot}   │
    │    - MCP Tools: darwin_pi_{continued_fraction, billiards, spigot}│
    └──────────────────────────────────────────────────────────────────┘
```

---

## 3. Detailed Component Design

### 3.1. `core/src/pi_continued_fractions.rs`
- **Representation:** A continued fraction is defined by sequences $a_k$ (partial numerators) and $b_k$ (partial denominators):
  $$S_k = b_0 + \cfrac{a_1}{b_1 + \cfrac{a_2}{b_2 + \dots + \cfrac{a_k}{b_k}}}$$
- **Euler-Wallis Algorithm:**
  $$A_{-1} = 1, \quad A_0 = b_0, \quad B_{-1} = 0, \quad B_0 = 1$$
  $$A_k = b_k A_{k-1} + a_k A_{k-2}$$
  $$B_k = b_k B_{k-1} + a_k B_{k-2}$$
  $$S_k = \frac{A_k}{B_k}$$
- **Numerical Safeguards:** Prevent division by zero and handle scaling to avoid floating-point / integer overflow during evaluation.
- **Metrics:**
  - Absolute error: $\epsilon_k = |S_k - \pi|$.
  - Effective decimal digits: $-\log_{10}(\epsilon_k + 10^{-16})$.

### 3.2. `core/src/pi_galperin_billiards.rs`
- **Physical Setup:**
  - Rigid elastic wall at $x = 0$.
  - Block 1 (mass $m = 1.0$) at $x_1 = 1.0$, initial velocity $v_1 = 0.0$.
  - Block 2 (mass $M = 100^N \cdot m$) at $x_2 > x_1$, initial velocity $v_2 = -u < 0$.
- **Collision Dynamics:**
  - Wall collision: When $x_1 = 0$, velocity flips: $v_1' = -v_1$.
  - Inter-block collision: Perfectly elastic 1D collision conserving momentum and kinetic energy:
    $$v_1' = \frac{m - M}{m + M} v_1 + \frac{2M}{m + M} v_2$$
    $$v_2' = \frac{2m}{m + M} v_1 + \frac{M - m}{m + M} v_2$$
- **Event-Driven Simulation:**
  - Compute exact $\Delta t_{\text{wall}} = -x_1 / v_1$ (if $v_1 < 0$).
  - Compute exact $\Delta t_{\text{blocks}} = (x_2 - x_1) / (v_1 - v_2)$ (if $v_1 > v_2$).
  - Advance time to the earliest event, apply collision law, increment counter, repeat until $v_2 \ge v_1 \ge 0$ (no further collisions possible).
- **Theorem Verification:**
  - $N=0 \implies M/m = 1 \implies 3$ collisions.
  - $N=1 \implies M/m = 100 \implies 31$ collisions.
  - $N=2 \implies M/m = 10000 \implies 314$ collisions.
  - $N=3 \implies M/m = 1000000 \implies 3141$ collisions.

### 3.3. `core/src/pi_bbp_spigot.rs`
- **Target Dataset:** The first 100 digits of $\pi$:
  $$[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5, 0, 2, 8, 8, 4, 1, 9, 7, 1, 6, 9, 3, 9, 9, 3, 7, 5, 1...]$$
- **Discrete Circuit Evaluator:** Evaluates candidate expression graphs $f(n)$ where $n \in \{0..99\}$ and maps output to integer digits $\{0..9\}$.
- **Loss / Fitness:** Percentage of correctly matched digits over the training window.

---

## 4. Integration & Challenges Workflow

1. **Challenges in `cli/src/main.rs`:**
   - `"pi_cf"`: Recurrent Continued Fractions targeting $\pi$ convergence.
   - `"pi_billiards"`: Galperin collision parameters and phase space exploration.
   - `"pi_spigot"`: Discrete digit extraction.
2. **Auto-Rotation:** Continuously cycles across the three challenges, updating the Hall of Fame upon achieving high precision or upon generation timeout.
3. **HTTP API in `cli/src/server.rs`:**
   - `POST /api/challenge/pi_cf`
   - `POST /api/challenge/pi_billiards`
   - `POST /api/challenge/pi_spigot`
4. **MCP Tools:**
   - `darwin_pi_continued_fraction`
   - `darwin_pi_billiards_simulate`
   - `darwin_pi_spigot_extract`

---

## 5. Verification Plan

1. **Unit Tests:**
   - `core/tests/pi_continued_fractions_test.rs`: Test Euler-Wallis recurrence against Stern-Brouncker ($\pi = 4 / (1 + 1^2/(2 + 3^2/(2 + ...)))$) and ensure convergence within $10^{-6}$.
   - `core/tests/pi_galperin_billiards_test.rs`: Verify collision counts 3, 31, 314 for $M/m = 1, 100, 10000$. Verify energy conservation $\Delta E / E < 10^{-12}$.
   - `core/tests/pi_bbp_spigot_test.rs`: Verify exact digit lookup and modular loss calculation.
2. **Integration Tests:**
   - Test MCP tools in `cli/tests/mcp_tools_test.rs` and `cli/tests/mcp_stdio_test.rs`.
   - Test HTTP server responses in `cli`.
3. **Live System Run:**
   - Build `darwin_cli` in release mode.
   - Run live engine and confirm rotation among the 3 Pi challenges in the terminal.
