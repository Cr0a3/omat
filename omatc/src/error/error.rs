use colored::Colorize;

pub fn error(ecode: &str, _msg: &str, file: &str, line: String, line_no: usize, where_start: usize, where_length: usize) {
    let err: String = format!("{}{}{}", "error[".red(), ecode, "]".red());
    let arrow: String = format!("\n-->{}:{}:{}",file, line_no, where_start);
    let where_str: String = format!("{}{}", " ".repeat(where_start), "^".repeat(where_length));

    println!("{}: {}{}\n{}\n{}\n", err, _msg.bold(), arrow.truecolor(136, 136, 136), line, where_str);
}