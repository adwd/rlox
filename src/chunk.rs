use crate::value::{Value, ValueArray};

pub mod op_code {
    pub const OP_CONSTANT: u8 = 0;
    pub const OP_RETURN: u8 = 1;
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            lines: vec![],
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write(value);
        (self.constants.values.len() - 1) as u8
    }
}
