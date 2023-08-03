mod parser;
mod runner;

fn main() {
    let input = "3 + 4 - (2 + 1) * 4  / 3";

    match parser::parse_expr(input) {
        Ok((_, expr)) => {
            let output = runner::run_expr(expr);
            println!("programm-output: {}", output);
            println!("Real-result: {}", 3 + 4 - (2 + 1) * 4 / 3);

        },

        Err(e) => println!("Error: {:?}", e),
    }
}
