pub mod detector;
pub mod error;
pub mod parser;
pub mod writer;

use error::Result;
use std::io::Write;
use std::str::FromStr;

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
