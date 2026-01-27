# xls-rs

A fast, unified CLI tool for spreadsheet and columnar data manipulation.

## The Problem

Working with tabular data often requires juggling multiple tools:

- **Excel/LibreOffice** - GUI-only, slow for batch processing, no scripting
- **pandas/Python** - Requires Python environment, slow startup, memory-heavy
- **csvkit** - CSV-only, no Excel/Parquet/Avro support
- **xsv** - Fast but CSV-only, no formulas
- **Apache Spark** - Overkill for simple transformations, complex setup

Common pain points:
- Converting between formats requires different tools
- Applying Excel-like formulas to CSV files is awkward
- Batch processing spreadsheets in CI/CD pipelines is difficult
- No single tool handles CSV, Excel, Parquet, and Avro uniformly

## The Solution

**xls-rs** is a single, fast CLI tool that:

- Reads/writes **all major formats**: CSV, XLSX, XLS, ODS, Parquet, Avro
- Applies **Excel-like formulas** to any format (SUM, VLOOKUP, IF, etc.)
- Performs **data operations** without code (sort, filter, dedupe, transpose)
- Converts **between any formats** with one command
- Outputs to **JSON/Markdown** for easy integration
- Runs as an **MCP server** for AI assistant integration
- Provides **pandas-style operations** for data manipulation

## Why xls-rs?

| Feature | xls-rs | pandas | csvkit | xsv | Excel |
|---------|----------|--------|--------|-----|-------|
| Single binary | ✅ | ❌ | ❌ | ✅ | ❌ |
| CSV support | ✅ | ✅ | ✅ | ✅ | ✅ |
| Excel support | ✅ | ✅ | ❌ | ❌ | ✅ |
| Parquet/Avro | ✅ | ✅ | ❌ | ❌ | ❌ |
| Formulas | ✅ | ❌ | ❌ | ❌ | ✅ |
| CLI-native | ✅ | ❌ | ✅ | ✅ | ❌ |
| Fast startup | ✅ | ❌ | ❌ | ✅ | ❌ |
| Scriptable | ✅ | ✅ | ✅ | ✅ | ❌ |
| No runtime dependencies | ✅ | ❌ | ✅ | ✅ | ❌ |

## Quick Start

```bash
# Build
cargo build --release

# Convert CSV to Parquet
xls-rs convert --input data.csv --output data.parquet

# Apply formula
xls-rs formula --input sales.csv --output result.csv --formula "SUM(C2:C100)" --cell "D1"

# Filter and sort (SQL-like syntax)
xls-rs filter --input data.csv --output filtered.csv --where "status = 'active'"
xls-rs sort --input filtered.csv --output sorted.csv --column date --descending

# Output as JSON for API consumption
xls-rs read --input report.xlsx --format json > report.json
```

## Features

- **Read** XLSX, ODS, CSV, Parquet, and Avro files
- **Write** data to XLSX, CSV, Parquet, and Avro files
- **Convert** between any formats (CSV, XLSX, ODS, Parquet, Avro)
- **Apply formulas** to cells in both CSV and Excel files
  - Supports basic arithmetic operations (+, -, *, /)
  - Supports SUM(), AVERAGE(), MIN(), MAX(), COUNT() functions
  - Supports ROUND(), ABS(), LEN() functions
  - Supports VLOOKUP(), SUMIF(), COUNTIF() functions
  - Supports IF() for conditional logic
  - Supports CONCAT() for string concatenation
  - Supports cell references (e.g., A1, B2)
- **Data operations**
  - Sort rows by column (ascending/descending)
  - Filter rows by condition
  - Find and replace values
  - Remove duplicate rows
  - Transpose data (rows to columns)
  - Merge cells (Excel output)
