use crate::genome::node::{GenomeNode, NodeId, NodeType};
use crate::types::{DataType, OpCode, Value};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenomeGraph {
    pub nodes: Vec<GenomeNode>,
    pub output_node: Option<NodeId>,
    pub expected_output_type: DataType,
}

impl GenomeGraph {
    pub fn new(expected_output_type: DataType) -> Self {
        Self {
            nodes: Vec::new(),
            output_node: None,
            expected_output_type,
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn node(&self, id: NodeId) -> Option<&GenomeNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut GenomeNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    fn next_id(&self) -> NodeId {
        self.nodes.iter().map(|n| n.id).max().map_or(0, |max| max + 1)
    }

    pub fn add_input(&mut self, name: &str, data_type: DataType) -> NodeId {
        let id = self.next_id();
        let input_idx = self.nodes.iter().filter(|n| n.is_input()).count();
        self.nodes.push(GenomeNode::new_input(id, input_idx, name, data_type));
        id
    }

    pub fn add_constant(&mut self, value: Value) -> NodeId {
        let id = self.next_id();
        self.nodes.push(GenomeNode::new_constant(id, value));
        id
    }

    pub fn add_operator(
        &mut self,
        op: OpCode,
        inputs: Vec<NodeId>,
        output_type: DataType,
    ) -> Result<NodeId, String> {
        let (expected_inputs, expected_out) = op.signature();
        if inputs.len() != expected_inputs.len() {
            return Err(format!(
                "OpCode {:?} espera {} argumentos, se proporcionaron {}",
                op,
                expected_inputs.len(),
                inputs.len()
            ));
        }

        for (idx, &inp_id) in inputs.iter().enumerate() {
            let inp_node = self
                .node(inp_id)
                .ok_or_else(|| format!("Nodo de entrada {} no encontrado", inp_id))?;
            if inp_node.output_type != expected_inputs[idx] {
                return Err(format!(
                    "Incompatibilidad de tipos en argumento {}: esperado {:?}, recibido {:?}",
                    idx, expected_inputs[idx], inp_node.output_type
                ));
            }
        }

        if output_type != expected_out {
            return Err(format!(
                "Incompatibilidad de tipo de salida para {:?}: esperado {:?}, recibido {:?}",
                op, expected_out, output_type
            ));
        }

        let id = self.next_id();
        self.nodes
            .push(GenomeNode::new_operator(id, op, inputs, output_type));
        Ok(id)
    }

    pub fn set_output(&mut self, id: NodeId) -> Result<(), String> {
        let node = self
            .node(id)
            .ok_or_else(|| format!("Nodo de salida {} no existe", id))?;
        if node.output_type != self.expected_output_type {
            return Err(format!(
                "El nodo {} produce tipo {:?}, pero el grafo requiere salida {:?}",
                id, node.output_type, self.expected_output_type
            ));
        }
        self.output_node = Some(id);
        Ok(())
    }

    /// Valida que el grafo sea un DAG acíclico, tipado y con salida alcanzable.
    pub fn is_valid(&self) -> bool {
        if self.output_node.is_none() {
            return false;
        }
        self.topological_sort().is_ok()
    }

    /// Ordenación topológica para ejecución determinista y detección de ciclos.
    pub fn topological_sort(&self) -> Result<Vec<NodeId>, String> {
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        let mut adj_list: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

        for node in &self.nodes {
            in_degree.entry(node.id).or_insert(0);
            adj_list.entry(node.id).or_default();
        }

        for node in &self.nodes {
            if let NodeType::Operator { inputs, .. } = &node.node_type {
                for &inp in inputs {
                    if !self.nodes.iter().any(|n| n.id == inp) {
                        return Err(format!("Referencia a nodo inexistente {}", inp));
                    }
                    adj_list.entry(inp).or_default().push(node.id);
                    *in_degree.entry(node.id).or_insert(0) += 1;
                }
            }
        }

        let mut zero_deg: Vec<NodeId> = in_degree
            .iter()
            .filter_map(|(&id, &deg)| if deg == 0 { Some(id) } else { None })
            .collect();
        zero_deg.sort_unstable();

        let mut queue: VecDeque<NodeId> = VecDeque::from(zero_deg);

        let mut sorted = Vec::new();
        while let Some(u) = queue.pop_front() {
            sorted.push(u);
            if let Some(neighbors) = adj_list.get_mut(&u) {
                neighbors.sort_unstable();
                for &v in neighbors.iter() {
                    let deg = in_degree.get_mut(&v).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(v);
                    }
                }
            }
        }

        if sorted.len() != self.nodes.len() {
            return Err("Se detectó un ciclo en el grafo genómico (no es DAG)".to_string());
        }

        Ok(sorted)
    }

    /// Devuelve los IDs de los nodos que realmente contribuyen a la salida final (podando código muerto).
    pub fn reachable_nodes(&self) -> HashSet<NodeId> {
        let mut reachable = HashSet::new();
        if let Some(out_id) = self.output_node {
            let mut stack = vec![out_id];
            while let Some(curr) = stack.pop() {
                if reachable.insert(curr) {
                    if let Some(node) = self.node(curr) {
                        if let NodeType::Operator { inputs, .. } = &node.node_type {
                            for &inp in inputs {
                                stack.push(inp);
                            }
                        }
                    }
                }
            }
        }
        reachable
    }

    /// Elimina todos los nodos que no contribuyen a la salida final (código muerto o intrones).
    /// Mantiene intactas las entradas obligatorias para preservar la interfaz.
    pub fn prune_dead_nodes(&mut self) {
        let reachable = self.reachable_nodes();
        self.nodes.retain(|node| node.is_input() || reachable.contains(&node.id));
    }

    /// Descompone el genoma en una fórmula matemática legible por humanos
    pub fn to_expression_string(&self) -> String {
        if let Some(out_id) = self.output_node {
            self.node_to_string(out_id)
        } else {
            "Vacío".to_string()
        }
    }

    fn node_to_string(&self, id: NodeId) -> String {
        if let Some(node) = self.node(id) {
            match &node.node_type {
                NodeType::Input { name, .. } => name.clone(),
                NodeType::Constant(val) => match val {
                    Value::Float(f) => format!("{:.2}", f),
                    Value::Int(i) => format!("{}", i),
                    Value::Boolean(b) => format!("{}", b),
                    Value::Vec3(v) => format!("[{:.1},{:.1},{:.1}]", v[0], v[1], v[2]),
                },
                NodeType::Operator { op, inputs, .. } => {
                    let in_strs: Vec<String> = inputs.iter().map(|&inp| self.node_to_string(inp)).collect();
                    match op {
                        OpCode::Add => format!("({} + {})", in_strs.first().unwrap_or(&"?".to_string()), in_strs.get(1).unwrap_or(&"?".to_string())),
                        OpCode::Subtract => format!("({} - {})", in_strs.first().unwrap_or(&"?".to_string()), in_strs.get(1).unwrap_or(&"?".to_string())),
                        OpCode::Multiply => format!("({} * {})", in_strs.first().unwrap_or(&"?".to_string()), in_strs.get(1).unwrap_or(&"?".to_string())),
                        OpCode::Divide => format!("({} / {})", in_strs.first().unwrap_or(&"?".to_string()), in_strs.get(1).unwrap_or(&"?".to_string())),
                        OpCode::Sin => format!("sin({})", in_strs.first().unwrap_or(&"?".to_string())),
                        OpCode::Cos => format!("cos({})", in_strs.first().unwrap_or(&"?".to_string())),
                        OpCode::SquareRoot => format!("sqrt({})", in_strs.first().unwrap_or(&"?".to_string())),
                        OpCode::Absolute => format!("abs({})", in_strs.first().unwrap_or(&"?".to_string())),
                        _ => format!("{:?}({})", op, in_strs.join(", ")),
                    }
                }
            }
        } else {
            "?".to_string()
        }
    }

    /// Calcula la disposición espacial en 3D (pisos y coordenadas) para la animación de construcción bloque a bloque
    pub fn compute_spatial_layout(&self) -> Vec<NodePosition3D> {
        let mut layers: HashMap<NodeId, usize> = HashMap::new();
        let sorted = match self.topological_sort() {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        // Calcular altura de piso (Layer)
        for &id in &sorted {
            let node = match self.node(id) {
                Some(n) => n,
                None => continue,
            };

            let layer = match &node.node_type {
                NodeType::Input { .. } | NodeType::Constant(_) => 0,
                NodeType::Operator { inputs, .. } => {
                    let max_parent_layer = inputs
                        .iter()
                        .filter_map(|inp| layers.get(inp))
                        .max()
                        .copied()
                        .unwrap_or(0);
                    max_parent_layer + 1
                }
            };
            layers.insert(id, layer);
        }

        // Agrupar nodos por piso para distribuir en X y Z
        let mut nodes_per_layer: HashMap<usize, Vec<NodeId>> = HashMap::new();
        for &id in &sorted {
            let l = layers.get(&id).copied().unwrap_or(0);
            nodes_per_layer.entry(l).or_default().push(id);
        }

        let mut layout = Vec::new();
        for (&layer, node_ids) in &nodes_per_layer {
            let count = node_ids.len();
            let spacing = 2.4f32;
            let start_x = -((count as f32 - 1.0) * spacing) / 2.0;

            for (idx, &id) in node_ids.iter().enumerate() {
                let node = self.node(id).unwrap();
                let x = start_x + (idx as f32) * spacing;
                let y = (layer as f32) * 2.2;
                let z = (idx as f32 % 2.0) * 0.8 - 0.4;

                let (label, color_tag, inputs) = match &node.node_type {
                    NodeType::Input { name, .. } => (name.clone(), "input".to_string(), Vec::new()),
                    NodeType::Constant(val) => {
                        let txt = match val {
                            Value::Float(f) => format!("{:.1}", f),
                            _ => "C".to_string(),
                        };
                        (txt, "constant".to_string(), Vec::new())
                    }
                    NodeType::Operator { op, inputs, .. } => {
                        let name = format!("{:?}", op);
                        let is_out = self.output_node == Some(id);
                        let tag = if is_out { "output" } else { "operator" };
                        (name, tag.to_string(), inputs.clone())
                    }
                };

                layout.push(NodePosition3D {
                    id,
                    x,
                    y,
                    z,
                    layer,
                    label,
                    color_tag,
                    inputs,
                });
            }
        }

        layout
    }

    /// Genera la representación en voxeles (bloques físicos de construcción 3D tipo casa/ciudad)
    /// Retorna [x, y, z, tipo_de_bloque]
    pub fn generate_voxel_representation(&self) -> Vec<[i32; 4]> {
        let mut voxels = Vec::new();
        let layout = self.compute_spatial_layout();

        // 1. Cimientos y suelo
        for x in -3..=3 {
            for z in -3..=3 {
                voxels.push([x, 0, z, 0]); // Tipo 0: Cimientos / Terreno
            }
        }

        // 2. Colocar bloques estructurales por cada nodo y pilares conectores
        for node in &layout {
            let bx = (node.x / 1.5).round() as i32;
            let by = ((node.y / 1.8) as i32 + 1).max(1);
            let bz = (node.z / 1.5).round() as i32;

            let block_type = match node.color_tag.as_str() {
                "input" => 1,    // Pilar base
                "constant" => 2, // Muro lateral
                "operator" => 3, // Viga / Piso elevado
                "output" => 4,   // Cúspide / Techo dorado
                _ => 1,
            };

            voxels.push([bx, by, bz, block_type]);

            // Pilares de soporte que bajan hasta el suelo
            for py in 1..by {
                voxels.push([bx, py, bz, 1]); // Columna vertical
            }
        }

        voxels
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePosition3D {
    pub id: NodeId,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub layer: usize,
    pub label: String,
    pub color_tag: String,
    pub inputs: Vec<NodeId>,
}
