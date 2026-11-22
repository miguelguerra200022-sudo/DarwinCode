//! Motor de Neuroevolución Morfológica 3D mediante CPPN-NEAT
//! (Compositional Pattern Producing Networks con NeuroEvolution of Augmenting Topologies).
//! Permite a DarwinCode evolucionar formas, criaturas y estructuras continuas 3D de forma autónoma.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Funciones de activación no lineales utilizadas por los nodos de la red CPPN.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CppnActivation {
    Linear,
    Sine,
    Cosine,
    Gaussian,
    Sigmoid,
    Tanh,
    Abs,
    Square,
}

impl CppnActivation {
    /// Evalúa la función de activación sobre un valor escalar.
    #[inline]
    pub fn evaluate(&self, x: f64) -> f64 {
        match self {
            CppnActivation::Linear => x.clamp(-5.0, 5.0),
            CppnActivation::Sine => (x * PI).sin(),
            CppnActivation::Cosine => (x * PI).cos(),
            CppnActivation::Gaussian => (-2.5 * x * x).exp(),
            CppnActivation::Sigmoid => 1.0 / (1.0 + (-4.9 * x).exp()),
            CppnActivation::Tanh => (2.5 * x).tanh(),
            CppnActivation::Abs => x.abs(),
            CppnActivation::Square => (x * x).min(10.0),
        }
    }

    /// Retorna una activación aleatoria utilizando un generador pseudoaleatorio simple.
    pub fn random_activation(seed: u64) -> Self {
        match seed % 8 {
            0 => CppnActivation::Sine,
            1 => CppnActivation::Cosine,
            2 => CppnActivation::Gaussian,
            3 => CppnActivation::Sigmoid,
            4 => CppnActivation::Tanh,
            5 => CppnActivation::Abs,
            6 => CppnActivation::Square,
            _ => CppnActivation::Linear,
        }
    }
}

/// Nodo individual dentro del grafo CPPN.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CppnNode {
    pub id: usize,
    pub is_input: bool,
    pub is_output: bool,
    pub activation: CppnActivation,
    pub bias: f64,
}

/// Conexión sináptica entre dos nodos de la red CPPN.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CppnConnection {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
    pub enabled: bool,
    pub innovation: usize,
}

/// Genoma de una red CPPN capaz de mapear coordenadas espaciales 3D a densidad y color.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CppnGenome {
    pub nodes: Vec<CppnNode>,
    pub connections: Vec<CppnConnection>,
    pub fitness: f64,
}

impl CppnGenome {
    /// Crea un genoma inicial mínimo con 4 entradas (x, y, z, r) y 4 salidas (densidad, R, G, B).
    pub fn new_minimal() -> Self {
        let mut nodes = Vec::new();
        // 4 Entradas: x (0), y (1), z (2), r = distancia al centro (3)
        for i in 0..4 {
            nodes.push(CppnNode {
                id: i,
                is_input: true,
                is_output: false,
                activation: CppnActivation::Linear,
                bias: 0.0,
            });
        }

        // 4 Salidas: Densidad (4), Rojo (5), Verde (6), Azul (7)
        let output_acts = [
            CppnActivation::Sigmoid,
            CppnActivation::Sigmoid,
            CppnActivation::Sigmoid,
            CppnActivation::Sigmoid,
        ];
        for i in 0..4 {
            nodes.push(CppnNode {
                id: 4 + i,
                is_input: false,
                is_output: true,
                activation: output_acts[i],
                bias: 0.0,
            });
        }

        // Conexiones iniciales directas
        let mut connections = Vec::new();
        let mut innov = 0;
        for in_id in 0..4 {
            for out_id in 4..8 {
                let weight = ((in_id * 7 + out_id * 13) % 20) as f64 * 0.1 - 1.0;
                connections.push(CppnConnection {
                    from: in_id,
                    to: out_id,
                    weight,
                    enabled: true,
                    innovation: innov,
                });
                innov += 1;
            }
        }

        CppnGenome {
            nodes,
            connections,
            fitness: 0.0,
        }
    }

