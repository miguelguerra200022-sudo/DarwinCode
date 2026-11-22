use crate::genome::graph::GenomeGraph;
use crate::genome::node::{NodeId, NodeType};
use crate::types::Value;
use crate::vm::evaluate_graph_with_order;
use std::collections::HashSet;

/// Optimiza de manera continua todas las constantes numéricas activas de un genoma
/// utilizando el algoritmo no lineal de Levenberg-Marquardt con derivadas numéricas.
///
/// Devuelve el Error Cuadrático Medio (MSE) final alcanzado.
pub fn optimize_graph_constants(
    graph: &mut GenomeGraph,
    dataset: &[(&[Value], Value)],
    max_iters: usize,
) -> f64 {
    if dataset.is_empty() {
        return 0.0;
    }

    let sorted_order = match graph.topological_sort() {
        Ok(s) => s,
        Err(_) => return f64::INFINITY,
    };

    // 1. Identificar únicamente los nodos de constante alcanzables desde la salida
    let active_node_ids = get_reachable_node_ids(graph);
    let mut const_node_ids: Vec<NodeId> = graph
        .nodes
        .iter()
        .filter(|n| {
            active_node_ids.contains(&n.id)
                && matches!(n.node_type, NodeType::Constant(Value::Float(_)))
        })
        .map(|n| n.id)
        .collect();

    const_node_ids.sort();

    let num_params = const_node_ids.len();
    if num_params == 0 {
        return calculate_mse(graph, &sorted_order, dataset);
    }

    // Extraer vector inicial de parámetros c
    let mut current_params: Vec<f64> = const_node_ids
        .iter()
        .map(|&id| {
            match &graph.node(id).unwrap().node_type {
                NodeType::Constant(Value::Float(f)) => *f,
                _ => 0.0,
            }
        })
        .collect();

    let mut current_mse = calculate_mse(graph, &sorted_order, dataset);
    if current_mse < 1e-12 {
        return current_mse;
    }

    let num_samples = dataset.len();
    let mut lambda = 1e-3;

    // Bucle iterativo de Levenberg-Marquardt
    for _ in 0..max_iters {
        // A. Calcular residuos en cada punto: r_i = pred_i - target_i
        let mut residuals = Vec::with_capacity(num_samples);
        let mut preds = Vec::with_capacity(num_samples);
        for (inputs, target) in dataset {
            let p = match evaluate_graph_with_order(graph, &sorted_order, inputs, 10_000) {
                Ok(val) => val.as_float().unwrap_or(0.0),
                Err(_) => 0.0,
            };
            let t = target.as_float().unwrap_or(0.0);
            preds.push(p);
            residuals.push(p - t);
        }

        // B. Matriz Jacobiana J (num_samples x num_params) vía diferencias finitas
        let mut jacobian = vec![vec![0.0; num_params]; num_samples];

        for (p_idx, &node_id) in const_node_ids.iter().enumerate() {
            let orig_val = current_params[p_idx];
            let h = (1e-6 * orig_val.abs()).max(1e-6);

            // Perturbar hacia adelante
            if let Some(node) = graph.node_mut(node_id) {
                node.node_type = NodeType::Constant(Value::Float(orig_val + h));
            }

            for (s_idx, (inputs, _)) in dataset.iter().enumerate() {
                let p_pert = match evaluate_graph_with_order(graph, &sorted_order, inputs, 10_000) {
                    Ok(val) => val.as_float().unwrap_or(0.0),
                    Err(_) => 0.0,
                };
                jacobian[s_idx][p_idx] = (p_pert - preds[s_idx]) / h;
            }

            // Restaurar
            if let Some(node) = graph.node_mut(node_id) {
                node.node_type = NodeType::Constant(Value::Float(orig_val));
            }
        }

        // C. Formar ecuaciones normales: A = J^T * J, g = J^T * r
        let mut jtj = vec![vec![0.0; num_params]; num_params];
        let mut jtr = vec![0.0; num_params];

        for i in 0..num_params {
            for j in 0..num_params {
                let mut sum = 0.0;
                for k in 0..num_samples {
                    sum += jacobian[k][i] * jacobian[k][j];
                }
                jtj[i][j] = sum;
            }

            let mut sum_r = 0.0;
            for k in 0..num_samples {
                sum_r += jacobian[k][i] * residuals[k];
            }
            jtr[i] = sum_r;
        }

        // D. Amortiguamiento Levenberg-Marquardt: A_damp = J^T J + lambda * diag(J^T J)
        let mut a_damp = jtj.clone();
        for i in 0..num_params {
            let diag = a_damp[i][i].abs().max(1e-6);
            a_damp[i][i] += lambda * diag;
        }

        let rhs: Vec<f64> = jtr.iter().map(|&val| -val).collect();

        // E. Resolver sistema lineal para delta_c
        if let Some(delta) = solve_linear_system(a_damp, rhs) {
            let mut candidate_params = current_params.clone();
            for i in 0..num_params {
                candidate_params[i] += delta[i];
            }

            // Aplicar candidatos al grafo
            for (p_idx, &node_id) in const_node_ids.iter().enumerate() {
                if let Some(node) = graph.node_mut(node_id) {
                    node.node_type = NodeType::Constant(Value::Float(candidate_params[p_idx]));
                }
            }

            let new_mse = calculate_mse(graph, &sorted_order, dataset);

            if new_mse < current_mse {
                // Mejora aceptada
                let rel_change = (current_mse - new_mse) / current_mse.max(1e-12);
                current_params = candidate_params;
                current_mse = new_mse;
                lambda = (lambda * 0.25).max(1e-8);

                if rel_change < 1e-7 || current_mse < 1e-10 {
                    break;
                }
            } else {
                // Rechazada: restaurar parámetros y aumentar penalización lambda
                for (p_idx, &node_id) in const_node_ids.iter().enumerate() {
                    if let Some(node) = graph.node_mut(node_id) {
                        node.node_type = NodeType::Constant(Value::Float(current_params[p_idx]));
                    }
                }
                lambda = (lambda * 4.0).min(1e8);
            }
        } else {
            lambda = (lambda * 10.0).min(1e8);
        }
    }

    current_mse
}

