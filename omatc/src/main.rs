mod args;
mod ast;
pub mod error;
pub use error::error::*;

fn main() {
    let args: args::Args = args::Args::new();

    let code: String;
    code = String::from("Hello World!");

    let mut scanner = ast::scanner::Scanner::new(code, args.input.clone());
    scanner.scan();

    let mut tokens  = scanner.get_tokens();

    let mut parser: ast::parser::Parser = ast::parser::Parser::new(&mut tokens);

    parser.parse();

    let exprs: Vec<ast::expr::Expr> = parser.get_exprs();

    let mut code_gen = ast::code::CodeGenerator::new(exprs, args);
    code_gen.gen();
}