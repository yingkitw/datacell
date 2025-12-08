# datacell TODO

## High Priority
- [ ] Add VLOOKUP formula function
- [ ] Merge cells (for Excel output)
- [ ] Append data to existing file

## Medium Priority
- [ ] Write specific cell ranges
- [ ] Read multiple sheets at once
- [ ] Add more formula functions (SUMIF, COUNTIF)

## Low Priority
- [ ] Support .ods (OpenDocument) format
- [ ] Add streaming support for large files
- [ ] Add progress indicator for large file operations
- [ ] Improve error messages with line/column context

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
