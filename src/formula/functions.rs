//! Formula function implementations

use anyhow::Result;
use super::evaluator::FormulaEvaluator;

impl FormulaEvaluator {
    pub(crate) fn evaluate_sum(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let range = self.extract_range(formula)?;
        let mut sum = 0.0;

        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if let Some(value) = self.get_cell_value_by_index(row, col, data) {
                    sum += value;
                }
            }
        }

        Ok(sum)
    }

    pub(crate) fn evaluate_average(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let range = self.extract_range(formula)?;
        let mut sum = 0.0;
        let mut count = 0;

        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if let Some(value) = self.get_cell_value_by_index(row, col, data) {
                    sum += value;
                    count += 1;
                }
            }
        }

        if count == 0 {
            Ok(0.0)
        } else {
            Ok(sum / count as f64)
        }
    }

    pub(crate) fn evaluate_min(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let range = self.extract_range(formula)?;
        let mut min: Option<f64> = None;

        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if let Some(value) = self.get_cell_value_by_index(row, col, data) {
                    min = Some(min.map_or(value, |m| m.min(value)));
                }
            }
        }

        min.ok_or_else(|| anyhow::anyhow!("No numeric values found in range"))
    }

    pub(crate) fn evaluate_max(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let range = self.extract_range(formula)?;
        let mut max: Option<f64> = None;

        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if let Some(value) = self.get_cell_value_by_index(row, col, data) {
                    max = Some(max.map_or(value, |m| m.max(value)));
                }
            }
        }

        max.ok_or_else(|| anyhow::anyhow!("No numeric values found in range"))
    }

    pub(crate) fn evaluate_count(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let range = self.extract_range(formula)?;
        let mut count = 0;

        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if self.get_cell_value_by_index(row, col, data).is_some() {
                    count += 1;
                }
            }
        }

        Ok(count as f64)
    }
    
    pub(crate) fn evaluate_round(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let inner = self.extract_function_args(formula)?;
        let args = self.split_args(&inner)?;
        
        if args.is_empty() || args.len() > 2 {
            anyhow::bail!("ROUND requires 1-2 arguments: ROUND(value, [decimals])");
        }
        
        let value = self.evaluate_formula(&args[0], data)?;
        let decimals = if args.len() > 1 {
            self.evaluate_formula(&args[1], data)? as i32
        } else {
            0
        };
        
        let multiplier = 10f64.powi(decimals);
        Ok((value * multiplier).round() / multiplier)
    }
    
    pub(crate) fn evaluate_abs(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let inner = self.extract_function_args(formula)?;
        let value = self.evaluate_formula(&inner, data)?;
        Ok(value.abs())
    }
    
    pub(crate) fn evaluate_len(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let inner = self.extract_function_args(formula)?;
        let inner = inner.trim().to_uppercase();
        
        if let Ok((row, col)) = self.parse_cell_reference(&inner) {
            if let Some(text) = self.get_cell_text_by_index(row, col, data) {
                return Ok(text.len() as f64);
            }
        }
        
        let text = inner.trim_matches('"');
        Ok(text.len() as f64)
    }
    
    pub(crate) fn evaluate_vlookup(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let inner = self.extract_function_args(formula)?;
        let args = self.split_args(&inner)?;
        
        if args.len() < 3 || args.len() > 4 {
            anyhow::bail!("VLOOKUP requires 3-4 arguments: VLOOKUP(lookup_value, range, col_index, [exact_match])");
        }
        
        let lookup_value = if let Ok(num) = self.evaluate_formula(&args[0], data) {
            num.to_string()
        } else {
            args[0].trim().trim_matches('"').to_string()
        };
        
        let range = self.extract_range(&format!("X({})", args[1]))?;
        let col_index = self.evaluate_formula(&args[2], data)? as usize;
        if col_index < 1 {
            anyhow::bail!("VLOOKUP col_index must be >= 1");
        }
        
        for row in range.start_row..=range.end_row {
            if let Some(cell_text) = self.get_cell_text_by_index(row, range.start_col, data) {
                let matches = if let (Ok(cell_num), Ok(lookup_num)) = (cell_text.parse::<f64>(), lookup_value.parse::<f64>()) {
                    (cell_num - lookup_num).abs() < f64::EPSILON
                } else {
                    cell_text.to_uppercase() == lookup_value.to_uppercase()
                };
                
                if matches {
                    let result_col = range.start_col + (col_index as u16 - 1);
                    if let Some(value) = self.get_cell_value_by_index(row, result_col, data) {
                        return Ok(value);
                    } else if let Some(text) = self.get_cell_text_by_index(row, result_col, data) {
                        if let Ok(num) = text.parse::<f64>() {
                            return Ok(num);
                        }
                    }
                    anyhow::bail!("VLOOKUP: value at result column is not numeric");
                }
            }
        }
        
        anyhow::bail!("VLOOKUP: no match found for '{}'", lookup_value)
    }
    
    pub(crate) fn evaluate_sumif(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let inner = self.extract_function_args(formula)?;
        let args = self.split_args(&inner)?;
        
        if args.len() < 2 || args.len() > 3 {
            anyhow::bail!("SUMIF requires 2-3 arguments: SUMIF(range, criteria, [sum_range])");
        }
        
        let criteria_range = self.extract_range(&format!("X({})", args[0]))?;
        let criteria = args[1].trim().trim_matches('"').to_string();
        
        let sum_range = if args.len() == 3 {
            self.extract_range(&format!("X({})", args[2]))?
        } else {
            criteria_range.clone()
        };
        
        let mut sum = 0.0;
        
        for row_offset in 0..=(criteria_range.end_row - criteria_range.start_row) {
            let criteria_row = criteria_range.start_row + row_offset;
            let sum_row = sum_range.start_row + row_offset;
            
            for col_offset in 0..=(criteria_range.end_col - criteria_range.start_col) {
                let criteria_col = criteria_range.start_col + col_offset;
                let sum_col = sum_range.start_col + col_offset;
                
                if let Some(cell_text) = self.get_cell_text_by_index(criteria_row, criteria_col, data) {
                    let matches = self.matches_criteria(&cell_text, &criteria);
                    
                    if matches {
                        if let Some(value) = self.get_cell_value_by_index(sum_row, sum_col, data) {
                            sum += value;
                        }
                    }
                }
            }
        }
        
        Ok(sum)
    }
    
    pub(crate) fn evaluate_countif(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let inner = self.extract_function_args(formula)?;
        let args = self.split_args(&inner)?;
        
        if args.len() != 2 {
            anyhow::bail!("COUNTIF requires 2 arguments: COUNTIF(range, criteria)");
        }
        
        let range = self.extract_range(&format!("X({})", args[0]))?;
        let criteria = args[1].trim().trim_matches('"').to_string();
        
        let mut count = 0;
        
        for row in range.start_row..=range.end_row {
            for col in range.start_col..=range.end_col {
                if let Some(cell_text) = self.get_cell_text_by_index(row, col, data) {
                    if self.matches_criteria(&cell_text, &criteria) {
                        count += 1;
                    }
                }
            }
        }
        
        Ok(count as f64)
    }
    
    pub(crate) fn matches_criteria(&self, value: &str, criteria: &str) -> bool {
        let criteria = criteria.trim();
        
        if criteria.starts_with(">=") {
            if let (Ok(v), Ok(c)) = (value.parse::<f64>(), criteria[2..].trim().parse::<f64>()) {
                return v >= c;
            }
        } else if criteria.starts_with("<=") {
            if let (Ok(v), Ok(c)) = (value.parse::<f64>(), criteria[2..].trim().parse::<f64>()) {
                return v <= c;
            }
        } else if criteria.starts_with("<>") || criteria.starts_with("!=") {
            let c = criteria[2..].trim();
            return value != c;
        } else if criteria.starts_with('>') {
            if let (Ok(v), Ok(c)) = (value.parse::<f64>(), criteria[1..].trim().parse::<f64>()) {
                return v > c;
            }
        } else if criteria.starts_with('<') {
            if let (Ok(v), Ok(c)) = (value.parse::<f64>(), criteria[1..].trim().parse::<f64>()) {
                return v < c;
            }
        } else if criteria.starts_with('=') {
            let c = criteria[1..].trim();
            return value == c;
        }
        
        // Exact match
        value.to_uppercase() == criteria.to_uppercase()
    }
    
    pub(crate) fn evaluate_arithmetic(&self, formula: &str, data: &[Vec<String>]) -> Result<f64> {
        let cell_ref_regex = regex::Regex::new(r"([A-Z]+[0-9]+)")?;
        
        let mut expr = formula.to_string();
        for cap in cell_ref_regex.captures_iter(formula) {
            let cell_ref = &cap[1];
            let value = self.get_cell_value(cell_ref, data)?;
            expr = expr.replace(cell_ref, &value.to_string());
        }
        
        self.evaluate_simple_arithmetic(&expr)
    }
    
    fn evaluate_simple_arithmetic(&self, expr: &str) -> Result<f64> {
        let expr = expr.replace(" ", "");
        
        if let Ok(num) = expr.parse::<f64>() {
            return Ok(num);
        }
        
        // Handle + and - (left to right, lowest precedence)
        let mut depth = 0;
        for (i, c) in expr.chars().rev().enumerate() {
            let pos = expr.len() - 1 - i;
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '+' if depth == 0 && pos > 0 => {
                    let left = self.evaluate_simple_arithmetic(&expr[..pos])?;
                    let right = self.evaluate_simple_arithmetic(&expr[pos+1..])?;
                    return Ok(left + right);
                }
                '-' if depth == 0 && pos > 0 => {
                    let left = self.evaluate_simple_arithmetic(&expr[..pos])?;
                    let right = self.evaluate_simple_arithmetic(&expr[pos+1..])?;
                    return Ok(left - right);
                }
                _ => {}
            }
        }
        
        // Handle * and /
        depth = 0;
        for (i, c) in expr.chars().rev().enumerate() {
            let pos = expr.len() - 1 - i;
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '*' if depth == 0 => {
                    let left = self.evaluate_simple_arithmetic(&expr[..pos])?;
                    let right = self.evaluate_simple_arithmetic(&expr[pos+1..])?;
                    return Ok(left * right);
                }
                '/' if depth == 0 => {
                    let left = self.evaluate_simple_arithmetic(&expr[..pos])?;
                    let right = self.evaluate_simple_arithmetic(&expr[pos+1..])?;
                    if right == 0.0 {
                        anyhow::bail!("Division by zero");
                    }
                    return Ok(left / right);
                }
                _ => {}
            }
        }
        
        // Handle parentheses
        if expr.starts_with('(') && expr.ends_with(')') {
            return self.evaluate_simple_arithmetic(&expr[1..expr.len()-1]);
        }
        
        anyhow::bail!("Cannot evaluate expression: {}", expr)
    }
}
