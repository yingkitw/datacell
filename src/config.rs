//! Configuration file support for datacell
//!
//! Supports loading default options from ~/.datacell.toml or .datacell.toml

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for datacell CLI
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Default output format (csv, json, markdown)
    #[serde(default)]
    pub default_format: Option<String>,
    
    /// Default date format for parsing
    #[serde(default)]
    pub date_format: Option<String>,
    
    /// Default output directory for batch operations
    #[serde(default)]
    pub output_dir: Option<String>,
    
    /// Excel styling options
    #[serde(default)]
    pub excel: ExcelConfig,
    
    /// CSV options
    #[serde(default)]
    pub csv: CsvConfig,
}

/// Excel-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExcelConfig {
    /// Default header style
    #[serde(default)]
    pub header_bold: Option<bool>,
    
    /// Header background color (hex like "4472C4")
    #[serde(default)]
    pub header_bg_color: Option<String>,
    
    /// Header font color (hex)
    #[serde(default)]
    pub header_font_color: Option<String>,
    
    /// Enable auto-filter on headers
    #[serde(default)]
    pub auto_filter: Option<bool>,
    
    /// Freeze first row
    #[serde(default)]
    pub freeze_header: Option<bool>,
    
    /// Auto-fit column widths
    #[serde(default)]
    pub auto_fit: Option<bool>,
}

/// CSV-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CsvConfig {
    /// Delimiter character
    #[serde(default)]
    pub delimiter: Option<String>,
    
    /// Quote character
    #[serde(default)]
    pub quote: Option<String>,
    
    /// Has header row
    #[serde(default)]
    pub has_header: Option<bool>,
}

impl Config {
    /// Load configuration from default locations
    pub fn load() -> Result<Self> {
        // Try loading from multiple locations in order
        let paths = vec![
            // Current directory
            PathBuf::from(".datacell.toml"),
            // Home directory
            dirs::home_dir().map(|p| p.join(".datacell.toml")).unwrap_or_default(),
            // XDG config
            dirs::config_dir().map(|p| p.join("datacell/config.toml")).unwrap_or_default(),
        ];
        
        for path in paths {
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                let config: Config = toml::from_str(&content)?;
                return Ok(config);
            }
        }
        
        // Return default config if no file found
        Ok(Config::default())
    }
    
    /// Load configuration from a specific path
    pub fn load_from(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// Save configuration to a file
    pub fn save(&self, path: &str) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Generate a default config file content
    pub fn default_config_content() -> &'static str {
        r#"# datacell configuration file
# Place this file at ~/.datacell.toml or .datacell.toml in your project

# Default output format: csv, json, markdown
default_format = "csv"

# Default date format for parsing
date_format = "%Y-%m-%d"

# Default output directory for batch operations
# output_dir = "output"

[excel]
# Make header row bold
header_bold = true

# Header background color (hex without #)
header_bg_color = "4472C4"

# Header font color (hex without #)
header_font_color = "FFFFFF"

# Enable auto-filter on headers
auto_filter = true

# Freeze first row (header)
freeze_header = true

# Auto-fit column widths
auto_fit = true

[csv]
# Delimiter character (default: comma)
delimiter = ","

# Quote character (default: double quote)
quote = "\""

# Has header row
has_header = true
"#
    }
}
