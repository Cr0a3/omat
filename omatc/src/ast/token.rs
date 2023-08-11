#[derive(Clone, Copy)]
pub enum TokenTyp {
    EOF, // End of file
    IDENTIFIER,

    //Math
    NUMBER,
    ADD, MIN, MUL, DIV,
    AddAdd, MinMin,

    //vars
    LET, CONST,
    IMUT,
    EQUAL,
    AddEqual, MinEqual, MulEqual, DivEqual,
    COLON,

    //functions
    FN,
    LeftBracet, RightBracet,
    LeftParam, RightParam,
    RetType,
    Return,

}

impl TokenTyp {
    pub fn _str(&self) -> String {
        let str: &str = match *self {
            Self::EOF => "eof",
            Self::NUMBER => "number",
            Self::ADD => "",
            Self::MIN => "",
            Self::MUL => "",
            Self::DIV => "",
            Self::AddAdd => "",
            Self::MinMin => "",
            Self::LET => "",
            Self::CONST => "",
            Self::AddEqual => "",
            Self::MinEqual => "",
            Self::DivEqual => "",
            Self::COLON => "",
            Self::FN => "",
            Self::LeftBracet => "",
            Self::RightBracet => "",
            Self::LeftParam => "",
            Self::RightParam => "",
            Self::RetType => "",
            Self::Return => "",
             _ => "",
        };

        format!("{}", str)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenTyp,
    pub identifier: String,
    pub line: i32,
    pub file: String,
}

impl Token {
    pub fn new(_token_type: TokenTyp, line: i32, _ident: String, _file: String) -> Self {
        Token { 
            token_type: _token_type,
            identifier: _ident.clone(),
            line: line.clone(),
            file: _file.clone() }
    }

    pub fn print(&self) {
        println!("{}:{} token {}:{}", self.file, self.line, self.token_type._str(), self.identifier);
    }
}