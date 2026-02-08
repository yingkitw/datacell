# datacell

A fast, unified CLI tool for spreadsheet and columnar data manipulation.

## Features

- **Multi-format support**: CSV, XLSX, XLS, ODS, Parquet, Avro
- **Excel-like formulas**: SUM, AVERAGE, VLOOKUP, IF, and more
- **Data operations**: sort, filter, dedupe, transpose, join, groupby
- **Convert between formats**: One command for any format conversion
- **Output options**: CSV, JSON, Markdown
- **MCP server**: For AI assistant integration

## Installation

```bash
# Homebrew (macOS/Linux)
brew tap yingkitw/datacell
brew install datacell

# From source
cargo build --release
sudo cp target/release/datacell /usr/local/bin/

# Using cargo
cargo install datacell
```

## Quick Start

```bash
# Convert formats
datacell convert --input data.csv --output data.xlsx

# Apply formula
datacell formula --input sales.csv --output result.csv --formula "SUM(C2:C100)" --cell "D1"

# Filter and sort
datacell filter --input data.csv --output filtered.csv --where "status = 'active'"
datacell sort --input filtered.csv --output sorted.csv --column date --descending

# Output as JSON
datacell read --input report.xlsx --format json > report.json
```

## Common Commands

### Read & Write
```bash
# Read any format
datacell read --input data.csv
datacell read --input data.xlsx --sheet "Sheet2"
datacell read --input data.parquet

# Read with format conversion
datacell read --input data.csv --format json
datacell read --input data.csv --format markdown

# Write to any format
datacell write --output output.xlsx --csv input.csv
datacell write --output output.parquet --csv input.csv
```

### Convert Formats
```bash
datacell convert --input data.csv --output data.xlsx
datacell convert --input data.xlsx --output data.parquet
datacell convert --input data.ods --output data.csv
```

### Data Operations
```bash
# Sort
datacell sort --input data.csv --output sorted.csv --column A

# Filter
datacell filter --input data.csv --output filtered.csv --where "price > 100"

# Remove duplicates
datacell dedupe --input data.csv --output unique.csv

# Transpose
datacell transpose --input data.csv --output transposed.csv

# Find and replace
datacell replace --input data.csv --output result.csv --find "old" --replace "new"
```

### Pandas-Style Operations
```bash
# First/last rows
datacell head --input data.csv -n 5
datacell tail --input data.csv -n 5

# Select columns
datacell select --input data.csv --output subset.csv --columns "name,age"

# Summary statistics
datacell describe --input data.csv

# Group and aggregate
datacell groupby --input sales.csv --output summary.csv --by category --agg "sum:amount"

# Join files
datacell join --left orders.csv --right customers.csv --output merged.csv --on customer_id

# Fill/drop missing values
datacell fillna --input data.csv --output filled.csv --value "N/A"
datacell dropna --input data.csv --output clean.csv
```

## Formula Reference

| Formula | Description |
|---------|-------------|
| `SUM(A1:A10)` | Sum of range |
| `AVERAGE(A1:A10)` | Average of range |
| `MIN(A1:A10)` / `MAX(A1:A10)` | Min/Max values |
| `COUNT(A1:A10)` | Count of cells |
| `IF(A1>10, "High", "Low")` | Conditional logic |
| `VLOOKUP(2, A1:C10, 3)` | Lookup value |
| `CONCAT(A1, " ", B1)` | String concatenation |
| `ROUND(A1, 2)` | Round to decimals |
| `ABS(A1)` | Absolute value |
| `LEN(A1)` | Text length |

## Shell Completions

```bash
datacell completions zsh >> ~/.zshrc
datacell completions bash >> ~/.bashrc
datacell completions fish > ~/.config/fish/completions/datacell.fish
```

## Configuration

Generate a config file with default options:

```bash
datacell config-init
```

Example `.datacell.toml`:
```toml
[excel]
header_bold = true
header_bg_color = "4472C4"
auto_filter = true
freeze_header = true

[output]
default_format = "csv"
include_headers = true
```

## Custom XLSX Writer

datacell generates Excel files using a **from-scratch XLSX writer** — no external Excel writing library needed. The writer produces Office Open XML (OOXML) files directly as ZIP archives containing XML, validated against the ECMA-376 standard.

### Why Build From Scratch?

We tried several existing Rust crates for XLSX writing and ran into issues:

| Approach | Issue |
|----------|-------|
| `rust_xlsxwriter` | Worked well initially, but added a heavy dependency tree and had version compatibility friction with the `zip` crate |
| `simple_excel_writer` | Limited feature set, no formula support |
| `xlsxwriter` (C bindings) | Requires system-level C library, complicates cross-compilation |

Building our own writer using just the `zip` crate gave us full control over the XML output, a smaller dependency footprint, and the ability to fix compatibility issues directly. The trade-off is that advanced features (charts, conditional formatting) require implementing complex XML markup ourselves.

### What It Supports
- **Multiple sheets** with name validation
- **Cell types**: String, Number, Formula, Empty
- **Column widths** (auto-fit and manual)
- **Freeze headers** (frozen top row)
- **Auto-filter** for table columns
- **Basic styling** (bold headers, alignment, borders, fills)
- **Proper OOXML structure** — opens in Excel, Numbers, LibreOffice, Google Sheets

### Usage

```bash
# Convert CSV to XLSX
datacell convert --input data.csv --output data.xlsx

# Export with styled headers, freeze panes, and auto-filter
datacell export-styled --input data.csv --output styled.xlsx

# Write formulas to XLSX
datacell formula --input data.csv --output result.xlsx --formula "SUM(C2:C100)" --cell "D1"
```

```rust
// Programmatic usage
use datacell::excel::xlsx_writer::{XlsxWriter, RowData};

let mut writer = XlsxWriter::new();
writer.add_sheet("Sales")?;

let mut header = RowData::new();
header.add_string("Product");
header.add_string("Revenue");
writer.add_row(header);

let mut row = RowData::new();
row.add_string("Widget");
row.add_number(1234.56);
writer.add_row(row);

let file = std::fs::File::create("output.xlsx")?;
writer.save(&mut std::io::BufWriter::new(file))?;
```

### Current Limitations
- **Charts** — not yet implemented (requires complex XML drawing/chart markup)
- **Sparklines** — not yet implemented
- **Conditional formatting** — not yet implemented (color scales, data bars, icon sets)
- **Merged cells** — not yet implemented
- **Rich text within cells** — not supported

For charts and visualizations, consider exporting to CSV and using dedicated plotting tools (e.g., Python matplotlib, gnuplot).

## MCP Server

Start the MCP server for AI assistant integration:

```bash
datacell serve
```
