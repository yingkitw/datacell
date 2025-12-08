use anyhow::{Context, Result};
use crate::excel::ExcelHandler;
use calamine::{open_workbook, Reader, Xlsx};
use csv::{ReaderBuilder, WriterBuilder};

/// Result of formula evaluation - can be number or string
#[derive(Debug, Clone)]
pub enum FormulaResult {
    Number(f64),
    Text(String),
}

impl std::fmt::Display for FormulaResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaResult::Number(n) => write!(f, "{}", n),
            FormulaResult::Text(s) => write!(f, "{}", s),
        }
    }
}

impl FormulaResult {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            FormulaResult::Number(n) => Some(*n),
            FormulaResult::Text(s) => s.parse().ok(),
        }
    }
}

struct CellRange {
    start_row: u32,
    start_col: u16,
    end_row: u32,
    end_col: u16,
}

pub struct FormulaEvaluator {
    excel_handler: ExcelHandler,
}

impl FormulaEvaluator {
    pub fn new() -> Self {
        Self {
            excel_handler: ExcelHandler::new(),
        }
    }

    pub fn apply_to_excel(
        &self,
        input: &str,
        output: &str,
        formula: &str,
        cell: &str,
        sheet_name: Option<&str>,
    ) -> Result<()> {
        // Read existing Excel file
        let mut workbook: Xlsx<_> = open_workbook(input)
            .with_context(|| format!("Failed to open Excel file: {}", input))?;

        let sheet_names = workbook.sheet_names();
        let sheet_name = sheet_name
            .or_else(|| sheet_names.first().map(|s| s.as_str()))
            .ok_or_else(|| anyhow::anyhow!("No sheets found in workbook"))?;

        let range = workbook
            .worksheet_range(sheet_name)
            .with_context(|| format!("Failed to read sheet: {}", sheet_name))?;

        // Create new workbook with data and formula
        let mut new_workbook = rust_xlsxwriter::Workbook::new();
        let worksheet = new_workbook.add_worksheet().set_name(sheet_name)?;

        // Copy existing data
        for (row_idx, row) in range.rows().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if let Ok(num) = cell.to_string().parse::<f64>() {
                    worksheet.write_number(row_idx as u32, col_idx as u16, num)?;
                } else {
                    worksheet.write_string(row_idx as u32, col_idx as u16, &cell.to_string())?;
                }
            }
        }

        // Parse cell reference and apply formula
        let (row, col) = self.excel_handler.parse_cell_reference(cell)?;
        worksheet.write_formula(row, col, formula)?;

        new_workbook.save(output)
            .with_context(|| format!("Failed to save Excel file: {}", output))?;

        Ok(())
    }

    pub fn apply_to_csv(&self, input: &str, output: &str, formula: &str, cell: &str) -> Result<()> {
        // Read CSV
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(input)
            .with_context(|| format!("Failed to open CSV file: {}", input))?;

        let mut records: Vec<Vec<String>> = Vec::new();
        for result in reader.records() {
            let record = result?;
            records.push(record.iter().map(|s| s.to_string()).collect());
        }

        // Evaluate formula
        let (row, col) = self.parse_cell_reference(cell)?;
        let value = self.evaluate_formula_full(formula, &records)?;

        // Ensure we have enough rows
        while records.len() <= row as usize {
            records.push(Vec::new());
        }

        // Find max columns to ensure consistent row lengths
        let max_cols = records.iter()
            .map(|r| r.len())
            .max()
            .unwrap_or(0)
            .max((col as usize) + 1);

        // Ensure all rows have the same number of columns
        for record in &mut records {
            while record.len() < max_cols {
                record.push(String::new());
            }
        }

        // Ensure we have enough columns in the target row
        while records[row as usize].len() <= col as usize {
            records[row as usize].push(String::new());
        }

        // Set the calculated value
        records[row as usize][col as usize] = value.to_string();

        // Write back to CSV
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(output)
            .with_context(|| format!("Failed to create CSV file: {}", output))?;

        for record in records {
            writer.write_record(&record)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn parse_cell_reference(&self, cell: &str) -> Result<(u32, u16)> {
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
        
        // CSV rows are 1-indexed, but we use 0-indexed internally
        Ok((row - 1, col))
    }

    fn column_to_index(&self, col: &str) -> Result<u16> {
        let mut index = 0u16;
        for ch in col.chars() {
            index = index * 26 + (ch.to_ascii_uppercase() as u16 - b'A' as u16 + 1);
        }
        Ok(index - 1)
    }

    /// Evaluate formula returning FormulaResult (supports both numeric and text results)
    fn evaluate_formula_full(&self, formula: &str, data: &[Vec<String>]) -> Result<FormulaResult> {
        let formula_upper = formula.trim().to_uppercase();

        if formula_upper.starts_with("IF(") {
            self.evaluate_if(formula, data)
        } else if formula_upper.starts_with("CONCAT(") {
            self.evaluate_concat(formula, data)
        } else {
            // Delegate to numeric evaluation
            let num = self.evaluate_formula(&formula_upper, data)?;
            Ok(FormulaResult::Number(num))
        }
    }

    fn evaluate_formula(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        // Basic formula evaluation
        // Supports: SUM, AVERAGE, MIN, MAX, COUNT, basic arithmetic operations
        
        let formula = formula.trim().to_uppercase();

        if formula.starts_with("SUM(") {
            self.evaluate_sum(&formula, data)
        } else if formula.starts_with("AVERAGE(") {
            self.evaluate_average(&formula, data)
        } else if formula.starts_with("MIN(") {
            self.evaluate_min(&formula, data)
        } else if formula.starts_with("MAX(") {
            self.evaluate_max(&formula, data)
        } else if formula.starts_with("COUNT(") {
            self.evaluate_count(&formula, data)
        } else if formula.starts_with("ROUND(") {
            self.evaluate_round(&formula, data)
        } else if formula.starts_with("ABS(") {
            self.evaluate_abs(&formula, data)
        } else if formula.starts_with("LEN(") {
            self.evaluate_len(&formula, data)
        } else if formula.contains('+') || formula.contains('-') || formula.contains('*') || formula.contains('/') {
            self.evaluate_arithmetic(&formula, data)
        } else if let Ok(num) = formula.parse::<f64>() {
            // It's a numeric literal
            Ok(num)
        } else {
            // Try to parse as a cell reference
            self.get_cell_value(&formula, data)
        }
    }
    
    /// Evaluate IF(condition, true_value, false_value)
    fn evaluate_if(&self, formula: &str, data: &[Vec<String>]) -> Result<FormulaResult> {
        let inner = self.extract_function_args(formula)?;
        let args = self.split_args(&inner)?;
        
        if args.len() != 3 {
            anyhow::bail!("IF requires 3 arguments: IF(condition, true_value, false_value)");
        }
        
        let condition = self.evaluate_condition(&args[0], data)?;
        let result_expr = if condition { &args[1] } else { &args[2] };
        
        // Try to evaluate as formula, otherwise return as text
        if let Ok(num) = self.evaluate_formula(result_expr, data) {
            Ok(FormulaResult::Number(num))
        } else {
            // Return as text (strip quotes if present)
            let text = result_expr.trim().trim_matches('"').to_string();
            Ok(FormulaResult::Text(text))
        }
    }
    
    /// Evaluate CONCAT(arg1, arg2, ...)
    fn evaluate_concat(&self, formula: &str, data: &[Vec<String>]) -> Result<FormulaResult> {
        let inner = self.extract_function_args(formula)?;
        let args = self.split_args(&inner)?;
        
        let mut result = String::new();
        for arg in args {
            let arg = arg.trim();
            if arg.starts_with('"') && arg.ends_with('"') {
                // String literal
                result.push_str(&arg[1..arg.len()-1]);
            } else if let Ok(num) = self.evaluate_formula(arg, data) {
                result.push_str(&num.to_string());
            } else {
                // Try as cell reference for text
                let (row, col) = self.parse_cell_reference(arg)?;
                if let Some(text) = self.get_cell_text_by_index(row, col, data) {
                    result.push_str(&text);
                }
            }
        }
        
        Ok(FormulaResult::Text(result))
    }
    
    /// Evaluate a condition like "A1>5" or "A1=B1"
    fn evaluate_condition(&self, condition: &str, data: &[Vec<String>]) -> Result<bool> {
        let condition = condition.trim().to_uppercase();
        
        // Check for comparison operators (order matters - check multi-char first)
        let operators = [">=", "<=", "<>", ">", "<", "="];
        
        for op in operators {
            if let Some(pos) = condition.find(op) {
                let left = &condition[..pos];
                let right = &condition[pos + op.len()..];
                let left_val = self.evaluate_formula(left, data)?;
                let right_val = self.evaluate_formula(right, data)?;
                
                return Ok(match op {
                    ">=" => left_val >= right_val,
                    "<=" => left_val <= right_val,
                    "<>" => (left_val - right_val).abs() > f64::EPSILON,
                    ">" => left_val > right_val,
                    "<" => left_val < right_val,
                    "=" => (left_val - right_val).abs() < f64::EPSILON,
                    _ => false,
                });
            }
        }
        
        // If no operator, treat as boolean (non-zero = true)
        let val = self.evaluate_formula(&condition, data)?;
        Ok(val != 0.0)
    }
    
    /// Extract arguments from function call like "IF(a,b,c)" -> "a,b,c"
    fn extract_function_args(&self, formula: &str) -> Result<String> {
        let start = formula.find('(').ok_or_else(|| anyhow::anyhow!("Missing opening parenthesis"))?;
        let end = formula.rfind(')').ok_or_else(|| anyhow::anyhow!("Missing closing parenthesis"))?;
        Ok(formula[start+1..end].to_string())
    }
    
    /// Split arguments respecting nested parentheses and quotes
    fn split_args(&self, args: &str) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut paren_depth = 0;
        let mut in_quotes = false;
        
        for ch in args.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                }
                '(' if !in_quotes => {
                    paren_depth += 1;
                    current.push(ch);
                }
                ')' if !in_quotes => {
                    paren_depth -= 1;
                    current.push(ch);
                }
                ',' if !in_quotes && paren_depth == 0 => {
                    result.push(current.trim().to_string());
                    current = String::new();
                }
                _ => current.push(ch),
            }
        }
        
        if !current.is_empty() {
            result.push(current.trim().to_string());
        }
        
        Ok(result)
    }
    
    /// Get cell value as text
    fn get_cell_text_by_index(&self, row: u32, col: u16, data: &[Vec<String>]) -> Option<String> {
        if row as usize >= data.len() {
            return None;
        }
        let row_data = &data[row as usize];
        if col as usize >= row_data.len() {
            return None;
        }
        Some(row_data[col as usize].clone())
    }

    fn evaluate_sum(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let range = self.extract_range(formula)?;
        let mut sum = 0.0;

        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if let Some(value) = self.get_cell_value_by_index(row, col, data) {
                    sum += value;
                }
            }
        }

        Ok(sum)
    }

    fn evaluate_average(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let range = self.extract_range(formula)?;
        let mut sum = 0.0;
        let mut count = 0;

        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if let Some(value) = self.get_cell_value_by_index(row, col, data) {
                    sum += value;
                    count += 1;
                }
            }
        }

        if count == 0 {
            Ok(0.0)
        } else {
            Ok(sum / count as f64)
        }
    }

    fn evaluate_min(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let range = self.extract_range(formula)?;
        let mut min: Option<f64> = None;

        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if let Some(value) = self.get_cell_value_by_index(row, col, data) {
                    min = Some(min.map_or(value, |m| m.min(value)));
                }
            }
        }

        min.ok_or_else(|| anyhow::anyhow!("No numeric values found in range"))
    }

    fn evaluate_max(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let range = self.extract_range(formula)?;
        let mut max: Option<f64> = None;

        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if let Some(value) = self.get_cell_value_by_index(row, col, data) {
                    max = Some(max.map_or(value, |m| m.max(value)));
                }
            }
        }

        max.ok_or_else(|| anyhow::anyhow!("No numeric values found in range"))
    }

    fn evaluate_count(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let range = self.extract_range(formula)?;
        let mut count = 0;

        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if self.get_cell_value_by_index(row, col, data).is_some() {
                    count += 1;
                }
            }
        }

        Ok(count as f64)
    }
    
    /// ROUND(value, decimals) - Round to specified decimal places
    fn evaluate_round(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let inner = self.extract_function_args(formula)?;
        let args = self.split_args(&inner)?;
        
        if args.is_empty() || args.len() > 2 {
            anyhow::bail!("ROUND requires 1-2 arguments: ROUND(value, [decimals])");
        }
        
        let value = self.evaluate_formula(&args[0], data)?;
        let decimals = if args.len() > 1 {
            self.evaluate_formula(&args[1], data)? as i32
        } else {
            0
        };
        
        let multiplier = 10f64.powi(decimals);
        Ok((value * multiplier).round() / multiplier)
    }
    
    /// ABS(value) - Absolute value
    fn evaluate_abs(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let inner = self.extract_function_args(formula)?;
        let value = self.evaluate_formula(&inner, data)?;
        Ok(value.abs())
    }
    
    /// LEN(text) - Length of text in cell
    fn evaluate_len(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let inner = self.extract_function_args(formula)?;
        let inner = inner.trim().to_uppercase();
        
        // Try as cell reference first
        if let Ok((row, col)) = self.parse_cell_reference(&inner) {
            if let Some(text) = self.get_cell_text_by_index(row, col, data) {
                return Ok(text.len() as f64);
            }
        }
        
        // Try as string literal
        let text = inner.trim_matches('"');
        Ok(text.len() as f64)
    }

    fn evaluate_arithmetic(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        // Replace cell references with values
        let mut formula_str = formula.to_string();
        
        // Find and replace cell references (e.g., A1, B2)
        let re = regex::Regex::new(r"([A-Z]+)(\d+)").unwrap();
        let mut replacements = Vec::new();
        
        for cap in re.captures_iter(formula) {
            let col_str = &cap[1];
            let row_str = &cap[2];
            let cell_ref = format!("{}{}", col_str, row_str);
            let (row, col) = self.parse_cell_reference(&cell_ref)?;
            
            if let Some(value) = self.get_cell_value_by_index(row, col, data) {
                replacements.push((cell_ref, value.to_string()));
            }
        }
        
        for (cell_ref, value) in replacements {
            formula_str = formula_str.replace(&cell_ref, &value);
        }

        // Evaluate the arithmetic expression
        // Simple evaluation - in production, use a proper expression evaluator
        // For now, support basic operations
        if let Ok(result) = self.simple_eval(&formula_str) {
            Ok(result)
        } else {
            anyhow::bail!("Failed to evaluate formula: {}", formula_str)
        }
    }

    fn simple_eval(&self, expr: &str) -> Result<f64> {
        // Very basic arithmetic evaluation
        // This is a simplified version - for production, use a proper math parser
        let expr = expr.replace(" ", "");
        
        // Handle addition
        if let Some(pos) = expr.rfind('+') {
            let left = self.simple_eval(&expr[..pos])?;
            let right = self.simple_eval(&expr[pos+1..])?;
            return Ok(left + right);
        }
        
        // Handle subtraction
        if let Some(pos) = expr.rfind('-') {
            if pos > 0 { // Not at the start (negative number)
                let left = self.simple_eval(&expr[..pos])?;
                let right = self.simple_eval(&expr[pos+1..])?;
                return Ok(left - right);
            }
        }
        
        // Handle multiplication
        if let Some(pos) = expr.rfind('*') {
            let left = self.simple_eval(&expr[..pos])?;
            let right = self.simple_eval(&expr[pos+1..])?;
            return Ok(left * right);
        }
        
        // Handle division
        if let Some(pos) = expr.rfind('/') {
            let left = self.simple_eval(&expr[..pos])?;
            let right = self.simple_eval(&expr[pos+1..])?;
            if right == 0.0 {
                anyhow::bail!("Division by zero");
            }
            return Ok(left / right);
        }
        
        // Parse number
        expr.parse::<f64>()
            .with_context(|| format!("Failed to parse number: {}", expr))
    }

    fn get_cell_value(&self, cell_ref: &str, data: &[Vec<String>]) -> Result<f64> {
        let (row, col) = self.parse_cell_reference(cell_ref)?;
        self.get_cell_value_by_index(row, col, data)
            .ok_or_else(|| anyhow::anyhow!("Cell {} is empty or invalid", cell_ref))
    }

    fn get_cell_value_by_index(&self, row: u32, col: u16, data: &[Vec<String>]) -> Option<f64> {
        if row as usize >= data.len() {
            return None;
        }
        let row_data = &data[row as usize];
        if col as usize >= row_data.len() {
            return None;
        }
        row_data[col as usize].parse::<f64>().ok()
    }

    fn extract_range(&self, formula: &str) -> Result<CellRange> {
        // Extract range like "A1:B10" from "SUM(A1:B10)"
        let start = formula.find('(').ok_or_else(|| anyhow::anyhow!("Invalid formula format"))?;
        let end = formula.rfind(')').ok_or_else(|| anyhow::anyhow!("Invalid formula format"))?;
        let range_str = &formula[start+1..end];

        if let Some(colon_pos) = range_str.find(':') {
            let start_cell = &range_str[..colon_pos];
            let end_cell = &range_str[colon_pos+1..];
            
            let (start_row, start_col) = self.parse_cell_reference(start_cell)?;
            let (end_row, end_col) = self.parse_cell_reference(end_cell)?;

            Ok(CellRange {
                start_row,
                start_col,
                end_row,
                end_col,
            })
        } else {
            // Single cell
            let (row, col) = self.parse_cell_reference(range_str)?;
            Ok(CellRange {
                start_row: row,
                start_col: col,
                end_row: row,
                end_col: col,
            })
        }
    }
}

