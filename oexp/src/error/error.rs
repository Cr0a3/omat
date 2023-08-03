use colored::Colorize;

pub fn error(e_code: &str, msg: &str) {
    let error_str:String = format!("error[{}]", e_code);
    println!("{}: {}", error_str.red().bold(), msg);
}