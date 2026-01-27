//! Parquet and Avro file handling

use anyhow::{Context, Result};
use std::fs::File;
use std::sync::Arc;

use arrow::array::{ArrayRef, BooleanArray, Float64Array, Int64Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

use apache_avro::{
    types::Value as AvroValue, Reader as AvroReader, Schema as AvroSchema, Writer as AvroWriter,
};

use crate::csv_handler::CellRange;
use crate::helpers::{default_column_names, filter_by_range, max_column_count};
use crate::traits::{DataReader, DataWriteOptions, DataWriter, FileHandler, SchemaProvider};

/// Handler for Parquet files
#[derive(Default)]
pub struct ParquetHandler;

impl ParquetHandler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read Parquet file into `Vec<Vec<String>>`
    pub fn read(&self, path: &str) -> Result<Vec<Vec<String>>> {
        let file =
            File::open(path).with_context(|| format!("Failed to open Parquet file: {path}"))?;

        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let reader = builder.build()?;

        let mut all_rows: Vec<Vec<String>> = Vec::new();

        for batch_result in reader {
            let batch = batch_result?;
            let num_rows = batch.num_rows();
            let num_cols = batch.num_columns();

            for row_idx in 0..num_rows {
                let mut row: Vec<String> = Vec::with_capacity(num_cols);
                for col_idx in 0..num_cols {
                    let col = batch.column(col_idx);
                    let value = self.array_value_to_string(col, row_idx);
                    row.push(value);
                }
                all_rows.push(row);
            }
        }

        Ok(all_rows)
    }

    /// Read Parquet file with column names as first row
    pub fn read_with_headers(&self, path: &str) -> Result<Vec<Vec<String>>> {
        let file =
            File::open(path).with_context(|| format!("Failed to open Parquet file: {path}"))?;

        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let schema = builder.schema().clone();
        let reader = builder.build()?;

        let mut all_rows: Vec<Vec<String>> = Vec::new();

        // Add header row
        let headers: Vec<String> = schema.fields().iter().map(|f| f.name().clone()).collect();
        all_rows.push(headers);

        for batch_result in reader {
            let batch = batch_result?;
            let num_rows = batch.num_rows();
            let num_cols = batch.num_columns();

            for row_idx in 0..num_rows {
                let mut row: Vec<String> = Vec::with_capacity(num_cols);
                for col_idx in 0..num_cols {
                    let col = batch.column(col_idx);
                    let value = self.array_value_to_string(col, row_idx);
                    row.push(value);
                }
                all_rows.push(row);
            }
        }

        Ok(all_rows)
    }

    /// Write data to Parquet file (all columns as strings)
    pub fn write(
        &self,
        path: &str,
        data: &[Vec<String>],
        column_names: Option<&[String]>,
    ) -> Result<()> {
        if data.is_empty() {
            anyhow::bail!("Cannot write empty data to Parquet");
        }

        let num_cols = max_column_count(data);

        // Generate column names if not provided
        let col_names: Vec<String> = column_names
            .map(|names| names.to_vec())
            .unwrap_or_else(|| default_column_names(num_cols, "col"));

        // Create schema with string columns
        let fields: Vec<Field> = col_names
            .iter()
            .map(|name| Field::new(name, DataType::Utf8, true))
            .collect();
        let schema = Arc::new(Schema::new(fields));

        // Build column arrays
        let mut columns: Vec<ArrayRef> = Vec::with_capacity(num_cols);
        for col_idx in 0..num_cols {
            let values: Vec<Option<&str>> = data
                .iter()
                .map(|row| row.get(col_idx).map(|s| s.as_str()))
                .collect();
            let array = StringArray::from(values);
            columns.push(Arc::new(array));
        }

        let batch = RecordBatch::try_new(schema.clone(), columns)?;

        let file =
            File::create(path).with_context(|| format!("Failed to create Parquet file: {path}"))?;

        let props = WriterProperties::builder().build();
        let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;
        writer.write(&batch)?;
        writer.close()?;

        Ok(())
    }

    /// Get schema information from Parquet file
    pub fn get_schema(&self, path: &str) -> Result<Vec<(String, String)>> {
        let file =
            File::open(path).with_context(|| format!("Failed to open Parquet file: {path}"))?;

        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let schema = builder.schema();

        let fields: Vec<(String, String)> = schema
            .fields()
            .iter()
            .map(|f| (f.name().clone(), format!("{:?}", f.data_type())))
            .collect();

        Ok(fields)
    }

    fn array_value_to_string(&self, array: &ArrayRef, idx: usize) -> String {
        if array.is_null(idx) {
            return String::new();
        }

        match array.data_type() {
            DataType::Utf8 => array
                .as_any()
                .downcast_ref::<StringArray>()
                .map(|arr| arr.value(idx).to_string())
                .unwrap_or_else(|| format!("{:?}", array)),
            DataType::LargeUtf8 => array
                .as_any()
                .downcast_ref::<arrow::array::LargeStringArray>()
                .map(|arr| arr.value(idx).to_string())
                .unwrap_or_else(|| format!("{:?}", array)),
            DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 => {
                array
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .map(|a| a.value(idx).to_string())
                    .unwrap_or_else(|| format!("{:?}", array))
            }
            DataType::Float32 | DataType::Float64 => array
                .as_any()
                .downcast_ref::<Float64Array>()
                .map(|a| a.value(idx).to_string())
                .unwrap_or_else(|| format!("{:?}", array)),
            DataType::Boolean => array
                .as_any()
                .downcast_ref::<BooleanArray>()
                .map(|arr| arr.value(idx).to_string())
                .unwrap_or_else(|| format!("{:?}", array)),
            _ => format!("{:?}", array.data_type()),
        }
    }
}

