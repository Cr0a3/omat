use crate::ast::token::Token;

pub enum Expr {
    Assing(Assing),
    Binary(Binary),
    Set(Set),
    Grouping(Grouping),
    Logical(Logical),
}

pub struct Assing {
    name: Token,
    _type: Token,
    value: Expr,
}

pub struct Binary {
    lhs: Expr,
    op: Token,
    rhs: Expr,
}

pub struct Set {
    object: Box<Expr>,
    name: Token,
    value: Box<Expr>,
}

pub struct Grouping {
    expr: Expr,
}

pub struct Logical {
    lhs: Expr,
    op: Token,
    rhs: Expr,
}