- **Pandas-style operations**
  - Head/tail (first/last n rows)
  - Sample random rows
  - Select/drop columns
  - Describe (summary statistics)
  - Value counts
  - Group by with aggregations (sum, count, mean, min, max)
  - Pivot tables
  - Join/merge files (inner, left, right, outer)
  - Concatenate files
  - Fill/drop missing values
  - Rename columns
  - Correlation matrix
  - Column type inference (dtypes)
  - Type casting (astype)
  - Unique values
  - Dataset info
  - SQL-like query with WHERE clause
  - Computed columns (mutate)
  - Value clipping
  - Column normalization (0-1)
  - Date parsing and formatting
  - Regex filter and replace
- **Batch processing** - process multiple files with glob patterns
- **Shell completions** - bash, zsh, fish, powershell
- **Config file** - `.datacell.toml` for default options
- **Styled Excel export** - headers, colors, borders, freeze panes, auto-filter
- **Chart visualization** - bar, column, line, area, pie, scatter, doughnut charts
- **Cell range operations** - read/write specific ranges like A1:C10
- **Multiple output formats** - CSV, JSON, Markdown
- **Multi-sheet support** - list sheets, read all sheets at once
- **Streaming API** - process large files efficiently
- **Progress callbacks** - track long-running operations
- **MCP server** for integration with AI assistants

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/xls-rs`.

## Usage

### Read a file

```bash
# Read CSV
xls-rs read --input data.csv

# Read Excel (first sheet)
xls-rs read --input data.xlsx

# Read specific sheet
xls-rs read --input data.xlsx --sheet "Sheet2"

# Read specific cell range
xls-rs read --input data.csv --range "A1:C10"

# Read as JSON
xls-rs read --input data.csv --format json

# Read range as JSON
xls-rs read --input data.xlsx --range "B2:D5" --format json

# Read as Markdown table
xls-rs read --input data.csv --format markdown

# Read Parquet file
xls-rs read --input data.parquet

# Read Avro file
xls-rs read --input data.avro
```

### Write a file

```bash
# Write CSV from CSV
xls-rs write --output output.csv --csv input.csv

# Write Excel from CSV
xls-rs write --output output.xlsx --csv input.csv

# Write Parquet from CSV
xls-rs write --output output.parquet --csv input.csv

# Write Avro from CSV
xls-rs write --output output.avro --csv input.csv

# Write Excel with specific sheet name
xls-rs write --output output.xlsx --csv input.csv --sheet "Data"
```

### Convert between formats

Supports conversion between: CSV, XLSX, XLS, ODS, Parquet, Avro

```bash
# CSV to Excel
xls-rs convert --input data.csv --output data.xlsx

# Excel to CSV
xls-rs convert --input data.xlsx --output data.csv

# Excel to CSV (specific sheet)
xls-rs convert --input data.xlsx --output data.csv --sheet "Sheet2"

# CSV to Parquet
xls-rs convert --input data.csv --output data.parquet

# Parquet to CSV
xls-rs convert --input data.parquet --output data.csv

# Excel to Avro
xls-rs convert --input data.xlsx --output data.avro

# Avro to Parquet
xls-rs convert --input data.avro --output data.parquet

# ODS to CSV
xls-rs convert --input data.ods --output data.csv
```

### Apply formulas

```bash
# Apply SUM formula to CSV
xls-rs formula --input data.csv --output result.csv --formula "SUM(A1:A10)" --cell "C1"

# Apply arithmetic formula
xls-rs formula --input data.csv --output result.csv --formula "A1+B1" --cell "C1"

# Apply AVERAGE formula to Excel
xls-rs formula --input data.xlsx --output result.xlsx --formula "AVERAGE(A1:A10)" --cell "B1" --sheet "Sheet1"
```

### Data operations

```bash
# Sort by column A (ascending)
xls-rs sort --input data.csv --output sorted.csv --column A

# Sort by column B (descending)
xls-rs sort --input data.csv --output sorted.csv --column B --descending

# Filter rows using SQL-like WHERE clause
xls-rs filter --input data.csv --output filtered.csv --where "A > 10"
xls-rs filter --input data.csv --output filtered.csv --where "status = 'active'"

