//! Data profiling operations
//!
//! Provides comprehensive data profiling capabilities including
//! statistical analysis, data quality metrics, and insights.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::Datelike;
use crate::common::{string, collection};

/// Column profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnProfile {
    pub name: String,
    pub data_type: DataType,
    pub null_count: usize,
    pub null_percentage: f64,
    pub unique_count: usize,
    pub unique_percentage: f64,
    pub distinct_values: Vec<String>,
    pub top_values: Vec<ValueFrequency>,
    pub length_stats: Option<LengthStats>,
    pub numeric_stats: Option<NumericStats>,
    pub date_stats: Option<DateStats>,
    pub text_stats: Option<TextStats>,
    pub quality_score: f64,
}

/// Data type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
    DateTime,
    Email,
    Url,
    Phone,
    Unknown,
}

/// Value frequency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueFrequency {
    pub value: String,
    pub count: usize,
    pub percentage: f64,
}

/// Length statistics for text columns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LengthStats {
    pub min_length: usize,
    pub max_length: usize,
    pub avg_length: f64,
    pub median_length: usize,
    pub std_dev_length: f64,
}

/// Numeric statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub mode: Vec<String>,
    pub std_dev: f64,
    pub variance: f64,
    pub q1: f64,
    pub q3: f64,
    pub iqr: f64,
    pub skewness: f64,
    pub kurtosis: f64,
}

/// Date statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateStats {
    pub min_date: String,
    pub max_date: String,
    pub date_range_days: i64,
    pub most_common_year: u32,
    pub most_common_month: u32,
    pub most_common_day_of_week: String,
}

/// Text statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStats {
    pub avg_word_count: f64,
    pub max_word_count: usize,
    pub min_word_count: usize,
    pub contains_numbers: bool,
    pub contains_special_chars: bool,
    pub all_uppercase: usize,
    pub all_lowercase: usize,
    pub title_case: usize,
    pub mixed_case: usize,
}

/// Overall data profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProfile {
    pub file_path: String,
    pub total_rows: usize,
    pub total_columns: usize,
    pub total_cells: usize,
    pub null_cells: usize,
    pub null_percentage: f64,
    pub duplicate_rows: usize,
    pub duplicate_percentage: f64,
    pub columns: Vec<ColumnProfile>,
    pub data_quality_score: f64,
    pub recommendations: Vec<String>,
    pub profiling_timestamp: String,
}

/// Data profiler
pub struct DataProfiler {
    max_distinct_values: usize,
    sample_size: Option<usize>,
}

impl DataProfiler {
    /// Create a new profiler with options
    pub fn new() -> Self {
        Self {
            max_distinct_values: 100,
            sample_size: None,
        }
    }
    
    /// Set maximum distinct values to track
    pub fn with_max_distinct_values(mut self, max: usize) -> Self {
        self.max_distinct_values = max;
        self
    }
    
    /// Set sample size for large datasets
    pub fn with_sample_size(mut self, size: usize) -> Self {
        self.sample_size = Some(size);
        self
    }
    
