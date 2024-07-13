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
        // Write headers
        writeln!(
            output,
            "{}",
            table.headers.join(&self.delimiter.to_string())
        )?;

        // Write rows
        for row in &table.rows {
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
}
