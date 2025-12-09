use anyhow::{Context, Result};
use rust_xlsxwriter::Workbook;
use std::path::Path;

use crate::csv_handler::CellRange;

use super::types::WriteOptions;
use super::reader::ExcelHandler;

impl ExcelHandler {
    pub fn write_from_csv(&self, csv_path: &str, excel_path: &str, sheet_name: Option<&str>) -> Result<()> {
        let mut workbook = Workbook::new();
        let sheet_name = sheet_name.unwrap_or("Sheet1");
        let worksheet = workbook.add_worksheet().set_name(sheet_name)?;

        let mut reader = csv::Reader::from_path(csv_path)
            .with_context(|| format!("Failed to open CSV file: {}", csv_path))?;

        let mut row_num = 0u32;
        for result in reader.records() {
            let record = result?;
            for (col_num, field) in record.iter().enumerate() {
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
            Workbook::new()
        } else {
            Workbook::new()
        };

        let sheet_name = sheet_name.unwrap_or("Sheet1");
        let worksheet = workbook.add_worksheet().set_name(sheet_name)?;

        let (row, col) = self.parse_cell_reference(cell)?;
        worksheet.write_formula(row, col, formula)?;

        workbook.save(excel_path)
            .with_context(|| format!("Failed to save Excel file: {}", excel_path))?;

        Ok(())
    }

    pub fn write_with_merge(
        &self,
        excel_path: &str,
        data: &[Vec<String>],
        merges: &[(CellRange, CellRange)],
        sheet_name: Option<&str>,
    ) -> Result<()> {
        let mut workbook = Workbook::new();
        let sheet_name = sheet_name.unwrap_or("Sheet1");
        let worksheet = workbook.add_worksheet().set_name(sheet_name)?;
        
        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                if let Ok(num) = value.parse::<f64>() {
                    worksheet.write_number(row_idx as u32, col_idx as u16, num)?;
                } else {
                    worksheet.write_string(row_idx as u32, col_idx as u16, value)?;
                }
            }
        }
        
        let merge_format = rust_xlsxwriter::Format::new();
        for (start, end) in merges {
            worksheet.merge_range(
                start.start_row as u32,
                start.start_col as u16,
                end.end_row as u32,
                end.end_col as u16,
                "",
                &merge_format,
            )?;
        }
        
        workbook.save(excel_path)
            .with_context(|| format!("Failed to save Excel file: {}", excel_path))?;
        
        Ok(())
    }
    
    pub fn write_range(
        &self,
        excel_path: &str,
        data: &[Vec<String>],
        start_row: u32,
        start_col: u16,
        sheet_name: Option<&str>,
    ) -> Result<()> {
        let mut workbook = Workbook::new();
        let sheet_name = sheet_name.unwrap_or("Sheet1");
        let worksheet = workbook.add_worksheet().set_name(sheet_name)?;
        
        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                let target_row = start_row + row_idx as u32;
                let target_col = start_col + col_idx as u16;
                
                if let Ok(num) = value.parse::<f64>() {
                    worksheet.write_number(target_row, target_col, num)?;
                } else {
                    worksheet.write_string(target_row, target_col, value)?;
                }
            }
        }
        
        workbook.save(excel_path)
            .with_context(|| format!("Failed to save Excel file: {}", excel_path))?;
        
        Ok(())
    }
    
    /// Write data to Excel with styling options
    pub fn write_styled(
        &self,
        path: &str,
        data: &[Vec<String>],
        options: &WriteOptions,
    ) -> Result<()> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        
        if let Some(ref name) = options.sheet_name {
            worksheet.set_name(name)?;
        }
        
        for (row_idx, row) in data.iter().enumerate() {
            let is_header = row_idx == 0 && options.style_header;
            
            for (col_idx, cell) in row.iter().enumerate() {
                let format = if is_header {
                    Some(options.header_style.to_format())
                } else if let Some(ref col_styles) = options.column_styles {
                    col_styles.get(&col_idx).map(|s| s.to_format())
                } else {
                    None
                };
                
                if let Ok(num) = cell.parse::<f64>() {
                    if let Some(fmt) = format {
                        worksheet.write_number_with_format(row_idx as u32, col_idx as u16, num, &fmt)?;
                    } else {
                        worksheet.write_number(row_idx as u32, col_idx as u16, num)?;
                    }
                } else {
                    if let Some(fmt) = format {
                        worksheet.write_string_with_format(row_idx as u32, col_idx as u16, cell, &fmt)?;
                    } else {
                        worksheet.write_string(row_idx as u32, col_idx as u16, cell)?;
                    }
                }
            }
        }
        
        if options.freeze_header && !data.is_empty() {
            worksheet.set_freeze_panes(1, 0)?;
        }
        
        if options.auto_filter && !data.is_empty() {
            let last_col = data[0].len().saturating_sub(1) as u16;
            let last_row = data.len().saturating_sub(1) as u32;
            worksheet.autofilter(0, 0, last_row, last_col)?;
        }
        
        if options.auto_fit {
            for col_idx in 0..data.get(0).map(|r| r.len()).unwrap_or(0) {
                let max_width = data.iter()
                    .map(|row| row.get(col_idx).map(|s| s.len()).unwrap_or(0))
                    .max()
                    .unwrap_or(10);
                worksheet.set_column_width(col_idx as u16, (max_width + 2) as f64)?;
            }
        }
        
        workbook.save(path)?;
        Ok(())
    }
}
