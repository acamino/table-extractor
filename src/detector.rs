use crate::Format;
use once_cell::sync::Lazy;
use regex::Regex;

// Compile regexes once at startup for performance
// These are used for format auto-detection
static MYSQL_BORDER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\+[-+]+\+$").expect("Invalid MySQL border regex"));

static POSTGRES_SEP: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[\s\-]+\+[\s\-\+]+$").expect("Invalid PostgreSQL separator regex"));

static MARKDOWN_SEP: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*\|[\s:-]*-[\s:-]*\|").expect("Invalid Markdown separator regex"));

/// Detects the table format from input text
pub fn detect_format(input: &str) -> Format {
    let lines: Vec<&str> = input.lines().take(10).collect();

    if lines.is_empty() {
        return Format::CSV; // Default
    }

    // Check for MySQL format: +---+ or +----+ borders
    if is_mysql_format(&lines) {
        return Format::MySQL;
    }

    // Check for PostgreSQL format: dashes and pipes as separator
    if is_postgres_format(&lines) {
        return Format::PostgreSQL;
    }

    // Check for Markdown format: |---|---| pattern
    if is_markdown_format(&lines) {
        return Format::Markdown;
    }

    // Check for TSV: contains tabs
    if is_tsv_format(&lines) {
        return Format::TSV;
    }

    // Default to CSV
    Format::CSV
}

fn is_mysql_format(lines: &[&str]) -> bool {
    // MySQL tables have border lines like +----+----+
    lines.iter().any(|line| MYSQL_BORDER.is_match(line.trim()))
}

fn is_postgres_format(lines: &[&str]) -> bool {
    // PostgreSQL has a separator line like ----+----+----
    // Usually on the second line
    if lines.len() < 2 {
        return false;
    }

    lines.iter().any(|line| POSTGRES_SEP.is_match(line))
}

fn is_markdown_format(lines: &[&str]) -> bool {
    // Markdown tables have separator lines like |---|---|
    lines.iter().any(|line| MARKDOWN_SEP.is_match(line))
}

fn is_tsv_format(lines: &[&str]) -> bool {
    // TSV contains tabs. We need to distinguish from formats that use pipes as delimiters.
    if lines.is_empty() {
        return false;
    }

    let has_tabs = lines.iter().any(|line| line.contains('\t'));
    if !has_tabs {
        return false;
    }

    // Check if pipes appear in a way that suggests they're structural delimiters
    // Markdown tables start and end with pipes: |col1|col2|
    let looks_like_markdown = lines.iter().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with('|') && trimmed.ends_with('|')
    });

    // PostgreSQL tables have pipes as column separators consistently across lines
    // Count how many lines have the pattern " | " (space-pipe-space)
    // If most lines have them and in similar counts, it's likely PostgreSQL format
    let lines_with_postgres_pipes = lines.iter().filter(|line| line.contains(" | ")).count();
    let looks_like_postgres = lines_with_postgres_pipes > lines.len() / 2;

    // Check for MySQL-style box drawing (+ characters in borders)
    let has_plus_borders = lines.iter().any(|line| line.contains('+'));

    has_tabs && !looks_like_markdown && !looks_like_postgres && !has_plus_borders
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_mysql() {
        let input = r#"+----+-------+
| id | name  |
+----+-------+
|  1 | Alice |
+----+-------+"#;
        assert_eq!(detect_format(input), Format::MySQL);
    }

    #[test]
    fn test_detect_postgres() {
        let input = r#" id | name
----+-------
  1 | Alice
  2 | Bob"#;
        assert_eq!(detect_format(input), Format::PostgreSQL);
    }

    #[test]
    fn test_detect_markdown() {
        let input = r#"| id | name  |
|----|-------|
| 1  | Alice |
| 2  | Bob   |"#;
        assert_eq!(detect_format(input), Format::Markdown);
    }

    #[test]
    fn test_detect_tsv() {
        let input = "id\tname\n1\tAlice\n2\tBob";
        assert_eq!(detect_format(input), Format::TSV);
    }

    #[test]
    fn test_detect_csv() {
        let input = "id,name\n1,Alice\n2,Bob";
        assert_eq!(detect_format(input), Format::CSV);
    }
}
