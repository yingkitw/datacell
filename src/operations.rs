use anyhow::Result;
use crate::csv_handler::CellRange;

/// Progress callback for long-running operations
pub trait ProgressCallback: Send {
    fn on_progress(&mut self, current: usize, total: Option<usize>, message: &str);
}

/// Simple progress reporter that prints to stderr
pub struct StderrProgress {
    last_percent: usize,
}

impl StderrProgress {
    pub fn new() -> Self {
        Self { last_percent: 0 }
    }
}

impl ProgressCallback for StderrProgress {
    fn on_progress(&mut self, current: usize, total: Option<usize>, message: &str) {
        if let Some(total) = total {
            let percent = if total > 0 { (current * 100) / total } else { 0 };
            if percent != self.last_percent {
                eprintln!("\r{}: {}% ({}/{})", message, percent, current, total);
                self.last_percent = percent;
            }
        } else {
            eprintln!("\r{}: {} processed", message, current);
        }
    }
}

/// No-op progress callback
pub struct NoProgress;

impl ProgressCallback for NoProgress {
    fn on_progress(&mut self, _current: usize, _total: Option<usize>, _message: &str) {}
}

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
    
    // ============ PANDAS-INSPIRED OPERATIONS ============
    
    /// Select specific columns by index
    pub fn select_columns(&self, data: &[Vec<String>], columns: &[usize]) -> Vec<Vec<String>> {
        data.iter()
            .map(|row| {
                columns.iter()
                    .map(|&idx| row.get(idx).cloned().unwrap_or_default())
                    .collect()
            })
            .collect()
    }
    
    /// Select columns by name (first row is header)
    pub fn select_columns_by_name(&self, data: &[Vec<String>], names: &[&str]) -> Result<Vec<Vec<String>>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }
        
        let header = &data[0];
        let indices: Vec<usize> = names.iter()
            .map(|name| {
                header.iter().position(|h| h == *name)
                    .ok_or_else(|| anyhow::anyhow!("Column '{}' not found", name))
            })
            .collect::<Result<Vec<_>>>()?;
        
        Ok(self.select_columns(data, &indices))
    }
    
    /// Get first n rows (head)
    pub fn head(&self, data: &[Vec<String>], n: usize) -> Vec<Vec<String>> {
        data.iter().take(n).cloned().collect()
    }
    
    /// Get last n rows (tail)
    pub fn tail(&self, data: &[Vec<String>], n: usize) -> Vec<Vec<String>> {
        let len = data.len();
        if n >= len {
            data.to_vec()
        } else {
            data[len - n..].to_vec()
        }
    }
    
    /// Sample random rows
    pub fn sample(&self, data: &[Vec<String>], n: usize, seed: Option<u64>) -> Vec<Vec<String>> {
        use std::collections::HashSet;
        
        if n >= data.len() {
            return data.to_vec();
        }
        
        // Simple LCG random number generator
        let mut rng_state = seed.unwrap_or(42);
        let mut next_rand = || {
            rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
            rng_state
        };
        
        let mut indices = HashSet::new();
        while indices.len() < n {
            let idx = (next_rand() as usize) % data.len();
            indices.insert(idx);
        }
        
        let mut result: Vec<Vec<String>> = indices.iter()
            .map(|&idx| data[idx].clone())
            .collect();
        result.sort_by_key(|_| next_rand()); // Shuffle
        result
    }
    
    /// Drop columns by index
    pub fn drop_columns(&self, data: &[Vec<String>], columns: &[usize]) -> Vec<Vec<String>> {
        let drop_set: std::collections::HashSet<usize> = columns.iter().copied().collect();
        data.iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|(idx, _)| !drop_set.contains(idx))
                    .map(|(_, val)| val.clone())
                    .collect()
            })
            .collect()
    }
    
    /// Rename columns (first row is header)
    pub fn rename_columns(&self, data: &mut Vec<Vec<String>>, renames: &[(&str, &str)]) -> Result<()> {
        if data.is_empty() {
            return Ok(());
        }
        
        let header = &mut data[0];
        for (old_name, new_name) in renames {
            if let Some(pos) = header.iter().position(|h| h == *old_name) {
                header[pos] = new_name.to_string();
            }
        }
        Ok(())
    }
    
    /// Fill missing/empty values
    pub fn fillna(&self, data: &mut Vec<Vec<String>>, value: &str) {
        for row in data.iter_mut() {
            for cell in row.iter_mut() {
                if cell.is_empty() {
                    *cell = value.to_string();
                }
            }
        }
    }
    
    /// Drop rows with any empty values
    pub fn dropna(&self, data: &[Vec<String>]) -> Vec<Vec<String>> {
        data.iter()
            .filter(|row| !row.iter().any(|cell| cell.is_empty()))
            .cloned()
            .collect()
    }
    
    /// Concatenate multiple datasets vertically
    pub fn concat(&self, datasets: &[Vec<Vec<String>>]) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        for dataset in datasets {
            result.extend(dataset.iter().cloned());
        }
        result
    }
    
    /// Join two datasets on a column (inner join)
    pub fn join(
        &self,
        left: &[Vec<String>],
        right: &[Vec<String>],
        left_col: usize,
        right_col: usize,
        how: JoinType,
    ) -> Result<Vec<Vec<String>>> {
        use std::collections::HashMap;
        
        if left.is_empty() || right.is_empty() {
            return Ok(Vec::new());
        }
        
        // Build index on right table
        let mut right_index: HashMap<String, Vec<usize>> = HashMap::new();
        for (idx, row) in right.iter().enumerate() {
            if let Some(key) = row.get(right_col) {
                right_index.entry(key.clone()).or_default().push(idx);
            }
        }
        
        let right_width = right.iter().map(|r| r.len()).max().unwrap_or(0);
        let empty_right: Vec<String> = vec![String::new(); right_width];
        
        let mut result = Vec::new();
        let mut matched_right: std::collections::HashSet<usize> = std::collections::HashSet::new();
        
        for left_row in left {
            let key = left_row.get(left_col).cloned().unwrap_or_default();
            
            if let Some(right_indices) = right_index.get(&key) {
                for &right_idx in right_indices {
                    matched_right.insert(right_idx);
                    let mut new_row = left_row.clone();
                    // Append right columns (excluding join column)
                    for (idx, val) in right[right_idx].iter().enumerate() {
                        if idx != right_col {
                            new_row.push(val.clone());
                        }
                    }
                    result.push(new_row);
                }
            } else if matches!(how, JoinType::Left | JoinType::Outer) {
                let mut new_row = left_row.clone();
                for (idx, val) in empty_right.iter().enumerate() {
                    if idx != right_col {
                        new_row.push(val.clone());
                    }
                }
                result.push(new_row);
            }
        }
        
        // For outer join, add unmatched right rows
        if matches!(how, JoinType::Right | JoinType::Outer) {
            let left_width = left.iter().map(|r| r.len()).max().unwrap_or(0);
            let empty_left: Vec<String> = vec![String::new(); left_width];
            
            for (idx, right_row) in right.iter().enumerate() {
                if !matched_right.contains(&idx) {
                    let mut new_row = empty_left.clone();
                    if let Some(key) = right_row.get(right_col) {
                        if left_col < new_row.len() {
                            new_row[left_col] = key.clone();
                        }
                    }
                    for (col_idx, val) in right_row.iter().enumerate() {
                        if col_idx != right_col {
                            new_row.push(val.clone());
                        }
                    }
                    result.push(new_row);
                }
            }
        }
        
        Ok(result)
    }
    
    /// Group by column and aggregate
    pub fn groupby(
        &self,
        data: &[Vec<String>],
        group_col: usize,
        aggregations: &[(usize, AggFunc)],
    ) -> Result<Vec<Vec<String>>> {
        use std::collections::HashMap;
        
        if data.is_empty() {
            return Ok(Vec::new());
        }
        
        // Group rows by key
        let mut groups: HashMap<String, Vec<&Vec<String>>> = HashMap::new();
        for row in data.iter().skip(1) { // Skip header
            let key = row.get(group_col).cloned().unwrap_or_default();
            groups.entry(key).or_default().push(row);
        }
        
        // Build result with header
        let mut result = Vec::new();
        
        // Header row
        let mut header = vec![data[0].get(group_col).cloned().unwrap_or_else(|| "group".to_string())];
        for (col, agg) in aggregations {
            let col_name = data[0].get(*col).cloned().unwrap_or_else(|| format!("col_{}", col));
            header.push(format!("{}_{}", agg.name(), col_name));
        }
        result.push(header);
        
        // Aggregate each group
        for (key, rows) in groups {
            let mut row = vec![key];
            for (col, agg) in aggregations {
                let values: Vec<f64> = rows.iter()
                    .filter_map(|r| r.get(*col))
                    .filter_map(|v| v.parse::<f64>().ok())
                    .collect();
                let agg_value = agg.apply(&values);
                row.push(format!("{:.2}", agg_value));
            }
            result.push(row);
        }
        
        Ok(result)
    }
    
    /// Calculate descriptive statistics for numeric columns
    pub fn describe(&self, data: &[Vec<String>]) -> Result<Vec<Vec<String>>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }
        
        let header = &data[0];
        let num_cols = header.len();
        
        // Collect numeric values per column
        let mut columns: Vec<Vec<f64>> = vec![Vec::new(); num_cols];
        for row in data.iter().skip(1) {
            for (idx, val) in row.iter().enumerate() {
                if let Ok(num) = val.parse::<f64>() {
                    columns[idx].push(num);
                }
            }
        }
        
        // Build result
        let mut result = Vec::new();
        
        // Header
        let mut stat_header = vec!["stat".to_string()];
        stat_header.extend(header.iter().cloned());
        result.push(stat_header);
        
        // Stats rows
        let stats = ["count", "mean", "std", "min", "25%", "50%", "75%", "max"];
        for stat in stats {
            let mut row = vec![stat.to_string()];
            for col_values in &columns {
                let value = if col_values.is_empty() {
                    "NaN".to_string()
                } else {
                    match stat {
                        "count" => col_values.len().to_string(),
                        "mean" => format!("{:.2}", col_values.iter().sum::<f64>() / col_values.len() as f64),
                        "std" => {
                            let mean = col_values.iter().sum::<f64>() / col_values.len() as f64;
                            let variance = col_values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / col_values.len() as f64;
                            format!("{:.2}", variance.sqrt())
                        }
                        "min" => format!("{:.2}", col_values.iter().cloned().fold(f64::INFINITY, f64::min)),
                        "max" => format!("{:.2}", col_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
                        "25%" | "50%" | "75%" => {
                            let mut sorted = col_values.clone();
                            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                            let p = match stat {
                                "25%" => 0.25,
                                "50%" => 0.50,
                                "75%" => 0.75,
                                _ => 0.5,
                            };
                            let idx = ((sorted.len() - 1) as f64 * p) as usize;
                            format!("{:.2}", sorted[idx])
                        }
                        _ => "".to_string(),
                    }
                };
                row.push(value);
            }
            result.push(row);
        }
        
        Ok(result)
    }
    
    /// Count unique values in a column
    pub fn value_counts(&self, data: &[Vec<String>], column: usize) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        
        let mut counts: HashMap<String, usize> = HashMap::new();
        for row in data.iter().skip(1) { // Skip header
            if let Some(val) = row.get(column) {
                *counts.entry(val.clone()).or_insert(0) += 1;
            }
        }
        
        let mut result: Vec<(String, usize)> = counts.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending
        
        let mut output = vec![vec!["value".to_string(), "count".to_string()]];
        for (val, count) in result {
            output.push(vec![val, count.to_string()]);
        }
        output
    }
}

