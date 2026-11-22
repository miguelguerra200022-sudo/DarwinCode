use crate::genome::graph::GenomeGraph;
use crate::genome::node::NodeType;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum VerificationError {
    #[error("Grafo vacío sin nodos")]
    EmptyGraph,
    #[error("Nodo de salida no asignado")]
    MissingOutput,
    #[error("El nodo de salida {0} produce tipo {1:?}, esperado {2:?}")]
    OutputTypeMismatch(usize, crate::types::DataType, crate::types::DataType),
    #[error("Nodo {0} referencia a nodo inexistente {1}")]
    DanglingReference(usize, usize),
    #[error("Incompatibilidad de tipos: nodo {0} espera {1:?} pero recibe {2:?} de nodo {3}")]
    PortTypeMismatch(usize, crate::types::DataType, crate::types::DataType, usize),
    #[error("Ciclo detectado en el grafo genómico: {0}")]
    CycleDetected(String),
}

/// Verifica formalmente las invariantes estructurales y de tipos de un genoma.
pub fn verify_invariants(graph: &GenomeGraph) -> Result<(), VerificationError> {
    if graph.nodes.is_empty() {
        return Err(VerificationError::EmptyGraph);
    }

    let output_id = graph.output_node.ok_or(VerificationError::MissingOutput)?;
    let output_node = graph
        .node(output_id)
        .ok_or(VerificationError::DanglingReference(output_id, output_id))?;

    if output_node.output_type != graph.expected_output_type {
        return Err(VerificationError::OutputTypeMismatch(
            output_id,
            output_node.output_type,
            graph.expected_output_type,
        ));
    }

    // Verificar compatibilidad de puertos e integridad referencial
    for node in &graph.nodes {
        if let NodeType::Operator { op, inputs, output_type } = &node.node_type {
            let (expected_inputs, expected_out) = op.signature();
            if *output_type != expected_out {
                return Err(VerificationError::PortTypeMismatch(
                    node.id,
                    expected_out,
                    *output_type,
                    node.id,
                ));
            }

            if inputs.len() != expected_inputs.len() {
                return Err(VerificationError::DanglingReference(node.id, 0));
            }

            for (idx, &inp_id) in inputs.iter().enumerate() {
                let inp_node = graph
                    .node(inp_id)
                    .ok_or(VerificationError::DanglingReference(node.id, inp_id))?;
                if inp_node.output_type != expected_inputs[idx] {
                    return Err(VerificationError::PortTypeMismatch(
                        node.id,
                        expected_inputs[idx],
                        inp_node.output_type,
                        inp_id,
                    ));
                }
            }
        }
    }

    // Verificar aciclicidad formal
    graph
        .topological_sort()
        .map_err(VerificationError::CycleDetected)?;

    Ok(())
}