# Find and replace
xls-rs replace --input data.csv --output replaced.csv --find "old" --replace "new"

# Remove duplicate rows
xls-rs dedupe --input data.csv --output unique.csv

# Transpose (rows to columns)
xls-rs transpose --input data.csv --output transposed.csv

# Append data from one file to another
xls-rs append --source new_data.csv --target existing.csv

# List sheets in Excel/ODS file
xls-rs sheets --input workbook.xlsx

# Read all sheets at once (as JSON)
xls-rs read-all --input workbook.xlsx --format json

# Write data to specific cell range
xls-rs write-range --input data.csv --output result.xlsx --start B2
```

### Pandas-style operations

```bash
# First/last n rows
xls-rs head --input data.csv -n 5
xls-rs tail --input data.csv -n 5

# Sample random rows
xls-rs sample --input data.csv -n 10 --seed 42

# Select specific columns
xls-rs select --input data.csv --output subset.csv --columns "name,age,salary"

# Describe statistics
xls-rs describe --input data.csv --format markdown

# Value counts
xls-rs value-counts --input data.csv --column category

# Group by and aggregate
xls-rs groupby --input sales.csv --output summary.csv --by category --agg "sum:amount,count:id,mean:price"

# Join two files
xls-rs join --left orders.csv --right customers.csv --output merged.csv --on customer_id --how left

# Concatenate files
xls-rs concat --inputs "jan.csv,feb.csv,mar.csv" --output q1.csv

# Fill empty values
xls-rs fillna --input data.csv --output filled.csv --value "N/A"

# Drop rows with empty values
xls-rs dropna --input data.csv --output clean.csv

# Drop columns
xls-rs drop --input data.csv --output slim.csv --columns "temp,debug"

# Rename columns
xls-rs rename --input data.csv --output renamed.csv --from "old_name" --to "new_name"

# Pivot table
xls-rs pivot --input sales.csv --output pivot.csv --index Category --columns Product --values Price --agg sum

# Correlation matrix
xls-rs corr --input data.csv --columns "Price,Quantity" --format markdown

# Show column types
xls-rs dtypes --input data.csv --format markdown

# SQL-like query
xls-rs query --input data.csv --output filtered.csv -w "Price > 100"

# Add computed column
xls-rs mutate --input data.csv --output result.csv --column Total --formula "Price * Quantity"

# Cast column type
xls-rs astype --input data.csv --output result.csv --column Price -t int

# Get unique values
xls-rs unique --input data.csv --column Category

# Dataset info
xls-rs info --input data.csv --format markdown

# Clip values to range
xls-rs clip --input data.csv --output clipped.csv --column Price --min 0 --max 1000

# Normalize column (0-1)
xls-rs normalize --input data.csv --output normalized.csv --column Price

# Parse and reformat dates
xls-rs parse-date --input data.csv --output result.csv --column Date --from-format "%Y-%m-%d" --to-format "%d/%m/%Y"

# Filter with regex
xls-rs regex-filter --input data.csv --output filtered.csv --column Name --pattern "^[A-M]"

# Replace with regex
xls-rs regex-replace --input data.csv --output result.csv --column Category --pattern "Electronics" --replacement "Tech"

# Batch process multiple files
xls-rs batch --inputs "data/*.csv" --output-dir processed/ --operation sort --args '{"column":"Price","desc":true}'

# Generate shell completions
xls-rs completions zsh >> ~/.zshrc
xls-rs completions bash >> ~/.bashrc
xls-rs completions fish > ~/.config/fish/completions/xls-rs.fish

# Initialize config file
xls-rs config-init --output .datacell.toml

# Export to Excel with styling
xls-rs export-styled --input data.csv --output styled.xlsx --header-bg 4472C4 --header-font FFFFFF

