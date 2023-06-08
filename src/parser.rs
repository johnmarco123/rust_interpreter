use crate::token::*;
use crate::expr::*;
use crate::token_type::TokenType;

struct Parser {
    tokens: Vec<Token>,
    current: isize,
}

impl Parser {
    pub fn parse(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }

    pub fn expression(&self) -> Expr {
        return self.equality();
    }

    fn equality(&self) -> Expr {
        let expr: Expr = self.comparison();

        while same(TokenType::BangEqual, TokenType::BangEqual) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::
        }

        return expr
    }
}
