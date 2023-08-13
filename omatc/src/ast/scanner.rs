use crate::ast::token::*;
use crate::error::error;

pub struct Scanner {
    tokens: Vec<Token>,
    code: String,

    current: usize,
    start: usize,
    line: usize,

    pos_in_line: isize,
    line_str: String,
    line_vec: Vec<String>,

    file: String,
}

impl Scanner {
    pub fn new(_code: String, _file: String) -> Self {

        let lines: Vec<String> = _code.lines().map(String::from).collect();

        let mut first_line: String = String::new();

        if let Some(first) = lines.first().cloned() {
            first_line = first;
        } else {
            std::process::exit(0);
        }

        Scanner {
            tokens: Vec::new(),
            code: format!("a{}", _code),    // the a because, peek() + advance() do not get the first char out of the string
            current: 0,
            start: 0,
            line: 1,
            pos_in_line: -1,
            line_vec: lines,
            line_str: first_line,
            file: _file.clone(),
        }
    }

    fn add_token(&mut self, _token_type: TokenTyp) {
        let tok: Token  = Token::new(_token_type, self.line as i32, String::new(), self.file.clone());

        self.tokens.push(tok);
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.code.len() -1)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let peek_res = self.peek();

        if peek_res == '\n' {
            self.pos_in_line = 0;
            self.line += 1;

            if let Some(first) = self.line_vec.get(self.line -1).cloned() {
                self.line_str = first;
            } else {
                eprintln!("error, while resolving new line");
            }

        }
        else {
            self.pos_in_line += 1;
        }

        peek_res
    }

    fn peek(&self) -> char {
        self.code.chars().nth(self.current).expect("code chars out of range")
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '+' => {
                if self.peek() == '+' { //++
                    self.add_token(TokenTyp::AddAdd);
                }
                else if self.peek() == '=' { //+=
                    self.add_token(TokenTyp::AddEqual);
                }
                else { //+
                    self.add_token(TokenTyp::ADD);
                }
            },

            '-' => {
                if self.peek() == '-' { //--
                    self.add_token(TokenTyp::MinMin);
                }
                else if self.peek() == '=' { //-=
                    self.add_token(TokenTyp::MinEqual);
                }
                else { //-
                    self.add_token(TokenTyp::MIN);
                }
            },

            '*' => {
                if self.peek() == '=' { // *=
                    self.add_token(TokenTyp::MulEqual);
                }
                else if self.peek() == '*' { // **
                    self.add_token(TokenTyp::POW);
                }
                else {  //*
                    self.add_token(TokenTyp::MUL);
                }
            },

            '/' => {
                if self.peek() == '/' { // single line comment  //...
                    while self.advance() != '\n' {}
                }
                else if self.peek() == '*' {    // multi line comment /*...*/
                    let mut last_advance: char = ' ';
                    while last_advance == '*' && self.peek() == '/' {
                        last_advance = self.advance();
                    }
                }
                else if self.peek() == '=' {  // /=
                    self.add_token(TokenTyp::DivEqual);
                }
                else {
                    self.add_token(TokenTyp::DIV);
                }
            },

            '(' => {
                self.add_token(TokenTyp::LeftParam);
            }

            ')' => {
                self.add_token(TokenTyp::RightParam);
            }

            '{' => {
                self.add_token(TokenTyp::LeftBracet);

            }

            '}' => {
                self.add_token(TokenTyp::RightBracet);

            }

            '.' => {
                self.add_token(TokenTyp::DOT);
            }

            ',' => {
                self.add_token(TokenTyp::COMMA);
            }

            '_' => {
                self.add_token(TokenTyp::UNDER_SCORE);
            }

            ' ' => {}
            '\r' => {}
            '\t' => {}

            _ => { 
                error::error("E0001", "unexpected character", self.file.as_str(), self.line_str.clone(), self.line, self.pos_in_line as usize, 1);
            }
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