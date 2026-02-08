use anyhow::Result;

use super::reader::ExcelHandler;

/// Chart type for visualization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataChartType {
    Bar,
    Column,
    Line,
    Area,
    Pie,
    Scatter,
    Doughnut,
}

impl DataChartType {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "bar" => Ok(DataChartType::Bar),
            "column" => Ok(DataChartType::Column),
            "line" => Ok(DataChartType::Line),
            "area" => Ok(DataChartType::Area),
            "pie" => Ok(DataChartType::Pie),
            "scatter" => Ok(DataChartType::Scatter),
            "doughnut" | "donut" => Ok(DataChartType::Doughnut),
            _ => anyhow::bail!(
                "Unknown chart type: {}. Use: bar, column, line, area, pie, scatter, doughnut",
                s
            ),
        }
    }
}

/// Chart configuration
#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub chart_type: DataChartType,
    pub title: Option<String>,
    pub x_axis_title: Option<String>,
    pub y_axis_title: Option<String>,
    pub category_column: usize,
    pub value_columns: Vec<usize>,
    pub width: u32,
    pub height: u32,
    pub show_legend: bool,
    pub colors: Option<Vec<String>>,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            chart_type: DataChartType::Column,
            title: None,
            x_axis_title: None,
            y_axis_title: None,
            category_column: 0,
            value_columns: vec![1],
            width: 600,
            height: 400,
            show_legend: true,
            colors: None,
        }
    }
}

impl ExcelHandler {
    pub fn write_with_chart(
        &self,
        _path: &str,
        _data: &[Vec<String>],
        _chart_config: &ChartConfig,
    ) -> Result<()> {
        // Chart XML generation is complex and requires additional XML namespaces
        // For the custom XLSX writer, chart support requires implementing:
        // 1. xl/drawings/drawing1.xml
        // 2. xl/drawings/_rels/drawing1.xml.rels
        // 3. xl/charts/chart1.xml
        // 4. xl/charts/_rels/chart1.xml.rels
        // 5. xl/worksheets/_rels/sheet1.xml.rels
        // This will be implemented in a future update
        anyhow::bail!(
            "Chart support is not yet implemented in the custom XLSX writer. \
            Charts require complex XML drawing markup which is planned for a future release."
        );
    }

    pub fn add_chart_to_data(
        &self,
        _data: &[Vec<String>],
        _chart_config: &ChartConfig,
        _output_path: &str,
    ) -> Result<()> {
        anyhow::bail!(
            "Chart support is not yet implemented in the custom XLSX writer. \
            Charts require complex XML drawing markup which is planned for a future release."
        );
    }
}
