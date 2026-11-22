# Implementación de Motores para Problemas del Milenio (BSD y Navier-Stokes)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implementar en DarwinCode dos motores matemáticos de frontera: (1) un motor algebraico para la Conjetura de Birch y Swinnerton-Dyer (BSD) sobre curvas elípticas, y (2) un motor dinámico para la búsqueda de singularidades y perfiles autosimilares de Navier-Stokes/Euler (modelo Hou-Luo / De Gregorio), ambos con integración interactiva en la web sandbox.

**Architecture:** Módulos en Rust puro sin dependencias externas (`elliptic_bsd.rs` y `navier_stokes.rs`) integrados en el núcleo `darwin_core`. Los motores exponen funciones deterministas, cálculo de invariantes aritméticos exactos (Cremona / LMFDB) y búsqueda genética ultra-rápida de perfiles y fórmulas simbólicas, expuestos vía `mcp_tools` y visualizados en `sandbox.html`.

**Tech Stack:** Rust (2021 edition), Álgebra de Curvas Elípticas (recuento de puntos sobre campos finitos F_p, símbolos de Legendre, discriminante de Weierstrass), Dinámica de Fluidos (discretización 1D/2D del modelo de vorticidad de Hou-Luo, enstrofía, criterio Beale-Kato-Majda), WebAssembly/HTML5 Canvas en `sandbox.html`.

**Spec:** `/sdcard/Antigravity/Universo/DarwinCode/docs/superpowers/plans/2026-11-21-millennium-bsd-navier-stokes.md`

## Global Constraints
- Cero advertencias (`warnings`) ni errores de compilación con `cargo check` y `cargo test`.
- Toda prueba numérica debe basarse en datos matemáticos exactos verificables (LMFDB para curvas elípticas, solución estacionaria para Hou-Luo).
- Preservar compatibilidad con el entorno Termux / Linux (`CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target`).
- Visualización interactiva en tiempo real en `sandbox.html` sin dependencias externas bloqueantes.

## Review Focus
1. **Desbordamiento numérico en discriminantes de curvas elípticas:** Delta = -16(4A^3 + 27B^2) puede crecer rápidamente; usar enteros de 64/128 bits o aritmética controlada.
2. **Singularidad matemática en curvas:** Manejar curvas singulares (Delta = 0) rechazándolas formalmente con error explícito.
3. **Criterio de Beale-Kato-Majda (BKM):** Asegurar que la integral temporal de vorticidad detecte divergencia hiperbólica antes del tiempo crítico T*.
4. **Conservación de masa/circulación en el modelo de fluido:** int omega dx = 0 para perfiles antisimétricos.
5. **Rendimiento de evaluación:** Mantener el evaluador de perfiles de Navier-Stokes por encima de 100.000 iteraciones por segundo.

---

### Task 1: Motor Algebraico de Curvas Elípticas y BSD (`core/src/elliptic_bsd.rs`)

**Files:**
- Create: `core/src/elliptic_bsd.rs`
- Modify: `core/src/lib.rs`
- Test: `core/tests/elliptic_bsd_test.rs`

**Interfaces:**
- Produces: `struct EllipticCurve`, `fn count_points_fp(a, b, p) -> i64`, `fn fourier_trace_ap(a, b, p) -> i64`, `fn root_number(a, b) -> i8`, `fn bsd_invariants(curve) -> BSDInvariants`, `fn synthesize_rank_relation(curves) -> BSDSynthesisResult`.

- [ ] **Step 1: Write the failing test**
  Crear `core/tests/elliptic_bsd_test.rs` probando curvas elípticas canónicas de LMFDB:
  - Curva 11a1: y^2 + y = x^3 - x^2 - 10x - 20 (rango 0, N=11).
  - Curva 37a1: y^2 + y = x^3 - x (rango 1, N=37).
  - Curva 389a1: y^2 + y = x^3 + x^2 - 2x (rango 2, N=389).
  - Curva 5077a1: y^2 + y = x^3 - 7x + 6 (rango 3, N=5077).
  Verificar cálculo de trazas de Frobenius a_p para p en {2, 3, 5, 7} y paridad del número de raíz W(E).

- [ ] **Step 2: Run test to verify it fails**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --test elliptic_bsd_test`
  Expected: FAIL (module not found).

- [ ] **Step 3: Implement `elliptic_bsd.rs`**
  Implementar la aritmética de curvas elípticas, recuento de puntos vía el símbolo de Legendre en F_p, cálculo de trazas a_p = p + 1 - #E(F_p), conductor estimado, número de raíz analítico y optimizador simbólico para invariantes de rango.

- [ ] **Step 4: Run test to verify it passes**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --test elliptic_bsd_test`
  Expected: PASS con código 0.

