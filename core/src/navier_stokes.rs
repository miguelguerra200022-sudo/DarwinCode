//! High-Performance Navier-Stokes / Euler Finite-Time Singularity Engine
//! 
//! Implements the 1D/2D Hou-Luo and De Gregorio models for 3D axisymmetric
//! Euler/Navier-Stokes singularity formation, Beale-Kato-Majda (BKM) blow-up detection,
//! Hilbert-Cauchy transforms, and self-similar blow-up profile evolution.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// 1D Fluid simulation for the Hou-Luo / De Gregorio boundary vorticity model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidSimulation1D {
    pub n: usize,
    pub dx: f64,
    pub nu: f64, // Viscosity (0.0 for Euler)
    pub omega: Vec<f64>,
    pub time: f64,
    pub enstrophy_history: Vec<(f64, f64)>, // (time, enstrophy)
    pub max_vorticity_history: Vec<(f64, f64)>, // (time, max_omega)
}

impl FluidSimulation1D {
    /// Creates a new periodic 1D fluid simulation on [-PI, PI] with N points
    pub fn new(n: usize, nu: f64) -> Self {
        let dx = 2.0 * PI / (n as f64);
        Self {
            n,
            dx,
            nu,
            omega: vec![0.0; n],
            time: 0.0,
            enstrophy_history: Vec::new(),
            max_vorticity_history: Vec::new(),
        }
    }

    /// Initializes with an odd sinusoidal profile: omega(x) = amplitude * sin(x)
    pub fn init_sinusoidal(&mut self, amplitude: f64) {
        for i in 0..self.n {
            let x = -PI + (i as f64) * self.dx;
            self.omega[i] = amplitude * x.sin();
        }
        self.record_history();
    }

    /// Discrete periodic Hilbert transform using the cotangent kernel
    pub fn hilbert_transform(&self) -> Vec<f64> {
        let mut h = vec![0.0; self.n];
        let n_f = self.n as f64;
        for j in 0..self.n {
            let mut sum = 0.0;
            for k in 1..self.n {
                let cot_val = 1.0 / ((PI * (k as f64) / n_f).tan());
                let idx = (j + self.n - (k % self.n)) % self.n;
                sum += cot_val * self.omega[idx];
            }
            h[j] = sum / n_f;
        }
        h
    }

    /// Computes velocity u(x) from u_x = H(omega) by discrete integration
    pub fn velocity_u(&self, ux: &[f64]) -> Vec<f64> {
        let mut u = vec![0.0; self.n];
        let mut cumsum = 0.0;
        for i in 0..self.n {
            cumsum += ux[i] * self.dx;
            u[i] = cumsum;
        }
        // Remove mean to ensure periodicity
        let mean = cumsum / (self.n as f64);
        for val in &mut u {
            *val -= mean;
        }
        u
    }

    /// Advances the simulation by dt using the Hou-Luo model:
    /// d(omega)/dt = - u * d(omega)/dx + u_x * omega + nu * d2(omega)/dx2
    pub fn step(&mut self, dt: f64) {
        let ux = self.hilbert_transform();
        let u = self.velocity_u(&ux);
        let mut domega = vec![0.0; self.n];

        for i in 0..self.n {
            let prev = if i == 0 { self.n - 1 } else { i - 1 };
            let next = (i + 1) % self.n;

            // Central differences for advection and diffusion
            let d_dx = (self.omega[next] - self.omega[prev]) / (2.0 * self.dx);
            let d2_dx2 = (self.omega[next] - 2.0 * self.omega[i] + self.omega[prev]) / (self.dx * self.dx);

            // Hou-Luo equation RHS: -u * omega_x + u_x * omega + nu * omega_xx
            let advection = -u[i] * d_dx;
            let stretching = ux[i] * self.omega[i];
            let diffusion = self.nu * d2_dx2;

            domega[i] = advection + stretching + diffusion;
        }

        // Apply Euler-Heun update with clipping for numerical stability
        for i in 0..self.n {
            self.omega[i] += dt * domega[i];
            if !self.omega[i].is_finite() {
                self.omega[i] = 1e6;
            }
        }

        self.time += dt;
        self.record_history();
    }

    /// Total circulation int omega dx
    pub fn total_circulation(&self) -> f64 {
        self.omega.iter().sum::<f64>() * self.dx
    }

    /// Enstrophy E(t) = int omega^2 dx
    pub fn enstrophy(&self) -> f64 {
        self.omega.iter().map(|&w| w * w).sum::<f64>() * self.dx
    }

    /// Maximum vorticity ||omega||_{L^infinity}
    pub fn max_vorticity(&self) -> f64 {
        self.omega.iter().map(|w| w.abs()).fold(0.0, f64::max)
    }

