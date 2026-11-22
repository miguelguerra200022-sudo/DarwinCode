use crate::genome::graph::GenomeGraph;
use crate::genome::node::NodeType;
use crate::types::OpCode;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Elite {
    pub genome: GenomeGraph,
    pub fitness: f64,
    pub descriptor: (usize, usize),
}

/// Archivo bidimensional de élites fenotípicas (MAP-Elites).
/// Almacena el mejor espécimen encontrado para cada celda de comportamiento o complejidad.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapElitesArchive {
    pub dim1_size: usize,
    pub dim2_size: usize,
    pub grid: Vec<Vec<Option<Elite>>>,
}

impl MapElitesArchive {
    pub fn new(dim1_size: usize, dim2_size: usize) -> Self {
        let grid = vec![vec![None; dim2_size]; dim1_size];
        Self {
            dim1_size,
            dim2_size,
            grid,
        }
    }

    /// Añade o reemplaza un individuo si su aptitud (fitness) supera la del ocupante actual de la celda.
    /// Clampa las coordenadas para evitar desbordamiento de límites.
    pub fn add_or_replace(
        &mut self,
        genome: GenomeGraph,
        fitness: f64,
        d1: usize,
        d2: usize,
    ) -> bool {
        let c1 = d1.min(self.dim1_size.saturating_sub(1));
        let c2 = d2.min(self.dim2_size.saturating_sub(1));

        match &self.grid[c1][c2] {
            Some(existing) => {
                if fitness > existing.fitness {
                    self.grid[c1][c2] = Some(Elite {
                        genome,
                        fitness,
                        descriptor: (c1, c2),
                    });
                    true
                } else {
                    false
                }
            }
            None => {
                self.grid[c1][c2] = Some(Elite {
                    genome,
                    fitness,
                    descriptor: (c1, c2),
                });
                true
            }
        }
    }

    /// Muestrea uniformemente un genoma élite de entre todas las celdas ocupadas del archivo.
    pub fn sample_elite(&self, rng: &mut impl Rng) -> Option<&GenomeGraph> {
        let occupied: Vec<&Elite> = self
            .grid
            .iter()
            .flat_map(|row| row.iter().filter_map(|cell| cell.as_ref()))
            .collect();

        if occupied.is_empty() {
            None
        } else {
            let idx = rng.gen_range(0..occupied.len());
            Some(&occupied[idx].genome)
        }
    }

    /// Devuelve (celdas_ocupadas, celdas_totales).
    pub fn coverage(&self) -> (usize, usize) {
        let occupied_count = self
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.is_some())
            .count();
        (occupied_count, self.dim1_size * self.dim2_size)
    }

    /// Devuelve una referencia al mejor individuo global en todo el archivo.
    pub fn best(&self) -> Option<(&GenomeGraph, f64)> {
        self.grid
            .iter()
            .flat_map(|row| row.iter().filter_map(|cell| cell.as_ref()))
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap_or(std::cmp::Ordering::Equal))
            .map(|elite| (&elite.genome, elite.fitness))
    }

    /// Calcula descriptores fenotípicos de nicho (complejidad sintáctica en eje 1, familia algebraica en eje 2).
    /// Separa lineles, polinomios, funciones racionales y trascendentales en nichos no competidores.
    pub fn compute_default_descriptor(genome: &GenomeGraph) -> (usize, usize) {
        let node_count = genome.node_count();
        let d1 = match node_count {
            0..=3 => 0,
            4..=6 => 1,
            7..=10 => 2,
            11..=15 => 3,
            _ => 4,
        };

        let has_divide = genome.nodes.iter().any(|n| match &n.node_type {
            NodeType::Operator { op, .. } => *op == OpCode::Divide,
            _ => false,
        });

        let has_multiply = genome.nodes.iter().any(|n| match &n.node_type {
            NodeType::Operator { op, .. } => *op == OpCode::Multiply,
            _ => false,
        });

        let has_advanced = genome.nodes.iter().any(|n| match &n.node_type {
            NodeType::Operator { op, .. } => matches!(
                op,
                OpCode::Sin | OpCode::Cos | OpCode::SquareRoot | OpCode::IfElse | OpCode::Modulo
            ),
            _ => false,
        });

        let d2 = if has_advanced {
            4
        } else if has_divide {
            3 // Nicho de funciones racionales (cocientes hiperbólicos)
        } else if has_multiply {
            2 // Nicho de polinomios y acoplamientos no lineales
        } else {
            1 // Nicho aditivo / lineal
        };

        (d1, d2)
    }
}
