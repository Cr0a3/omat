mod args;
mod ast;

fn main() {
    let args: args::Args = args::Args::new();

    let code: String;
    code = String::from("Hello World");

    let scanner: ast::scanner::Scanner = ast::scanner::Scanner::new(code);
    scanner.scan();

    let tokens: Vec<ast::token::TokenTyp> = scanner.get_tokens();

    let parser: ast::parser::Parser = ast::parser::Parser::new(tokens);
    parser.parse();

    let exprs: Vec<ast::expr::Expr> = parser.get_exprs();

    let code_gen: ast::code::CodeGenerator = ast::code::CodeGenerator::new(exprs, args);
    code_gen.gen();

}