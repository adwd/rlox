use crate::scanner::{Scanner, TokenType};

pub fn compile(source: String) {
    let mut scanner = Scanner::new(source);
    let mut line = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:<12} '{}'", token.token_type.to_string(), token.lexeme);

        if token.token_type == TokenType::Eof {
            break;
        }
    }
}
