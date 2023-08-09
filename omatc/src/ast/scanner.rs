use crate::ast::token::TokenTyp;

pub struct Scanner {
    tokens: Vec<TokenTyp>
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            tokens: Vec::new(),
        }
    }

    pub fn scan(&self) {

    }

    pub fn get_tokens(&self) -> Vec<TokenTyp> {
        
    }
}