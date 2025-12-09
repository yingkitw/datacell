//! Core data operations struct and basic methods

use anyhow::Result;
use super::types::SortOrder;

/// Data operations for spreadsheet manipulation
pub struct DataOperations;

impl DataOperations {
    pub fn new() -> Self {
        Self
    }
    
    /// Sort rows by a specific column
    pub fn sort_by_column(
        &self,
        data: &mut Vec<Vec<String>>,
        column: usize,
        order: SortOrder,
    ) -> Result<()> {
        if data.is_empty() {
            return Ok(());
        }
        
        let max_cols = data.iter().map(|r| r.len()).max().unwrap_or(0);
        if column >= max_cols {
            anyhow::bail!("Column index {} out of range (max: {})", column, max_cols - 1);
        }
        
        data.sort_by(|a, b| {
            let val_a = a.get(column).map(|s| s.as_str()).unwrap_or("");
            let val_b = b.get(column).map(|s| s.as_str()).unwrap_or("");
            
            let cmp = match (val_a.parse::<f64>(), val_b.parse::<f64>()) {
                (Ok(num_a), Ok(num_b)) => num_a.partial_cmp(&num_b).unwrap_or(std::cmp::Ordering::Equal),
                _ => val_a.cmp(val_b),
            };
            
            match order {
                SortOrder::Ascending => cmp,
                SortOrder::Descending => cmp.reverse(),
            }
        });
        
        Ok(())
    }
    
    /// Filter rows by a condition on a column
    pub fn filter_rows(
        &self,
        data: &[Vec<String>],
        column: usize,
        operator: &str,
        value: &str,
    ) -> Result<Vec<Vec<String>>> {
        let mut result = Vec::new();
        
        for row in data {
            let cell_value = row.get(column).map(|s| s.as_str()).unwrap_or("");
            if self.evaluate_filter_condition(cell_value, operator, value)? {
                result.push(row.clone());
            }
        }
        
        Ok(result)
    }
    
    /// Evaluate a filter condition
    pub fn evaluate_filter_condition(&self, cell_value: &str, operator: &str, value: &str) -> Result<bool> {
        let result = match operator {
            "=" | "==" => cell_value == value,
            "!=" | "<>" => cell_value != value,
            ">" => {
                match (cell_value.parse::<f64>(), value.parse::<f64>()) {
                    (Ok(a), Ok(b)) => a > b,
                    _ => cell_value > value,
                }
            }
            ">=" => {
                match (cell_value.parse::<f64>(), value.parse::<f64>()) {
                    (Ok(a), Ok(b)) => a >= b,
                    _ => cell_value >= value,
                }
            }
            "<" => {
                match (cell_value.parse::<f64>(), value.parse::<f64>()) {
                    (Ok(a), Ok(b)) => a < b,
                    _ => cell_value < value,
                }
            }
            "<=" => {
                match (cell_value.parse::<f64>(), value.parse::<f64>()) {
                    (Ok(a), Ok(b)) => a <= b,
                    _ => cell_value <= value,
                }
            }
            "contains" => cell_value.contains(value),
            "starts_with" => cell_value.starts_with(value),
            "ends_with" => cell_value.ends_with(value),
            _ => anyhow::bail!("Unknown operator: {}", operator),
        };
        Ok(result)
    }
    
    /// Replace values in a column
    pub fn replace(
        &self,
        data: &mut Vec<Vec<String>>,
        column: usize,
        find: &str,
        replace_with: &str,
    ) -> usize {
        let mut count = 0;
        for row in data.iter_mut() {
            if let Some(cell) = row.get_mut(column) {
                if cell.contains(find) {
                    *cell = cell.replace(find, replace_with);
                    count += 1;
                }
            }
        }
        count
    }
    
    /// Find and replace across all columns
    pub fn find_replace(
        &self,
        data: &mut Vec<Vec<String>>,
        find: &str,
        replace_with: &str,
        _column: Option<usize>,
    ) -> Result<usize> {
        let mut count = 0;
        for row in data.iter_mut() {
            for cell in row.iter_mut() {
                if cell.contains(find) {
                    *cell = cell.replace(find, replace_with);
                    count += 1;
                }
            }
        }
        Ok(count)
    }
    
    /// Remove duplicate rows (returns new vec)
    pub fn deduplicate(&self, data: &[Vec<String>]) -> Vec<Vec<String>> {
        use std::collections::HashSet;
        let mut seen: HashSet<Vec<String>> = HashSet::new();
        data.iter()
            .filter(|row| seen.insert((*row).clone()))
            .cloned()
            .collect()
    }
    
    /// Remove duplicate rows in place
    pub fn deduplicate_mut(&self, data: &mut Vec<Vec<String>>) -> usize {
        use std::collections::HashSet;
        let original_len = data.len();
        let mut seen: HashSet<Vec<String>> = HashSet::new();
        data.retain(|row| seen.insert(row.clone()));
        original_len - data.len()
    }
    
    /// Transpose data (rows to columns)
    pub fn transpose(&self, data: &[Vec<String>]) -> Vec<Vec<String>> {
        if data.is_empty() {
            return Vec::new();
        }
        
        let max_cols = data.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut result = vec![vec![String::new(); data.len()]; max_cols];
        
        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                result[col_idx][row_idx] = cell.clone();
            }
        }
        
        result
    }
    
    /// Format data as markdown table
    pub fn to_markdown(&self, data: &[Vec<String>]) -> String {
        if data.is_empty() {
            return String::new();
        }
        
        let mut output = String::new();
        
        // Header row
        if let Some(header) = data.first() {
            output.push_str("| ");
            output.push_str(&header.join(" | "));
            output.push_str(" |\n");
            
            // Separator
            output.push_str("| ");
            output.push_str(&header.iter().map(|_| "---").collect::<Vec<_>>().join(" | "));
            output.push_str(" |\n");
        }
        
        // Data rows
        for row in data.iter().skip(1) {
            output.push_str("| ");
            output.push_str(&row.join(" | "));
            output.push_str(" |\n");
        }
        
        output
    }
    
    /// Insert a row at a specific index
    pub fn insert_row(&self, data: &mut Vec<Vec<String>>, index: usize, row: Vec<String>) {
        if index <= data.len() {
            data.insert(index, row);
        }
    }
    
    /// Delete a row at a specific index
    pub fn delete_row(&self, data: &mut Vec<Vec<String>>, index: usize) -> Option<Vec<String>> {
        if index < data.len() {
            Some(data.remove(index))
        } else {
            None
        }
    }
    
    /// Insert a column at a specific index
    pub fn insert_column(&self, data: &mut Vec<Vec<String>>, index: usize, values: Vec<String>) {
        for (row_idx, row) in data.iter_mut().enumerate() {
            let value = values.get(row_idx).cloned().unwrap_or_default();
            if index <= row.len() {
                row.insert(index, value);
            } else {
                row.push(value);
            }
        }
    }
    
    /// Delete a column at a specific index
    pub fn delete_column(&self, data: &mut Vec<Vec<String>>, index: usize) {
        for row in data.iter_mut() {
            if index < row.len() {
                row.remove(index);
            }
        }
    }
}
