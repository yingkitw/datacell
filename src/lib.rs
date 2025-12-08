//! datacell - A library for reading, writing, and converting spreadsheet files
//! 
//! Supports CSV, Excel (xlsx/xls), ODS, Parquet, and Avro formats with formula evaluation.

#![allow(dead_code)] // Library exports many public APIs not used internally

pub mod excel;
pub mod csv_handler;
pub mod converter;
pub mod formula;
pub mod mcp;
pub mod operations;
pub mod columnar;

pub use excel::ExcelHandler;
pub use csv_handler::{CsvHandler, CellRange, StreamingCsvReader, StreamingCsvWriter};
pub use converter::Converter;
pub use formula::{FormulaEvaluator, FormulaResult};
pub use mcp::DatacellMcpServer;
pub use operations::{DataOperations, SortOrder, ProgressCallback, StderrProgress, NoProgress, JoinType, AggFunc};
pub use columnar::{ParquetHandler, AvroHandler};

