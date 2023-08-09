mod args;
mod ast;

fn main() {
    let args: args::Args = args::Args::new();

    let code: String = String::from(args.input);

    let scanner: ast::scanner::Scanner = ast::scanner::Scanner::new();

    scanner.scan();

    println!("Code: == \n{}\n==", code.clone());
    println!("Output-File: {}", args.output);
    println!("Release: {}", args.release);
    println!("Debug: {}", args.debug);
    println!("No_main: {}", args.no_main);
    println!("BareMetal: {}", args.bare_metal);
    println!("Object: {}", args.obj);
    println!("StartCode: {}", args.start_code);
    println!("Dynamic linking: {}", args.dynamic_linking);
    println!("Static linking: {}", args.static_linking);

}