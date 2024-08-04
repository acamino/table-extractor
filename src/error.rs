use std::fmt;

/// Error types for table parsing and writing operations.
///
/// This enum represents all possible errors that can occur during
/// table operations, including parsing, validation, and I/O.
#[derive(Debug)]
pub enum Error {
    /// Error during table parsing.
    ///
    /// This variant is used when input data cannot be parsed into a valid table.
    /// The string contains details about what went wrong.
    ParseError(String),

    /// I/O error during reading or writing.
    ///
    /// This wraps standard I/O errors that occur when reading input or
    /// writing output.
    IoError(std::io::Error),

    /// Invalid format or format constraint violation.
    ///
    /// Used when the input or output violates format constraints,
    /// such as exceeding the maximum column count or containing
    /// invalid delimiter characters.
    InvalidFormat(String),

    /// Inconsistent column count in table data.
    ///
    /// This error occurs when a table row has a different number of
    /// columns than the header row.
    ///
    /// # Fields
    ///
    /// - `row`: The row number (1-indexed) that has inconsistent columns
    /// - `expected`: The expected number of columns (from header)
    /// - `found`: The actual number of columns found in the row
    InconsistentColumns {
        /// The row number (1-indexed) with inconsistent columns
        row: usize,
        /// Expected number of columns
        expected: usize,
        /// Actual number of columns found
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

/// Type alias for `Result<T, Error>`.
///
/// This is a convenience type that uses the library's [`Error`] type
/// as the error variant.
///
/// # Examples
///
/// ```
/// use table_extractor::error::Result;
/// use table_extractor::Table;
///
/// fn create_table() -> Result<Table> {
///     Table::new_validated(
///         vec!["id".to_string(), "name".to_string()],
///         vec![vec!["1".to_string(), "Alice".to_string()]],
///     )
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;
