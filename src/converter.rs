use anyhow::Result;
use crate::excel::ExcelHandler;
use crate::csv_handler::CsvHandler;
use crate::handler_registry::HandlerRegistry;
use crate::traits::DataWriteOptions;

pub struct Converter {
    registry: HandlerRegistry,
    excel_handler: ExcelHandler,
    csv_handler: CsvHandler,
}

impl Converter {
    pub fn new() -> Self {
        Self {
            registry: HandlerRegistry::new(),
            excel_handler: ExcelHandler::new(),
            csv_handler: CsvHandler::new(),
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
        // Check if it's an Excel format that needs special handling
        if path.to_lowercase().ends_with(".xlsx") || 
           path.to_lowercase().ends_with(".xls") ||
           path.to_lowercase().ends_with(".ods") {
            if path.to_lowercase().ends_with(".ods") {
                return self.excel_handler.read_ods_data(path, sheet_name);
            }
            let content = self.excel_handler.read_with_sheet(path, sheet_name)?;
            return Ok(self.parse_csv_data(&content));
        }
        
        // For Parquet and Avro, use read_with_headers to include column names
        if path.to_lowercase().ends_with(".parquet") {
            use crate::columnar::ParquetHandler;
            let handler = ParquetHandler::new();
            return handler.read_with_headers(path);
        }
        
        if path.to_lowercase().ends_with(".avro") {
            use crate::columnar::AvroHandler;
            let handler = AvroHandler::new();
            return handler.read_with_headers(path);
        }
        
        // Use registry for CSV and other formats
        self.registry.read(path)
    }
    
    /// Write data to any supported format
    fn write_any(&self, path: &str, data: &[Vec<String>], sheet_name: Option<&str>) -> Result<()> {
        // Check if it's an Excel format that needs special handling
        if path.to_lowercase().ends_with(".xlsx") || 
           path.to_lowercase().ends_with(".xls") {
            // Write to temp CSV then convert
            let temp_csv = format!("{}.tmp.csv", path);
            self.csv_handler.write_records(&temp_csv, data.to_vec())?;
            self.excel_handler.write_from_csv(&temp_csv, path, sheet_name)?;
            std::fs::remove_file(&temp_csv).ok();
            return Ok(());
        }
        
        // Use registry for other formats
        let options = DataWriteOptions {
            sheet_name: sheet_name.map(|s| s.to_string()),
            ..Default::default()
        };
        self.registry.write(path, data, options)
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

