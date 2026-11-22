//! Motor de Fracciones Continuas Generalizadas de Ramanujan (Ramanujan Machine Engine)
//! Permite evaluar, descubrir y verificar identidades matemáticas profundas y conjeturas
//! abiertas para constantes matemáticas fundamentales (Catalan G, Apéry zeta(3), pi, e).

use serde::{Deserialize, Serialize};

/// Constantes matemáticas universales de alta precisión.
pub const CONST_CATALAN_G: f64 = 0.91596559417721901505;
pub const CONST_APERY_ZETA3: f64 = 1.20205690315959428540;
pub const CONST_ZETA_5: f64 = 1.03692775514336992633;
pub const CONST_GAMMA: f64 = 0.57721566490153286060;
pub const CONST_PI: f64 = 3.14159265358979323846;
pub const CONST_E: f64 = 2.71828182845904523536;
pub const CONST_LN2: f64 = 0.69314718055994530941;

/// Representación polinomial P(n) = c0 + c1*n + c2*n^2 + c3*n^3 + c4*n^4.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polynomial {
    pub coeffs: Vec<f64>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<f64>) -> Self {
        Polynomial { coeffs }
    }

    #[inline]
    pub fn eval(&self, n: f64) -> f64 {
        let mut res = 0.0;
        let mut term = 1.0;
        for &c in &self.coeffs {
            res += c * term;
            term *= n;
        }
        res
    }

    pub fn to_formula_string(&self, var: &str) -> String {
        if self.coeffs.is_empty() || self.coeffs.iter().all(|&c| c.abs() < 1e-9) {
            return "0".to_string();
        }
        let mut parts = Vec::new();
        for (pow, &c) in self.coeffs.iter().enumerate() {
            if c.abs() < 1e-9 {
                continue;
            }
            let c_round = (c * 1000.0).round() / 1000.0;
            if pow == 0 {
                parts.push(format!("{}", c_round));
            } else if pow == 1 {
                if (c_round - 1.0).abs() < 1e-9 {
                    parts.push(var.to_string());
                } else if (c_round + 1.0).abs() < 1e-9 {
                    parts.push(format!("-{}", var));
                } else {
                    parts.push(format!("{}*{}", c_round, var));
                }
            } else {
                if (c_round - 1.0).abs() < 1e-9 {
                    parts.push(format!("{}^{}", var, pow));
                } else {
                    parts.push(format!("{}*{}^{}", c_round, var, pow));
                }
            }
        }
        if parts.is_empty() { "0".to_string() } else { parts.join(" + ") }
    }
}

/// Fracción Continua Generalizada:
/// C = b0 + a1 / (b1 + a2 / (b2 + a3 / (b3 + ...)))
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralizedContinuedFraction {
    pub b0: f64,
    pub poly_a: Polynomial,
    pub poly_b: Polynomial,
    pub name: String,
    pub target_constant: f64,
    #[serde(default)]
    pub custom_b: Option<Vec<f64>>,
}

/// Registro de convergencia paso a paso.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceStep {
    pub depth: usize,
    pub convergent_value: f64,
    pub absolute_error: f64,
    pub matching_digits: usize,
}

impl GeneralizedContinuedFraction {
    /// Evalúa la fracción continua hasta una profundidad N usando el Algoritmo Modificado de Lentz.
    pub fn evaluate_lentz(&self, max_depth: usize) -> (f64, Vec<ConvergenceStep>) {
        const TINY: f64 = 1e-30;
        let mut f = if self.b0.abs() < TINY { TINY } else { self.b0 };
        let mut c = f;
        let mut d = 0.0;

        let mut steps = Vec::with_capacity(max_depth);

        for j in 1..=max_depth {
            let n = j as f64;
            let aj = self.poly_a.eval(n);
            let bj = if let Some(ref cb) = self.custom_b {
                if j <= cb.len() {
                    cb[j - 1]
                } else {
                    self.poly_b.eval(n)
                }
            } else {
                self.poly_b.eval(n)
            };

            d = bj + aj * d;
            if d.abs() < TINY { d = TINY; }
            d = 1.0 / d;

            c = bj + aj / c;
            if c.abs() < TINY { c = TINY; }

            let delta = c * d;
            f *= delta;

            let abs_err = (f - self.target_constant).abs();
            let digits = if abs_err > 0.0 {
                (-abs_err.log10()).max(0.0).floor() as usize
            } else {
                16
            };

            steps.push(ConvergenceStep {
                depth: j,
                convergent_value: f,
                absolute_error: abs_err,
                matching_digits: digits.min(16),
            });
        }

        (f, steps)
    }