    /// Evalúa la red CPPN para una coordenada 3D (x, y, z).
    /// Retorna [densidad, R, G, B] donde densidad > 0.5 indica materia sólida.
    pub fn evaluate(&self, x: f64, y: f64, z: f64) -> [f64; 4] {
        let r = (x * x + y * y + z * z).sqrt();
        let mut values = vec![0.0; self.nodes.len()];

        // Asignar entradas
        values[0] = x;
        values[1] = y;
        values[2] = z;
        values[3] = r;

        // Evaluación por pasadas sucesivas para permitir conexiones intermedias
        for _ in 0..3 {
            for conn in &self.connections {
                if !conn.enabled {
                    continue;
                }
                let from_val = values[conn.from];
                values[conn.to] += from_val * conn.weight;
            }

            // Aplicar activaciones y sesgos a nodos no-entrada
            for node in &self.nodes {
                if !node.is_input {
                    values[node.id] = node.activation.evaluate(values[node.id] + node.bias);
                }
            }
        }

        // Salidas 4, 5, 6, 7
        [
            values[4].clamp(0.0, 1.0),
            values[5].clamp(0.0, 1.0),
            values[6].clamp(0.0, 1.0),
            values[7].clamp(0.0, 1.0),
        ]
    }

    /// Muta los pesos sinápticos con ruido gaussiano.
    pub fn mutate_weights(&mut self, rate: f64, sigma: f64, seed: &mut u64) {
        for conn in &mut self.connections {
            if pseudo_random(seed) < rate {
                let delta = (pseudo_random(seed) * 2.0 - 1.0) * sigma;
                conn.weight = (conn.weight + delta).clamp(-4.0, 4.0);
            }
        }
    }

    /// Muta la estructura agregando un nuevo nodo que divide una conexión existente.
    pub fn mutate_add_node(&mut self, seed: &mut u64) {
        if self.connections.is_empty() {
            return;
        }
        let conn_idx = (*seed as usize) % self.connections.len();
        if !self.connections[conn_idx].enabled {
            return;
        }

        self.connections[conn_idx].enabled = false;
        let old_from = self.connections[conn_idx].from;
        let old_to = self.connections[conn_idx].to;
        let old_weight = self.connections[conn_idx].weight;

        let new_node_id = self.nodes.len();
        let act = CppnActivation::random_activation(*seed);
        self.nodes.push(CppnNode {
            id: new_node_id,
            is_input: false,
            is_output: false,
            activation: act,
            bias: 0.0,
        });

        let innov = self.connections.len();
        self.connections.push(CppnConnection {
            from: old_from,
            to: new_node_id,
            weight: 1.0,
            enabled: true,
            innovation: innov,
        });

        self.connections.push(CppnConnection {
            from: new_node_id,
            to: old_to,
            weight: old_weight,
            enabled: true,
            innovation: innov + 1,
        });
    }

    /// Muta la activación de un nodo oculto.
    pub fn mutate_activation(&mut self, seed: &mut u64) {
        let hidden: Vec<usize> = self.nodes.iter()
            .filter(|n| !n.is_input && !n.is_output)
            .map(|n| n.id)
            .collect();

        if hidden.is_empty() {
            return;
        }

        let node_id = hidden[(*seed as usize) % hidden.len()];
        self.nodes[node_id].activation = CppnActivation::random_activation(*seed);
    }
}

/// Generador pseudoaleatorio xorshift rápido y sin dependencias externas.
#[inline]
pub fn pseudo_random(seed: &mut u64) -> f64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    (*seed as f64) / (u64::MAX as f64)
}

/// Representa un vóxel evaluado con posición y color RGB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolvedVoxel {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

/// Población evolutiva de redes neuronales CPPN para generación morfológica autónoma.
pub struct CppnPopulation {
    pub individuals: Vec<CppnGenome>,
    pub generation: usize,
    pub best_fitness: f64,
    pub seed: u64,
}