- [ ] **Step 5: Commit**
  ```bash
  git add core/src/elliptic_bsd.rs core/src/lib.rs core/tests/elliptic_bsd_test.rs
  git commit -m "feat(core): add elliptic curve and Birch-Swinnerton-Dyer algebraic engine"
  ```

---

### Task 2: Motor de Singularidad de Navier-Stokes / Euler (`core/src/navier_stokes.rs`)

**Files:**
- Create: `core/src/navier_stokes.rs`
- Modify: `core/src/lib.rs`
- Test: `core/tests/navier_stokes_test.rs`

**Interfaces:**
- Produces: `struct FluidSimulation1D`, `struct SelfSimilarBlowupProfile`, `fn step_hou_luo(sim, dt)`, `fn compute_enstrophy(sim) -> f64`, `fn check_bkm_blowup(sim) -> Option<f64>`, `fn evolve_blowup_profile(target_scaling) -> BlowupSynthesisResult`.

- [ ] **Step 1: Write the failing test**
  Crear `core/tests/navier_stokes_test.rs` verificando:
  - Inicialización con perfil de vorticidad suave impar omega(x, 0) = sin(2*pi*x).
  - Conservación de media nula int omega dx approx 0.
  - Crecimiento de enstrofía Omega(t) = int omega(x, t)^2 dx.
  - Búsqueda evolutiva de perfil autosimilar con residual de Cauchy-Hilbert < 10^-3.

- [ ] **Step 2: Run test to verify it fails**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --test navier_stokes_test`
  Expected: FAIL (module not found).

- [ ] **Step 3: Implement `navier_stokes.rs`**
  Implementar simulación 1D de Hou-Luo y De Gregorio con transformada discreta de Hilbert, cálculo de enstrofía, detector de BKM blow-up y evaluador de perfiles autosimilares.

- [ ] **Step 4: Run test to verify it passes**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test --test navier_stokes_test`
  Expected: PASS con código 0.

- [ ] **Step 5: Commit**
  ```bash
  git add core/src/navier_stokes.rs core/src/lib.rs core/tests/navier_stokes_test.rs
  git commit -m "feat(core): add Navier-Stokes Hou-Luo finite-time blow-up engine"
  ```

---

### Task 3: Exposición MCP y CLI de los Problemas del Milenio (`cli/src/mcp_tools.rs`)

**Files:**
- Modify: `cli/src/mcp_tools.rs`
- Modify: `cli/src/main.rs`
- Test: `cli/tests/mcp_tools_test.rs`

**Interfaces:**
- Consumes: `darwin_core::elliptic_bsd`, `darwin_core::navier_stokes`.
- Produces: MCP tools y endpoints HTTP `/api/bsd` y `/api/navier_stokes`.

- [ ] **Step 1: Write the failing test in `cli/tests/mcp_tools_test.rs`**
  Agregar pruebas para las nuevas herramientas `darwin_bsd_analysis` y `darwin_navier_stokes_blowup`.

- [ ] **Step 2: Run test to verify it fails**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test -p darwin_cli --test mcp_tools_test`
  Expected: FAIL.

- [ ] **Step 3: Implement endpoints y MCP tools en `cli`**
  Implementar llamadas directas a las rutinas de BSD y Navier-Stokes con formato JSON y Markdown.

- [ ] **Step 4: Run test to verify it passes**
  Run: `CARGO_TARGET_DIR=/data/data/com.termux/files/home/.cargo_target cargo test -p darwin_cli --test mcp_tools_test`
  Expected: PASS con código 0.

- [ ] **Step 5: Commit**
  ```bash
  git add cli/src/mcp_tools.rs cli/src/main.rs cli/tests/mcp_tools_test.rs
  git commit -m "feat(cli): expose BSD and Navier-Stokes engines via MCP and HTTP API"
  ```

---

### Task 4: Integración Visual Interactiva en `sandbox.html`

**Files:**
- Modify: `web/sandbox.html`

**Interfaces:**
- Consumes: Endpoints `/api/bsd` y `/api/navier_stokes`.
- Produces: Opciones en el menú desplegable:
  - "🏆 BSD: Invariantes y Rango de Curvas Elípticas (Criptografía)"
  - "🏆 Navier-Stokes: Vórtice Crítico y Blow-up Autosimilar"
  Renderizado de la curva elíptica y gráfico de vorticidad/enstrofía de Navier-Stokes.

- [ ] **Step 1: Update `web/sandbox.html`**
  Agregar controles interactivos, gráficos de fase y paneles explicativos en español e inglés técnico.

- [ ] **Step 2: Test via browser / curl**
  Verificar que `http://localhost:8080/sandbox` renderiza las dos nuevas opciones sin errores en consola.

- [ ] **Step 3: Commit**
  ```bash
  git add web/sandbox.html
  git commit -m "feat(web): add interactive visualizers for BSD elliptic curves and Navier-Stokes blow-up"
  ```
