# datacell Architecture

A Rust CLI tool and MCP server for reading, writing, converting spreadsheet files (CSV, XLS, XLSX, Parquet, Avro) with formula support and pandas-inspired operations.

## Overview

```
datacell/
├── src/
│   ├── main.rs          # CLI entry point with clap
│   ├── lib.rs           # Library exports
│   ├── csv_handler.rs   # CSV file operations
│   ├── excel.rs         # Excel file operations (calamine + rust_xlsxwriter)
│   ├── converter.rs     # Format conversion
│   ├── columnar.rs      # Parquet and Avro support
│   ├── config.rs        # Configuration file support
│   ├── error.rs         # Enhanced error types
│   ├── mcp.rs           # MCP server implementation (rmcp)
│   ├── formula/         # Formula evaluation module
│   │   ├── mod.rs       # Module exports
│   │   ├── types.rs     # FormulaResult, CellRange
│   │   ├── evaluator.rs # Main evaluator logic
│   │   ├── functions.rs # SUM, AVERAGE, VLOOKUP, etc.
│   │   └── parser.rs    # Cell reference parsing
│   └── operations/      # Data operations module
│       ├── mod.rs       # Module exports
│       ├── types.rs     # SortOrder, JoinType, AggFunc
│       ├── core.rs      # Basic ops: sort, filter, replace
│       ├── pandas.rs    # head, tail, sample, join, groupby
│       ├── stats.rs     # describe, value_counts, correlation
│       └── transform.rs # query, mutate, astype, normalize
├── tests/
│   └── test_basic.rs    # Integration tests
├── examples/            # Example CSV files
├── Cargo.toml
└── README.md
```

## Core Components

### CellRange (`csv_handler.rs`)
- **parse**: Parse range strings like "A1:C10" or "B2"
- Used for reading specific cell ranges

### CsvHandler (`csv_handler.rs`)
- **read**: Read CSV file contents as string
- **read_range**: Read specific cell range from CSV
- **read_as_json**: Read CSV and return as JSON array
- **write_from_csv**: Copy CSV to another CSV
- **write_records**: Write Vec<Vec<String>> to CSV

### ExcelHandler (`excel.rs`)
- **read_with_sheet**: Read Excel sheet as CSV-formatted string
- **read_range**: Read specific cell range from Excel
- **read_as_json**: Read Excel and return as JSON array
- **write_from_csv**: Convert CSV to Excel
- **parse_cell_reference**: Parse "A1" style references to (row, col)

### Converter (`converter.rs`)
- **convert**: Convert between CSV and Excel formats
- Delegates to CsvHandler and ExcelHandler

### FormulaEvaluator (`formula.rs`)
- **apply_to_csv**: Evaluate formula and write result to CSV
- **apply_to_excel**: Write Excel formula to cell
- Supports: SUM, AVERAGE, MIN, MAX, COUNT, IF, CONCAT, arithmetic (+, -, *, /)

### DatacellMcpServer (`mcp.rs`)
- MCP server exposing tools: read_file, write_file, convert_file, apply_formula
- Uses rmcp 0.10 with stdio transport

## Dependencies

| Crate | Purpose |
|-------|---------|
| clap | CLI argument parsing |
| calamine | Read Excel files (.xls, .xlsx) |
| rust_xlsxwriter | Write Excel files (.xlsx) |
| csv | CSV file handling |
| rmcp | MCP server implementation |
| tokio | Async runtime |
| anyhow | Error handling |
| regex | Formula parsing |
| schemars | JSON schema for MCP |

## Data Flow

```
CLI/MCP Request
      │
      ▼
┌─────────────┐
│   main.rs   │ ◄── CLI parsing (clap)
│   mcp.rs    │ ◄── MCP server (rmcp)
└─────────────┘
      │
      ▼
┌─────────────────────────────────┐
│         Handler Layer           │
├─────────────┬───────────────────┤
│ CsvHandler  │   ExcelHandler    │
└─────────────┴───────────────────┘
      │                │
      ▼                ▼
┌─────────────────────────────────┐
│        Converter / Formula      │
└─────────────────────────────────┘
      │
      ▼
   File I/O
```

## Design Principles

- **DRY**: Handlers are reused by CLI, MCP, and Converter
- **KISS**: Simple struct-based handlers, no complex abstractions
- **Testable**: Each handler can be tested independently
