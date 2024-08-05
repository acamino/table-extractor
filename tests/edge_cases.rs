use assert_cmd::Command;
use predicates::prelude::*;
use table_extractor::parser::{CsvParser, MarkdownParser};
use table_extractor::{Parser, Table};

#[test]
fn test_inconsistent_column_count_markdown() {
    let input =
        "| id | name | email |\n|----|----|----|\n| 1 | Alice | alice@example.com |\n| 2 | Bob |";
    let parser = MarkdownParser;
    let result = parser.parse(input);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Inconsistent column count"));
    assert!(err_msg.contains("row 2"));
}

#[test]
fn test_inconsistent_column_count_csv() {
    let input = "id,name,email\n1,Alice,alice@example.com\n2,Bob";
    let parser = CsvParser::new(b',');
    let result = parser.parse(input);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    // CSV parser has its own validation that catches this
    assert!(
        err_msg.contains("found record with 2 fields")
            || err_msg.contains("Inconsistent column count")
    );
}

#[test]
fn test_maximum_input_size_exceeded() {
    // Create input larger than 100 MB
    let large_row = "x".repeat(1024); // 1 KB row
    let mut large_input = String::from("id,data\n");

    // Add enough rows to exceed 100 MB
    for i in 0..110_000 {
        large_input.push_str(&format!("{},{}\n", i, large_row));
    }

    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(large_input)
        .assert()
        .failure()
        .stderr(predicate::str::contains("exceeds maximum size"));
}

#[test]
fn test_many_columns() {
    // Test table with 1000 columns
    let mut headers = Vec::new();
    let mut row = Vec::new();

    for i in 0..1000 {
        headers.push(format!("col{}", i));
        row.push(i.to_string());
    }

    let table = Table::new_validated(headers.clone(), vec![row.clone()]);
    assert!(table.is_ok());

    let table = table.unwrap();
    assert_eq!(table.column_count(), 1000);
    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0].len(), 1000);
}

#[test]
fn test_max_columns_limit() {
    // Test exactly at the limit (10,000 columns)
    let headers: Vec<String> = (0..10_000).map(|i| format!("col{}", i)).collect();
    let result = Table::new_validated(headers, vec![]);
    assert!(result.is_ok());
}

#[test]
fn test_exceed_max_columns() {
    // Test exceeding the limit (10,001 columns)
    let headers: Vec<String> = (0..10_001).map(|i| format!("col{}", i)).collect();
    let result = Table::new_validated(headers, vec![]);

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Too many columns"));
    assert!(err_msg.contains("10001"));
    assert!(err_msg.contains("10000"));
}

#[test]
fn test_very_long_cell() {
    // Test cell with 1 MB of data
    let long_cell = "a".repeat(1_000_000); // 1 MB

    let table = Table::new_validated(
        vec!["id".to_string(), "data".to_string()],
        vec![vec!["1".to_string(), long_cell.clone()]],
    );

    assert!(table.is_ok());
    let table = table.unwrap();
    assert_eq!(table.rows[0][1].len(), 1_000_000);
}

#[test]
fn test_empty_input() {
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin("")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_only_whitespace() {
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin("   \n\n  \t\n")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_control_characters_in_data() {
    // Test with null byte in data
    let input = "id,name\n1,Alice\x00Bob\n2,Charlie";

    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input).assert().success(); // Should handle gracefully without crashing
}

#[test]
fn test_unicode_control_characters() {
    // Test with various control characters
    let input = "id,name\n1,Alice\u{0001}\n2,Bob\u{0002}\n3,Charlie";

    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input).assert().success();
}

#[test]
fn test_empty_rows_rejected() {
    let input = "| id | name |\n|-------|-------|\n| 1 | Alice |\n|||";
    let parser = MarkdownParser;
    let result = parser.parse(input);

    // Empty row should be rejected due to inconsistent columns
    assert!(result.is_err());
}

