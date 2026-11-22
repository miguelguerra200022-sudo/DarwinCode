use crate::types::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymmetryReport {
    pub symmetric_pairs: Vec<(usize, usize)>,
    pub even_variables: Vec<usize>,
    pub odd_variables: Vec<usize>,
}

impl SymmetryReport {
    pub fn new() -> Self {
        Self {
            symmetric_pairs: Vec::new(),
            even_variables: Vec::new(),
            odd_variables: Vec::new(),
        }
    }

    pub fn is_symmetric_pair(&self, a: usize, b: usize) -> bool {
        self.symmetric_pairs
            .iter()
            .any(|&(x, y)| (x == a && y == b) || (x == b && y == a))
    }

    pub fn is_even_variable(&self, idx: usize) -> bool {
        self.even_variables.contains(&idx)
    }

    pub fn is_odd_variable(&self, idx: usize) -> bool {
        self.odd_variables.contains(&idx)
    }
}

/// Detecta simetrías de permutación y paridad (par/impar) en un conjunto de observaciones.
pub fn detect_symmetries(
    dataset: &[(&[Value], Value)],
    num_inputs: usize,
    tolerance: f64,
) -> SymmetryReport {
    let mut report = SymmetryReport::new();
    if dataset.len() < 2 || num_inputs == 0 {
        return report;
    }

    // 1. Detección de Simetría de Permutación: f(..., x_i, ..., x_j, ...) == f(..., x_j, ..., x_i, ...)
    for i in 0..num_inputs {
        for j in (i + 1)..num_inputs {
            let mut pair_count = 0;
            let mut all_match = true;

            for a_idx in 0..dataset.len() {
                let (a_in, a_out) = &dataset[a_idx];
                let a_val_i = a_in.get(i).and_then(|v| v.as_float()).unwrap_or(0.0);
                let a_val_j = a_in.get(j).and_then(|v| v.as_float()).unwrap_or(0.0);
                let a_target = a_out.as_float().unwrap_or(0.0);

                if (a_val_i - a_val_j).abs() < 1e-6 {
                    continue; // Puntos idénticos en la diagonal u == v no determinan permutación
                }

                for b_idx in (a_idx + 1)..dataset.len() {
                    let (b_in, b_out) = &dataset[b_idx];
                    let b_val_i = b_in.get(i).and_then(|v| v.as_float()).unwrap_or(0.0);
                    let b_val_j = b_in.get(j).and_then(|v| v.as_float()).unwrap_or(0.0);
                    let b_target = b_out.as_float().unwrap_or(0.0);

                    // Verificar transposición cruzada: a_i == b_j y a_j == b_i
                    if (a_val_i - b_val_j).abs() < 1e-5 && (a_val_j - b_val_i).abs() < 1e-5 {
                        // Todas las demás entradas deben ser idénticas
                        let mut others_match = true;
                        for m in 0..num_inputs {
                            if m != i && m != j {
                                let m_a = a_in.get(m).and_then(|v| v.as_float()).unwrap_or(0.0);
                                let m_b = b_in.get(m).and_then(|v| v.as_float()).unwrap_or(0.0);
                                if (m_a - m_b).abs() > 1e-5 {
                                    others_match = false;
                                    break;
                                }
                            }
                        }

                        if others_match {
                            pair_count += 1;
                            if (a_target - b_target).abs() > tolerance {
                                all_match = false;
                                break;
                            }
                        }
                    }
                }

                if !all_match {
                    break;
                }
            }

            if pair_count > 0 && all_match {
                report.symmetric_pairs.push((i, j));
            }
        }
    }

    // 2. Detección de Paridad (Simetría Par e Impar): f(..., -x_i, ...) vs f(..., x_i, ...)
    for i in 0..num_inputs {
        let mut pair_count = 0;
        let mut is_even = true;
        let mut is_odd = true;

        for a_idx in 0..dataset.len() {
            let (a_in, a_out) = &dataset[a_idx];
            let a_val_i = a_in.get(i).and_then(|v| v.as_float()).unwrap_or(0.0);
            let a_target = a_out.as_float().unwrap_or(0.0);

            if a_val_i.abs() < 1e-6 {
                continue; // x_i == 0 no discrimina paridad
            }

            for b_idx in (a_idx + 1)..dataset.len() {
                let (b_in, b_out) = &dataset[b_idx];
                let b_val_i = b_in.get(i).and_then(|v| v.as_float()).unwrap_or(0.0);
                let b_target = b_out.as_float().unwrap_or(0.0);

                // Verificar signo opuesto: a_val_i + b_val_i == 0
                if (a_val_i + b_val_i).abs() < 1e-5 {
                    let mut others_match = true;
                    for m in 0..num_inputs {
                        if m != i {
                            let m_a = a_in.get(m).and_then(|v| v.as_float()).unwrap_or(0.0);
                            let m_b = b_in.get(m).and_then(|v| v.as_float()).unwrap_or(0.0);
                            if (m_a - m_b).abs() > 1e-5 {
                                others_match = false;
                                break;
                            }
                        }
                    }

                    if others_match {
                        pair_count += 1;
                        if (a_target - b_target).abs() > tolerance {
                            is_even = false;
                        }
                        if (a_target + b_target).abs() > tolerance {
                            is_odd = false;
                        }
                    }
                }
            }
        }

        if pair_count > 0 {
            if is_even {
                report.even_variables.push(i);
            }
            if is_odd {
                report.odd_variables.push(i);
            }
        }
    }

    report
}

