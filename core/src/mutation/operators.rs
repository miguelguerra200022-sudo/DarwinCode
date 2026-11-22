use crate::genome::graph::GenomeGraph;
use crate::genome::node::{NodeId, NodeType};
use crate::types::{DataType, OpCode, Value};
use rand::Rng;

pub const ARITHMETIC_OPS: &[OpCode] = &[
    OpCode::Add,
    OpCode::Subtract,
    OpCode::Multiply,
    OpCode::Divide,
];

pub const ADVANCED_OPS: &[OpCode] = &[
    OpCode::Modulo,
    OpCode::Absolute,
    OpCode::SquareRoot,
    OpCode::Sin,
    OpCode::Cos,
    OpCode::Min,
    OpCode::Max,
    OpCode::LessThan,
    OpCode::GreaterThan,
    OpCode::IfElse,
];

pub const AVAILABLE_OPS: &[OpCode] = &[
    OpCode::Add,
    OpCode::Subtract,
    OpCode::Multiply,
    OpCode::Divide,
    OpCode::Modulo,
    OpCode::Absolute,
    OpCode::SquareRoot,
    OpCode::Sin,
    OpCode::Cos,
    OpCode::Min,
    OpCode::Max,
    OpCode::LessThan,
    OpCode::GreaterThan,
    OpCode::IfElse,
];

#[inline]
pub fn pick_random_op(rng: &mut impl Rng) -> OpCode {
    if rng.gen_bool(0.75) {
        ARITHMETIC_OPS[rng.gen_range(0..ARITHMETIC_OPS.len())]
    } else {
        ADVANCED_OPS[rng.gen_range(0..ADVANCED_OPS.len())]
    }
}

#[inline]
pub fn pick_random_op_from(allowed: &[OpCode], rng: &mut impl Rng) -> OpCode {
    if allowed.is_empty() {
        pick_random_op(rng)
    } else {
        allowed[rng.gen_range(0..allowed.len())]
    }
}

/// Genera un genoma aleatorio válido que conecta las entradas dadas con la salida requerida.
pub fn generate_random_genome(
    input_names: &[&str],
    output_type: DataType,
    depth: usize,
    rng: &mut impl Rng,
) -> GenomeGraph {
    generate_random_genome_with_ops(input_names, output_type, depth, AVAILABLE_OPS, rng)
}

/// Genera un genoma restringiendo los operadores a los permitidos (allowed_ops)
pub fn generate_random_genome_with_ops(
    input_names: &[&str],
    output_type: DataType,
    depth: usize,
    allowed_ops: &[OpCode],
    rng: &mut impl Rng,
) -> GenomeGraph {
    let mut graph = GenomeGraph::new(output_type);
    let mut available_nodes: Vec<(NodeId, DataType)> = Vec::new();

    // 1. Añadir entradas requeridas
    for name in input_names {
        let id = graph.add_input(name, DataType::Float);
        available_nodes.push((id, DataType::Float));
    }

    // 2. Añadir constantes iniciales diversas (enteros y flotantes)
    let c1 = graph.add_constant(Value::Float(1.0));
    available_nodes.push((c1, DataType::Float));
    let c2 = graph.add_constant(Value::Float(2.0));
    available_nodes.push((c2, DataType::Float));
    let c3 = graph.add_constant(Value::Float(rng.gen_range(-3.0..3.0)));
    available_nodes.push((c3, DataType::Float));

    // 3. Añadir capas de operadores aleatorios
    let target_nodes = depth.clamp(2, 6);
    for _ in 0..target_nodes {
        let op = pick_random_op_from(allowed_ops, rng);
        let (req_inputs, out_type) = op.signature();

        let mut chosen_inputs = Vec::new();
        let mut possible = true;

        for req_type in req_inputs {
            let matches: Vec<NodeId> = available_nodes
                .iter()
                .filter(|(_, dt)| *dt == req_type)
                .map(|(id, _)| *id)
                .collect();

            if matches.is_empty() {
                possible = false;
                break;
            }
            let chosen = matches[rng.gen_range(0..matches.len())];
            chosen_inputs.push(chosen);
        }

        if possible {
            if let Ok(id) = graph.add_operator(op, chosen_inputs, out_type) {
                available_nodes.push((id, out_type));
            }
        }
    }

    // 4. Conectar la salida a un nodo compatible
    let output_candidates: Vec<NodeId> = available_nodes
        .iter()
        .filter(|(_, dt)| *dt == output_type)
        .map(|(id, _)| *id)
        .collect();

    let final_out = if let Some(&id) = output_candidates.last() {
        id
    } else {
        graph.add_constant(Value::default_for_type(output_type))
    };

    let _ = graph.set_output(final_out);
    graph.prune_dead_nodes();
    graph
}

