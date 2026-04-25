use std::fmt;

#[derive(Debug)]
pub enum Error {
    Config(String),
    Runtime(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Config(m)  => write!(f, "config error: {}", m),
            Error::Runtime(m) => write!(f, "runtime error: {}", m),
        }
    }
}

impl std::error::Error for Error {}
