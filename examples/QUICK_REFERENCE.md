# datacell Quick Reference Card

## Installation
```bash
cargo build --release
export PATH="$PATH:$(pwd)/target/release"
```

## File I/O

### Read Files
```bash
datacell read --input data.csv                    # Read CSV
datacell read --input data.xlsx                   # Read Excel
datacell read --input data.parquet                # Read Parquet
datacell read --input data.avro                   # Read Avro
datacell read --input data.csv --format json      # Output as JSON
datacell read --input data.csv --format markdown  # Output as Markdown
datacell read --input data.xlsx --range "A1:C10"  # Read range
datacell sheets --input data.xlsx                 # List sheets
```

### Convert Formats
```bash
datacell convert --input data.csv --output data.xlsx      # CSV → Excel
datacell convert --input data.xlsx --output data.csv      # Excel → CSV
datacell convert --input data.csv --output data.parquet   # CSV → Parquet
datacell convert --input data.parquet --output data.csv   # Parquet → CSV
datacell convert --input data.csv --output data.avro      # CSV → Avro
datacell convert --input data.avro --output data.csv      # Avro → CSV
```

## Formulas

### Basic Arithmetic
```bash
datacell formula --input data.csv --output result.csv --formula "A1+B1" --cell C1
datacell formula --input data.csv --output result.csv --formula "A1*B1" --cell C1
```

### Aggregate Functions
```bash
datacell formula --input data.csv --output result.csv --formula "SUM(A1:A10)" --cell B1
datacell formula --input data.csv --output result.csv --formula "AVERAGE(A1:A10)" --cell B1
datacell formula --input data.csv --output result.csv --formula "MIN(A1:A10)" --cell B1
datacell formula --input data.csv --output result.csv --formula "MAX(A1:A10)" --cell B1
datacell formula --input data.csv --output result.csv --formula "COUNT(A1:A10)" --cell B1
```

### Conditional Functions
```bash
datacell formula --input data.csv --output result.csv --formula "IF(A1>10,\"High\",\"Low\")" --cell B1
datacell formula --input data.csv --output result.csv --formula "SUMIF(A1:A10,\">5\",B1:B10)" --cell C1
datacell formula --input data.csv --output result.csv --formula "COUNTIF(A1:A10,\">5\")" --cell B1
```

### Lookup & Text
```bash
datacell formula --input data.csv --output result.csv --formula "VLOOKUP(2,A1:C10,3)" --cell D1
datacell formula --input data.csv --output result.csv --formula "CONCAT(A1,\" \",B1)" --cell C1
datacell formula --input data.csv --output result.csv --formula "LEN(A1)" --cell B1
datacell formula --input data.csv --output result.csv --formula "ROUND(A1,2)" --cell B1
datacell formula --input data.csv --output result.csv --formula "ABS(A1)" --cell B1
```

## Data Operations

### Sort & Filter
```bash
datacell sort --input data.csv --output sorted.csv --column Amount
datacell sort --input data.csv --output sorted.csv --column Amount --descending
datacell filter --input data.csv --output filtered.csv --where "Amount > 1000"
datacell filter --input data.csv --output filtered.csv --where "Status = 'active'"
```

### Transform
```bash
datacell replace --input data.csv --output replaced.csv --find "old" --replace "new"
datacell dedupe --input data.csv --output unique.csv
datacell transpose --input data.csv --output transposed.csv
```

## Pandas-Style Operations

### Selection
```bash
datacell head --input data.csv -n 5                                    # First 5 rows
datacell tail --input data.csv -n 5                                    # Last 5 rows
datacell sample --input data.csv -n 10 --seed 42                       # Random sample
datacell select --input data.csv --output subset.csv --columns "A,B"  # Select columns
datacell drop --input data.csv --output slim.csv --columns "X,Y"      # Drop columns
```

### Statistics
```bash
datacell describe --input data.csv --format markdown        # Summary statistics
datacell value-counts --input data.csv --column Category    # Value counts
datacell corr --input data.csv --columns "A,B" --format md  # Correlation
datacell dtypes --input data.csv --format markdown          # Column types
datacell info --input data.csv --format markdown            # Dataset info
datacell unique --input data.csv --column Category          # Unique values
```

### Aggregation
```bash
datacell groupby --input data.csv --output grouped.csv --by Category --agg "sum:Amount,count:ID"
datacell pivot --input data.csv --output pivot.csv --index Region --columns Product --values Sales --agg sum
```

### Joining
```bash
datacell join --left a.csv --right b.csv --output merged.csv --on ID --how left
datacell concat --inputs "a.csv,b.csv,c.csv" --output combined.csv
```

### Missing Data
```bash
datacell fillna --input data.csv --output filled.csv --value "0"
datacell dropna --input data.csv --output clean.csv
```

### Transformation
```bash
datacell rename --input data.csv --output renamed.csv --from "OldName" --to "NewName"
datacell mutate --input data.csv --output result.csv --column Total --formula "Price * Qty"
datacell astype --input data.csv --output result.csv --column Age -t int
datacell query --input data.csv --output filtered.csv -w "Price > 100"
```

## Advanced Operations

