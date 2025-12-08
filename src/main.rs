use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use rmcp::{transport::stdio, ServiceExt};

mod excel;
mod csv_handler;
mod converter;
mod formula;
mod mcp;
mod operations;

use excel::ExcelHandler;
use csv_handler::{CsvHandler, CellRange};
use converter::Converter;
use formula::FormulaEvaluator;
use mcp::DatacellMcpServer;
use operations::{DataOperations, SortOrder};

#[derive(Parser)]
#[command(name = "datacell")]
#[command(about = "A CLI tool for reading, writing, converting XLS and CSV files with formula support")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Output format for read command
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum OutputFormat {
    #[default]
    Csv,
    Json,
    Markdown,
}

#[derive(Subcommand)]
enum Commands {
    /// Read data from a file
    Read {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Sheet name (for Excel files)
        #[arg(short, long)]
        sheet: Option<String>,
        /// Cell range to read (e.g., "A1:C10")
        #[arg(short, long)]
        range: Option<String>,
        /// Output format (csv or json)
        #[arg(short = 'f', long, default_value = "csv")]
        format: OutputFormat,
    },
    /// Write data to a file
    Write {
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Input CSV file path (for CSV output)
        #[arg(short, long)]
        csv: Option<String>,
        /// Sheet name (for Excel files)
        #[arg(short, long)]
        sheet: Option<String>,
    },
    /// Convert between file formats
    Convert {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Sheet name (for Excel input)
        #[arg(short, long)]
        sheet: Option<String>,
    },
    /// Apply formulas to a file
    Formula {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Formula to apply (e.g., "SUM(A1:A10)" or "A1+B1")
        #[arg(short, long)]
        formula: String,
        /// Target cell (e.g., "C1")
        #[arg(short, long)]
        cell: String,
        /// Sheet name (for Excel files)
        #[arg(short, long)]
        sheet: Option<String>,
    },
    /// Start MCP server (stdio transport)
    Serve,
    /// Sort rows by column
    Sort {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Column to sort by (e.g., "A" or "0" for first column)
        #[arg(short, long)]
        column: String,
        /// Sort in descending order
        #[arg(short, long, default_value = "false")]
        descending: bool,
    },
    /// Filter rows by condition
    Filter {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Column to filter (e.g., "A" or "0")
        #[arg(short, long)]
        column: String,
        /// Operator (=, !=, >, <, >=, <=, contains, starts_with, ends_with)
        #[arg(long, default_value = "=")]
        op: String,
        /// Value to compare against
        #[arg(short, long)]
        value: String,
    },
    /// Find and replace values
    Replace {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Text to find
        #[arg(long)]
        find: String,
        /// Text to replace with
        #[arg(long)]
        replace: String,
    },
    /// Remove duplicate rows
    Dedupe {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
    },
    /// Transpose data (rows to columns)
    Transpose {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Read { input, sheet, range, format } => {
            let ops = DataOperations::new();
            let output = if input.ends_with(".csv") {
                let handler = CsvHandler::new();
                let data = if let Some(range_str) = &range {
                    let cell_range = CellRange::parse(range_str)?;
                    handler.read_range(&input, &cell_range)?
                } else {
                    read_csv_data(&input)?
                };
                format_output(&data, &format, &ops)
            } else if input.ends_with(".xls") || input.ends_with(".xlsx") {
                let handler = ExcelHandler::new();
                let data = if let Some(range_str) = &range {
                    let cell_range = CellRange::parse(range_str)?;
                    handler.read_range(&input, &cell_range, sheet.as_deref())?
                } else {
                    // Read all data
                    let csv_str = handler.read_with_sheet(&input, sheet.as_deref())?;
                    csv_str.lines()
                        .filter(|l| !l.is_empty())
                        .map(|l| l.split(',').map(|s| s.to_string()).collect())
                        .collect()
                };
                format_output(&data, &format, &ops)
            } else {
                anyhow::bail!("Unsupported file format. Supported: .csv, .xls, .xlsx");
            };
            println!("{}", output);
        }
        Commands::Write { output, csv, sheet } => {
            if let Some(csv_path) = csv {
                if output.ends_with(".csv") {
                    let handler = CsvHandler::new();
                    handler.write_from_csv(&csv_path, &output)?;
                    println!("Written CSV to {}", output);
                } else if output.ends_with(".xls") || output.ends_with(".xlsx") {
                    let handler = ExcelHandler::new();
                    handler.write_from_csv(&csv_path, &output, sheet.as_deref())?;
                    println!("Written Excel to {}", output);
                } else {
                    anyhow::bail!("Unsupported output format. Supported: .csv, .xls, .xlsx");
                }
            } else {
                anyhow::bail!("Please provide --csv input file");
            }
        }
        Commands::Convert { input, output, sheet } => {
            let converter = Converter::new();
            converter.convert(&input, &output, sheet.as_deref())?;
            println!("Converted {} to {}", input, output);
        }
        Commands::Formula { input, output, formula, cell, sheet } => {
            let evaluator = FormulaEvaluator::new();
            if input.ends_with(".csv") {
                evaluator.apply_to_csv(&input, &output, &formula, &cell)?;
                println!("Applied formula {} to {} in {}", formula, cell, output);
            } else if input.ends_with(".xls") || input.ends_with(".xlsx") {
                evaluator.apply_to_excel(&input, &output, &formula, &cell, sheet.as_deref())?;
                println!("Applied formula {} to {} in {}", formula, cell, output);
            } else {
                anyhow::bail!("Unsupported file format. Supported: .csv, .xls, .xlsx");
            }
        }
        Commands::Serve => {
            let service = DatacellMcpServer::new().serve(stdio()).await?;
            service.waiting().await?;
        }
        Commands::Sort { input, output, column, descending } => {
            let mut data = read_csv_data(&input)?;
            let col_idx = parse_column_ref(&column)?;
            let order = if descending { SortOrder::Descending } else { SortOrder::Ascending };
            
            let ops = DataOperations::new();
            ops.sort_by_column(&mut data, col_idx, order)?;
            
            write_csv_data(&output, &data)?;
            println!("Sorted by column {} and saved to {}", column, output);
        }
        Commands::Filter { input, output, column, op, value } => {
            let data = read_csv_data(&input)?;
            let col_idx = parse_column_ref(&column)?;
            
            let ops = DataOperations::new();
            let filtered = ops.filter_rows(&data, col_idx, &op, &value)?;
            
            write_csv_data(&output, &filtered)?;
            println!("Filtered {} rows to {}", filtered.len(), output);
        }
        Commands::Replace { input, output, find, replace } => {
            let mut data = read_csv_data(&input)?;
            
            let ops = DataOperations::new();
            let count = ops.find_replace(&mut data, &find, &replace, None)?;
            
            write_csv_data(&output, &data)?;
            println!("Replaced {} occurrences, saved to {}", count, output);
        }
        Commands::Dedupe { input, output } => {
            let data = read_csv_data(&input)?;
            let original_len = data.len();
            
            let ops = DataOperations::new();
            let deduped = ops.deduplicate(&data);
            
            write_csv_data(&output, &deduped)?;
            println!("Removed {} duplicates, {} rows saved to {}", original_len - deduped.len(), deduped.len(), output);
        }
        Commands::Transpose { input, output } => {
            let data = read_csv_data(&input)?;
            
            let ops = DataOperations::new();
            let transposed = ops.transpose(&data);
            
            write_csv_data(&output, &transposed)?;
            println!("Transposed {} rows x {} cols to {} cols x {} rows, saved to {}", 
                data.len(), data.first().map(|r| r.len()).unwrap_or(0),
                transposed.len(), transposed.first().map(|r| r.len()).unwrap_or(0),
                output);
        }
    }

