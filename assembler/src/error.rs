use core::fmt;
use std::{io, result};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    PreprocessError(String),
    AssembleError(String),
    SymbolTableError(String),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO Error: {}", e),
            Error::PreprocessError(e) => write!(f, "Preprocess Error: {}", e),
            Error::AssembleError(e) => write!(f, "Assemble Error: {}", e),
            Error::SymbolTableError(e) => write!(f, "Symbol Table Error: {}", e),
        }
    }
}