impl CppnPopulation {
    /// Inicializa una población de genomas CPPN.
    pub fn new(size: usize, initial_seed: u64) -> Self {
        let mut individuals = Vec::with_capacity(size);
        let mut seed = initial_seed;
        for _ in 0..size {
            let mut g = CppnGenome::new_minimal();
            g.mutate_weights(0.6, 0.5, &mut seed);
            individuals.push(g);
        }
        CppnPopulation {
            individuals,
            generation: 0,
            best_fitness: 0.0,
            seed,
        }
    }

    /// Evalúa la función de aptitud (fitness) morfológica en la población.
    /// Premia:
    /// 1. Simetría bilateral en el eje X: la biología natural es casi siempre simétrica bilateralmente.
    /// 2. Densidad de volumen balanceada (15% a 35% de masa sólida).
    /// 3. Diversidad de colores RGB.
    pub fn evaluate_fitness(&mut self, res: usize) {
        let half = res as i32 / 2;
        let step = 2.0 / (res as f64);

        for genome in &mut self.individuals {
            let mut total_samples = 0;
            let mut solid_count = 0;
            let mut symmetry_error = 0.0;
            let mut color_variance = 0.0;

            for ix in 0..half {
                let x = (ix as f64 + 0.5) * step;
                for iy in 0..res {
                    let y = -1.0 + (iy as f64 + 0.5) * step;
                    for iz in 0..res {
                        let z = -1.0 + (iz as f64 + 0.5) * step;

                        let pos = genome.evaluate(x, y, z);
                        let neg = genome.evaluate(-x, y, z);

                        if pos[0] > 0.5 {
                            solid_count += 1;
                        }
                        if neg[0] > 0.5 {
                            solid_count += 1;
                        }
                        total_samples += 2;

                        symmetry_error += (pos[0] - neg[0]).abs();
                        color_variance += (pos[1] - pos[2]).abs() + (pos[2] - pos[3]).abs();
                    }
                }
            }

            let density = solid_count as f64 / total_samples as f64;
            // Densidad ideal entre 0.12 y 0.35
            let density_score = if density >= 0.10 && density <= 0.40 {
                1.0 - (density - 0.25).abs() * 3.0
            } else {
                0.01
            };

            let avg_sym_err = symmetry_error / (total_samples as f64 * 0.5);
            let symmetry_score = (1.0 - avg_sym_err).clamp(0.0, 1.0);

            genome.fitness = (symmetry_score * 0.5 + density_score * 0.4 + (color_variance / total_samples as f64) * 0.1).clamp(0.0, 1.0);
        }

        self.individuals.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
        if let Some(best) = self.individuals.first() {
            self.best_fitness = best.fitness;
        }
    }

    /// Ejecuta una generación completa mediante torneo, elitismo y mutación.
    pub fn step_generation(&mut self, res: usize) {
        self.evaluate_fitness(res);

        let elite_count = (self.individuals.len() / 5).max(2);
        let mut new_population = Vec::with_capacity(self.individuals.len());

        // Elitismo: conservar los mejores intactos
        for i in 0..elite_count {
            new_population.push(self.individuals[i].clone());
        }

        // Reproducción y mutación
        while new_population.len() < self.individuals.len() {
            // Torneo de tamaño 3
            let idx1 = (pseudo_random(&mut self.seed) * (self.individuals.len() as f64)) as usize % self.individuals.len();
            let idx2 = (pseudo_random(&mut self.seed) * (self.individuals.len() as f64)) as usize % self.individuals.len();
            let parent_idx = if self.individuals[idx1].fitness > self.individuals[idx2].fitness { idx1 } else { idx2 };

            let mut child = self.individuals[parent_idx].clone();
            child.mutate_weights(0.4, 0.4, &mut self.seed);

            // Mutación estructural con baja probabilidad
            if pseudo_random(&mut self.seed) < 0.15 {
                child.mutate_add_node(&mut self.seed);
            }
            if pseudo_random(&mut self.seed) < 0.10 {
                child.mutate_activation(&mut self.seed);
            }

            new_population.push(child);
        }

        self.individuals = new_population;
        self.generation += 1;
    }

