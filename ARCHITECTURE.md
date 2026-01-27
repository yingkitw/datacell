# datacell Architecture

A Rust CLI tool and MCP server for reading, writing, converting spreadsheet files (CSV, XLS, XLSX, ODS, Parquet, Avro) with formula support and pandas-inspired operations.

## Overview

**Status**: Production-ready with 189 unit tests and 60+ integration tests passing

```
datacell/
├── src/
│   ├── main.rs              # CLI entry point with clap
│   ├── lib.rs               # Public library exports
│   ├── cli/                 # CLI module (refactored for better organization)
│   │   ├── mod.rs           # CLI exports
│   │   ├── handler.rs       # Command routing and execution
│   │   ├── format.rs        # Output formatting (JSON, Markdown, CSV)
│   │   └── commands/        # Command handlers by category
│   │       ├── io.rs        # read, write, convert, sheets
│   │       ├── transform.rs # sort, filter, replace, dedupe, transpose
│   │       ├── pandas.rs    # head, tail, select, groupby, join, etc.
│   │       └── advanced.rs  # validate, profile, encrypt, chart
│   ├── traits.rs            # Core trait definitions (DataReader, DataWriter, etc.)
│   ├── common.rs            # Shared utilities (format, validation, transform, error)
│   ├── converter.rs         # Format conversion (uses HandlerRegistry)
│   ├── handler_registry.rs  # Format-based handler selection
│   ├── format_detector.rs   # File format detection from extension
│   │
│   ├── csv_handler.rs       # CSV file operations
│   ├── excel/               # Excel/ODS operations (calamine + rust_xlsxwriter)
│   │   ├── mod.rs           # Module exports + ExcelHandler
│   │   ├── reader.rs        # Read Excel/ODS files
│   │   ├── writer.rs        # Write Excel files (XLSX only)
│   │   ├── chart.rs         # Chart generation
│   │   └── types.rs         # Excel-specific types (CellStyle, WriteOptions)
│   ├── columnar.rs          # Parquet and Avro support
│   │
│   ├── formula/             # Formula evaluation module
│   │   ├── mod.rs           # Module exports
│   │   ├── types.rs         # FormulaResult, CellRange
│   │   ├── evaluator.rs     # Main evaluator logic
│   │   ├── functions.rs     # SUM, AVERAGE, VLOOKUP, IF, etc.
│   │   └── parser.rs        # Cell reference parsing (A1 → row,col)
│   │
│   ├── operations/          # Data operations module (pandas-inspired)
│   │   ├── mod.rs           # Module exports
│   │   ├── types.rs         # SortOrder, JoinType, AggFunc
│   │   ├── core.rs          # Basic ops: sort, filter, replace, transpose
│   │   ├── pandas.rs        # head, tail, sample, join, groupby, concat
│   │   ├── stats.rs         # describe, value_counts, correlation
│   │   └── transform.rs     # query, mutate, astype, normalize, clip, parse-date
│   │
│   ├── config.rs            # Configuration file support (.datacell.toml)
│   ├── error.rs             # Enhanced error types with context
│   ├── error_traits.rs      # Trait-based error handling
│   ├── mcp.rs               # MCP server implementation (rmcp 0.12)
│   │
│   ├── validation.rs        # Data validation framework
│   ├── profiling.rs         # Data profiling and quality reports
│   ├── quality.rs           # Data quality checks
│   ├── text_analysis.rs     # Text stats, sentiment, keywords, language
│   ├── timeseries.rs        # Time series resampling, rolling windows
│   ├── geospatial.rs        # Distance calculations
│   ├── anomaly.rs           # Anomaly detection (zscore, IQR, percentile)
│   ├── encryption.rs        # File encryption (XOR, AES256)
│   ├── workflow.rs          # Pipeline/workflow execution
│   ├── api.rs               # REST API server (placeholder)
│   ├── plugins.rs           # Plugin function registry
│   └── streaming.rs         # Streaming data processing
│
├── tests/
│   ├── test_basic.rs        # Basic integration tests
│   ├── test_excel.rs        # Excel operations tests
│   ├── test_csv_handler.rs  # CSV handler tests
│   ├── test_converter.rs    # Format conversion tests
│   ├── test_formula.rs      # Formula evaluation tests
│   ├── test_operations.rs   # Data operations tests
│   ├── test_traits.rs       # Trait implementation tests
│   ├── test_columnar.rs     # Parquet/Avro tests
│   ├── test_config.rs       # Configuration tests
│   └── test_error.rs        # Error handling tests
│
├── examples/                # Example data files
├── CLAUDE.md                # AI assistant development guide
├── Cargo.toml
└── README.md
```

## Trait-Based Architecture

The codebase uses a trait-based design for extensibility and testability:

### Core Traits ([src/traits.rs](src/traits.rs))

