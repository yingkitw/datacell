# Example Data Files

Sample data files for testing datacell functionality.

## Files

| File | Description | Use Case |
|------|-------------|----------|
| `sales.csv` | Product sales data | Filtering, sorting, SUMIF/COUNTIF |
| `employees.csv` | Employee records | VLOOKUP, sorting, filtering |
| `numbers.csv` | Numeric grid | SUM, AVERAGE, MIN, MAX, formulas |
| `lookup.csv` | Product catalog | VLOOKUP examples |
| `duplicates.csv` | Data with duplicates | Dedupe testing |

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
