//! Helper functions for common operations (DRY principle)

use crate::csv_handler::CellRange;

/// Filter data by cell range (used by multiple handlers)
pub fn filter_by_range(
    data: &[Vec<String>],
    range: &CellRange,
) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    
    for (row_idx, row) in data.iter().enumerate() {
        if row_idx < range.start_row {
            continue;
        }
        if row_idx > range.end_row {
            break;
        }
        
        let filtered_row: Vec<String> = row.iter()
            .enumerate()
            .filter(|(col_idx, _)| *col_idx >= range.start_col && *col_idx <= range.end_col)
            .map(|(_, val)| val.clone())
            .collect();
        result.push(filtered_row);
    }
    
    result
}

/// Get default column names if not provided
pub fn default_column_names(num_cols: usize, prefix: &str) -> Vec<String> {
    (0..num_cols)
        .map(|i| format!("{}_{}", prefix, i))
        .collect()
}

/// Get maximum column count from data
pub fn max_column_count(data: &[Vec<String>]) -> usize {
    data.iter().map(|r| r.len()).max().unwrap_or(0)
}

/// Check if a path matches any of the given extensions
pub fn matches_extension(path: &str, extensions: &[&str]) -> bool {
    let path_lower = path.to_lowercase();
    extensions.iter().any(|ext| path_lower.ends_with(&format!(".{}", ext)))
}

