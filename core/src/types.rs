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
    
    // Aritmética Avanzada y Trascendental
    Power,
    Exp,
    Ln,
    Floor,
    Ceil,
    Signum,
    Sinh,
    Cosh,
    Tanh,

    // Aritmética Modular y Teoría de Números
    ModPow,
    Gcd,
    Factorial,

    // Operaciones Discretas de Bits
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    
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
            | OpCode::Modulo | OpCode::Min | OpCode::Max | OpCode::Power
            | OpCode::Gcd | OpCode::BitwiseAnd | OpCode::BitwiseOr | OpCode::BitwiseXor => {
                (vec![DataType::Float, DataType::Float], DataType::Float)
            }
            OpCode::Absolute | OpCode::SquareRoot | OpCode::Sin | OpCode::Cos
            | OpCode::Exp | OpCode::Ln | OpCode::Floor | OpCode::Ceil
            | OpCode::Signum | OpCode::Sinh | OpCode::Cosh | OpCode::Tanh | OpCode::Factorial => {
                (vec![DataType::Float], DataType::Float)
            }
            OpCode::ModPow => {
                (vec![DataType::Float, DataType::Float, DataType::Float], DataType::Float)
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

#[inline]
pub fn safe_pow(a: f64, b: f64) -> f64 {
    let b_clamped = b.clamp(-25.0, 25.0);
    let r = a.abs().powf(b_clamped);
    if r.is_nan() || r.is_infinite() { 0.0 } else { r }
}

#[inline]
pub fn safe_exp(a: f64) -> f64 {
    let r = a.clamp(-40.0, 40.0).exp();
    if r.is_nan() || r.is_infinite() { 0.0 } else { r }
}

#[inline]
pub fn safe_ln(a: f64) -> f64 {
    (a.abs() + 1e-12).ln()
}

#[inline]
pub fn safe_floor(a: f64) -> f64 {
    if a.is_nan() || a.is_infinite() { 0.0 } else { a.floor() }
}

#[inline]
pub fn safe_ceil(a: f64) -> f64 {
    if a.is_nan() || a.is_infinite() { 0.0 } else { a.ceil() }
}

#[inline]
pub fn safe_signum(a: f64) -> f64 {
    if a.is_nan() { 0.0 } else { a.signum() }
}

#[inline]
pub fn safe_sinh(a: f64) -> f64 {
    let r = a.clamp(-30.0, 30.0).sinh();
    if r.is_nan() || r.is_infinite() { 0.0 } else { r }
}

#[inline]
pub fn safe_cosh(a: f64) -> f64 {
    let r = a.clamp(-30.0, 30.0).cosh();
    if r.is_nan() || r.is_infinite() { 0.0 } else { r }
}

#[inline]
pub fn safe_tanh(a: f64) -> f64 {
    let r = a.clamp(-30.0, 30.0).tanh();
    if r.is_nan() || r.is_infinite() { 0.0 } else { r }
}

#[inline]
pub fn safe_mod_pow(base: f64, exp: f64, modulus: f64) -> f64 {
    let m = modulus.abs().round() as i64;
    if m <= 1 {
        return 0.0;
    }
    let b = ((base.round() as i64 % m) + m) % m;
    let mut e = exp.abs().round() as u64;
    if e > 100_000 {
        e = e % (m as u64 - 1).max(1);
    }
    let mut result = 1i64;
    let mut cur = b;
    while e > 0 {
        if e & 1 == 1 {
            result = (result as i128 * cur as i128 % m as i128) as i64;
        }
        cur = (cur as i128 * cur as i128 % m as i128) as i64;
        e >>= 1;
    }
    result as f64
}

#[inline]
pub fn safe_gcd(a: f64, b: f64) -> f64 {
    let mut x = a.abs().round() as i64;
    let mut y = b.abs().round() as i64;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x as f64
}

#[inline]
pub fn safe_factorial(n: f64) -> f64 {
    let k = n.abs().round().clamp(0.0, 20.0) as usize;
    const FACT: [f64; 21] = [
        1.0, 1.0, 2.0, 6.0, 24.0, 120.0, 720.0, 5040.0, 40320.0, 362880.0,
        3628800.0, 39916800.0, 479001600.0, 6227020800.0, 87178291200.0,
        1307674368000.0, 20922789888000.0, 355687428096000.0, 6402373705728000.0,
        121645100408832000.0, 2432902008176640000.0,
    ];
    FACT[k]
}

#[inline]
pub fn safe_bitwise_and(a: f64, b: f64) -> f64 {
    ((a.round() as i64) & (b.round() as i64)) as f64
}

#[inline]
pub fn safe_bitwise_or(a: f64, b: f64) -> f64 {
    ((a.round() as i64) | (b.round() as i64)) as f64
}

#[inline]
pub fn safe_bitwise_xor(a: f64, b: f64) -> f64 {
    ((a.round() as i64) ^ (b.round() as i64)) as f64
}
