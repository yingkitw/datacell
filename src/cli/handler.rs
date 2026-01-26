//! Main command handler implementation
//!
//! This module provides the default command handler that delegates
//! to specialized command handlers based on the command type.

use crate::cli::{
    commands::{advanced::AdvancedCommandHandler, io::IoCommandHandler, pandas::PandasCommandHandler, transform::TransformCommandHandler},
    Commands,
};
use anyhow::{Context, Result};

/// Default command handler
///
/// This handler delegates to specialized command handlers based on the command type.
pub struct DefaultCommandHandler {
    io: IoCommandHandler,
    transform: TransformCommandHandler,
    pandas: PandasCommandHandler,
    advanced: AdvancedCommandHandler,
}

impl DefaultCommandHandler {
    /// Create a new default command handler
    pub fn new() -> Self {
        Self {
            io: IoCommandHandler::new(),
            transform: TransformCommandHandler::new(),
            pandas: PandasCommandHandler::new(),
            advanced: AdvancedCommandHandler::new(),
        }
    }
}

impl Default for DefaultCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl super::commands::CommandHandler for DefaultCommandHandler {
    /// Handle a command by delegating to the appropriate specialized handler
    fn handle(&self, command: Commands) -> Result<()> {
        match command {
            // I/O commands
            Commands::Read {
                input,
                sheet,
                range,
                format,
            } => self.io.handle_read(input, sheet, range, format),

            Commands::Write { output, csv, sheet } => self.io.handle_write(output, csv, sheet),

            Commands::Convert {
                input,
                output,
                sheet,
            } => self.io.handle_convert(input, output, sheet),

            Commands::Formula {
                input,
                output,
                formula,
                cell,
                sheet,
            } => self.io.handle_formula(input, output, formula, cell, sheet),

            Commands::Serve => self.io.handle_serve(),

            Commands::Sheets { input } => self.io.handle_sheets(input),

            Commands::ReadAll { input, format } => self.io.handle_read_all(input, format),

            Commands::WriteRange {
                input,
                output,
                start,
            } => self.io.handle_write_range(input, output, start),

            Commands::Append { source, target } => self.io.handle_append(source, target),

            // Transform commands
            Commands::Sort {
                input,
                output,
                column,
                ascending,
            } => self.transform.handle_sort(input, output, column, ascending),

            Commands::Filter {
                input,
                output,
                where_clause,
            } => self.transform.handle_filter(input, output, where_clause),

            Commands::Replace {
                input,
                output,
                find,
                replace,
                column,
            } => self.transform.handle_replace(input, output, find, replace, column),

            Commands::Dedupe {
                input,
                output,
                columns,
            } => self.transform.handle_dedupe(input, output, columns),

            Commands::Transpose { input, output } => self.transform.handle_transpose(input, output),

            Commands::Select {
                input,
                output,
                columns,
            } => self.transform.handle_select(input, output, columns),

            Commands::Rename {
                input,
                output,
                from,
                to,
            } => self.transform.handle_rename(input, output, from, to),

            Commands::Drop {
                input,
                output,
                columns,
            } => self.transform.handle_drop(input, output, columns),

            Commands::Fillna {
                input,
                output,
                value,
                columns,
            } => self.transform.handle_fillna(input, output, value, columns),

            Commands::Dropna { input, output } => self.transform.handle_dropna(input, output),

            Commands::Mutate {
                input,
                output,
                column,
                formula,
            } => self.transform.handle_mutate(input, output, column, formula),

            Commands::Query {
                input,
                output,
                where_clause,
            } => self.transform.handle_query(input, output, where_clause),

            Commands::Astype {
                input,
                output,
                column,
                target_type,
            } => self.transform.handle_astype(input, output, column, target_type),

            // Pandas-style commands
            Commands::Head { input, n, format } => self.pandas.handle_head(input, n, format),

            Commands::Tail { input, n, format } => self.pandas.handle_tail(input, n, format),

            Commands::Sample {
                input,
                n,
                seed,
                format,
            } => self.pandas.handle_sample(input, n, seed, format),

            Commands::Describe { input, format } => self.pandas.handle_describe(input, format),

            Commands::ValueCounts { input, column } => self.pandas.handle_value_counts(input, column),

            Commands::Corr { input, columns } => self.pandas.handle_corr(input, columns),

            Commands::Groupby {
                input,
                output,
                by,
                agg,
            } => self.pandas.handle_groupby(input, output, by, agg),

            Commands::Join {
                left,
                right,
                output,
                on,
                how,
            } => self.pandas.handle_join(left, right, output, on, how),

            Commands::Concat { inputs, output } => self.pandas.handle_concat(inputs, output),

            Commands::Unique { input, column } => self.pandas.handle_unique(input, column),

            Commands::Info { input } => self.pandas.handle_info(input),

            Commands::Dtypes { input } => self.pandas.handle_dtypes(input),

            Commands::Pivot {
                input,
                output,
                index,
                columns,
                values,
                agg,
            } => self.pandas.handle_pivot(input, output, index, columns, values, agg),

            // Advanced commands
            Commands::Profile {
                input,
                output,
            } => self.advanced.handle_profile(input, output),

            Commands::Validate {
                input,
                rules,
                output,
                report,
            } => self.advanced.handle_validate(input, rules, output, report),

            Commands::Chart {
                input,
                output,
                chart_type,
                title,
                x_column,
                y_column,
            } => self.advanced.handle_chart(input, output, chart_type, title, x_column, y_column),

            Commands::Encrypt {
                input,
                output,
                algorithm,
            } => self.advanced.handle_encrypt(input, output, algorithm),

            Commands::Decrypt {
                input,
                output,
            } => self.advanced.handle_decrypt(input, output),

            Commands::Batch {
                inputs,
                output_dir,
                operation,
                args,
            } => self.advanced.handle_batch(inputs, output_dir, operation, args),

            Commands::Plugin {
                function,
                input,
                output,
                args,
            } => self.advanced.handle_plugin(function, input, output, args),

            Commands::Stream {
                input,
                output,
                chunk_size,
            } => self.advanced.handle_stream(input, output, chunk_size),

            Commands::Completions { shell } => self.advanced.handle_completions(shell),

            Commands::ConfigInit => self.advanced.handle_config_init(),

            Commands::ExportStyled {
                input,
                output,
                style,
            } => self.advanced.handle_export_styled(input, output, style),

            // Additional transform commands that were in the original cli.rs
            Commands::Clip {
                input,
                output,
                column,
                min,
                max,
            } => {
                let converter = crate::converter::Converter::new();
                let mut data = converter.read_any_data(&input, None)?;

                let col_idx = Self::find_column_index(&data, &column)?;
                validation::validate_column_index(&data, col_idx)?;

                let min_val: f64 = min.parse()
                    .with_context(|| format!("Invalid min value: {}", min))?;
                let max_val: f64 = max.parse()
                    .with_context(|| format!("Invalid max value: {}", max))?;

                let ops = crate::operations::DataOperations::new();
                let clipped = ops.clip(&mut data, col_idx, Some(min_val), Some(max_val))?;

                converter.write_any_data(&output, &data, None)?;
                println!("Clipped {} cells; wrote {}", clipped, output);
                Ok(())
            }

            Commands::Normalize {
                input,
                output,
                column,
            } => {
                let converter = crate::converter::Converter::new();
                let mut data = converter.read_any_data(&input, None)?;

                let col_idx = Self::find_column_index(&data, &column)?;
                validation::validate_column_index(&data, col_idx)?;

                let ops = crate::operations::DataOperations::new();
                ops.normalize(&mut data, col_idx)?;

                converter.write_any_data(&output, &data, None)?;
                println!("Normalized column {}; wrote {}", column, output);
                Ok(())
            }

            Commands::ParseDate {
                input,
                output,
                column,
                from_format,
                to_format,
            } => {
                let converter = crate::converter::Converter::new();
                let mut data = converter.read_any_data(&input, None)?;

                let col_idx = Self::find_column_index(&data, &column)?;
                validation::validate_column_index(&data, col_idx)?;

                let ops = crate::operations::DataOperations::new();
                let converted = ops.parse_date(&mut data, col_idx, &from_format, &to_format)?;

                converter.write_any_data(&output, &data, None)?;
                println!("Converted {} dates; wrote {}", converted, output);
                Ok(())
            }

            Commands::RegexFilter {
                input,
                output,
                column,
                pattern,
            } => {
                let converter = crate::converter::Converter::new();
                let data = converter.read_any_data(&input, None)?;

                let col_idx = Self::find_column_index(&data, &column)?;
                validation::validate_column_index(&data, col_idx)?;

                let ops = crate::operations::DataOperations::new();
                let filtered = ops.regex_filter(&data, col_idx, &pattern)?;

                converter.write_any_data(&output, &filtered, None)?;
                println!("Filtered to {} rows; wrote {}", filtered.len().saturating_sub(1), output);
                Ok(())
            }

            Commands::RegexReplace {
                input,
                output,
                column,
                pattern,
                replacement,
            } => {
                let converter = crate::converter::Converter::new();
                let mut data = converter.read_any_data(&input, None)?;

                let col_idx = Self::find_column_index(&data, &column)?;
                validation::validate_column_index(&data, col_idx)?;

                let ops = crate::operations::DataOperations::new();
                let replaced = ops.regex_replace(&mut data, col_idx, &pattern, &replacement)?;

                converter.write_any_data(&output, &data, None)?;
                println!("Replaced {} cells; wrote {}", replaced, output);
                Ok(())
            }
        }
    }
}

// Import validation utility
use crate::common::validation;

impl DefaultCommandHandler {
    /// Find column index by name (helper method)
    fn find_column_index(data: &[Vec<String>], column: &str) -> Result<usize> {
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
