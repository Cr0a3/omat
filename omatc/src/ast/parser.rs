use crate::ast::token::Token;
use crate::ast::expr::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    exprs: Vec<Expr>,
    current: i32,
    start: i32,
}

impl Parser {
    pub fn new(_tokens: Vec<Token>) -> Self {
        Parser {
            tokens: _tokens,
            exprs: Vec::new(),
            current: 0,
            start: 0,
        }
    }

    pub fn parse(&mut self) {

    }

    pub fn get_exprs(self) -> Vec<Expr> {
        self.exprs
    }
}