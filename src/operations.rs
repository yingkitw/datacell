use anyhow::Result;
use crate::csv_handler::CellRange;

/// Data operations for spreadsheet manipulation
pub struct DataOperations;

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

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
        
        // Check column exists
        let max_cols = data.iter().map(|r| r.len()).max().unwrap_or(0);
        if column >= max_cols {
            anyhow::bail!("Column index {} out of range (max: {})", column, max_cols - 1);
        }
        
        data.sort_by(|a, b| {
            let val_a = a.get(column).map(|s| s.as_str()).unwrap_or("");
            let val_b = b.get(column).map(|s| s.as_str()).unwrap_or("");
            
            // Try numeric comparison first
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
            
            let matches = self.evaluate_filter_condition(cell_value, operator, value)?;
            if matches {
                result.push(row.clone());
            }
        }
        
        Ok(result)
    }
    
    fn evaluate_filter_condition(&self, cell_value: &str, operator: &str, filter_value: &str) -> Result<bool> {
        // Try numeric comparison
        if let (Ok(cell_num), Ok(filter_num)) = (cell_value.parse::<f64>(), filter_value.parse::<f64>()) {
            return Ok(match operator {
                "=" | "==" => (cell_num - filter_num).abs() < f64::EPSILON,
                "!=" | "<>" => (cell_num - filter_num).abs() >= f64::EPSILON,
                ">" => cell_num > filter_num,
                "<" => cell_num < filter_num,
                ">=" => cell_num >= filter_num,
                "<=" => cell_num <= filter_num,
                _ => anyhow::bail!("Unknown operator: {}", operator),
            });
        }
        
        // String comparison
        Ok(match operator {
            "=" | "==" => cell_value == filter_value,
            "!=" | "<>" => cell_value != filter_value,
            ">" => cell_value > filter_value,
            "<" => cell_value < filter_value,
            ">=" => cell_value >= filter_value,
            "<=" => cell_value <= filter_value,
            "contains" => cell_value.contains(filter_value),
            "starts_with" => cell_value.starts_with(filter_value),
            "ends_with" => cell_value.ends_with(filter_value),
            _ => anyhow::bail!("Unknown operator: {}", operator),
        })
    }
    
    /// Find and replace values in data
    pub fn find_replace(
        &self,
        data: &mut Vec<Vec<String>>,
        find: &str,
        replace: &str,
        range: Option<&CellRange>,
    ) -> Result<usize> {
        let mut count = 0;
        
        for (row_idx, row) in data.iter_mut().enumerate() {
            for (col_idx, cell) in row.iter_mut().enumerate() {
                // Check if within range
                if let Some(r) = range {
                    if row_idx < r.start_row || row_idx > r.end_row {
                        continue;
                    }
                    if col_idx < r.start_col || col_idx > r.end_col {
                        continue;
                    }
                }
                
                if cell.contains(find) {
                    *cell = cell.replace(find, replace);
                    count += 1;
                }
            }
        }
        
        Ok(count)
    }
    
    /// Remove duplicate rows
    pub fn deduplicate(&self, data: &[Vec<String>]) -> Vec<Vec<String>> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        
        for row in data {
            let key = row.join("\0"); // Use null byte as separator
            if seen.insert(key) {
                result.push(row.clone());
            }
        }
        
        result
    }
    
    /// Transpose data (rows become columns)
    pub fn transpose(&self, data: &[Vec<String>]) -> Vec<Vec<String>> {
        if data.is_empty() {
            return Vec::new();
        }
        
        let max_cols = data.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut result = vec![Vec::new(); max_cols];
        
        for row in data {
            for (col_idx, cell) in row.iter().enumerate() {
                result[col_idx].push(cell.clone());
            }
            // Pad with empty strings for shorter rows
            for col_idx in row.len()..max_cols {
                result[col_idx].push(String::new());
            }
        }
        
        result
    }
    
    /// Insert a row at specified index
    pub fn insert_row(&self, data: &mut Vec<Vec<String>>, index: usize, row: Vec<String>) {
        if index >= data.len() {
            data.push(row);
        } else {
            data.insert(index, row);
        }
    }
    
    /// Insert a column at specified index
    pub fn insert_column(&self, data: &mut Vec<Vec<String>>, index: usize, values: &[String]) {
        for (row_idx, row) in data.iter_mut().enumerate() {
            let value = values.get(row_idx).cloned().unwrap_or_default();
            if index >= row.len() {
                row.push(value);
            } else {
                row.insert(index, value);
            }
        }
    }
    
    /// Delete a row at specified index
    pub fn delete_row(&self, data: &mut Vec<Vec<String>>, index: usize) -> Option<Vec<String>> {
        if index < data.len() {
            Some(data.remove(index))
        } else {
            None
        }
    }
    
    /// Delete a column at specified index
    pub fn delete_column(&self, data: &mut Vec<Vec<String>>, index: usize) {
        for row in data.iter_mut() {
            if index < row.len() {
                row.remove(index);
            }
        }
    }
    
    /// Export data as Markdown table
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
            output.push_str("|");
            for _ in header {
                output.push_str(" --- |");
            }
            output.push('\n');
        }
        
        // Data rows
        for row in data.iter().skip(1) {
            output.push_str("| ");
            output.push_str(&row.join(" | "));
            output.push_str(" |\n");
        }
        
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sort_ascending() {
        let ops = DataOperations::new();
        let mut data = vec![
            vec!["3".to_string(), "c".to_string()],
            vec!["1".to_string(), "a".to_string()],
            vec!["2".to_string(), "b".to_string()],
        ];
        
        ops.sort_by_column(&mut data, 0, SortOrder::Ascending).unwrap();
        
        assert_eq!(data[0][0], "1");
        assert_eq!(data[1][0], "2");
        assert_eq!(data[2][0], "3");
    }
    
    #[test]
    fn test_filter_rows() {
        let ops = DataOperations::new();
        let data = vec![
            vec!["10".to_string(), "a".to_string()],
            vec!["5".to_string(), "b".to_string()],
            vec!["15".to_string(), "c".to_string()],
        ];
        
        let result = ops.filter_rows(&data, 0, ">", "8").unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0][0], "10");
        assert_eq!(result[1][0], "15");
    }
    
    #[test]
    fn test_deduplicate() {
        let ops = DataOperations::new();
        let data = vec![
            vec!["a".to_string(), "1".to_string()],
            vec!["b".to_string(), "2".to_string()],
            vec!["a".to_string(), "1".to_string()],
        ];
        
        let result = ops.deduplicate(&data);
        
        assert_eq!(result.len(), 2);
    }
    
    #[test]
    fn test_transpose() {
        let ops = DataOperations::new();
        let data = vec![
            vec!["1".to_string(), "2".to_string()],
            vec!["3".to_string(), "4".to_string()],
        ];
        
        let result = ops.transpose(&data);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec!["1", "3"]);
        assert_eq!(result[1], vec!["2", "4"]);
    }
}