    /// Crea la conjetura de la Ramanujan Machine para la Constante de Catalan G:
    /// 1/G = 1 + 1^4 / (8 + 3^4 / (16 + 5^4 / (24 + 7^4 / (32 + ...))))
    pub fn catalan_ramanujan_conjecture() -> Self {
        // a_n = (2n - 1)^4 = 16n^4 - 32n^3 + 24n^2 - 8n + 1
        // b_n = 8n
        // Converge hacia 1 / G = 1.09174406370428
        GeneralizedContinuedFraction {
            b0: 1.0,
            poly_a: Polynomial::new(vec![1.0, -8.0, 24.0, -32.0, 16.0]),
            poly_b: Polynomial::new(vec![0.0, 8.0]),
            name: "Conjetura Ramanujan Machine para 1/G (Catalan)".to_string(),
            target_constant: 1.0 / CONST_CATALAN_G,
            custom_b: None,
        }
    }

    /// Crea la fracción continua de Euler/Brouncker para 4/pi:
    /// 4/pi = 1 + 1^2 / (2 + 3^2 / (2 + 5^2 / (2 + ...)))
    pub fn pi_brouncker() -> Self {
        GeneralizedContinuedFraction {
            b0: 1.0,
            // a_n = (2n - 1)^2 = 4n^2 - 4n + 1
            poly_a: Polynomial::new(vec![1.0, -4.0, 4.0]),
            poly_b: Polynomial::new(vec![2.0]),
            name: "Fracción Continua de Brouncker para 4/pi".to_string(),
            target_constant: 4.0 / CONST_PI,
            custom_b: None,
        }
    }

    /// Crea la fracción continua canónica de la Ramanujan Machine para e (Nature 2021):
    /// e = 3 + (-1) / (4 + (-2) / (5 + (-3) / (6 + ...)))
    pub fn euler_e() -> Self {
        GeneralizedContinuedFraction {
            b0: 3.0,
            poly_a: Polynomial::new(vec![0.0, -1.0]), // a_n = -n
            poly_b: Polynomial::new(vec![3.0, 1.0]),  // b_n = n + 3
            name: "Fracción Continua de Ramanujan Machine para e".to_string(),
            target_constant: CONST_E,
            custom_b: None,
        }
    }

    /// Conjetura de Ramanujan / Apéry para la Constante de Apéry zeta(3):
    /// 6 / zeta(3) = 5 - 1^6 / (117 - 2^6 / (535 - 3^6 / ...))
    pub fn apery_zeta3_ramanujan() -> Self {
        GeneralizedContinuedFraction {
            b0: 5.0,
            // a_n = -n^6
            poly_a: Polynomial::new(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0]),
            // b_n = 34n^3 + 51n^2 + 27n + 5
            poly_b: Polynomial::new(vec![5.0, 27.0, 51.0, 34.0]),
            name: "Conjetura Ramanujan-Apéry para 6/zeta(3)".to_string(),
            target_constant: 6.0 / CONST_APERY_ZETA3,
            custom_b: None,
        }
    }

    /// Fracción continua regular canónica para la constante zeta(5) de Riemann (OEIS A013681):
    /// zeta(5) = [1; 27, 12, 1, 1, 15, 1, 5, 1, 2, 19, 1, 1, 32, 1, 13, 1, 1, 1, 3, ...]
    pub fn zeta5_canonical() -> Self {
        GeneralizedContinuedFraction {
            b0: 1.0,
            poly_a: Polynomial::new(vec![1.0]),
            poly_b: Polynomial::new(vec![1.0]),
            name: "Fracción Continua Canónica de Riemann ζ(5) (OEIS A013681)".to_string(),
            target_constant: CONST_ZETA_5,
            custom_b: Some(vec![
                27.0, 12.0, 1.0, 1.0, 15.0, 1.0, 5.0, 1.0, 2.0, 19.0,
                1.0, 1.0, 32.0, 1.0, 13.0, 1.0, 1.0, 1.0, 3.0, 1.0,
                2.0, 3.0, 1.0, 3.0, 1.0, 5.0, 1.0, 11.0, 1.0, 9.0,
            ]),
        }
    }

    /// Fracción continua regular canónica para la constante de Euler-Mascheroni gamma (OEIS A002852):
    /// gamma = [0; 1, 1, 2, 1, 2, 1, 4, 3, 13, 5, 1, 1, 8, 1, 2, 4, 1, 1, 40, ...]
    pub fn gamma_canonical() -> Self {
        GeneralizedContinuedFraction {
            b0: 0.0,
            poly_a: Polynomial::new(vec![1.0]),
            poly_b: Polynomial::new(vec![1.0]),
            name: "Fracción Continua Canónica de Euler-Mascheroni γ (OEIS A002852)".to_string(),
            target_constant: CONST_GAMMA,
            custom_b: Some(vec![
                1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 4.0, 3.0, 13.0, 5.0,
                1.0, 1.0, 8.0, 1.0, 2.0, 4.0, 1.0, 1.0, 40.0, 1.0,
                11.0, 3.0, 7.0, 1.0, 7.0, 1.0, 1.0, 5.0, 1.0, 49.0,
            ]),
        }
    }

    /// Candidato polinomial cuadrático descubierto por DarwinCode para zeta(5):
    /// a_n = n^2, b_n = 24n + 3
    pub fn zeta5_ramanujan_candidate() -> Self {
        GeneralizedContinuedFraction {
            b0: 1.0,
            poly_a: Polynomial::new(vec![0.0, 0.0, 1.0]),
            poly_b: Polynomial::new(vec![3.0, 24.0]),
            name: "Candidato Polinomial DarwinCode para ζ(5)".to_string(),
            target_constant: CONST_ZETA_5,
            custom_b: None,
        }
    }

    /// Candidato polinomial de aproximación descubierto por DarwinCode para gamma:
    /// a_n = n + 1, b_n = 3n
    pub fn gamma_ramanujan_candidate() -> Self {
        GeneralizedContinuedFraction {
            b0: 0.0,
            poly_a: Polynomial::new(vec![1.0, 1.0]),
            poly_b: Polynomial::new(vec![0.0, 3.0]),
            name: "Candidato Polinomial DarwinCode para γ".to_string(),
            target_constant: CONST_GAMMA,
            custom_b: None,
        }
    }
}

