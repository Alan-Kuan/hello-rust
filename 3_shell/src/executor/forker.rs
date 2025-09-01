use nix::{errno::Errno, sys::wait::waitpid, unistd::{ForkResult, Pid}};

pub struct Forker {
    child_pids: Vec<Pid>,
}

impl Forker {
    pub fn new() -> Self {
        Self { child_pids: vec![] }
    }

    /// Returns if it is a child process
    pub fn fork(&mut self) -> Result<bool, Errno> {
        match unsafe {nix::unistd::fork()} {
            Ok(ForkResult::Parent { child, .. }) => {
                self.child_pids.push(child);
                return Ok(false);
            },
            Ok(ForkResult::Child) => return Ok(true),
            Err(err) => return Err(err),
        }
    }

    pub fn wait_all(&mut self) {
        while !self.child_pids.is_empty() {
            let pid = self.child_pids.pop();
            waitpid(pid, None).unwrap();
        }
    }
}
