use std::{fs::File, io::{BufReader, Read, Write}, os::fd::OwnedFd};

macro_rules! preserve_fd {
    ($fd_pre: ident, $dir: ident) => {
        paste::paste! {
            match nix::unistd::dup(
                <std::io::[<Std $dir>] as std::os::unix::io::AsFd>::as_fd(&std::io::[<std $dir>]())
            ) {
                Ok(fd) => $fd_pre = Some(fd),
                Err(_) => return Err(concat!("dup: failed to duplicate std", stringify!($dir)).into()),
            }
        }
    }
}

macro_rules! restore_fd {
    ($fd_pre: ident, $dir: ident) => {
        paste::paste! {
            if let Err(_) = nix::unistd::[<dup2_std $dir>]($fd_pre) {
                return Err(concat!("dup2_std", stringify!($dir), ": failed to restore std", stringify!($dir)).into());
            }
        }
    };
}

macro_rules! redirect {
    ($file: ident, $dir: ident) => {
        paste::paste! {
            if let Err(_) = nix::unistd::[<dup2_std $dir>](
                // <std::os::unix::io::OwnedFd as std::os::unix::io::AsFd>::as_fd(&$fd)
                $file.as_fd()
            ) {
                return Err(concat!("dup2_std", stringify!($dir), ": failed to replace std", stringify!($dir)).into());
            }
        }
    };
}

/// ```
/// files[0]   ─┐
///   ...      ─┼─> fd_dst
/// files[n-1] ─┘
/// ```
/// The function has a cat-like behavior.
pub fn merge(files: &Vec<File>, fd_dst: OwnedFd) -> Result<(), std::io::Error> {
    let mut buf = [0u8; 4096];
    let mut file_dst = File::from(fd_dst);

    for f in files {
        let mut reader = BufReader::new(f);

        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 {
                break;
            }
            file_dst.write_all(&buf[..n])?;
        }
    }

    Ok(())
}

/// ```
///         ┌─> files[0]
/// fd_src ─┼─>   ...
///         └─> files[n-1]
/// ```
/// The function has a tee-like behavior.
pub fn spread(files: &mut Vec<File>, fd_src: OwnedFd) -> Result<(), std::io::Error> {
    let mut buf = [0u8; 4096];
    let mut reader = BufReader::new(File::from(fd_src));

    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
        for f in files.into_iter() {
            f.write_all(&buf[..n])?;
        }
    }

    Ok(())
}
