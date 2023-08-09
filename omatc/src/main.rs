mod args;
mod ast;

fn main() {
    let args: args::Args = args::Args::new();

    let code: String = String::from(args.input);

    let scanner: ast::scanner::Scanner = ast::scanner::Scanner::new();

    scanner.scan();

    let parser: ast::parser::Parser = ast::parser::Parser::new(scanner.get_tokens());

    parser.parse();

    let code_gen: ast::code::CodeGenerator = ast::code::CodeGenerator::new(parser.get_exprs(), args);

    code_gen.gen();

}