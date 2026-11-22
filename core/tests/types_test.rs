use darwin_core::types::{safe_div, safe_mod, safe_sqrt, DataType, OpCode, Value};

#[test]
fn test_safe_division_by_zero_and_near_zero() {
    let r1 = safe_div(10.0, 0.0);
    assert!(!r1.is_nan());
    assert!(!r1.is_infinite());
    assert!(r1 > 1e6);

    let r2 = safe_div(10.0, -0.0);
    assert!(!r2.is_nan());
    assert!(!r2.is_infinite());

    let r3 = safe_div(10.0, 2.0);
    assert_eq!(r3, 5.0);
}

#[test]
fn test_safe_sqrt_negative() {
    let r = safe_sqrt(-16.0);
    assert_eq!(r, 4.0);
}

#[test]
fn test_safe_mod_zero() {
    let r = safe_mod(10.0, 0.0);
    assert_eq!(r, 0.0);
}

#[test]
fn test_value_types() {
    assert_eq!(Value::Float(1.23).get_type(), DataType::Float);
    assert_eq!(Value::Int(42).get_type(), DataType::Int);
    assert_eq!(Value::Boolean(true).get_type(), DataType::Boolean);
    assert_eq!(Value::Vec3([1.0, 2.0, 3.0]).get_type(), DataType::Vec3);
}

#[test]
fn test_opcode_signatures() {
    let (inputs, out) = OpCode::Add.signature();
    assert_eq!(inputs, vec![DataType::Float, DataType::Float]);
    assert_eq!(out, DataType::Float);

    let (inputs, out) = OpCode::LessThan.signature();
    assert_eq!(inputs, vec![DataType::Float, DataType::Float]);
    assert_eq!(out, DataType::Boolean);
}
