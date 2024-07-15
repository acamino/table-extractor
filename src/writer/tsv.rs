use crate::error::Result;
use crate::{Table, Writer};
use std::io::Write as IoWrite;

pub struct TsvWriter {
    delimiter: char,
}

impl TsvWriter {
    pub fn new(delimiter: char) -> Self {
        Self { delimiter }
    }
}

impl Default for TsvWriter {
    fn default() -> Self {
        Self::new('\t')
    }
}

impl Writer for TsvWriter {
    fn write(&self, table: &Table, output: &mut dyn IoWrite) -> Result<()> {
        // Validate headers don't contain delimiter to prevent data corruption
        for header in &table.headers {
            if header.contains(self.delimiter) {
                return Err(crate::error::Error::InvalidFormat(
                    format!(
                        "Header '{}' contains delimiter character '{}'. Use -o csv for proper escaping.",
                        header, self.delimiter
                    )
                ));
            }
        }

        // Write headers
        writeln!(
            output,
            "{}",
            table.headers.join(&self.delimiter.to_string())
        )?;

        // Validate and write rows
        for (idx, row) in table.rows.iter().enumerate() {
            for cell in row {
                if cell.contains(self.delimiter) {
                    return Err(crate::error::Error::InvalidFormat(
                        format!(
                            "Row {} contains delimiter character '{}' in data. Use -o csv for proper escaping.",
                            idx + 1, self.delimiter
                        )
                    ));
                }
            }
            writeln!(output, "{}", row.join(&self.delimiter.to_string()))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_tsv() {
        let table = Table::new(
            vec!["id".to_string(), "name".to_string()],
            vec![
                vec!["1".to_string(), "Alice".to_string()],
                vec!["2".to_string(), "Bob".to_string()],
            ],
        );

        let writer = TsvWriter::default();
        let mut output = Vec::new();
        writer.write(&table, &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "id\tname\n1\tAlice\n2\tBob\n");
    }

    #[test]
    fn test_write_custom_delimiter() {
        let table = Table::new(
            vec!["id".to_string(), "name".to_string()],
            vec![vec!["1".to_string(), "Alice".to_string()]],
        );

        let writer = TsvWriter::new('|');
        let mut output = Vec::new();
        writer.write(&table, &mut output).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "id|name\n1|Alice\n");
    }

    #[test]
    fn test_reject_tab_in_data() {
        let table = Table::new(
            vec!["id".to_string(), "name".to_string()],
            vec![vec!["1".to_string(), "Alice\tBob".to_string()]],
        );

        let writer = TsvWriter::default();
        let mut output = Vec::new();
        let result = writer.write(&table, &mut output);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("delimiter"));
    }

    #[test]
    fn test_reject_custom_delimiter_in_data() {
        let table = Table::new(
            vec!["id".to_string(), "name".to_string()],
            vec![vec!["1".to_string(), "Uses | pipes".to_string()]],
        );

        let writer = TsvWriter::new('|');
        let mut output = Vec::new();
        let result = writer.write(&table, &mut output);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("delimiter"));
        assert!(error_msg.contains("|"));
    }

    #[test]
    fn test_reject_delimiter_in_header() {
        let table = Table::new(
            vec!["id".to_string(), "name|alias".to_string()],
            vec![vec!["1".to_string(), "Alice".to_string()]],
        );

        let writer = TsvWriter::new('|');
        let mut output = Vec::new();
        let result = writer.write(&table, &mut output);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Header"));
        assert!(error_msg.contains("name|alias"));
    }
}
