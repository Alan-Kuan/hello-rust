use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::str::Split;

/// # Returns
///
/// whether to exit
pub fn parse_cmd_line(cmd_line: &str) -> bool {
    let mut args = cmd_line.trim().split(' ');
    let cmd = args.next().unwrap();

    match cmd {
        "" => (),
        "exit" => return true,
        "echo" => echo(args),
        "cd" => cd(args),
        "pwd" => pwd(args),
        _ => exec(cmd, args),
    }
    false
}

fn echo(args: Split<'_, char>) {
    for arg in args {
        if arg.is_empty() {
            continue;
        }
        print!("{} ", arg);
    }
    println!();
}

fn cd(args: Split<'_, char>) {
    let args: Vec<&str> = args.collect();
    let path;

    match args.len() {
        #![allow(deprecated)]
        0 => path = env::home_dir().unwrap(),
        1 => path = PathBuf::from(args[0]),
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

fn pwd(args: Split<'_, char>) {
    if args.count() > 1 {
        eprintln!("pwd: too many arguments");
        return;
    }
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(_) => eprintln!("pwd: failed to get current directory"),
    }
}

fn exec(cmd: &str, args: Split<'_, char>) {
    match Command::new(cmd).args(args).status() {
        Ok(_) => (),
        Err(err) => eprintln!("exec: {}", err),
    }
}