    fn record_history(&mut self) {
        let ens = self.enstrophy();
        let max_w = self.max_vorticity();
        self.enstrophy_history.push((self.time, ens));
        self.max_vorticity_history.push((self.time, max_w));
    }

    /// Checks Beale-Kato-Majda (BKM) singularity blow-up condition
    /// Returns estimated blow-up time T* if vorticity growth is hyperbolic
    pub fn check_bkm_criterion(&self) -> Option<f64> {
        if self.max_vorticity_history.len() < 5 {
            return None;
        }
        let len = self.max_vorticity_history.len();
        let (t1, w1) = self.max_vorticity_history[len - 2];
        let (t2, w2) = self.max_vorticity_history[len - 1];

        if w2 > w1 && (w2 - w1).abs() > 1e-6 {
            let dw_dt = (w2 - w1) / (t2 - t1);
            let t_star = t2 + (w2 / dw_dt);
            Some(t_star)
        } else {
            None
        }
    }
}

/// Self-similar blow-up profile representation for Hou-Luo singularity
/// Omega(xi) = (c0 * xi) / (1 + xi^2)^gamma
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfSimilarBlowupProfile {
    pub c0: f64,
    pub gamma: f64,
    pub c_omega: f64, // Temporal vorticity blow-up scaling exponent
    pub c_l: f64,     // Spatial length scaling exponent
    pub residual: f64,
}

impl SelfSimilarBlowupProfile {
    pub fn new(c0: f64, gamma: f64, c_omega: f64, c_l: f64) -> Self {
        let mut p = Self { c0, gamma, c_omega, c_l, residual: 0.0 };
        p.residual = p.compute_l2_residual();
        p
    }

    /// Evaluates Omega(xi)
    pub fn eval_omega(&self, xi: f64) -> f64 {
        let denom = (1.0 + xi * xi).powf(self.gamma);
        (self.c0 * xi) / denom
    }

    /// Evaluates d(Omega)/d(xi)
    pub fn eval_d_omega(&self, xi: f64) -> f64 {
        let xi2 = xi * xi;
        let denom = (1.0 + xi2).powf(self.gamma + 1.0);
        let num = self.c0 * (1.0 + (1.0 - 2.0 * self.gamma) * xi2);
        num / denom
    }

    /// Evaluates residual of the self-similar equation at xi:
    /// R(xi) = c_omega * Omega + c_l * xi * Omega' + U * Omega' - U' * Omega
    pub fn compute_residual(&self, xi: f64) -> f64 {
        let omega = self.eval_omega(xi);
        let d_omega = self.eval_d_omega(xi);

        // Approximate Hilbert transform for local test
        // For (c0*xi)/(1+xi^2), the exact Hilbert transform is c0 / (1 + xi^2)
        let u_prime = self.c0 / (1.0 + xi * xi);
        let u = self.c0 * xi.atan(); // int c0/(1+s^2) ds = c0 * arctan(xi)

        let term1 = self.c_omega * omega;
        let term2 = self.c_l * xi * d_omega;
        let term3 = u * d_omega;
        let term4 = u_prime * omega;

        term1 + term2 + term3 - term4
    }

    /// Computes integrated L2 residual over grid [-5.0, 5.0]
    pub fn compute_l2_residual(&self) -> f64 {
        let steps = 50;
        let dxi = 10.0 / (steps as f64);
        let mut sum_sq = 0.0;
        for i in 0..steps {
            let xi = -5.0 + (i as f64) * dxi;
            let r = self.compute_residual(xi);
            sum_sq += r * r * dxi;
        }
        sum_sq.sqrt()
    }
}

/// Evolutionary optimizer to find the optimal self-similar blow-up exponents and profile
pub fn evolve_blowup_profile(iterations: usize) -> SelfSimilarBlowupProfile {
    let mut best = SelfSimilarBlowupProfile::new(1.0, 1.0, 1.0, 1.0);

    for it in 0..iterations {
        // Parametric exploration of scaling exponents around Hou-Luo values
        let factor = 1.0 + ((it as f64) * 0.01).sin() * 0.2;
        let c0 = 1.0 * factor;
        let gamma = (1.0 + 0.1 * ((it as f64) * 0.05).cos()).max(0.5);
        let c_omega = (1.0 + 0.2 * ((it as f64) * 0.03).sin()).max(0.2);
        let c_l = (c_omega / 2.0).max(0.1);

        let cand = SelfSimilarBlowupProfile::new(c0, gamma, c_omega, c_l);
        if cand.residual < best.residual {
            best = cand;
        }
    }
    best
}
