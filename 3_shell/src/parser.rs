use std::fs::File;

use crate::types::error::GenericError;
use crate::types::command::Command;

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
    ($args: ident, $arg: expr) => {{
        if !$arg.is_empty() {
            $args.push($arg);
            // reset the argument buffer
            $arg = String::new();
        }
    }};
}

/// # Arguments
///
/// - `$path`: `String`
/// - `$create`: `bool`
macro_rules! create_or_open {
    ($path: expr, true) => { File::create($path)? };
    ($path: expr, false) => { File::open($path)? };
}

/// # Arguments
///
/// - `$files`: `Vec<String>`
/// - `$path`: `String`
/// - `$create`: `bool`
/// - `$io_redir_state?`: if given, $path can be empty and $io_redir_state
///   is updated to `IORedirectState::None` if $path is added
macro_rules! add_file {
    ($files: ident, $path: expr, $create: tt) => {{
        if $path.is_empty() {
            // NOTE: return from `parse`
            return Err("no file path provided".into());
        }
        $files.push(create_or_open!($path, $create));
        // reset the argument buffer
        $path = String::new()
    }};
    ($files: ident, $path: expr, $create: tt, $io_redir_state: ident) => {{
        if !$path.is_empty() {
            $files.push(create_or_open!($path, $create));
            // reset the argument buffer
            $path = String::new();
            $io_redir_state = IORedirectState::None;
        }
    }};
}

/// # Arguments
///
/// - `$cmds`: `Vec<Command>`
/// - `$args`: `Vec<String>`
/// - `$files_in`: `Vec<String>`
/// - `$files_out`: `Vec<String>`
macro_rules! add_cmd {
    ($cmds: ident, $args: ident, $files_in: ident, $files_out: ident) => {
        $cmds.push(Command {
            args: $args,
            files_in: $files_in,
            files_out: $files_out,
        });
        // reset the buffers
        $args = vec![];
        $files_in = vec![];
        $files_out = vec![];
    };
}

/// Returns a vector of `Command`s or a `String` on error
pub fn parse(cmd_line: &str) -> Result<Vec<Command>, GenericError> {
    let mut cmds = vec![];
    let mut args = vec![];
    let mut files_in = vec![];
    let mut files_out = vec![];

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
                    IORedirectState::None => {
                        add_arg!(args, arg);
                        io_redir_state = IORedirectState::Stdout;
                    },
                    IORedirectState::Stdin => {
                        add_file!(files_in, arg, false);
                        io_redir_state = IORedirectState::Stdout;
                    },
                    IORedirectState::Stdout => add_file!(files_out, arg, true),
                }
                _ => arg.push(ch),
            },
            '<' => match quote_state {
                QuoteState::None => match io_redir_state {
                    IORedirectState::None => {
                        add_arg!(args, arg);
                        io_redir_state = IORedirectState::Stdin;
                    },
                    IORedirectState::Stdin => add_file!(files_in, arg, false),
                    IORedirectState::Stdout => {
                        add_file!(files_out, arg, true);
                        io_redir_state = IORedirectState::Stdin;
                    },
                },
                _ => arg.push(ch),
            },
            '|' => match quote_state {
                QuoteState::None => {
                    match io_redir_state {
                        IORedirectState::None => add_arg!(args, arg),
                        IORedirectState::Stdin => add_file!(files_in, arg, false),
                        IORedirectState::Stdout => add_file!(files_out, arg, true),
                    }
                    io_redir_state = IORedirectState::None;
                    let cmd_is_empty = args.is_empty() && files_in.is_empty() && files_out.is_empty();
                    if cmd_is_empty {
                        return Err("no command is provided before the pipe".into());
                    }
                    add_cmd!(cmds, args, files_in, files_out);
                },
                _ => arg.push(ch),
            },
            ' ' => match quote_state {
                QuoteState::None => match io_redir_state {
                    IORedirectState::None => add_arg!(args, arg),
                    IORedirectState::Stdin => add_file!(files_in, arg, false, io_redir_state),
                    IORedirectState::Stdout => add_file!(files_out, arg, true, io_redir_state),
                },
                _ => arg.push(ch),
            },
            '\n' => match quote_state {
                QuoteState::None => {
                    match io_redir_state {
                        IORedirectState::None => add_arg!(args, arg),
                        IORedirectState::Stdin => add_file!(files_in, arg, false),
                        IORedirectState::Stdout => add_file!(files_out, arg, true),
                    }
                    let cmd_is_empty = args.is_empty() && files_in.is_empty() && files_out.is_empty();
                    if !cmd_is_empty {
                        add_cmd!(cmds, args, files_in, files_out);
                    } else if !cmds.is_empty() {
                        return Err("no command is provided after the pipe".into());
                    }
                },
                _ => return Err("unclosed quotes".into()),
            },
            _ => arg.push(ch),
        }
    }

    Ok(cmds)
}
