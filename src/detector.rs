use crate::Format;
use regex::Regex;
use std::sync::OnceLock;

/// Number of lines to examine for format detection
const FORMAT_DETECTION_LINE_LIMIT: usize = 10;

// Compile regexes once at startup for performance
// These are used for format auto-detection
static MYSQL_BORDER: OnceLock<Regex> = OnceLock::new();
static POSTGRES_SEP: OnceLock<Regex> = OnceLock::new();
static MARKDOWN_SEP: OnceLock<Regex> = OnceLock::new();

fn get_mysql_border() -> &'static Regex {
    MYSQL_BORDER.get_or_init(|| Regex::new(r"^\+[-+]+\+$").expect("Invalid MySQL border regex"))
}

fn get_postgres_sep() -> &'static Regex {
    POSTGRES_SEP.get_or_init(|| {
        Regex::new(r"^\s*-+(\+-+)+\s*$").expect("Invalid PostgreSQL separator regex")
    })
}

fn get_markdown_sep() -> &'static Regex {
    MARKDOWN_SEP.get_or_init(|| {
        Regex::new(r"^\s*\|(?:\s*:?\s*-+\s*:?\s*\|)+").expect("Invalid Markdown separator regex")
    })
}

/// Detects the table format from input text
pub fn detect_format(input: &str) -> Format {
    let lines: Vec<&str> = input.lines().take(FORMAT_DETECTION_LINE_LIMIT).collect();

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
    lines
        .iter()
        .any(|line| get_mysql_border().is_match(line.trim()))
}

fn is_postgres_format(lines: &[&str]) -> bool {
    // PostgreSQL has a separator line like ----+----+----
    // Usually on the second line
    if lines.len() < 2 {
        return false;
    }

    lines.iter().any(|line| get_postgres_sep().is_match(line))
}

fn is_markdown_format(lines: &[&str]) -> bool {
    // Markdown tables have separator lines like |---|---|
    lines.iter().any(|line| get_markdown_sep().is_match(line))
}

fn is_tsv_format(lines: &[&str]) -> bool {
    // TSV contains tabs
    let has_tabs = lines.iter().any(|line| line.contains('\t'));
    if !has_tabs {
        return false;
    }

    // If it looks like a structured format (Markdown/PostgreSQL), it's not TSV
    // Markdown has pipes at consistent positions (|col|col|)
    // PostgreSQL has separator lines with +
    let looks_like_markdown = lines.iter().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with('|') && trimmed.ends_with('|')
    });

    let has_plus = lines.iter().any(|line| line.contains('+'));

    has_tabs && !looks_like_markdown && !has_plus
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

    #[test]
    fn test_detect_tsv_with_pipes_in_data() {
        // TSV should be detected even if data contains pipe characters
        let input = "id\tname\tdesc\n1\tAlice\tUses | pipes\n2\tBob\tNormal text";
        assert_eq!(detect_format(input), Format::TSV);
    }

    // ReDoS vulnerability tests - ensure patterns complete quickly even with attack vectors
    #[test]
    fn test_postgres_sep_redos_protection() {
        // Previously vulnerable pattern: r"^[\s\-]+\+[\s\-\+]+$"
        // Attack vector: many spaces/dashes followed by non-matching char
        let attack_string = format!("{} X", " -".repeat(100));

        // This should complete quickly (not hang)
        let result = get_postgres_sep().is_match(&attack_string);
        assert!(
            !result,
            "Attack string should not match valid PostgreSQL separator"
        );

        // Valid PostgreSQL separators should still match
        assert!(get_postgres_sep().is_match("----+----+----"));
        assert!(get_postgres_sep().is_match("  ----+-------  "));
        assert!(get_postgres_sep().is_match("-+-"));
    }

    #[test]
    fn test_markdown_sep_redos_protection() {
        // Previously vulnerable pattern: r"^\s*\|[\s:-]*-[\s:-]*\|"
        // Attack vector: pipe followed by many spaces/colons/dashes without final pipe
        let attack_string = format!("|{} X", " :-".repeat(100));

        // This should complete quickly (not hang)
        let result = get_markdown_sep().is_match(&attack_string);
        assert!(
            !result,
            "Attack string should not match valid Markdown separator"
        );

        // Valid Markdown separators should still match
        assert!(get_markdown_sep().is_match("|---|---|"));
        assert!(get_markdown_sep().is_match("|:---|:---:|"));
        assert!(get_markdown_sep().is_match("| --- | --- |"));
        assert!(get_markdown_sep().is_match("|:-|:-:|"));
    }

    #[test]
    fn test_mysql_border_edge_cases() {
        // Ensure MySQL border regex is robust
        assert!(get_mysql_border().is_match("+--+"));
        assert!(get_mysql_border().is_match("+----+----+"));
        assert!(get_mysql_border().is_match("+-+"));

        // Should not match invalid patterns
        assert!(!get_mysql_border().is_match("+ - +"));
        assert!(!get_mysql_border().is_match("++"));
        assert!(!get_mysql_border().is_match("----"));
    }

    #[test]
    fn test_postgres_format_detection_with_various_separators() {
        // Test detection with different valid PostgreSQL separator styles
        let input1 = " id | name\n----+-------\n  1 | Alice";
        assert_eq!(detect_format(input1), Format::PostgreSQL);

        let input2 = " id | name\n--+--\n  1 | Alice";
        assert_eq!(detect_format(input2), Format::PostgreSQL);

        let input3 = " id | name | age\n----+-------+-----\n  1 | Alice | 30";
        assert_eq!(detect_format(input3), Format::PostgreSQL);
    }

    #[test]
    fn test_markdown_format_detection_with_alignment() {
        // Test detection with different Markdown alignment styles
        let input1 = "| id | name |\n|---|---|\n| 1 | Alice |";
        assert_eq!(detect_format(input1), Format::Markdown);

        let input2 = "| id | name |\n|:---|:---:|\n| 1 | Alice |";
        assert_eq!(detect_format(input2), Format::Markdown);

        let input3 = "| id | name |\n| :--- | ---: |\n| 1 | Alice |";
        assert_eq!(detect_format(input3), Format::Markdown);
    }

    #[test]
    fn test_no_catastrophic_backtracking_large_input() {
        // Create a large string that would cause catastrophic backtracking
        // with the old vulnerable patterns
        let large_attack = format!("{}X", " -".repeat(1000));

        // This should complete quickly
        use std::time::Instant;
        let start = Instant::now();
        let _ = get_postgres_sep().is_match(&large_attack);
        let duration = start.elapsed();

        // Should complete in milliseconds, not seconds
        assert!(
            duration.as_millis() < 100,
            "Regex matching took too long: {:?} - possible ReDoS vulnerability",
            duration
        );
    }
}