    /// Profile data from rows
    pub fn profile(&self, data: &[Vec<String>], file_path: &str) -> Result<DataProfile> {
        if data.is_empty() {
            return Ok(DataProfile {
                file_path: file_path.to_string(),
                total_rows: 0,
                total_columns: 0,
                total_cells: 0,
                null_cells: 0,
                null_percentage: 0.0,
                duplicate_rows: 0,
                duplicate_percentage: 0.0,
                columns: Vec::new(),
                data_quality_score: 0.0,
                recommendations: Vec::new(),
                profiling_timestamp: chrono::Utc::now().to_rfc3339(),
            });
        }
        
        let header = &data[0];
        let total_rows = data.len() - 1;
        let total_columns = header.len();
        let total_cells = total_rows * total_columns;
        
        // Sample data if needed
        let data_to_profile = if let Some(sample_size) = self.sample_size {
            if total_rows > sample_size {
                let mut sampled = vec![header.clone()];
                let step = total_rows / sample_size;
                for i in (1..=total_rows).step_by(step.max(1)) {
                    if i < data.len() {
                        sampled.push(data[i].clone());
                    }
                }
                sampled
            } else {
                data.to_vec()
            }
        } else {
            data.to_vec()
        };
        
        // Profile each column
        let mut columns = Vec::new();
        let mut null_cells = 0;
        
        for (col_idx, col_name) in header.iter().enumerate() {
            let column_data: Vec<String> = data_to_profile
                .iter()
                .skip(1)
                .filter_map(|row| row.get(col_idx).cloned())
                .collect();
            
            let column_profile = self.profile_column(col_name, &column_data, total_rows)?;
            null_cells += column_profile.null_count;
            columns.push(column_profile);
        }
        
        // Calculate duplicates
        let duplicate_rows = self.count_duplicate_rows(&data_to_profile[1..]);
        let duplicate_percentage = (duplicate_rows as f64 / total_rows as f64) * 100.0;
        let null_percentage = (null_cells as f64 / total_cells as f64) * 100.0;
        
        // Calculate overall quality score
        let data_quality_score = self.calculate_overall_quality_score(&columns, null_percentage, duplicate_percentage);
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&columns, null_percentage, duplicate_percentage);
        
