# datacell Comprehensive Testing Guide

This guide provides comprehensive examples to test all datacell capabilities.

## Quick Start

### 1. Build datacell
```bash
cd ..
cargo build --release
cd examples
```

### 2. Generate Test Data
```bash
chmod +x test_data_generator.sh
./test_data_generator.sh
```

### 3. Run All Capability Tests
```bash
chmod +x test_all_capabilities.sh
./test_all_capabilities.sh
```

### 4. Run Rust API Examples
```bash
cargo run --example api_usage
```

### 5. Run Integration Tests
```bash
cargo test --example integration_tests
```

## Test Categories

### 1. File Format I/O (8 tests)
- **CSV Reading**: Read CSV files with various encodings
- **Excel Reading**: Read XLSX files with multiple sheets
- **Parquet Reading**: Read columnar Parquet files
- **Avro Reading**: Read Avro binary files
- **JSON Output**: Export data as JSON
- **Markdown Output**: Export data as Markdown tables
- **Range Reading**: Read specific cell ranges (A1:C10)
- **Multi-sheet Operations**: List and read all sheets

**Example Commands:**
```bash
datacell read --input employees.csv
datacell read --input employees.xlsx --sheet "Sheet1"
datacell read --input employees.parquet
datacell read --input employees.avro
datacell read --input employees.csv --format json
datacell read --input employees.csv --format markdown
datacell read --input employees.xlsx --range "A1:C5"
datacell sheets --input employees.xlsx
```

### 2. Format Conversions (12 tests)
- **CSV → Excel/Parquet/Avro**
- **Excel → CSV/Parquet/Avro**
- **Parquet → CSV/Excel/Avro**
- **Avro → CSV/Excel/Parquet**

**Example Commands:**
```bash
datacell convert --input employees.csv --output employees.xlsx
datacell convert --input employees.xlsx --output employees.csv
datacell convert --input employees.csv --output employees.parquet
datacell convert --input employees.parquet --output employees.csv
datacell convert --input employees.csv --output employees.avro
datacell convert --input employees.avro --output employees.csv
```

### 3. Formula Evaluation (15 tests)
- **Arithmetic**: +, -, *, /
- **Aggregates**: SUM, AVERAGE, MIN, MAX, COUNT
- **Conditionals**: IF, SUMIF, COUNTIF
- **Lookup**: VLOOKUP
- **Text**: CONCAT, LEN
- **Math**: ROUND, ABS

**Example Commands:**
```bash
datacell formula --input numbers.csv --output result.csv --formula "A1+B1" --cell "C1"
datacell formula --input sales.csv --output result.csv --formula "SUM(C2:C10)" --cell "C11"
datacell formula --input sales.csv --output result.csv --formula "AVERAGE(C2:C10)" --cell "D11"
datacell formula --input sales.csv --output result.csv --formula "IF(C2>1000,\"High\",\"Low\")" --cell "D2"
datacell formula --input sales.csv --output result.csv --formula "SUMIF(B2:B10,\"Electronics\",C2:C10)" --cell "E2"
datacell formula --input lookup.csv --output result.csv --formula "VLOOKUP(2,A1:C5,3)" --cell "D1"
datacell formula --input employees.csv --output result.csv --formula "CONCAT(A2,\" \",B2)" --cell "E2"
```

### 4. Data Operations (10 tests)
- **Sorting**: Ascending/descending by column
- **Filtering**: SQL-like WHERE conditions
- **Find/Replace**: Text replacement
- **Deduplication**: Remove duplicate rows
- **Transpose**: Swap rows and columns
- **Append**: Add data to existing files

**Example Commands:**
```bash
datacell sort --input sales.csv --output sorted.csv --column Amount
datacell sort --input sales.csv --output sorted.csv --column Amount --descending
datacell filter --input sales.csv --output filtered.csv --where "Amount > 1000"
datacell replace --input sales.csv --output replaced.csv --find "Electronics" --replace "Tech"
datacell dedupe --input duplicates.csv --output unique.csv
datacell transpose --input employees.csv --output transposed.csv
```

### 5. Pandas-Style Operations (25 tests)
- **Selection**: head, tail, sample, select, drop
- **Statistics**: describe, value_counts, correlation
- **Grouping**: groupby, pivot
- **Joining**: join, concat
- **Missing Data**: fillna, dropna
- **Transformation**: rename, mutate, astype
- **Query**: SQL-like filtering
- **Info**: dtypes, unique, info

