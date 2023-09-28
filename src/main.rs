use crate::{
    chunk::{Chunk, OpCode},
    debug::disassemble_chunk,
};

mod chunk;
mod debug;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn);
    disassemble_chunk(&chunk, "test chunk");
}
