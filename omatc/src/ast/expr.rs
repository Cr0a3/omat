pub enum ExprTyp {
    EOF,
}

pub struct Expr {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    typ: ExprTyp,
}

impl Expr {
    pub fn new(lhs: Expr, rhs: Expr, typ: ExprTyp) -> Self {
        Expr {
            lhs: Box::from(lhs),
            rhs: Box::from(rhs),
            typ: typ,
        }
    }

    pub fn get_type(self) -> ExprTyp {
        self.typ
    }

    pub fn get_lhs(self) -> Box<Expr> {
        self.lhs
    }

    pub fn get_rhs(self) -> Box<Expr> {
        self.rhs
    }
}