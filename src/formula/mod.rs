//! Formula evaluation module
//!
//! Supports Excel-like formulas: SUM, AVERAGE, MIN, MAX, COUNT, IF, CONCAT, VLOOKUP, etc.

mod types;
mod evaluator;
mod functions;
mod parser;

#[allow(unused_imports)]
pub use types::FormulaResult;
pub use evaluator::FormulaEvaluator;
