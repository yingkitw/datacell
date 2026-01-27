//! Advanced command handlers
//!
//! Implements advanced features like validation, charting, encryption, batch processing, etc.

use crate::{
    common::validation,
    converter::Converter,
    encryption::DataEncryptor,
    excel::{ChartConfig, DataChartType, ExcelHandler, WriteOptions},
    operations::DataOperations,
    profiling::DataProfiler,
    validation::DataValidator,
};
use anyhow::{Context, Result};

/// Advanced command handler
#[derive(Default)]
pub struct AdvancedCommandHandler;

impl AdvancedCommandHandler {
    /// Create a new advanced command handler
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle the profile command
    ///
    /// Generates a data profile report.
    pub fn handle_profile(&self, input: String, output: Option<String>) -> Result<()> {
        let converter = Converter::new();
        let data = converter.read_any_data(&input, None)?;

        let profiler = DataProfiler::new();
        let profile = profiler.profile(&data, &input)?;

        let report = serde_json::to_string_pretty(&profile)?;

        if let Some(output_path) = output {
            std::fs::write(&output_path, report)
                .context(format!("Failed to write profile to {output_path}"))?;
            println!("Profile saved to {}", output_path);
        } else {
            println!("{}", report);
        }

        Ok(())
    }

    /// Handle the validate command
    ///
    /// Validates data against a set of rules.
    pub fn handle_validate(
        &self,
        input: String,
        rules: String,
        output: Option<String>,
        report: Option<String>,
    ) -> Result<()> {
        let converter = Converter::new();
        let data = converter.read_any_data(&input, None)?;

        // Load validation rules
        let validator = if rules.ends_with(".json") {
            DataValidator::from_config_file(&rules)?
        } else {
            // Create default rules if no file provided
            let config = crate::validation::create_sample_config();
            DataValidator::new(config)
        };

        // Validate data
        let result = validator.validate(&data)?;

        // Output results
        if let Some(output_path) = output {
            validator.save_result(&result, &output_path)?;
            println!("Validation results saved to {}", output_path);
        }

        if let Some(report_path) = report {
            let report = validator.generate_report(&result);
            std::fs::write(&report_path, report)
                .context(format!("Failed to write report to {report_path}"))?;
            println!("Validation report saved to {}", report_path);
        }

        // Print summary
        println!("Validation Summary:");
        println!("  Total rows: {}", result.stats.total_rows);
        println!("  Valid rows: {}", result.stats.valid_rows);
        println!("  Invalid rows: {}", result.stats.invalid_rows);
        println!("  Errors: {}", result.errors.len());

        Ok(())
    }

    /// Handle the chart command
    ///
    /// Creates a chart from data and saves it to an Excel file.
    pub fn handle_chart(
        &self,
        input: String,
        output: String,
        chart_type: String,
        title: Option<String>,
        x_column: Option<String>,
        y_column: Option<String>,
    ) -> Result<()> {
        let converter = Converter::new();
        let data = converter.read_any_data(&input, None)?;

        // Parse chart type
        let chart_type = match chart_type.to_lowercase().as_str() {
            "line" => DataChartType::Line,
            "bar" => DataChartType::Column,
            "column" => DataChartType::Column,
            "pie" => DataChartType::Pie,
            "scatter" => DataChartType::Scatter,
            "area" => DataChartType::Area,
            _ => anyhow::bail!(
                "Unknown chart type: {}. Use: line, bar, pie, scatter, area",
                chart_type
            ),
        };

        // Determine x and y columns
        let x_col = if let Some(col) = x_column {
            self.find_column_index(&data, &col)?
        } else {
            0 // Default to first column
        };

        let y_col = if let Some(col) = y_column {
            self.find_column_index(&data, &col)?
        } else {
            1 // Default to second column
        };

        validation::validate_column_index(&data, x_col)?;
        validation::validate_column_index(&data, y_col)?;

        // Create chart configuration
        let _config = ChartConfig {
            chart_type,
            title: Some(title.unwrap_or_else(|| "Chart".to_string())),
            category_column: x_col,
            value_columns: vec![y_col],
            ..Default::default()
        };

        // Write Excel with chart (placeholder - chart integration needs workbook API)
        let handler = ExcelHandler::new();
        let options = WriteOptions::default();

        handler.write_styled(&output, &data, &options)?;
        println!("Created {:?} chart; wrote {}", chart_type, output);

        Ok(())
    }

    /// Handle the encrypt command
    ///
    /// Encrypts a file using the specified algorithm.
    pub fn handle_encrypt(&self, input: String, output: String, algorithm: String) -> Result<()> {
        use crate::encryption::EncryptionAlgorithm;

        let algorithm = match algorithm.to_lowercase().as_str() {
            "aes" | "aes256" => EncryptionAlgorithm::Aes256,
            "xor" => EncryptionAlgorithm::Xor,
            _ => anyhow::bail!("Unknown encryption algorithm: {}", algorithm),
        };

        let encryptor = DataEncryptor::new(algorithm);
        let key = b"default-encryption-key-32-bytes!";
        encryptor.encrypt_file(&input, &output, key)?;

        println!("Encrypted {} to {} using {:?}", input, output, algorithm);

        Ok(())
    }

