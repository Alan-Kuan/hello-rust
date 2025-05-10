use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

enum QuoteState {
    None,
    InsideSingleQuote,
    InsideDoubleQuote,
}

enum IORedirectState {
    None,
    Stdin,
    Stdout,
}

macro_rules! add_arg {
    ($args: ident, $arg: expr) => {
        {
            if !$arg.is_empty() {
                $args.push($arg.clone());
                $arg.clear();
            }
        }
    };
}

/// # Arguments
///
/// - `$files`: `Vec<String>`
/// - `$path`: `String`
/// - `$io_redir_state?`: if given, $path can be empty and $io_redir_state
///   is updated to `IORedirectState::None` if $path is added
macro_rules! add_file {
    ($files: ident, $path: expr) => {
        {
            if $path.is_empty() {
                eprintln!("shell: no file path provided");
                // NOTE: return from `parse_cmd_line`
                return false;
            }
            $files.push($path.clone());
            $path.clear();
        }
    };
    ($files: ident, $path: expr, $io_redir_state: ident) => {
        {
            if !$path.is_empty() {
                $files.push($path.clone());
                $path.clear();
                $io_redir_state = IORedirectState::None;
            }
        }
    };
}

/// # Returns
///
/// whether to exit
pub fn parse_cmd_line(cmd_line: &str) -> bool {
    let mut args: Vec<String> = vec![];
    let mut files_in: Vec<String> = vec![];
    let mut files_out: Vec<String> = vec![];

    let mut arg = String::new();
    let mut quote_state = QuoteState::None;
    let mut io_redir_state = IORedirectState::None;

    for ch in cmd_line.chars() {
        match ch {
            '\'' => match quote_state {
                QuoteState::None => quote_state = QuoteState::InsideSingleQuote,
                QuoteState::InsideSingleQuote => quote_state = QuoteState::None,
                _ => arg.push(ch),
            },
            '"' => match quote_state {
                QuoteState::None => quote_state = QuoteState::InsideDoubleQuote,
                QuoteState::InsideDoubleQuote => quote_state = QuoteState::None,
                _ => arg.push(ch),
            },
            '>' => match quote_state {
                QuoteState::None => match io_redir_state {
                    IORedirectState::None => io_redir_state = IORedirectState::Stdout,
                    IORedirectState::Stdin => {
                        add_file!(files_in, arg);
                        io_redir_state = IORedirectState::Stdout;
                    },
                    IORedirectState::Stdout => add_file!(files_out, arg),
                }
                _ => arg.push(ch),
            },
            '<' => match quote_state {
                QuoteState::None => match io_redir_state {
                    IORedirectState::None => io_redir_state = IORedirectState::Stdin,
                    IORedirectState::Stdin => add_file!(files_in, arg),
                    IORedirectState::Stdout => {
                        add_file!(files_out, arg);
                        io_redir_state = IORedirectState::Stdin;
                    },
                },
                _ => arg.push(ch),
            },
            ' ' => match quote_state {
                QuoteState::None => match io_redir_state {
                    IORedirectState::None => add_arg!(args, arg),
                    IORedirectState::Stdin => add_file!(files_in, arg, io_redir_state),
                    IORedirectState::Stdout => add_file!(files_out, arg, io_redir_state),
                },
                _ => arg.push(ch),
            },
            '\n' => match quote_state {
                QuoteState::None => match io_redir_state {
                    IORedirectState::None => add_arg!(args, arg),
                    IORedirectState::Stdin => add_file!(files_in, arg),
                    IORedirectState::Stdout => add_file!(files_out, arg),
                },
                _ => {
                    eprintln!("shell: unclosed quotes");
                    return false;
                }
            },
            _ => arg.push(ch),
        }
    }

    if let Some(cmd) = args.first() {
        let cmd_clone = cmd.clone();
        let res = match cmd.as_str() {
            "" => return false,
            "exit" => return true,
            "echo" => echo(args, files_out),
            "cd" => cd(args),
            "pwd" => pwd(args, files_out),
            _ => exec(args, files_in, files_out),
        };
        match res {
            Err(err) => eprintln!("{cmd_clone}: {err}"),
            Ok(_) => (),
        }
    }

    false
}

fn echo(args: Vec<String>, files_out: Vec<String>) -> io::Result<()> {
    if files_out.is_empty() {
        for arg in &args[1..] {
            print!("{arg} ");
        }
        println!();
    } else {
        for path in files_out {
            let mut f = File::create(path)?;
            for arg in &args[1..] {
                write!(f, "{arg} ")?;
            }
            writeln!(f)?;
        }
    }
    Ok(())
}

fn cd(args: Vec<String>) -> io::Result<()> {
    let path;

    match args.len() {
        #![allow(deprecated)]
        1 => path = env::home_dir().unwrap(),
        2 => path = PathBuf::from(&args[1]),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "too many arguments")),
    }
    env::set_current_dir(&path)?;
    Ok(())
}

fn pwd(args: Vec<String>, files_out: Vec<String>) -> io::Result<()> {
    if args.len() > 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "too many arguments"));
    }
    let pwd = env::current_dir()?;

    if files_out.is_empty() {
        println!("{}", pwd.display());
        return Ok(());
    }
    for path in files_out {
        let mut f = File::create(path)?;
        writeln!(f, "{}", pwd.display())?;
    }
    Ok(())
}

fn exec(args: Vec<String>, files_in: Vec<String>, files_out: Vec<String>) -> io::Result<()> {
    let mut comm = Command::new(&args[0]);
    comm.args(&args[1..]);

    match files_in.len() {
        0 => (),
        1 => {
            let input = File::open(&files_in[0])?;
            comm.stdin(input);
        },
        _ => {
            comm.stdin(Stdio::piped());
        },
    }
    match files_out.len() {
        0 => (),
        1 => {
            let output = File::create(&files_out[0])?;
            comm.stdout(output);
        },
        _ => {
            comm.stdout(Stdio::piped());
        },
    }

    // execute the command and return early if an error occurs
    let child = comm.spawn();
    if let Err(err) = child {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err);
        }
        eprintln!("shell: no such command: {}", &args[0]);
        return Ok(());
    }
    let mut child = child.unwrap();

    // set multiple files as input sources
    if files_in.len() > 1 {
        if let Some(mut stdin) = child.stdin.take() {
            for path in files_in {
                let f = File::open(path)?;
                let reader = BufReader::new(f);

                for line in reader.lines() {
                    let line = line?;
                    writeln!(stdin, "{line}")?;
                }
            }
        }
    }
    // set multiple files as output destinations
    if files_out.len() > 1 {
        let mut f_list: Vec<File> = vec![];
        for path in files_out {
            f_list.push(File::create(path)?);
        }

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                let line = line?;
                for f in &mut f_list {
                    writeln!(f, "{line}")?;
                }
            }
        }
    }

    match child.wait() {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
