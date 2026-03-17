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

    /// Encode the barcode data into a binary pattern for 1D barcodes
    fn encode_1d(&self) -> Option<Vec<u8>> {
        match self.format {
            BarcodeFormat::Code128 => {
                // Code128 requires a charset prefix; default to charset B for general alphanumeric
                let prefixed = if self.data.starts_with('\u{00C0}')
                    || self.data.starts_with('\u{0181}')
                    || self.data.starts_with('\u{0106}')
                {
                    self.data.clone()
                } else {
                    format!("\u{0181}{}", self.data)
                };
                barcoders::sym::code128::Code128::new(&prefixed)
                    .ok()
                    .map(|b| b.encode())
            }
            BarcodeFormat::Code39 => barcoders::sym::code39::Code39::new(&self.data)
                .ok()
                .map(|b| b.encode()),
            BarcodeFormat::Ean13 => barcoders::sym::ean13::EAN13::new(&self.data)
                .ok()
                .map(|b| b.encode()),
            BarcodeFormat::Ean8 => barcoders::sym::ean8::EAN8::new(&self.data)
                .ok()
                .map(|b| b.encode()),
            BarcodeFormat::Codabar => barcoders::sym::codabar::Codabar::new(&self.data)
                .ok()
                .map(|b| b.encode()),
            BarcodeFormat::Itf => barcoders::sym::tf::TF::interleaved(&self.data)
                .ok()
                .map(|b| b.encode()),
            // UPC-A and UPC-E: encode via EAN13 with leading zero (UPC-A is a subset of EAN-13)
            BarcodeFormat::UpcA => {
                let ean = if self.data.len() == 12 {
                    format!("0{}", &self.data[..12])
                } else {
                    format!("0{}", self.data)
                };
                barcoders::sym::ean13::EAN13::new(&ean)
                    .ok()
                    .map(|b| b.encode())
            }
            BarcodeFormat::UpcE => {
                // UPC-E doesn't have direct support; fall back to placeholder
                None
            }
            _ => None,
        }
    }

    /// Encode QR code data into a 2D module matrix
    fn encode_qr(&self) -> Option<(Vec<Vec<u8>>, usize)> {
        use qrcode::EcLevel;

        let ec = match self.error_correction.as_deref() {
            Some("L") => EcLevel::L,
            Some("Q") => EcLevel::Q,
            Some("H") => EcLevel::H,
            _ => EcLevel::M,
        };

        qrcode::QrCode::with_error_correction_level(self.data.as_bytes(), ec)
            .ok()
            .map(|code| {
                let width = code.width();
                let colors = code.to_colors();
                let matrix: Vec<Vec<u8>> = colors
                    .chunks(width)
                    .map(|row| {
                        row.iter()
                            .map(|c| if *c == qrcode::Color::Dark { 1 } else { 0 })
                            .collect()
                    })
                    .collect();
                (matrix, width)
            })
    }
}

impl Component for Barcode {
    fn component_id(&self) -> &'static str {
        "barcode"
    }

    fn to_data(&self) -> serde_json::Value {
        let mut val = serde_json::to_value(self).unwrap_or_default();
        let obj = val.as_object_mut().unwrap();

        match self.format {
            BarcodeFormat::QrCode => {
                if let Some((matrix, width)) = self.encode_qr() {
                    obj.insert("encoding_2d".into(), serde_json::json!(matrix));
                    obj.insert("qr_width".into(), serde_json::json!(width));
                }
            }
            BarcodeFormat::DataMatrix | BarcodeFormat::Pdf417 => {
                // No Rust crate available for these; template will show placeholder
                obj.insert("unsupported".into(), serde_json::json!(true));
            }
            _ => {
                if let Some(bars) = self.encode_1d() {
                    let bars_json: Vec<serde_json::Value> =
                        bars.iter().map(|&b| serde_json::json!(b)).collect();
                    obj.insert("encoding".into(), serde_json::json!(bars_json));
                }
            }
        }

        val
    }
}
