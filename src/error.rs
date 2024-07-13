use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    IoError(std::io::Error),
    InvalidFormat(String),
    InconsistentColumns {
        row: usize,
        expected: usize,
        found: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Error::IoError(err) => write!(f, "I/O error: {}", err),
            Error::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            Error::InconsistentColumns {
                row,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Inconsistent column count at row {}: expected {}, found {}",
                    row, expected, found
                )
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        Error::ParseError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
