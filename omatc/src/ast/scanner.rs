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
            code: _code.clone(),
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

    fn is_at_end(&mut self) -> bool {
        self.current >= self.tokens.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let peek_result = self.peek();
        peek_result
    }

    fn peek(&mut self) -> char {
        self.code.chars().nth(self.current).expect("error")
    }

    fn scan_token(&mut self) {
        let c: char = self.peek();
        println!("{}", c);
    } 

    pub fn scan(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.current += 1;

            self.scan_token();
        }

        self.add_token(TokenTyp::EOF);
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }
}