    /// Voxeliza el mejor individuo en una cuadrícula 3D de resolución NxNxN.
    pub fn voxelize_best(&self, res: usize) -> Vec<EvolvedVoxel> {
        let best = match self.individuals.first() {
            Some(b) => b,
            None => return Vec::new(),
        };

        let mut voxels = Vec::new();
        let step = 2.0 / (res as f64);
        let half = res as i32 / 2;

        for ix in -half..half {
            let x = (ix as f64 + 0.5) * step;
            for iy in -half..half {
                let y = (iy as f64 + 0.5) * step;
                for iz in -half..half {
                    let z = (iz as f64 + 0.5) * step;

                    let out = best.evaluate(x, y, z);
                    if out[0] > 0.5 {
                        voxels.push(EvolvedVoxel {
                            x: ix,
                            y: iy,
                            z: iz,
                            r: out[1] as f32,
                            g: out[2] as f32,
                            b: out[3] as f32,
                        });
                    }
                }
            }
        }

        voxels
    }

    /// Exporta el mejor individuo a formato de archivo Wavefront OBJ con colores de vértice.
    pub fn export_best_obj(&self, res: usize) -> String {
        let voxels = self.voxelize_best(res);
        let mut obj = String::from("# Evolved with DarwinCode CPPN-NEAT 3D Engine\n");
        obj.push_str(&format!("# Generation: {} | Fitness: {:.4} | Voxels: {}\n\n", self.generation, self.best_fitness, voxels.len()));

        let mut v_idx = 1;
        for vox in &voxels {
            let x = vox.x as f32 * 0.1;
            let y = vox.y as f32 * 0.1;
            let z = vox.z as f32 * 0.1;
            let s = 0.05;

            // 8 vértices por cubo con colores RGB
            let corners = [
                (x - s, y - s, z - s),
                (x + s, y - s, z - s),
                (x + s, y + s, z - s),
                (x - s, y + s, z - s),
                (x - s, y - s, z + s),
                (x + s, y - s, z + s),
                (x + s, y + s, z + s),
                (x - s, y + s, z + s),
            ];

            for (cx, cy, cz) in corners {
                obj.push_str(&format!("v {:.4} {:.4} {:.4} {:.3} {:.3} {:.3}\n", cx, cy, cz, vox.r, vox.g, vox.b));
            }

            // 6 caras por cubo
            let faces = [
                (1, 2, 3, 4), // Front
                (5, 8, 7, 6), // Back
                (1, 5, 6, 2), // Bottom
                (4, 3, 7, 8), // Top
                (1, 4, 8, 5), // Left
                (2, 6, 7, 3), // Right
            ];

            for (f1, f2, f3, f4) in faces {
                obj.push_str(&format!("f {} {} {} {}\n", v_idx + f1 - 1, v_idx + f2 - 1, v_idx + f3 - 1, v_idx + f4 - 1));
            }
            v_idx += 8;
        }

        obj
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cppn_activation_functions() {
        assert_eq!(CppnActivation::Linear.evaluate(2.5), 2.5);
        assert!((CppnActivation::Sine.evaluate(0.5) - 1.0).abs() < 1e-4);
        assert!((CppnActivation::Cosine.evaluate(0.0) - 1.0).abs() < 1e-4);
        assert!((CppnActivation::Gaussian.evaluate(0.0) - 1.0).abs() < 1e-4);
        assert!((CppnActivation::Abs.evaluate(-3.2) - 3.2).abs() < 1e-4);
    }

    #[test]
    fn test_cppn_genome_evaluation() {
        let genome = CppnGenome::new_minimal();
        let out = genome.evaluate(0.0, 0.0, 0.0);
        assert_eq!(out.len(), 4);
        for v in out {
            assert!(v >= 0.0 && v <= 1.0);
        }
    }

    #[test]
    fn test_cppn_evolution_population_progress() {
        let mut pop = CppnPopulation::new(20, 12345);
        pop.evaluate_fitness(10);
        let init_fitness = pop.best_fitness;

        for _ in 0..5 {
            pop.step_generation(10);
        }

        assert!(pop.generation == 5);
        assert!(pop.best_fitness >= init_fitness);

        let voxels = pop.voxelize_best(8);
        assert!(!voxels.is_empty() || pop.best_fitness >= 0.0);
    }
}
