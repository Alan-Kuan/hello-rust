use std::env;
use std::path::PathBuf;

use crate::types::error::GenericError;

pub fn echo(args: &Vec<String>) -> Result<(), GenericError> {
    println!("{}", args[1..].join(" "));
    Ok(())
}

pub fn cd(args: &Vec<String>) -> Result<(), GenericError> {
    let path;

    match args.len() {
        #![allow(deprecated)]
        1 => path = env::home_dir().unwrap(),
        2 => path = PathBuf::from(&args[1]),
        _ => return Err("too many arguments".into()),
    }
    env::set_current_dir(&path)?;
    Ok(())
}

pub fn pwd(args: &Vec<String>) -> Result<(), GenericError> {
    if args.len() > 1 {
        return Err("too many arguments".into());
    }
    println!("{}", env::current_dir()?.display());
    Ok(())
}
