//! Core data operations struct and basic methods

use anyhow::Result;
use super::types::SortOrder;
use crate::traits::{SortOperator, FilterOperator, TransformOperator, DataOperator, FilterCondition, TransformOperation};

/// Data operations for spreadsheet manipulation
pub struct DataOperations;

impl DataOperations {
    pub fn new() -> Self {
        Self
    }
}

// Trait implementations for better SOC
impl SortOperator for DataOperations {
    
    fn sort(
        &self,
        data: &mut Vec<Vec<String>>,
        column: usize,
        ascending: bool,
    ) -> Result<()> {
        let order = if ascending { SortOrder::Ascending } else { SortOrder::Descending };
        self.sort_by_column(data, column, order)
    }
}

impl DataOperations {
    /// Sort rows by a specific column (public for backward compatibility)
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
    }
    
impl FilterOperator for DataOperations {
    fn filter(
        &self,
        data: &[Vec<String>],
        column: usize,
        condition: FilterCondition,
    ) -> Result<Vec<Vec<String>>> {
        let mut result = Vec::new();
        
        for row in data {
            let cell_value = row.get(column).map(|s| s.as_str()).unwrap_or("");
            if self.evaluate_condition(cell_value, &condition)? {
                result.push(row.clone());
            }
        }
        
        Ok(result)
    }
}

impl DataOperations {
    /// Filter rows by a condition on a column (legacy method for compatibility)
    pub fn filter_rows(
        &self,
        data: &[Vec<String>],
        column: usize,
        operator: &str,
        value: &str,
    ) -> Result<Vec<Vec<String>>> {
        let condition = self.parse_filter_condition(operator, value)?;
        <Self as FilterOperator>::filter(self, data, column, condition)
    }
    
    fn parse_filter_condition(&self, operator: &str, value: &str) -> Result<FilterCondition> {
        Ok(match operator {
            "=" | "==" => FilterCondition::Equals(value.to_string()),
            "!=" | "<>" => FilterCondition::NotEquals(value.to_string()),
            ">" => FilterCondition::GreaterThan(value.to_string()),
            ">=" => FilterCondition::GreaterThanOrEqual(value.to_string()),
            "<" => FilterCondition::LessThan(value.to_string()),
            "<=" => FilterCondition::LessThanOrEqual(value.to_string()),
            "contains" => FilterCondition::Contains(value.to_string()),
            "starts_with" => FilterCondition::StartsWith(value.to_string()),
            "ends_with" => FilterCondition::EndsWith(value.to_string()),
            _ => anyhow::bail!("Unknown operator: {}", operator),
        })
    }
    
    fn evaluate_condition(&self, cell_value: &str, condition: &FilterCondition) -> Result<bool> {
        Ok(match condition {
            FilterCondition::Equals(v) => cell_value == v,
            FilterCondition::NotEquals(v) => cell_value != v,
            FilterCondition::GreaterThan(v) => {
                match (cell_value.parse::<f64>(), v.parse::<f64>()) {
                    (Ok(a), Ok(b)) => a > b,
                    _ => cell_value > v.as_str(),
                }
            }
            FilterCondition::GreaterThanOrEqual(v) => {
                match (cell_value.parse::<f64>(), v.parse::<f64>()) {
                    (Ok(a), Ok(b)) => a >= b,
                    _ => cell_value >= v.as_str(),
                }
            }
            FilterCondition::LessThan(v) => {
                match (cell_value.parse::<f64>(), v.parse::<f64>()) {
                    (Ok(a), Ok(b)) => a < b,
                    _ => cell_value < v.as_str(),
                }
            }
            FilterCondition::LessThanOrEqual(v) => {
                match (cell_value.parse::<f64>(), v.parse::<f64>()) {
                    (Ok(a), Ok(b)) => a <= b,
                    _ => cell_value <= v.as_str(),
                }
            }
            FilterCondition::Contains(v) => cell_value.contains(v),
            FilterCondition::StartsWith(v) => cell_value.starts_with(v),
            FilterCondition::EndsWith(v) => cell_value.ends_with(v),
            FilterCondition::Regex(pattern) => {
                use regex::Regex;
                let re = Regex::new(pattern)?;
                re.is_match(cell_value)
            }
        })
    }
    
    /// Evaluate a filter condition (legacy method for compatibility)
    pub fn evaluate_filter_condition(&self, cell_value: &str, operator: &str, value: &str) -> Result<bool> {
        let condition = self.parse_filter_condition(operator, value)?;
        self.evaluate_condition(cell_value, &condition)
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

impl TransformOperator for DataOperations {
    fn transform(
        &self,
        data: &mut Vec<Vec<String>>,
        operation: TransformOperation,
    ) -> Result<()> {
        match operation {
            TransformOperation::RenameColumn { from, to } => {
                if let Some(row) = data.first_mut() {
                    if from < row.len() {
                        row[from] = to;
                    }
                }
            }
            TransformOperation::DropColumn(col_idx) => {
                for row in data.iter_mut() {
                    if col_idx < row.len() {
                        row.remove(col_idx);
                    }
                }
            }
            TransformOperation::AddColumn { name, formula: _ } => {
                // TODO: Implement formula evaluation
                for row in data.iter_mut() {
                    row.push(name.clone());
                }
            }
            TransformOperation::FillNa { column, value } => {
                for row in data.iter_mut() {
                    if column < row.len() && row[column].is_empty() {
                        row[column] = value.clone();
                    }
                }
            }
        }
        Ok(())
    }
}

impl DataOperator for DataOperations {}

