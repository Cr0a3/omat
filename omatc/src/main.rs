mod args;
mod ast;

fn main() {
    let args: args::Args = args::Args::new();

    let code: String;
    code = String::from("Hello World");

    let mut scanner = ast::scanner::Scanner::new(&code, &args.input);
    scanner.scan();

    let tokens  = scanner.get_tokens();

    let mut parser: ast::parser::Parser = ast::parser::Parser::new(&tokens);

    parser.parse();

    let exprs: Vec<ast::expr::Expr> = parser.get_exprs();

    let mut code_gen = ast::code::CodeGenerator::new(&exprs, &args);
    code_gen.gen();

}