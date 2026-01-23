//! Barcode components
//! Inspired by JasperReports Barcode (Barbecue/Barcode4j) and Pentaho Barcode

use super::Component;
use serde::{Deserialize, Serialize};

/// Barcode format types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BarcodeFormat {
    /// Code 128 (variable length, alphanumeric)
    Code128,
    /// Code 39 (variable length, alphanumeric)
    Code39,
    /// EAN-13 (13 digits, retail products)
    Ean13,
    /// EAN-8 (8 digits, small products)
    Ean8,
    /// UPC-A (12 digits, retail products)
    UpcA,
    /// UPC-E (6 digits, compressed UPC)
    UpcE,
    /// QR Code (2D, high capacity)
    QrCode,
    /// Data Matrix (2D, compact)
    DataMatrix,
    /// PDF417 (2D, high capacity)
    Pdf417,
    /// ITF (Interleaved 2 of 5)
    Itf,
    /// Codabar
    Codabar,
}

/// Barcode component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barcode {
    /// Data to encode
    pub data: String,
    /// Barcode format
    pub format: BarcodeFormat,
    /// Width
    #[serde(default = "default_barcode_width")]
    pub width: String,
    /// Height
    #[serde(default = "default_barcode_height")]
    pub height: String,
    /// Show text below barcode
    #[serde(default = "default_true")]
    pub show_text: bool,
    /// Error correction level for QR codes (L, M, Q, H)
    #[serde(default)]
    pub error_correction: Option<String>,
}

fn default_barcode_width() -> String {
    "150pt".into()
}

fn default_barcode_height() -> String {
    "50pt".into()
}

fn default_true() -> bool {
    true
}

impl Barcode {
    pub fn new(data: impl Into<String>, format: BarcodeFormat) -> Self {
        Self {
            data: data.into(),
            format,
            width: "150pt".into(),
            height: "50pt".into(),
            show_text: true,
            error_correction: None,
        }
    }

    /// Create Code 128 barcode (most common 1D barcode)
    pub fn code128(data: impl Into<String>) -> Self {
        Self::new(data, BarcodeFormat::Code128)
    }

    /// Create EAN-13 barcode (retail products)
    pub fn ean13(data: impl Into<String>) -> Self {
        Self::new(data, BarcodeFormat::Ean13)
    }

    /// Create QR code
    pub fn qr_code(data: impl Into<String>) -> Self {
        Self {
            width: "100pt".into(),
            height: "100pt".into(),
            error_correction: Some("M".into()),
            ..Self::new(data, BarcodeFormat::QrCode)
        }
    }

    /// Create Data Matrix (2D compact)
    pub fn data_matrix(data: impl Into<String>) -> Self {
        Self {
            width: "80pt".into(),
            height: "80pt".into(),
            ..Self::new(data, BarcodeFormat::DataMatrix)
        }
    }

    pub fn with_size(mut self, width: impl Into<String>, height: impl Into<String>) -> Self {
        self.width = width.into();
        self.height = height.into();
        self
    }

    pub fn hide_text(mut self) -> Self {
        self.show_text = false;
        self
    }

    pub fn with_error_correction(mut self, level: impl Into<String>) -> Self {
        self.error_correction = Some(level.into());
        self
    }
}

impl Component for Barcode {
    fn component_id(&self) -> &'static str {
        "barcode"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
