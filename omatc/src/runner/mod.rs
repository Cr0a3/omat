use crate::parser::Expression;

pub fn run_expr(expr: Expression) -> String {
    let result = expr.evaluate();

    return format!("{}", result); //for type conversation
}