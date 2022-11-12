use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    RomError(String),
    InvalidMemoryAddress(usize),
    InvalidROMAddress(usize),
    InvalidCInstructionPadding(usize),
    InvalidCInstructionComp(usize),
    PixelsError(pixels::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<pixels::Error> for Error {
    fn from(error: pixels::Error) -> Self {
        Error::PixelsError(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO Error: {}", e),
            Error::RomError(e) => write!(f, "ROM Error: {}", e),
            Error::InvalidMemoryAddress(e) => write!(f, "Invalid Memory Address: {:04X}", e),
            Error::InvalidROMAddress(e) => write!(f, "Invalid ROM Address: {:04X}", e),
            Error::InvalidCInstructionPadding(pc) => write!(f, "Invalid C Instruction Padding, should be 111, at PC: {:04X}", pc),
            Error::InvalidCInstructionComp(pc) => write!(f, "Invalid C Instruction Computation, at PC: {:04X}", pc),
            Error::PixelsError(e) => write!(f, "Pixels Error: {}", e),
        }
    }
}