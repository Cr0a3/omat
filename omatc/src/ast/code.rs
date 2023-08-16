use crate::ast::ir::ir;
use crate::args;

use libtcc::*;
use std::ffi::CString;
use std::path::Path;
use std::process::Output;

const OUT_FILE_STDTYPE: FileType = FileType::Object;

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
    c_programm: CString,
}

impl CodeGenerator {
    pub fn new(_ir: ir, _args: args::Args) -> Self {
        let c_programm = CString::new(ir.str().as_bytes()).unwrap();

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
        let mut output_type = OutputType::Exe;
        if self.obj {
            output_type = OutputType::Obj
        }

        let compile_ret = ctx
            .set_output_type(output_type)
            .set_call_back(|msg| err_warn = Some(String::from(msg.to_str().unwrap())))
            .compile_string(&self.c_programm);
    }
}