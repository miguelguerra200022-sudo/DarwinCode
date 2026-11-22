pub mod types;
pub mod genome;
pub mod vm;
pub mod mutation;
pub mod verifier;
pub mod engine;
pub mod optimizer;
pub mod symmetry;
pub mod map_elites;
pub mod cppn;
pub mod ramanujan;
pub mod srsd_engine;
pub mod oeis_engine;
pub mod elliptic_bsd;
pub mod navier_stokes;
pub mod pi_continued_fractions;

pub use types::{DataType, OpCode, Value};
pub use genome::{GenomeGraph, GenomeNode, NodeId, NodePosition3D, NodeType};
pub use vm::{evaluate_graph, GasMeter, VmError};
pub use verifier::{verify_invariants, VerificationError};
pub use mutation::{crossover_genomes, generate_random_genome, mutate_genome};
pub use engine::{EvolutionEngine, PopulationConfig, SotaEngineConfig, EngineStats};
pub use optimizer::optimize_graph_constants;
pub use symmetry::{augment_dataset_with_symmetries, detect_symmetries, SymmetryReport};
pub use map_elites::{Elite, MapElitesArchive};
pub use cppn::{CppnActivation, CppnGenome, CppnPopulation, EvolvedVoxel};
pub use ramanujan::{
    ConvergenceStep, GeneralizedContinuedFraction, Polynomial, RamanujanSearchEngine,
    CONST_APERY_ZETA3, CONST_CATALAN_G, CONST_E, CONST_GAMMA, CONST_LN2, CONST_PI, CONST_ZETA_5,
};
pub use srsd_engine::{
    analyze_and_prune_dummy_features, generate_srsd_relativistic_energy_dataset,
    solve_srsd_relativistic_benchmark, FeatureSensitivityReport, SrsdDataPoint, SrsdSolutionResult,
};
pub use oeis_engine::{
    DiscoveredRecurrence, HolonomicObstruction, ModularSpectrum, OeisRecurrenceEngine, OeisSequence,
};
pub use elliptic_bsd::{
    canonical_bsd_dataset, count_points_fp, fourier_trace_ap, legendre_symbol,
    root_number_estimate, BSDRecord, EllipticCurve, EllipticError,
};
pub use navier_stokes::{
    evolve_blowup_profile, FluidSimulation1D, SelfSimilarBlowupProfile,
};