**Example Commands:**
```bash
datacell head --input employees.csv -n 5
datacell tail --input employees.csv -n 5
datacell sample --input employees.csv -n 10 --seed 42
datacell select --input employees.csv --output subset.csv --columns "Name,Department"
datacell drop --input employees.csv --output slim.csv --columns "Salary"
datacell describe --input financial_data.csv --format markdown
datacell value-counts --input sales.csv --column Category
datacell groupby --input sales.csv --output summary.csv --by Category --agg "sum:Amount,count:Product"
datacell pivot --input sales.csv --output pivot.csv --index Category --columns Product --values Amount --agg sum
datacell join --left sales.csv --right lookup.csv --output merged.csv --on Category --how left
datacell concat --inputs "sales.csv,sales.csv" --output combined.csv
datacell fillna --input financial_data.csv --output filled.csv --value "0"
datacell dropna --input financial_data.csv --output clean.csv
datacell rename --input employees.csv --output renamed.csv --from "Name" --to "EmployeeName"
datacell mutate --input sales.csv --output result.csv --column "DoubleAmount" --formula "Amount * 2"
datacell dtypes --input employees.csv --format markdown
datacell unique --input sales.csv --column Category
datacell info --input employees.csv --format markdown
datacell query --input sales.csv --output filtered.csv -w "Amount > 500"
datacell corr --input financial_data.csv --columns "Revenue,Profit" --format markdown
```

### 6. Transform Operations (8 tests)
- **Clipping**: Limit values to range
- **Normalization**: Scale to 0-1
- **Date Parsing**: Convert date formats
- **Regex Filter**: Filter by pattern
- **Regex Replace**: Replace by pattern
- **Type Casting**: Convert column types

**Example Commands:**
```bash
datacell clip --input sales.csv --output clipped.csv --column Amount --min 500 --max 2000
datacell normalize --input sales.csv --output normalized.csv --column Amount
datacell parse-date --input financial_data.csv --output result.csv --column Date --from-format "%Y-%m-%d" --to-format "%d/%m/%Y"
datacell regex-filter --input employees.csv --output filtered.csv --column Name --pattern "^[A-M]"
datacell regex-replace --input sales.csv --output result.csv --column Category --pattern "Electronics" --replacement "Tech"
datacell astype --input sales.csv --output result.csv --column Amount -t int
```

### 7. Advanced Features (10 tests)
- **Validation**: Rule-based data validation
- **Profiling**: Data quality profiling
- **Text Analysis**: Text statistics and sentiment
- **Time Series**: Resampling and aggregation
- **Geospatial**: Distance calculations
- **Anomaly Detection**: Statistical outlier detection
- **Encryption**: AES256/XOR encryption

**Example Commands:**
```bash
datacell validate --input employees.csv --rules validation_rules.json --output validated.csv --report report.json
datacell profile --input employees.csv --output profile.json --report quality_report.md
datacell text-analysis --input employees.csv --column Name --operation stats
datacell resample --input financial_data.csv --output resampled.csv --date-column Date --value-column Revenue --interval daily --aggregation sum
datacell geo-distance --from "40.7128,-74.0060" --to "34.0522,-118.2437" --unit km
datacell detect-anomalies --input financial_data.csv --column Revenue --method zscore --threshold 3.0 --output anomalies.json
datacell encrypt --input employees.csv --output encrypted.csv --key "secretkey123" --algorithm aes256
datacell decrypt --input encrypted.csv --output decrypted.csv --key "secretkey123" --algorithm aes256
```

### 8. Styling and Visualization (6 tests)
- **Styled Excel**: Headers, colors, borders
- **Charts**: Column, bar, line, pie, area, scatter, doughnut

**Example Commands:**
```bash
datacell export-styled --input employees.csv --output styled.xlsx --header-bg 4472C4 --header-font FFFFFF
datacell chart --input sales.csv --output chart.xlsx -t column --title "Sales by Product"
datacell chart --input sales.csv --output chart.xlsx -t bar --title "Sales Comparison"
datacell chart --input sales.csv --output chart.xlsx -t line --title "Sales Trend"
datacell chart --input sales.csv --output chart.xlsx -t pie --title "Sales Distribution"
```

### 9. Configuration (3 tests)
- **Config Init**: Create .datacell.toml
- **Shell Completions**: bash, zsh, fish, powershell

**Example Commands:**
```bash
datacell config-init --output .datacell.toml
datacell completions bash > completions.bash
datacell completions zsh > completions.zsh
datacell completions fish > completions.fish
```

### 10. Batch Processing (2 tests)
- **Batch Operations**: Process multiple files with glob patterns

