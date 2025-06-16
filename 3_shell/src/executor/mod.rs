use nix::{sys::wait::waitpid, unistd::{fork, ForkResult, pipe}};
use std::os::fd::OwnedFd;

use crate::types::command::Command;

mod builtins;
mod exec;

/// Returns whether to exit
pub fn exec_cmds(cmds: Vec<Command>) -> bool {
    let mut should_exit = false;
    let last_cmd_idx = cmds.len() - 1;
    let in_subshell = cmds.len() > 1;
    let mut fd_in: Option<OwnedFd> = None;

    for (i, mut cmd) in cmds.into_iter().enumerate() {
        if i > 0 {
            cmd.files_in.push(fd_in.take().unwrap().into());
        }
        if i < last_cmd_idx {
            match pipe() {
                Ok(fds) => {
                    cmd.files_out.push(fds.1.into());
                    fd_in = Some(fds.0);
                },
                Err(_) => {
                    eprintln!("pipe: failed to create a pipe");
                    // FIXME: should make sure already executed commands
                    // finish correctly
                    return false;
                },
            }
        }
        should_exit |= exec_cmd(cmd, in_subshell);
    }

    should_exit
}

fn is_builtin(cmd_name: &str) -> bool {
    let builtin_names = ["", "exit", "echo", "cd", "pwd"];
    return builtin_names.contains(&cmd_name);
}

/// Returns whether to exit
///
/// Note: when `exit` is executed in a pipeline, the shell won't
/// terminate because `exit` is logically executed in a in subshell.
fn exec_cmd(cmd: Command, in_subshell: bool) -> bool {
    let mut should_exit = false;

    if let Some(cmd_name) = cmd.args.first() {
        let should_fork = is_builtin(&cmd_name) && in_subshell;
        let mut is_child = false;

        if should_fork {
            match unsafe {fork()} {
                Ok(ForkResult::Parent { child, .. }) => {
                    // TODO: wait after all sub processes are created
                    waitpid(child, None).unwrap();
                    return false;
                },
                Ok(ForkResult::Child) => is_child = true,
                Err(_) => {
                    eprintln!("fork: failed to fork");
                    return false;
                }
            }
        }

        let res = match cmd_name.as_str() {
            "" => unreachable!(),
            "exit" => {
                should_exit = true;
                Ok(())
            },
            "echo" => builtins::echo(&cmd.args, cmd.files_out),
            "cd" => builtins::cd(&cmd.args),
            "pwd" => builtins::pwd(&cmd.args, cmd.files_out),
            _ => exec::exec(&cmd.args, cmd.files_in, cmd.files_out),
        };
        if let Some(err) = res.err() {
            eprintln!("{cmd_name}: {err}");
        }

        if is_child {
            unsafe { libc::_exit(0); }
        }
    }

    should_exit
}