| Trait | Purpose |
|-------|---------|
| `DataReader` | Read data from files |
| `DataWriter` | Write data to files |
| `FileHandler` | Combined read/write (auto-implemented) |
| `FormatDetector` | Detect file format from extension |
| `SchemaProvider` | Get metadata (columns, row counts) |
| `StreamingReader` | Read large files incrementally |
| `StreamingWriter` | Write large files incrementally |
| `CellRangeProvider` | Parse cell references (A1:C10) |
| `SortOperator` | Sort operations |
| `FilterOperator` | Filter operations |
| `TransformOperator` | Transform operations |
| `DataOperator` | Combined operations trait |

### Handler Registry

[HandlerRegistry](src/handler_registry.rs) manages file handlers by format:

```rust
let registry = HandlerRegistry::new();

// Automatically selects appropriate handler
let data = registry.read("data.csv")?;
let data = registry.read("data.xlsx")?;
let data = registry.read("data.parquet")?;
```

## Core Components

### Data Handlers

| Handler | Format | Read | Write | Traits |
|---------|--------|------|-------|--------|
| [CsvHandler](src/csv_handler.rs) | CSV | ✅ | ✅ | DataReader, DataWriter, FileHandler |
| [ExcelHandler](src/excel/mod.rs) | XLSX, XLS, ODS | ✅ | ✅ (XLSX only) | DataReader, DataWriter |
| [ParquetHandler](src/columnar.rs) | Parquet | ✅ | ✅ | DataReader, DataWriter, FileHandler |
| [AvroHandler](src/columnar.rs) | Avro | ✅ | ✅ | DataReader, DataWriter, FileHandler |

### Format Conversion

[Converter](src/converter.rs) provides format-agnostic conversion:

```rust
let converter = Converter::new();
converter.convert("data.xlsx", "data.parquet", None)?;
```

### Formula Evaluation

[FormulaEvaluator](src/formula/evaluator.rs) supports Excel-like formulas:

**Functions**: SUM, AVERAGE, MIN, MAX, COUNT, ROUND, ABS, LEN, VLOOKUP, SUMIF, COUNTIF, IF, CONCAT
**Operations**: Arithmetic (+, -, *, /), cell references (A1, B2), ranges (A1:C10)

### Data Operations

[DataOperations](src/operations/core.rs) provides pandas-inspired operations:

- **Core**: sort, filter, replace, transpose, dedupe
- **Pandas**: head, tail, sample, join, groupby, concat, select, drop
- **Stats**: describe, value_counts, correlation
- **Transform**: query, mutate, astype, normalize, clip, parse-date, regex-filter, regex-replace

## Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                     Entry Points                            │
├──────────────────┬──────────────────┬──────────────────────┤
│   CLI (main.rs)  │  MCP (mcp.rs)    │  Library (lib.rs)    │
│   clap commands  │  rmcp server     │  Public APIs         │
└──────────────────┴──────────────────┴──────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│              Command Handler Layer (cli.rs)                  │
│         DefaultCommandHandler implements all commands        │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│           Format-Agnostic Layer (traits.rs)                 │
│    DataReader │ DataWriter │ FileHandler │ FormatDetector  │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│         Handler Registry (handler_registry.rs)              │
│         Selects appropriate handler based on format         │
└─────────────────────────────────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  CsvHandler  │  │ ExcelHandler │  │ Parquet/Avro │
└──────────────┘  └──────────────┘  └──────────────┘
        │                  │                  │
        └──────────────────┼──────────────────┘
                           ▼
                    ┌──────────────┐
                    │   File I/O   │
                    └──────────────┘
```

## Design Principles

### DRY (Don't Repeat Yourself)
- Handlers are reused by CLI, MCP, and Converter
- Common utilities in [common.rs](src/common.rs)
- Shared traits reduce code duplication

### KISS (Keep It Simple, Stupid)
- Simple struct-based handlers
- No complex abstractions beyond traits
- Direct file I/O without unnecessary layers

### Testability
- Each handler implements testable traits
- Mock implementations in [mocks.rs](src/mocks.rs)
- Integration tests in [tests/](tests/)
- 29+ passing tests

### Extensibility
- Trait-based design allows new handlers
- Plugin system for custom functions
- Workflow orchestration for pipelines

## Advanced Features

### Data Validation
[validation.rs](src/validation.rs): Rule-based validation with JSON config

### Data Profiling
[profiling.rs](src/profiling.rs): Column profiling, data quality scores

### Quality Reports
[quality.rs](src/quality.rs): Comprehensive quality reports with recommendations

### Text Analysis
[text_analysis.rs](src/text_analysis.rs): Statistics, sentiment, keywords, language detection

### Time Series
[timeseries.rs](src/timeseries.rs): Resampling, rolling windows, date parsing

### Geospatial
[geospatial.rs](src/geospatial.rs): Distance calculations (Haversine formula)

### Anomaly Detection
[anomaly.rs](src/anomaly.rs): Z-score, IQR, percentile methods

### Encryption
[encryption.rs](src/encryption.rs): XOR, AES256 encryption for data files

### Streaming
[streaming.rs](src/streaming.rs): Process large files incrementally

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| clap | 4.5 | CLI argument parsing with derive macros |
| calamine | 0.26 | Read Excel files (.xls, .xlsx, .ods) |
| rust_xlsxwriter | 0.80 | Write Excel files (.xlsx only) |
| csv | 1.3 | CSV file handling |
| parquet | 54 | Parquet format support |
| arrow | 54 | Arrow memory format |
| apache-avro | 0.17 | Avro format support |
| rmcp | 0.12 | MCP server implementation |
| tokio | 1 | Async runtime |
| serde_json | 1.0 | JSON serialization |
| regex | 1.10 | Pattern matching |
| chrono | 0.4 | Date/time handling |
| anyhow | 1.0 | Error handling |
| thiserror | 1.0 | Error types |
| schemars | 1.0 | JSON schema generation |

## Configuration

Configuration file support via [config.rs](src/config.rs):

```toml
# .datacell.toml
[excel]
header_bold = true
header_bg_color = "4472C4"
header_font_color = "FFFFFF"
auto_filter = true
freeze_header = true
auto_fit = true

