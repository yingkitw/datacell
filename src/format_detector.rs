//! Format detection for file types

use anyhow::Result;
use crate::traits::FormatDetector;

/// Default format detector implementation
pub struct DefaultFormatDetector;

impl DefaultFormatDetector {
    pub fn new() -> Self {
        Self
    }
}

impl FormatDetector for DefaultFormatDetector {
    fn detect_format(&self, path: &str) -> Result<String> {
        path.split('.')
            .last()
            .map(|s| s.to_lowercase())
            .ok_or_else(|| anyhow::anyhow!("No file extension found in: {}", path))
    }
    
    fn is_supported(&self, format: &str) -> bool {
        matches!(
            format.to_lowercase().as_str(),
            "csv" | "xlsx" | "xls" | "ods" | "parquet" | "avro"
        )
    }
    
    fn supported_formats(&self) -> Vec<String> {
        vec![
            "csv".to_string(),
            "xlsx".to_string(),
            "xls".to_string(),
            "ods".to_string(),
            "parquet".to_string(),
            "avro".to_string(),
        ]
    }
}

