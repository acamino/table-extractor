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

        // Get rows
        let mut rows = Vec::new();
        for result in reader.records() {
            let record = result?;
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
}