# Create charts
xls-rs chart --input sales.csv --output chart.xlsx -t column --title "Sales by Product"
xls-rs chart --input data.csv --output multi.xlsx -t bar --value-cols "1,2,3" --title "Comparison"
xls-rs chart --input data.csv --output pie.xlsx -t pie --title "Distribution"
```

## Formula Examples

- `SUM(A1:A10)` - Sum of cells A1 through A10
- `AVERAGE(A1:A10)` - Average of cells A1 through A10
- `MIN(A1:A10)` - Minimum value in range
- `MAX(A1:A10)` - Maximum value in range
- `COUNT(A1:A10)` - Count of numeric cells in range
- `ROUND(A1, 2)` - Round to 2 decimal places
- `ABS(A1)` - Absolute value
- `LEN(A1)` - Length of text in cell
- `VLOOKUP(2, A1:C10, 3)` - Lookup value in table
- `SUMIF(A1:A10, ">5", B1:B10)` - Sum cells matching criteria
- `COUNTIF(A1:A10, ">5")` - Count cells matching criteria
- `IF(A1>10, "High", "Low")` - Conditional logic
- `CONCAT(A1, " ", B1)` - String concatenation
- `A1+B1` - Add values in A1 and B1
- `A1-B1` - Subtract B1 from A1
- `A1*B1` - Multiply A1 by B1
- `A1/B1` - Divide A1 by B1
- `A1` - Reference a single cell

## Use Cases

### Data Pipeline Automation
```bash
# Daily ETL: Excel → Parquet for analytics
xls-rs convert --input daily_report.xlsx --output data/daily_$(date +%Y%m%d).parquet
```

### Report Generation
```bash
# Calculate totals and output as Markdown for documentation
xls-rs formula --input sales.csv --output report.csv --formula "SUM(D2:D100)" --cell "E1"
xls-rs read --input report.csv --format markdown > REPORT.md
```

### Data Cleaning
```bash
# Remove duplicates, filter invalid rows, sort
xls-rs dedupe --input raw.csv --output clean.csv
xls-rs filter --input clean.csv --output valid.csv --column status --op "!=" --value "invalid"
xls-rs sort --input valid.csv --output final.csv --column date
```

### Format Migration
```bash
# Migrate legacy Excel files to modern Parquet
for f in *.xlsx; do
  xls-rs convert --input "$f" --output "${f%.xlsx}.parquet"
