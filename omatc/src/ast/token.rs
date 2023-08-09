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

pub struct Token {
    token_type: TokenTyp,
    identifier: String,
    line: i32,
    file: String,
}