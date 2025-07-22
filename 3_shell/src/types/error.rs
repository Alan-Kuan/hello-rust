use std::fmt;

pub enum GenericError {
    IOError(std::io::Error),
    OtherError(String),
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenericError::IOError(err) => write!(f, "{err}"),
            GenericError::OtherError(s) => write!(f, "{s}"),
        }
    }
}

impl From<std::io::Error> for GenericError {
    fn from(err: std::io::Error) -> Self {
        GenericError::IOError(err)
    }
}

impl From<String> for GenericError {
    fn from(s: String) -> Self {
        GenericError::OtherError(s)
    }
}

impl From<&str> for GenericError {
    fn from(s: &str) -> Self {
        GenericError::OtherError(s.to_string())
    }
}
