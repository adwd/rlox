use crate::{
    chunk::{op_code::*, Chunk},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};
use enum_iterator::Sequence;

struct Compiler<'a> {
    scanner: Scanner,
    parser: Parser<'a>,
    current_chunk: Chunk,
}

struct Parser<'a> {
    current: Option<Token<'a>>,
    previous: Option<Token<'a>>,
    had_error: bool,
    panic_mode: bool,
}

struct ParseRule {
    prefix: Option<Box<dyn FnOnce() -> ()>>,
    infix: Option<Box<dyn FnOnce() -> ()>>,
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

impl Compiler<'_> {
    fn new<'a>(scanner: Scanner, chunk: Chunk) -> Compiler<'a> {
        Compiler {
            scanner,
            parser: Parser::new(),
            current_chunk: chunk,
        }
    }

    fn current_chunk(&self) -> &Chunk {
        &self.current_chunk
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::Eof {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // no-op
        } else {
            eprint!(" at '{}'", token.lexeme);
        }

        eprintln!(": {}", message);
        self.parser.had_error = true;
    }

    fn error(&mut self, message: &str) {
        let token = self.parser.previous.clone().unwrap();
        self.error_at(&token, message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.parser.current.clone().unwrap(), message);
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            let token = self.scanner.scan_token().clone();
            self.parser.current = Some(token);
            // if self.parser.current.unwrap().token_type != TokenType::Error {
            //     break;
            // }

            // self.error_at_current(self.parser.current.unwrap().lexeme);
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        // if self.parser.current.unwrap().token_type == token_type {
        //     self.advance();
        //     return;
        // }

        // self.error_at_current(message);
    }

    // fn emit_byte(&mut self, byte: u8) {
    //     self.current_chunk()
    //         .write(byte, self.parser.previous.unwrap().line as usize);
    // }

    // fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
    //     self.emit_byte(byte1);
    //     self.emit_byte(byte2);
    // }

    // fn emit_return(&mut self) {
    //     self.emit_byte(OP_RETURN);
    // }

    // fn make_constant(&mut self, value: Value) -> u8 {
    //     let constant = self.current_chunk().add_constant(value);
    //     if constant > u8::MAX {
    //         self.error("Too many constants in one chunk.");
    //         return 0;
    //     }

    //     constant
    // }

    // fn emit_constant(&mut self, value: Value) {
    //     self.emit_bytes(OP_CONSTANT, self.make_constant(value));
    // }

    // fn parse_precedence(&mut self, precedence: Precedence) {
    //     self.advance();

    //     let Some(prefix_rule) = self
    //         .get_rule(self.parser.previous.unwrap().token_type)
    //         .prefix
    //     else {
    //         self.error("Expect expression.");
    //         return;
    //     };

    //     prefix_rule();

    //     while precedence
    //         <= self
    //             .get_rule(self.parser.current.unwrap().token_type)
    //             .precedence
    //     {
    //         self.advance();
    //         let infix_rule = self
    //             .get_rule(self.parser.previous.unwrap().token_type)
    //             .infix;
    //         infix_rule.unwrap()();
    //     }
    // }

    fn expression(&mut self) {
        // self.parse_precedence(Precedence::Assignment);
    }

    // fn binary(&mut self) {
    //     let operator_type = self.parser.previous.unwrap().token_type;

    //     let rule = self.get_rule(operator_type);
    //     self.parse_precedence(rule.precedence.next().unwrap());

    //     match operator_type {
    //         TokenType::Plus => self.emit_byte(OP_ADD),
    //         TokenType::Minus => self.emit_byte(OP_SUBTRACT),
    //         TokenType::Star => self.emit_byte(OP_MULTIPLY),
    //         TokenType::Slash => self.emit_byte(OP_DIVIDE),
    //         _ => unreachable!(),
    //     }
    // }

    // fn grouping(&mut self) {
    //     self.expression();
    //     self.consume(TokenType::RightParen, "Expect ')' after expression.");
    // }

    // fn number(&mut self) {
    //     let value = self.parser.previous.unwrap().lexeme.parse::<f64>().unwrap();
    //     self.emit_constant(value);
    // }

