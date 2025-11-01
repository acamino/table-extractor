pub mod detector;
pub mod error;
pub mod parser;
pub mod writer;

use error::Result;
use std::io::Write;
use std::str::FromStr;

/// Maximum number of columns allowed in a table.
/// Prevents out-of-memory attacks via excessively wide tables.
const MAX_COLUMNS: usize = 10_000;

/// Represents a parsed table with headers and data rows.
///
/// All rows must have the same number of columns as the header.
/// Use [`Table::new_validated`] to create a table with automatic validation.
///
/// # Examples
///
/// ```
/// use table_extractor::Table;
///
/// let table = Table::new(
///     vec!["id".to_string(), "name".to_string()],
///     vec![
///         vec!["1".to_string(), "Alice".to_string()],
///         vec!["2".to_string(), "Bob".to_string()],
///     ],
/// );
///
/// assert_eq!(table.column_count(), 2);
/// assert!(!table.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    /// Column headers
    headers: Vec<String>,

    /// Data rows, where each row should have the same length as headers
    rows: Vec<Vec<String>>,
}

impl Table {
    /// Creates a new table without validation.
    ///
    /// For safer construction with automatic validation, use [`Table::new_validated`].
    ///
    /// # Examples
    ///
    /// ```
    /// use table_extractor::Table;
    ///
    /// let table = Table::new(
    ///     vec!["id".to_string(), "name".to_string()],
    ///     vec![vec!["1".to_string(), "Alice".to_string()]],
    /// );
    /// ```
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self { headers, rows }
    }

    /// Validates that all rows have the same number of columns as headers.
    ///
    /// # Errors
    ///
    /// Returns [`error::Error::InconsistentColumns`] if any row has a different
    /// column count than the header.
    ///
    /// # Examples
    ///
    /// ```
    /// use table_extractor::Table;
    ///
    /// let table = Table::new(
    ///     vec!["id".to_string(), "name".to_string()],
    ///     vec![
    ///         vec!["1".to_string(), "Alice".to_string()],
    ///         vec!["2".to_string(), "Bob".to_string()],
    ///     ],
    /// );
    ///
    /// assert!(table.validate().is_ok());
    ///
    /// // Table with inconsistent columns
    /// let bad_table = Table::new(
    ///     vec!["id".to_string(), "name".to_string()],
    ///     vec![vec!["1".to_string()]], // Missing column!
    /// );
    ///
    /// assert!(bad_table.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<()> {
        let expected = self.headers.len();
        for (idx, row) in self.rows.iter().enumerate() {
            if row.len() != expected {
                return Err(error::Error::InconsistentColumns {
                    row: idx + 1,
                    expected,
                    found: row.len(),
                });
            }
        }
        Ok(())
    }

    /// Creates a new table and validates it.
    ///
    /// This is the recommended way to create a table as it ensures data integrity
    /// by validating column counts and enforcing limits.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The number of columns exceeds 10,000 ([`error::Error::InvalidFormat`])
    /// - Any row has a different column count than the header ([`error::Error::InconsistentColumns`])
    ///
    /// # Examples
    ///
    /// ```
    /// use table_extractor::Table;
    ///
    /// // Valid table
    /// let table = Table::new_validated(
    ///     vec!["id".to_string(), "name".to_string()],
    ///     vec![
    ///         vec!["1".to_string(), "Alice".to_string()],
    ///         vec!["2".to_string(), "Bob".to_string()],
    ///     ],
    /// );
    /// assert!(table.is_ok());
    ///
    /// // Invalid table (inconsistent columns)
    /// let bad_table = Table::new_validated(
    ///     vec!["id".to_string(), "name".to_string()],
    ///     vec![vec!["1".to_string()]], // Missing column!
    /// );
    /// assert!(bad_table.is_err());
    /// ```
    pub fn new_validated(headers: Vec<String>, rows: Vec<Vec<String>>) -> Result<Self> {
        // Check column count limit
        if headers.len() > MAX_COLUMNS {
            return Err(error::Error::InvalidFormat(format!(
                "Too many columns: {} (maximum: {})",
                headers.len(),
                MAX_COLUMNS
            )));
        }

        let table = Self { headers, rows };
        table.validate()?;
        Ok(table)
    }

    /// Returns `true` if the table contains no data rows.
    ///
    /// Note: A table with headers but no data rows is considered empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use table_extractor::Table;
    ///
    /// let empty_table = Table::new(
    ///     vec!["id".to_string(), "name".to_string()],
    ///     vec![],
    /// );
    /// assert!(empty_table.is_empty());
    ///
    /// let table_with_data = Table::new(
    ///     vec!["id".to_string(), "name".to_string()],
    ///     vec![vec!["1".to_string(), "Alice".to_string()]],
    /// );
    /// assert!(!table_with_data.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns the number of columns in the table.
    ///
    /// This is equivalent to the length of the headers vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use table_extractor::Table;
    ///
    /// let table = Table::new(
    ///     vec!["id".to_string(), "name".to_string(), "email".to_string()],
    ///     vec![],
    /// );
    /// assert_eq!(table.column_count(), 3);
    /// ```
    pub fn column_count(&self) -> usize {
        self.headers.len()
    }

    /// Returns a reference to the table headers.
    ///
    /// # Examples
    ///
    /// ```
    /// use table_extractor::Table;
    ///
    /// let table = Table::new(
    ///     vec!["id".to_string(), "name".to_string()],
    ///     vec![],
    /// );
    /// assert_eq!(table.headers(), &["id", "name"]);
    /// ```
    pub fn headers(&self) -> &[String] {
        &self.headers
    }

    /// Returns a reference to the table rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use table_extractor::Table;
    ///
    /// let table = Table::new(
    ///     vec!["id".to_string(), "name".to_string()],
    ///     vec![
    ///         vec!["1".to_string(), "Alice".to_string()],
    ///         vec!["2".to_string(), "Bob".to_string()],
    ///     ],
    /// );
    /// assert_eq!(table.rows().len(), 2);
    /// assert_eq!(table.rows()[0], vec!["1", "Alice"]);
    /// ```
    pub fn rows(&self) -> &[Vec<String>] {
        &self.rows
    }

    /// Consumes the table and returns the headers and rows.
    ///
    /// This is useful when you need ownership of the table's data.
    ///
    /// # Examples
    ///
    /// ```
    /// use table_extractor::Table;
    ///
    /// let table = Table::new(
    ///     vec!["id".to_string(), "name".to_string()],
    ///     vec![vec!["1".to_string(), "Alice".to_string()]],
    /// );
    ///
    /// let (headers, rows) = table.into_parts();
    /// assert_eq!(headers, vec!["id", "name"]);
    /// assert_eq!(rows.len(), 1);
    /// ```
    pub fn into_parts(self) -> (Vec<String>, Vec<Vec<String>>) {
        (self.headers, self.rows)
    }
}

