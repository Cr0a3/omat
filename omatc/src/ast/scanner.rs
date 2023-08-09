use crate::ast::token::Token;

pub struct Scanner {
    tokens: Vec<Token>,
    code: String,
    current: i32,
    start: i32,
    line: i32,
}

impl Scanner {
    pub fn new(_code: String) -> Self {
        Scanner {
            tokens: Vec::new(),
            code: _code.clone(),
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) {

    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }
}