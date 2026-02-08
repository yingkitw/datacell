# datacell Specification

## Project Overview

**datacell** is a fast, unified CLI tool for spreadsheet and columnar data manipulation, built in Rust. It provides a single binary that handles CSV, Excel (XLSX/XLS/ODS), Parquet, and Avro files with Excel-like formulas, pandas-style operations, and data transformations.

## Core Requirements

### 1. File Format Support

#### Input Formats
- **CSV**: Standard comma-separated values with configurable delimiters
- **Excel**: XLSX, XLS, ODS (OpenDocument Spreadsheet)
- **Parquet**: Apache Parquet columnar format
- **Avro**: Apache Avro binary format

#### Output Formats
- **CSV**: Standard output with headers
- **Excel**: XLSX with styling support
- **Parquet**: Compressed columnar format
- **Avro**: Schema-based binary format
- **JSON**: Structured JSON output
- **Markdown**: Formatted tables for documentation

### 2. Formula Evaluation

#### Arithmetic Operations
- Basic operators: `+`, `-`, `*`, `/`
- Cell references: `A1`, `B2`, etc.
- Range references: `A1:A10`

#### Aggregate Functions
- `SUM(range)` - Sum of values
- `AVERAGE(range)` - Mean of values
- `MIN(range)` - Minimum value
- `MAX(range)` - Maximum value
- `COUNT(range)` - Count of numeric values

#### Conditional Functions
- `IF(condition, true_value, false_value)` - Conditional logic
- `SUMIF(range, criteria, sum_range)` - Conditional sum
- `COUNTIF(range, criteria)` - Conditional count

#### Lookup Functions
- `VLOOKUP(lookup_value, table_range, col_index)` - Vertical lookup

#### Text Functions
- `CONCAT(text1, text2, ...)` - String concatenation
- `LEN(text)` - String length

#### Math Functions
- `ROUND(number, decimals)` - Round to decimal places
- `ABS(number)` - Absolute value

### 3. Data Operations

#### Core Operations
- **Sort**: Sort by column (ascending/descending)
- **Filter**: SQL-like WHERE clause filtering
- **Replace**: Find and replace values
- **Dedupe**: Remove duplicate rows
- **Transpose**: Swap rows and columns
- **Append**: Add data to existing files

#### Pandas-Style Operations
- **Selection**: `head`, `tail`, `sample`, `select`, `drop`
- **Statistics**: `describe`, `value_counts`, `corr`
- **Grouping**: `groupby` with aggregations (sum, count, mean, min, max)
- **Joining**: `join` (left, right, inner, outer), `concat`
- **Missing Data**: `fillna`, `dropna`
- **Transformation**: `rename`, `mutate`, `astype`
- **Query**: SQL-like filtering with WHERE clause
- **Info**: `dtypes`, `unique`, `info`

#### Transform Operations
- **Clip**: Limit values to min/max range
- **Normalize**: Scale column to 0-1 range
- **Date Parsing**: Convert between date formats
- **Regex**: Filter and replace with patterns

### 4. Advanced Features

#### Data Quality
- **Validation**: Rule-based data validation with JSON rules
- **Profiling**: Data quality profiling and statistics
- **Quality Reports**: Automated quality assessment

#### Security
- **Encryption**: XOR and AES256 file encryption
- **Decryption**: Decrypt encrypted files

#### Visualization
- **Styled Excel**: Export with headers, colors, borders
- **Charts**: Column, bar, line, pie, area, scatter, doughnut charts

#### Configuration
- **Config File**: `.datacell.toml` for default settings
- **Shell Completions**: bash, zsh, fish, powershell

#### Integration
- **MCP Server**: Model Context Protocol server for AI assistants
- **Batch Processing**: Process multiple files with glob patterns
- **Streaming**: Handle large files efficiently
- **Plugins**: Extensible plugin system

### 5. Performance Requirements

- **Fast Startup**: < 100ms for simple operations
- **Memory Efficient**: Stream processing for large files
- **Single Binary**: No runtime dependencies
- **Cross-Platform**: macOS, Linux, Windows

### 6. API Design

#### Command Structure
```bash
datacell <command> [options]
```

#### Common Options
- `-i, --input <INPUT>` - Input file path
- `-o, --output <OUTPUT>` - Output file path
- `-s, --sheet <SHEET>` - Sheet name for Excel files
- `-f, --format <FORMAT>` - Output format (csv, json, markdown)

#### Error Handling
- Clear error messages with context
- File path, row, and column information
- Suggestions for common mistakes

### 7. Testing Requirements

#### Unit Tests
- 189+ unit tests covering all modules
- Test coverage for formulas, operations, conversions
- Mock implementations for testing

#### Integration Tests
- End-to-end CLI testing
- 60+ capability tests across 10 categories
- Automated test suite with exit code validation

#### Test Categories
1. File Format I/O (8 tests)
2. Format Conversions (12 tests)
3. Formula Evaluation (15 tests)
4. Data Operations (10 tests)
5. Pandas-Style Operations (25 tests)
6. Transform Operations (8 tests)
7. Advanced Features (7 tests)
8. Styling and Visualization (6 tests)
9. Configuration (3 tests)
10. Batch Processing (1 test)

