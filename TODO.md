# datacell TODO

## Immediate Improvements

- [x] Replace placeholder CLI handlers with real implementations
  - [x] Implement `config-init`
  - [x] Implement `export-styled`
  - [x] Implement transform handlers: clip/normalize/parse-date/regex-filter/regex-replace
- [x] Align README/ARCHITECTURE with actual supported formats and commands
- [x] Ensure `cargo test` passes after CLI refactor
- [x] Fix all compiler warnings (unused imports, unused variables)
- [x] Update documentation (README.md, ARCHITECTURE.md, CLAUDE.md)

## Inspired by pandas

### Data Selection & Indexing
- [x] Select columns by name: `datacell select --columns "name,age,salary"`
- [x] Head/tail: `datacell head --input data.csv -n 10`
- [x] Sample random rows: `datacell sample --input data.csv -n 100`

### Data Aggregation (GroupBy)
- [x] Group by column with aggregation: `datacell groupby --input sales.csv --by category --agg "sum:amount,count:id,avg:price"`
- [x] Pivot tables: `datacell pivot --input data.csv --index region --columns product --values sales --agg sum`

### Data Joining
- [x] Join/merge two files: `datacell join --left a.csv --right b.csv --on id --how inner`
- [x] Concat/stack files vertically: `datacell concat --inputs "a.csv,b.csv,c.csv" --output combined.csv`

### Data Transformation
- [x] Add computed column: `datacell mutate --input data.csv --output out.csv --column total --formula "Price * Quantity"`
- [x] Rename columns: `datacell rename --input data.csv --from "old_name" --to "new_name"`
- [x] Drop columns: `datacell drop --input data.csv --columns "temp,unused"`
- [x] Fill missing values: `datacell fillna --input data.csv --value 0`
- [x] Drop rows with missing values: `datacell dropna --input data.csv`

### Data Statistics
- [x] Describe/summary stats: `datacell describe --input data.csv` (count, mean, std, min, max, quartiles)
- [x] Value counts: `datacell value-counts --input data.csv --column category`
- [x] Correlation matrix: `datacell corr --input data.csv --columns "Price,Quantity"`

### Data Types
- [x] Infer and display column types: `datacell dtypes --input data.csv`
- [x] Cast column types: `datacell astype --input data.csv --column age -t int`
- [x] Get unique values: `datacell unique --input data.csv --column category`
- [x] Dataset info: `datacell info --input data.csv`

### Data Preprocessing
- [x] Clip values to range: `datacell clip --input data.csv --column price --min 0 --max 1000`
- [x] Normalize column (0-1): `datacell normalize --input data.csv --column price`

### Query Language
- [x] SQL-like query: `datacell query --input data.csv -w "Price > 100"`
- [x] Expression evaluation (via mutate): `datacell mutate --input data.csv --column profit --formula "revenue - cost"`

## Code Quality Improvements (Trait-Facing Architecture)
- [x] Create `FileHandler` trait for unified file operations (read, write, read_range)
- [x] Create `DataReader` and `DataWriter` traits for better testability
- [x] Create `FormatDetector` trait for format detection and conversion
- [x] Refactor `DataOperations` to use trait-based approach
- [x] Create `SchemaProvider` trait for schema/metadata operations
- [x] Create `StreamingReader` and `StreamingWriter` traits
- [x] Add mock implementations for all traits for testing
- [x] Refactor `Converter` to use trait-based handlers
- [x] Create `CellRangeProvider` trait for range operations
- [x] Improve error handling with trait-based error types

## Future Enhancements
- [x] Add sparklines support (using Excel formulas as workaround)
- [x] Add conditional formatting (placeholder - requires workbook reading support)
- [x] Data validation rules: `datacell validate --input data.csv --rules rules.json`
- [x] Data profiling: `datacell profile --input data.csv --output profile.json`
- [x] Time series operations: `datacell resample --input data.csv --interval daily --agg sum`
- [x] Geospatial operations: `datacell geo-distance --from lat1,lon1 --to lat2,lon2`
- [x] Text analysis: `datacell text-stats --input data.csv --column content`
- [x] Anomaly detection: `datacell detect-anomalies --input data.csv --column value`
- [x] Data encryption: `datacell encrypt --input data.csv --output encrypted.csv --key keyfile`
- [x] Workflow orchestration: `datacell pipeline --config pipeline.toml`
- [x] Plugin system for custom functions
- [x] REST API server mode (placeholder - requires HTTP framework)
- [x] Real-time data streaming support
- [x] Data lineage tracking
- [x] Automated data quality reports

## Recently Completed
- [x] Chart/visualization: `datacell chart --input data.csv --output chart.xlsx -t column --title "Sales"`
- [x] Config file support: `datacell config-init` creates `.datacell.toml`
- [x] Cell styling for Excel: `datacell export-styled --input data.csv --output styled.xlsx`
- [x] Improved error types with file/row/column context
- [x] Date/time parsing: `datacell parse-date --input data.csv --column Date --from-format "%Y-%m-%d" --to-format "%d/%m/%Y"`
- [x] Regex filter: `datacell regex-filter --input data.csv --column Name --pattern "^[A-M]"`
- [x] Regex replace: `datacell regex-replace --input data.csv --column Category --pattern "Old" --replacement "New"`
- [x] Batch processing: `datacell batch --inputs "*.csv" --output-dir processed/ --operation sort --args '{"column":"Price"}'`
- [x] Shell completions: `datacell completions zsh >> ~/.zshrc`

## Completed
- [x] Rename project from cell-rs to datacell
- [x] Rename CellMcpServer to DatacellMcpServer
- [x] Fix rmcp 0.10 API compatibility
- [x] Fix dead code warnings
- [x] Basic CSV read/write
- [x] Basic Excel read/write
- [x] CSV to Excel conversion
- [x] Excel to CSV conversion
- [x] Formula evaluation (SUM, AVERAGE, MIN, MAX, COUNT, arithmetic)
- [x] IF formula function with comparison operators
- [x] CONCAT formula function for string concatenation
- [x] Cell range reading (e.g., A1:C10)
- [x] JSON output format for CLI
- [x] Unit tests (19 tests passing)
- [x] MCP server implementation
- [x] CLI with clap
- [x] ROUND, ABS, LEN formula functions
- [x] Sort rows by column (ascending/descending)
- [x] Filter rows by condition
- [x] Find and replace values
- [x] Deduplicate rows
- [x] Transpose data (rows to columns)
- [x] Markdown table output format
- [x] Insert/delete rows and columns (API)
- [x] VLOOKUP formula function
- [x] SUMIF, COUNTIF formula functions
- [x] Append data to existing file
- [x] Merge cells for Excel output
- [x] Write specific cell ranges
- [x] Read multiple sheets at once
- [x] ODS (OpenDocument) format support
- [x] Streaming support for large files
- [x] Progress callback API
- [x] List sheets command
- [x] Read all sheets command
- [x] Parquet file support (read/write)
- [x] Avro file support (read/write)
- [x] Unit tests (105 tests passing)
