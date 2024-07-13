use crate::error::Result;
use crate::{Parser, Table};

pub struct MarkdownParser;

impl Parser for MarkdownParser {
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

            // Check if this is a separator line (contains only |, -, :, and whitespace)
            if is_separator_line(trimmed) {
                found_separator = true;
                continue;
            }

            // Parse the row
            let cells = parse_markdown_row(trimmed);

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
    // A separator line contains only |, -, :, and whitespace
    line.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
        && line.contains('-')
        && line.contains('|')
}

fn parse_markdown_row(line: &str) -> Vec<String> {
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
    fn test_parse_markdown() {
        let input = r#"| API Metric Name | MongoDB Slice | Position |
|-----------------|---------------|----------|
| sessions        | ACQUISITION   | Index 0  |
| newUsers        | ACQUISITION   | Index 1  |"#;

        let parser = MarkdownParser;
        let table = parser.parse(input).unwrap();

        assert_eq!(
            table.headers,
            vec!["API Metric Name", "MongoDB Slice", "Position"]
        );
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["sessions", "ACQUISITION", "Index 0"]);
        assert_eq!(table.rows[1], vec!["newUsers", "ACQUISITION", "Index 1"]);
    }
}
