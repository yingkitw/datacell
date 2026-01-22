use rmcp::{
    ServerHandler,
    handler::server::wrapper::Parameters,
    model::{ErrorData as McpError, *},
    schemars, tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::converter::Converter;
use crate::csv_handler::CsvHandler;
use crate::excel::ExcelHandler;
use crate::formula::FormulaEvaluator;

use rmcp::handler::server::tool::ToolRouter;

#[derive(Debug, Clone)]
pub struct DatacellMcpServer {
    tool_router: ToolRouter<DatacellMcpServer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReadRequest {
    #[schemars(description = "Path to the file to read (CSV, XLS, or XLSX)")]
    pub path: String,
    #[schemars(description = "Sheet name for Excel files (optional, defaults to first sheet)")]
    pub sheet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WriteRequest {
    #[schemars(description = "Path to the input CSV file")]
    pub csv_path: String,
    #[schemars(description = "Path to the output file (CSV, XLS, or XLSX)")]
    pub output_path: String,
    #[schemars(description = "Sheet name for Excel output (optional, defaults to Sheet1)")]
    pub sheet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ConvertRequest {
    #[schemars(description = "Path to the input file (CSV, XLS, or XLSX)")]
    pub input: String,
    #[schemars(description = "Path to the output file (CSV, XLS, or XLSX)")]
    pub output: String,
    #[schemars(description = "Sheet name for Excel input (optional, defaults to first sheet)")]
    pub sheet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct FormulaRequest {
    #[schemars(description = "Path to the input file (CSV, XLS, or XLSX)")]
    pub input: String,
    #[schemars(description = "Path to the output file")]
    pub output: String,
    #[schemars(description = "Formula to apply (e.g., 'SUM(A1:A10)' or 'A1+B1')")]
    pub formula: String,
    #[schemars(description = "Target cell for the formula result (e.g., 'C1')")]
    pub cell: String,
    #[schemars(description = "Sheet name for Excel files (optional)")]
    pub sheet: Option<String>,
}

fn make_error(msg: String) -> McpError {
    McpError {
        code: ErrorCode::INTERNAL_ERROR,
        message: Cow::from(msg),
        data: None,
    }
}

#[tool_router]
impl DatacellMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Read data from a CSV, XLS, or XLSX file and return its contents")]
    async fn read_file(
        &self,
        request: Parameters<ReadRequest>,
    ) -> Result<CallToolResult, McpError> {
        let result = if request.0.path.ends_with(".csv") {
            let handler = CsvHandler::new();
            handler.read(&request.0.path)
        } else if request.0.path.ends_with(".xls") || request.0.path.ends_with(".xlsx") {
            let handler = ExcelHandler::new();
            handler.read_with_sheet(&request.0.path, request.0.sheet.as_deref())
        } else {
            return Err(make_error(
                "Unsupported file format. Supported: .csv, .xls, .xlsx".to_string(),
            ));
        };

        match result {
            Ok(data) => Ok(CallToolResult::success(vec![Content::text(data)])),
            Err(e) => Err(make_error(format!("Failed to read file: {}", e))),
        }
    }

    #[tool(description = "Write data from a CSV file to a new CSV, XLS, or XLSX file")]
    async fn write_file(
        &self,
        request: Parameters<WriteRequest>,
    ) -> Result<CallToolResult, McpError> {
        let result = if request.0.output_path.ends_with(".csv") {
            let handler = CsvHandler::new();
            handler.write_from_csv(&request.0.csv_path, &request.0.output_path)
        } else if request.0.output_path.ends_with(".xls")
            || request.0.output_path.ends_with(".xlsx")
        {
            let handler = ExcelHandler::new();
            handler.write_from_csv(
                &request.0.csv_path,
                &request.0.output_path,
                request.0.sheet.as_deref(),
            )
        } else {
            return Err(make_error(
                "Unsupported output format. Supported: .csv, .xls, .xlsx".to_string(),
            ));
        };

        match result {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Successfully written to {}",
                request.0.output_path
            ))])),
            Err(e) => Err(make_error(format!("Failed to write file: {}", e))),
        }
    }

    #[tool(description = "Convert between file formats (CSV to Excel or Excel to CSV)")]
    async fn convert_file(
        &self,
        request: Parameters<ConvertRequest>,
    ) -> Result<CallToolResult, McpError> {
        let converter = Converter::new();
        match converter.convert(
            &request.0.input,
            &request.0.output,
            request.0.sheet.as_deref(),
        ) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Successfully converted {} to {}",
                request.0.input, request.0.output
            ))])),
            Err(e) => Err(make_error(format!("Failed to convert file: {}", e))),
        }
    }

    #[tool(description = "Apply a formula to a spreadsheet file and save the result")]
    async fn apply_formula(
        &self,
        request: Parameters<FormulaRequest>,
    ) -> Result<CallToolResult, McpError> {
        let evaluator = FormulaEvaluator::new();
        let result = if request.0.input.ends_with(".csv") {
            evaluator.apply_to_csv(
                &request.0.input,
                &request.0.output,
                &request.0.formula,
                &request.0.cell,
            )
        } else if request.0.input.ends_with(".xls") || request.0.input.ends_with(".xlsx") {
            evaluator.apply_to_excel(
                &request.0.input,
                &request.0.output,
                &request.0.formula,
                &request.0.cell,
                request.0.sheet.as_deref(),
            )
        } else {
            return Err(make_error(
                "Unsupported file format. Supported: .csv, .xls, .xlsx".to_string(),
            ));
        };

        match result {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Successfully applied formula {} to cell {} in {}",
                request.0.formula, request.0.cell, request.0.output
            ))])),
            Err(e) => Err(make_error(format!("Failed to apply formula: {}", e))),
        }
    }
}

#[tool_handler]
impl ServerHandler for DatacellMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "A spreadsheet tool for reading, writing, converting CSV and Excel files with formula support. \
                Use read_file to read data, write_file to write data, convert_file to convert between formats, \
                and apply_formula to apply formulas to spreadsheets."
                    .to_string(),
            ),
        }
    }
}
