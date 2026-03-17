//! Barcode Demo - Comprehensive Barcode Generation Example
//!
//! This example demonstrates all barcode formats supported in RenderReport:
//! - 1D Barcodes: Code128, Code39, EAN-13, EAN-8, UPC-A, UPC-E, ITF, Codabar
//! - 2D Barcodes: QR Code, Data Matrix, PDF417
//!
//! Inspired by JasperReports Barcode Component and Pentaho Barcode Elements
//!
//! Run with: cargo run --example barcode_demo

use renderreport::components::barcode::{Barcode, BarcodeFormat};
use renderreport::components::text::Label;
use renderreport::components::{Component, Section};
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    println!("Generating comprehensive barcode demonstration report...");

    let engine = Engine::new()?;

    // Helper function to add components
    let add_comp = |component: &dyn Component| -> serde_json::Value {
        serde_json::json!({
            "type": component.component_id(),
            "data": component.to_data()
        })
    };

    let report = engine
        .report("default")
        .title("Barcode Generation Showcase")
        .subtitle("Complete Guide to 1D and 2D Barcodes")
        // ============================================
        // SECTION 1: Code128 - Most Versatile 1D Barcode
        // ============================================
        .add_component(Section::new("Code 128 - Universal 1D Barcode"))
        .add_raw_component(add_comp(&Label::new(
            "Code 128 is the most commonly used 1D barcode. It can encode all ASCII characters \
             and is widely used in shipping, packaging, and inventory management.",
        )))
        .add_raw_component(add_comp(&Label::new("Product SKU:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::code128("PROD-2024-XYZ-12345")))
        .add_raw_component(add_comp(&Label::new("Shipment Tracking:").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &Barcode::code128("1Z999AA10123456784").with_size("180pt", "60pt"),
        ))
        .add_raw_component(add_comp(&Label::new("Serial Number:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::code128("SN-A1B2C3D4E5F6")))
        // ============================================
        // SECTION 2: Code39 - Alphanumeric Industrial Code
        // ============================================
        .add_component(Section::new("Code 39 - Industrial Standard"))
        .add_raw_component(add_comp(&Label::new(
            "Code 39 supports alphanumeric characters and is commonly used in automotive, \
             defense, and healthcare industries.",
        )))
        .add_raw_component(add_comp(&Label::new("Asset Tag:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::new("ASSET-2024-001", BarcodeFormat::Code39)))
        .add_raw_component(add_comp(&Label::new("Equipment ID:").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &Barcode::new("EQ-MFG-789", BarcodeFormat::Code39).with_size("160pt", "55pt"),
        ))
        // ============================================
        // SECTION 3: EAN-13 - Retail Product Codes
        // ============================================
        .add_component(Section::new("EAN-13 - European Article Number"))
        .add_raw_component(add_comp(&Label::new(
            "EAN-13 is the standard barcode for retail products worldwide. It encodes \
             13 digits and includes country code, manufacturer code, and product code.",
        )))
        .add_raw_component(add_comp(&Label::new("Consumer Product:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::ean13("5901234123457")))
        .add_raw_component(add_comp(&Label::new("Electronics Item:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::ean13("4006381333931")))
        .add_raw_component(add_comp(&Label::new("Food Product:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::ean13("8712345678906")))
        // ============================================
        // SECTION 4: EAN-8 - Compact Retail Code
        // ============================================
        .add_component(Section::new("EAN-8 - Compact Product Code"))
        .add_raw_component(add_comp(&Label::new(
            "EAN-8 is used for small products where EAN-13 would be too large. \
             It encodes 8 digits.",
        )))
        .add_raw_component(add_comp(&Label::new("Small Package:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::new("12345670", BarcodeFormat::Ean8)))
        .add_raw_component(add_comp(&Label::new("Travel Size:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::new("96385074", BarcodeFormat::Ean8)))
        // ============================================
        // SECTION 5: UPC-A - North American Retail
        // ============================================
        .add_component(Section::new("UPC-A - Universal Product Code"))
        .add_raw_component(add_comp(&Label::new(
            "UPC-A is the standard retail barcode in North America. It encodes 12 digits.",
        )))
        .add_raw_component(add_comp(&Label::new("US Product:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::new("012345678905", BarcodeFormat::UpcA)))
        .add_raw_component(add_comp(&Label::new("Grocery Item:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::new("042100005264", BarcodeFormat::UpcA)))
        // ============================================
        // SECTION 6: UPC-E - Compressed UPC
        // ============================================
        .add_component(Section::new("UPC-E - Zero-Suppressed UPC"))
        .add_raw_component(add_comp(&Label::new(
            "UPC-E is a compressed version of UPC-A for small packages. \
             It encodes 6 digits that expand to 12.",
        )))
        .add_raw_component(add_comp(&Label::new("Compact Product:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::new("123456", BarcodeFormat::UpcE)))
        // ============================================
        // SECTION 7: ITF - Shipping and Logistics
        // ============================================
        .add_component(Section::new("ITF - Interleaved 2 of 5"))
        .add_raw_component(add_comp(&Label::new(
            "ITF is used for shipping containers and cases. It's commonly seen on \
             corrugated cardboard boxes.",
        )))
        .add_raw_component(add_comp(&Label::new("Carton Code:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::new("12345678901234", BarcodeFormat::Itf)))
        // ============================================
        // SECTION 8: Codabar - Medical and Library
        // ============================================
        .add_component(Section::new("Codabar - Healthcare & Libraries"))
        .add_raw_component(add_comp(&Label::new(
            "Codabar is used in blood banks, libraries, and package tracking. \
             It supports digits and a few special characters.",
        )))
        .add_raw_component(add_comp(&Label::new("Library Book:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::new("A1234567B", BarcodeFormat::Codabar)))
        // ============================================
        // SECTION 9: QR Code - High Capacity 2D
        // ============================================
        .add_component(Section::new("QR Code - Quick Response Code"))
        .add_raw_component(add_comp(&Label::new(
            "QR codes can store large amounts of data including URLs, contact information, \
             and structured data. They support error correction levels L, M, Q, H.",
        )))
        .add_raw_component(add_comp(&Label::new("Website URL:").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &Barcode::qr_code("https://github.com/casoon/renderreport").with_error_correction("M"),
        ))
        .add_raw_component(add_comp(&Label::new("Product Information:").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &Barcode::qr_code("PRODUCT:XYZ-123|PRICE:$99.99|MFG:2024-03-15")
                .with_error_correction("H"),
        ))
        .add_raw_component(add_comp(&Label::new("Contact Card (vCard):").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &Barcode::qr_code(
                "BEGIN:VCARD\nVERSION:3.0\nFN:John Doe\nTEL:+1-555-0123\nEMAIL:john@example.com\nEND:VCARD"
            )
            .with_error_correction("M")
            .with_size("120pt", "120pt"),
        ))
        .add_raw_component(add_comp(&Label::new("WiFi Configuration:").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &Barcode::qr_code("WIFI:T:WPA;S:MyNetwork;P:SecurePassword123;;")
                .with_error_correction("Q"),
        ))
        // ============================================
        // SECTION 10: Data Matrix - Compact 2D
        // ============================================
        .add_component(Section::new("Data Matrix - Compact 2D Code"))
        .add_raw_component(add_comp(&Label::new(
            "Data Matrix is a very compact 2D code often used in electronics manufacturing, \
             pharmaceuticals, and small item tracking.",
        )))
        .add_raw_component(add_comp(&Label::new("Component Serial:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::data_matrix("COMP-2024-SN-987654321")))
        .add_raw_component(add_comp(&Label::new("Batch Number:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Barcode::data_matrix("BATCH:A123|EXP:2025-12-31")))
        // ============================================
        // SECTION 11: Additional 2D Examples
        // ============================================
        .add_component(Section::new("2D Codes - Real-World Applications"))
        .add_raw_component(add_comp(&Label::new(
            "2D barcodes are widely used for inventory, logistics, and digital interactions. \
             Here are practical examples with QR Code and Data Matrix formats.",
        )))
        .add_raw_component(add_comp(&Label::new("Event Ticket:").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &Barcode::qr_code("TICKET:EVT-2026-0317|SEAT:A12|GATE:3|TIME:19:30")
                .with_size("120pt", "120pt"),
        ))
        .add_raw_component(add_comp(&Label::new("Pharmaceutical Package:").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &Barcode::data_matrix("NDC:12345-678-90|LOT:X42B|EXP:2027-06")
                .with_size("100pt", "100pt"),
        ))
        .add_raw_component(add_comp(&Label::new("Asset Tag:").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &Barcode::data_matrix("ASSET:LAP-2024-0891|DEPT:ENG|PURCHASED:2024-03-15"),
        ))
        .build();

    // Render to PDF
    let pdf = engine.render_pdf(&report)?;

    // Write to file
    let output_path = "examples/output/barcode_demo.pdf";
    std::fs::write(output_path, &pdf)?;

    println!("✓ Barcode demonstration report generated successfully!");
    println!("  Output: {}", output_path);
    println!("  Size: {} KB", pdf.len() / 1024);
    println!("\nBarcode formats demonstrated:");
    println!("  1D Barcodes:");
    println!("    ✓ Code 128 (universal, alphanumeric)");
    println!("    ✓ Code 39 (industrial standard)");
    println!("    ✓ EAN-13 (retail products, 13 digits)");
    println!("    ✓ EAN-8 (compact retail, 8 digits)");
    println!("    ✓ UPC-A (North American retail, 12 digits)");
    println!("    ✓ UPC-E (compressed UPC, 6 digits)");
    println!("    ✓ ITF (shipping containers)");
    println!("    ✓ Codabar (healthcare, libraries)");
    println!("  2D Barcodes:");
    println!("    ✓ QR Code (high capacity, error correction)");
    println!("    ✓ Data Matrix (compact, electronics)");

    Ok(())
}
