use anyhow::Result;
use crate::excel::ExcelHandler;
use crate::csv_handler::CsvHandler;

pub struct Converter {
    excel_handler: ExcelHandler,
    csv_handler: CsvHandler,
}

impl Converter {
    pub fn new() -> Self {
        Self {
            excel_handler: ExcelHandler::new(),
            csv_handler: CsvHandler::new(),
        }
    }

    pub fn convert(&self, input: &str, output: &str, sheet_name: Option<&str>) -> Result<()> {
        let input_ext = self.get_extension(input)?;
        let output_ext = self.get_extension(output)?;

        match (input_ext.as_str(), output_ext.as_str()) {
            ("csv", "xlsx") | ("csv", "xls") => {
                self.excel_handler.write_from_csv(input, output, sheet_name)?;
            }
            ("xlsx", "csv") | ("xls", "csv") => {
                let data = self.excel_handler.read_with_sheet(input, sheet_name)?;
                self.csv_handler.write_records(output, self.parse_csv_data(&data))?;
            }
            ("csv", "csv") => {
                self.csv_handler.write_from_csv(input, output)?;
            }
            _ => {
                anyhow::bail!("Unsupported conversion from {} to {}", input_ext, output_ext);
            }
        }

        Ok(())
    }

    fn get_extension(&self, path: &str) -> Result<String> {
        path.split('.')
            .last()
            .map(|s| s.to_lowercase())
            .ok_or_else(|| anyhow::anyhow!("No file extension found in: {}", path))
    }

    fn parse_csv_data(&self, data: &str) -> Vec<Vec<String>> {
        data.lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            })
            .collect()
    }
}

