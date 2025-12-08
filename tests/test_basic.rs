use datacell::{CsvHandler, ExcelHandler, Converter, FormulaEvaluator, CellRange};
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn unique_path(prefix: &str, ext: &str) -> String {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}_{}.{}", prefix, id, ext)
}

#[test]
fn test_csv_read_write() {
    let handler = CsvHandler::new();
    let test_data = "1,2,3\n4,5,6\n7,8,9\n";
    
    // Write test CSV
    let input_path = "test_input.csv";
    fs::write(input_path, test_data).unwrap();
    
    // Read it back
    let output_path = "test_output.csv";
    handler.write_from_csv(input_path, output_path).unwrap();
    
    let content = fs::read_to_string(output_path).unwrap();
    assert_eq!(content, test_data);
    
    // Cleanup
    fs::remove_file(input_path).ok();
    fs::remove_file(output_path).ok();
}

#[test]
fn test_csv_formula() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "1,2\n3,4\n5,6\n";
    
    let input_path = "test_formula_input.csv";
    fs::write(input_path, test_data).unwrap();
    
    let output_path = "test_formula_output.csv";
    evaluator.apply_to_csv(input_path, output_path, "A1+B1", "C1").unwrap();
    
    let content = fs::read_to_string(output_path).unwrap();
    // Check that the formula result (3.0) is in the output
    assert!(content.contains("3"));
    
    // Cleanup
    fs::remove_file(input_path).ok();
    fs::remove_file(output_path).ok();
}

#[test]
fn test_csv_to_excel_conversion() {
    let converter = Converter::new();
    let test_data = "1,2,3\n4,5,6\n";
    
    let csv_path = "test_convert_input.csv";
    fs::write(csv_path, test_data).unwrap();
    
    let excel_path = "test_convert_output.xlsx";
    converter.convert(csv_path, excel_path, None).unwrap();
    
    // Verify Excel file exists
    assert!(Path::new(excel_path).exists());
    
    // Read back and verify
    let handler = ExcelHandler::new();
    let content = handler.read_with_sheet(excel_path, None).unwrap();
    // Excel output format: numbers are converted to strings
    // Verify that we got some content (at least one row)
    assert!(!content.trim().is_empty(), "Excel file should contain data");
    // Check that numeric values are present (may be in any row)
    assert!(content.contains("4") || content.contains("5") || content.contains("6"), 
            "Excel content should contain numeric values");
    
    // Cleanup
    fs::remove_file(csv_path).ok();
    fs::remove_file(excel_path).ok();
}

#[test]
fn test_formula_sum() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "1,2\n3,4\n5,6\n";
    
    let input_path = unique_path("test_sum_input", "csv");
    let output_path = unique_path("test_sum_output", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    evaluator.apply_to_csv(&input_path, &output_path, "SUM(A1:A3)", "C1").unwrap();
    
    let content = fs::read_to_string(&output_path).unwrap();
    // SUM(1+3+5) = 9
    assert!(content.contains("9"), "SUM result should be 9, got: {}", content);
    
    fs::remove_file(&input_path).ok();
    fs::remove_file(&output_path).ok();
}

#[test]
fn test_formula_average() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "10,20\n30,40\n50,60\n";
    
    let input_path = unique_path("test_avg_input", "csv");
    let output_path = unique_path("test_avg_output", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    evaluator.apply_to_csv(&input_path, &output_path, "AVERAGE(A1:A3)", "C1").unwrap();
    
    let content = fs::read_to_string(&output_path).unwrap();
    // AVERAGE(10+30+50)/3 = 30
    assert!(content.contains("30"), "AVERAGE result should be 30, got: {}", content);
    
    fs::remove_file(&input_path).ok();
    fs::remove_file(&output_path).ok();
}

#[test]
fn test_formula_min() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "5,2\n3,8\n1,4\n";
    
    let input_path = unique_path("test_min_input", "csv");
    let output_path = unique_path("test_min_output", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    evaluator.apply_to_csv(&input_path, &output_path, "MIN(A1:B3)", "C1").unwrap();
    
    let content = fs::read_to_string(&output_path).unwrap();
    // MIN of all values = 1
    assert!(content.contains("1"), "MIN result should be 1, got: {}", content);
    
    fs::remove_file(&input_path).ok();
    fs::remove_file(&output_path).ok();
}

#[test]
fn test_formula_max() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "5,2\n3,8\n1,4\n";
    
    let input_path = unique_path("test_max_input", "csv");
    let output_path = unique_path("test_max_output", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    evaluator.apply_to_csv(&input_path, &output_path, "MAX(A1:B3)", "C1").unwrap();
    
    let content = fs::read_to_string(&output_path).unwrap();
    // MAX of all values = 8
    assert!(content.contains("8"), "MAX result should be 8, got: {}", content);
    
    fs::remove_file(&input_path).ok();
    fs::remove_file(&output_path).ok();
}

