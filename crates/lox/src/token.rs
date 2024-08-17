use std::fmt;

use self::token_type::TokenType;

pub mod token_type;

#[derive(Debug, Clone)]
pub struct Token {
    typ: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, literal: String, line: usize) -> Token {
        Self {
            typ,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{:?} {} {}",
            self.typ, self.lexeme, self.literal
        ))
    }
}