**Example Commands:**
```bash
datacell batch --inputs "*.csv" --output-dir processed/ --operation sort --args '{"column":"A","desc":false}'
```

## Test Data Files

### Existing Files
- `employees.csv/xlsx/parquet/avro` - Employee data with Name, Department, Salary
- `sales.csv/xlsx/parquet/avro` - Sales data with Product, Category, Amount
- `lookup.csv/avro` - Lookup table for VLOOKUP tests
- `numbers.csv/parquet` - Numeric data for formula tests
- `duplicates.csv` - Data with duplicate rows
- `financial_data.csv` - Financial data with dates
- `validation_rules.json` - Validation rules configuration

### Generated Test Files
Run `./test_data_generator.sh` to create:
- `products.csv` - Product catalog (10 products)
- `orders.csv` - Order transactions (8 orders)
- `customers.csv` - Customer data (5 customers)
- `timeseries.csv` - Time series data (14 rows)
- `missing_data.csv` - Data with missing values (6 rows)
- `text_data.csv` - Text analysis data (5 rows)
- `coordinates.csv` - Geospatial coordinates (5 locations)
- `anomaly_data.csv` - Anomaly detection data (10 rows)
- `multi_type.csv` - Multiple data types (5 rows)
- `large_numbers.csv` - Large numeric values (5 rows)
- `dates_various.csv` - Various date formats (5 rows)
- `regex_test.csv` - Regex pattern testing (5 rows)
- `correlation_data.csv` - Correlation analysis (7 rows)
- `pivot_data.csv` - Pivot table data (12 rows)

## API Usage Examples

See `api_usage.rs` for programmatic usage examples:

```rust
use datacell::*;

// Read CSV
let csv_handler = csv_handler::CsvHandler::new();
let data = csv_handler.read("employees.csv")?;

// Convert formats
let converter = converter::Converter::new();
converter.convert("input.csv", "output.xlsx", None, None)?;

// Apply formulas
let evaluator = formula::FormulaEvaluator::new();
let result = evaluator.evaluate("SUM(A1:A10)", &data)?;

// Data operations
let ops = operations::DataOperations::new();
let sorted = ops.sort(&data, "Amount", false)?;
let filtered = ops.filter(&data, "Amount > 1000")?;
```

## Integration Tests

Run comprehensive integration tests:

```bash
cargo test --example integration_tests
```

This runs 40+ integration tests covering:
- Round-trip conversions
- Formula evaluation
- Data operations
- Pandas-style operations
- Streaming
- Validation and profiling
- Encryption
- Plugins
- Geospatial
- Anomaly detection
- Text analysis
- Time series

## Performance Testing

### Large File Testing
```bash
# Generate large CSV (1M rows)
seq 1 1000000 | awk '{print $1","$1*2","$1*3}' > large.csv

# Test read performance
time datacell read --input large.csv > /dev/null

# Test conversion performance
time datacell convert --input large.csv --output large.parquet

# Test streaming
time datacell head --input large.csv -n 100
```

### Memory Testing
```bash
# Monitor memory usage during operations
/usr/bin/time -l datacell convert --input large.csv --output large.xlsx
```

## Troubleshooting

### Build Issues
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Check for missing dependencies
cargo check
```

### Test Failures
```bash
# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_csv_round_trip -- --nocapture
```

### Data Issues
```bash
# Validate input data
datacell read --input data.csv --format json | jq .

# Check file format
file data.csv
```

## Coverage Report

Total capabilities tested: **80+**

| Category | Tests | Coverage |
|----------|-------|----------|
| File I/O | 8 | 100% |
| Conversions | 12 | 100% |
| Formulas | 15 | 100% |
| Data Ops | 10 | 100% |
| Pandas Ops | 25 | 100% |
| Transforms | 8 | 100% |
| Advanced | 10 | 100% |
| Visualization | 6 | 100% |
| Config | 3 | 100% |
| Batch | 2 | 100% |

## Next Steps

1. **Extend Tests**: Add more edge cases and error handling tests
2. **Benchmarking**: Create performance benchmarks for each operation
3. **CI/CD**: Integrate tests into continuous integration pipeline
4. **Documentation**: Add more inline documentation and examples
5. **Error Cases**: Test error handling and recovery scenarios

## Contributing

When adding new capabilities:
1. Add test data to `examples/`
2. Add CLI test to `test_all_capabilities.sh`
3. Add API example to `api_usage.rs`
4. Add integration test to `integration_tests.rs`
5. Update this guide with new test cases
6. Update `TODO.md` with completion status
