use std::cell::Cell;

use crate::{
    chunk::{Chunk, OpCode::*},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};
use enum_iterator::Sequence;

pub struct Compiler {
    scanner: Scanner,
    current_chunk: Chunk,
    current: Token,
    previous: Token,
    had_error: Cell<bool>,
    panic_mode: Cell<bool>,
}

type ParseFn = fn(&mut Compiler) -> ();

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

macro_rules! rule {
    (None, None, $precedence:ident) => {
        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::$precedence,
        }
    };
    ($prefix:ident, None, $precedence:ident) => {
        ParseRule {
            prefix: Some(parse_fn::$prefix),
            infix: None,
            precedence: Precedence::$precedence,
        }
    };
    (None, $infix:ident, $precedence:ident) => {
        ParseRule {
            prefix: None,
            infix: Some(parse_fn::$infix),
            precedence: Precedence::$precedence,
        }
    };
    ($prefix:ident, $infix:ident, $precedence:ident) => {
        ParseRule {
            prefix: Some(parse_fn::$prefix),
            infix: Some(parse_fn::$infix),
            precedence: Precedence::$precedence,
        }
    };
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Sequence)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Compiler {
    fn new<'a>(scanner: Scanner, chunk: Chunk) -> Compiler {
        Compiler {
            scanner,
            current_chunk: chunk,
            current: Token::none(),
            previous: Token::none(),
            had_error: Cell::new(false),
            panic_mode: Cell::new(false),
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.current_chunk
    }

    fn error_at(&self, token: &Token, message: &str) {
        if self.panic_mode.get() {
            return;
        }
        self.panic_mode.set(true);
        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::Eof {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // no-op
        } else {
            eprint!(" at '{}'", token.lexeme);
        }

        eprintln!(": {}", message);
        self.had_error.set(true);
    }

    fn error(&self, message: &str) {
        self.error_at(&self.previous, message);
    }

    fn error_at_current(&self, message: &str) {
        self.error_at(&self.current, message);
    }

    fn advance(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current);

        loop {
            let token = self.scanner.scan_token();
            self.current = token;
            if self.current.token_type != TokenType::Error {
                break;
            }

            self.error_at_current(&self.current.lexeme);
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.line as usize;
        self.current_chunk().write(byte, line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OP_RETURN as u8);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk().add_constant(value);
        if constant > u8::MAX {
            self.error("Too many constants in one chunk.");
            return 0;
        }

        constant
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OP_CONSTANT as u8, constant);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let Some(prefix_rule) = self.get_rule(self.previous.token_type).prefix else {
            self.error("Expect expression.");
            return;
        };

        prefix_rule(self);

        while precedence <= self.get_rule(self.current.token_type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.previous.token_type).infix;
            infix_rule.unwrap()(self);
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn binary(&mut self) {
        let operator_type = self.previous.token_type;

        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence.next().unwrap());

        use TokenType::*;
        match operator_type {
            BangEqual => {
                self.emit_bytes(OP_EQUAL as u8, OP_NOT as u8);
            }
            EqualEqual => {
                self.emit_byte(OP_EQUAL as u8);
            }
            Greater => {
                self.emit_byte(OP_GREATER as u8);
            }
            GreaterEqual => {
                self.emit_bytes(OP_LESS as u8, OP_NOT as u8);
            }
            Less => {
                self.emit_byte(OP_LESS as u8);
            }
            LessEqual => {
                self.emit_bytes(OP_GREATER as u8, OP_NOT as u8);
            }
            Plus => self.emit_byte(OP_ADD as u8),
            Minus => self.emit_byte(OP_SUBTRACT as u8),
            Star => self.emit_byte(OP_MULTIPLY as u8),
            Slash => self.emit_byte(OP_DIVIDE as u8),
            _ => unreachable!(),
        }
    }

    fn literal(&mut self) {
        use TokenType::*;
        match self.previous.token_type {
            False => self.emit_byte(OP_FALSE as u8),
            Nil => self.emit_byte(OP_NIL as u8),
            True => self.emit_byte(OP_TRUE as u8),
            _ => unreachable!(),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value = self.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn unary(&mut self) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Bang => self.emit_byte(OP_NOT as u8),
            TokenType::Minus => self.emit_byte(OP_NEGATE as u8),
            _ => unreachable!(),
        }
    }

    fn get_rule(&mut self, t: TokenType) -> ParseRule {
        use TokenType::*;
        match t {
            LeftParen => rule!(grouping, None, None),
            RightParen => rule!(None, None, None),
            LeftBrace => rule!(None, None, None),
            RightBrace => rule!(None, None, None),
            Comma => rule!(None, None, None),
            Dot => rule!(None, None, None),
            Minus => rule!(unary, binary, Term),
            Plus => rule!(None, binary, Term),
            Semicolon => rule!(None, None, None),
            Slash => rule!(None, binary, Factor),
            Star => rule!(None, binary, Factor),
            Bang => rule!(unary, None, None),
            BangEqual => rule!(None, binary, Equality),
            Equal => rule!(None, None, None),
            EqualEqual => rule!(None, binary, Equality),
            Greater => rule!(None, binary, Comparison),
            GreaterEqual => rule!(None, binary, Comparison),
            Less => rule!(None, binary, Comparison),
            LessEqual => rule!(None, binary, Comparison),
            Identifier => rule!(None, None, None),
            String => rule!(None, None, None),
            Number => rule!(number, None, None),
            And => rule!(None, None, None),
            Class => rule!(None, None, None),
            Else => rule!(None, None, None),
            False => rule!(literal, None, None),
            For => rule!(None, None, None),
            Fun => rule!(None, None, None),
            If => rule!(None, None, None),
            Nil => rule!(literal, None, None),
            Or => rule!(None, None, None),
            Print => rule!(None, None, None),
            Return => rule!(None, None, None),
            Super => rule!(None, None, None),
            This => rule!(None, None, None),
            True => rule!(literal, None, None),
            Var => rule!(None, None, None),
            While => rule!(None, None, None),
            Error => rule!(None, None, None),
            Eof => rule!(None, None, None),
        }
    }
}

mod parse_fn {
    use super::*;

    pub fn binary(compiler: &mut Compiler) {
        compiler.binary();
    }

    pub fn grouping(compiler: &mut Compiler) {
        compiler.grouping();
    }

    pub fn number(compiler: &mut Compiler) {
        compiler.number();
    }

    pub fn unary(compiler: &mut Compiler) {
        compiler.unary();
    }

    pub fn literal(compiler: &mut Compiler) {
        compiler.literal();
    }
}

pub fn compile(source: String) -> Option<Chunk> {
    let mut compiler = Compiler::new(Scanner::new(source), Chunk::new());
    compiler.advance();
    compiler.expression();
    compiler.consume(TokenType::Eof, "Expect end of expression.");
    compiler.end_compiler();

    if !compiler.had_error.get() {
        Some(compiler.current_chunk)
    } else {
        None
    }
}