/// Aumenta el conjunto de datos con los invariantes simétricos fundamentales descubiertos.
pub fn augment_dataset_with_symmetries(
    dataset: &[(&[Value], Value)],
    input_names: &[&str],
    report: &SymmetryReport,
) -> (Vec<(Vec<Value>, Value)>, Vec<String>) {
    let mut names: Vec<String> = input_names.iter().map(|s| s.to_string()).collect();

    // Especificación de nuevas columnas a generar
    enum VirtualCol {
        PairSum(usize, usize),
        PairMul(usize, usize),
        Square(usize),
    }

    let mut virtual_cols = Vec::new();

    // 1. Añadir polinomios simétricos elementales S = u + v, P = u * v
    for &(i, j) in &report.symmetric_pairs {
        let name_i = input_names.get(i).copied().unwrap_or("x");
        let name_j = input_names.get(j).copied().unwrap_or("y");

        let sum_name = format!("{}_plus_{}", name_i, name_j);
        let mul_name = format!("{}_mul_{}", name_i, name_j);

        names.push(sum_name);
        virtual_cols.push(VirtualCol::PairSum(i, j));

        names.push(mul_name);
        virtual_cols.push(VirtualCol::PairMul(i, j));
    }

    // 2. Añadir variables cuadráticas para variables de paridad par
    for &i in &report.even_variables {
        let name_i = input_names.get(i).copied().unwrap_or("x");
        let sq_name = format!("sq_{}", name_i);

        if !names.contains(&sq_name) {
            names.push(sq_name);
            virtual_cols.push(VirtualCol::Square(i));
        }
    }

    // Generar dataset aumentado
    let mut augmented = Vec::with_capacity(dataset.len());

    for (inputs, target) in dataset {
        let mut row: Vec<Value> = inputs.to_vec();

        for col in &virtual_cols {
            match col {
                VirtualCol::PairSum(i, j) => {
                    let v_i = inputs.get(*i).and_then(|v| v.as_float()).unwrap_or(0.0);
                    let v_j = inputs.get(*j).and_then(|v| v.as_float()).unwrap_or(0.0);
                    row.push(Value::Float(v_i + v_j));
                }
                VirtualCol::PairMul(i, j) => {
                    let v_i = inputs.get(*i).and_then(|v| v.as_float()).unwrap_or(0.0);
                    let v_j = inputs.get(*j).and_then(|v| v.as_float()).unwrap_or(0.0);
                    row.push(Value::Float(v_i * v_j));
                }
                VirtualCol::Square(i) => {
                    let v_i = inputs.get(*i).and_then(|v| v.as_float()).unwrap_or(0.0);
                    row.push(Value::Float(v_i * v_i));
                }
            }
        }

        augmented.push((row, target.clone()));
    }

    (augmented, names)
}
