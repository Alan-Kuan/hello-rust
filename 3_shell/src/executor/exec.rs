use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::process::{Command, Stdio};

use crate::types::error::GenericError;

pub fn exec(args: &[String], files_in: Vec<File>, files_out: Vec<File>) -> Result<(), GenericError> {
    let mut comm = Command::new(&args[0]);
    comm.args(&args[1..]);

    if !files_in.is_empty() {
        comm.stdin(Stdio::piped());
    }
    if !files_out.is_empty() {
        comm.stdout(Stdio::piped());
    }

    // execute the command and return early if an error occurs
    let child = comm.spawn();
    if let Err(err) = child {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err.into());
        }
        eprintln!("shell: no such command: {}", &args[0]);
        return Ok(());
    }
    let mut child = child.unwrap();

    // set multiple files as input sources
    if files_in.len() > 0 {
        if let Some(mut stdin) = child.stdin.take() {
            for file in files_in {
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let line = line?;
                    writeln!(stdin, "{line}")?;
                }
            }
        }
    }
    // set multiple files as output destinations
    if files_out.len() > 0 {
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                let line = line?;
                for mut file in &files_out {
                    writeln!(file, "{line}")?;
                }
            }
        }
    }

    match child.wait() {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
