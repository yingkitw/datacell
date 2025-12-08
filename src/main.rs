#![allow(dead_code)] // Modules expose APIs for library use

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use rmcp::{transport::stdio, ServiceExt};

mod excel;
mod csv_handler;
mod converter;
mod formula;
mod mcp;
mod operations;
mod columnar;

use excel::ExcelHandler;
use csv_handler::{CsvHandler, CellRange};
use converter::Converter;
use formula::FormulaEvaluator;
use mcp::DatacellMcpServer;
use operations::{DataOperations, SortOrder, JoinType, AggFunc};
use columnar::{ParquetHandler, AvroHandler};

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
    /// Append data from one file to another
    Append {
        /// Source file to read data from
        #[arg(short, long)]
        source: String,
        /// Target file to append data to
        #[arg(short, long)]
        target: String,
    },
    /// List sheets in an Excel/ODS file
    Sheets {
        /// Input file path
        #[arg(short, long)]
        input: String,
    },
    /// Read all sheets from Excel file
    ReadAll {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output format (csv or json)
        #[arg(short = 'f', long, default_value = "json")]
        format: OutputFormat,
    },
    /// Write data to a specific cell range
    WriteRange {
        /// Input CSV file
        #[arg(short, long)]
        input: String,
        /// Output file
        #[arg(short, long)]
        output: String,
        /// Starting cell (e.g., "B2")
        #[arg(short, long)]
        start: String,
    },
    /// Select specific columns
    Select {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Column names (comma-separated)
        #[arg(short, long)]
        columns: String,
    },
    /// Get first n rows
    Head {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Number of rows
        #[arg(short, long, default_value = "10")]
        n: usize,
        /// Output format
        #[arg(short = 'f', long, default_value = "csv")]
        format: OutputFormat,
    },
    /// Get last n rows
    Tail {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Number of rows
        #[arg(short, long, default_value = "10")]
        n: usize,
        /// Output format
        #[arg(short = 'f', long, default_value = "csv")]
        format: OutputFormat,
    },
    /// Sample random rows
    Sample {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Number of rows to sample
        #[arg(short, long)]
        n: usize,
        /// Random seed
        #[arg(long)]
        seed: Option<u64>,
        /// Output format
        #[arg(short = 'f', long, default_value = "csv")]
        format: OutputFormat,
    },
    /// Describe statistics for numeric columns
    Describe {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output format
        #[arg(short = 'f', long, default_value = "csv")]
        format: OutputFormat,
    },
    /// Count unique values in a column
    ValueCounts {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Column name or index
        #[arg(short, long)]
        column: String,
        /// Output format
        #[arg(short = 'f', long, default_value = "csv")]
        format: OutputFormat,
    },
    /// Group by column and aggregate
    Groupby {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Column to group by
        #[arg(short, long)]
        by: String,
        /// Aggregations (format: "sum:col1,count:col2,mean:col3")
        #[arg(short, long)]
        agg: String,
    },
    /// Join two files
    Join {
        /// Left file path
        #[arg(short, long)]
        left: String,
        /// Right file path
        #[arg(short, long)]
        right: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Column to join on
        #[arg(long)]
        on: String,
        /// Join type (inner, left, right, outer)
        #[arg(long, default_value = "inner")]
        how: String,
    },
    /// Concatenate multiple files
    Concat {
        /// Input files (comma-separated)
        #[arg(short, long)]
        inputs: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
    },
    /// Fill empty values
    Fillna {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Value to fill
        #[arg(short, long)]
        value: String,
    },
    /// Drop rows with empty values
    Dropna {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
    },
    /// Drop columns
    Drop {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Column names to drop (comma-separated)
        #[arg(short, long)]
        columns: String,
    },
    /// Rename columns
    Rename {
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Old column name
        #[arg(long)]
        from: String,
        /// New column name
        #[arg(long)]
        to: String,
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
            } else if input.ends_with(".ods") {
                let handler = ExcelHandler::new();
                let data = handler.read_ods_data(&input, sheet.as_deref())?;
                format_output(&data, &format, &ops)
            } else if input.ends_with(".parquet") {
                let handler = ParquetHandler::new();
                let data = handler.read_with_headers(&input)?;
                format_output(&data, &format, &ops)
            } else if input.ends_with(".avro") {
                let handler = AvroHandler::new();
                let data = handler.read_with_headers(&input)?;
                format_output(&data, &format, &ops)
            } else {
                anyhow::bail!("Unsupported file format. Supported: .csv, .xls, .xlsx, .ods, .parquet, .avro");
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
                } else if output.ends_with(".parquet") {
                    let data = read_csv_data(&csv_path)?;
                    let handler = ParquetHandler::new();
                    handler.write(&output, &data, None)?;
                    println!("Written Parquet to {}", output);
                } else if output.ends_with(".avro") {
                    let data = read_csv_data(&csv_path)?;
                    let handler = AvroHandler::new();
                    handler.write(&output, &data, None)?;
                    println!("Written Avro to {}", output);
                } else {
                    anyhow::bail!("Unsupported output format. Supported: .csv, .xls, .xlsx, .parquet, .avro");
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
        Commands::Append { source, target } => {
            let data = read_csv_data(&source)?;
            let handler = CsvHandler::new();
            handler.append_records(&target, &data)?;
            println!("Appended {} rows from {} to {}", data.len(), source, target);
        }
        Commands::Sheets { input } => {
            let handler = ExcelHandler::new();
            let sheets = if input.ends_with(".ods") {
                handler.list_ods_sheets(&input)?
            } else {
                handler.list_sheets(&input)?
            };
            println!("Sheets in {}:", input);
            for (i, sheet) in sheets.iter().enumerate() {
                println!("  {}. {}", i + 1, sheet);
            }
        }
        Commands::ReadAll { input, format } => {
            let handler = ExcelHandler::new();
            let all_sheets = handler.read_all_sheets(&input)?;
            let ops = DataOperations::new();
            
            match format {
                OutputFormat::Json => {
                    let json = serde_json::to_string_pretty(&all_sheets)?;
                    println!("{}", json);
                }
                OutputFormat::Csv | OutputFormat::Markdown => {
                    for (sheet_name, data) in &all_sheets {
                        println!("=== {} ===", sheet_name);
                        let output = format_output(data, &format, &ops);
                        println!("{}", output);
                        println!();
                    }
                }
            }
        }
        Commands::WriteRange { input, output, start } => {
            let data = read_csv_data(&input)?;
            let range = CellRange::parse(&start)?;
            
            if output.ends_with(".csv") {
                let handler = CsvHandler::new();
                handler.write_range(&output, &data, range.start_row, range.start_col)?;
            } else if output.ends_with(".xlsx") {
                let handler = ExcelHandler::new();
                handler.write_range(&output, &data, range.start_row as u32, range.start_col as u16, None)?;
            } else {
                anyhow::bail!("Unsupported output format");
            }
            println!("Wrote {} rows starting at {} to {}", data.len(), start, output);
        }
        Commands::Select { input, output, columns } => {
            let data = read_any_file(&input)?;
            let ops = DataOperations::new();
            let col_names: Vec<&str> = columns.split(',').map(|s| s.trim()).collect();
            let result = ops.select_columns_by_name(&data, &col_names)?;
            write_csv_data(&output, &result)?;
            println!("Selected {} columns, saved to {}", col_names.len(), output);
        }
        Commands::Head { input, n, format } => {
            let data = read_any_file(&input)?;
            let ops = DataOperations::new();
            let result = ops.head(&data, n + 1); // +1 for header
            println!("{}", format_output(&result, &format, &ops));
        }
        Commands::Tail { input, n, format } => {
            let data = read_any_file(&input)?;
            let ops = DataOperations::new();
            // Keep header + last n rows
            let mut result = vec![data[0].clone()];
            result.extend(ops.tail(&data[1..].to_vec(), n));
            println!("{}", format_output(&result, &format, &ops));
        }
        Commands::Sample { input, n, seed, format } => {
            let data = read_any_file(&input)?;
            let ops = DataOperations::new();
            // Keep header + sample from data rows
            let mut result = vec![data[0].clone()];
            result.extend(ops.sample(&data[1..].to_vec(), n, seed));
            println!("{}", format_output(&result, &format, &ops));
        }
        Commands::Describe { input, format } => {
            let data = read_any_file(&input)?;
            let ops = DataOperations::new();
            let result = ops.describe(&data)?;
            println!("{}", format_output(&result, &format, &ops));
        }
        Commands::ValueCounts { input, column, format } => {
            let data = read_any_file(&input)?;
            let ops = DataOperations::new();
            let col_idx = find_column_index(&data, &column)?;
            let result = ops.value_counts(&data, col_idx);
            println!("{}", format_output(&result, &format, &ops));
        }
        Commands::Groupby { input, output, by, agg } => {
            let data = read_any_file(&input)?;
            let ops = DataOperations::new();
            let group_col = find_column_index(&data, &by)?;
            
            // Parse aggregations: "sum:col1,count:col2"
            let aggregations: Vec<(usize, AggFunc)> = agg.split(',')
                .map(|s| {
                    let parts: Vec<&str> = s.trim().split(':').collect();
                    if parts.len() != 2 {
                        anyhow::bail!("Invalid aggregation format: {}. Use 'func:column'", s);
                    }
                    let func = AggFunc::from_str(parts[0])?;
                    let col_idx = find_column_index(&data, parts[1])?;
                    Ok((col_idx, func))
                })
                .collect::<Result<Vec<_>>>()?;
            
            let result = ops.groupby(&data, group_col, &aggregations)?;
            write_csv_data(&output, &result)?;
            println!("Grouped by '{}' with {} aggregations, saved to {}", by, aggregations.len(), output);
        }
        Commands::Join { left, right, output, on, how } => {
            let left_data = read_any_file(&left)?;
            let right_data = read_any_file(&right)?;
            let ops = DataOperations::new();
            
            let left_col = find_column_index(&left_data, &on)?;
            let right_col = find_column_index(&right_data, &on)?;
            let join_type = JoinType::from_str(&how)?;
            
            let result = ops.join(&left_data, &right_data, left_col, right_col, join_type)?;
            write_csv_data(&output, &result)?;
            println!("Joined {} rows, saved to {}", result.len(), output);
        }
        Commands::Concat { inputs, output } => {
            let ops = DataOperations::new();
            let files: Vec<&str> = inputs.split(',').map(|s| s.trim()).collect();
            
            let datasets: Vec<Vec<Vec<String>>> = files.iter()
                .map(|f| read_any_file(f))
                .collect::<Result<Vec<_>>>()?;
            
            let result = ops.concat(&datasets.iter().map(|d| d.clone()).collect::<Vec<_>>());
            write_csv_data(&output, &result)?;
            println!("Concatenated {} files ({} rows), saved to {}", files.len(), result.len(), output);
        }
        Commands::Fillna { input, output, value } => {
            let mut data = read_any_file(&input)?;
            let ops = DataOperations::new();
            ops.fillna(&mut data, &value);
            write_csv_data(&output, &data)?;
            println!("Filled empty values with '{}', saved to {}", value, output);
        }
        Commands::Dropna { input, output } => {
            let data = read_any_file(&input)?;
            let ops = DataOperations::new();
            let result = ops.dropna(&data);
            let dropped = data.len() - result.len();
            write_csv_data(&output, &result)?;
            println!("Dropped {} rows with empty values, saved to {}", dropped, output);
        }
        Commands::Drop { input, output, columns } => {
            let data = read_any_file(&input)?;
            let ops = DataOperations::new();
            let col_names: Vec<&str> = columns.split(',').map(|s| s.trim()).collect();
            let col_indices: Vec<usize> = col_names.iter()
                .map(|name| find_column_index(&data, name))
                .collect::<Result<Vec<_>>>()?;
            let result = ops.drop_columns(&data, &col_indices);
            write_csv_data(&output, &result)?;
            println!("Dropped {} columns, saved to {}", col_names.len(), output);
        }
        Commands::Rename { input, output, from, to } => {
            let mut data = read_any_file(&input)?;
            let ops = DataOperations::new();
            ops.rename_columns(&mut data, &[(&from, &to)])?;
            write_csv_data(&output, &data)?;
            println!("Renamed '{}' to '{}', saved to {}", from, to, output);
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

/// Read any supported file format into Vec<Vec<String>>
fn read_any_file(path: &str) -> Result<Vec<Vec<String>>> {
    if path.ends_with(".csv") {
        read_csv_data(path)
    } else if path.ends_with(".xlsx") || path.ends_with(".xls") {
        let handler = ExcelHandler::new();
        let content = handler.read_with_sheet(path, None)?;
        Ok(content.lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.split(',').map(|s| s.to_string()).collect())
            .collect())
    } else if path.ends_with(".parquet") {
        let handler = ParquetHandler::new();
        handler.read_with_headers(path)
    } else if path.ends_with(".avro") {
        let handler = AvroHandler::new();
        handler.read_with_headers(path)
    } else if path.ends_with(".ods") {
        let handler = ExcelHandler::new();
        handler.read_ods_data(path, None)
    } else {
        anyhow::bail!("Unsupported file format: {}", path)
    }
}

/// Find column index by name or number
fn find_column_index(data: &[Vec<String>], column: &str) -> Result<usize> {
    // Try as number first
    if let Ok(idx) = column.parse::<usize>() {
        return Ok(idx);
    }
    
    // Try as column letter (A, B, C, ...)
    if column.chars().all(|c| c.is_ascii_alphabetic()) && column.len() <= 2 {
        return parse_column_ref(column);
    }
    
    // Find by name in header
    if let Some(header) = data.first() {
        if let Some(pos) = header.iter().position(|h| h == column) {
            return Ok(pos);
        }
    }
    
    anyhow::bail!("Column '{}' not found", column)
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

