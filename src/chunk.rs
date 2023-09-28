pub enum OpCode {
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: vec![] }
    }

    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte);
    }
}