/// Motor de búsqueda evolutiva de nuevas conjeturas para constantes matemáticas.
pub struct RamanujanSearchEngine {
    pub target: f64,
    pub target_name: String,
    pub best_gcf: GeneralizedContinuedFraction,
    pub best_error: f64,
    pub best_matching_digits: usize,
}

impl RamanujanSearchEngine {
    pub fn new(target: f64, target_name: &str) -> Self {
        let default_gcf = GeneralizedContinuedFraction {
            b0: target.floor(),
            poly_a: Polynomial::new(vec![1.0]),
            poly_b: Polynomial::new(vec![2.0]),
            name: format!("Búsqueda de conjetura para {}", target_name),
            target_constant: target,
            custom_b: None,
        };
        RamanujanSearchEngine {
            target,
            target_name: target_name.to_string(),
            best_gcf: default_gcf,
            best_error: 1.0,
            best_matching_digits: 0,
        }
    }

    /// Ejecuta una búsqueda estocástica guiada en el espacio de celosías polinomiales enteras.
    pub fn search(&mut self, iterations: usize, depth: usize, seed: u64) {
        let mut rng_state = seed;
        let mut pseudo_rng = || -> f64 {
            rng_state ^= rng_state << 13;
            rng_state ^= rng_state >> 7;
            rng_state ^= rng_state << 17;
            (rng_state as f64) / (u64::MAX as f64)
        };

        for _ in 0..iterations {
            let b0 = (self.target.floor() + (pseudo_rng() * 3.0).floor() - 1.0).max(0.0);

            // Generar coeficientes enteros pequeños para poly_a (hasta grado 2)
            let a0 = ((pseudo_rng() * 7.0).floor() - 3.0) as f64;
            let a1 = ((pseudo_rng() * 7.0).floor() - 3.0) as f64;
            let a2 = ((pseudo_rng() * 5.0).floor()) as f64; // Cuadrático a menudo positivo

            // Generar coeficientes enteros para poly_b (hasta grado 1)
            let b_const = ((pseudo_rng() * 10.0).floor() + 1.0) as f64;
            let b_lin = ((pseudo_rng() * 6.0).floor()) as f64;

            let candidate = GeneralizedContinuedFraction {
                b0,
                poly_a: Polynomial::new(vec![a0, a1, a2]),
                poly_b: Polynomial::new(vec![b_const, b_lin]),
                name: format!("Candidato para {}", self.target_name),
                target_constant: self.target,
                custom_b: None,
            };

            let (val, _) = candidate.evaluate_lentz(depth);
            if !val.is_nan() && !val.is_infinite() {
                let err = (val - self.target).abs();
                if err < self.best_error {
                    self.best_error = err;
                    self.best_gcf = candidate;
                    let digits = if err > 0.0 { (-err.log10()).max(0.0).floor() as usize } else { 16 };
                    self.best_matching_digits = digits.min(16);
                }
            }
        }
    }