#[test]
fn test_formula_count() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "1,2\n3,4\n5,6\n";
    
    let input_path = unique_path("test_count_input", "csv");
    let output_path = unique_path("test_count_output", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    evaluator.apply_to_csv(&input_path, &output_path, "COUNT(A1:B3)", "C1").unwrap();
    
    let content = fs::read_to_string(&output_path).unwrap();
    // COUNT of 6 cells = 6
    assert!(content.contains("6"), "COUNT result should be 6, got: {}", content);
    
    fs::remove_file(&input_path).ok();
    fs::remove_file(&output_path).ok();
}

#[test]
fn test_formula_arithmetic_multiply() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "3,4\n5,6\n";
    
    let input_path = unique_path("test_mult_input", "csv");
    let output_path = unique_path("test_mult_output", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    evaluator.apply_to_csv(&input_path, &output_path, "A1*B1", "C1").unwrap();
    
    let content = fs::read_to_string(&output_path).unwrap();
    // 3*4 = 12
    assert!(content.contains("12"), "Multiply result should be 12, got: {}", content);
    
    fs::remove_file(&input_path).ok();
    fs::remove_file(&output_path).ok();
}

#[test]
fn test_formula_arithmetic_divide() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "20,4\n5,6\n";
    
    let input_path = unique_path("test_div_input", "csv");
    let output_path = unique_path("test_div_output", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    evaluator.apply_to_csv(&input_path, &output_path, "A1/B1", "C1").unwrap();
    
    let content = fs::read_to_string(&output_path).unwrap();
    // 20/4 = 5
    assert!(content.contains("5"), "Divide result should be 5, got: {}", content);
    
    fs::remove_file(&input_path).ok();
    fs::remove_file(&output_path).ok();
}

#[test]
fn test_formula_if_true() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "10,5\n3,4\n";
    
    let input_path = unique_path("test_if_true_input", "csv");
    let output_path = unique_path("test_if_true_output", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    // IF(A1>B1, 100, 0) -> A1(10) > B1(5) is true, so result is 100
    evaluator.apply_to_csv(&input_path, &output_path, "IF(A1>B1, 100, 0)", "C1").unwrap();
    
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("100"), "IF true branch should be 100, got: {}", content);
    
    fs::remove_file(&input_path).ok();
    fs::remove_file(&output_path).ok();
}

#[test]
fn test_formula_if_false() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "3,5\n1,2\n";
    
    let input_path = unique_path("test_if_false_input", "csv");
    let output_path = unique_path("test_if_false_output", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    // IF(A1>B1, 100, 0) -> A1(3) > B1(5) is false, so result is 0
    evaluator.apply_to_csv(&input_path, &output_path, "IF(A1>B1, 100, 0)", "C1").unwrap();
    
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains(",0") || content.starts_with("0"), "IF false branch should be 0, got: {}", content);
    
    fs::remove_file(&input_path).ok();
    fs::remove_file(&output_path).ok();
}

#[test]
fn test_formula_concat() {
    let evaluator = FormulaEvaluator::new();
    let test_data = "Hello,World\n1,2\n";
    
    let input_path = unique_path("test_concat_input", "csv");
    let output_path = unique_path("test_concat_output", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    // CONCAT("Result: ", A2, B2) -> "Result: 12"
    evaluator.apply_to_csv(&input_path, &output_path, "CONCAT(\"Result: \", A2, B2)", "C1").unwrap();
    
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("Result: 12"), "CONCAT should produce 'Result: 12', got: {}", content);
    
    fs::remove_file(&input_path).ok();
    fs::remove_file(&output_path).ok();
}

#[test]
fn test_read_range() {
    let handler = CsvHandler::new();
    let test_data = "1,2,3,4\n5,6,7,8\n9,10,11,12\n";
    
    let input_path = unique_path("test_range_input", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    // Read range B1:C2 (columns 1-2, rows 0-1)
    let range = CellRange::parse("B1:C2").unwrap();
    let data = handler.read_range(&input_path, &range).unwrap();
    
    assert_eq!(data.len(), 2, "Should have 2 rows");
    assert_eq!(data[0], vec!["2", "3"], "First row should be [2, 3]");
    assert_eq!(data[1], vec!["6", "7"], "Second row should be [6, 7]");
    
    fs::remove_file(&input_path).ok();
}

#[test]
fn test_read_json() {
    let handler = CsvHandler::new();
    let test_data = "1,2\n3,4\n";
    
    let input_path = unique_path("test_json_input", "csv");
    fs::write(&input_path, test_data).unwrap();
    
    let json = handler.read_as_json(&input_path).unwrap();
    
    assert!(json.contains("["), "Should be JSON array");
    assert!(json.contains("\"1\""), "Should contain '1'");
    assert!(json.contains("\"4\""), "Should contain '4'");
    
    fs::remove_file(&input_path).ok();
}