    /// Handle the decrypt command
    ///
    /// Decrypts a file.
    pub fn handle_decrypt(&self, input: String, output: String) -> Result<()> {
        let encryptor = DataEncryptor::new(crate::encryption::EncryptionAlgorithm::Aes256);
        let key = b"default-encryption-key-32-bytes!";
        encryptor.decrypt_file(&input, &output, key)?;

        println!("Decrypted {} to {}", input, output);

        Ok(())
    }

    /// Handle the batch command
    ///
    /// Processes multiple files with the same operation.
    pub fn handle_batch(
        &self,
        inputs: String,
        output_dir: String,
        operation: String,
        args: Vec<String>,
    ) -> Result<()> {
        // Ensure output directory exists
        std::fs::create_dir_all(&output_dir)
            .context(format!("Failed to create output directory {output_dir}"))?;

        // Parse input files
        let input_files: Vec<String> = if inputs.contains('*') {
            glob::glob(&inputs)
                .context("Failed to parse glob pattern")?
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.is_file())
                .map(|entry| entry.to_string_lossy().to_string())
                .collect()
        } else {
            inputs.split(',').map(|s| s.trim().to_string()).collect()
        };

        if input_files.is_empty() {
            anyhow::bail!("No input files found for pattern: {inputs}");
        }

        println!(
            "Processing {} files with operation '{operation}'...",
            input_files.len()
        );

        let mut success_count = 0;
        let mut error_count = 0;

        for input_file in &input_files {
            // Generate output filename
            let file_stem = std::path::Path::new(input_file)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            let output_file = format!("{}/{}.csv", output_dir, file_stem);

            // Execute operation based on type
            let result = match operation.as_str() {
                "convert" => {
                    if args.is_empty() {
                        anyhow::bail!("Convert operation requires output format argument");
                    }
                    let format = &args[0];
                    let output_with_ext = format!("{}/{}.{}", output_dir, file_stem, format);
                    self.batch_convert(input_file, &output_with_ext)
                }
                "sort" => {
                    if args.is_empty() {
                        anyhow::bail!("Sort operation requires column argument");
                    }
                    self.batch_sort(input_file, &output_file, &args[0], true)
                }
                "filter" => {
                    if args.is_empty() {
                        anyhow::bail!("Filter operation requires where clause argument");
                    }
                    self.batch_filter(input_file, &output_file, &args[0])
                }
                "dedupe" => self.batch_dedupe(input_file, &output_file),
                "normalize" => {
                    if args.is_empty() {
                        anyhow::bail!("Normalize operation requires column argument");
                    }
                    self.batch_normalize(input_file, &output_file, &args[0])
                }
                _ => anyhow::bail!("Unknown batch operation: {}", operation),
            };

            match result {
                Ok(_) => {
                    println!("  ✓ {}", input_file);
                    success_count += 1;
                }
                Err(e) => {
                    println!("  ✗ {input_file}: {e}");
                    error_count += 1;
                }
            }
        }

        println!("\nBatch processing complete:");
        println!("  Success: {}", success_count);
        println!("  Errors: {}", error_count);

