use crate::error::Result;
use crate::{Table, Writer};
use csv::WriterBuilder;
use std::io::Write as IoWrite;

pub struct CsvWriter;

impl CsvWriter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CsvWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl Writer for CsvWriter {
    fn write(&self, table: &Table, output: &mut dyn IoWrite) -> Result<()> {
        // Write directly to output instead of buffering in Vec
        // The csv crate uses an internal buffer, and stdout is already wrapped in BufWriter
        let mut writer = WriterBuilder::new().has_headers(false).from_writer(output);

        // Write headers
        writer.write_record(table.headers())?;

        // Write rows
        for row in table.rows() {
            writer.write_record(row)?;
        }

        // Flush the csv writer to ensure all data is written
        writer.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_csv() {
        let table = Table::new(
            vec!["id".to_string(), "name".to_string()],
            vec![
                vec!["1".to_string(), "Alice".to_string()],
                vec!["2".to_string(), "Bob".to_string()],
            ],
        );

        let writer = CsvWriter::new();
        let mut output = Vec::new();
        writer.write(&table, &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "id,name\n1,Alice\n2,Bob\n");
    }

    #[test]
    fn test_write_csv_with_quotes() {
        let table = Table::new(
            vec!["id".to_string(), "name".to_string()],
            vec![vec![
                "1".to_string(),
                "Alice, Bob".to_string(), // Contains comma, should be quoted
            ]],
        );

        let writer = CsvWriter::new();
        let mut output = Vec::new();
        writer.write(&table, &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "id,name\n1,\"Alice, Bob\"\n");
    }
}
