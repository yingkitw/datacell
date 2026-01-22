use anyhow::Result;
use rust_xlsxwriter::{Chart, ChartSolidFill, ChartType, Workbook};

use super::reader::ExcelHandler;
use super::types::CellStyle;

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

    fn to_xlsx_type(&self) -> ChartType {
        match self {
            DataChartType::Bar => ChartType::Bar,
            DataChartType::Column => ChartType::Column,
            DataChartType::Line => ChartType::Line,
            DataChartType::Area => ChartType::Area,
            DataChartType::Pie => ChartType::Pie,
            DataChartType::Scatter => ChartType::Scatter,
            DataChartType::Doughnut => ChartType::Doughnut,
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
        path: &str,
        data: &[Vec<String>],
        chart_config: &ChartConfig,
    ) -> Result<()> {
        if data.is_empty() {
            anyhow::bail!("No data to chart");
        }

        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        worksheet.set_name("Data")?;

        let header_format = CellStyle::header().to_format();
        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if row_idx == 0 {
                    worksheet.write_string_with_format(
                        row_idx as u32,
                        col_idx as u16,
                        cell,
                        &header_format,
                    )?;
                } else if let Ok(num) = cell.parse::<f64>() {
                    worksheet.write_number(row_idx as u32, col_idx as u16, num)?;
                } else {
                    worksheet.write_string(row_idx as u32, col_idx as u16, cell)?;
                }
            }
        }

        let mut chart = Chart::new(chart_config.chart_type.to_xlsx_type());

        if let Some(ref title) = chart_config.title {
            chart.title().set_name(title);
        }
        if let Some(ref x_title) = chart_config.x_axis_title {
            chart.x_axis().set_name(x_title);
        }
        if let Some(ref y_title) = chart_config.y_axis_title {
            chart.y_axis().set_name(y_title);
        }

        let num_rows = data.len();
        let header = &data[0];

        let default_colors = vec![
            "4472C4", "ED7D31", "A5A5A5", "FFC000", "5B9BD5", "70AD47", "264478", "9E480E",
            "636363", "997300",
        ];

        for (series_idx, &col_idx) in chart_config.value_columns.iter().enumerate() {
            if col_idx >= header.len() {
                continue;
            }

            let series_name = header
                .get(col_idx)
                .cloned()
                .unwrap_or_else(|| format!("Series {}", series_idx + 1));
            let cat_col = chart_config.category_column;

            let color_hex = chart_config
                .colors
                .as_ref()
                .and_then(|c| c.get(series_idx))
                .map(|s| s.as_str())
                .unwrap_or_else(|| {
                    default_colors
                        .get(series_idx % default_colors.len())
                        .unwrap_or(&"4472C4")
                });

            let series = chart.add_series();
            series
                .set_name(&series_name)
                .set_categories((
                    "Data",
                    1,
                    cat_col as u16,
                    (num_rows - 1) as u32,
                    cat_col as u16,
                ))
                .set_values((
                    "Data",
                    1,
                    col_idx as u16,
                    (num_rows - 1) as u32,
                    col_idx as u16,
                ));

            if let Ok(color) = CellStyle::parse_hex_color(color_hex) {
                series.set_format(ChartSolidFill::new().set_color(color));
            }
        }

        chart.set_width(chart_config.width);
        chart.set_height(chart_config.height);

        if !chart_config.show_legend {
            chart.legend().set_hidden();
        }

        let chart_row = (num_rows + 2) as u32;
        worksheet.insert_chart(chart_row, 0, &chart)?;

        workbook.save(path)?;
        Ok(())
    }

    pub fn add_chart_to_data(
        &self,
        data: &[Vec<String>],
        chart_config: &ChartConfig,
        output_path: &str,
    ) -> Result<()> {
        self.write_with_chart(output_path, data, chart_config)
    }
}