/// Supported table formats for parsing and auto-detection.
///
/// This enum represents the various table formats that can be parsed by the library.
/// Formats can be auto-detected or explicitly specified.
///
/// # Examples
///
/// ```
/// use table_extractor::Format;
/// use std::str::FromStr;
///
/// // Parse format from string
/// let format = Format::from_str("markdown").unwrap();
/// assert_eq!(format, Format::Markdown);
///
/// // Case insensitive
/// let format = Format::from_str("MySQL").unwrap();
/// assert_eq!(format, Format::MySQL);
///
/// // Aliases are supported
/// let format = Format::from_str("psql").unwrap();
/// assert_eq!(format, Format::PostgreSQL);
///
/// // Display trait converts back to canonical string
/// assert_eq!(format.to_string(), "postgresql");
/// assert_eq!(Format::CSV.to_string(), "csv");
///
/// // Round-trip conversion works
/// let original = Format::Markdown;
/// let parsed = Format::from_str(&original.to_string()).unwrap();
/// assert_eq!(original, parsed);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Markdown table format with pipe delimiters (e.g., `| col1 | col2 |`)
    Markdown,

    /// MySQL CLI output format with box-drawing characters (e.g., `+----+----+`)
    MySQL,

    /// PostgreSQL CLI output format with simple separators (e.g., `----+----`)
    PostgreSQL,

    /// Comma-separated values (CSV) format
    CSV,

    /// Tab-separated values (TSV) format
    TSV,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "markdown" | "md" => Ok(Format::Markdown),
            "mysql" => Ok(Format::MySQL),
            "postgres" | "postgresql" | "psql" => Ok(Format::PostgreSQL),
            "csv" => Ok(Format::CSV),
            "tsv" => Ok(Format::TSV),
            _ => Err(format!(
                "Invalid format: '{}'. Valid formats: markdown, mysql, postgres, csv, tsv",
                s
            )),
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Format::Markdown => "markdown",
            Format::MySQL => "mysql",
            Format::PostgreSQL => "postgresql",
            Format::CSV => "csv",
            Format::TSV => "tsv",
        };
        write!(f, "{}", name)
    }
}

/// Trait for parsing table data from various input formats.
///
/// Implement this trait to add support for new table formats.
/// All parsers should validate column consistency by using [`Table::new_validated`].
///
/// # Examples
///
/// ```
/// use table_extractor::{Parser, Table, error::Result};
///
/// struct CustomParser;
///
/// impl Parser for CustomParser {
///     fn parse(&self, input: &str) -> Result<Table> {
///         // Parse input into headers and rows
///         let headers = vec!["col1".to_string(), "col2".to_string()];
///         let rows = vec![vec!["val1".to_string(), "val2".to_string()]];
///
///         // Use new_validated to ensure data integrity
///         Table::new_validated(headers, rows)
///     }
/// }
/// ```
pub trait Parser {
    /// Parses the input string into a table.
    ///
    /// # Errors
    ///
    /// Returns an error if the input cannot be parsed or if the resulting
    /// table fails validation (inconsistent columns, too many columns, etc.).
    fn parse(&self, input: &str) -> Result<Table>;
}

