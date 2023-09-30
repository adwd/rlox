mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use crate::chunk::Chunk;
use crate::vm::InterpretResult;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use vm::VM;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: clox [path]");
            std::process::exit(64);
        }
    }
}

fn repl() {
    let stdin = io::stdin();
    let mut vm = VM::new(Chunk::new());

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();

        let result = vm.interpret(line);
        match result {
            InterpretResult::CompileError => std::process::exit(65),
            InterpretResult::RuntimeError => std::process::exit(70),
            _ => (),
        }
    }
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn run_file(path: &str) {
    let source = read_file(path);
    let mut vm = VM::new(Chunk::new());
    let result = vm.interpret(source);
    match result {
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
        _ => (),
    }
}
