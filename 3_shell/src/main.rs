use std::io;
use std::io::Write;

use crate::executor::exec_cmds;
use crate::parser::parse;

pub mod executor;
pub mod parser;
pub mod types;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("shell: failed to flush");

        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line)
            .expect("shell: failed to read line");

        if bytes_read == 0 {
            break;
        }

        match parse(&line) {
            Ok(cmds) => {
                if cmds.is_empty() {
                    continue;
                }
                if exec_cmds(cmds) {
                    break;
                }
            },
            Err(err) => eprintln!("shell: {err}"),
        }
    }
}
