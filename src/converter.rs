use anyhow::Result;
use crate::excel::ExcelHandler;
use crate::csv_handler::CsvHandler;
use crate::columnar::{ParquetHandler, AvroHandler};

pub struct Converter {
    excel_handler: ExcelHandler,
    csv_handler: CsvHandler,
    parquet_handler: ParquetHandler,
    avro_handler: AvroHandler,
}

impl Converter {
    pub fn new() -> Self {
        Self {
            excel_handler: ExcelHandler::new(),
            csv_handler: CsvHandler::new(),
            parquet_handler: ParquetHandler::new(),
            avro_handler: AvroHandler::new(),
        }
    }

    /// Convert between any supported formats
    /// Supported: csv, xlsx, xls, ods, parquet, avro
    pub fn convert(&self, input: &str, output: &str, sheet_name: Option<&str>) -> Result<()> {
        // Read input data
        let data = self.read_any(input, sheet_name)?;
        
        // Write to output format
        self.write_any(output, &data, sheet_name)?;
        
        Ok(())
    }
    
    /// Read data from any supported format
    fn read_any(&self, path: &str, sheet_name: Option<&str>) -> Result<Vec<Vec<String>>> {
        let ext = self.get_extension(path)?;
        
        match ext.as_str() {
            "csv" => {
                let content = self.csv_handler.read(path)?;
                Ok(self.parse_csv_data(&content))
            }
            "xlsx" | "xls" => {
                let content = self.excel_handler.read_with_sheet(path, sheet_name)?;
                Ok(self.parse_csv_data(&content))
            }
            "ods" => {
                self.excel_handler.read_ods_data(path, sheet_name)
            }
            "parquet" => {
                self.parquet_handler.read_with_headers(path)
            }
            "avro" => {
                self.avro_handler.read_with_headers(path)
            }
            _ => anyhow::bail!("Unsupported input format: {}", ext),
        }
    }
    
    /// Write data to any supported format
    fn write_any(&self, path: &str, data: &[Vec<String>], sheet_name: Option<&str>) -> Result<()> {
        let ext = self.get_extension(path)?;
        
        match ext.as_str() {
            "csv" => {
                self.csv_handler.write_records(path, data.to_vec())?;
            }
            "xlsx" | "xls" => {
                // Write to temp CSV then convert
                let temp_csv = format!("{}.tmp.csv", path);
                self.csv_handler.write_records(&temp_csv, data.to_vec())?;
                self.excel_handler.write_from_csv(&temp_csv, path, sheet_name)?;
                std::fs::remove_file(&temp_csv).ok();
            }
            "parquet" => {
                self.parquet_handler.write(path, data, None)?;
            }
            "avro" => {
                self.avro_handler.write(path, data, None)?;
            }
            _ => anyhow::bail!("Unsupported output format: {}", ext),
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

