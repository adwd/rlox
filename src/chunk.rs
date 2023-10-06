use crate::value::{Value, ValueArray};

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OP_CONSTANT,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_NEGATE,
    OP_RETURN,

    UNKNOWN,
}

impl From<u8> for OpCode {
    fn from(from: u8) -> Self {
        use self::OpCode::*;
        match from {
            0 => OP_CONSTANT,
            1 => OP_ADD,
            2 => OP_SUBTRACT,
            3 => OP_MULTIPLY,
            4 => OP_DIVIDE,
            5 => OP_NEGATE,
            6 => OP_RETURN,
            _ => UNKNOWN,
        }
    }
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
