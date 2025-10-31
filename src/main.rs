use clap::{CommandFactory, Parser as ClapParser, Subcommand};
use clap_complete::{generate, Shell};
use std::io::{self, BufWriter, Read};
use std::process;
use table_extractor::detector::detect_format;
use table_extractor::parser::{CsvParser, MarkdownParser, MySqlParser, PostgresParser};
use table_extractor::writer::{CsvWriter, TsvWriter};
use table_extractor::{Format, Parser, Writer};

/// Maximum input size: 100 MB
/// Prevents DoS attacks via unbounded memory allocation
const MAX_INPUT_SIZE: usize = 100 * 1024 * 1024;

#[derive(ClapParser)]
#[command(name = "tabx")]
#[command(author = "Agustin Camino")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Convert various tabular data formats into TSV or CSV", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Force input format detection (auto, markdown, mysql, postgres, csv, tsv)
    #[arg(short = 'i', long = "input-format", default_value = "auto")]
    input_format: String,

    /// Output format (tsv, csv)
    #[arg(short = 'o', long = "output-format", default_value = "tsv")]
    output_format: String,

    /// Custom output delimiter (overrides --output-format)
    #[arg(short = 'd', long = "delimiter")]
    delimiter: Option<char>,

    /// Custom input delimiter for CSV/TSV
    #[arg(long = "input-delimiter")]
    input_delimiter: Option<char>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate shell completions
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

/// Validates that a delimiter character is safe for CSV/TSV parsing
fn validate_delimiter(c: char, delimiter_type: &str) -> Result<u8, String> {
    // Reject control characters except tab (which is valid for TSV)
    if c.is_control() && c != '\t' {
        return Err(format!(
            "Invalid {} delimiter '{}': control characters not allowed (except tab for TSV)",
            delimiter_type,
            c.escape_default()
        ));
    }

    // Ensure ASCII to prevent truncation issues when casting to u8
    if !c.is_ascii() {
        return Err(format!(
            "Invalid {} delimiter '{}': must be ASCII character",
            delimiter_type, c
        ));
    }

    // Reject common problematic characters
    if matches!(c, '\n' | '\r' | '\0') {
        return Err(format!(
            "Invalid {} delimiter '{}': newline and null characters not allowed",
            delimiter_type,
            c.escape_default()
        ));
    }

    Ok(c as u8)
}

fn main() {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(command) = cli.command {
        match command {
            Commands::Completions { shell } => {
                let mut cmd = Cli::command();
                generate(shell, &mut cmd, "tabx", &mut io::stdout());
                return;
            }
        }
    }

    // Validate custom delimiters early
    if let Some(delimiter) = cli.input_delimiter {
        if let Err(e) = validate_delimiter(delimiter, "input") {
            eprintln!("tabx: error: {}", e);
            process::exit(2);
        }
    }

    if let Some(delimiter) = cli.delimiter {
        if let Err(e) = validate_delimiter(delimiter, "output") {
            eprintln!("tabx: error: {}", e);
            process::exit(2);
        }
    }

    // Default behavior: convert table format
    convert_table(cli);
}

fn convert_table(cli: Cli) {
    // Read input from stdin with size limit to prevent DoS
    let mut input = String::new();
    let stdin = io::stdin();
    let bytes_read = match stdin
        .take(MAX_INPUT_SIZE as u64 + 1)
        .read_to_string(&mut input)
    {
        Ok(n) => n,
        Err(e) => {
            eprintln!("tabx: error: Failed to read from stdin: {}", e);
            process::exit(3);
        }
    };

    if bytes_read > MAX_INPUT_SIZE {
        eprintln!(
            "tabx: error: Input exceeds maximum size of {} MB",
            MAX_INPUT_SIZE / 1024 / 1024
        );
        process::exit(3);
    }

    // Handle empty input
    if input.trim().is_empty() {
        process::exit(0);
    }

    // Detect or parse input format
    let format = if cli.input_format == "auto" {
        detect_format(&input)
    } else {
        match cli.input_format.parse::<Format>() {
            Ok(fmt) => fmt,
            Err(err) => {
                eprintln!("tabx: error: {}", err);
                process::exit(2);
            }
        }
    };

    // Select the appropriate parser
    let table = match format {
        Format::Markdown => {
            let parser = MarkdownParser;
            parser.parse(&input)
        }
        Format::MySQL => {
            let parser = MySqlParser;
            parser.parse(&input)
        }
        Format::PostgreSQL => {
            let parser = PostgresParser;
            parser.parse(&input)
        }
        Format::CSV => {
            let delimiter = cli.input_delimiter.unwrap_or(',') as u8;
            let parser = CsvParser::new(delimiter);
            parser.parse(&input)
        }
        Format::TSV => {
            let delimiter = cli.input_delimiter.unwrap_or('\t') as u8;
            let parser = CsvParser::new(delimiter);
            parser.parse(&input)
        }
    };

    let table = match table {
        Ok(t) => t,
        Err(e) => {
            eprintln!("tabx: error: {}", e);
            process::exit(1);
        }
    };

    // Early delimiter conflict detection for TSV/custom delimiters
    // Check if output delimiter exists in data BEFORE writing
    // This provides fast feedback instead of failing after writing starts
    let output_delimiter = if let Some(delimiter) = cli.delimiter {
        Some(delimiter)
    } else if cli.output_format == "tsv" {
        Some('\t')
    } else {
        None // CSV handles escaping, no need to check
    };

    if let Some(delimiter) = output_delimiter {
        // Check headers
        for header in &table.headers {
            if header.contains(delimiter) {
                eprintln!(
                    "tabx: error: Header '{}' contains delimiter character '{}'. Use -o csv for proper escaping.",
                    header, delimiter
                );
                process::exit(1);
            }
        }

        // Check rows
        for (idx, row) in table.rows.iter().enumerate() {
            for cell in row {
                if cell.contains(delimiter) {
                    eprintln!(
                        "tabx: error: Row {} contains delimiter character '{}' in data. Use -o csv for proper escaping.",
                        idx + 1, delimiter
                    );
                    process::exit(1);
                }
            }
        }
    }

    // Select the appropriate writer
    // Use BufWriter for 3-6x performance improvement on large outputs
    let mut stdout = BufWriter::new(io::stdout());
    let result = if let Some(delimiter) = cli.delimiter {
        let writer = TsvWriter::new(delimiter);
        writer.write(&table, &mut stdout)
    } else {
        match cli.output_format.as_str() {
            "tsv" => {
                let writer = TsvWriter::default();
                writer.write(&table, &mut stdout)
            }
            "csv" => {
                let writer = CsvWriter::new();
                writer.write(&table, &mut stdout)
            }
            _ => {
                eprintln!(
                    "tabx: error: Invalid output format '{}'. Valid formats: tsv, csv",
                    cli.output_format
                );
                process::exit(2);
            }
        }
    };

    if let Err(e) = result {
        eprintln!("tabx: error: {}", e);
        process::exit(1);
    }
}
