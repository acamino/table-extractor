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

/// Represents a parsed table with headers and rows
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self { headers, rows }
    }

    /// Validates that all rows have the same number of columns as headers
    ///
    /// # Errors
    ///
    /// Returns an error if any row has a different column count than the header
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

    /// Creates a new table and validates it
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The number of columns exceeds MAX_COLUMNS (10,000)
    /// - Any row has a different column count than the header
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

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn column_count(&self) -> usize {
        self.headers.len()
    }
}

/// Format detection result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Markdown,
    MySQL,
    PostgreSQL,
    CSV,
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

/// Parser trait for input formats
pub trait Parser {
    fn parse(&self, input: &str) -> Result<Table>;
}

/// Writer trait for output formats
pub trait Writer {
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
}
