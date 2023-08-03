use crate::error;

pub fn help() {
    println!("omats's package manager (opack)");
    println!("");
    println!("Usage: opack [COMMAND] [CMDARG]");
    print!("\n");
    println!("Commands:");
    println!("build     Builds the current opack package");
    println!("clean     Cleans the builded binarys");
    println!("help      Shows info about command");
    println!("new       Create a new opack package");
    println!("run       Builds and runs the current package");
    println!("");
    println!("See 'opack help <cmd>' for more info");
}

pub fn help_cmd(cmd: String) {
    match cmd.as_str() {
        "build" => {
            println!("Builds the current package");
            print!("\n");
            println!("SNYTAX: opack build [OPTION]");
            print!("\n");
            println!("OPTIONS:");
            println!("debug     builds the package as debug version   (standart)");
            println!("release   builds the package as release version");
        }

        "clean" => {
            println!("Cleans the build of the current package");
            print!("\n");
            println!("SNYTAX: opack clean [OPTION]");
            print!("\n");
            println!("OPTIONS:");
            println!("debug     cleans the debug build   (standart)");
            println!("release   cleans the release build");
        }

        "help" => {
            println!("Shows the help or the help of the specified command");
            print!("\n");
            println!("SNYTAX: opack help");
            println!("        opack help <cmd>");
        }

        "new" => {
            println!("Creates an new package");
            print!("\n");
            println!("SNYTAX: opack new <package-name>");
        }

        "run" => {
            println!("Builds and runs the current package");
            print!("\n");
            println!("SNYTAX: opack run [OPTION]");
            print!("\n");
            println!("OPTIONS:");
            println!("debug     builds and runs the package as debug version   (standart)");
            println!("release   builds and runs the package as release version");
        }

        _ => {
            error::error::error("Et002", "unknow command");
        }
    }
}