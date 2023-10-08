use crate::{
    chunk::{
        Chunk,
        OpCode::{self, *},
    },
    compiler::compile,
    debug::disassemble_instruction,
    value::{print_value, values_equal, Value},
};

#[derive(Debug)]
pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    stack_: Vec<Value>,
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
            stack_: vec![],
        }
    }

    fn reset_stack(&mut self) {
        self.stack = self.stack_.clone();
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let Some(chunk) = compile(source) else {
            return InterpretResult::CompileError;
        };

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
            let instruction = self.read_byte::<OpCode>();

            match instruction {
                OP_CONSTANT => {
                    let value = self.read_constant();
                    print_value(&value);
                    self.push(value);
                    println!();
                }
                OP_NIL => self.push(Value::Nil),
                OP_TRUE => self.push(Value::Boolean(true)),
                OP_FALSE => self.push(Value::Boolean(false)),
                OP_EQUAL => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(values_equal(&a, &b)));
                }
                OP_GREATER => self.binary_op(|a, b| Value::Boolean(a > b)),
                OP_LESS => self.binary_op(|a, b| Value::Boolean(a < b)),
                OP_ADD => self.binary_op(|a, b| Value::Number(a + b)),
                OP_SUBTRACT => self.binary_op(|a, b| Value::Number(a - b)),
                OP_MULTIPLY => self.binary_op(|a, b| Value::Number(a * b)),
                OP_DIVIDE => self.binary_op(|a, b| Value::Number(a / b)),
                OP_NOT => {
                    let value = self.pop();
                    self.push(Value::Boolean(Self::is_falsy(&value)));
                }
                OP_NEGATE => match self.peek(0) {
                    Value::Number(n) => self.push(Value::Number(-n)),
                    _ => {
                        self.runtime_error("Operand must be a number.");
                        return InterpretResult::RuntimeError;
                    }
                },
                OP_RETURN => {
                    print_value(&self.pop());
                    println!();
                    return InterpretResult::Ok;
                }
                unknown_opcode => {
                    println!("Unknown opcode {:?}", unknown_opcode);
                    return InterpretResult::CompileError;
                }
            }
        }
    }

    fn read_byte<T>(&mut self) -> T
    where
        T: From<u8>,
    {
        let instruction = self.chunk.code[self.ip];
        self.ip += 1;
        instruction.into()
    }

    fn read_constant(&mut self) -> Value {
        let position = self.read_byte::<u8>();
        self.chunk.constants.values[position as usize]
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack.len() - 1 - distance]
    }

    fn is_falsy(value: &Value) -> bool {
        match value {
            Value::Nil => true,
            Value::Boolean(b) => !b,
            _ => false,
        }
    }

    fn binary_op(&mut self, op: impl FnOnce(f64, f64) -> Value) {
        match (self.peek(0), self.peek(1)) {
            (Value::Number(_), Value::Number(_)) => {
                let b = self.pop();
                let a = self.pop();
                match (a, b) {
                    (Value::Number(a), Value::Number(b)) => self.push(op(a, b)),
                    _ => unreachable!(),
                }
            }
            _ => {
                self.runtime_error("Operands must be numbers.");
                return;
            }
        }
    }

    fn runtime_error(&mut self, message: &str) {
        eprintln!("{}", message);

        let instruction = self.ip - 1;
        let line = self.chunk.lines[instruction] as usize;
        eprintln!("[line {}] in script", line);

        self.reset_stack();
    }
}
