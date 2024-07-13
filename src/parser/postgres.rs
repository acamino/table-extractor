use crate::error::Result;
use crate::{Parser, Table};

pub struct PostgresParser;

impl Parser for PostgresParser {
    fn parse(&self, input: &str) -> Result<Table> {
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return Ok(Table::new(vec![], vec![]));
        }

        let mut headers = Vec::new();
        let mut rows = Vec::new();
        let mut found_separator = false;

        for line in lines {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                continue;
            }

            // Check if this is a separator line (contains dashes and pipes)
            if is_separator_line(trimmed) {
                found_separator = true;
                continue;
            }

            // Parse the row
            let cells = parse_postgres_row(trimmed);

            if !found_separator && headers.is_empty() {
                // First row is the header
                headers = cells;
            } else if found_separator {
                // Data rows come after the separator
                rows.push(cells);
            }
        }

        Ok(Table::new(headers, rows))
    }
}

fn is_separator_line(line: &str) -> bool {
    // PostgreSQL separator line contains only -, +, and whitespace
    line.chars().all(|c| matches!(c, '-' | '+' | ' ')) && line.contains('-') && line.contains('+')
}

fn parse_postgres_row(line: &str) -> Vec<String> {
    // Split by | and trim each cell
    line.split('|')
        .map(|cell| cell.trim().to_string())
        .filter(|cell| !cell.is_empty()) // Remove empty cells from edges
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_postgres() {
        let input = r#" id | store_id | shopify_location_id | name | active
----+----------+---------------------+------+--------
  1 |        1 | gid://shopify/...   | 2299 | t
  2 |        1 | gid://shopify/...   | 4510 | t"#;

        let parser = PostgresParser;
        let table = parser.parse(input).unwrap();

        assert_eq!(
            table.headers,
            vec!["id", "store_id", "shopify_location_id", "name", "active"]
        );
        assert_eq!(table.rows.len(), 2);
        assert_eq!(
            table.rows[0],
            vec!["1", "1", "gid://shopify/...", "2299", "t"]
        );
    }
}