/// Trait for writing table data to various output formats.
///
/// Implement this trait to add support for new output formats.
///
/// # Examples
///
/// ```
/// use table_extractor::{Writer, Table, error::Result};
/// use std::io::Write as IoWrite;
///
/// struct CustomWriter;
///
/// impl Writer for CustomWriter {
///     fn write(&self, table: &Table, output: &mut dyn IoWrite) -> Result<()> {
///         // Write headers
///         writeln!(output, "{}", table.headers().join(","))?;
///
///         // Write rows
///         for row in table.rows() {
///             writeln!(output, "{}", row.join(","))?;
///         }
///
///         Ok(())
///     }
/// }
/// ```
pub trait Writer {
    /// Writes the table to the provided output stream.
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails or if the table data is invalid
    /// for the output format (e.g., delimiter conflicts).
    fn write(&self, table: &Table, output: &mut dyn Write) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_consistent_columns() {
        let table = Table::new(
            vec!["id".to_string(), "name".to_string()],
            vec![
                vec!["1".to_string(), "Alice".to_string()],
                vec!["2".to_string(), "Bob".to_string()],
            ],
        );
        assert!(table.validate().is_ok());
    }

    #[test]
    fn test_validate_inconsistent_columns() {
        let table = Table::new(
            vec!["id".to_string(), "name".to_string()],
            vec![
                vec!["1".to_string(), "Alice".to_string()],
                vec!["2".to_string()], // Missing column
            ],
        );
        let result = table.validate();
        assert!(result.is_err());
        if let Err(error::Error::InconsistentColumns {
            row,
            expected,
            found,
        }) = result
        {
            assert_eq!(row, 2);
            assert_eq!(expected, 2);
            assert_eq!(found, 1);
        } else {
            panic!("Expected InconsistentColumns error");
        }
    }

    #[test]
    fn test_new_validated_success() {
        let result = Table::new_validated(
            vec!["id".to_string(), "name".to_string()],
            vec![
                vec!["1".to_string(), "Alice".to_string()],
                vec!["2".to_string(), "Bob".to_string()],
            ],
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_validated_fails_on_inconsistent_columns() {
        let result = Table::new_validated(
            vec!["id".to_string(), "name".to_string()],
            vec![
                vec!["1".to_string(), "Alice".to_string()],
                vec!["2".to_string()], // Missing column
            ],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_empty_table() {
        let table = Table::new(vec![], vec![]);
        assert!(table.validate().is_ok());
    }

    #[test]
    fn test_validate_no_rows() {
        let table = Table::new(vec!["id".to_string(), "name".to_string()], vec![]);
        assert!(table.validate().is_ok());
    }

    #[test]
    fn test_new_validated_rejects_too_many_columns() {
        let headers: Vec<String> = (0..10001).map(|i| format!("col{}", i)).collect();
        let result = Table::new_validated(headers, vec![]);
        assert!(result.is_err());
        if let Err(error::Error::InvalidFormat(msg)) = result {
            assert!(msg.contains("Too many columns"));
            assert!(msg.contains("10001"));
            assert!(msg.contains("10000"));
        } else {
            panic!("Expected InvalidFormat error");
        }
    }

    #[test]
    fn test_new_validated_accepts_max_columns() {
        let headers: Vec<String> = (0..10000).map(|i| format!("col{}", i)).collect();
        let result = Table::new_validated(headers, vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_validated_accepts_just_under_max() {
        let headers: Vec<String> = (0..9999).map(|i| format!("col{}", i)).collect();
        let result = Table::new_validated(headers, vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_display() {
        // Test Display trait for all Format variants
        assert_eq!(Format::Markdown.to_string(), "markdown");
        assert_eq!(Format::MySQL.to_string(), "mysql");
        assert_eq!(Format::PostgreSQL.to_string(), "postgresql");
        assert_eq!(Format::CSV.to_string(), "csv");
        assert_eq!(Format::TSV.to_string(), "tsv");
    }

    #[test]
    fn test_format_display_roundtrip() {
        use std::str::FromStr;

        // Test that Display output can be parsed back to the same Format
        let formats = vec![
            Format::Markdown,
            Format::MySQL,
            Format::PostgreSQL,
            Format::CSV,
            Format::TSV,
        ];

        for format in formats {
            let string = format.to_string();
            let parsed = Format::from_str(&string).unwrap();
            assert_eq!(format, parsed, "Round-trip failed for {}", string);
        }
    }
}
