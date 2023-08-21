mod args;
mod ast;
pub mod error;
pub use error::error::*;

use std::fs::File;
use std::io::Read;

fn main() {
    let args: args::Args = args::Args::new();

    let mut code: String = String::new();
    
    let file = File::open(args.input.clone());
    match file {
        Ok(mut f) => {
            let result = f.read_to_string(&mut code);
            match result {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("error while reding: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("could not open file: {}", e);
            std::process::exit(1);
        }
    }

    let mut scanner = ast::scanner::Scanner::new(code, args.input.clone());
    scanner.scan();

    let mut tokens  = scanner.get_tokens();

    let mut parser: ast::parser::Parser = ast::parser::Parser::new(&mut tokens);
    parser.parse();

    let exprs = parser.get_exprs();

    let mut ir = ast::ir::ir::new(exprs);
    ir.gen();

    let mut code_gen = ast::code::CodeGenerator::new(ir, args);
    code_gen.gen();
}