use crate::object::{print_object, Object};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    Object(Object),
}

#[derive(Debug)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: vec![] }
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }
}

pub fn print_value(value: &Value) {
    match value {
        Value::Boolean(b) => print!("{}", b),
        Value::Number(n) => print!("{}", n),
        Value::Nil => print!("nil"),
        Value::Object(o) => print_object(o),
    }
}

pub fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Boolean(a), Value::Boolean(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::Object(a), Value::Object(b)) => {
            if let (Object::String(a), Object::String(b)) = (a, b) {
                a == b
            } else {
                false
            }
        }
        _ => false,
    }
}
