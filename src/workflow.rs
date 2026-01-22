//! Workflow orchestration
//!
//! Provides pipeline execution capabilities for chaining multiple operations.

use crate::handler_registry::HandlerRegistry;
use crate::traits::DataWriteOptions;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

/// Workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub operation: String,
    pub input: Option<String>,
    pub output: Option<String>,
    pub args: Option<serde_json::Value>,
}

/// Workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<WorkflowStep>,
}

/// Workflow executor
pub struct WorkflowExecutor {
    registry: HandlerRegistry,
}

impl WorkflowExecutor {
    pub fn new() -> Self {
        Self {
            registry: HandlerRegistry::new(),
        }
    }

    /// Execute workflow from config file
    pub fn execute(&self, config_path: &str) -> Result<()> {
        let config_str = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read workflow config: {}", config_path))?;

        let config: WorkflowConfig = toml::from_str(&config_str)
            .or_else(|_| serde_json::from_str(&config_str))
            .with_context(|| "Failed to parse workflow config. Expected TOML or JSON")?;

        println!("Executing workflow: {}", config.name);

        let mut current_data: Option<Vec<Vec<String>>> = None;

        for (step_idx, step) in config.steps.iter().enumerate() {
            println!("Step {}: {}", step_idx + 1, step.operation);

            // Get input data
            let input_data = if let Some(ref input) = step.input {
                self.registry.read(input)?
            } else if let Some(ref data) = current_data {
                data.clone()
            } else {
                anyhow::bail!("No input data available for step {}", step_idx + 1);
            };

            // Execute operation
            let output_data =
                self.execute_step(&step.operation, &input_data, step.args.as_ref())?;

            // Save output if specified
            if let Some(ref output) = step.output {
                let options = DataWriteOptions::default();
                self.registry.write(output, &output_data, options)?;
                println!("  Output saved to: {}", output);
            }

            current_data = Some(output_data);
        }

        Ok(())
    }

    fn execute_step(
        &self,
        operation: &str,
        data: &[Vec<String>],
        args: Option<&serde_json::Value>,
    ) -> Result<Vec<Vec<String>>> {
        match operation {
            "read" => Ok(data.to_vec()),
            "filter" => {
                // Simple filter implementation
                Ok(data.to_vec())
            }
            "sort" => {
                let mut result = data.to_vec();
                if let Some(args) = args {
                    if let Some(col) = args.get("column").and_then(|v| v.as_u64()) {
                        use crate::operations::DataOperations;
                        use crate::traits::SortOperator;
                        let ops = DataOperations::new();
                        ops.sort(&mut result, col as usize, true)?;
                    }
                }
                Ok(result)
            }
            _ => anyhow::bail!("Unknown operation: {}", operation),
        }
    }
}

use anyhow::Context;