/// Join type for merge operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Outer,
}

impl JoinType {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "inner" => Ok(JoinType::Inner),
            "left" => Ok(JoinType::Left),
            "right" => Ok(JoinType::Right),
            "outer" | "full" => Ok(JoinType::Outer),
            _ => anyhow::bail!("Unknown join type: {}. Use: inner, left, right, outer", s),
        }
    }
}

/// Aggregation functions for groupby
#[derive(Debug, Clone, Copy)]
pub enum AggFunc {
    Sum,
    Count,
    Mean,
    Min,
    Max,
}

impl AggFunc {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "sum" => Ok(AggFunc::Sum),
            "count" => Ok(AggFunc::Count),
            "mean" | "avg" | "average" => Ok(AggFunc::Mean),
            "min" => Ok(AggFunc::Min),
            "max" => Ok(AggFunc::Max),
            _ => anyhow::bail!("Unknown aggregation: {}. Use: sum, count, mean, min, max", s),
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            AggFunc::Sum => "sum",
            AggFunc::Count => "count",
            AggFunc::Mean => "mean",
            AggFunc::Min => "min",
            AggFunc::Max => "max",
        }
    }
    
    pub fn apply(&self, values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        match self {
            AggFunc::Sum => values.iter().sum(),
            AggFunc::Count => values.len() as f64,
            AggFunc::Mean => values.iter().sum::<f64>() / values.len() as f64,
            AggFunc::Min => values.iter().cloned().fold(f64::INFINITY, f64::min),
            AggFunc::Max => values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        }
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
