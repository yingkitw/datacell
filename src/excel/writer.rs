use anyhow::{Context, Result};
use rust_xlsxwriter::Workbook;
use std::path::Path;

use crate::csv_handler::CellRange;

use super::reader::ExcelHandler;
use super::types::WriteOptions;

impl ExcelHandler {
    pub fn write_from_csv(
        &self,
        csv_path: &str,
        excel_path: &str,
        sheet_name: Option<&str>,
    ) -> Result<()> {
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

        workbook
            .save(excel_path)
            .with_context(|| format!("Failed to save Excel file: {}", excel_path))?;

        Ok(())
    }

    pub fn write_with_formula(
        &self,
        excel_path: &str,
        formula: &str,
        cell: &str,
        sheet_name: Option<&str>,
    ) -> Result<()> {
        let mut workbook = if Path::new(excel_path).exists() {
            Workbook::new()
        } else {
            Workbook::new()
        };

        let sheet_name = sheet_name.unwrap_or("Sheet1");
        let worksheet = workbook.add_worksheet().set_name(sheet_name)?;

        let (row, col) = self.parse_cell_reference_writer(cell)?;
        let formula_obj = rust_xlsxwriter::Formula::new(formula);
        worksheet.write_formula(row, col, &formula_obj)?;

        workbook
            .save(excel_path)
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

        workbook
            .save(excel_path)
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

        workbook
            .save(excel_path)
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
                        worksheet.write_number_with_format(
                            row_idx as u32,
                            col_idx as u16,
                            num,
                            &fmt,
                        )?;
                    } else {
                        worksheet.write_number(row_idx as u32, col_idx as u16, num)?;
                    }
                } else {
                    if let Some(fmt) = format {
                        worksheet.write_string_with_format(
                            row_idx as u32,
                            col_idx as u16,
                            cell,
                            &fmt,
                        )?;
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
                let max_width = data
                    .iter()
                    .map(|row| row.get(col_idx).map(|s| s.len()).unwrap_or(0))
                    .max()
                    .unwrap_or(10);
                worksheet.set_column_width(col_idx as u16, (max_width + 2) as f64)?;
            }
        }

        workbook.save(path)?;
        Ok(())
    }

    /// Add sparklines using formulas (workaround for rust_xlsxwriter limitation)
    /// Note: This creates a formula that Excel will render as a sparkline
    pub fn add_sparkline_formula(
        &self,
        excel_path: &str,
        data_range: &str,
        sparkline_cell: &str,
        sheet_name: Option<&str>,
    ) -> Result<()> {
        // Create a new workbook or open existing
        let mut workbook = Workbook::new();
        let sheet_name = sheet_name.unwrap_or("Sheet1");
        let worksheet = workbook.add_worksheet().set_name(sheet_name)?;

        // Write sparkline formula
        // Excel sparkline formula: =SPARKLINE(data_range)
        let (row, col) = self.parse_cell_reference_writer(sparkline_cell)?;
        let formula = rust_xlsxwriter::Formula::new(format!("=SPARKLINE({})", data_range));
        worksheet.write_formula(row, col, &formula)?;

        workbook
            .save(excel_path)
            .with_context(|| format!("Failed to save Excel file: {}", excel_path))?;

        Ok(())
    }

    /// Apply conditional formatting using formulas (workaround)
    /// Note: This uses Excel formulas for conditional formatting
    pub fn apply_conditional_format_formula(
        &self,
        _excel_path: &str,
        _range: &str,
        _condition: &str,
        _true_format: &super::types::CellStyle,
        _false_format: Option<&super::types::CellStyle>,
        _sheet_name: Option<&str>,
    ) -> Result<()> {
        // Note: rust_xlsxwriter doesn't support conditional formatting directly
        // This would require reading the existing workbook, which isn't supported
        // For now, we'll create a new file with the conditional format as a note
        anyhow::bail!(
            "Conditional formatting requires reading existing workbook, which rust_xlsxwriter doesn't support. Use Excel's built-in conditional formatting after file creation."
        );
    }

    fn parse_cell_reference_writer(&self, cell: &str) -> Result<(u32, u16)> {
        // Parse cell reference like "A1" to (row, col)
        let mut chars = cell.chars();
        let mut col_str = String::new();

        while let Some(c) = chars.next() {
            if c.is_alphabetic() {
                col_str.push(c);
            } else {
                break;
            }
        }

        let row_str: String = chars.collect();
        let row: u32 = row_str
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid row number in cell reference: {}", cell))?;

        let col = self.column_name_to_index_writer(&col_str)?;

        Ok((row - 1, col)) // Convert to 0-indexed
    }

    fn column_name_to_index_writer(&self, name: &str) -> Result<u16> {
        let mut index = 0;
        for c in name.chars() {
            if !c.is_alphabetic() {
                anyhow::bail!("Invalid column name: {}", name);
            }
            index = index * 26 + (c.to_ascii_uppercase() as u16 - b'A' as u16 + 1);
        }
        Ok(index - 1) // Convert to 0-indexed
    }
}
