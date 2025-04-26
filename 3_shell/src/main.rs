use std::io;
use std::io::Write;

use crate::parser::parse_cmd_line;

pub mod parser;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush.");

        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line)
            .expect("Failed to read line.");

        if bytes_read == 0 || parse_cmd_line(&line) {
            break;
        }
    }
}
