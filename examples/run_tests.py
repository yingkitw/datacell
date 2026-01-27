#!/usr/bin/env python3
"""
datacell Comprehensive Test Runner
Runs all capability tests and generates a detailed report
"""

import subprocess
import sys
import os
import json
from pathlib import Path
from datetime import datetime

class TestRunner:
    def __init__(self):
        self.datacell = "../target/release/datacell"
        self.output_dir = "./test_output"
        self.results = []
        self.passed = 0
        self.failed = 0
        
    def setup(self):
        """Setup test environment"""
        print("=== Setting up test environment ===\n")
        
        if not Path(self.datacell).exists():
            print("Building datacell...")
            subprocess.run(["cargo", "build", "--release"], cwd="..", check=True)
        
        if Path(self.output_dir).exists():
            subprocess.run(["rm", "-rf", self.output_dir])
        Path(self.output_dir).mkdir(parents=True)
        
        print("✓ Setup complete\n")
    
    def run_command(self, cmd, description, category):
        """Run a single test command"""
        try:
            result = subprocess.run(
                cmd,
                shell=True,
                capture_output=True,
                text=True,
                timeout=30
            )
            
            success = result.returncode == 0
            if success:
                self.passed += 1
                status = "✓"
            else:
                self.failed += 1
                status = "✗"
            
            self.results.append({
                "category": category,
                "description": description,
                "command": cmd,
                "status": "PASS" if success else "FAIL",
                "stdout": result.stdout[:200] if result.stdout else "",
                "stderr": result.stderr[:200] if result.stderr else ""
            })
            
            print(f"{status} {description}")
            return success
            
        except subprocess.TimeoutExpired:
            self.failed += 1
            self.results.append({
                "category": category,
                "description": description,
                "command": cmd,
                "status": "TIMEOUT",
                "stdout": "",
                "stderr": "Command timed out after 30 seconds"
            })
            print(f"✗ {description} (TIMEOUT)")
            return False
        except Exception as e:
            self.failed += 1
            self.results.append({
                "category": category,
                "description": description,
                "command": cmd,
                "status": "ERROR",
                "stdout": "",
                "stderr": str(e)
            })
            print(f"✗ {description} (ERROR: {e})")
            return False
    
    def test_file_io(self):
        """Test file I/O operations"""
        print("\n=== 1. File Format I/O Tests ===\n")
        
        tests = [
            (f"{self.datacell} read --input employees.csv > {self.output_dir}/read_csv.txt", "Read CSV"),
            (f"{self.datacell} read --input employees.xlsx > {self.output_dir}/read_xlsx.txt", "Read Excel"),
            (f"{self.datacell} read --input employees.parquet > {self.output_dir}/read_parquet.txt", "Read Parquet"),
            (f"{self.datacell} read --input employees.avro > {self.output_dir}/read_avro.txt", "Read Avro"),
            (f"{self.datacell} read --input employees.csv --format json > {self.output_dir}/employees.json", "JSON output"),
            (f"{self.datacell} read --input employees.csv --format markdown > {self.output_dir}/employees.md", "Markdown output"),
            (f"{self.datacell} read --input employees.xlsx --range 'A1:C5' > {self.output_dir}/range.txt", "Range reading"),
            (f"{self.datacell} sheets --input employees.xlsx > {self.output_dir}/sheets.txt", "List sheets"),
        ]
        
        for cmd, desc in tests:
            self.run_command(cmd, desc, "File I/O")
    
    def test_conversions(self):
        """Test format conversions"""
        print("\n=== 2. Format Conversion Tests ===\n")
        
        tests = [
            (f"{self.datacell} convert --input employees.csv --output {self.output_dir}/csv_to_xlsx.xlsx", "CSV → Excel"),
            (f"{self.datacell} convert --input employees.csv --output {self.output_dir}/csv_to_parquet.parquet", "CSV → Parquet"),
            (f"{self.datacell} convert --input employees.csv --output {self.output_dir}/csv_to_avro.avro", "CSV → Avro"),
            (f"{self.datacell} convert --input employees.xlsx --output {self.output_dir}/xlsx_to_csv.csv", "Excel → CSV"),
            (f"{self.datacell} convert --input employees.parquet --output {self.output_dir}/parquet_to_csv.csv", "Parquet → CSV"),
            (f"{self.datacell} convert --input employees.avro --output {self.output_dir}/avro_to_csv.csv", "Avro → CSV"),
        ]
        
        for cmd, desc in tests:
            self.run_command(cmd, desc, "Conversions")
    
    def test_formulas(self):
        """Test formula evaluation"""
        print("\n=== 3. Formula Evaluation Tests ===\n")
        
        tests = [
            (f"{self.datacell} formula --input numbers.csv --output {self.output_dir}/formula_add.csv --formula 'A1+B1' --cell C1", "Arithmetic: A1+B1"),
            (f"{self.datacell} formula --input sales.csv --output {self.output_dir}/formula_sum.csv --formula 'SUM(C2:C10)' --cell C11", "SUM function"),
            (f"{self.datacell} formula --input sales.csv --output {self.output_dir}/formula_avg.csv --formula 'AVERAGE(C2:C10)' --cell D11", "AVERAGE function"),
            (f"{self.datacell} formula --input sales.csv --output {self.output_dir}/formula_if.csv --formula 'IF(C2>1000,\"High\",\"Low\")' --cell D2", "IF function"),
        ]
        
        for cmd, desc in tests:
            self.run_command(cmd, desc, "Formulas")
    
    def test_operations(self):
        """Test data operations"""
        print("\n=== 4. Data Operations Tests ===\n")
        
        tests = [
            (f"{self.datacell} sort --input sales.csv --output {self.output_dir}/sorted.csv --column Amount", "Sort ascending"),
            (f"{self.datacell} sort --input sales.csv --output {self.output_dir}/sorted_desc.csv --column Amount --descending", "Sort descending"),
            (f"{self.datacell} filter --input sales.csv --output {self.output_dir}/filtered.csv --where 'Amount > 1000'", "Filter data"),
            (f"{self.datacell} dedupe --input duplicates.csv --output {self.output_dir}/deduped.csv", "Remove duplicates"),
            (f"{self.datacell} transpose --input employees.csv --output {self.output_dir}/transposed.csv", "Transpose data"),
        ]
        
        for cmd, desc in tests:
            self.run_command(cmd, desc, "Operations")
    
    def test_pandas_ops(self):
        """Test pandas-style operations"""
        print("\n=== 5. Pandas-Style Operations Tests ===\n")
        
        tests = [
            (f"{self.datacell} head --input employees.csv -n 3 > {self.output_dir}/head.txt", "Head"),
            (f"{self.datacell} tail --input employees.csv -n 3 > {self.output_dir}/tail.txt", "Tail"),
            (f"{self.datacell} select --input employees.csv --output {self.output_dir}/selected.csv --columns 'Name,Department'", "Select columns"),
            (f"{self.datacell} describe --input financial_data.csv --format markdown > {self.output_dir}/describe.md", "Describe statistics"),
            (f"{self.datacell} groupby --input sales.csv --output {self.output_dir}/grouped.csv --by Category --agg 'sum:Amount'", "Group by"),
        ]
        
        for cmd, desc in tests:
            self.run_command(cmd, desc, "Pandas Ops")
    
    def generate_report(self):
        """Generate test report"""
        print("\n=== Test Summary ===\n")
        
        total = self.passed + self.failed
        pass_rate = (self.passed / total * 100) if total > 0 else 0
        
        print(f"Total Tests: {total}")
        print(f"Passed: {self.passed} ({pass_rate:.1f}%)")
        print(f"Failed: {self.failed}")
        print()
        
        # Group by category
        categories = {}
        for result in self.results:
            cat = result["category"]
            if cat not in categories:
                categories[cat] = {"passed": 0, "failed": 0}
            
            if result["status"] == "PASS":
                categories[cat]["passed"] += 1
            else:
                categories[cat]["failed"] += 1
        
        print("Results by Category:")
        for cat, stats in categories.items():
            total_cat = stats["passed"] + stats["failed"]
            rate = (stats["passed"] / total_cat * 100) if total_cat > 0 else 0
            print(f"  {cat}: {stats['passed']}/{total_cat} ({rate:.1f}%)")
        
        # Save JSON report
        report = {
            "timestamp": datetime.now().isoformat(),
            "summary": {
                "total": total,
                "passed": self.passed,
                "failed": self.failed,
                "pass_rate": pass_rate
            },
            "categories": categories,
            "results": self.results
        }
        
        report_path = f"{self.output_dir}/test_report.json"
        with open(report_path, "w") as f:
            json.dump(report, f, indent=2)
        
        print(f"\nDetailed report saved to: {report_path}")
        
        # Show failed tests
        if self.failed > 0:
            print("\n=== Failed Tests ===\n")
            for result in self.results:
                if result["status"] != "PASS":
                    print(f"✗ {result['description']}")
                    print(f"  Command: {result['command']}")
                    if result["stderr"]:
                        print(f"  Error: {result['stderr'][:100]}")
                    print()
    
    def run_all(self):
        """Run all tests"""
        self.setup()
        self.test_file_io()
        self.test_conversions()
        self.test_formulas()
        self.test_operations()
        self.test_pandas_ops()
        self.generate_report()
        
        return self.failed == 0

def main():
    runner = TestRunner()
    success = runner.run_all()
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
