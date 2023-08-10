use crate::ast::token::Token;

pub enum Expr {
    Assing(Assing),
    Binary(Binary),
    Set(Set),
    Grouping(Grouping),
    Logical(Logical),
}

pub struct Assing {
    name: Box<Token>,
    _type: Box<Token>,
    value: Box<Expr>,
}

pub struct Binary {
    lhs: Box<Expr>,
    op: Box<Token>,
    rhs: Box<Expr>,
}

pub struct Set {
    object: Box<Expr>,
    name: Box<Token>,
    value: Box<Expr>,
}

pub struct Grouping {
    expr: Box<Expr>,
}

pub struct Logical {
    lhs: Box<Expr>,
    op: Box<Token>,
    rhs: Box<Expr>,
}