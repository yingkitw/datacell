# datacell

A Rust CLI tool for reading, writing, converting XLS and CSV files with formula support.

## Features

- **Read** XLS, XLSX, and CSV files
- **Write** data to XLS, XLSX, and CSV files
- **Convert** between CSV and Excel formats
- **Apply formulas** to cells in both CSV and Excel files
  - Supports basic arithmetic operations (+, -, *, /)
  - Supports SUM(), AVERAGE(), MIN(), MAX(), COUNT() functions
  - Supports ROUND(), ABS(), LEN() functions
  - Supports IF() for conditional logic
  - Supports CONCAT() for string concatenation
  - Supports cell references (e.g., A1, B2)
- **Data operations**
  - Sort rows by column (ascending/descending)
  - Filter rows by condition
  - Find and replace values
  - Remove duplicate rows
  - Transpose data (rows to columns)
- **Cell range reading** - read specific ranges like A1:C10
- **Multiple output formats** - CSV, JSON, Markdown
- **MCP server** for integration with AI assistants

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/datacell`.

## Usage

### Read a file

```bash
# Read CSV
datacell read --input data.csv

# Read Excel (first sheet)
datacell read --input data.xlsx

# Read specific sheet
datacell read --input data.xlsx --sheet "Sheet2"

# Read specific cell range
datacell read --input data.csv --range "A1:C10"

# Read as JSON
datacell read --input data.csv --format json

# Read range as JSON
datacell read --input data.xlsx --range "B2:D5" --format json

# Read as Markdown table
datacell read --input data.csv --format markdown
```

### Write a file

```bash
# Write CSV from CSV
datacell write --output output.csv --csv input.csv

# Write Excel from CSV
datacell write --output output.xlsx --csv input.csv

# Write Excel with specific sheet name
datacell write --output output.xlsx --csv input.csv --sheet "Data"
```

### Convert between formats

```bash
# CSV to Excel
datacell convert --input data.csv --output data.xlsx

# Excel to CSV
datacell convert --input data.xlsx --output data.csv

# Excel to CSV (specific sheet)
datacell convert --input data.xlsx --output data.csv --sheet "Sheet2"
```

### Apply formulas

```bash
# Apply SUM formula to CSV
datacell formula --input data.csv --output result.csv --formula "SUM(A1:A10)" --cell "C1"

# Apply arithmetic formula
datacell formula --input data.csv --output result.csv --formula "A1+B1" --cell "C1"

# Apply AVERAGE formula to Excel
datacell formula --input data.xlsx --output result.xlsx --formula "AVERAGE(A1:A10)" --cell "B1" --sheet "Sheet1"
```

### Data operations

```bash
# Sort by column A (ascending)
datacell sort --input data.csv --output sorted.csv --column A

# Sort by column B (descending)
datacell sort --input data.csv --output sorted.csv --column B --descending

# Filter rows where column A > 10
datacell filter --input data.csv --output filtered.csv --column A --op ">" --value 10

# Filter rows containing text
datacell filter --input data.csv --output filtered.csv --column B --op contains --value "hello"

# Find and replace
datacell replace --input data.csv --output replaced.csv --find "old" --replace "new"

# Remove duplicate rows
datacell dedupe --input data.csv --output unique.csv

# Transpose (rows to columns)
datacell transpose --input data.csv --output transposed.csv
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
- `IF(A1>10, "High", "Low")` - Conditional logic
- `CONCAT(A1, " ", B1)` - String concatenation
- `A1+B1` - Add values in A1 and B1
- `A1-B1` - Subtract B1 from A1
- `A1*B1` - Multiply A1 by B1
- `A1/B1` - Divide A1 by B1
- `A1` - Reference a single cell

## Architecture

```
datacell/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── excel.rs         # Excel file handling
│   ├── csv_handler.rs   # CSV file handling
│   ├── converter.rs     # Format conversion
│   └── formula.rs       # Formula evaluation
└── Cargo.toml
```

## Dependencies

- `clap` - CLI argument parsing
- `calamine` - Excel file reading (.xls, .xlsx)
- `rust_xlsxwriter` - Excel file writing (.xlsx)
- `csv` - CSV file handling
- `anyhow` - Error handling
- `regex` - Formula parsing

## License

MIT

