use crate::ast::token::Token;

use super::token::TokenTyp;

pub struct Scanner {
    tokens: Vec<Token>,
    code: std::str::Chars<'_>,
    file: String,
    current: usize,
    start: usize,
    line: i32,
}

impl Scanner {
    pub fn new(_code: &String, _file: &String) -> Self {
        Scanner {
            tokens: Vec::new(),
            code: (*_code).chars(),
            current: 0,
            start: 0,
            line: 1,
            file: *_file,
        }
    }

    fn add_token(&self, _token_type: &TokenTyp) {
        let mut tok: Token  = Token::new(_token_type, &self.line, &String::new(), &self.file);

        self.tokens.push(tok);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn advance(&self) -> char {
        self.current += 1;
        self.peek()
    }

    fn peek(&self) -> char {
        self.code.nth(self.current).expect("error")
    }

    fn scan_token(&self) {
        let c: char = self.peek();
        println!("{}", c);
    } 

    pub fn scan(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.current += 1;

            self.scan_token();
        }

        self.add_token(&TokenTyp::EOF);
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }
}