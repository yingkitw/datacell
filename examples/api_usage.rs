use datacell::*;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== datacell API Usage Examples ===\n");

    example_1_read_write()?;
    example_2_conversions()?;
    example_3_formulas()?;
    example_4_operations()?;
    example_5_pandas_style()?;
    example_6_ranges()?;
    example_7_streaming()?;
    example_8_validation()?;

    println!("\n=== All examples completed successfully! ===");
    Ok(())
}

fn example_1_read_write() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 1: Basic Read/Write Operations");
    
    let csv_handler = csv_handler::CsvHandler::new();
    
    let data = csv_handler.read("examples/employees.csv")?;
    println!("  ✓ Read {} rows from CSV", data.len());
    
    csv_handler.write("examples/test_output/api_write.csv", &data)?;
    println!("  ✓ Written data to new CSV file");
    
    let excel_handler = excel::ExcelHandler::new();
    excel_handler.write("examples/test_output/api_write.xlsx", &data, Some("Employees"))?;
    println!("  ✓ Written data to Excel file\n");
    
    Ok(())
}

fn example_2_conversions() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 2: Format Conversions");
    
    let converter = converter::Converter::new();
    
    converter.convert(
        "examples/employees.csv",
        "examples/test_output/api_converted.xlsx",
        None,
        None,
    )?;
    println!("  ✓ Converted CSV to Excel");
    
    converter.convert(
        "examples/sales.xlsx",
        "examples/test_output/api_converted.csv",
        None,
        None,
    )?;
    println!("  ✓ Converted Excel to CSV");
    
    converter.convert(
        "examples/employees.csv",
        "examples/test_output/api_converted.parquet",
        None,
        None,
    )?;
    println!("  ✓ Converted CSV to Parquet\n");
    
    Ok(())
}

fn example_3_formulas() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 3: Formula Evaluation");
    
    let csv_handler = csv_handler::CsvHandler::new();
    let mut data = csv_handler.read("examples/sales.csv")?;
    
    let evaluator = formula::FormulaEvaluator::new();
    
    let sum_result = evaluator.evaluate("SUM(C2:C10)", &data)?;
    println!("  ✓ SUM formula result: {:?}", sum_result);
    
    let avg_result = evaluator.evaluate("AVERAGE(C2:C10)", &data)?;
    println!("  ✓ AVERAGE formula result: {:?}", avg_result);
    
    let if_result = evaluator.evaluate("IF(C2>1000,\"High\",\"Low\")", &data)?;
    println!("  ✓ IF formula result: {:?}\n", if_result);
    
    Ok(())
}

fn example_4_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 4: Data Operations");
    
    let csv_handler = csv_handler::CsvHandler::new();
    let data = csv_handler.read("examples/sales.csv")?;
    
    let ops = operations::DataOperations::new();
    
    let sorted = ops.sort(&data, "Amount", false)?;
    println!("  ✓ Sorted data by Amount (ascending)");
    
    let filtered = ops.filter(&data, "Amount > 1000")?;
    println!("  ✓ Filtered data (Amount > 1000): {} rows", filtered.len());
    
    let deduped = ops.deduplicate(&data)?;
    println!("  ✓ Deduplicated data: {} unique rows", deduped.len());
    
    let transposed = ops.transpose(&data)?;
    println!("  ✓ Transposed data\n");
    
    Ok(())
}

fn example_5_pandas_style() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 5: Pandas-Style Operations");
    
    let csv_handler = csv_handler::CsvHandler::new();
    let data = csv_handler.read("examples/employees.csv")?;
    
    let ops = operations::DataOperations::new();
    
    let head = ops.head(&data, 3)?;
    println!("  ✓ Head (first 3 rows): {} rows", head.len());
    
    let tail = ops.tail(&data, 3)?;
    println!("  ✓ Tail (last 3 rows): {} rows", tail.len());
    
    let selected = ops.select(&data, &["Name", "Department"])?;
    println!("  ✓ Selected columns: {} columns", selected[0].len());
    
    let stats = ops.describe(&data)?;
    println!("  ✓ Generated descriptive statistics");
    
    let grouped = ops.groupby(&data, "Department", &[("Salary", "sum")])?;
    println!("  ✓ Grouped by Department with sum aggregation\n");
    
    Ok(())
}

fn example_6_ranges() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 6: Cell Range Operations");
    
    let excel_handler = excel::ExcelHandler::new();
    
    let range_data = excel_handler.read_range("examples/employees.xlsx", "A1:C5", None)?;
    println!("  ✓ Read range A1:C5: {} rows", range_data.len());
    
    let sheets = excel_handler.list_sheets("examples/employees.xlsx")?;
    println!("  ✓ Listed sheets: {:?}", sheets);
    
    let all_sheets = excel_handler.read_all_sheets("examples/employees.xlsx")?;
    println!("  ✓ Read all sheets: {} sheets\n", all_sheets.len());
    
    Ok(())
}

fn example_7_streaming() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 7: Streaming Operations");
    
    let csv_handler = csv_handler::CsvHandler::new();
    
    let mut row_count = 0;
    csv_handler.stream_read("examples/employees.csv", |chunk| {
        row_count += chunk.len();
        Ok(())
    })?;
    println!("  ✓ Streamed {} rows from CSV", row_count);
    
    let data = csv_handler.read("examples/sales.csv")?;
    csv_handler.stream_write("examples/test_output/api_streamed.csv", &data, |progress| {
        println!("  ✓ Write progress: {}%", progress);
    })?;
    println!("  ✓ Streamed write completed\n");
    
    Ok(())
}

fn example_8_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 8: Data Validation and Profiling");
    
    let csv_handler = csv_handler::CsvHandler::new();
    let data = csv_handler.read("examples/employees.csv")?;
    
    let validator = validation::DataValidator::new();
    let rules = validation::ValidationRules::from_file("examples/validation_rules.json")?;
    
    let validation_result = validator.validate(&data, &rules)?;
    println!("  ✓ Validation completed: {} errors", validation_result.errors.len());
    
    let profiler = profiling::DataProfiler::new();
    let profile = profiler.profile(&data)?;
    println!("  ✓ Data profiling completed");
    println!("    - Rows: {}", profile.row_count);
    println!("    - Columns: {}", profile.column_count);
    
    let quality = quality::QualityReporter::new();
    let report = quality.generate_report(&data)?;
    println!("  ✓ Quality report generated\n");
    
    Ok(())
}