### 8. Documentation Requirements

- **README.md**: Overview, quick start, features, usage examples
- **ARCHITECTURE.md**: System design, modules, data flow
- **TESTING_GUIDE.md**: Comprehensive testing documentation
- **QUICK_REFERENCE.md**: Command reference card
- **EXAMPLES_SUMMARY.md**: Test suite overview
- **TODO.md**: Project status and roadmap

### 9. Quality Standards

- **Code Quality**: Zero compiler warnings, clean code
- **Test Coverage**: 100% of major features tested
- **Documentation**: All commands documented with examples
- **Performance**: Benchmarked against pandas and xsv
- **Reliability**: All tests passing, no known bugs

### 10. Future Enhancements

#### Planned Features
- Real-time data streaming support
- Data lineage tracking
- REST API server mode
- Web UI for interactive data exploration
- Machine learning integration
- Cloud storage support (S3, GCS, Azure)

#### Optimization Opportunities
- Parallel processing for large datasets
- Query optimization for complex operations
- Caching for repeated operations
- Incremental updates for large files

## Custom XLSX Writer Design

datacell generates Excel files using a from-scratch XLSX writer rather than an external library. This section documents the design rationale and implementation.

### Motivation

XLSX files are ZIP archives containing XML files that follow the ECMA-376 Office Open XML (OOXML) standard. Several Rust crates exist for writing XLSX, but each had drawbacks for our use case:

| Crate | Drawback |
|-------|----------|
| `rust_xlsxwriter` | Heavy dependency tree; version conflicts with our `zip` crate usage |
| `simple_excel_writer` | No formula support; limited cell types |
| `xlsxwriter` (C FFI) | Requires system C library; complicates cross-compilation and CI |

We chose to generate the XML directly using only the `zip` crate. This gives us a smaller binary, full control over output, and no external library version conflicts. The trade-off is that we must implement OOXML compliance ourselves, and advanced features like charts require significant XML markup work.

### OOXML Structure

A valid XLSX file contains these required entries in the ZIP archive:

```
[Content_Types].xml          # MIME types for all parts
_rels/.rels                  # Top-level relationships
xl/workbook.xml              # Workbook with sheet list
xl/_rels/workbook.xml.rels   # Workbook relationships
xl/styles.xml                # Fonts, fills, borders, cell formats
xl/theme/theme1.xml          # Color/font theme
xl/worksheets/sheet1.xml     # Worksheet data (one per sheet)
```

### Key Implementation Details

The writer lives in `src/excel/xlsx_writer/` with three files:
- **`mod.rs`** — `XlsxWriter` struct, sheet/row/cell API
- **`types.rs`** — `CellData`, `RowData`, `SheetData` types
- **`xml_gen.rs`** — All XML generation functions

Critical OOXML elements that Excel/Numbers require (discovered by byte-level comparison with openpyxl output):
- `<sheetPr>` with `<outlinePr>` and `<pageSetUpPr/>`
- `<sheetFormatPr baseColWidth="8" defaultRowHeight="15"/>`
- `<selection>` inside `<sheetView>` for cursor positioning
- `<pageMargins>` at the end of each worksheet
- `<workbookPr/>`, `<bookViews>`, `<calcPr>` in workbook.xml
- `<diagonal/>` in every `<border>` element
- `<tableStyles>` in styles.xml
- Number cells with explicit `t="n"` type attribute
- Theme with `lnStyleLst` and triple entries in fill/effect/bg style lists

### Supported Features
- Multiple sheets (max 31-char names, invalid character validation)
- Cell types: String (inline), Number, Formula, Empty
- Column widths (auto-fit and manual)
- Freeze header row
- Auto-filter
- Basic styling (bold, fills, borders, alignment)

### Not Yet Implemented
- Charts (requires `xl/drawings/`, `xl/charts/`, complex relationship XML)
- Sparklines
- Conditional formatting (color scales, data bars, icon sets)
- Merged cells
- Data validation dropdowns
- Pivot tables

## Technical Stack

- **Language**: Rust (Edition 2024)
- **CLI Framework**: clap (derive macros)
- **Excel Read**: calamine (reads .xls, .xlsx, .ods)
- **Excel Write**: Custom OOXML writer using `zip` crate (no external Excel library)
- **CSV**: csv crate
- **Parquet**: parquet + arrow (v54)
- **Avro**: apache-avro
- **Async**: tokio
- **Serialization**: serde_json
- **Error Handling**: anyhow, thiserror
- **Testing**: cargo test, integration tests

## Success Criteria

1. ✅ All 189 unit tests passing
2. ✅ All 60+ integration tests passing
3. ✅ Zero compilation errors or warnings
4. ✅ Comprehensive documentation complete
5. ✅ Single binary deployment
6. ✅ Cross-platform compatibility
7. ✅ Performance benchmarks met
8. ✅ User-friendly CLI with help text
9. ✅ Extensible architecture
10. ✅ Production-ready quality

## Version History

- **v0.1.3** (Jan 2026): Comprehensive test suite, 189 tests passing, full CLI implementation
- **v0.1.2**: Added pandas-style operations, advanced features
- **v0.1.1**: Formula evaluation, data operations
- **v0.1.0**: Initial release with basic CSV/Excel support
