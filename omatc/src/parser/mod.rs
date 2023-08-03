pub extern crate nom;

pub use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map_res, recognize, opt}, // Import für opt
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};

#[derive(Debug)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),

    Number(i32),
}

impl Expression {
    pub fn evaluate(&self) -> i32 {
        match self {
            Expression::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expression::Sub(lhs, rhs) => lhs.evaluate() - rhs.evaluate(),
            Expression::Mul(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            Expression::Div(lhs, rhs) => lhs.evaluate() / rhs.evaluate(),
            Expression::Number(num) => *num,
        }
    }
}

pub fn parse_number(input: &str) -> IResult<&str, Expression> {
    map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        s.parse::<i32>().map(Expression::Number)
    })(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expression> {
    alt((parse_number, delimited(char('('), parse_expr, char(')'))))(input)
}

fn parse_term(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_factor(input)?;
    let (input, exprs) = many0(pair(alt((char('*'), char('/'))), parse_factor))(input)?;

    let mut result = init;
    for (op, val) in exprs {
        match op {
            '*' => result = Expression::Mul(Box::new(result), Box::new(val)),
            '/' => result = Expression::Div(Box::new(result), Box::new(val)),
            _ => unreachable!(),
        }
    }

    Ok((input, result))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_term(input)?;
    let (input, exprs) = many0(pair(alt((char('+'), char('-'))), parse_term))(input)?;

    let mut result = init;
    for (op, val) in exprs {
        match op {
            '+' => result = Expression::Add(Box::new(result), Box::new(val)),
            '-' => result = Expression::Sub(Box::new(result), Box::new(val)),
            _ => unreachable!(),
        }
    }

    Ok((input, result))
}