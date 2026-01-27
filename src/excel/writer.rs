use anyhow::{Context, Result};
use rust_xlsxwriter::Workbook;

use super::reader::ExcelHandler;
use super::types::WriteOptions;
use crate::traits::{DataWriteOptions, DataWriter};

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
            .with_context(|| format!("Failed to open CSV file: {csv_path}"))?;

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
            .with_context(|| format!("Failed to save Excel file: {excel_path}"))?;

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
            .with_context(|| format!("Failed to save Excel file: {excel_path}"))?;

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
            .map_err(|_| anyhow::anyhow!("Invalid row number in cell reference: {cell}"))?;

        let col = self.column_name_to_index_writer(&col_str)?;

        Ok((row - 1, col)) // Convert to 0-indexed
    }

    /// Write data to a specific range in Excel starting at the given row and column
    pub fn write_range(
        &self,
        path: &str,
        data: &[Vec<String>],
        start_row: u32,
        start_col: u16,
        sheet_name: Option<&str>,
    ) -> Result<()> {
        let mut workbook = Workbook::new();
        let sheet_name = sheet_name.unwrap_or("Sheet1");
        let worksheet = workbook.add_worksheet().set_name(sheet_name)?;

        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                let actual_row = start_row + row_idx as u32;
                let actual_col = start_col + col_idx as u16;

                if let Ok(num) = cell.parse::<f64>() {
                    worksheet.write_number(actual_row, actual_col, num)?;
                } else {
                    worksheet.write_string(actual_row, actual_col, cell)?;
                }
            }
        }

        workbook
            .save(path)
            .with_context(|| format!("Failed to save Excel file: {path}"))?;

        Ok(())
    }

    fn column_name_to_index_writer(&self, name: &str) -> Result<u16> {
        let mut index = 0;
        for c in name.chars() {
            if !c.is_alphabetic() {
                anyhow::bail!("Invalid column name: {name}");
            }
            index = index * 26 + (c.to_ascii_uppercase() as u16 - b'A' as u16 + 1);
        }
        Ok(index - 1) // Convert to 0-indexed
    }
}

impl DataWriter for ExcelHandler {
    fn write(&self, path: &str, data: &[Vec<String>], options: DataWriteOptions) -> Result<()> {
        let mut workbook = Workbook::new();
        let sheet_name = options.sheet_name.as_deref().unwrap_or("Sheet1");
        let worksheet = workbook.add_worksheet().set_name(sheet_name)?;

        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if let Ok(num) = cell.parse::<f64>() {
                    worksheet.write_number(row_idx as u32, col_idx as u16, num)?;
                } else {
                    worksheet.write_string(row_idx as u32, col_idx as u16, cell)?;
                }
            }
        }

        workbook
            .save(path)
            .with_context(|| format!("Failed to save Excel file: {path}"))?;

        Ok(())
    }

    fn write_range(
        &self,
        path: &str,
        data: &[Vec<String>],
        start_row: usize,
        start_col: usize,
    ) -> Result<()> {
        self.write_range(
            path,
            data,
            start_row as u32,
            start_col as u16,
            None,
        )
    }

    fn append(&self, path: &str, data: &[Vec<String>]) -> Result<()> {
        // rust_xlsxwriter doesn't support modifying existing files, so we need to:
        // 1. Read the existing file
        // 2. Append the new data
        // 3. Write everything back

        use calamine::{open_workbook, Reader, Xlsx};
        use rust_xlsxwriter::Workbook;

        // Check if file exists
        if !std::path::Path::new(path).exists() {
            // File doesn't exist, just write the data
            return self.write(path, data, DataWriteOptions::default());
        }

        // Read existing data from the file
        let mut existing_data: Vec<Vec<String>> = Vec::new();

        // Try to open as Excel file first
        if let Ok(mut workbook) = open_workbook::<Xlsx<_>, _>(path) {
            let sheet_names = workbook.sheet_names();
            let sheet_name = sheet_names
                .first()
                .map(|s| s.as_str())
                .unwrap_or("Sheet1");

            if let Ok(range) = workbook.worksheet_range(sheet_name) {
                for row in range.rows() {
                    let row_data: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
                    existing_data.push(row_data);
                }
            }
        } else {
            // If not Excel, try reading as CSV (fallback)
            use crate::csv_handler::CsvHandler;
            let csv_handler = CsvHandler::new();
            let csv_str = csv_handler.read(path)?;
            // Parse CSV string into Vec<Vec<String>>
            existing_data = csv_str
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.split(',').map(|s| s.to_string()).collect())
                .collect();
        }

        // Append new data
        existing_data.extend(data.iter().cloned());

        // Write everything back
        let mut workbook = Workbook::new();

        // Determine if first row is a header (check if all strings)
        let has_header = existing_data.first()
            .map(|row| row.iter().all(|cell| cell.parse::<f64>().is_err()))
            .unwrap_or(false);

        let worksheet = workbook.add_worksheet();

        for (row_idx, row) in existing_data.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                // Try to parse as number first
                if let Ok(num) = cell.parse::<f64>() {
                    worksheet.write_number(row_idx as u32, col_idx as u16, num)?;
                } else {
                    worksheet.write_string(row_idx as u32, col_idx as u16, cell)?;
                }
            }
        }

        // Add auto-filter if there's a header
        if has_header && !existing_data.is_empty() {
            let last_col = existing_data[0].len().saturating_sub(1) as u16;
            let last_row = existing_data.len().saturating_sub(1) as u32;
            worksheet.autofilter(0, 0, last_row, last_col)?;
        }

        workbook
            .save(path)
            .with_context(|| format!("Failed to save Excel file: {path}"))?;

        Ok(())
    }

    fn supports_format(&self, path: &str) -> bool {
        let path_lower = path.to_lowercase();
        path_lower.ends_with(".xlsx")
    }
}
