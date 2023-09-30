use crate::{
    chunk::{op_code::*, Chunk},
    debug::disassemble_chunk,
};

mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = Chunk::new();

    let position = chunk.add_constant(12.3);
    chunk.write(OP_CONSTANT, 123);
    chunk.write(position, 123);

    chunk.write(OP_RETURN, 124);
    disassemble_chunk(&chunk, "test chunk");
}
