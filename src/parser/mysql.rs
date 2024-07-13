use crate::error::Result;
use crate::{Parser, Table};

pub struct MySqlParser;

impl Parser for MySqlParser {
    fn parse(&self, input: &str) -> Result<Table> {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Ok(Table::new(vec![], vec![]));
        }

        let mut headers = Vec::new();
        let mut rows = Vec::new();

        for line in lines {
            let trimmed = line.trim();

            // Skip empty lines and border lines (starting with +)
            if trimmed.is_empty() || trimmed.starts_with('+') {
                continue;
            }

            // Parse data lines (starting and ending with |)
            if trimmed.starts_with('|') && trimmed.ends_with('|') {
                let cells = parse_mysql_row(trimmed);

                if headers.is_empty() {
                    headers = cells;
                } else {
                    rows.push(cells);
                }
            }
        }

        Ok(Table::new(headers, rows))
    }
}

fn parse_mysql_row(line: &str) -> Vec<String> {
    // Remove leading and trailing pipes
    let trimmed = line.trim().trim_start_matches('|').trim_end_matches('|');

    // Split by | and trim each cell
    trimmed
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mysql() {
        let input = r#"+----+----------------------------+
| id | name                       |
+----+----------------------------+
|  1 | Preston Carlton's Company  |
|  2 | Fawzia Masud's Company     |
+----+----------------------------+"#;

        let parser = MySqlParser;
        let table = parser.parse(input).unwrap();

        assert_eq!(table.headers, vec!["id", "name"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["1", "Preston Carlton's Company"]);
        assert_eq!(table.rows[1], vec!["2", "Fawzia Masud's Company"]);
    }
}