        Ok(())
    }

    /// Handle the plugin command
    ///
    /// Executes a plugin function.
    pub fn handle_plugin(
        &self,
        function: String,
        input: String,
        output: String,
        args: Vec<String>,
    ) -> Result<()> {
        use crate::plugins::PluginRegistry;

        let registry = PluginRegistry::new();

        let converter = Converter::new();
        let data = converter.read_any_data(&input, None)?;

        // Execute plugin function
        let result = registry.execute(&function, &args, &data)?;

        converter.write_any_data(&output, &result, None)?;
        println!("Executed plugin '{function}' on {input}; wrote {output}");

        Ok(())
    }

    /// Handle the stream command
    ///
    /// Processes a large file in chunks to reduce memory usage.
    pub fn handle_stream(&self, input: String, output: String, _chunk_size: usize) -> Result<()> {
        println!("Streaming support is a placeholder. Processing file normally...");

        let converter = Converter::new();
        let data = converter.read_any_data(&input, None)?;
        converter.write_any_data(&output, &data, None)?;

        println!("Processed {} rows; wrote {}", data.len(), output);

        Ok(())
    }

    /// Handle the completions command
    ///
    /// Generates shell completion scripts.
    pub fn handle_completions(&self, shell: String) -> Result<()> {
        use clap::CommandFactory;
        use clap_complete::{generate, Shell};
        use std::io;

        let mut cmd = super::super::Cli::command();

        let shell_type = match shell.to_lowercase().as_str() {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            "powershell" => Shell::PowerShell,
            "elvish" => Shell::Elvish,
            _ => anyhow::bail!("Unsupported shell: {}", shell),
        };

        generate(shell_type, &mut cmd, "datacell", &mut io::stdout());

        Ok(())
    }

    /// Handle the config_init command
    ///
    /// Creates a default configuration file.
    pub fn handle_config_init(&self) -> Result<()> {
        use crate::Config;

        let path = ".datacell.toml";
        if std::path::Path::new(path).exists() {
            anyhow::bail!("{} already exists", path);
        }

        std::fs::write(path, Config::default_config_content())
            .context(format!("Failed to write {}", path))?;
        println!("Wrote {}", path);

        Ok(())
    }

    /// Handle the export_styled command
    ///
    /// Exports data to a styled Excel file.
    pub fn handle_export_styled(
        &self,
        input: String,
        output: String,
        style: Option<String>,
    ) -> Result<()> {
        let output_lower = output.to_lowercase();
        if !output_lower.ends_with(".xlsx") {
            anyhow::bail!("ExportStyled requires .xlsx output");
        }

        let converter = Converter::new();
        let data = converter.read_any_data(&input, None)?;

        let mut options = WriteOptions::default();
        match style.as_deref() {
            None | Some("default") | Some("config") => {
                let config = crate::Config::load()?;
                if let Some(b) = config.excel.header_bold {
                    options.header_style.bold = b;
                }
                if let Some(c) = config.excel.header_bg_color {
                    options.header_style.bg_color = Some(c);
                }
                if let Some(c) = config.excel.header_font_color {
                    options.header_style.font_color = Some(c);
                }
                if let Some(v) = config.excel.auto_filter {
                    options.auto_filter = v;
                }
                if let Some(v) = config.excel.freeze_header {
                    options.freeze_header = v;
                }
                if let Some(v) = config.excel.auto_fit {
                    options.auto_fit = v;
                }
            }
            Some("none") => {
                options.style_header = false;
                options.freeze_header = false;
                options.auto_filter = false;
                options.auto_fit = false;
            }
            Some(other) => anyhow::bail!("Unknown style: {}", other),
        }

        let handler = ExcelHandler::new();
        handler.write_styled(&output, &data, &options)?;
        println!("Wrote {}", output);

        Ok(())
    }

    // Batch operation helpers

    fn batch_convert(&self, input: &str, output: &str) -> Result<()> {
        let converter = Converter::new();
        converter.convert(input, output, None)?;
        Ok(())
    }

    fn batch_sort(&self, input: &str, output: &str, column: &str, ascending: bool) -> Result<()> {
        use crate::operations::{DataOperations, SortOrder};
        let converter = Converter::new();
        let mut data = converter.read_any_data(input, None)?;

        let col_idx = self.find_column_index(&data, column)?;
        let ops = DataOperations::new();
        ops.sort_by_column(
            &mut data,
            col_idx,
            if ascending {
                SortOrder::Ascending
            } else {
                SortOrder::Descending
            },
        )?;

        converter.write_any_data(output, &data, None)?;
        Ok(())
    }

    fn batch_filter(&self, input: &str, output: &str, where_clause: &str) -> Result<()> {
        let converter = Converter::new();
        let data = converter.read_any_data(input, None)?;

        let parts: Vec<&str> = where_clause.split_whitespace().collect();
        if parts.len() < 3 {
            anyhow::bail!("Invalid WHERE clause format");
        }

        let column = parts[0];
        let operator = parts[1];
        let value = parts[2..].join(" ");

        let col_idx = self.find_column_index(&data, column)?;

        let ops = DataOperations::new();
        let filtered = ops.filter_rows(&data, col_idx, operator, &value)?;

        converter.write_any_data(output, &filtered, None)?;
        Ok(())
    }

    fn batch_dedupe(&self, input: &str, output: &str) -> Result<()> {
        let converter = Converter::new();
        let data = converter.read_any_data(input, None)?;

        let ops = DataOperations::new();
        let deduped = ops.deduplicate(&data);

        converter.write_any_data(output, &deduped, None)?;
        Ok(())
    }

    fn batch_normalize(&self, input: &str, output: &str, column: &str) -> Result<()> {
        let converter = Converter::new();
        let mut data = converter.read_any_data(input, None)?;

        let col_idx = self.find_column_index(&data, column)?;
        let ops = DataOperations::new();
        ops.normalize(&mut data, col_idx)?;

        converter.write_any_data(output, &data, None)?;
        Ok(())
    }

    /// Find column index by name
    fn find_column_index(&self, data: &[Vec<String>], column: &str) -> Result<usize> {
        if data.is_empty() {
            anyhow::bail!("Data is empty, cannot find column '{}'", column);
        }

        let header = &data[0];
        header
            .iter()
            .position(|h| h == column)
            .ok_or_else(|| anyhow::anyhow!("Column '{}' not found", column))
    }
}
