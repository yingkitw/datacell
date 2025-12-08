# datacell TODO

## Future Enhancements
- [ ] Improve error messages with line/column context
- [ ] Add more chart/visualization support
- [ ] Add cell styling (colors, fonts, borders)

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
