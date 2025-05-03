use std::env;
use std::io;
use std::path::PathBuf;
use std::process::Command;

enum ParsingState {
    Normal,
    InsideSingleQuote,
    InsideDoubleQuote,
}

/// # Returns
///
/// whether to exit
pub fn parse_cmd_line(cmd_line: &str) -> bool {
    let mut args: Vec<String> = vec![];
    let mut arg = String::new();
    let mut state = ParsingState::Normal;

    for ch in cmd_line.chars() {
        match ch {
            ' ' => {
                match state {
                    ParsingState::Normal => {
                        if !arg.is_empty() {
                            args.push(arg.clone());
                            arg.clear();
                        }
                    },
                    _ => arg.push(ch),
                }
            },
            '\'' => {
                match state {
                    ParsingState::Normal => state = ParsingState::InsideSingleQuote,
                    ParsingState::InsideSingleQuote => state = ParsingState::Normal,
                    _ => arg.push(ch),
                }
            },
            '"' => {
                match state {
                    ParsingState::Normal => state = ParsingState::InsideDoubleQuote,
                    ParsingState::InsideDoubleQuote => state = ParsingState::Normal,
                    _ => arg.push(ch),
                }
            },
            '\n' => {
                match state {
                    ParsingState::Normal => {
                        if !arg.is_empty() {
                            args.push(arg.clone());
                        }
                    },
                    _ => {
                        eprintln!("Unclosed quotes");
                        return false;
                    }
                }
            },
            _ => arg.push(ch),
        }
    }

    match args[0].as_str() {
        "" => (),
        "exit" => return true,
        "echo" => echo(args),
        "cd" => cd(args),
        "pwd" => pwd(args),
        _ => exec(args),
    }
    false
}

fn echo(args: Vec<String>) {
    for arg in &args[1..] {
        print!("{} ", arg);
    }
    println!();
}

fn cd(args: Vec<String>) {
    let path;

    match args.len() {
        #![allow(deprecated)]
        1 => path = env::home_dir().unwrap(),
        2 => path = PathBuf::from(&args[1]),
        _ => {
            eprintln!("cd: too many arguments");
            return;
        },
    }
    match env::set_current_dir(&path) {
        Ok(_) => (),
        Err(err) => eprintln!("cd: {}", err),
    }
}

fn pwd(args: Vec<String>) {
    if args.len() > 1 {
        eprintln!("pwd: too many arguments");
        return;
    }
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(_) => eprintln!("pwd: failed to get current directory"),
    }
}

fn exec(args: Vec<String>) {
    match Command::new(&args[0]).args(&args[1..]).status() {
        Ok(_) => (),
        Err(err) => {
            match err.kind() {
                io::ErrorKind::NotFound => eprintln!("No such command: {}", &args[0]),
                _ => eprintln!("exec: {}", err),
            }
        },
    }
}
