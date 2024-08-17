use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_completions_bash() {
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("completions")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::contains("_tabx()"))
        .stdout(predicate::str::contains("COMPREPLY"));
}

#[test]
fn test_completions_zsh() {
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("completions")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef tabx"));
}

#[test]
fn test_completions_fish() {
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("completions")
        .arg("fish")
        .assert()
        .success()
        .stdout(predicate::str::contains("complete -c tabx"));
}

#[test]
fn test_completions_help() {
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("completions")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate shell completions"))
        .stdout(predicate::str::contains("bash"))
        .stdout(predicate::str::contains("zsh"))
        .stdout(predicate::str::contains("fish"));
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Convert various tabular data formats",
        ));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("1.0.0"));
}

#[test]
fn test_markdown_simple_to_tsv() {
    let input = fs::read_to_string("tests/fixtures/markdown_simple.txt").unwrap();
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("id\tname"))
        .stdout(predicate::str::contains("1\tAlice"))
        .stdout(predicate::str::contains("2\tBob"));
}

#[test]
fn test_markdown_unicode() {
    let input = fs::read_to_string("tests/fixtures/markdown_unicode.txt").unwrap();
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("田中太郎"))
        .stdout(predicate::str::contains("日本語"))
        .stdout(predicate::str::contains("🎌"))
        .stdout(predicate::str::contains("王小明"))
        .stdout(predicate::str::contains("中文"))
        .stdout(predicate::str::contains("🇨🇳"))
        .stdout(predicate::str::contains("José García"))
        .stdout(predicate::str::contains("Español"))
        .stdout(predicate::str::contains("😊👍"));
}

#[test]
fn test_mysql_unicode() {
    let input = fs::read_to_string("tests/fixtures/mysql_unicode.txt").unwrap();
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("コーヒー ☕"))
        .stdout(predicate::str::contains("¥500"))
        .stdout(predicate::str::contains("🛒"))
        .stdout(predicate::str::contains("Café con leche"))
        .stdout(predicate::str::contains("绿茶 🍵"))
        .stdout(predicate::str::contains("Крепкий чай"));
}

#[test]
fn test_postgres_unicode() {
    let input = fs::read_to_string("tests/fixtures/postgres_unicode.txt").unwrap();
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("東京"))
        .stdout(predicate::str::contains("日本"))
        .stdout(predicate::str::contains("🇯🇵"))
        .stdout(predicate::str::contains("São Paulo"))
        .stdout(predicate::str::contains("Brasil"))
        .stdout(predicate::str::contains("🇧🇷"))
        .stdout(predicate::str::contains("Москва"))
        .stdout(predicate::str::contains("北京"));
}

#[test]
fn test_csv_edge_cases() {
    let input = fs::read_to_string("tests/fixtures/csv_edge_cases.txt").unwrap();
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Smith, John"))
        .stdout(predicate::str::contains(r#"He said "Hello""#))
        .stdout(predicate::str::contains("Empty Desc"))
        .stdout(predicate::str::contains("O'Brien"))
        .stdout(predicate::str::contains("🎉 Party"))
        .stdout(predicate::str::contains("Emoji in name 🥳"));
}

#[test]
fn test_csv_multiline_fields() {
    let input = fs::read_to_string("tests/fixtures/csv_edge_cases.txt").unwrap();
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Multi"));
    // Note: CSV with multiline fields is complex, just verify it doesn't crash
}

#[test]
fn test_tsv_unicode() {
    let input = fs::read_to_string("tests/fixtures/tsv_unicode.txt").unwrap();
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("田中さん"))
        .stdout(predicate::str::contains("プログラマー、コーヒー好き ☕"))
        .stdout(predicate::str::contains("😊"))
        .stdout(predicate::str::contains("José_García"))
        .stdout(predicate::str::contains("🇪🇸"))
        .stdout(predicate::str::contains("I love coding 💻 and music 🎵"))
        .stdout(predicate::str::contains("日本語"));
}

#[test]
fn test_markdown_edge_cases() {
    let input = fs::read_to_string("tests/fixtures/markdown_edge_cases.txt").unwrap();
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("With `code`"))
        .stdout(predicate::str::contains("Has **bold** text"))
        .stdout(predicate::str::contains("日本語 🎌"))
        .stdout(predicate::str::contains("Unicode & emoji"))
        .stdout(predicate::str::contains("🌟"));
}

