use crate::genome::graph::GenomeGraph;
use crate::genome::node::NodeType;
use crate::types::{safe_div, safe_mod, safe_sqrt, OpCode, Value};
use crate::vm::gas::{GasMeter, VmError};

pub fn evaluate_graph(
    graph: &GenomeGraph,
    inputs: &[Value],
    gas_limit: u64,
) -> Result<Value, VmError> {
    let sorted_order = graph
        .topological_sort()
        .map_err(|_| VmError::EvaluationError("Topological sort falló en ejecución"))?;
    evaluate_graph_with_order(graph, &sorted_order, inputs, gas_limit)
}

/// Evaluación de alto rendimiento reutilizando el orden topológico pre-calculado y memoria indexada O(1)
pub fn evaluate_graph_with_order(
    graph: &GenomeGraph,
    sorted_order: &[usize],
    inputs: &[Value],
    gas_limit: u64,
) -> Result<Value, VmError> {
    let output_id = graph.output_node.ok_or(VmError::UnsetOutput)?;
    let mut gas_meter = GasMeter::new(gas_limit);

    let max_id = graph.nodes.iter().map(|n| n.id).max().unwrap_or(0);
    let mut env: Vec<Option<Value>> = vec![None; max_id + 1];

    // Tabla de búsqueda directa O(1) para evitar escaneo lineal de nodos
    let mut node_lut: Vec<Option<&crate::genome::node::GenomeNode>> = vec![None; max_id + 1];
    for n in &graph.nodes {
        if n.id <= max_id {
            node_lut[n.id] = Some(n);
        }
    }

    for &id in sorted_order {
        gas_meter.consume(1)?;
        let node = node_lut
            .get(id)
            .and_then(|&n| n)
            .ok_or(VmError::InvalidNode(id))?;

        let val = match &node.node_type {
            NodeType::Input { index, .. } => inputs
                .get(*index)
                .cloned()
                .ok_or(VmError::MissingInput(*index))?,
            NodeType::Constant(val) => val.clone(),
            NodeType::Operator { op, inputs, .. } => {
                eval_operator_fast(*op, inputs, &env, &mut gas_meter)?
            }
        };
        if id < env.len() {
            env[id] = Some(val);
        }
    }

    env.get(output_id)
        .and_then(|v| v.clone())
        .ok_or(VmError::InvalidNode(output_id))
}

#[inline]
fn eval_operator_fast(
    op: OpCode,
    input_ids: &[usize],
    env: &[Option<Value>],
    gas_meter: &mut GasMeter,
) -> Result<Value, VmError> {
    gas_meter.consume(1)?;

    let get_val = |idx: usize| -> Result<&Value, VmError> {
        let id = *input_ids.get(idx).ok_or(VmError::TypeMismatch)?;
        env.get(id)
            .and_then(|opt| opt.as_ref())
            .ok_or(VmError::InvalidNode(id))
    };

    match op {
        OpCode::Add => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(a + b))
        }
        OpCode::Subtract => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(a - b))
        }
        OpCode::Multiply => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(a * b))
        }
        OpCode::Divide => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(safe_div(a, b)))
        }
        OpCode::Modulo => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(safe_mod(a, b)))
        }
        OpCode::Absolute => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(a.abs()))
        }
        OpCode::SquareRoot => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(safe_sqrt(a)))
        }
        OpCode::Sin => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(a.sin()))
        }
        OpCode::Cos => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(a.cos()))
        }
        OpCode::Min => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(a.min(b)))
        }
        OpCode::Max => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(a.max(b)))
        }
        OpCode::LessThan => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Boolean(a < b))
        }
        OpCode::GreaterThan => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Boolean(a > b))
        }
        OpCode::Equal => {
            let a = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Boolean((a - b).abs() < 1e-6))
        }
        OpCode::And => {
            let a = get_val(0)?.as_bool().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_bool().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Boolean(a && b))
        }
        OpCode::Or => {
            let a = get_val(0)?.as_bool().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_bool().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Boolean(a || b))
        }
        OpCode::Not => {
            let a = get_val(0)?.as_bool().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Boolean(!a))
        }
        OpCode::IfElse => {
            let cond = get_val(0)?.as_bool().ok_or(VmError::TypeMismatch)?;
            let t = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            let f = get_val(2)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Float(if cond { t } else { f }))
        }
        OpCode::Vec3Make => {
            let x = get_val(0)?.as_float().ok_or(VmError::TypeMismatch)?;
            let y = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            let z = get_val(2)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Vec3([x, y, z]))
        }
        OpCode::Vec3Add => {
            let a = get_val(0)?.as_vec3().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_vec3().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Vec3([a[0] + b[0], a[1] + b[1], a[2] + b[2]]))
        }
        OpCode::Vec3Scale => {
            let v = get_val(0)?.as_vec3().ok_or(VmError::TypeMismatch)?;
            let s = get_val(1)?.as_float().ok_or(VmError::TypeMismatch)?;
            Ok(Value::Vec3([v[0] * s, v[1] * s, v[2] * s]))
        }
        OpCode::Vec3Length => {
            let v = get_val(0)?.as_vec3().ok_or(VmError::TypeMismatch)?;
            let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            Ok(Value::Float(len))
        }
        OpCode::Vec3Dot => {
            let a = get_val(0)?.as_vec3().ok_or(VmError::TypeMismatch)?;
            let b = get_val(1)?.as_vec3().ok_or(VmError::TypeMismatch)?;
            let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
            Ok(Value::Float(dot))
        }
    }
}