/// Handler for Avro files
#[derive(Default)]
pub struct AvroHandler;

impl AvroHandler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read Avro file into `Vec<Vec<String>>`
    pub fn read(&self, path: &str) -> Result<Vec<Vec<String>>> {
        let file = File::open(path).with_context(|| format!("Failed to open Avro file: {path}"))?;

        let reader = AvroReader::new(file)?;
        let mut all_rows: Vec<Vec<String>> = Vec::new();

        for value in reader {
            let value = value?;
            if let AvroValue::Record(fields) = value {
                let row: Vec<String> = fields
                    .iter()
                    .map(|(_, v)| self.avro_value_to_string(v))
                    .collect();
                all_rows.push(row);
            }
        }

        Ok(all_rows)
    }

    /// Read Avro file with field names as first row
    pub fn read_with_headers(&self, path: &str) -> Result<Vec<Vec<String>>> {
        let file = File::open(path).with_context(|| format!("Failed to open Avro file: {path}"))?;

        let reader = AvroReader::new(file)?;
        let mut all_rows: Vec<Vec<String>> = Vec::new();

        // Get field names from schema
        if let AvroSchema::Record(record) = reader.writer_schema() {
            let headers: Vec<String> = record.fields.iter().map(|f| f.name.clone()).collect();
            all_rows.push(headers);
        }

        for value in reader {
            let value = value?;
            if let AvroValue::Record(fields) = value {
                let row: Vec<String> = fields
                    .iter()
                    .map(|(_, v)| self.avro_value_to_string(v))
                    .collect();
                all_rows.push(row);
            }
        }

        Ok(all_rows)
    }

    /// Write data to Avro file (all fields as strings)
    pub fn write(
        &self,
        path: &str,
        data: &[Vec<String>],
        field_names: Option<&[String]>,
    ) -> Result<()> {
        if data.is_empty() {
            anyhow::bail!("Cannot write empty data to Avro");
        }

        let num_cols = max_column_count(data);

        // Generate field names if not provided
        let names: Vec<String> = field_names
            .map(|n| n.to_vec())
            .unwrap_or_else(|| default_column_names(num_cols, "field"));

        // Build Avro schema
        let schema_json = format!(
            r#"{{
                "type": "record",
                "name": "Row",
                "fields": [{}]
            }}"#,
            names
                .iter()
                .map(|n| format!(r#"{{"name": "{}", "type": ["null", "string"]}}"#, n))
                .collect::<Vec<_>>()
                .join(", ")
        );

        let schema = AvroSchema::parse_str(&schema_json)?;

        let file =
            File::create(path).with_context(|| format!("Failed to create Avro file: {path}"))?;

        {
            let mut writer = AvroWriter::new(&schema, file);

            for row in data {
                let mut record: Vec<(String, AvroValue)> = Vec::new();
                for (i, name) in names.iter().enumerate() {
                    let value = row
                        .get(i)
                        .map(|s| AvroValue::Union(1, Box::new(AvroValue::String(s.clone()))))
                        .unwrap_or(AvroValue::Union(0, Box::new(AvroValue::Null)));
                    record.push((name.clone(), value));
                }
                writer.append(AvroValue::Record(record))?;
            }

            // Flush and finalize the writer - this is critical for Avro format
            writer.flush()?;
            // Writer is dropped here, which finalizes the Avro file
        }

        Ok(())
    }

    /// Get schema information from Avro file
    pub fn get_schema(&self, path: &str) -> Result<Vec<(String, String)>> {
        let file = File::open(path).with_context(|| format!("Failed to open Avro file: {path}"))?;

        let reader = AvroReader::new(file)?;

        let fields = if let AvroSchema::Record(record) = reader.writer_schema() {
            record
                .fields
                .iter()
                .map(|f| (f.name.clone(), format!("{:?}", f.schema)))
                .collect()
        } else {
            Vec::new()
        };

        Ok(fields)
    }

    fn avro_value_to_string(&self, value: &AvroValue) -> String {
        match value {
            AvroValue::Null => String::new(),
            AvroValue::Boolean(b) => b.to_string(),
            AvroValue::Int(i) => i.to_string(),
            AvroValue::Long(l) => l.to_string(),
            AvroValue::Float(f) => f.to_string(),
            AvroValue::Double(d) => d.to_string(),
            AvroValue::String(s) => s.clone(),
            AvroValue::Bytes(b) => String::from_utf8_lossy(b).to_string(),
            AvroValue::Union(_, inner) => self.avro_value_to_string(inner),
            AvroValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.avro_value_to_string(v)).collect();
                format!("[{}]", items.join(", "))
            }
            AvroValue::Map(map) => {
                let items: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.avro_value_to_string(v)))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            _ => format!("{:?}", value),
        }
    }
}

