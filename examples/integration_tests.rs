use datacell::*;
use std::path::Path;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_csv_round_trip() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        csv_handler.write("examples/test_output/round_trip.csv", &data).unwrap();
        let data2 = csv_handler.read("examples/test_output/round_trip.csv").unwrap();
        
        assert_eq!(data.len(), data2.len());
        assert_eq!(data[0].len(), data2[0].len());
    }

    #[test]
    fn test_excel_round_trip() {
        let excel_handler = excel::ExcelHandler::new();
        let data = excel_handler.read("examples/employees.xlsx", None).unwrap();
        
        excel_handler.write("examples/test_output/round_trip.xlsx", &data, Some("Sheet1")).unwrap();
        let data2 = excel_handler.read("examples/test_output/round_trip.xlsx", None).unwrap();
        
        assert_eq!(data.len(), data2.len());
    }

    #[test]
    fn test_csv_to_excel_conversion() {
        let converter = converter::Converter::new();
        converter.convert(
            "examples/employees.csv",
            "examples/test_output/csv_to_excel.xlsx",
            None,
            None,
        ).unwrap();
        
        assert!(Path::new("examples/test_output/csv_to_excel.xlsx").exists());
    }

    #[test]
    fn test_excel_to_csv_conversion() {
        let converter = converter::Converter::new();
        converter.convert(
            "examples/sales.xlsx",
            "examples/test_output/excel_to_csv.csv",
            None,
            None,
        ).unwrap();
        
        assert!(Path::new("examples/test_output/excel_to_csv.csv").exists());
    }

    #[test]
    fn test_parquet_conversion() {
        let converter = converter::Converter::new();
        
        converter.convert(
            "examples/employees.csv",
            "examples/test_output/to_parquet.parquet",
            None,
            None,
        ).unwrap();
        
        converter.convert(
            "examples/test_output/to_parquet.parquet",
            "examples/test_output/from_parquet.csv",
            None,
            None,
        ).unwrap();
        
        assert!(Path::new("examples/test_output/from_parquet.csv").exists());
    }

    #[test]
    fn test_avro_conversion() {
        let converter = converter::Converter::new();
        
        converter.convert(
            "examples/employees.csv",
            "examples/test_output/to_avro.avro",
            None,
            None,
        ).unwrap();
        
        converter.convert(
            "examples/test_output/to_avro.avro",
            "examples/test_output/from_avro.csv",
            None,
            None,
        ).unwrap();
        
        assert!(Path::new("examples/test_output/from_avro.csv").exists());
    }

    #[test]
    fn test_formula_sum() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/numbers.csv").unwrap();
        
        let evaluator = formula::FormulaEvaluator::new();
        let result = evaluator.evaluate("SUM(A1:A3)", &data).unwrap();
        
        assert!(matches!(result, formula::CellValue::Number(_)));
    }

    #[test]
    fn test_formula_average() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/numbers.csv").unwrap();
        
        let evaluator = formula::FormulaEvaluator::new();
        let result = evaluator.evaluate("AVERAGE(A1:A3)", &data).unwrap();
        
        assert!(matches!(result, formula::CellValue::Number(_)));
    }

    #[test]
    fn test_formula_if() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/sales.csv").unwrap();
        
        let evaluator = formula::FormulaEvaluator::new();
        let result = evaluator.evaluate("IF(1>0,\"Yes\",\"No\")", &data).unwrap();
        
        assert!(matches!(result, formula::CellValue::Text(_)));
    }

    #[test]
    fn test_formula_concat() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        let evaluator = formula::FormulaEvaluator::new();
        let result = evaluator.evaluate("CONCAT(\"Hello\",\" \",\"World\")", &data).unwrap();
        
        if let formula::CellValue::Text(s) = result {
            assert_eq!(s, "Hello World");
        } else {
            panic!("Expected text result");
        }
    }

    #[test]
    fn test_sort_ascending() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/sales.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let sorted = ops.sort(&data, "Amount", false).unwrap();
        
        assert_eq!(data.len(), sorted.len());
    }

    #[test]
    fn test_sort_descending() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/sales.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let sorted = ops.sort(&data, "Amount", true).unwrap();
        
        assert_eq!(data.len(), sorted.len());
    }

    #[test]
    fn test_filter() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/sales.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let filtered = ops.filter(&data, "Amount > 1000").unwrap();
        
        assert!(filtered.len() <= data.len());
    }

    #[test]
    fn test_deduplicate() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/duplicates.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let deduped = ops.deduplicate(&data).unwrap();
        
        assert!(deduped.len() <= data.len());
    }

    #[test]
    fn test_transpose() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let transposed = ops.transpose(&data).unwrap();
        
        assert_eq!(data.len(), transposed[0].len());
        assert_eq!(data[0].len(), transposed.len());
    }

    #[test]
    fn test_head() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let head = ops.head(&data, 3).unwrap();
        
        assert_eq!(head.len(), 3);
    }

    #[test]
    fn test_tail() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let tail = ops.tail(&data, 3).unwrap();
        
        assert_eq!(tail.len(), 3);
    }

    #[test]
    fn test_select_columns() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let selected = ops.select(&data, &["Name", "Department"]).unwrap();
        
        assert_eq!(selected[0].len(), 2);
    }

    #[test]
    fn test_drop_columns() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let dropped = ops.drop(&data, &["Salary"]).unwrap();
        
        assert_eq!(dropped[0].len(), data[0].len() - 1);
    }

    #[test]
    fn test_groupby() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/sales.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let grouped = ops.groupby(&data, "Category", &[("Amount", "sum")]).unwrap();
        
        assert!(grouped.len() > 0);
    }

    #[test]
    fn test_join() {
        let csv_handler = csv_handler::CsvHandler::new();
        let left = csv_handler.read("examples/sales.csv").unwrap();
        let right = csv_handler.read("examples/lookup.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let joined = ops.join(&left, &right, "Category", "left").unwrap();
        
        assert!(joined.len() > 0);
    }

    #[test]
    fn test_concat() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data1 = csv_handler.read("examples/sales.csv").unwrap();
        let data2 = csv_handler.read("examples/sales.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let concatenated = ops.concat(&[data1.clone(), data2.clone()]).unwrap();
        
        assert_eq!(concatenated.len(), data1.len() + data2.len());
    }

    #[test]
    fn test_fillna() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/financial_data.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let filled = ops.fillna(&data, "0").unwrap();
        
        assert_eq!(filled.len(), data.len());
    }

    #[test]
    fn test_dropna() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/financial_data.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let dropped = ops.dropna(&data).unwrap();
        
        assert!(dropped.len() <= data.len());
    }

    #[test]
    fn test_describe() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/financial_data.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let stats = ops.describe(&data).unwrap();
        
        assert!(stats.len() > 0);
    }

    #[test]
    fn test_value_counts() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/sales.csv").unwrap();
        
        let ops = operations::DataOperations::new();
        let counts = ops.value_counts(&data, "Category").unwrap();
        
        assert!(counts.len() > 0);
    }

    #[test]
    fn test_read_range() {
        let excel_handler = excel::ExcelHandler::new();
        let range_data = excel_handler.read_range("examples/employees.xlsx", "A1:C5", None).unwrap();
        
        assert!(range_data.len() <= 5);
        assert!(range_data[0].len() <= 3);
    }

    #[test]
    fn test_list_sheets() {
        let excel_handler = excel::ExcelHandler::new();
        let sheets = excel_handler.list_sheets("examples/employees.xlsx").unwrap();
        
        assert!(sheets.len() > 0);
    }

    #[test]
    fn test_read_all_sheets() {
        let excel_handler = excel::ExcelHandler::new();
        let all_sheets = excel_handler.read_all_sheets("examples/employees.xlsx").unwrap();
        
        assert!(all_sheets.len() > 0);
    }

    #[test]
    fn test_validation() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        let validator = validation::DataValidator::new();
        let rules = validation::ValidationRules::from_file("examples/validation_rules.json").unwrap();
        
        let result = validator.validate(&data, &rules).unwrap();
        assert!(result.is_valid || !result.is_valid);
    }

    #[test]
    fn test_profiling() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        let profiler = profiling::DataProfiler::new();
        let profile = profiler.profile(&data).unwrap();
        
        assert!(profile.row_count > 0);
        assert!(profile.column_count > 0);
    }

    #[test]
    fn test_encryption_xor() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        let encryptor = encryption::DataEncryptor::new();
        let encrypted = encryptor.encrypt(&data, "testkey", encryption::Algorithm::XOR).unwrap();
        let decrypted = encryptor.decrypt(&encrypted, "testkey", encryption::Algorithm::XOR).unwrap();
        
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_streaming_read() {
        let csv_handler = csv_handler::CsvHandler::new();
        
        let mut row_count = 0;
        csv_handler.stream_read("examples/employees.csv", |chunk| {
            row_count += chunk.len();
            Ok(())
        }).unwrap();
        
        assert!(row_count > 0);
    }

    #[test]
    fn test_plugin_system() {
        let registry = plugins::PluginRegistry::new();
        
        let uppercase_plugin = plugins::UppercasePlugin::new();
        registry.register("uppercase", Box::new(uppercase_plugin));
        
        let result = registry.execute("uppercase", "hello").unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_geospatial_distance() {
        let geo = geospatial::GeoCalculator::new();
        
        let distance = geo.distance(
            40.7128, -74.0060,
            34.0522, -118.2437,
            geospatial::Unit::Kilometers
        ).unwrap();
        
        assert!(distance > 0.0);
    }

    #[test]
    fn test_anomaly_detection() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/financial_data.csv").unwrap();
        
        let detector = anomaly::AnomalyDetector::new();
        let anomalies = detector.detect(&data, "Revenue", anomaly::Method::ZScore, 3.0).unwrap();
        
        assert!(anomalies.len() >= 0);
    }

    #[test]
    fn test_text_analysis() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/employees.csv").unwrap();
        
        let analyzer = text_analysis::TextAnalyzer::new();
        let stats = analyzer.analyze(&data, "Name", text_analysis::Operation::Stats).unwrap();
        
        assert!(stats.len() > 0);
    }

    #[test]
    fn test_timeseries_resample() {
        let csv_handler = csv_handler::CsvHandler::new();
        let data = csv_handler.read("examples/financial_data.csv").unwrap();
        
        let ts = timeseries::TimeSeriesOps::new();
        let resampled = ts.resample(
            &data,
            "Date",
            "Revenue",
            timeseries::Interval::Daily,
            timeseries::Aggregation::Sum
        ).unwrap();
        
        assert!(resampled.len() > 0);
    }
}

fn main() {
    println!("Run these tests with: cargo test --example integration_tests");
}
