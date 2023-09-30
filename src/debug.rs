use crate::{
    chunk::{op_code::*, Chunk},
    value::print_value,
};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {name} ==");

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(&chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ")
    } else {
        print!("{:4} ", chunk.lines[offset])
    }

    match chunk.code[offset] {
        OP_CONSTANT => constant_instruction("OP_CONSTANT", chunk, offset),
        OP_RETURN => simple_instruction("OP_RETURN", offset),
        unknown_opcode => {
            println!("Unknown opcode {}", unknown_opcode);
            offset + 1
        }
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:<16} {:>4} '", name, constant);
    print_value(&chunk.constants.values[constant as usize]);
    println!("'");
    offset + 2
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