[output]
default_format = "csv"
include_headers = true
```

## Error Handling

Two-tier error handling:
1. **Enhanced errors** ([error.rs](src/error.rs)): `DatacellError` with context
2. **Trait-based errors** ([error_traits.rs](src/error_traits.rs)): Categorization and recovery

## Testing Architecture

### Unit Tests (189 tests)

Located in `tests/` directory with comprehensive coverage:

- **test_formula.rs**: 21 tests for formula evaluation (SUM, AVERAGE, IF, VLOOKUP, etc.)
- **test_csv.rs**: 21 tests for CSV operations (read, write, range, streaming)
- **test_excel.rs**: 23 tests for Excel operations (read, write, charts, styling)
- **test_converter.rs**: 13 tests for format conversions (CSV↔Excel↔Parquet↔Avro)
- **test_operations.rs**: 29 tests for data operations (sort, filter, groupby, join)
- **test_encryption.rs**: 9 tests for XOR/AES256 encryption
- **test_plugins.rs**: 15 tests for plugin system
- **test_streaming.rs**: 8 tests for streaming operations
- **test_handler_registry.rs**: 13 tests for format detection and handler selection
- **test_error.rs**: 17 tests for error handling and context
- **test_traits.rs**: 6 tests for trait implementations

### Integration Tests (60+ tests)

Located in `examples/test_all_capabilities.sh`:

**Test Categories:**
1. **File Format I/O** (8 tests): CSV, XLSX, Parquet, Avro reading with JSON/Markdown output
2. **Format Conversions** (12 tests): All format combinations (CSV↔Excel↔Parquet↔Avro)
3. **Formula Evaluation** (15 tests): Arithmetic, aggregates, conditionals, lookup, text, math
4. **Data Operations** (10 tests): Sort, filter, replace, dedupe, transpose
5. **Pandas-Style Operations** (25 tests): head/tail, select/drop, groupby, join, fillna, mutate, etc.
6. **Transform Operations** (8 tests): Clip, normalize, date parsing, regex
7. **Advanced Features** (7 tests): Validation, profiling, encryption/decryption
8. **Styling & Visualization** (6 tests): Styled Excel export, charts (column, bar, line, pie)
9. **Configuration** (3 tests): Config initialization, shell completions
10. **Batch Processing** (1 test): Multi-file operations

### Test Infrastructure

**Test Runners:**
- `test_all_capabilities.sh`: Bash script for comprehensive CLI testing (80+ operations)
- `run_tests.py`: Python test runner with JSON reporting and detailed analytics
- `test_data_generator.sh`: Generates 14 additional test data files

**Test Data:**
- `employees.csv/xlsx/parquet/avro`: Employee records
- `sales.csv/xlsx/parquet/avro`: Sales transactions
- `numbers.csv/parquet`: Numeric data for formulas
- `lookup.csv/avro`: Lookup tables
- `duplicates.csv`: Duplicate row testing
- `financial_data.csv`: Time series data
- `validation_rules.json`: Validation configuration

**Documentation:**
- `TESTING_GUIDE.md`: Comprehensive testing guide with all test cases
- `QUICK_REFERENCE.md`: Command reference card
- `EXAMPLES_SUMMARY.md`: Test suite overview

### Test Execution

```bash
# Run all unit tests
cargo test

# Run integration tests
cd examples
./test_all_capabilities.sh

# Run with detailed reporting
python3 run_tests.py

# Generate test data
./test_data_generator.sh
```

### Test Results

- ✅ **189 unit tests passing** (0 failures)
- ✅ **60+ integration tests passing** (exit code 0)
- ✅ **Zero compilation errors**
- ✅ **100% of major features tested**
- ✅ **All test categories validated**

## Performance Considerations

- **Zero-copy parsing** where possible (Arrow for Parquet)
- **Streaming support** for large files
- **Efficient CSV parsing** with the `csv` crate
- **Lazy evaluation** in operations pipeline

## Future Improvements

- Full REST API server implementation (requires HTTP framework choice)
- More Excel formula functions
- Conditional formatting in Excel output
- Query optimizer for complex operations
- Parallel processing for large datasets
- Caching layer for repeated operations
