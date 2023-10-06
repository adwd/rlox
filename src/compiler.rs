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

        match operator_type {
            TokenType::Plus => self.emit_byte(OP_ADD as u8),
            TokenType::Minus => self.emit_byte(OP_SUBTRACT as u8),
            TokenType::Star => self.emit_byte(OP_MULTIPLY as u8),
            TokenType::Slash => self.emit_byte(OP_DIVIDE as u8),
            _ => unreachable!(),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value = self.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(value);
    }

    fn unary(&mut self) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OP_NEGATE as u8),
            _ => unreachable!(),
        }
    }

    fn get_rule(&mut self, t: TokenType) -> ParseRule {
        match t {
            TokenType::LeftParen => pr(Some(parse_fn::grouping), None, Precedence::None),
            TokenType::RightParen => pr(None, None, Precedence::None),
            TokenType::LeftBrace => pr(None, None, Precedence::None),
            TokenType::RightBrace => pr(None, None, Precedence::None),
            TokenType::Comma => pr(None, None, Precedence::None),
            TokenType::Dot => pr(None, None, Precedence::None),
            TokenType::Minus => pr(
                Some(parse_fn::unary),
                Some(parse_fn::binary),
                Precedence::Term,
            ),
            TokenType::Plus => pr(None, Some(parse_fn::binary), Precedence::Term),
            TokenType::Semicolon => pr(None, None, Precedence::None),
            TokenType::Slash => pr(None, Some(parse_fn::binary), Precedence::Factor),
            TokenType::Star => pr(None, Some(parse_fn::binary), Precedence::Factor),
            TokenType::Bang => pr(None, None, Precedence::None),
            TokenType::BangEqual => pr(None, None, Precedence::None),
            TokenType::Equal => pr(None, None, Precedence::None),
            TokenType::EqualEqual => pr(None, None, Precedence::None),
            TokenType::Greater => pr(None, None, Precedence::None),
            TokenType::GreaterEqual => pr(None, None, Precedence::None),
            TokenType::Less => pr(None, None, Precedence::None),
            TokenType::LessEqual => pr(None, None, Precedence::None),
            TokenType::Identifier => pr(None, None, Precedence::None),
            TokenType::String => pr(None, None, Precedence::None),
            TokenType::Number => pr(Some(parse_fn::number), None, Precedence::None),
            TokenType::And => pr(None, None, Precedence::None),
            TokenType::Class => pr(None, None, Precedence::None),
            TokenType::Else => pr(None, None, Precedence::None),
            TokenType::False => pr(None, None, Precedence::None),
            TokenType::For => pr(None, None, Precedence::None),
            TokenType::Fun => pr(None, None, Precedence::None),
            TokenType::If => pr(None, None, Precedence::None),
            TokenType::Nil => pr(None, None, Precedence::None),
            TokenType::Or => pr(None, None, Precedence::None),
            TokenType::Print => pr(None, None, Precedence::None),
            TokenType::Return => pr(None, None, Precedence::None),
            TokenType::Super => pr(None, None, Precedence::None),
            TokenType::This => pr(None, None, Precedence::None),
            TokenType::True => pr(None, None, Precedence::None),
            TokenType::Var => pr(None, None, Precedence::None),
            TokenType::While => pr(None, None, Precedence::None),
            TokenType::Error => pr(None, None, Precedence::None),
            TokenType::Eof => pr(None, None, Precedence::None),
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
}

fn pr(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> ParseRule {
    ParseRule {
        prefix,
        infix,
        precedence,
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
