use colored::Colorize;

pub fn error(ecode: &str, _msg: &str, file: &str, line: String, line_no: usize, where_start: usize, where_length: usize) {
    let err: String = format!("error[{}]", ecode);
    let arrow: String = format!("-->{}:{}:{}",file, line_no, where_start);
    let where_str: String = format!("{}{}", " ".repeat(where_start), "^".repeat(where_length));

    println!("{}: {}", err.red(), _msg.bold());
    println!("  {}", arrow.truecolor(75, 75, 75).bold());
    println!("{} | {}", line_no, line);
    println!("    {}", where_str);
}