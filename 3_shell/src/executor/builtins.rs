use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::types::error::GenericError;

pub fn echo(args: &[String], files_out: Vec<File>) -> Result<(), GenericError> {
    if files_out.is_empty() {
        for arg in &args[1..] {
            print!("{arg} ");
        }
        println!();
    } else {
        for mut file in files_out {
            for arg in &args[1..] {
                write!(file, "{arg} ")?;
            }
            writeln!(file)?;
        }
    }
    Ok(())
}

pub fn cd(args: &[String]) -> Result<(), GenericError> {
    let path;

    match args.len() {
        #![allow(deprecated)]
        1 => path = env::home_dir().unwrap(),
        2 => path = PathBuf::from(&args[1]),
        _ => return Err("too many arguments".to_string().into()),
    }
    env::set_current_dir(&path)?;
    Ok(())
}

pub fn pwd(args: &[String], files_out: Vec<File>) -> Result<(), GenericError> {
    if args.len() > 1 {
        return Err("too many arguments".to_string().into());
    }
    let pwd = env::current_dir()?;

    if files_out.is_empty() {
        println!("{}", pwd.display());
        return Ok(());
    }
    for mut file in files_out {
        writeln!(file, "{}", pwd.display())?;
    }
    Ok(())
}
