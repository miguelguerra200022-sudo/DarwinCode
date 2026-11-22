use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum VmError {
    #[error("Límite de gas agotado (prevención de bucle infinito)")]
    GasExhausted,
    #[error("Entrada requerida no proporcionada: {0}")]
    MissingInput(usize),
    #[error("Nodo de cálculo inválido o no encontrado: {0}")]
    InvalidNode(usize),
    #[error("Incompatibilidad de tipo en tiempo de ejecución")]
    TypeMismatch,
    #[error("El grafo no tiene un nodo de salida asignado")]
    UnsetOutput,
    #[error("Error de ciclo o grafo no evaluable: {0}")]
    EvaluationError(&'static str),
}

#[derive(Debug, Clone)]
pub struct GasMeter {
    pub remaining: u64,
}

impl GasMeter {
    pub fn new(limit: u64) -> Self {
        Self { remaining: limit }
    }

    #[inline]
    pub fn consume(&mut self, amount: u64) -> Result<(), VmError> {
        if self.remaining < amount {
            self.remaining = 0;
            Err(VmError::GasExhausted)
        } else {
            self.remaining -= amount;
            Ok(())
        }
    }
}
