# tabx

[![CI](https://github.com/acamino/table-extractor/actions/workflows/ci.yml/badge.svg)](https://github.com/acamino/table-extractor/actions)
[![Version](https://img.shields.io/crates/v/table-extractor.svg)](https://crates.io/crates/table-extractor)

A simple, fast command-line tool to convert between table formats (Markdown, MySQL, PostgreSQL, CSV, TSV) with automatic format detection.

## Installation

```bash
# Build from source
cargo build --release

# Install globally  
cargo install --path .
```

## Usage

```bash
# From file (recommended)
tabx data.csv
tabx input.txt -o csv > output.csv

# From stdin
pbpaste | tabx | pbcopy
cat data.txt | tabx -o csv > clean.csv

# Force input format
mysql -e "SELECT * FROM users" | tabx -i mysql
```

## Examples

```bash
# File input (like wc, cat, head, tail)
tabx data.csv > output.tsv
tabx -o csv users.txt > users.csv
tabx -i markdown README.md

# Database output to spreadsheet
mysql -e "SELECT * FROM companies" | tabx -o csv > companies.csv
psql -c "SELECT * FROM users" | tabx | pbcopy

# Pipeline composition
pbpaste | tabx | grep "active" | wc -l
tail -n +3 input.txt | tabx | head -10
```

## Supported Formats

| Format         | Auto-detection            | Example Source            |
|----------------|---------------------------|---------------------------|
| **MySQL**      | `+---+` borders           | `mysql -e "SELECT ..."`   |
| **PostgreSQL** | `----+----` separators    | `psql -c "SELECT ..."`    |
| **Markdown**   | `\|---\|` separator lines | Documentation tables      |
| **CSV**        | Comma-separated           | Data files, Excel exports |
| **TSV**        | Tab-separated             | Spreadsheet exports       |

Output formats: **TSV** (default), **CSV**, or custom delimiter.

## Command-line Options

```
Usage: tabx [OPTIONS] [FILE]

Arguments:
  [FILE]  Input file (reads from stdin if not provided)

Options:
  -i, --input-format <FORMAT>      Force input format (auto, markdown, mysql, postgres, csv, tsv)
  -o, --output-format <FORMAT>     Output format (tsv, csv) [default: tsv]
  -d, --delimiter <CHAR>           Custom output delimiter
      --input-delimiter <CHAR>     Custom input delimiter for CSV/TSV
  -h, --help                       Print help
  -V, --version                    Print version
```

**Delimiter Requirements:**
- Must be a single ASCII character
- Control characters are not allowed (except tab `\t` for TSV)
- Common valid delimiters: `,` (comma), `|` (pipe), `;` (semicolon), `:` (colon)

## Format Examples

### MySQL → TSV
```bash
mysql -e "SELECT id, name FROM users" | tabx
```
```
+----+-------+          id	name
| id | name  |    →     1	Alice
+----+-------+          2	Bob
|  1 | Alice |
|  2 | Bob   |
+----+-------+
```

### Markdown → CSV
```bash
cat docs.md | tabx -o csv
```
```
| API | Method  |        API,Method
|-----|---------|   →    /users,GET
| /users | GET  |        /posts,POST
| /posts | POST |
```

### PostgreSQL → TSV
```bash
psql -c "SELECT * FROM products" | tabx
```
```
 id | name          id	name
----+-------   →    1	Coffee
  1 | Coffee        2	Tea
  2 | Tea
```

## Development

```bash
git clone https://github.com/acamino/table-extractor
cd table-extractor
cargo build
cargo test
cargo install --path .
```