// Trait implementations for ParquetHandler

impl DataReader for ParquetHandler {
    fn read(&self, path: &str) -> Result<Vec<Vec<String>>> {
        self.read(path)
    }

    fn read_with_headers(&self, path: &str) -> Result<Vec<Vec<String>>> {
        self.read_with_headers(path)
    }

    fn read_range(&self, path: &str, range: &CellRange) -> Result<Vec<Vec<String>>> {
        let all_data = self.read(path)?;
        Ok(filter_by_range(&all_data, range))
    }

    fn read_as_json(&self, path: &str) -> Result<String> {
        let data = self.read(path)?;
        serde_json::to_string_pretty(&data).with_context(|| "Failed to serialize to JSON")
    }

    fn supports_format(&self, path: &str) -> bool {
        path.to_lowercase().ends_with(".parquet")
    }
}

impl DataWriter for ParquetHandler {
    fn write(&self, path: &str, data: &[Vec<String>], options: DataWriteOptions) -> Result<()> {
        self.write(path, data, options.column_names.as_deref())
    }

    fn write_range(
        &self,
        path: &str,
        data: &[Vec<String>],
        _start_row: usize,
        _start_col: usize,
    ) -> Result<()> {
        // For Parquet, we write the entire dataset
        self.write(path, data, None)
    }

    fn append(&self, _path: &str, _data: &[Vec<String>]) -> Result<()> {
        anyhow::bail!("Append operation not supported for Parquet files")
    }

    fn supports_format(&self, path: &str) -> bool {
        path.to_lowercase().ends_with(".parquet")
    }
}

impl FileHandler for ParquetHandler {
    fn format_name(&self) -> &'static str {
        "parquet"
    }

    fn supported_extensions(&self) -> &'static [&'static str] {
        &["parquet"]
    }
}

impl SchemaProvider for ParquetHandler {
    fn get_schema(&self, path: &str) -> Result<Vec<(String, String)>> {
        self.get_schema(path)
    }

