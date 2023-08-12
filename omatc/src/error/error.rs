use colored::Colorize;

pub struct error_fab {
    ecode: String,
    msg: String,
    fmt_lines: Vec<String>,
}

impl error_fab {
    pub fn new(_ecode: String, _msg: String) -> Self {
        error_fab {  
            ecode: _ecode,
            msg: _msg,
            fmt_lines: Vec::new(),
        }
    }

    pub fn add_code_line(&self, line: String, display_line_no: bool, line_no: usize, add: bool) {

    }

    pub fn add_where(&self, line_no: usize, where_start: usize, where_length: usize, where_msg_b: bool, where_msg: String) {

    }

    pub fn add_arrow(&self, file: String, line: usize, where_start: usize) {

    }

    pub fn print(&self) {
        let fmt_error = format!("error[{}]", self.ecode).red();
        println!("{}: {}", fmt_error, self.msg);
        
        //print out all elements of self.fmt_lines
        for line in self.fmt_lines.iter() {
            println!("{}", line);
        }
    } 
}

pub fn error(ecode: &str, _msg: &str, file: &str, line: String, line_no: usize, where_start: usize, where_length: usize) {
    let err: String = format!("error[{}]", ecode);
    let arrow: String = format!("-->{}:{}:{}",file, line_no, where_start);
    let where_str: String = format!("{}{}", " ".repeat(where_start), "^".repeat(where_length));

    println!("{}: {}", err.red(), _msg.bold());
    println!("  {}", arrow.truecolor(75, 75, 75).bold());
    println!("{} | {}", line_no, line);
    println!("    {}", where_str);
}