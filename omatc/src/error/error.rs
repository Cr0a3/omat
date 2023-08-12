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

    pub fn add_code_line(&mut self, line: String, display_line_no: bool, line_no: usize, display_add: bool) {
        let mut code_line = String::new();

        if display_line_no {
            code_line += line_no.to_string().as_str();
        }

        else if display_add {
            code_line += "+++";
        }

        code_line += " | ";
        code_line += line.as_str();

        self.fmt_lines.push(code_line);
    }

    pub fn add_where(&mut self, line_no: usize, where_start: usize, where_length: usize, where_msg_b: bool, where_msg: String) {
        let where_str = format!("");
        self.fmt_lines.push(where_str);
    }

    pub fn add_arrow(&mut self, file: String, line: usize, where_start: usize) {
        let arrow = format!("");
        self.fmt_lines.push(arrow);
    }

    pub fn print(&self) {
        let fmt_error = format!("error[{}]", self.ecode).red();
        println!("{}: {}", fmt_error, self.msg.bold());
        
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