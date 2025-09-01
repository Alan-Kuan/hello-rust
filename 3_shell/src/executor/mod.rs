use nix::unistd::pipe;
use std::{fs::File, os::fd::AsFd};

use crate::types::{command::Command, error::GenericError};
use crate::executor::redirect::{merge, spread};

mod builtins;
mod exec;
mod forker;
#[macro_use]
mod redirect;

/// Returns whether to exit
pub fn exec_cmds(cmds: Vec<Command>) -> Result<bool, GenericError> {
    let mut should_exit = false;
    let last_idx = cmds.len() - 1;
    let in_subshell = cmds.len() > 1;
    let mut file_in: Option<File> = None;
    let mut file_out: Option<File> = None;
    let mut file_in_next: Option<File> = None;

    let mut forker = forker::Forker::new();
    let mut err_res = None;

    for (i, cmd) in cmds.into_iter().enumerate() {
        // merge input files and the pipe's read end
        let mut files_in: Vec<File> = file_in
            .take()
            .into_iter()
            .chain(
                cmd.files_in
                    .iter()
                    .map(File::open)
                    .collect::<Result<Vec<_>, _>>()?
            )
            .collect();

        // If there're multiple input sources, merge them.
        if files_in.len() > 1 {
            match pipe() {
                Ok(fds) => {
                    match forker.fork() {
                        Ok(is_child) => {
                            if is_child {
                                if let Err(err) = merge(&files_in, fds.1) {
                                    eprintln!("shell: merge: failed with an error: {err}");
                                }
                                unsafe { libc::_exit(0); }
                            }
                            file_in = Some(File::from(fds.0));
                        },
                        Err(_) => {
                            err_res = Some("fork: failed to create the merger".into());
                            break;
                        },
                    }
                },
                Err(_) => {
                    err_res = Some("pipe: failed to create the pipe connecting from the merger".into());
                    break;
                },
            }
        } else {
            file_in = files_in.pop().take();
        }

        if i < last_idx {
            match pipe() {
                Ok(fds) => {
                    file_in_next = Some(File::from(fds.0));
                    file_out = Some(File::from(fds.1));
                },
                Err(_) => {
                    err_res = Some("pipe: failed to create the pipe connecting two commands".into());
                    break;
                },
            }
        }

        // merge output files and the pipe's write end
        let mut files_out: Vec<File> = file_out
            .take()
            .into_iter()
            .chain(
                cmd.files_out
                    .iter()
                    .map(File::create)
                    .collect::<Result<Vec<_>, _>>()?
            )
            .collect();

        // If there're multiple output destinations, spread to them.
        if files_out.len() > 1 {
            match pipe() {
                Ok(fds) => {
                    match forker.fork() {
                        Ok(is_child) => {
                            if is_child {
                                // The pipe's write end should be closed in the child process
                                // before spread() starts reading from the pipe's read end.
                                drop(fds.1);
                                if let Err(err) = spread(&mut files_out, fds.0) {
                                    eprintln!("shell: spread: failed with an error: {err}");
                                }
                                unsafe { libc::_exit(0); }
                            }
                            file_out = Some(File::from(fds.1));
                        },
                        Err(_) => {
                            err_res = Some("fork: failed to create the spreader".into());
                            break;
                        },
                    }
                },
                Err(_) => {
                    err_res = Some("pipe: failed to create the pipe connecting to the spreader".into());
                    break;
                }
            }
        } else {
            file_out = files_out.pop().take();
        }

        match exec_cmd(cmd, file_in.take(), file_out.take(), in_subshell, &mut forker) {
            Ok(exit_or_not) => should_exit |= exit_or_not,
            Err(err) => {
                err_res = Some(err);
                break;
            }
        }

        file_in = file_in_next.take();
    }

    forker.wait_all();

    if let Some(err) = err_res {
        return Err(err);
    }
    Ok(should_exit)
}

fn is_builtin(cmd_name: &str) -> bool {
    static BUILTIN_NAMES: [&str; 4] = ["exit", "echo", "cd", "pwd"];
    return BUILTIN_NAMES.contains(&cmd_name);
}

/// Returns whether to exit or a generic error
///
/// Note: when `exit` is executed in a pipeline, the shell won't terminate because `exit` is logically executed
/// in a in subshell.
fn exec_cmd(
    cmd: Command,
    fd_in: Option<File>,
    fd_out: Option<File>,
    in_subshell: bool,
    forker: &mut forker::Forker,
) -> Result<bool, GenericError> {
    let cmd_name = match cmd.args.first() {
        Some(v) => v,
        None => return Ok(false),
    };
    let mut should_exit = false;
    let should_fork = !is_builtin(&cmd_name) || in_subshell;
    let mut stdin_pre = None;
    let mut stdout_pre = None;

    if should_fork {
        match forker.fork() {
            Ok(is_child) => if !is_child { return Ok(false) },
            Err(_) => return Err("fork: failed to fork".into()),
        }
    }

    // replace stdin/stdout and preserve it if needed
    if let Some(fd) = fd_in {
        if !should_fork {
            preserve_fd!(stdin_pre, in);
        }
        redirect!(fd, in);
    }
    if let Some(fd) = fd_out {
        if !should_fork {
            preserve_fd!(stdout_pre, out);
        }
        redirect!(fd, out);
    }

    let res = match cmd_name.as_str() {
        "exit" => {
            should_exit = true;
            Ok(())
        },
        "echo" => builtins::echo(&cmd.args),
        "cd" => builtins::cd(&cmd.args),
        "pwd" => builtins::pwd(&cmd.args),
        _ => exec::exec(&cmd.args),
    };

    if should_fork {
        if let Some(err) = res.err() {
            eprintln!("shell: {cmd_name}: {err}");
        }
        // child process exits here
        unsafe { libc::_exit(0); }
    }

    // restore stdin/stdout
    if let Some(fd) = stdin_pre {
        restore_fd!(fd, in);
    }
    if let Some(fd) = stdout_pre {
        restore_fd!(fd, out);
    }

    if let Some(err) = res.err() {
        return Err(format!("{cmd_name}: {err}").into());
    }
    Ok(should_exit)
}
