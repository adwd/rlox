pub enum ValueType {
    Bool,
    Nil,
    Number,
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
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
    }
}
