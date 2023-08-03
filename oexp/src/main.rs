use std::env;
mod arg;
mod error;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() - 1 { // - 1 for the actual args
        1 => {
            let cmd: String = args[1].clone();

            //switch cmds
            match cmd.as_str() {
                "help" => {
                    arg::helper::help();
                }

                "explain" => {
                    error::error::error("Ee002", "explain command takes an argument");
                }

                _ => {
                    error::error::error("Ee001", "invalid command");
                }
            }
        }

        2 => {
            let cmd: String = args[1].clone();
            let opt: String = args[2].clone();

            match cmd.as_str() {
                "help" => {
                    arg::helper::help_cmd(opt.as_str());
                }

                "explain" => {
                    arg::arg_runner::explain(opt.as_str());
                }

                _ => {
                    error::error::error("Ee001", "invalid command");
                }
            }
        }
        _ => {
            arg::helper::help();
        }
    }
}