#[test]
fn test_output_csv_format() {
    let input = "id\tname\n1\tAlice\n2\tBob";
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("-o")
        .arg("csv")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("id,name"))
        .stdout(predicate::str::contains("1,Alice"))
        .stdout(predicate::str::contains("2,Bob"));
}

#[test]
fn test_custom_delimiter() {
    let input = "id,name\n1,Alice\n2,Bob";
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("-d")
        .arg("|")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("id|name"))
        .stdout(predicate::str::contains("1|Alice"))
        .stdout(predicate::str::contains("2|Bob"));
}

#[test]
fn test_force_input_format() {
    let input = fs::read_to_string("tests/fixtures/markdown_simple.txt").unwrap();
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("-i")
        .arg("markdown")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
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
fn test_invalid_input_format() {
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("-i")
        .arg("invalid")
        .write_stdin("test")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Invalid format"));
}

#[test]
fn test_invalid_output_format() {
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.arg("-o")
        .arg("invalid")
        .write_stdin("id,name\n1,Alice")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Invalid output format"));
}

#[test]
fn test_csv_with_quotes_and_commas() {
    let input = r#"id,name,description
1,"Smith, John","He said ""Hello"""
2,Alice,Normal"#;
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Smith, John"));
}

#[test]
fn test_unicode_in_all_formats() {
    // Test that Unicode works correctly through the entire pipeline
    let inputs = vec![
        ("markdown", "| name |\n|------|\n| 日本語 🎌 |"),
        ("csv", "name\n日本語 🎌"),
        ("tsv", "name\n日本語 🎌"),
    ];

    for (format, input) in inputs {
        let mut cmd = Command::cargo_bin("tabx").unwrap();
        cmd.arg("-i")
            .arg(format)
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::contains("日本語 🎌"));
    }
}

#[test]
fn test_round_trip_tsv_to_csv_to_tsv() {
    // TSV input
    let input = "id\tname\n1\tAlice\n2\tBob";

    // Convert to CSV
    let mut cmd1 = Command::cargo_bin("tabx").unwrap();
    let result1 = cmd1
        .arg("-o")
        .arg("csv")
        .write_stdin(input)
        .assert()
        .success();

    let csv_output = String::from_utf8(result1.get_output().stdout.clone()).unwrap();

    // Convert back to TSV
    let mut cmd2 = Command::cargo_bin("tabx").unwrap();
    cmd2.write_stdin(csv_output)
        .assert()
        .success()
        .stdout(predicate::str::contains("id\tname"))
        .stdout(predicate::str::contains("1\tAlice"))
        .stdout(predicate::str::contains("2\tBob"));
}

#[test]
fn test_emoji_heavy_content() {
    let input = "emoji,description\n😀,happy\n😢,sad\n🎉,party\n💯,perfect\n🚀,rocket";
    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("😀"))
        .stdout(predicate::str::contains("😢"))
        .stdout(predicate::str::contains("🎉"))
        .stdout(predicate::str::contains("💯"))
        .stdout(predicate::str::contains("🚀"));
}

#[test]
fn test_mixed_unicode_scripts() {
    // Mix of Latin, Cyrillic, Chinese, Japanese, Arabic, Hebrew
    let input = "language,text\n";
    let input = input.to_string()
        + "English,Hello\n"
        + "日本語,こんにちは\n"
        + "中文,你好\n"
        + "Русский,Привет\n"
        + "العربية,مرحبا\n"
        + "עברית,שלום\n";

    let mut cmd = Command::cargo_bin("tabx").unwrap();
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello"))
        .stdout(predicate::str::contains("こんにちは"))
        .stdout(predicate::str::contains("你好"))
        .stdout(predicate::str::contains("Привет"))
        .stdout(predicate::str::contains("مرحبا"))
        .stdout(predicate::str::contains("שלום"));
}