        Ok(DataProfile {
            file_path: file_path.to_string(),
            total_rows,
            total_columns,
            total_cells,
            null_cells,
            null_percentage,
            duplicate_rows,
            duplicate_percentage,
            columns,
            data_quality_score,
            recommendations,
            profiling_timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }
    
    /// Profile a single column
    fn profile_column(&self, name: &str, data: &[String], total_rows: usize) -> Result<ColumnProfile> {
        let null_count = data.iter().filter(|v| string::is_empty_or_whitespace(v)).count();
        let null_percentage = (null_count as f64 / total_rows as f64) * 100.0;
        
        // Get distinct values
        let distinct_values: Vec<String> = collection::unique_preserve_order(
            &data.iter()
                .filter(|v| !string::is_empty_or_whitespace(v))
                .cloned()
                .collect::<Vec<_>>(),
        );
        
        let unique_count = distinct_values.len();
        let unique_percentage = (unique_count as f64 / total_rows as f64) * 100.0;
        
        // Get top values
        let top_values = self.get_value_frequencies(data);
        
        // Determine data type
        let data_type = self.infer_data_type(data);
        
        // Calculate type-specific statistics
        let length_stats = if matches!(data_type, DataType::String | DataType::Email | DataType::Url | DataType::Phone) {
            Some(self.calculate_length_stats(data))
        } else {
            None
        };
        
        let numeric_stats = if matches!(data_type, DataType::Integer | DataType::Float) {
            self.calculate_numeric_stats(data)
        } else {
            None
        };
        
        let date_stats = if matches!(data_type, DataType::Date | DataType::DateTime) {
            self.calculate_date_stats(data)
        } else {
            None
        };
        
        let text_stats = if matches!(data_type, DataType::String) {
            Some(self.calculate_text_stats(data))
        } else {
            None
        };
        
        // Calculate quality score for this column
        let quality_score = self.calculate_column_quality_score(
            null_percentage,
            unique_percentage,
            &data_type,
            length_stats.as_ref(),
            numeric_stats.as_ref(),
        );
        
        Ok(ColumnProfile {
            name: name.to_string(),
            data_type,
            null_count,
            null_percentage,
            unique_count,
            unique_percentage,
            distinct_values: distinct_values.into_iter().take(self.max_distinct_values).collect(),
            top_values,
            length_stats,
            numeric_stats,
            date_stats,
            text_stats,
            quality_score,
        })
    }
    
    /// Infer data type from sample values
    fn infer_data_type(&self, data: &[String]) -> DataType {
        let non_null_values: Vec<&str> = data
            .iter()
            .filter(|v| !string::is_empty_or_whitespace(v))
            .map(|v| v.as_str())
            .collect();
        
        if non_null_values.is_empty() {
            return DataType::Unknown;
        }
        
        let sample_size = non_null_values.len().min(100);
        let sample = &non_null_values[..sample_size];
        
        // Check for boolean
        let boolean_count = sample
            .iter()
            .filter(|v| {
                matches!(v.to_lowercase().as_str(), "true" | "false" | "1" | "0" | "yes" | "no")
            })
            .count();
        
        if boolean_count as f64 / sample_size as f64 > 0.8 {
            return DataType::Boolean;
        }
        
        // Check for email
        let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        let email_count = sample.iter().filter(|v| email_regex.is_match(v)).count();
        
        if email_count as f64 / sample_size as f64 > 0.8 {
            return DataType::Email;
        }
        
        // Check for URL
        let url_regex = regex::Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
        let url_count = sample.iter().filter(|v| url_regex.is_match(v)).count();
        
        if url_count as f64 / sample_size as f64 > 0.8 {
            return DataType::Url;
        }
        
        // Check for phone
        let phone_regex = regex::Regex::new(r"^\+?[\d\s\-\(\)]{10,}$").unwrap();
        let phone_count = sample.iter().filter(|v| phone_regex.is_match(v)).count();
        
        if phone_count as f64 / sample_size as f64 > 0.8 {
            return DataType::Phone;
        }
        
        // Check for date/datetime
        let date_formats = vec![
            "%Y-%m-%d",
            "%d/%m/%Y",
            "%m/%d/%Y",
            "%Y-%m-%d %H:%M:%S",
            "%d/%m/%Y %H:%M:%S",
        ];
        
        for format in &date_formats {
            let date_count = sample
                .iter()
                .filter(|v| chrono::NaiveDate::parse_from_str(v, format).is_ok() || 
                           chrono::NaiveDateTime::parse_from_str(v, format).is_ok())
                .count();
            
            if date_count as f64 / sample_size as f64 > 0.8 {
                return if format.contains("%H") {
                    DataType::DateTime
                } else {
                    DataType::Date
                };
            }
        }
        
        // Check for numeric
        let numeric_count = sample.iter().filter(|v| string::is_numeric(v)).count();
        
        if numeric_count as f64 / sample_size as f64 > 0.8 {
            // Check if all are integers
            let int_count = sample
                .iter()
                .filter(|v| v.parse::<i64>().is_ok())
                .count();
            
            return if int_count as f64 / numeric_count as f64 > 0.8 {
                DataType::Integer
            } else {
                DataType::Float
            };
        }
        
        DataType::String
    }
    
    /// Get value frequencies
    fn get_value_frequencies(&self, data: &[String]) -> Vec<ValueFrequency> {
        let mut frequency_map: HashMap<String, usize> = HashMap::new();
        let total = data.len();
        
        for value in data {
            if !string::is_empty_or_whitespace(value) {
                *frequency_map.entry(value.clone()).or_insert(0) += 1;
            }
        }
        
        let mut frequencies: Vec<ValueFrequency> = frequency_map
            .into_iter()
            .map(|(value, count)| ValueFrequency {
                value,
                count,
                percentage: (count as f64 / total as f64) * 100.0,
            })
            .collect();
        
        // Sort by count (descending), then by value (ascending) for deterministic ordering
        frequencies.sort_by(|a, b| {
            match b.count.cmp(&a.count) {
                std::cmp::Ordering::Equal => a.value.cmp(&b.value),
                other => other,
            }
        });
        frequencies.truncate(10); // Top 10 values
        frequencies
    }
    
    /// Calculate length statistics
    fn calculate_length_stats(&self, data: &[String]) -> LengthStats {
        let lengths: Vec<usize> = data
            .iter()
            .filter(|v| !string::is_empty_or_whitespace(v))
            .map(|v| v.len())
            .collect();
        
        if lengths.is_empty() {
            return LengthStats {
                min_length: 0,
                max_length: 0,
                avg_length: 0.0,
                median_length: 0,
                std_dev_length: 0.0,
            };
        }
        
        let min_length = *lengths.iter().min().unwrap();
        let max_length = *lengths.iter().max().unwrap();
        let avg_length = lengths.iter().sum::<usize>() as f64 / lengths.len() as f64;
        
        let mut sorted_lengths = lengths.clone();
        sorted_lengths.sort_unstable();
        let median_length = if sorted_lengths.len() % 2 == 0 {
            let mid = sorted_lengths.len() / 2;
            (sorted_lengths[mid - 1] + sorted_lengths[mid]) / 2
        } else {
            sorted_lengths[sorted_lengths.len() / 2]
        };
        
        let variance = lengths
            .iter()
            .map(|&len| (len as f64 - avg_length).powi(2))
            .sum::<f64>() / lengths.len() as f64;
        let std_dev_length = variance.sqrt();
        
        LengthStats {
            min_length,
            max_length,
            avg_length,
            median_length,
            std_dev_length,
        }
    }
    
    /// Calculate numeric statistics
    fn calculate_numeric_stats(&self, data: &[String]) -> Option<NumericStats> {
        let numbers: Vec<f64> = data
            .iter()
            .filter(|v| !string::is_empty_or_whitespace(v))
            .filter_map(|v| string::to_number(v))
            .collect();
        
        if numbers.is_empty() {
            return None;
        }
        
        let min = numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
        
        let mut sorted_numbers = numbers.clone();
        sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if sorted_numbers.len() % 2 == 0 {
            let mid = sorted_numbers.len() / 2;
            (sorted_numbers[mid - 1] + sorted_numbers[mid]) / 2.0
        } else {
            sorted_numbers[sorted_numbers.len() / 2]
        };
        
        // Calculate mode
        let mut frequency_map: HashMap<String, usize> = HashMap::new();
        for &num in &numbers {
            let key = num.to_string();
            *frequency_map.entry(key).or_insert(0) += 1;
        }
        
        let frequency_map_clone = frequency_map.clone();
        let max_freq = frequency_map_clone.values().max().unwrap();
        let mode: Vec<String> = frequency_map
            .into_iter()
            .filter(|(_, freq)| *freq == *max_freq)
            .map(|(num, _)| num)
            .collect();
        
        let variance = numbers
            .iter()
            .map(|&num| (num - mean).powi(2))
            .sum::<f64>() / numbers.len() as f64;
        let std_dev = variance.sqrt();
        
        // Quartiles
        let q1_idx = numbers.len() / 4;
        let q3_idx = (numbers.len() * 3) / 4;
        let q1 = sorted_numbers[q1_idx];
        let q3 = sorted_numbers[q3_idx];
        let iqr = q3 - q1;
        
        // Skewness and kurtosis (simplified calculations)
        let skewness = if std_dev > 0.0 {
            numbers
                .iter()
                .map(|&num| ((num - mean) / std_dev).powi(3))
                .sum::<f64>() / numbers.len() as f64
        } else {
            0.0
        };
        
        let kurtosis = if std_dev > 0.0 {
            numbers
                .iter()
                .map(|&num| ((num - mean) / std_dev).powi(4))
                .sum::<f64>() / numbers.len() as f64
                - 3.0
        } else {
            0.0
        };
        
        Some(NumericStats {
            min,
            max,
            mean,
            median,
            mode,
            std_dev,
            variance,
            q1,
            q3,
            iqr,
            skewness,
            kurtosis,
        })
    }
    
    /// Calculate date statistics
    fn calculate_date_stats(&self, data: &[String]) -> Option<DateStats> {
        let dates: Vec<chrono::NaiveDate> = data
            .iter()
            .filter(|v| !string::is_empty_or_whitespace(v))
            .filter_map(|v| {
                // Try multiple date formats
                let formats = vec!["%Y-%m-%d", "%d/%m/%Y", "%m/%d/%Y"];
                for format in &formats {
                    if let Ok(date) = chrono::NaiveDate::parse_from_str(v, format) {
                        return Some(date);
                    }
                }
                None
            })
            .collect();
        
        if dates.is_empty() {
            return None;
        }
        
        let min_date = dates.iter().min().unwrap();
        let max_date = dates.iter().max().unwrap();
        let date_range_days = (*max_date - *min_date).num_days();
        
        // Most common year, month, day of week
        let mut year_counts: HashMap<u32, usize> = HashMap::new();
        let mut month_counts: HashMap<u32, usize> = HashMap::new();
        let mut dow_counts: HashMap<String, usize> = HashMap::new();
        
        for date in &dates {
            *year_counts.entry(date.year() as u32).or_insert(0) += 1;
            *month_counts.entry(date.month()).or_insert(0) += 1;
            *dow_counts.entry(date.weekday().to_string()).or_insert(0) += 1;
        }
        
        let most_common_year = *year_counts
            .iter()
            .max_by_key(|&(_, &count)| count)
            .unwrap()
            .0;
        
        let most_common_month = *month_counts
            .iter()
            .max_by_key(|&(_, &count)| count)
            .unwrap()
            .0;
        
        let most_common_day_of_week = dow_counts
            .iter()
            .max_by_key(|&(_, &count)| count)
            .unwrap()
            .0
            .clone();
        
        Some(DateStats {
            min_date: min_date.to_string(),
            max_date: max_date.to_string(),
            date_range_days,
            most_common_year,
            most_common_month,
            most_common_day_of_week,
        })
    }
    
    /// Calculate text statistics
    fn calculate_text_stats(&self, data: &[String]) -> TextStats {
        let non_empty: Vec<&str> = data
            .iter()
            .filter(|v| !string::is_empty_or_whitespace(v))
            .map(|v| v.as_str())
            .collect();
        
        if non_empty.is_empty() {
            return TextStats {
                avg_word_count: 0.0,
                max_word_count: 0,
                min_word_count: 0,
                contains_numbers: false,
                contains_special_chars: false,
                all_uppercase: 0,
                all_lowercase: 0,
                title_case: 0,
                mixed_case: 0,
            };
        }
        
        let word_counts: Vec<usize> = non_empty
            .iter()
            .map(|v| v.split_whitespace().count())
            .collect();
        
        let avg_word_count = word_counts.iter().sum::<usize>() as f64 / word_counts.len() as f64;
        let max_word_count = *word_counts.iter().max().unwrap();
        let min_word_count = *word_counts.iter().min().unwrap();
        
        let contains_numbers = non_empty.iter().any(|v| v.chars().any(|c| c.is_ascii_digit()));
        let contains_special_chars = non_empty
            .iter()
            .any(|v| v.chars().any(|c| !c.is_alphanumeric() && !c.is_whitespace()));
        
        let mut all_uppercase = 0;
        let mut all_lowercase = 0;
        let mut title_case = 0;
        let mut mixed_case = 0;
        
        for text in &non_empty {
            if text.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
                all_uppercase += 1;
            } else if text.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
                all_lowercase += 1;
            } else if text
                .chars()
                .enumerate()
                .all(|(i, c)| (i == 0 && c.is_uppercase()) || (i > 0 && c.is_lowercase()) || !c.is_alphabetic())
            {
                title_case += 1;
            } else {
                mixed_case += 1;
            }
        }
        
