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
- `DataOperator` - Combined trait for sort, filter, transform operations

**Handler Registry** ([src/handler_registry.rs](src/handler_registry.rs)):
- `HandlerRegistry` uses `FormatDetector` to select the appropriate handler
- Delegates to `CsvHandler`, `ExcelHandler`, `ParquetHandler`, or `AvroHandler`
- Used internally by `Converter` for format-agnostic operations

**Type-Safe Cell Values** ([src/types.rs](src/types.rs)):
- `CellValue` enum provides strongly-typed cell representation
- Variants: `String`, `Number(f64)`, `Integer(i64)`, `Boolean(bool)`, `DateTime(i64)`, `Empty`
- Improves type safety and performance over string-only representations
- Used throughout the codebase for type-safe data operations

### Modular CLI Architecture

The CLI is organized into specialized command handlers ([src/cli/](src/cli/)):

**Command Structure** ([src/cli/mod.rs](src/cli/mod.rs)):
- `Commands` enum - All CLI commands defined with clap derive macros
- `DefaultCommandHandler` ([src/cli/handler.rs](src/cli/handler.rs)) - Main coordinator that delegates to specialized handlers

**Specialized Handlers** ([src/cli/commands/](src/cli/commands/)):
- `IoCommandHandler` ([io.rs](src/cli/commands/io.rs)) - File I/O operations (read, write, convert, formula, sheets, read-all, write-range, append, serve)
- `TransformCommandHandler` ([transform.rs](src/cli/commands/transform.rs)) - Data transformations (sort, filter, replace, dedupe, transpose, select, rename, drop, fillna, dropna, mutate, astype, normalize, clip, parse-date, regex-filter, regex-replace)
- `PandasCommandHandler` ([pandas.rs](src/cli/commands/pandas.rs)) - Pandas-style operations (head, tail, sample, describe, value-counts, corr, groupby, join, concat, unique, info, dtypes, pivot)
- `AdvancedCommandHandler` ([advanced.rs](src/cli/commands/advanced.rs)) - Advanced features (validate, profile, chart, encrypt, decrypt, batch, plugin, stream)

### Module Organization

