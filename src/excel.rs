use anyhow::{Context, Result};
use calamine::{open_workbook, Reader, Xlsx};
use rust_xlsxwriter::Workbook;
use std::path::Path;
use crate::csv_handler::CellRange;

pub struct ExcelHandler;

impl ExcelHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn read(&self, path: &str) -> Result<String> {
        self.read_with_sheet(path, None)
    }

    pub fn read_with_sheet(&self, path: &str, sheet_name: Option<&str>) -> Result<String> {
        let mut workbook: Xlsx<_> = open_workbook(path)
            .with_context(|| format!("Failed to open Excel file: {}", path))?;

        let sheet_names = workbook.sheet_names();
        let sheet_name = sheet_name
            .or_else(|| sheet_names.first().map(|s| s.as_str()))
            .ok_or_else(|| anyhow::anyhow!("No sheets found in workbook"))?;

        let range = workbook
            .worksheet_range(sheet_name)
            .with_context(|| format!("Failed to read sheet: {}", sheet_name))?;

        let mut output = String::new();
        for row in range.rows() {
            let row_str: Vec<String> = row
                .iter()
                .map(|cell| {
                    cell.to_string()
                })
                .collect();
            output.push_str(&row_str.join(","));
            output.push('\n');
        }

        Ok(output)
    }

    pub fn write_from_csv(&self, csv_path: &str, excel_path: &str, sheet_name: Option<&str>) -> Result<()> {
        let mut workbook = Workbook::new();
        let sheet_name = sheet_name.unwrap_or("Sheet1");
        let worksheet = workbook.add_worksheet().set_name(sheet_name)?;

        // Read CSV and write to Excel
        let mut reader = csv::Reader::from_path(csv_path)
            .with_context(|| format!("Failed to open CSV file: {}", csv_path))?;

        let mut row_num = 0u32;
        for result in reader.records() {
            let record = result?;
            for (col_num, field) in record.iter().enumerate() {
                // Try to parse as number, otherwise write as string
                if let Ok(num) = field.parse::<f64>() {
                    worksheet.write_number(row_num, col_num as u16, num)?;
                } else {
                    worksheet.write_string(row_num, col_num as u16, field)?;
                }
            }
            row_num += 1;
        }

        workbook.save(excel_path)
            .with_context(|| format!("Failed to save Excel file: {}", excel_path))?;

        Ok(())
    }

    pub fn write_with_formula(&self, excel_path: &str, formula: &str, cell: &str, sheet_name: Option<&str>) -> Result<()> {
        let mut workbook = if Path::new(excel_path).exists() {
            // For existing files, we'd need to read and modify
            // For simplicity, creating new workbook
            Workbook::new()
        } else {
            Workbook::new()
        };

        let sheet_name = sheet_name.unwrap_or("Sheet1");
        let worksheet = workbook.add_worksheet().set_name(sheet_name)?;

        // Parse cell reference (e.g., "C1" -> row 0, col 2)
        let (row, col) = self.parse_cell_reference(cell)?;

        // Write formula
        worksheet.write_formula(row, col, formula)?;

        workbook.save(excel_path)
            .with_context(|| format!("Failed to save Excel file: {}", excel_path))?;

        Ok(())
    }

    pub fn parse_cell_reference(&self, cell: &str) -> Result<(u32, u16)> {
        let mut col_str = String::new();
        let mut row_str = String::new();

        for ch in cell.chars() {
            if ch.is_alphabetic() {
                col_str.push(ch);
            } else if ch.is_ascii_digit() {
                row_str.push(ch);
            }
        }

        let col = self.column_to_index(&col_str)?;
        let row = row_str.parse::<u32>()
            .with_context(|| format!("Invalid row number in cell reference: {}", cell))?;
        
        // Excel rows are 1-indexed, but rust_xlsxwriter uses 0-indexed
        Ok((row - 1, col))
    }

    fn column_to_index(&self, col: &str) -> Result<u16> {
        let mut index = 0u16;
        for ch in col.chars() {
            index = index * 26 + (ch.to_ascii_uppercase() as u16 - b'A' as u16 + 1);
        }
        Ok(index - 1)
    }
    
    /// Read a specific range from Excel file
    pub fn read_range(&self, path: &str, range: &CellRange, sheet_name: Option<&str>) -> Result<Vec<Vec<String>>> {
        let mut workbook: Xlsx<_> = open_workbook(path)
            .with_context(|| format!("Failed to open Excel file: {}", path))?;

        let sheet_names = workbook.sheet_names();
        let sheet_name = sheet_name
            .or_else(|| sheet_names.first().map(|s| s.as_str()))
            .ok_or_else(|| anyhow::anyhow!("No sheets found in workbook"))?;

        let ws_range = workbook
            .worksheet_range(sheet_name)
            .with_context(|| format!("Failed to read sheet: {}", sheet_name))?;

        let mut result = Vec::new();
        for (row_idx, row) in ws_range.rows().enumerate() {
            if row_idx < range.start_row {
                continue;
            }
            if row_idx > range.end_row {
                break;
            }
            
            let row_data: Vec<String> = row.iter()
                .enumerate()
                .filter(|(col_idx, _)| *col_idx >= range.start_col && *col_idx <= range.end_col)
                .map(|(_, cell)| cell.to_string())
                .collect();
            result.push(row_data);
        }

        Ok(result)
    }
    
    /// Read Excel and return as JSON array
    pub fn read_as_json(&self, path: &str, sheet_name: Option<&str>) -> Result<String> {
        let mut workbook: Xlsx<_> = open_workbook(path)
            .with_context(|| format!("Failed to open Excel file: {}", path))?;

        let sheet_names = workbook.sheet_names();
        let sheet_name = sheet_name
            .or_else(|| sheet_names.first().map(|s| s.as_str()))
            .ok_or_else(|| anyhow::anyhow!("No sheets found in workbook"))?;

        let range = workbook
            .worksheet_range(sheet_name)
            .with_context(|| format!("Failed to read sheet: {}", sheet_name))?;

        let mut rows: Vec<Vec<String>> = Vec::new();
        for row in range.rows() {
            rows.push(row.iter().map(|cell| cell.to_string()).collect());
        }

        serde_json::to_string_pretty(&rows)
            .with_context(|| "Failed to serialize to JSON")
    }
}