    /// Ejecuta una búsqueda estocástica guiada en celosías polinomiales de grados superiores (hasta grado 6).
    pub fn search_with_degree(&mut self, max_deg_a: usize, max_deg_b: usize, iterations: usize, depth: usize, seed: u64) {
        let mut rng_state = seed;
        let mut pseudo_rng = || -> f64 {
            rng_state ^= rng_state << 13;
            rng_state ^= rng_state >> 7;
            rng_state ^= rng_state << 17;
            (rng_state as f64) / (u64::MAX as f64)
        };

        for _ in 0..iterations {
            let b0 = (self.target.floor() + (pseudo_rng() * 3.0).floor() - 1.0).max(0.0);

            let mut coeffs_a = Vec::with_capacity(max_deg_a + 1);
            for d in 0..=max_deg_a {
                let range = if d == 0 { 7.0 } else { 5.0 };
                let val = ((pseudo_rng() * range).floor() - (range / 2.0).floor()) as f64;
                coeffs_a.push(val);
            }

            let mut coeffs_b = Vec::with_capacity(max_deg_b + 1);
            for d in 0..=max_deg_b {
                let val = if d == 0 {
                    ((pseudo_rng() * 15.0).floor() + 1.0) as f64
                } else {
                    ((pseudo_rng() * 30.0).floor() + 1.0) as f64
                };
                coeffs_b.push(val);
            }

            let candidate = GeneralizedContinuedFraction {
                b0,
                poly_a: Polynomial::new(coeffs_a),
                poly_b: Polynomial::new(coeffs_b),
                name: format!("Candidato de Alto Orden para {}", self.target_name),
                target_constant: self.target,
                custom_b: None,
            };

            let (val, _) = candidate.evaluate_lentz(depth);
            if !val.is_nan() && !val.is_infinite() {
                let err = (val - self.target).abs();
                if err < self.best_error {
                    self.best_error = err;
                    self.best_gcf = candidate;
                    let digits = if err > 0.0 { (-err.log10()).max(0.0).floor() as usize } else { 16 };
                    self.best_matching_digits = digits.min(16);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brouncker_pi_convergence() {
        let gcf = GeneralizedContinuedFraction::pi_brouncker();
        let (val, steps) = gcf.evaluate_lentz(30);
        let target = 4.0 / CONST_PI;
        assert!((val - target).abs() < 0.05, "Brouncker formula must converge towards 4/pi");
        assert!(steps.len() == 30);
    }

    #[test]
    fn test_euler_e_convergence() {
        let gcf = GeneralizedContinuedFraction::euler_e();
        let (val, steps) = gcf.evaluate_lentz(15);
        let target = CONST_E;
        assert!((val - target).abs() < 1e-12, "Euler formula must converge to 12+ decimal places");
        assert!(steps.last().unwrap().matching_digits >= 12);
    }

    #[test]
    fn test_apery_zeta3_convergence() {
        let gcf = GeneralizedContinuedFraction::apery_zeta3_ramanujan();
        let (val, steps) = gcf.evaluate_lentz(12);
        let target = 6.0 / CONST_APERY_ZETA3;
        assert!((val - target).abs() < 1e-12, "Apéry formula must converge to machine precision");
        assert!(steps.last().unwrap().matching_digits >= 12);
    }

    #[test]
    fn test_catalan_constant_ramanujan() {
        let gcf = GeneralizedContinuedFraction::catalan_ramanujan_conjecture();
        let (val, steps) = gcf.evaluate_lentz(25);
        let target = 1.0 / CONST_CATALAN_G;
        assert!((val - target).abs() < 1e-3, "Catalan conjecture must converge");
        assert!(!steps.is_empty());
    }

    #[test]
    fn test_zeta5_canonical_convergence() {
        let gcf = GeneralizedContinuedFraction::zeta5_canonical();
        let (val, steps) = gcf.evaluate_lentz(20);
        let target = CONST_ZETA_5;
        assert!((val - target).abs() < 1e-12, "zeta(5) canonical CF must reach 12+ matching digits");
        assert!(steps.last().unwrap().matching_digits >= 12);
    }

    #[test]
    fn test_gamma_canonical_convergence() {
        let gcf = GeneralizedContinuedFraction::gamma_canonical();
        let (val, steps) = gcf.evaluate_lentz(20);
        let target = CONST_GAMMA;
        assert!((val - target).abs() < 1e-12, "gamma canonical CF must reach 12+ matching digits");
        assert!(steps.last().unwrap().matching_digits >= 12);
    }

    #[test]
    fn test_zeta5_polynomial_candidate() {
        let gcf = GeneralizedContinuedFraction::zeta5_ramanujan_candidate();
        let (val, _steps) = gcf.evaluate_lentz(20);
        let target = CONST_ZETA_5;
        assert!((val - target).abs() < 1e-4, "zeta(5) candidate must approximate target within 1e-4");
    }

    #[test]
    fn test_gamma_polynomial_candidate() {
        let gcf = GeneralizedContinuedFraction::gamma_ramanujan_candidate();
        let (val, _steps) = gcf.evaluate_lentz(20);
        let target = CONST_GAMMA;
        assert!((val - target).abs() < 1e-2, "gamma candidate must approximate target within 1e-2");
    }
}