    fn get_column_names(&self, path: &str) -> Result<Vec<String>> {
        let schema = self.get_schema(path)?;
        Ok(schema.into_iter().map(|(name, _)| name).collect())
    }

    fn get_row_count(&self, path: &str) -> Result<usize> {
        let data = self.read(path)?;
        Ok(data.len())
    }

    fn get_column_count(&self, path: &str) -> Result<usize> {
        let data = self.read(path)?;
        Ok(data.first().map(|r| r.len()).unwrap_or(0))
    }
}

// Trait implementations for AvroHandler

impl DataReader for AvroHandler {
    fn read(&self, path: &str) -> Result<Vec<Vec<String>>> {
        self.read(path)
    }

    fn read_with_headers(&self, path: &str) -> Result<Vec<Vec<String>>> {
        self.read_with_headers(path)
    }

    fn read_range(&self, path: &str, range: &CellRange) -> Result<Vec<Vec<String>>> {
        let all_data = self.read(path)?;
        Ok(filter_by_range(&all_data, range))
    }

    fn read_as_json(&self, path: &str) -> Result<String> {
        let data = self.read(path)?;
        serde_json::to_string_pretty(&data).with_context(|| "Failed to serialize to JSON")
    }

    fn supports_format(&self, path: &str) -> bool {
        path.to_lowercase().ends_with(".avro")
    }
}

impl DataWriter for AvroHandler {
    fn write(&self, path: &str, data: &[Vec<String>], options: DataWriteOptions) -> Result<()> {
        self.write(path, data, options.column_names.as_deref())
    }

    fn write_range(
        &self,
        path: &str,
        data: &[Vec<String>],
        _start_row: usize,
        _start_col: usize,
    ) -> Result<()> {
        // For Avro, we write the entire dataset
        self.write(path, data, None)
    }

    fn append(&self, _path: &str, _data: &[Vec<String>]) -> Result<()> {
        anyhow::bail!("Append operation not supported for Avro files")
    }

    fn supports_format(&self, path: &str) -> bool {
        path.to_lowercase().ends_with(".avro")
    }
}

impl FileHandler for AvroHandler {
    fn format_name(&self) -> &'static str {
        "avro"
    }

    fn supported_extensions(&self) -> &'static [&'static str] {
        &["avro"]
    }
}

impl SchemaProvider for AvroHandler {
    fn get_schema(&self, path: &str) -> Result<Vec<(String, String)>> {
        self.get_schema(path)
    }

    fn get_column_names(&self, path: &str) -> Result<Vec<String>> {
        let schema = self.get_schema(path)?;
        Ok(schema.into_iter().map(|(name, _)| name).collect())
    }

    fn get_row_count(&self, path: &str) -> Result<usize> {
        let data = self.read(path)?;
        Ok(data.len())
    }

    fn get_column_count(&self, path: &str) -> Result<usize> {
        let data = self.read(path)?;
        Ok(data.first().map(|r| r.len()).unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parquet_write_read() {
        let handler = ParquetHandler::new();
        let data = vec![
            vec!["a".to_string(), "1".to_string()],
            vec!["b".to_string(), "2".to_string()],
        ];

        let path = "/tmp/test_datacell.parquet";
        handler
            .write(
                path,
                &data,
                Some(&["name".to_string(), "value".to_string()]),
            )
            .unwrap();

        let read_data = handler.read(path).unwrap();
        assert_eq!(read_data.len(), 2);
        assert_eq!(read_data[0][0], "a");

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_avro_write_read() {
        let handler = AvroHandler::new();
        let data = vec![
            vec!["x".to_string(), "10".to_string()],
            vec!["y".to_string(), "20".to_string()],
        ];

        let path = "/tmp/test_datacell.avro";
        handler
            .write(
                path,
                &data,
                Some(&["name".to_string(), "value".to_string()]),
            )
            .unwrap();

        let read_data = handler.read(path).unwrap();
        assert_eq!(read_data.len(), 2);
        assert_eq!(read_data[0][0], "x");

        fs::remove_file(path).ok();
    }
}