fn calculate_mse(
    graph: &GenomeGraph,
    sorted_order: &[usize],
    dataset: &[(&[Value], Value)],
) -> f64 {
    let mut sum_sq = 0.0;
    for (inputs, target) in dataset {
        match evaluate_graph_with_order(graph, sorted_order, inputs, 10_000) {
            Ok(out) => {
                let p = out.as_float().unwrap_or(0.0);
                let t = target.as_float().unwrap_or(0.0);
                let diff = p - t;
                sum_sq += diff * diff;
            }
            Err(_) => return f64::INFINITY,
        }
    }
    sum_sq / (dataset.len() as f64)
}

fn get_reachable_node_ids(graph: &GenomeGraph) -> HashSet<NodeId> {
    let mut reachable = HashSet::new();
    let output_id = match graph.output_node {
        Some(id) => id,
        None => return reachable,
    };

    let mut stack = vec![output_id];
    reachable.insert(output_id);

    while let Some(curr) = stack.pop() {
        if let Some(node) = graph.node(curr) {
            if let NodeType::Operator { inputs, .. } = &node.node_type {
                for &inp in inputs {
                    if reachable.insert(inp) {
                        stack.push(inp);
                    }
                }
            }
        }
    }

    reachable
}

/// Solucionador de sistemas lineales A x = b mediante eliminación gaussiana con pivoteo parcial.
fn solve_linear_system(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for i in 0..n {
        let mut max_row = i;
        let mut max_val = a[i][i].abs();
        for k in (i + 1)..n {
            if a[k][i].abs() > max_val {
                max_val = a[k][i].abs();
                max_row = k;
            }
        }

        if max_val < 1e-12 {
            return None;
        }

        a.swap(i, max_row);
        b.swap(i, max_row);

        for k in (i + 1)..n {
            let factor = a[k][i] / a[i][i];
            for j in i..n {
                a[k][j] -= factor * a[i][j];
            }
            b[k] -= factor * b[i];
        }
    }

    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = b[i];
        for j in (i + 1)..n {
            sum -= a[i][j] * x[j];
        }
        x[i] = sum / a[i][i];
    }

    Some(x)
}
