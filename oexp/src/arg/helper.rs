use crate::error::error;

pub fn help() {
    println!("omat's error explainer (oexp)");
    print!("\n");
    println!("SYNTAX: oexp [CMD] [CMD OPTIONS]");
    print!("\n");
    println!("COMMANDS:");
    println!("help      Shows the help for an specific command");
    println!("explain   Explaines the specified omat error");
    print!("\n");
    println!("See 'oexp help <cmd>' for more information");
}

pub fn help_cmd(cmd: &str) {
    match cmd {
        "explain" => {
            println!("Explaines the specified error");
            print!("\n");
            println!("SYNTAX: oexp explain <error-code>");
        }

        "help" => {
            println!("Shows the help or the help of the specified command");
            print!("\n");
            println!("SNYTAX: oexp help");
            println!("        oexp help <cmd>");
        }

        _ => {
            error::error("Ee001", "invalid command");
        }
    }
}