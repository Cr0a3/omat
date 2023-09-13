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

        let  first_line: String;

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

    fn add_token_l(&mut self, _token_type: TokenTyp, lexme: String) {
        let tok: Token  = Token::new(_token_type, self.line as i32, lexme, self.file.clone());

        self.tokens.push(tok);
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.code.len() -1)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let mut peek_res = self.peek();

        if peek_res == '\n' {
            self.pos_in_line = 0;
            self.line += 1;

            if let Some(first) = self.line_vec.get(self.line -1).cloned() {
                self.line_str = first;
            } else {
                eprintln!("error, while resolving new line");
            }

            peek_res = self.advance(); //result = new advance

        }
        else {
            self.pos_in_line += 1;
        }

        peek_res
    }

    fn peek(&self) -> char {
        self.code.chars().nth(self.current -1).expect("code chars out of range")
    }

    fn peek_next(&self) -> char {
        self.code.chars().nth(self.current).expect("code chars out of range")
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '+' => {
                if self.peek_next() == '+' { //++
                    self.add_token(TokenTyp::AddAdd);
                    self.advance(); //get the next token, else wrong tockens
                }
                else if self.peek_next() == '=' { //+=
                    self.add_token(TokenTyp::AddEqual);
                    self.advance(); //get the next token, else wrong tockens
                }
                else { //+
                    self.add_token(TokenTyp::ADD);
                }
            },

            '-' => {
                if self.peek_next() == '-' { //--
                    self.add_token(TokenTyp::MinMin);
                    self.advance(); //get the next token, else wrong tockens
                }
                else if self.peek_next() == '=' { //-=
                    self.add_token(TokenTyp::MinEqual);
                    self.advance(); //get the next token, else wrong tockens
                }

                else if self.peek_next() == '>' { //->
                    self.add_token(TokenTyp::RArrow);
                    self.advance(); //get the next token, else error
                }

                else { //-
                    self.add_token(TokenTyp::MIN);
                }
            },

            '*' => {
                if self.peek_next() == '=' { // *=
                    self.add_token(TokenTyp::MulEqual);
                    self.advance(); //get the next token, else wrong tockens
                }
                else if self.peek_next() == '*' { // **
                    self.add_token(TokenTyp::POW);
                    self.advance(); //get the next token, else wrong tockens
                }
                else {  //*
                    self.add_token(TokenTyp::MUL);
                }
            },

            '/' => {
                if self.peek_next() == '/' { // single line comment  //...
                    while self.advance() != '\n' {}
                }
                else if self.peek_next() == '*' {    // multi line comment /*...*/
                    let mut last_advance: char = ' ';
                    while last_advance == '*' && self.peek_next() == '/' {
                        last_advance = self.advance();
                    }
                }
                else if self.peek_next() == '=' {  // /=
                    self.add_token(TokenTyp::DivEqual);
                    self.advance(); //get the next token, else wrong tockens
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
                self.add_token(TokenTyp::UnderScore);
            }

            ' ' => {}
            '\r' => {}
            '\t' => {}

            ';' => {
                self.add_token(TokenTyp::SYMICOLON);
            }

            '"' => {
                self.string();
            }

            ':' => {
                if self.peek_next() == ':' {
                    self.add_token(TokenTyp::ColonColon);
                    self.advance(); //get the next token, else wrong tockens
                }
                
                else {
                    self.add_token(TokenTyp::COLON);
                }
            }

            _ => {
                if  c >= '0' && c <= '9'{
                    self.num();
                }
                else if  (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || c == '_' {
                    self.identifer();
                }
                else {
                    error::error("E0001", "unexpected character", self.file.as_str(), self.line_str.clone(), self.line, self.pos_in_line as usize, 1);
                }
            }
        }
    } 

    fn string(&mut self) {
        let mut str = String::new();

        while self.advance() != '"' && !self.is_at_end() {
            let ad = self.peek();
            str.push(ad);
        }

        if self.is_at_end() {
            error::error("E0002", "undetermend string", self.file.as_str(), self.line_str.clone(), self.line, self.pos_in_line as usize, 1);
        }

        self.advance(); // skip the closing "

        self.add_token_l(TokenTyp::STRING, str);
    }

    fn identifer(&mut self) {
        let mut str = String::new();

        let mut ad: char = self.peek();

        while (ad >= 'A' && ad <= 'Z') || (ad >= 'a' && ad <= 'z') || ad == '_' || (ad >= '0' && ad <= '9') {
            str.push(ad);
            ad = self.advance();
        }

        self.add_token_l(TokenTyp::IDENTIFIER, str);
    }

    fn num(&mut self) {
        let mut str = String::new();

        let mut ad: char = self.peek();

        println!("number. start char = {}", ad);

        while (ad >= '0' && ad <= '9') || ad == '.' || ad == '_' || ad == ',' {
            str.push(ad);
            ad = self.advance();
        }

        str = str.replace("_", "");

        self.add_token_l(TokenTyp::NUMBER, str);

    }

    pub fn scan(&mut self) {
        self.advance();
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