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
pub mod config;
pub mod error;
pub mod traits;
pub mod format_detector;
pub mod helpers;
pub mod handler_registry;
#[cfg(test)]
pub mod mocks;
pub mod common;
pub mod cli;
pub mod validation;
pub mod profiling;
pub mod timeseries;
pub mod text_analysis;
pub mod anomaly;
pub mod geospatial;
pub mod encryption;
pub mod workflow;
pub mod lineage;
pub mod quality;
pub mod error_traits;
pub mod plugins;
pub mod api;
pub mod streaming;

pub use excel::{ExcelHandler, CellStyle, WriteOptions, DataChartType, ChartConfig};
pub use csv_handler::{CsvHandler, CellRange, StreamingCsvReader, StreamingCsvWriter, CellRangeHelper};
pub use converter::Converter;
pub use formula::{FormulaEvaluator, FormulaResult};
pub use mcp::DatacellMcpServer;
pub use operations::{DataOperations, SortOrder, ProgressCallback, StderrProgress, NoProgress, JoinType, AggFunc};
pub use columnar::{ParquetHandler, AvroHandler};
pub use config::Config;
pub use error::{DatacellError, ErrorKind, ErrorContext, ResultExt};
pub use traits::{
    DataReader, DataWriter, FileHandler, FormatDetector, SchemaProvider,
    StreamingReader, StreamingWriter, CellRangeProvider, DataOperator,
    SortOperator, FilterOperator, TransformOperator,
    DataWriteOptions, FilterCondition, TransformOperation,
};
pub use format_detector::DefaultFormatDetector;
pub use handler_registry::HandlerRegistry;
pub use helpers::{filter_by_range, default_column_names, max_column_count, matches_extension};
pub use validation::{DataValidator, ValidationConfig, ValidationRule, ValidationResult};
pub use profiling::{DataProfiler, DataProfile, ColumnProfile};
pub use timeseries::{TimeSeriesProcessor, TimeSeriesPoint, ResampleInterval, TimeSeriesAgg, RollingWindow};
pub use text_analysis::{TextAnalyzer, TextStats, SentimentResult, KeywordResult, LanguageResult};
pub use anomaly::{AnomalyDetector, AnomalyMethod, AnomalyResult, Anomaly};
pub use geospatial::{GeospatialCalculator, Coordinate};
pub use encryption::{DataEncryptor, EncryptionAlgorithm};
pub use workflow::{WorkflowExecutor, WorkflowConfig, WorkflowStep};
pub use lineage::{LineageTracker, LineageNode};
pub use quality::{QualityReportGenerator, QualityReport, QualityIssue, IssueSeverity};
pub use error_traits::{
    ErrorContextProvider, UserFriendlyError, RecoverableError, ErrorCategory,
    TraitBasedError, ErrorCategoryType, ErrorSeverity, ToTraitBasedError,
};
pub use plugins::{PluginRegistry, PluginFunction, PluginMetadata, FunctionMetadata};
pub use api::{ApiServer, ApiConfig, ApiRequest, ApiResponse, CommandHandler};
pub use streaming::{StreamingProcessor, StreamingDataReader, StreamingDataWriter, DataChunk, StreamingChannel};