/// Aplica una mutación atómica uniforme y efectiva al genoma.
pub fn mutate_genome(graph: &mut GenomeGraph, rng: &mut impl Rng) {
    mutate_genome_with_ops(graph, AVAILABLE_OPS, rng);
}

pub fn mutate_genome_with_ops(graph: &mut GenomeGraph, allowed_ops: &[OpCode], rng: &mut impl Rng) {
    let mutation_type = rng.gen_range(0..5);

    match mutation_type {
        0 => {
            // Mutación A: Perturbar o reemplazar una constante numérica con precisión
            let const_indices: Vec<usize> = graph
                .nodes
                .iter()
                .enumerate()
                .filter_map(|(idx, n)| match n.node_type {
                    NodeType::Constant(Value::Float(_)) => Some(idx),
                    _ => None,
                })
                .collect();

            if !const_indices.is_empty() {
                let chosen_idx = const_indices[rng.gen_range(0..const_indices.len())];
                if let NodeType::Constant(Value::Float(f)) = &mut graph.nodes[chosen_idx].node_type {
                    let r = rng.gen_range(0..10);
                    if r < 4 {
                        // Ajuste fino suave
                        *f += rng.gen_range(-0.25..0.25);
                    } else if r < 7 {
                        // Salto medio
                        *f += rng.gen_range(-1.5..1.5);
                    } else {
                        // Ajuste exacto a constantes comunes en física y matemáticas
                        let presets = [-2.0, -1.0, 0.0, 0.5, 1.0, 2.0, 3.0, 4.0, 5.0];
                        *f = presets[rng.gen_range(0..presets.len())];
                    }
                }
            } else {
                // Si no hay constantes, añadir una
                let c = graph.add_constant(Value::Float(1.0));
                // Empalmar a un operador si es posible
                splice_new_node(graph, c, DataType::Float, rng);
            }
        }
        1 => {
            // Mutación B: Sustituir un operador por otro con la misma signatura
            let op_indices: Vec<usize> = graph
                .nodes
                .iter()
                .enumerate()
                .filter_map(|(idx, n)| if n.is_operator() { Some(idx) } else { None })
                .collect();

            if !op_indices.is_empty() {
                let chosen_idx = op_indices[rng.gen_range(0..op_indices.len())];
                if let NodeType::Operator { op, .. } = &mut graph.nodes[chosen_idx].node_type {
                    let (req_in, req_out) = op.signature();
                    let compatible_ops: Vec<OpCode> = allowed_ops
                        .iter()
                        .copied()
                        .filter(|other| other != op && other.signature() == (req_in.clone(), req_out))
                        .collect();

                    if !compatible_ops.is_empty() {
                        let new_op = compatible_ops[rng.gen_range(0..compatible_ops.len())];
                        *op = new_op;
                    }
                }
            }
        }
        2 => {
            // Mutación C: Insertar un nuevo operador y CONECTARLO de inmediato al circuito
            let op = pick_random_op_from(allowed_ops, rng);
            let (req_inputs, out_type) = op.signature();

            let mut chosen = Vec::new();
            let mut can_add = true;

            for req in &req_inputs {
                let matches: Vec<NodeId> = graph
                    .nodes
                    .iter()
                    .filter(|n| n.output_type == *req)
                    .map(|n| n.id)
                    .collect();

                if matches.is_empty() {
                    can_add = false;
                    break;
                }
                chosen.push(matches[rng.gen_range(0..matches.len())]);
            }

            if can_add {
                if let Ok(new_id) = graph.add_operator(op, chosen, out_type) {
                    if out_type == graph.expected_output_type && rng.gen_bool(0.4) {
                        let _ = graph.set_output(new_id);
                    } else {
                        splice_new_node(graph, new_id, out_type, rng);
                    }
                }
            }
        }
        3 => {
            // Mutación D: Reconectar un puerto de entrada a otro nodo previo compatible
            let sorted = match graph.topological_sort() {
                Ok(s) => s,
                Err(_) => return,
            };

            let op_nodes: Vec<(usize, NodeId)> = sorted
                .iter()
                .enumerate()
                .filter(|(idx, &node_id)| *idx > 0 && graph.node(node_id).map_or(false, |n| n.is_operator()))
                .map(|(idx, &node_id)| (idx, node_id))
                .collect();

            if !op_nodes.is_empty() {
                let (topological_idx, node_id) = op_nodes[rng.gen_range(0..op_nodes.len())];
                let node = graph.node(node_id).unwrap().clone();

                if let NodeType::Operator { op, inputs, output_type } = node.node_type {
                    let (req_types, _) = op.signature();
                    let input_idx_to_change = rng.gen_range(0..inputs.len());
                    let needed_type = req_types[input_idx_to_change];

                    let candidates: Vec<NodeId> = sorted[..topological_idx]
                        .iter()
                        .copied()
                        .filter(|&cand_id| {
                            graph.node(cand_id).map_or(false, |n| n.output_type == needed_type)
                        })
                        .collect();

                    if !candidates.is_empty() {
                        let mut new_inputs = inputs;
                        new_inputs[input_idx_to_change] = candidates[rng.gen_range(0..candidates.len())];
                        if let Some(target_mut) = graph.node_mut(node_id) {
                            target_mut.node_type = NodeType::Operator {
                                op,
                                inputs: new_inputs,
                                output_type,
                            };
                        }
                    }
                }
            }
        }
        _ => {
            // Mutación E: Cambiar la salida hacia otro nodo compatible existente
            let compatible_outputs: Vec<NodeId> = graph
                .nodes
                .iter()
                .filter(|n| n.output_type == graph.expected_output_type && graph.output_node != Some(n.id))
                .map(|n| n.id)
                .collect();

            if !compatible_outputs.is_empty() && rng.gen_bool(0.35) {
                let chosen = compatible_outputs[rng.gen_range(0..compatible_outputs.len())];
                let _ = graph.set_output(chosen);
            }
        }
    }

    // Poda activa de código muerto tras cada mutación para mantener el genoma ágil y eficiente
    graph.prune_dead_nodes();
}

