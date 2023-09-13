use crate::ast::ir::Ir;
use crate::args;

pub struct CodeGenerator {
    // options from args struct
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

    // libtcc
    c_programm: String,
}

impl CodeGenerator {
    pub fn new(_ir: Ir, _args: args::Args) -> Self {
        let c_programm = _ir.get_c_code();

        CodeGenerator { 
            // options from args struct
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

            // libtcc
            c_programm: c_programm,
        }
    }

    pub fn gen(&mut self) {
    }
}