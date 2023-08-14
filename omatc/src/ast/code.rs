use crate::ast::expr::Expr;
use crate::args;

pub struct CodeGenerator {
    exprs: Vec<Expr>,

    //options from args struct
    output: String,
    target_machine: String,

    release: bool,
    debug: bool,
    no_main: bool,
    bare_metal: bool,
    obj: bool,
    start_code: String,
    static_linking: bool,
    dynamic_linking: bool,

    current: usize,
    start: usize,
}

impl CodeGenerator {
    pub fn new(_exprs: Vec<Expr>, _args: args::Args) -> Self {
        CodeGenerator { 
            exprs: _exprs, 
            output: _args.output, 
            target_machine: _args.target,

            release: _args.release, 
            debug: _args.debug, 
            no_main: _args.no_main, 
            bare_metal: _args.bare_metal, 
            obj: _args.obj, 
            start_code: _args.start_code, 
            static_linking: _args.static_linking, 
            dynamic_linking: _args.dynamic_linking, 
            current: 0,
            start: 0,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current == self.exprs.len()
    }

    pub fn gen_expr(&mut self) {

    }

    pub fn gen(&mut self) {
        while !self.is_at_end() {
            self.start = self.current.clone();
            self.current += 1;
            
            self.gen_expr();
        }
    }
}