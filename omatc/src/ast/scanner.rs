use crate::ast::token;

pub struct Scanner {
    tokens: Vec<token::TokenTyp>
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            tokens: Vec::new(),
        }
    }

    pub fn scan(&self) {

    }
}