### Transform
```bash
datacell clip --input data.csv --output clipped.csv --column Price --min 0 --max 1000
datacell normalize --input data.csv --output normalized.csv --column Price
datacell parse-date --input data.csv --output result.csv --column Date --from-format "%Y-%m-%d" --to-format "%d/%m/%Y"
datacell regex-filter --input data.csv --output filtered.csv --column Name --pattern "^[A-M]"
datacell regex-replace --input data.csv --output result.csv --column Text --pattern "old" --replacement "new"
```

### Analysis
```bash
datacell validate --input data.csv --rules rules.json --output validated.csv --report report.json
datacell profile --input data.csv --output profile.json --report quality.md
datacell text-analysis --input data.csv --column Text --operation stats
datacell detect-anomalies --input data.csv --column Value --method zscore --threshold 3.0 --output anomalies.json
```

### Time Series & Geospatial
```bash
datacell resample --input data.csv --output resampled.csv --date-column Date --value-column Value --interval daily --aggregation sum
datacell geo-distance --from "40.7128,-74.0060" --to "34.0522,-118.2437" --unit km
```

### Security
```bash
datacell encrypt --input data.csv --output encrypted.csv --key "secret" --algorithm aes256
datacell decrypt --input encrypted.csv --output decrypted.csv --key "secret" --algorithm aes256
```

## Visualization

### Styled Excel
```bash
datacell export-styled --input data.csv --output styled.xlsx --header-bg 4472C4 --header-font FFFFFF
```

### Charts
```bash
datacell chart --input data.csv --output chart.xlsx -t column --title "Sales"
datacell chart --input data.csv --output chart.xlsx -t bar --title "Comparison"
datacell chart --input data.csv --output chart.xlsx -t line --title "Trend"
datacell chart --input data.csv --output chart.xlsx -t pie --title "Distribution"
datacell chart --input data.csv --output chart.xlsx -t area --title "Area Chart"
datacell chart --input data.csv --output chart.xlsx -t scatter --title "Scatter Plot"
```

## Configuration

### Setup
```bash
datacell config-init --output .datacell.toml
datacell completions bash >> ~/.bashrc
datacell completions zsh >> ~/.zshrc
datacell completions fish > ~/.config/fish/completions/datacell.fish
```

### Batch Processing
```bash
datacell batch --inputs "*.csv" --output-dir processed/ --operation sort --args '{"column":"Price","desc":true}'
```

## Common Patterns

### ETL Pipeline
```bash
# Extract, Transform, Load
datacell convert --input raw.xlsx --output data.csv
datacell filter --input data.csv --output filtered.csv --where "Status = 'active'"
datacell sort --input filtered.csv --output sorted.csv --column Date --descending
datacell convert --input sorted.csv --output final.parquet
```

### Data Cleaning
```bash
datacell dedupe --input raw.csv --output step1.csv
datacell dropna --input step1.csv --output step2.csv
datacell filter --input step2.csv --output clean.csv --where "Amount > 0"
```

### Report Generation
```bash
datacell groupby --input sales.csv --output summary.csv --by Region --agg "sum:Sales,count:Orders"
datacell export-styled --input summary.csv --output report.xlsx --header-bg 4472C4
datacell chart --input summary.csv --output report_chart.xlsx -t column --title "Sales by Region"
```

### Data Analysis
```bash
datacell describe --input data.csv --format markdown > stats.md
datacell corr --input data.csv --columns "Price,Quantity,Revenue" --format markdown > correlation.md
datacell value-counts --input data.csv --column Category > distribution.txt
```

## Tips & Tricks

### Piping with jq
```bash
datacell read --input data.csv --format json | jq '.[] | select(.Amount > 1000)'
```

### Combining Operations
```bash
datacell filter --input data.csv --output temp.csv --where "Status = 'active'"
datacell sort --input temp.csv --output final.csv --column Date --descending
rm temp.csv
```

### Large Files
```bash
# Use streaming for large files
datacell head --input large.csv -n 1000 > sample.csv
datacell convert --input large.csv --output large.parquet  # Parquet is more efficient
```

### Validation Workflow
```bash
datacell validate --input data.csv --rules rules.json --report validation.json
datacell profile --input data.csv --report quality.md
datacell detect-anomalies --input data.csv --column Revenue --method zscore --threshold 3.0 --output anomalies.json
```

## Error Handling

### Check File Format
```bash
file data.csv
datacell read --input data.csv --format json | jq . | head
```

### Debug Mode
```bash
RUST_LOG=debug datacell read --input data.csv
```

### Validate Output
```bash
datacell convert --input data.csv --output data.xlsx
datacell read --input data.xlsx > /dev/null && echo "Success" || echo "Failed"
```

## Performance

### Benchmark Operations
```bash
time datacell convert --input large.csv --output large.parquet
time datacell sort --input large.csv --output sorted.csv --column Amount
```

### Memory Usage
```bash
/usr/bin/time -l datacell convert --input large.csv --output large.xlsx
```

## Getting Help

```bash
datacell --help                    # General help
datacell read --help               # Command-specific help
datacell formula --help            # Formula help
```

## Resources

- Full Documentation: `README.md`
- Testing Guide: `examples/TESTING_GUIDE.md`
- Architecture: `ARCHITECTURE.md`
- Development: `CLAUDE.md`
