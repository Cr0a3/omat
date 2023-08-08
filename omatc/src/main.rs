mod args;

fn main() {
    let args: args::Args = args::Args::new();

    let code = String::from(args.input.clone());

    println!("Code: == \n{}\n==", code.clone());
    println!("Output-File: {}", args.output);
    println!("Release: {}", args.release);
    println!("Debug: {}", args.debug);
    println!("No_main: {}", args.no_main);
    println!("BareMetal: {}", args.bare_metal);
    println!("Object: {}", args.obj);
    println!("StartCode: {}", args.start_code);
}