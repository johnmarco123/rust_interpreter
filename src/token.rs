use crate::token_type::*;

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
   pub fn new(kind: TokenType, lexeme: String,
           literal: Option<String>, line: usize) -> Token {

        return Token {
            kind,
            lexeme,
            literal,
            line
        };
    }

    pub fn to_string(&self) -> String {
        return format!("{:?}{:?}{:?}",self.kind,self.lexeme, self.literal)
    }
}