```
src/
├── main.rs              # CLI entry point (clap-based)
├── lib.rs               # Library exports (all public APIs)
├── types.rs             # Type-safe cell value representations
│
├── cli/                 # Modular CLI command structure
│   ├── mod.rs           # Commands enum definition
│   ├── handler.rs       # DefaultCommandHandler coordinator
│   ├── format.rs        # Output format types
│   └── commands/        # Specialized command handlers
│       ├── mod.rs       # CommandHandler trait
│       ├── io.rs        # I/O operations
│       ├── transform.rs # Transformations
│       ├── pandas.rs    # Pandas-style ops
│       └── advanced.rs  # Advanced features
│
├── converter.rs         # Format conversion (uses HandlerRegistry)
├── handler_registry.rs  # Format-based handler selection
├── format_detector.rs   # DefaultFormatDetector implementation
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
   cli/mod.rs (Commands enum)
       │
       ▼
   cli/handler.rs (DefaultCommandHandler)
       │
       ├─▶ IoCommandHandler (read, write, convert, formula, serve)
       ├─▶ TransformCommandHandler (sort, filter, replace, etc.)
       ├─▶ PandasCommandHandler (head, tail, join, groupby, etc.)
       └─▶ AdvancedCommandHandler (validate, profile, chart, etc.)
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
- Parsing in `csv_handler.rs::CellRange::parse()` and helpers in `src/traits.rs` via `CellRangeProvider` trait
- Used by formula evaluator and Excel operations
- Supported ranges: single cells (`A1`), ranges (`A1:C10`), entire columns (`A:A`)

### Format Detection
- `DefaultFormatDetector` in `src/format_detector.rs` detects format from file extension
- Implements `FormatDetector` trait from `src/traits.rs`
- Supported formats: csv, xlsx, xls, ods, parquet, avro
- Used by `HandlerRegistry` to select appropriate handlers dynamically

### Type System
- **CellValue enum** (`src/types.rs`) - Strongly-typed cell representation
  - `String(String)` - Text data
  - `Number(f64)` - Floating point numbers
  - `Integer(i64)` - Integer values for exact precision
  - `Boolean(bool)` - True/false values
  - `DateTime(i64)` - Timestamps
  - `Empty` - Null/missing values
- Helper methods: `is_numeric()`, `as_str()`, `as_number()`, `as_bool()`, `is_empty()`
- Improves type safety and eliminates repeated string parsing

### Command Handler Pattern
- Each handler group implements `CommandHandler` trait from `src/cli/commands/mod.rs`
- `DefaultCommandHandler` uses composition pattern to delegate to specialized handlers
- Handlers are stateless and thread-safe (Send + Sync)
- Easy to add new commands: add to `Commands` enum, implement in appropriate handler

### Error Handling
- `anyhow::Result<T>` for most operations
- Custom `DatacellError` in `src/error.rs` for domain-specific errors
- Error traits in `src/error_traits.rs` for categorization
- Context-rich errors using `.context()` and `.with_context()`

### Formula Evaluation
- Cell references parsed to (row, col) indices
- Functions implemented in `src/formula/functions.rs`
- Supports: arithmetic, SUM, AVERAGE, MIN, MAX, COUNT, ROUND, ABS, LEN, VLOOKUP, SUMIF, COUNTIF, IF, CONCAT
- Interpreter pattern for formula evaluation

### Testing Strategy
- Integration tests in `tests/` directory
- Unit tests inline in source files
- Test data in `examples/` directory
- Recent test additions: `test_encryption.rs`, `test_handler_registry.rs`, `test_plugins.rs`, `test_streaming.rs`

## Common Development Patterns

### Adding a New File Format Handler

1. Implement `DataReader` and `DataWriter` traits from `src/traits.rs`
2. Add format detection to `DefaultFormatDetector::is_supported()` and `supported_formats()` in `src/format_detector.rs`
3. Register in `HandlerRegistry::get_reader()` and `get_writer()` in `src/handler_registry.rs`
4. Add to `Converter::read_any()` / `write_any()` if special handling needed

### Adding a New CLI Command

1. Add variant to `Commands` enum in `src/cli/mod.rs` with clap attributes
2. Add handler method to the appropriate specialized handler in `src/cli/commands/`:
   - `IoCommandHandler` for I/O operations
   - `TransformCommandHandler` for transformations
   - `PandasCommandHandler` for pandas-style operations
   - `AdvancedCommandHandler` for advanced features
3. Add match arm in `DefaultCommandHandler::handle()` implementation in `src/cli/handler.rs`
4. Run `cargo run -- completions <shell>` to update shell completions

### Adding a New Data Operation

1. Add method to `DataOperations` in the appropriate `src/operations/` submodule:
   - `core.rs` for basic operations (sort, filter, replace, transpose)
   - `pandas.rs` for pandas-style operations
   - `stats.rs` for statistical operations
   - `transform.rs` for query/mutate/astype operations
2. Add types to `src/operations/types.rs` if needed (AggFunc, JoinType, SortOrder)
3. Call from the appropriate command handler in `src/cli/commands/`
4. Add test in `tests/test_operations.rs` or create a new test file

### Working with Type-Safe Cell Values

When implementing new operations, prefer using `CellValue` from `src/types.rs`:

```rust
use crate::types::CellValue;

// Parse a string value into CellValue
let cell = CellValue::string("42");
let number = CellValue::number(42.0);

// Check types and extract values
if cell.is_numeric() {
    if let Some(n) = cell.as_number() {
        // Use the numeric value
    }
}
```

This provides better type safety and performance compared to string-only operations.

## File Format Support Matrix

| Format | Read | Write | Notes |
|--------|------|-------|-------|
| CSV    | ✅   | ✅    | Full support via `csv` crate |
| XLSX   | ✅   | ✅    | Read: `calamine`, Write: `rust_xlsxwriter` |
| XLS    | ✅   | ❌    | Legacy Excel (read-only) |
| ODS    | ✅   | ❌    | OpenDocument Spreadsheet (read-only) |
| Parquet| ✅   | ✅    | Via `arrow`/`parquet` crates |
| Avro   | ✅   | ✅    | Via `apache-avro` crate |
| Google Sheets | ✅ | ✅ | Placeholder implementation (requires API setup) |

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
