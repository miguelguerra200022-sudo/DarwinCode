use crate::genome::graph::GenomeGraph;
use crate::map_elites::MapElitesArchive;
use crate::mutation::{
    crossover_genomes, generate_random_genome, generate_random_genome_with_ops, mutate_genome,
    mutate_genome_with_ops, AVAILABLE_OPS,
};
use crate::optimizer::optimize_graph_constants;
use crate::symmetry::{augment_dataset_with_symmetries, detect_symmetries, SymmetryReport};
use crate::types::{DataType, OpCode, Value};
use crate::verifier::verify_invariants;
use crate::vm::evaluate_graph_with_order;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationConfig {
    pub population_size: usize,
    pub elite_ratio: f64,
    pub mutation_rate: f64,
    pub gas_limit: u64,
}

impl Default for PopulationConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            elite_ratio: 0.05,
            mutation_rate: 0.75,
            gas_limit: 1000,
        }
    }
}

/// Configuración avanzada para el motor neuro-simbólico SOTA DarwinCode v1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SotaEngineConfig {
    pub population_config: PopulationConfig,
    pub enable_constant_optimization: bool,
    pub constant_opt_interval: u64,
    pub constant_opt_top_k: usize,
    pub enable_symmetry_prepass: bool,
    pub enable_map_elites: bool,
    pub allowed_ops: Option<Vec<OpCode>>,
}