    Ok(())
}

/// Read CSV file into Vec<Vec<String>>
fn read_csv_data(path: &str) -> Result<Vec<Vec<String>>> {
    use csv::ReaderBuilder;
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path)?;
    
    let mut data = Vec::new();
    for record in reader.records() {
        let record = record?;
        data.push(record.iter().map(|s| s.to_string()).collect());
    }
    Ok(data)
}

/// Write Vec<Vec<String>> to CSV file
fn write_csv_data(path: &str, data: &[Vec<String>]) -> Result<()> {
    use csv::WriterBuilder;
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    
    for row in data {
        writer.write_record(row)?;
    }
    writer.flush()?;
    Ok(())
}

/// Format data for output
fn format_output(data: &[Vec<String>], format: &OutputFormat, ops: &DataOperations) -> String {
    match format {
        OutputFormat::Csv => data.iter().map(|row| row.join(",")).collect::<Vec<_>>().join("\n"),
        OutputFormat::Json => serde_json::to_string_pretty(data).unwrap_or_default(),
        OutputFormat::Markdown => ops.to_markdown(data),
    }
}

/// Parse column reference (e.g., "A" -> 0, "B" -> 1, "0" -> 0)
fn parse_column_ref(col: &str) -> Result<usize> {
    // Try as number first
    if let Ok(idx) = col.parse::<usize>() {
        return Ok(idx);
    }
    
    // Parse as letter column (A=0, B=1, etc.)
    let col = col.trim().to_uppercase();
    if col.is_empty() {
        anyhow::bail!("Empty column reference");
    }
    
    let mut index = 0usize;
    for ch in col.chars() {
        if !ch.is_ascii_alphabetic() {
            anyhow::bail!("Invalid column reference: {}", col);
        }
        index = index * 26 + (ch as usize - 'A' as usize + 1);
    }
    Ok(index - 1)
}

