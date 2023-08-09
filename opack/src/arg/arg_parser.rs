use std::env;
use crate::arg::helper;
use crate::arg::arg_runner;
use crate::error::error;

pub fn parse() {
    let args: Vec<String> = env::args().collect();

    match args.len() - 1 { // - 1 for the actual args
        1 => {
            let cmd: String = args[1].clone();

            //switch cmds
            match cmd.as_str() {
                "help" => {
                    helper::help();
                }

                "build" => {
                    arg_runner::build("debug");
                }

                "clean" => {
                    arg_runner::clean("debug");
                }

                "new" => {
                    error::error("Et003", "expect an package name");
                }

                "run" => {
                    arg_runner::run("debug");
                }
                _ => {
                    error::error("Et001", "invalid command");
                }
            }
        }

        2 => {
            let cmd: String = args[1].clone();
            let opt: String = args[2].clone();

            match cmd.as_str() {
                "help" => {
                    helper::help_cmd(opt);
                }

                "build" => {
                    arg_runner::build(opt.as_str());
                }

                "clean" => {
                    arg_runner::clean(opt.as_str());
                }

                "new" => {
                    arg_runner::new(opt.as_str());
                }

                "run" => {
                    arg_runner::run(opt.as_str());
                }

                _ => {
                    error::error("Et001", "invalid command");
                }
            }
        }
        _ => {
            helper::help();
        }
    }
}