done
```

### AI/LLM Integration
```bash
# Start MCP server for AI assistant integration
xls-rs serve
```

## Example Data

See the `examples/` folder for sample data files and usage examples.

## Architecture

```
xls-rs/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports
│   ├── cli.rs           # CLI commands + handlers
│   ├── traits.rs        # Core trait definitions
│   ├── excel/           # Excel/ODS file handling
│   ├── csv_handler.rs   # CSV file handling
│   ├── columnar.rs      # Parquet/Avro handling
│   ├── converter.rs     # Format conversion
│   ├── formula/         # Formula evaluation
│   ├── operations/      # Data operations (sort, filter, etc.)
│   ├── common.rs        # Shared utilities
│   ├── config.rs        # Configuration file support
│   ├── validation.rs    # Data validation
│   ├── profiling.rs     # Data profiling
│   ├── quality.rs       # Data quality reports
│   ├── text_analysis.rs # Text analysis
│   ├── timeseries.rs    # Time series operations
│   ├── geospatial.rs    # Geospatial calculations
│   ├── anomaly.rs       # Anomaly detection
│   ├── encryption.rs    # File encryption
│   ├── workflow.rs      # Workflow orchestration
│   ├── api.rs           # REST API server
│   ├── plugins.rs       # Plugin system
│   ├── streaming.rs     # Streaming data processing
│   └── mcp.rs           # MCP server for AI integration
├── tests/               # Integration tests
├── examples/            # Sample data files
├── CLAUDE.md            # AI assistant guide
└── Cargo.toml
```

## Advanced Features

### Data Validation
```bash
# Validate data against rules
xls-rs validate --input data.csv --rules rules.json --output validated.csv --report report.json
```

### Data Profiling
```bash
# Generate data profile and quality report
xls-rs profile --input data.csv --output profile.json --report quality_report.md
```

### Text Analysis
```bash
# Analyze text content
xls-rs text-analysis --input data.csv --column content --operation stats
xls-rs text-analysis --input data.csv --column content --operation sentiment
xls-rs text-analysis --input data.csv --column content --operation keywords
```

### Time Series Operations
```bash
# Resample time series data
xls-rs resample --input data.csv --output resampled.csv --date-column date --value-column value --interval daily --aggregation sum
```

### Geospatial Operations
```bash
# Calculate distance between coordinates
xls-rs geo-distance --from "40.7128,-74.0060" --to "34.0522,-118.2437" --unit km
```

### Anomaly Detection
```bash
# Detect anomalies using statistical methods
xls-rs detect-anomalies --input data.csv --column value --method zscore --threshold 3.0 --output anomalies.json
```

### Data Encryption
```bash
# Encrypt/decrypt data files
xls-rs encrypt --input data.csv --output encrypted.csv --key secretkey --algorithm aes256
xls-rs decrypt --input encrypted.csv --output decrypted.csv --key secretkey --algorithm aes256
```

### Workflow Orchestration
```bash
# Execute multi-step workflow pipeline
xls-rs pipeline --config pipeline.toml
```

### REST API Server
```bash
# Start REST API server (requires HTTP framework implementation)
xls-rs api-server --host 127.0.0.1 --port 8080 --cors
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing with derive macros |
| `calamine` | Excel/ODS reading |
| `rust_xlsxwriter` | Excel writing (XLSX only) |
| `csv` | CSV handling |
| `parquet` + `arrow` | Parquet support (v54) |
| `apache-avro` | Avro support |
| `rmcp` | MCP server implementation (v0.12) |
| `tokio` | Async runtime |
| `serde_json` | JSON output |
| `regex` | Pattern matching |
| `chrono` | Date/time handling |
| `anyhow` | Error handling |
| `thiserror` | Error types |

## Testing

### Unit Tests (189 tests)

```bash
# Run all unit tests
cargo test

# Run specific test
cargo test test_read_xlsx

# Run with output
cargo test -- --nocapture

# Generate test coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

**Test Coverage:**
- Formula evaluation (21 tests)
- CSV operations (21 tests)
- Excel operations (23 tests)
- Format conversions (13 tests)
- Data operations (29 tests)
- Encryption (9 tests)
- Plugins (15 tests)
- Streaming (8 tests)
- Handler registry (13 tests)
- Error handling (17 tests)
- Trait implementations (6 tests)

### Integration Tests (60+ tests)

Comprehensive CLI testing suite in `examples/`:

```bash
cd examples

# Run all capability tests
./test_all_capabilities.sh

# Run with detailed reporting
python3 run_tests.py

# Generate additional test data
./test_data_generator.sh
```

**Test Categories:**
1. File Format I/O (8 tests)
2. Format Conversions (12 tests)
3. Formula Evaluation (15 tests)
4. Data Operations (10 tests)
5. Pandas-Style Operations (25 tests)
6. Transform Operations (8 tests)
7. Advanced Features (7 tests)
8. Styling & Visualization (6 tests)
9. Configuration (3 tests)
10. Batch Processing (1 test)

**Test Documentation:**
- `examples/TESTING_GUIDE.md` - Comprehensive testing guide
- `examples/QUICK_REFERENCE.md` - Command reference card
- `examples/EXAMPLES_SUMMARY.md` - Test suite overview

**Test Results:**
- ✅ 189 unit tests passing
- ✅ 60+ integration tests passing
- ✅ Zero compilation errors
- ✅ 100% major feature coverage

## Configuration

Create a `.datacell.toml` file for default options:

```bash
xls-rs config-init
```

Example configuration:
```toml
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

## License

Apache-2.0

## Contributing

See [CLAUDE.md](CLAUDE.md) for development guidance and architectural details.
