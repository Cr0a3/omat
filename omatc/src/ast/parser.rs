use crate::ast::token::TokenTyp;
use crate::ast::expr::Expr;

pub struct Parser {
    tokens: Vec<TokenTyp>,
    exprs: Vec<Expr>,
}

impl Parser {
    pub fn new(_tokens: Vec<TokenTyp>) -> Self {
        Parser {
            tokens: _tokens,
            exprs: Vec::new(),
        }
    }

    pub fn parse(&self) {

    }

    pub fn get_exprs(&self) -> Vec<Expr> {
        self.exprs
    }
}