    // fn unary(&mut self) {
    //     let operator_type = self.parser.previous.unwrap().token_type;

    //     self.parse_precedence(Precedence::Unary);

    //     match operator_type {
    //         TokenType::Minus => self.emit_byte(OP_NEGATE),
    //         _ => unreachable!(),
    //     }
    // }

    // fn get_rule(&mut self, t: TokenType) -> ParseRule {
    //     match t {
    //         TokenType::LeftParen => pr(Some(Box::new(|| self.grouping())), None, Precedence::None),
    //         TokenType::RightParen => pr(None, None, Precedence::None),
    //         TokenType::LeftBrace => pr(None, None, Precedence::None),
    //         TokenType::RightBrace => pr(None, None, Precedence::None),
    //         TokenType::Comma => pr(None, None, Precedence::None),
    //         TokenType::Dot => pr(None, None, Precedence::None),
    //         TokenType::Minus => pr(
    //             Some(Box::new(|| self.unary())),
    //             Some(Box::new(|| self.binary())),
    //             Precedence::Term,
    //         ),
    //         TokenType::Plus => pr(None, Some(Box::new(|| self.binary())), Precedence::Term),
    //         TokenType::Semicolon => pr(None, None, Precedence::None),
    //         TokenType::Slash => pr(None, Some(Box::new(|| self.binary())), Precedence::Factor),
    //         TokenType::Star => pr(None, Some(Box::new(|| self.binary())), Precedence::Factor),
    //         TokenType::Bang => pr(None, None, Precedence::None),
    //         TokenType::BangEqual => pr(None, None, Precedence::None),
    //         TokenType::Equal => pr(None, None, Precedence::None),
    //         TokenType::EqualEqual => pr(None, None, Precedence::None),
    //         TokenType::Greater => pr(None, None, Precedence::None),
    //         TokenType::GreaterEqual => pr(None, None, Precedence::None),
    //         TokenType::Less => pr(None, None, Precedence::None),
    //         TokenType::LessEqual => pr(None, None, Precedence::None),
    //         TokenType::Identifier => pr(None, None, Precedence::None),
    //         TokenType::String => pr(None, None, Precedence::None),
    //         TokenType::Number => pr(Some(Box::new(|| self.number())), None, Precedence::None),
    //         TokenType::And => pr(None, None, Precedence::None),
    //         TokenType::Class => pr(None, None, Precedence::None),
    //         TokenType::Else => pr(None, None, Precedence::None),
    //         TokenType::False => pr(None, None, Precedence::None),
    //         TokenType::For => pr(None, None, Precedence::None),
    //         TokenType::Fun => pr(None, None, Precedence::None),
    //         TokenType::If => pr(None, None, Precedence::None),
    //         TokenType::Nil => pr(None, None, Precedence::None),
    //         TokenType::Or => pr(None, None, Precedence::None),
    //         TokenType::Print => pr(None, None, Precedence::None),
    //         TokenType::Return => pr(None, None, Precedence::None),
    //         TokenType::Super => pr(None, None, Precedence::None),
    //         TokenType::This => pr(None, None, Precedence::None),
    //         TokenType::True => pr(None, None, Precedence::None),
    //         TokenType::Var => pr(None, None, Precedence::None),
    //         TokenType::While => pr(None, None, Precedence::None),
    //         TokenType::Error => pr(None, None, Precedence::None),
    //         TokenType::Eof => pr(None, None, Precedence::None),
    //     }
    // }
}

fn pr(
    prefix: Option<Box<dyn FnOnce() -> ()>>,
    infix: Option<Box<dyn FnOnce() -> ()>>,
    precedence: Precedence,
) -> ParseRule {
    ParseRule {
        prefix,
        infix,
        precedence,
    }
}

impl Parser<'_> {
    fn new<'a>() -> Parser<'a> {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }
}

pub fn compile(source: String, chunk: Chunk) -> bool {
    let mut compiler = Compiler::new(Scanner::new(source), chunk);
    compiler.advance();
    compiler.expression();
    compiler.consume(TokenType::Eof, "Expect end of expression.");

    !compiler.parser.had_error
}
