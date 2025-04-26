use std::env;
use std::path::PathBuf;

// @return: whether to exit
pub fn parse_cmd_line(cmd_line: &str) -> bool {
    let args: Vec<&str> = cmd_line.trim().split(' ').collect();

    match args[0] {
        "exit" => return true,
        "echo" => echo(args),
        "cd" => cd(args),
        "pwd" => pwd(args),
        _ => exec(args),
    }
    false
}

fn echo(args: Vec<&str>) {
    for arg in &args[1..] {
        if arg.is_empty() {
            continue;
        }
        print!("{} ", arg);
    }
    println!();
}

fn cd(args: Vec<&str>) {
    let path;

    match args.len() {
        1 => path = env::home_dir().unwrap(),
        2 => path = PathBuf::from(args[1]),
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

fn pwd(args: Vec<&str>) {
    if args.len() > 1 {
        println!("pwd: too many arguments");
        return;
    }
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(_) => println!("pwd: failed to get current directory"),
    }
}

fn exec(args: Vec<&str>) {
    // TODO
}
