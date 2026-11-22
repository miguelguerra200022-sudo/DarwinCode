use crate::types::{DataType, OpCode, Value};
use serde::{Deserialize, Serialize};

pub type NodeId = usize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    Input {
        index: usize,
        name: String,
        data_type: DataType,
    },
    Constant(Value),
    Operator {
        op: OpCode,
        inputs: Vec<NodeId>,
        output_type: DataType,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenomeNode {
    pub id: NodeId,
    pub node_type: NodeType,
    pub output_type: DataType,
}

impl GenomeNode {
    pub fn new_input(id: NodeId, index: usize, name: impl Into<String>, data_type: DataType) -> Self {
        Self {
            id,
            node_type: NodeType::Input {
                index,
                name: name.into(),
                data_type,
            },
            output_type: data_type,
        }
    }

    pub fn new_constant(id: NodeId, value: Value) -> Self {
        let output_type = value.get_type();
        Self {
            id,
            node_type: NodeType::Constant(value),
            output_type,
        }
    }

    pub fn new_operator(id: NodeId, op: OpCode, inputs: Vec<NodeId>, output_type: DataType) -> Self {
        Self {
            id,
            node_type: NodeType::Operator {
                op,
                inputs,
                output_type,
            },
            output_type,
        }
    }

    pub fn is_input(&self) -> bool {
        matches!(self.node_type, NodeType::Input { .. })
    }

    pub fn is_constant(&self) -> bool {
        matches!(self.node_type, NodeType::Constant(_))
    }

    pub fn is_operator(&self) -> bool {
        matches!(self.node_type, NodeType::Operator { .. })
    }
}
