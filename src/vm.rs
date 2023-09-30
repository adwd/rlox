use crate::{
    chunk::{
        op_code::{OP_CONSTANT, OP_NEGATE, OP_RETURN},
        Chunk,
    },
    debug::disassemble_instruction,
    value::{print_value, Value},
};

#[derive(Debug)]
pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            ip: 0,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        println!("{self:?}");

        loop {
            if cfg!(debug_assertions) {
                print!("          ");
                for slot in self.stack.iter() {
                    print!("[ ");
                    print_value(slot);
                    print!(" ]");
                }
                println!();
                disassemble_instruction(&self.chunk, self.ip);
            }
            let instruction = self.read_byte();

            match instruction {
                OP_CONSTANT => {
                    let value = self.read_constant();
                    self.push(value);
                    print_value(&value);
                    println!();
                }
                OP_NEGATE => {
                    let value = -self.pop();
                    self.push(value);
                }
                OP_RETURN => {
                    print_value(&self.pop());
                    println!();
                    return InterpretResult::Ok;
                }
                unknown_opcode => {
                    println!("Unknown opcode {}", unknown_opcode);
                    return InterpretResult::CompileError;
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let instruction = self.chunk.code[self.ip];
        self.ip += 1;
        instruction
    }

    fn read_constant(&mut self) -> Value {
        let position = self.read_byte();
        self.chunk.constants.values[position as usize]
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}
