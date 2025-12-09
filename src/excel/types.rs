use anyhow::Result;
use rust_xlsxwriter::{Format, Color, FormatAlign, FormatBorder};

/// Cell style configuration
#[derive(Debug, Clone, Default)]
pub struct CellStyle {
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Background color (hex without #, e.g., "4472C4")
    pub bg_color: Option<String>,
    /// Font color (hex without #)
    pub font_color: Option<String>,
    /// Font size
    pub font_size: Option<f64>,
    /// Border style
    pub border: bool,
    /// Horizontal alignment
    pub align: Option<String>,
    /// Number format (e.g., "0.00", "#,##0", "yyyy-mm-dd")
    pub number_format: Option<String>,
}

impl CellStyle {
    pub fn header() -> Self {
        Self {
            bold: true,
            bg_color: Some("4472C4".to_string()),
            font_color: Some("FFFFFF".to_string()),
            border: true,
            align: Some("center".to_string()),
            ..Default::default()
        }
    }
    
    pub fn to_format(&self) -> Format {
        let mut format = Format::new();
        
        if self.bold {
            format = format.set_bold();
        }
        if self.italic {
            format = format.set_italic();
        }
        if let Some(ref color) = self.bg_color {
            if let Ok(c) = Self::parse_hex_color(color) {
                format = format.set_background_color(c);
            }
        }
        if let Some(ref color) = self.font_color {
            if let Ok(c) = Self::parse_hex_color(color) {
                format = format.set_font_color(c);
            }
        }
        if let Some(size) = self.font_size {
            format = format.set_font_size(size);
        }
        if self.border {
            format = format
                .set_border(FormatBorder::Thin)
                .set_border_color(Color::Black);
        }
        if let Some(ref align) = self.align {
            format = match align.to_lowercase().as_str() {
                "center" => format.set_align(FormatAlign::Center),
                "right" => format.set_align(FormatAlign::Right),
                "left" => format.set_align(FormatAlign::Left),
                _ => format,
            };
        }
        if let Some(ref num_fmt) = self.number_format {
            format = format.set_num_format(num_fmt);
        }
        
        format
    }
    
    pub(crate) fn parse_hex_color(hex: &str) -> Result<Color> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            anyhow::bail!("Invalid hex color: {}", hex);
        }
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        Ok(Color::RGB(r as u32 * 0x10000 + g as u32 * 0x100 + b as u32))
    }
}

/// Options for styled Excel writing
#[derive(Debug, Clone)]
pub struct WriteOptions {
    /// Sheet name
    pub sheet_name: Option<String>,
    /// Apply header styling to first row
    pub style_header: bool,
    /// Header style
    pub header_style: CellStyle,
    /// Column-specific styles (by index)
    pub column_styles: Option<std::collections::HashMap<usize, CellStyle>>,
    /// Freeze first row
    pub freeze_header: bool,
    /// Enable auto-filter
    pub auto_filter: bool,
    /// Auto-fit column widths
    pub auto_fit: bool,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            sheet_name: None,
            style_header: true,
            header_style: CellStyle::header(),
            column_styles: None,
            freeze_header: true,
            auto_filter: true,
            auto_fit: true,
        }
    }
}
