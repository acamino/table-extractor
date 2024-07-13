use clap::Parser as ClapParser;
use std::io::{self, Read};
use std::process;
use table_extractor::detector::detect_format;
use table_extractor::parser::{CsvParser, MarkdownParser, MySqlParser, PostgresParser};
use table_extractor::writer::{CsvWriter, TsvWriter};
use table_extractor::{Format, Parser, Writer};

#[derive(ClapParser)]
#[command(name = "tabx")]
#[command(author = "Agustin Camino")]
#[command(version = "1.0.0")]
#[command(about = "Convert various tabular data formats into TSV or CSV", long_about = None)]
struct Cli {
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

fn main() {
    let cli = Cli::parse();

    // Read input from stdin
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("tabx: error: Failed to read from stdin: {}", e);
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

    // Select the appropriate writer
    let mut stdout = io::stdout();
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