impl Default for SotaEngineConfig {
    fn default() -> Self {
        Self {
            population_config: PopulationConfig::default(),
            enable_constant_optimization: true,
            constant_opt_interval: 3,
            constant_opt_top_k: 5,
            enable_symmetry_prepass: true,
            enable_map_elites: true,
            allowed_ops: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStats {
    pub generation: u64,
    pub best_fitness: f64,
    pub avg_fitness: f64,
    pub best_nodes_count: usize,
    pub total_evaluations: u64,
}

#[derive(Clone)]
pub struct Individual {
    pub genome: GenomeGraph,
    pub fitness: f64,
}

pub struct EvolutionEngine {
    pub config: PopulationConfig,
    pub sota_config: Option<SotaEngineConfig>,
    pub population: Vec<Individual>,
    next_population: Vec<Individual>,
    pub archive: MapElitesArchive,
    pub generation: u64,
    pub best_genome: GenomeGraph,
    pub best_fitness: f64,
    pub total_evaluations: u64,
    rng: StdRng,
    pub input_names: Vec<String>,
    pub original_input_names: Vec<String>,
    pub output_type: DataType,
    pub symmetry_report: Option<SymmetryReport>,
    pub augmented_input_names: Option<Vec<String>>,
}

impl EvolutionEngine {
    pub fn new(config: PopulationConfig, output_type: DataType, input_names: &[&str]) -> Self {
        let mut rng = StdRng::seed_from_u64(1337);
        let mut population = Vec::with_capacity(config.population_size);
        let next_population = Vec::with_capacity(config.population_size);

        for _ in 0..config.population_size {
            let genome = generate_random_genome(input_names, output_type, 3, &mut rng);
            population.push(Individual {
                genome,
                fitness: 0.0,
            });
        }

        let seed_best = population[0].genome.clone();
        let archive = MapElitesArchive::new(5, 5);
        let orig_inputs = input_names.iter().map(|s| s.to_string()).collect();

        Self {
            config,
            sota_config: None,
            population,
            next_population,
            archive,
            generation: 0,
            best_genome: seed_best,
            best_fitness: f64::NEG_INFINITY,
            total_evaluations: 0,
            rng,
            input_names: input_names.iter().map(|s| s.to_string()).collect(),
            original_input_names: orig_inputs,
            output_type,
            symmetry_report: None,
            augmented_input_names: None,
        }
    }

    pub fn new_sota(
        sota_config: SotaEngineConfig,
        output_type: DataType,
        input_names: &[&str],
    ) -> Self {
        let mut engine = Self::new(sota_config.population_config.clone(), output_type, input_names);
        if let Some(ops) = &sota_config.allowed_ops {
            engine.population.clear();
            for _ in 0..engine.config.population_size {
                let genome = generate_random_genome_with_ops(input_names, output_type, 3, ops, &mut engine.rng);
                engine.population.push(Individual {
                    genome,
                    fitness: 0.0,
                });
            }
            if let Some(first) = engine.population.first() {
                engine.best_genome = first.genome.clone();
            }
        }
        engine.sota_config = Some(sota_config);
        engine
    }

    pub fn stats(&self) -> EngineStats {
        let sum_fitness: f64 = self.population.iter().map(|ind| ind.fitness).sum();
        let avg_fitness = if self.population.is_empty() {
            0.0
        } else {
            sum_fitness / (self.population.len() as f64)
        };

        EngineStats {
            generation: self.generation,
            best_fitness: self.best_fitness,
            avg_fitness,
            best_nodes_count: self.best_genome.node_count(),
            total_evaluations: self.total_evaluations,
        }
    }

    /// Evalúa la población y engendra la siguiente generación mediante doble buffer O(1)
    pub fn step_with_evaluator<F>(&mut self, mut evaluator: F)
    where
        F: FnMut(&GenomeGraph, u64) -> f64,
    {
        self.generation += 1;

        // 1. Evaluar aptitud (Fitness) de cada individuo en la población actual
        for ind in &mut self.population {
            if verify_invariants(&ind.genome).is_ok() {
                let score = evaluator(&ind.genome, self.config.gas_limit);
                ind.fitness = if score.is_nan() { 0.0 } else { score };
            } else {
                ind.fitness = 0.0;
            }
            self.total_evaluations += 1;
        }

        // 2. Ordenar por aptitud descendente
        self.population.sort_by(|a, b| {
            b.fitness
                .partial_cmp(&a.fitness)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 3. Registrar el mejor individuo global y podar su código muerto
        if let Some(top) = self.population.first() {
            if top.fitness > self.best_fitness {
                self.best_fitness = top.fitness;
                let mut best = top.genome.clone();
                best.prune_dead_nodes();
                self.best_genome = best;
            }
        }

        // 4. Selección por torneo y reproducción hacia next_population
        let elite_count =
            ((self.config.population_size as f64) * self.config.elite_ratio).max(2.0) as usize;
        let immigrant_count =
            ((self.config.population_size as f64) * 0.06).max(1.0) as usize;
        let input_strs: Vec<&str> = self.input_names.iter().map(|s| s.as_str()).collect();

        self.next_population.clear();

        // Conservar élite intacta
        for i in 0..elite_count.min(self.population.len()) {
            let mut elite_genome = self.population[i].genome.clone();
            elite_genome.prune_dead_nodes();
            self.next_population.push(Individual {
                genome: elite_genome,
                fitness: self.population[i].fitness,
            });
        }

        // Inmigrantes aleatorios para inyectar sangre fresca
        for _ in 0..immigrant_count {
            self.next_population.push(Individual {
                genome: generate_random_genome(&input_strs, self.output_type, 3, &mut self.rng),
                fitness: 0.0,
            });
        }

        // Selección por torneo de tamaño 3 sobre el pool de progenitores completamente evaluado
        let pop_len = self.population.len();
        while self.next_population.len() < self.config.population_size {
            let p1 = {
                let a = self.rng.gen_range(0..pop_len);
                let b = self.rng.gen_range(0..pop_len);
                let c = self.rng.gen_range(0..pop_len);
                if self.population[a].fitness >= self.population[b].fitness
                    && self.population[a].fitness >= self.population[c].fitness
                {
                    a
                } else if self.population[b].fitness >= self.population[c].fitness {
                    b
                } else {
                    c
                }
            };

            let mut child = self.population[p1].genome.clone();

            if self.rng.gen_bool(0.40) {
                let p2 = {
                    let a = self.rng.gen_range(0..pop_len);
                    let b = self.rng.gen_range(0..pop_len);
                    if self.population[a].fitness >= self.population[b].fitness {
                        a
                    } else {
                        b
                    }
                };
                child = crossover_genomes(&child, &self.population[p2].genome, &mut self.rng);
            }

            if self.rng.gen_bool(self.config.mutation_rate) {
                mutate_genome(&mut child, &mut self.rng);
            }

            child.prune_dead_nodes();

            if verify_invariants(&child).is_err() {
                child = generate_random_genome(&input_strs, self.output_type, 3, &mut self.rng);
            }

            self.next_population.push(Individual {
                genome: child,
                fitness: 0.0,
            });
        }

        // Intercambiar buffers en O(1)
        std::mem::swap(&mut self.population, &mut self.next_population);
    }

    /// Atajo para evaluar funciones numéricas simples f(inputs) -> output con rendimiento O(1).
    pub fn step_with_dataset(&mut self, dataset: &[(&[Value], Value)]) {
        let gas = self.config.gas_limit;
        self.step_with_evaluator(|genome, _| {
            let sorted_order = match genome.topological_sort() {
                Ok(s) => s,
                Err(_) => return 0.0,
            };
            let mut error_sum = 0.0;
            for (inputs, expected) in dataset {
                match evaluate_graph_with_order(genome, &sorted_order, inputs, gas) {
                    Ok(out) => {
                        let diff = match (out.as_float(), expected.as_float()) {
                            (Some(o), Some(e)) => (o - e).abs(),
                            _ => 100.0,
                        };
                        error_sum += diff;
                    }
                    Err(_) => {
                        error_sum += 1000.0;
                    }
                }
            }
            let accuracy = 1.0 / (1.0 + error_sum);
            // Navaja de Ockham: bonus de simplicidad y elegancia cuando la precisión es alta
            if accuracy > 0.95 {
                let node_count = genome.node_count().max(1);
                let parsimony_bonus = 0.02 / (node_count as f64);
                accuracy + parsimony_bonus
            } else {
                accuracy
            }
        });
    }

    /// Expande un vector de entrada original con las características virtuales de simetría descubiertas
    pub fn expand_input_vector(&self, original_inputs: &[Value]) -> Vec<Value> {
        if let Some(report) = &self.symmetry_report {
            let mut expanded = original_inputs.to_vec();

            for &(i, j) in &report.symmetric_pairs {
                let vi = original_inputs.get(i).and_then(|v| v.as_float()).unwrap_or(0.0);
                let vj = original_inputs.get(j).and_then(|v| v.as_float()).unwrap_or(0.0);
                expanded.push(Value::Float(vi + vj));
                expanded.push(Value::Float(vi * vj));
            }

            for &i in &report.even_variables {
                let vi = original_inputs.get(i).and_then(|v| v.as_float()).unwrap_or(0.0);
                expanded.push(Value::Float(vi * vi));
            }

            expanded
        } else {
            original_inputs.to_vec()
        }
    }

    /// Ejecuta el bucle de descubrimiento SOTA de DarwinCode v1.0:
    /// Detección de simetrías preliminar + MAP-Elites + Calibrador de constantes Levenberg-Marquardt
    pub fn step_with_dataset_sota(&mut self, dataset: &[(&[Value], Value)]) {
        let sota_cfg = self
            .sota_config
            .clone()
            .unwrap_or_else(SotaEngineConfig::default);

        // 1. Fase de Simetría en Generación 0 (AI Feynman)
        if self.generation == 0 && sota_cfg.enable_symmetry_prepass {
            let report = detect_symmetries(dataset, self.original_input_names.len(), 1e-4);
            if !report.symmetric_pairs.is_empty() || !report.even_variables.is_empty() {
                let orig_slices: Vec<&str> =
                    self.original_input_names.iter().map(|s| s.as_str()).collect();
                let (_, augmented_names) =
                    augment_dataset_with_symmetries(dataset, &orig_slices, &report);

                self.symmetry_report = Some(report);
                self.augmented_input_names = Some(augmented_names.clone());

                let name_slices: Vec<&str> = augmented_names.iter().map(|s| s.as_str()).collect();
                self.reset_with_inputs(&name_slices);
            }
        }

        // 2. Construir dataset efectivo (aumentado con invariantes o directo)
        let expanded_inputs: Vec<Vec<Value>> = dataset
            .iter()
            .map(|(inp, _)| self.expand_input_vector(inp))
            .collect();

        let eff_dataset: Vec<(&[Value], Value)> = expanded_inputs
            .iter()
            .zip(dataset.iter())
            .map(|(exp_in, (_, target))| (exp_in.as_slice(), target.clone()))
            .collect();

        self.generation += 1;
        let gas = self.config.gas_limit;

        // 3. Evaluar aptitud básica en la población
        for ind in &mut self.population {
            if verify_invariants(&ind.genome).is_ok() {
                let sorted = match ind.genome.topological_sort() {
                    Ok(s) => s,
                    Err(_) => {
                        ind.fitness = 0.0;
                        continue;
                    }
                };

                let mut error_sum = 0.0;
                for (inputs, target) in &eff_dataset {
                    match evaluate_graph_with_order(&ind.genome, &sorted, inputs, gas) {
                        Ok(out) => {
                            let diff = match (out.as_float(), target.as_float()) {
                                (Some(o), Some(t)) => (o - t).abs(),
                                _ => 100.0,
                            };
                            error_sum += diff;
                        }
                        Err(_) => {
                            error_sum += 1000.0;
                        }
                    }
                }

                let node_count = ind.genome.node_count().max(1) as f64;
                let parsimony_penalty = 0.002 * node_count;
                ind.fitness = 1.0 / (1.0 + error_sum + parsimony_penalty);

                // Almacenar en el archivo MAP-Elites
                if sota_cfg.enable_map_elites {
                    let (d1, d2) = MapElitesArchive::compute_default_descriptor(&ind.genome);
                    self.archive
                        .add_or_replace(ind.genome.clone(), ind.fitness, d1, d2);
                }
            } else {
                ind.fitness = 0.0;
            }
            self.total_evaluations += 1;
        }

        // 4. Calibración continua de constantes (Levenberg-Marquardt)
        if sota_cfg.enable_constant_optimization
            && (self.generation % sota_cfg.constant_opt_interval == 0 || self.generation == 1)
        {
            // Ordenar para identificar los mejores candidatos estructurales
            self.population.sort_by(|a, b| {
                b.fitness
                    .partial_cmp(&a.fitness)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let top_k = sota_cfg.constant_opt_top_k.min(self.population.len());
            for i in 0..top_k {
                let ind = &mut self.population[i];
                let final_mse = optimize_graph_constants(&mut ind.genome, &eff_dataset, 25);
                if final_mse.is_finite() {
                    let error_sum = final_mse.sqrt() * (eff_dataset.len() as f64);
                    let node_count = ind.genome.node_count().max(1) as f64;
                    let parsimony_penalty = 0.002 * node_count;
                    ind.fitness = 1.0 / (1.0 + error_sum + parsimony_penalty);

                    if sota_cfg.enable_map_elites {
                        let (d1, d2) = MapElitesArchive::compute_default_descriptor(&ind.genome);
                        self.archive
                            .add_or_replace(ind.genome.clone(), ind.fitness, d1, d2);
                    }
                }
            }
        }

        // 5. Ordenar por aptitud final
        self.population.sort_by(|a, b| {
            b.fitness
                .partial_cmp(&a.fitness)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some(top) = self.population.first() {
            if top.fitness > self.best_fitness {
                self.best_fitness = top.fitness;
                let mut best = top.genome.clone();
                best.prune_dead_nodes();
                self.best_genome = best;
            }
        }

        // 6. Reproducción y reemplazo asistido por MAP-Elites
        let elite_count =
            ((self.config.population_size as f64) * self.config.elite_ratio).max(2.0) as usize;
        let immigrant_count =
            ((self.config.population_size as f64) * 0.06).max(1.0) as usize;
        let input_strs: Vec<&str> = self.input_names.iter().map(|s| s.as_str()).collect();

        self.next_population.clear();

        // Conservar élites
        for i in 0..elite_count.min(self.population.len()) {
            let mut elite_genome = self.population[i].genome.clone();
            elite_genome.prune_dead_nodes();
            self.next_population.push(Individual {
                genome: elite_genome,
                fitness: self.population[i].fitness,
            });
        }

        let allowed_ops = sota_cfg
            .allowed_ops
            .as_deref()
            .unwrap_or(AVAILABLE_OPS);

        // Inmigrantes
        for _ in 0..immigrant_count {
            self.next_population.push(Individual {
                genome: generate_random_genome_with_ops(&input_strs, self.output_type, 3, allowed_ops, &mut self.rng),
                fitness: 0.0,
            });
        }

        let pop_len = self.population.len();
        while self.next_population.len() < self.config.population_size {
            // Muestrear progenitor 1: 30% desde MAP-Elites (Quality-Diversity) o torneo
            let p1_genome = if sota_cfg.enable_map_elites && self.rng.gen_bool(0.30) {
                if let Some(elite) = self.archive.sample_elite(&mut self.rng) {
                    Some(elite.clone())
                } else {
                    None
                }
            } else {
                None
            };

            let mut child = if let Some(g) = p1_genome {
                g
            } else {
                let a = self.rng.gen_range(0..pop_len);
                let b = self.rng.gen_range(0..pop_len);
                let c = self.rng.gen_range(0..pop_len);
                let best_idx = if self.population[a].fitness >= self.population[b].fitness
                    && self.population[a].fitness >= self.population[c].fitness
                {
                    a
                } else if self.population[b].fitness >= self.population[c].fitness {
                    b
                } else {
                    c
                };
                self.population[best_idx].genome.clone()
            };

            // Crossover
            if self.rng.gen_bool(0.40) {
                let a = self.rng.gen_range(0..pop_len);
                let b = self.rng.gen_range(0..pop_len);
                let p2_idx = if self.population[a].fitness >= self.population[b].fitness {
                    a
                } else {
                    b
                };
                child = crossover_genomes(&child, &self.population[p2_idx].genome, &mut self.rng);
            }

            // Mutación
            if self.rng.gen_bool(self.config.mutation_rate) {
                mutate_genome_with_ops(&mut child, allowed_ops, &mut self.rng);
            }

            child.prune_dead_nodes();

            if verify_invariants(&child).is_err() {
                child = generate_random_genome_with_ops(&input_strs, self.output_type, 3, allowed_ops, &mut self.rng);
            }

            self.next_population.push(Individual {
                genome: child,
                fitness: 0.0,
            });
        }

        std::mem::swap(&mut self.population, &mut self.next_population);
    }

    /// Reinicia la población para un nuevo desafío con diferentes entradas
    pub fn reset_with_inputs(&mut self, input_names: &[&str]) {
        self.input_names = input_names.iter().map(|s| s.to_string()).collect();
        self.generation = 0;
        self.best_fitness = f64::NEG_INFINITY;
        self.total_evaluations = 0;
        self.population.clear();
        self.next_population.clear();
        self.archive = MapElitesArchive::new(5, 5);

        let allowed_ops = self
            .sota_config
            .as_ref()
            .and_then(|c| c.allowed_ops.as_deref())
            .unwrap_or(AVAILABLE_OPS);

        for _ in 0..self.config.population_size {
            let genome = generate_random_genome_with_ops(input_names, self.output_type, 3, allowed_ops, &mut self.rng);
            self.population.push(Individual {
                genome,
                fitness: 0.0,
            });
        }
        if let Some(first) = self.population.first() {
            self.best_genome = first.genome.clone();
        }
    }
}
