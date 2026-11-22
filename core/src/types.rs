use serde::{Deserialize, Serialize};

/// Tipos de datos estrictos del sistema genético de DarwinCode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    Float,
    Int,
    Boolean,
    Vec3,
}

/// Valores en tiempo de ejecución del intérprete.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Float(f64),
    Int(i64),
    Boolean(bool),
    Vec3([f64; 3]),
}

impl Value {
    pub fn get_type(&self) -> DataType {
        match self {
            Value::Float(_) => DataType::Float,
            Value::Int(_) => DataType::Int,
            Value::Boolean(_) => DataType::Boolean,
            Value::Vec3(_) => DataType::Vec3,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            Value::Float(f) => Some(*f as i64),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_vec3(&self) -> Option<[f64; 3]> {
        match self {
            Value::Vec3(v) => Some(*v),
            _ => None,
        }
    }

    pub fn default_for_type(dt: DataType) -> Self {
        match dt {
            DataType::Float => Value::Float(0.0),
            DataType::Int => Value::Int(0),
            DataType::Boolean => Value::Boolean(false),
            DataType::Vec3 => Value::Vec3([0.0, 0.0, 0.0]),
        }
    }
}

/// Códigos de operación atómicos disponibles para los nodos del genoma.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OpCode {
    // Operaciones Aritméticas (Float / Int)
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Absolute,
    SquareRoot,
    Sin,
    Cos,
    Min,
    Max,
    
    // Operaciones Lógicas y de Comparación
    LessThan,
    GreaterThan,
    Equal,
    And,
    Or,
    Not,
    IfElse,

    // Operaciones Vectoriales 3D
    Vec3Make,
    Vec3Add,
    Vec3Scale,
    Vec3Length,
    Vec3Dot,
}

impl OpCode {
    /// Devuelve los tipos esperados para cada argumento de entrada y el tipo devuelto.
    pub fn signature(&self) -> (Vec<DataType>, DataType) {
        match self {
            OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide 
            | OpCode::Modulo | OpCode::Min | OpCode::Max => {
                (vec![DataType::Float, DataType::Float], DataType::Float)
            }
            OpCode::Absolute | OpCode::SquareRoot | OpCode::Sin | OpCode::Cos => {
                (vec![DataType::Float], DataType::Float)
            }
            OpCode::LessThan | OpCode::GreaterThan | OpCode::Equal => {
                (vec![DataType::Float, DataType::Float], DataType::Boolean)
            }
            OpCode::And | OpCode::Or => {
                (vec![DataType::Boolean, DataType::Boolean], DataType::Boolean)
            }
            OpCode::Not => {
                (vec![DataType::Boolean], DataType::Boolean)
            }
            OpCode::IfElse => {
                (vec![DataType::Boolean, DataType::Float, DataType::Float], DataType::Float)
            }
            OpCode::Vec3Make => {
                (vec![DataType::Float, DataType::Float, DataType::Float], DataType::Vec3)
            }
            OpCode::Vec3Add => {
                (vec![DataType::Vec3, DataType::Vec3], DataType::Vec3)
            }
            OpCode::Vec3Scale => {
                (vec![DataType::Vec3, DataType::Float], DataType::Vec3)
            }
            OpCode::Vec3Length | OpCode::Vec3Dot => {
                (vec![DataType::Vec3, DataType::Vec3], DataType::Float)
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Operaciones Aritméticas Seguras (Safe Math Ops) para Cero Errores / Cero NaN
// ----------------------------------------------------------------------------

pub const EPSILON: f64 = 1e-7;

#[inline]
pub fn safe_div(a: f64, b: f64) -> f64 {
    if b.abs() < EPSILON {
        if b >= 0.0 {
            a * (1.0 / EPSILON)
        } else {
            a * (-1.0 / EPSILON)
        }
    } else {
        let r = a / b;
        if r.is_nan() || r.is_infinite() {
            0.0
        } else {
            r
        }
    }
}

#[inline]
pub fn safe_sqrt(a: f64) -> f64 {
    a.abs().sqrt()
}

#[inline]
pub fn safe_mod(a: f64, b: f64) -> f64 {
    if b.abs() < EPSILON {
        0.0
    } else {
        a % b
    }
}
