pub type Value = f64;

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
    print!("{}", value);
}
