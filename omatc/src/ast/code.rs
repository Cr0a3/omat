use crate::ast::expr::Expr;
use crate::args;

use inkwell::OptimizationLevel;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, RelocMode, FileType, Target, TargetMachine, TargetTriple, InitializationConfig};
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;

const OUT_FILE_STDTYPE: FileType = FileType::Object;

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

    //llvm
    target: Target,
    machine: TargetMachine,
    context: Rc<Context>,
}

impl CodeGenerator {
    pub fn new(_exprs: Vec<Expr>, _args: args::Args) -> Self {
        //init llvm
        Target::initialize_x86(&InitializationConfig::default());

        let target = Target::from_name("x86-64").unwrap();
        
        let opt = OptimizationLevel::Default;
        let reloc = RelocMode::Default;
        let model = CodeModel::Default;


        let target_machine = target.create_target_machine(
            &TargetTriple::create(&_args.target.clone()),
            "x86-64",
            "+avx2",
            opt,
            reloc,
            model
        )
        .unwrap();

        let context = Rc::new(Context::create());

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

            //llvm
            target: target,
            machine: target_machine,
            context: context,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current == self.exprs.len()
    }

    pub fn gen_expr(&mut self, module: &Module<'_>) {

    }

    pub fn gen(&mut self) {
        let context = self.context.clone();
        let module  = context.create_module("module");

        while !self.is_at_end() {
            self.start = self.current.clone();
            self.current += 1;
            
            self.gen_expr(&module);
        }
        let out_file_type: FileType;

        if self.obj {
            out_file_type = FileType::Object;
        }
        else {
            out_file_type = OUT_FILE_STDTYPE;
        }

        let result = self.machine.write_to_file(&module, out_file_type, Path::new(format!("{}.o", self.output).as_str()));

        match result {
            Ok(_) => {}

            Err(e) => {
                eprintln!("error while saving excutable: {}", e);
            }
        }
    }
}