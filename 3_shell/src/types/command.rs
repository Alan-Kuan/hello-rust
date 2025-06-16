use std::fs::File;

pub struct Command {
    pub args: Vec<String>,
    pub files_in: Vec<File>,
    pub files_out: Vec<File>,
}
