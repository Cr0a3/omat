use crate::ast::token::*;
use crate::ast::expr::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    exprs: Vec<Expr>,
    current: usize,
    start: usize,

    lastToken: Option<Token>,
}

impl Parser {
    pub fn new(_tokens: &mut Vec<Token>) -> Self {
        _tokens.insert(0, Token::new(TokenTyp::EOF, 0, String::new(), String::new()));
        Parser {
            tokens: _tokens.to_owned(),
            exprs: Vec::new(),
            current: 0,
            start: 0,
            lastToken: None,
        }
    }

    fn advance(&mut self) -> &Token {
        self.lastToken = Option::from(self.peek().to_owned());
        self.current += 1; 
        let res = self.peek();
        res
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).expect("token vector out of range")
    }

    fn parse_expr(&mut self) {
        let token = self.advance();
        token.print();
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= ( self.tokens.len() -1)
    } 

    pub fn parse(&mut self) {

        while !self.is_at_end() {
            self.start = self.current.clone();

            self.parse_expr();
        }
    }

    pub fn get_exprs(self) -> Vec<Expr> {
        self.exprs
    }
}