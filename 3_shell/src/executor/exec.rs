use nix::{unistd::execvp, errno::Errno};
use std::ffi::CString;

use crate::types::error::GenericError;

pub fn exec(args: &Vec<String>) -> Result<(), GenericError> {
    let filename = CString::new(args[0].as_bytes()).unwrap();
    let cargs: Vec<CString> = args.iter()
        .map(|arg| CString::new(arg.as_bytes()).unwrap())
        .collect();

    match execvp(&filename, &cargs) {
        Err(err) => match err {
            Errno::ENOENT => return Err("command not found!".to_string().into()),
            _ => return Err(err.desc().to_string().into()),
        },
        Ok(_) => unreachable!(),
    }
}