        TextStats {
            avg_word_count,
            max_word_count,
            min_word_count,
            contains_numbers,
            contains_special_chars,
            all_uppercase,
            all_lowercase,
            title_case,
            mixed_case,
        }
    }
    
    /// Count duplicate rows
    fn count_duplicate_rows(&self, data: &[Vec<String>]) -> usize {
        let mut seen: HashSet<String> = HashSet::new();
        let mut duplicates = 0;
        
        for row in data {
            let row_str = row.join("|");
            if seen.contains(&row_str) {
                duplicates += 1;
            } else {
                seen.insert(row_str);
            }
        }
        
        duplicates
    }
    
    /// Calculate column quality score
    fn calculate_column_quality_score(
        &self,
        null_percentage: f64,
        unique_percentage: f64,
        data_type: &DataType,
        length_stats: Option<&LengthStats>,
        numeric_stats: Option<&NumericStats>,
    ) -> f64 {
        let mut score = 100.0;
        
        // Penalize null values
        score -= null_percentage * 0.5;
        
        // Penalize too many unique values for categorical data
        if matches!(data_type, DataType::String | DataType::Email | DataType::Url | DataType::Phone) {
            if unique_percentage > 80.0 {
                score -= (unique_percentage - 80.0) * 0.2;
            }
        }
        
        // Check for consistent lengths (good for structured data)
        if let Some(length_stats) = length_stats {
            let length_variance = length_stats.std_dev_length / length_stats.avg_length;
            if length_variance < 0.1 {
                score += 5.0; // Bonus for consistent lengths
            }
        }
        
        // Check for reasonable numeric distributions
        if let Some(numeric_stats) = numeric_stats {
            // Penalize extreme skewness
            if numeric_stats.skewness.abs() > 2.0 {
                score -= 5.0;
            }
            
            // Bonus for reasonable variance
            if numeric_stats.std_dev > 0.0 && numeric_stats.std_dev < numeric_stats.mean * 2.0 {
                score += 5.0;
            }
        }
        
        score.max(0.0).min(100.0)
    }
    
    /// Calculate overall quality score
    fn calculate_overall_quality_score(&self, columns: &[ColumnProfile], null_percentage: f64, duplicate_percentage: f64) -> f64 {
        let column_scores: f64 = columns.iter().map(|c| c.quality_score).sum();
        let avg_column_score = column_scores / columns.len() as f64;
        
        let mut overall_score = avg_column_score;
        
        // Penalize high null percentage
        overall_score -= null_percentage * 0.3;
        
        // Penalize high duplicate percentage
        overall_score -= duplicate_percentage * 0.2;
        
        overall_score.max(0.0).min(100.0)
    }
    
    /// Generate recommendations
    fn generate_recommendations(&self, columns: &[ColumnProfile], null_percentage: f64, duplicate_percentage: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if null_percentage > 20.0 {
            recommendations.push(format!(
                "High null percentage ({:.1}%). Consider data imputation or cleaning.",
                null_percentage
            ));
        }
        
        if duplicate_percentage > 10.0 {
            recommendations.push(format!(
                "High duplicate percentage ({:.1}%). Consider deduplication.",
                duplicate_percentage
            ));
        }
        
        for column in columns {
            if column.null_percentage > 30.0 {
                recommendations.push(format!(
                    "Column '{}' has high null percentage ({:.1}%).",
                    column.name, column.null_percentage
                ));
            }
            
            if matches!(column.data_type, DataType::String) && column.unique_percentage > 90.0 {
                recommendations.push(format!(
                    "Column '{}' might be an identifier with {} unique values.",
                    column.name, column.unique_count
                ));
            }
            
            if let Some(numeric_stats) = &column.numeric_stats {
                if numeric_stats.skewness.abs() > 2.0 {
                    recommendations.push(format!(
                        "Column '{}' is highly skewed (skewness: {:.2}). Consider transformation.",
                        column.name, numeric_stats.skewness
                    ));
                }
            }
        }
        
        if recommendations.is_empty() {
            recommendations.push("Data quality looks good. No major issues detected.".to_string());
        }
        
        recommendations
    }
    
    /// Generate profiling report
    pub fn generate_report(&self, profile: &DataProfile) -> String {
        let mut report = String::new();
        
        report.push_str("# Data Profiling Report\n\n");
        
        // Overview
        report.push_str("## Overview\n\n");
        report.push_str(&format!(
            "- **File**: {}\n\
             - **Rows**: {}\n\
             - **Columns**: {}\n\
             - **Data Quality Score**: {:.1}/100\n\
             - **Null Percentage**: {:.1}%\n\
             - **Duplicate Percentage**: {:.1}%\n\
             - **Profiling Date**: {}\n\n",
            profile.file_path,
            profile.total_rows,
            profile.total_columns,
            profile.data_quality_score,
            profile.null_percentage,
            profile.duplicate_percentage,
            profile.profiling_timestamp
        ));
        
        // Recommendations
        report.push_str("## Recommendations\n\n");
        for recommendation in &profile.recommendations {
            report.push_str(&format!("- {}\n", recommendation));
        }
        report.push('\n');
        
        // Column details
        report.push_str("## Column Details\n\n");
        for column in &profile.columns {
            report.push_str(&format!(
                "### {}\n\n\
                 - **Type**: {:?}\n\
                 - **Quality Score**: {:.1}/100\n\
                 - **Null Count**: {} ({:.1}%)\n\
                 - **Unique Count**: {} ({:.1}%)\n",
                column.name,
                column.data_type,
                column.quality_score,
                column.null_count,
                column.null_percentage,
                column.unique_count,
                column.unique_percentage
            ));
            
            if !column.top_values.is_empty() {
                report.push_str("- **Top Values**:\n");
                for (i, val) in column.top_values.iter().take(5).enumerate() {
                    report.push_str(&format!(
                        "  {}. {} ({}%, {} occurrences)\n",
                        i + 1,
                        val.value,
                        val.percentage,
                        val.count
                    ));
                }
            }
            
            if let Some(numeric_stats) = &column.numeric_stats {
                report.push_str(&format!(
                    "- **Numeric Stats**: Min={}, Max={}, Mean={:.2}, Median={:.2}, StdDev={:.2}\n",
                    numeric_stats.min,
                    numeric_stats.max,
                    numeric_stats.mean,
                    numeric_stats.median,
                    numeric_stats.std_dev
                ));
            }
            
            if let Some(length_stats) = &column.length_stats {
                report.push_str(&format!(
                    "- **Length Stats**: Min={}, Max={}, Avg={:.1}, Median={}\n",
                    length_stats.min_length,
                    length_stats.max_length,
                    length_stats.avg_length,
                    length_stats.median_length
                ));
            }
            
            report.push('\n');
        }
        
        report
    }
    
    /// Save profile to file
    pub fn save_profile(&self, profile: &DataProfile, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(profile)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

impl Default for DataProfiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_data_type_inference() {
        let profiler = DataProfiler::new();
        
        // Test numeric
        let numeric_data: Vec<String> = vec!["1", "2", "3", "4.5", "-1.2"].into_iter().map(|s| s.to_string()).collect();
        assert!(matches!(
            profiler.infer_data_type(&numeric_data),
            DataType::Float
        ));
        
        // Test integer
        let int_data: Vec<String> = vec!["1", "2", "3", "4", "-1"].into_iter().map(|s| s.to_string()).collect();
        assert!(matches!(
            profiler.infer_data_type(&int_data),
            DataType::Integer
        ));
        
        // Test email
        let email_data: Vec<String> = vec!["test@example.com", "user@domain.org"].into_iter().map(|s| s.to_string()).collect();
        assert!(matches!(
            profiler.infer_data_type(&email_data),
            DataType::Email
        ));
        
        // Test boolean
        let bool_data: Vec<String> = vec!["true", "false", "yes", "no"].into_iter().map(|s| s.to_string()).collect();
        assert!(matches!(
            profiler.infer_data_type(&bool_data),
            DataType::Boolean
        ));
    }
    
    #[test]
    fn test_value_frequencies() {
        let profiler = DataProfiler::new();
        let data = vec![
            "apple".to_string(),
            "banana".to_string(),
            "apple".to_string(),
            "orange".to_string(),
            "apple".to_string(),
        ];
        
        let frequencies = profiler.get_value_frequencies(&data);
        assert_eq!(frequencies.len(), 3);
        assert_eq!(frequencies[0].value, "apple");
        assert_eq!(frequencies[0].count, 3);
        assert_eq!(frequencies[1].value, "banana");
        assert_eq!(frequencies[1].count, 1);
    }
}