/// Conecta un nodo recién creado a algún operador que acepte su tipo de salida
fn splice_new_node(graph: &mut GenomeGraph, new_id: NodeId, out_type: DataType, rng: &mut impl Rng) {
    let target_indices: Vec<usize> = graph
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| {
            if let NodeType::Operator { op, .. } = &n.node_type {
                let (req_in, _) = op.signature();
                req_in.contains(&out_type)
            } else {
                false
            }
        })
        .map(|(idx, _)| idx)
        .collect();

    if !target_indices.is_empty() {
        let t_idx = target_indices[rng.gen_range(0..target_indices.len())];
        if let NodeType::Operator { op, inputs, .. } = &mut graph.nodes[t_idx].node_type {
            let (req_in, _) = op.signature();
            for (slot, &req) in req_in.iter().enumerate() {
                if req == out_type {
                    inputs[slot] = new_id;
                    break;
                }
            }
        }
    }
}

/// Recombinación sexual (Crossover) que transfiere subárboles funcionales de B a A
pub fn crossover_genomes(
    parent_a: &GenomeGraph,
    parent_b: &GenomeGraph,
    rng: &mut impl Rng,
) -> GenomeGraph {
    let mut child = parent_a.clone();

    // Seleccionar una rama alcanzable del progenitor B
    let b_reachable: Vec<NodeId> = parent_b.reachable_nodes().into_iter().collect();
    if !b_reachable.is_empty() {
        let donor_id = b_reachable[rng.gen_range(0..b_reachable.len())];
        if let Some(donor_node) = parent_b.node(donor_id) {
            let donor_type = donor_node.output_type;

            // Transferir nodo de B a child
            let new_child_node = match &donor_node.node_type {
                NodeType::Constant(val) => child.add_constant(val.clone()),
                NodeType::Operator { op, .. } => {
                    let (req_inputs, _) = op.signature();
                    let mut chosen = Vec::new();
                    let mut ok = true;
                    for req in req_inputs {
                        let matches: Vec<NodeId> = child
                            .nodes
                            .iter()
                            .filter(|n| n.output_type == req)
                            .map(|n| n.id)
                            .collect();
                        if matches.is_empty() {
                            ok = false;
                            break;
                        }
                        chosen.push(matches[rng.gen_range(0..matches.len())]);
                    }
                    if ok {
                        child.add_operator(*op, chosen, donor_type).unwrap_or(child.output_node.unwrap_or(0))
                    } else {
                        child.output_node.unwrap_or(0)
                    }
                }
                NodeType::Input { .. } => {
                    // Si es entrada, buscar la entrada equivalente en child
                    child.nodes.iter().find(|n| n.is_input()).map_or(0, |n| n.id)
                }
            };

            // Empalmar el nuevo nodo en child
            splice_new_node(&mut child, new_child_node, donor_type, rng);
        }
    }

    child.prune_dead_nodes();
    child
}
