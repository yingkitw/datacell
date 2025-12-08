use anyhow::{Context, Result};
use csv::{ReaderBuilder, WriterBuilder};
use std::fs::File;
use std::io::Read;

/// Represents a cell range like A1:B3
#[derive(Debug, Clone)]
pub struct CellRange {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

impl CellRange {
    /// Parse a range string like "A1:B3" or "A1"
    pub fn parse(range_str: &str) -> Result<Self> {
        let range_str = range_str.trim().to_uppercase();
        
        if let Some(colon_pos) = range_str.find(':') {
            let start = &range_str[..colon_pos];
            let end = &range_str[colon_pos + 1..];
            let (start_row, start_col) = Self::parse_cell(start)?;
            let (end_row, end_col) = Self::parse_cell(end)?;
            Ok(Self { start_row, start_col, end_row, end_col })
        } else {
            let (row, col) = Self::parse_cell(&range_str)?;
            Ok(Self { start_row: row, start_col: col, end_row: row, end_col: col })
        }
    }
    
    fn parse_cell(cell: &str) -> Result<(usize, usize)> {
        let mut col_str = String::new();
        let mut row_str = String::new();
        
        for ch in cell.chars() {
            if ch.is_alphabetic() {
                col_str.push(ch);
            } else if ch.is_ascii_digit() {
                row_str.push(ch);
            }
        }
        
        let col = Self::column_to_index(&col_str)?;
        let row = row_str.parse::<usize>()
            .with_context(|| format!("Invalid row in cell: {}", cell))?;
        
        Ok((row.saturating_sub(1), col)) // Convert to 0-indexed
    }
    
    fn column_to_index(col: &str) -> Result<usize> {
        if col.is_empty() {
            anyhow::bail!("Empty column reference");
        }
        let mut index = 0usize;
        for ch in col.chars() {
            index = index * 26 + (ch.to_ascii_uppercase() as usize - b'A' as usize + 1);
        }
        Ok(index - 1)
    }
}

pub struct CsvHandler;

impl CsvHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn read(&self, path: &str) -> Result<String> {
        let mut file = File::open(path)
            .with_context(|| format!("Failed to open CSV file: {}", path))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        Ok(contents)
    }

    pub fn write_from_csv(&self, input_path: &str, output_path: &str) -> Result<()> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(input_path)
            .with_context(|| format!("Failed to open CSV file: {}", input_path))?;

        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_path(output_path)
            .with_context(|| format!("Failed to create CSV file: {}", output_path))?;

        for result in reader.records() {
            let record = result?;
            writer.write_record(&record)?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn write_records(&self, path: &str, records: Vec<Vec<String>>) -> Result<()> {
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_path(path)
            .with_context(|| format!("Failed to create CSV file: {}", path))?;

        for record in records {
            writer.write_record(&record)?;
        }

        writer.flush()?;
        Ok(())
    }
    
    /// Read a specific range from CSV file
    pub fn read_range(&self, path: &str, range: &CellRange) -> Result<Vec<Vec<String>>> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(path)
            .with_context(|| format!("Failed to open CSV file: {}", path))?;
        
        let mut result = Vec::new();
        
        for (row_idx, record) in reader.records().enumerate() {
            if row_idx < range.start_row {
                continue;
            }
            if row_idx > range.end_row {
                break;
            }
            
            let record = record?;
            let row: Vec<String> = record.iter()
                .enumerate()
                .filter(|(col_idx, _)| *col_idx >= range.start_col && *col_idx <= range.end_col)
                .map(|(_, val)| val.to_string())
                .collect();
            result.push(row);
        }
        
        Ok(result)
    }
    
    /// Read CSV and return as JSON array
    pub fn read_as_json(&self, path: &str) -> Result<String> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(path)
            .with_context(|| format!("Failed to open CSV file: {}", path))?;
        
        let mut rows: Vec<Vec<String>> = Vec::new();
        for record in reader.records() {
            let record = record?;
            rows.push(record.iter().map(|s| s.to_string()).collect());
        }
        
        serde_json::to_string_pretty(&rows)
            .with_context(|| "Failed to serialize to JSON")
    }
}

