use crate::ast::token::TokenTyp;

pub struct Scanner {
    tokens: Vec<TokenTyp>,
    code: String,
}

impl Scanner {
    pub fn new(_code: String) -> Self {
        Scanner {
            tokens: Vec::new(),
            code: _code.clone()
        }
    }

    pub fn scan(&self) {

    }

    pub fn get_tokens(self) -> Vec<TokenTyp> {
        self.tokens
    }
}