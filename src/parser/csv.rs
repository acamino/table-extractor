use crate::error::Result;
use crate::{Parser, Table};
use csv::ReaderBuilder;

pub struct CsvParser {
    delimiter: u8,
}

impl CsvParser {
    pub fn new(delimiter: u8) -> Self {
        Self { delimiter }
    }

    pub fn csv() -> Self {
        Self::new(b',')
    }

    pub fn tsv() -> Self {
        Self::new(b'\t')
    }
}

impl Parser for CsvParser {
    fn parse(&self, input: &str) -> Result<Table> {
        let mut reader = ReaderBuilder::new()
            .delimiter(self.delimiter)
            .has_headers(true)
            .from_reader(input.as_bytes());

        // Get headers
        let headers = reader
            .headers()?
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        // Get rows with row number tracking for better error messages
        let mut rows = Vec::new();
        for (idx, result) in reader.records().enumerate() {
            let record = result.map_err(|e| {
                crate::error::Error::ParseError(format!("CSV row {}: {}", idx + 2, e))
            })?;
            let row = record.iter().map(|s| s.to_string()).collect();
            rows.push(row);
        }

        Table::new_validated(headers, rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv() {
        let input = r#"id,name
1,Preston Carlton's Company
2,Fawzia Masud's Company"#;

        let parser = CsvParser::csv();
        let table = parser.parse(input).unwrap();

        assert_eq!(table.headers, vec!["id", "name"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["1", "Preston Carlton's Company"]);
        assert_eq!(table.rows[1], vec!["2", "Fawzia Masud's Company"]);
    }

    #[test]
    fn test_parse_tsv() {
        let input = "id\tname\n1\tAlice\n2\tBob";

        let parser = CsvParser::tsv();
        let table = parser.parse(input).unwrap();

        assert_eq!(table.headers, vec!["id", "name"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["1", "Alice"]);
        assert_eq!(table.rows[1], vec!["2", "Bob"]);
    }

    #[test]
    fn test_csv_error_includes_row_number() {
        // CSV with inconsistent field count on row 2 (first data row)
        let input = "id,name,email\n1,Alice,alice@example.com\n2,Bob";

        let parser = CsvParser::csv();
        let result = parser.parse(input);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        // Should include "CSV row 3" (header is row 1, first data is row 2, problem is row 3)
        assert!(
            err_msg.contains("CSV row 3"),
            "Error message should include row number: {}",
            err_msg
        );
    }

    #[test]
    fn test_csv_error_on_first_data_row() {
        // CSV with error on the very first data row
        let input = "id,name,email\n1,Alice";

        let parser = CsvParser::csv();
        let result = parser.parse(input);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        // Should include "CSV row 2" (first data row after header)
        assert!(
            err_msg.contains("CSV row 2"),
            "Error message should include row number: {}",
            err_msg
        );
    }

    #[test]
    fn test_csv_error_includes_original_error() {
        // CSV with inconsistent field count
        let input = "id,name,email\n1,Alice,alice@example.com\n2,Bob,bob@example.com\n3,Charlie";

        let parser = CsvParser::csv();
        let result = parser.parse(input);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();

        // Should include both row number and original error details
        assert!(err_msg.contains("CSV row 4"), "Should include row number");
        // The csv crate provides details about the error (field count mismatch)
        assert!(
            err_msg.contains("field") || err_msg.contains("2"),
            "Should include field count details"
        );
    }
}
