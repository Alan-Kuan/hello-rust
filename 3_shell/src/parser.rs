// @return: whether to exit
pub fn parse_cmd_line(cmd_line: &str) -> bool {
    let args: Vec<&str> = cmd_line.trim().split(' ').collect();

    match args[0] {
        "exit" => return true,
        "echo" => echo(args),
        "cd" => cd(args),
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
    // TODO
}

fn exec(args: Vec<&str>) {
    // TODO
}
