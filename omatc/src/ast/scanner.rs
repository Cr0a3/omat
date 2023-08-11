use crate::ast::token::*;

pub struct Scanner {
    tokens: Vec<Token>,
    code: String,
    current: usize,
    start: usize,
    line: i32,
    file: String,
}

impl Scanner {
    pub fn new(_code: String, _file: String) -> Self {
        Scanner {
            tokens: Vec::new(),
            code: format!("a{}", _code),    // the a because, peek() + advance() do not get the first char out of the string
            current: 0,
            start: 0,
            line: 1,
            file: _file.clone(),
        }
    }

    fn add_token(&mut self, _token_type: TokenTyp) {
        let tok: Token  = Token::new(_token_type, self.line, String::new(), self.file.clone());

        self.tokens.push(tok);
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.code.len() -1)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.peek()
    }

    fn peek(&self) -> char {
        self.code.chars().nth(self.current).expect("code chars out of range")
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            _ => { 
                eprintln!("unexpected character: {}", c);
            },
        }
    } 

    pub fn scan(&mut self) {
        while !self.is_at_end() {
            self.start = self.current.clone();

            self.scan_token();
        }

        self.add_token(TokenTyp::EOF);
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }
}