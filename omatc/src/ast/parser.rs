use crate::ast::token::Token;
use crate::ast::expr::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    exprs: Vec<Expr>,
    current: usize,
    start: usize,
}

impl Parser {
    pub fn new(_tokens: &Vec<Token>) -> Self {
        Parser {
            tokens: *_tokens,
            exprs: Vec::new(),
            current: 0,
            start: 0,
        }
    }

    fn parse_expr(&mut self) {
        let token = self.tokens.get(self.current).expect("vector out of range");
        token.print();
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.tokens.len() 
    } 

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            self.start = self.current.clone();
            self.current += 1;

            self.parse_expr();
        }
    }

    pub fn get_exprs(self) -> Vec<Expr> {
        self.exprs
    }
}