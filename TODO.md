# datacell TODO

## Inspired by pandas

### Data Selection & Indexing
- [x] Select columns by name: `datacell select --columns "name,age,salary"`
- [x] Head/tail: `datacell head --input data.csv -n 10`
- [x] Sample random rows: `datacell sample --input data.csv -n 100`

### Data Aggregation (GroupBy)
- [x] Group by column with aggregation: `datacell groupby --input sales.csv --by category --agg "sum:amount,count:id,avg:price"`
- [ ] Pivot tables: `datacell pivot --input data.csv --index region --columns product --values sales --agg sum`

### Data Joining
- [x] Join/merge two files: `datacell join --left a.csv --right b.csv --on id --how inner`
- [x] Concat/stack files vertically: `datacell concat --inputs "a.csv,b.csv,c.csv" --output combined.csv`

### Data Transformation
- [ ] Add computed column: `datacell mutate --input data.csv --output out.csv --column total --formula "C*D"`
- [x] Rename columns: `datacell rename --input data.csv --from "old_name" --to "new_name"`
- [x] Drop columns: `datacell drop --input data.csv --columns "temp,unused"`
- [x] Fill missing values: `datacell fillna --input data.csv --value 0`
- [x] Drop rows with missing values: `datacell dropna --input data.csv`

### Data Statistics
- [x] Describe/summary stats: `datacell describe --input data.csv` (count, mean, std, min, max, quartiles)
- [x] Value counts: `datacell value-counts --input data.csv --column category`
- [ ] Correlation matrix: `datacell corr --input data.csv --columns "a,b,c"`

### Data Types
- [ ] Infer and display column types: `datacell dtypes --input data.csv`
- [ ] Cast column types: `datacell astype --input data.csv --column age --type int`

### Query Language
- [ ] SQL-like query: `datacell query --input data.csv --where "age > 30 AND status = 'active'"`
- [ ] Expression evaluation: `datacell eval --input data.csv --expr "profit = revenue - cost"`

## Future Enhancements
- [ ] Improve error messages with line/column context
- [ ] Add more chart/visualization support
- [ ] Add cell styling (colors, fonts, borders)
- [ ] Date/time parsing and formatting
- [ ] Regular expression support in filter/replace
- [ ] Multi-file operations (batch processing)
- [ ] Config file for default options
- [ ] Shell completions (bash, zsh, fish)

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
- [x] Unit tests (25 tests passing)
