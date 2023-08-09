use crate::ast::expr::Expr;
use crate::args;

pub struct CodeGenerator {
    exprs: Vec<Expr>,

    //options from args struct
    pub output: String,
    pub release: bool,
    pub debug: bool,
    pub no_main: bool,
    pub bare_metal: bool,
    pub obj: bool,
    pub start_code: String,
    pub static_linking: bool,
    pub dynamic_linking: bool,
}

impl CodeGenerator {
    pub fn new(_exprs: Vec<Expr>, _args: args::Args) -> Self {
        CodeGenerator { 
            exprs: _exprs, 
            output: _args.output, 
            release: _args.release, 
            debug: _args.debug, 
            no_main: _args.no_main, 
            bare_metal: _args.bare_metal, 
            obj: _args.obj, 
            start_code: _args.start_code, 
            static_linking: _args.static_linking, 
            dynamic_linking: _args.dynamic_linking 
        }
    }

    pub fn gen(&self) {

    }
}