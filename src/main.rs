use vm::VM;

use crate::chunk::{op_code::*, Chunk};

mod chunk;
mod debug;
mod value;
mod vm;

fn main() {
    let mut vm = VM::new(Chunk::new());
    let mut chunk = Chunk::new();

    let position = chunk.add_constant(12.3);
    chunk.write(OP_CONSTANT, 123);
    chunk.write(OP_NEGATE, 123);

    chunk.write(position, 123);

    let position = chunk.add_constant(4.56);
    chunk.write(OP_CONSTANT, 124);
    chunk.write(position, 124);

    chunk.write(OP_SUBTRACT, 125);

    let position = chunk.add_constant(7.89);
    chunk.write(OP_CONSTANT, 125);
    chunk.write(position, 125);

    chunk.write(OP_DIVIDE, 126);

    chunk.write(OP_RETURN, 127);
    // disassemble_chunk(&chunk, "test chunk");
    vm.interpret(chunk);
}
