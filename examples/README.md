# datacell Examples and Testing

Comprehensive examples and test suite for datacell functionality.

## Quick Start

### Run All Tests
```bash
# Automated test suite (shell script)
chmod +x test_all_capabilities.sh
./test_all_capabilities.sh

# Python test runner with detailed reporting
chmod +x run_tests.py
python3 run_tests.py

# Generate additional test data
chmod +x test_data_generator.sh
./test_data_generator.sh
```

### Run API Examples
```bash
cargo run --example api_usage
```

### Run Integration Tests
```bash
cargo test --example integration_tests
```

## Documentation

- **[TESTING_GUIDE.md](TESTING_GUIDE.md)** - Comprehensive testing guide with 80+ test cases
- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Quick reference card for common commands
- **[test_all_capabilities.sh](test_all_capabilities.sh)** - Automated test runner (bash)
- **[run_tests.py](run_tests.py)** - Python test runner with JSON reporting
- **[api_usage.rs](api_usage.rs)** - Programmatic API usage examples
- **[integration_tests.rs](integration_tests.rs)** - 40+ integration tests

## Sample Data Files

### Existing Files

#### CSV Files
| File | Description | Use Case |
|------|-------------|----------|
| `sales.csv` | Product sales data | Filtering, sorting, SUMIF/COUNTIF |
| `employees.csv` | Employee records | VLOOKUP, sorting, filtering |
| `numbers.csv` | Numeric grid | SUM, AVERAGE, MIN, MAX, formulas |
| `lookup.csv` | Product catalog | VLOOKUP examples |
| `duplicates.csv` | Data with duplicates | Dedupe testing |
| `financial_data.csv` | Financial time series | Date parsing, time series ops |

#### Excel Files
| File | Description | Use Case |
|------|-------------|----------|
| `sales.xlsx` | Product sales (styled) | Excel read, sheet listing |
| `employees.xlsx` | Employee records (styled) | Excel read, JSON export |

#### Parquet Files
| File | Description | Use Case |
|------|-------------|----------|
| `sales.parquet` | Product sales | Columnar format read/write |
| `employees.parquet` | Employee records | Columnar format read/write |
| `numbers.parquet` | Numeric grid | Columnar format read/write |

#### Avro Files
| File | Description | Use Case |
|------|-------------|----------|
| `sales.avro` | Product sales | Avro format read/write |
| `employees.avro` | Employee records | Avro format read/write |
| `lookup.avro` | Product catalog | Avro format read/write |

#### Configuration Files
| File | Description | Use Case |
|------|-------------|----------|
| `validation_rules.json` | Data validation rules | Validation testing |

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

## Example Commands

### Read files
```bash
# Read as CSV
datacell read --input examples/sales.csv

# Read as JSON
datacell read --input examples/employees.csv --format json

# Read as Markdown table
datacell read --input examples/numbers.csv --format markdown
```

### Formulas
```bash
# Sum all prices
datacell formula --input examples/sales.csv --output /tmp/result.csv \
  --formula "SUM(C2:C11)" --cell "F1"

# Average salary
datacell formula --input examples/employees.csv --output /tmp/result.csv \
  --formula "AVERAGE(D2:D11)" --cell "F1"

# VLOOKUP - find price for code A002
datacell formula --input examples/lookup.csv --output /tmp/result.csv \
  --formula "VLOOKUP(\"A002\", A1:D8, 3)" --cell "F1"

# SUMIF - sum prices where quantity > 20
datacell formula --input examples/sales.csv --output /tmp/result.csv \
  --formula "SUMIF(D2:D11, \">20\", C2:C11)" --cell "F1"

# COUNTIF - count Electronics items
datacell formula --input examples/sales.csv --output /tmp/result.csv \
  --formula "COUNTIF(B2:B11, \"Electronics\")" --cell "F1"
```

### Data Operations
```bash
# Sort by price descending
datacell sort --input examples/sales.csv --output /tmp/sorted.csv \
  --column C --descending

# Filter Electronics only
datacell filter --input examples/sales.csv --output /tmp/filtered.csv \
  --column B --op "=" --value "Electronics"

# Filter salary > 70000
datacell filter --input examples/employees.csv --output /tmp/high_salary.csv \
  --column D --op ">" --value "70000"

# Remove duplicates
datacell dedupe --input examples/duplicates.csv --output /tmp/unique.csv

# Transpose data
datacell transpose --input examples/numbers.csv --output /tmp/transposed.csv

# Find and replace
datacell replace --input examples/sales.csv --output /tmp/replaced.csv \
  --find "Electronics" --replace "Tech"
```

### Convert Formats
```bash
# CSV to Excel
datacell convert --input examples/sales.csv --output /tmp/sales.xlsx

# CSV to Parquet
datacell convert --input examples/employees.csv --output /tmp/employees.parquet

# CSV to Avro
datacell convert --input examples/numbers.csv --output /tmp/numbers.avro

# Then convert back
datacell convert --input /tmp/sales.xlsx --output /tmp/sales_back.csv
datacell convert --input /tmp/employees.parquet --output /tmp/employees_back.csv
```

### Read Range
```bash
# Read specific range
datacell read --input examples/numbers.csv --range "A1:B3"

# Read range as JSON
datacell read --input examples/sales.csv --range "A1:C5" --format json
```
