# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands

```bash
# Build in debug mode
cargo build

# Build release binary
cargo build --release

# Run all tests
cargo test

# Run specific test
cargo test --test test_excel
cargo test test_read_xlsx

# Run with logging
RUST_LOG=debug cargo run -- read --input examples/data.csv

# Run single command
cargo run -- read --input examples/data.csv

# Generate shell completions
cargo run -- completions zsh
cargo run -- completions bash
```

## High-Level Architecture

datacell is a Rust CLI tool for spreadsheet manipulation (CSV, Excel, Parquet, Avro) with formula evaluation and pandas-style operations.

### Core Design Pattern: Trait-Based Handler System

The codebase uses a trait-based architecture for extensibility and testability:

**Key Traits** ([src/traits.rs](src/traits.rs)):
- `DataReader` - Read data from files
- `DataWriter` - Write data to files
- `FileHandler` - Combined read/write (auto-implemented for DataReader + DataWriter)
- `FormatDetector` - Detect file format from extension
- `SchemaProvider` - Get metadata (columns, row counts)
- `StreamingReader`/`StreamingWriter` - Process large files incrementally

**Handler Registry** ([src/handler_registry.rs](src/handler_registry.rs)):
- `HandlerRegistry` uses `FormatDetector` to select the appropriate handler
- Delegates to `CsvHandler`, `ExcelHandler`, `ParquetHandler`, or `AvroHandler`
- Used internally by `Converter` for format-agnostic operations

### Module Organization

```
src/
├── main.rs              # CLI entry point (clap-based)
├── cli.rs               # Command definitions + handler implementations
├── lib.rs               # Library exports (all public APIs)
├── converter.rs         # Format conversion (uses HandlerRegistry)
├── handler_registry.rs  # Format-based handler selection
├── traits.rs            # Core trait definitions
│
├── csv_handler.rs       # CSV operations
├── excel/               # Excel/ODS operations (calamine + rust_xlsxwriter)
│   ├── mod.rs
│   ├── reader.rs        # Read Excel sheets
│   ├── writer.rs        # Write Excel files
│   └── chart.rs         # Chart generation
├── columnar.rs          # Parquet + Avro operations
│
├── formula/             # Excel-like formula evaluation
│   ├── mod.rs
│   ├── parser.rs        # Parse cell references (A1, B2)
│   ├── evaluator.rs     # Main evaluation logic
│   └── functions.rs     # SUM, AVERAGE, VLOOKUP, IF, etc.
│
├── operations/          # Data manipulation (pandas-inspired)
│   ├── mod.rs
│   ├── types.rs         # AggFunc, JoinType, SortOrder
│   ├── core.rs          # sort, filter, replace, transpose
│   ├── pandas.rs        # head, tail, join, groupby, concat
│   ├── stats.rs         # describe, value_counts, corr
│   └── transform.rs     # query, mutate, astype, normalize
│
├── mcp.rs               # MCP server (rmcp-based)
├── config.rs            # .datacell.toml configuration
├── error.rs             # Error types with context
├── error_traits.rs      # Trait-based error handling
│
├── validation.rs        # Data validation framework
├── profiling.rs         # Data profiling and quality reports
├── quality.rs           # Data quality checks
├── lineage.rs           # Data lineage tracking
│
├── text_analysis.rs     # Text stats, sentiment, keywords
├── timeseries.rs        # Time series resampling, rolling windows
├── geospatial.rs        # Distance calculations
├── anomaly.rs           # Anomaly detection (zscore, IQR)
├── encryption.rs        # File encryption (XOR, AES256)
├── workflow.rs          # Pipeline/workflow execution
├── api.rs               # REST API server (placeholder)
├── plugins.rs           # Plugin function registry
└── streaming.rs         # Streaming data processing
```

### Data Flow

```
CLI/MCP Request
       │
       ▼
   cli.rs (Commands enum → DefaultCommandHandler)
       │
       ▼
┌─────────────────────────────────────────┐
│         Format-Agnostic Layer           │
│  (converter.rs, handler_registry.rs)    │
└─────────────────────────────────────────┘
       │
       ▼
┌──────────┬──────────┬──────────┬──────────┐
│ CsvHandler │ ExcelHandler │ ParquetHandler │ AvroHandler │
└──────────┴──────────┴──────────┴──────────┘
       │
       ▼
   File I/O
```

## Key Implementation Details

### Cell Reference System
- Excel-style references: `A1`, `B2`, `AA1` (base-26 column encoding)
- Parsing in `csv_handler.rs::CellRange::parse()` and `cli.rs::parse_column_ref()`
- Used by formula evaluator and Excel operations

### Format Detection
- `DefaultFormatDetector` detects format from file extension
- Located in `src/format_detector.rs`
- Used by `HandlerRegistry` to select handlers

### Error Handling
- `anyhow::Result<T>` for most operations
- Custom `DatacellError` in `src/error.rs` for domain-specific errors
- Error traits in `src/error_traits.rs` for categorization

### Formula Evaluation
- Cell references parsed to (row, col) indices
- Functions implemented in `src/formula/functions.rs`
- Supports: arithmetic, SUM, AVERAGE, MIN, MAX, COUNT, ROUND, ABS, LEN, VLOOKUP, SUMIF, COUNTIF, IF, CONCAT

### Testing Strategy
- Integration tests in `tests/` directory
- Unit tests inline in source files
- Test data in `examples/` directory

## Common Development Patterns

### Adding a New File Format Handler

1. Implement `DataReader` and `DataWriter` traits from `src/traits.rs`
2. Add format detection to `DefaultFormatDetector::detect_format()`
3. Register in `HandlerRegistry::get_reader()` and `get_writer()`
4. Add to `Converter::read_any()` / `write_any()` if special handling needed

### Adding a New CLI Command

1. Add variant to `Commands` enum in `src/cli.rs`
2. Add handler method to `DefaultCommandHandler` in `src/cli.rs`
3. Add match arm in `CommandHandler::handle()` implementation
4. Run `cargo run -- completions <shell>` to update completions

### Adding a New Data Operation

1. Add method to `DataOperations` in `src/operations/` (appropriate submodule)
2. Add types to `src/operations/types.rs` if needed
3. Call from `DefaultCommandHandler` in `src/cli.rs`
4. Add test in `tests/test_operations.rs`

## File Format Support Matrix

| Format | Read | Write | Notes |
|--------|------|-------|-------|
| CSV    | ✅   | ✅    | Full support via `csv` crate |
| XLSX   | ✅   | ✅    | Read: `calamine`, Write: `rust_xlsxwriter` |
| XLS    | ✅   | ❌    | Legacy Excel (read-only) |
| ODS    | ✅   | ❌    | OpenDocument Spreadsheet (read-only) |
| Parquet| ✅   | ✅    | Via `arrow`/`parquet` crates |
| Avro   | ✅   | ✅    | Via `apache-avro` crate |

## Dependencies Notes

- **Edition**: Rust 2024
- **rmcp 0.12**: MCP server implementation
- **calamine**: Excel/ODS reading (does not support writing)
- **rust_xlsxwriter**: Excel writing (XLSX only)
- **arrow/parquet 54**: Columnar format support
- **clap 4.5**: CLI with derive macros
- **tokio**: Async runtime for MCP server

## Configuration

- Config file: `.datacell.toml` (generated via `datacell config-init`)
- Loaded by `Config::load()` in `src/config.rs`
- Supports Excel styling defaults, output options

## MCP Server

- Entry point: `datacell serve` (handled by `mcp::DatacellMcpServer`)
- Uses stdio transport (rmcp)
- Exposes tools: read_file, write_file, convert_file, apply_formula