#[test]
fn test_single_column_table() {
    let table = Table::new_validated(
        vec!["id".to_string()],
        vec![
            vec!["1".to_string()],
            vec!["2".to_string()],
            vec!["3".to_string()],
        ],
    );

    assert!(table.is_ok());
    let table = table.unwrap();
    assert_eq!(table.column_count(), 1);
    assert_eq!(table.rows.len(), 3);
}

#[test]
fn test_table_with_empty_cells() {
    let table = Table::new_validated(
        vec!["id".to_string(), "name".to_string(), "email".to_string()],
        vec![
            vec![
                "1".to_string(),
                "Alice".to_string(),
                "alice@example.com".to_string(),
            ],
            vec!["2".to_string(), "".to_string(), "".to_string()], // Empty cells
            vec![
                "3".to_string(),
                "Charlie".to_string(),
                "charlie@example.com".to_string(),
            ],
        ],
    );

    assert!(table.is_ok());
    let table = table.unwrap();
    assert_eq!(table.rows[1][1], "");
    assert_eq!(table.rows[1][2], "");
}

#[test]
fn test_headers_only_no_data() {
    let table = Table::new_validated(
        vec!["id".to_string(), "name".to_string(), "email".to_string()],
        vec![],
    );

    assert!(table.is_ok());
    let table = table.unwrap();
    assert!(table.is_empty());
    assert_eq!(table.column_count(), 3);
}

#[test]
fn test_newlines_in_csv_cells() {
    // CSV format should handle newlines in quoted fields
    let input = "id,description\n1,\"Line 1\nLine 2\"\n2,\"Single line\"";

    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Line 1"))
        .stdout(predicate::str::contains("Line 2"));
}

#[test]
fn test_special_characters_in_headers() {
    let table = Table::new_validated(
        vec![
            "id@#$".to_string(),
            "name%^&".to_string(),
            "email*()".to_string(),
        ],
        vec![vec![
            "1".to_string(),
            "Alice".to_string(),
            "alice@example.com".to_string(),
        ]],
    );

    assert!(table.is_ok());
}

#[test]
fn test_very_wide_table_integration() {
    // Test 500 columns via CLI
    let mut headers = String::from("col0");
    let mut row1 = String::from("0");
    let mut row2 = String::from("1");

    for i in 1..500 {
        headers.push_str(&format!(",col{}", i));
        row1.push_str(&format!(",{}", i * 2));
        row2.push_str(&format!(",{}", i * 2 + 1));
    }

    let input = format!("{}\n{}\n{}", headers, row1, row2);

    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("col0"))
        .stdout(predicate::str::contains("col499"));
}

#[test]
fn test_all_empty_cells() {
    let table = Table::new_validated(
        vec!["col1".to_string(), "col2".to_string(), "col3".to_string()],
        vec![
            vec!["".to_string(), "".to_string(), "".to_string()],
            vec!["".to_string(), "".to_string(), "".to_string()],
        ],
    );

    assert!(table.is_ok());
}

#[test]
fn test_mixed_row_lengths_detected() {
    let table = Table::new_validated(
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()], // 3 columns - OK
            vec!["4".to_string(), "5".to_string()],                  // 2 columns - ERROR
        ],
    );

    assert!(table.is_err());
    let err = table.unwrap_err();
    assert!(err.to_string().contains("row 2"));
    assert!(err.to_string().contains("expected 3"));
    assert!(err.to_string().contains("found 2"));
}

#[test]
fn test_extra_columns_in_row() {
    let table = Table::new_validated(
        vec!["a".to_string(), "b".to_string()],
        vec![
            vec!["1".to_string(), "2".to_string()], // 2 columns - OK
            vec!["3".to_string(), "4".to_string(), "5".to_string()], // 3 columns - ERROR
        ],
    );

    assert!(table.is_err());
    let err = table.unwrap_err();
    assert!(err.to_string().contains("row 2"));
    assert!(err.to_string().contains("expected 2"));
    assert!(err.to_string().contains("found 3"));
}
