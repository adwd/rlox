use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {name} ==");

    for (index, instruction) in chunk.code.iter().enumerate() {
        match instruction {
            OpCode::OpReturn => println!("{index:04} OP_RETURN"),
        }
    }
}
