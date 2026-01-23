use renderreport::components::{barcode::*, charts::*, crosstab::*, text::*, Component};
use std::collections::HashMap;

#[test]
fn test_chart_creation() {
    let chart = Chart::bar("Sales by Quarter")
        .add_series("2024", vec![("Q1".into(), 100.0), ("Q2".into(), 150.0)])
        .with_labels("Quarter", "Sales ($K)");

    assert_eq!(chart.component_id(), "chart");
    assert_eq!(chart.title, "Sales by Quarter");
    assert_eq!(chart.series.len(), 1);
    assert!(chart.x_label.is_some());
    assert!(chart.y_label.is_some());
}

#[test]
fn test_chart_types() {
    let bar_chart = Chart::bar("Bar");
    let line_chart = Chart::line("Line");
    let pie_chart = Chart::pie("Pie");

    assert!(matches!(bar_chart.chart_type, ChartType::Bar));
    assert!(matches!(line_chart.chart_type, ChartType::Line));
    assert!(matches!(pie_chart.chart_type, ChartType::Pie));
}

#[test]
fn test_chart_serialization() {
    let chart = Chart::bar("Test").add_series("Data", vec![("A".into(), 10.0), ("B".into(), 20.0)]);

    let data = chart.to_data();
    assert!(data.is_object());
    assert!(data.get("title").is_some());
    assert!(data.get("series").is_some());
}

#[test]
fn test_sparkline_creation() {
    let sparkline = Sparkline::line(vec![1.0, 2.0, 3.0, 2.5, 4.0]);

    assert_eq!(sparkline.component_id(), "sparkline");
    assert_eq!(sparkline.data.len(), 5);
    assert_eq!(sparkline.sparkline_type, "line");
}

#[test]
fn test_sparkline_bar() {
    let sparkline = Sparkline::bar(vec![10.0, 20.0, 15.0]).with_color("#ff0000");

    assert_eq!(sparkline.sparkline_type, "bar");
    assert_eq!(sparkline.color, Some("#ff0000".into()));
}

#[test]
fn test_gauge_creation() {
    let gauge = Gauge::new("CPU Usage", 75.0);

    assert_eq!(gauge.component_id(), "gauge");
    assert_eq!(gauge.label, "CPU Usage");
    assert_eq!(gauge.value, 75.0);
    assert_eq!(gauge.min, 0.0);
    assert_eq!(gauge.max, 100.0);
}

#[test]
fn test_gauge_thermometer() {
    let gauge = Gauge::thermometer("Temperature", 25.5).with_range(0.0, 50.0);

    assert_eq!(gauge.style, "vertical");
    assert_eq!(gauge.min, 0.0);
    assert_eq!(gauge.max, 50.0);
}

#[test]
fn test_crosstab_creation() {
    let crosstab = Crosstab::new("Region", "Product", "Sales")
        .with_title("Sales Analysis")
        .with_aggregation("sum");

    assert_eq!(crosstab.component_id(), "crosstab");
    assert_eq!(crosstab.row_dimension, "Region");
    assert_eq!(crosstab.column_dimension, "Product");
    assert_eq!(crosstab.measure, "Sales");
    assert_eq!(crosstab.aggregation, "sum");
}

#[test]
fn test_crosstab_with_data() {
    let mut data = HashMap::new();
    data.insert("region".into(), serde_json::json!("North"));
    data.insert("product".into(), serde_json::json!("Widget"));
    data.insert("sales".into(), serde_json::json!(1000));

    let crosstab = Crosstab::new("region", "product", "sales").with_data(vec![data]);

    assert_eq!(crosstab.data.len(), 1);
}

#[test]
fn test_pivot_table_creation() {
    let pivot = PivotTable::new(
        vec!["Row 1".into(), "Row 2".into()],
        vec!["Col A".into(), "Col B".into()],
        vec![
            vec!["100".into(), "200".into()],
            vec!["150".into(), "250".into()],
        ],
    );

    assert_eq!(pivot.component_id(), "pivot-table");
    assert_eq!(pivot.row_headers.len(), 2);
    assert_eq!(pivot.column_headers.len(), 2);
    assert_eq!(pivot.values.len(), 2);
}

#[test]
fn test_barcode_creation() {
    let barcode = Barcode::code128("ABC123");

    assert_eq!(barcode.component_id(), "barcode");
    assert_eq!(barcode.data, "ABC123");
    assert!(matches!(barcode.format, BarcodeFormat::Code128));
    assert!(barcode.show_text);
}

#[test]
fn test_barcode_qr_code() {
    let qr = Barcode::qr_code("https://example.com").with_error_correction("H");

    assert!(matches!(qr.format, BarcodeFormat::QrCode));
    assert_eq!(qr.error_correction, Some("H".into()));
}

#[test]
fn test_barcode_formats() {
    let code39 = Barcode::new("TEST", BarcodeFormat::Code39);
    let ean13 = Barcode::ean13("1234567890123");
    let data_matrix = Barcode::data_matrix("DATA");

    assert!(matches!(code39.format, BarcodeFormat::Code39));
    assert!(matches!(ean13.format, BarcodeFormat::Ean13));
    assert!(matches!(data_matrix.format, BarcodeFormat::DataMatrix));
}

#[test]
fn test_label_creation() {
    let label = Label::new("Hello World");

    assert_eq!(label.component_id(), "label");
    assert_eq!(label.text, "Hello World");
    assert!(label.font_size.is_none());
}

#[test]
fn test_label_styling() {
    let label = Label::new("Title")
        .with_size("14pt")
        .bold()
        .with_color("#ff0000")
        .center();

    assert_eq!(label.font_size, Some("14pt".into()));
    assert_eq!(label.font_weight, Some("bold".into()));
    assert_eq!(label.color, Some("#ff0000".into()));
    assert_eq!(label.align, Some("center".into()));
}

#[test]
fn test_text_creation() {
    let text = Text::new("This is a longer text block with multiple sentences.");

    assert_eq!(text.component_id(), "textblock");
    assert!(text.content.contains("multiple sentences"));
}

#[test]
fn test_text_formatting() {
    let text = Text::new("Content")
        .with_size("12pt")
        .with_line_height("1.5")
        .with_max_width("500pt");

    assert_eq!(text.font_size, Some("12pt".into()));
    assert_eq!(text.line_height, Some("1.5".into()));
    assert_eq!(text.max_width, Some("500pt".into()));
}

#[test]
fn test_number_field_creation() {
    let num = NumberField::new(1234.56);

    assert_eq!(num.component_id(), "number-field");
    assert_eq!(num.value, 1234.56);
}

#[test]
fn test_number_field_currency() {
    let currency = NumberField::currency(99.99, "$");

    assert_eq!(currency.value, 99.99);
    assert_eq!(currency.prefix, Some("$".into()));
    assert_eq!(currency.format, Some("#,##0.00".into()));
}

#[test]
fn test_number_field_percentage() {
    let pct = NumberField::percentage(87.5);

    assert_eq!(pct.value, 87.5);
    assert_eq!(pct.suffix, Some("%".into()));
}

#[test]
fn test_date_field_creation() {
    let date = DateField::new("2024-03-15");

    assert_eq!(date.component_id(), "date-field");
    assert_eq!(date.date, "2024-03-15");
    assert_eq!(date.format, "YYYY-MM-DD");
}

#[test]
fn test_date_field_formats() {
    let european = DateField::european("2024-03-15");
    let us = DateField::us("2024-03-15");

    assert_eq!(european.format, "DD.MM.YYYY");
    assert_eq!(us.format, "MM/DD/YYYY");
}

#[test]
fn test_resource_field_creation() {
    let resource = ResourceField::new("app.title")
        .with_default("Application Title")
        .with_locale("en");

    assert_eq!(resource.component_id(), "resource-field");
    assert_eq!(resource.key, "app.title");
    assert_eq!(resource.default_value, Some("Application Title".into()));
    assert_eq!(resource.locale, Some("en".into()));
}

#[test]
fn test_all_components_implement_component_trait() {
    // Verify all new components implement Component trait
    let chart = Chart::bar("Test");
    let sparkline = Sparkline::line(vec![1.0, 2.0]);
    let gauge = Gauge::new("Test", 50.0);
    let crosstab = Crosstab::new("A", "B", "C");
    let pivot = PivotTable::new(vec![], vec![], vec![]);
    let barcode = Barcode::code128("TEST");
    let label = Label::new("Test");
    let text = Text::new("Test");
    let number = NumberField::new(100.0);
    let date = DateField::new("2024-01-01");
    let resource = ResourceField::new("key");

    // All should implement component_id()
    assert!(!chart.component_id().is_empty());
    assert!(!sparkline.component_id().is_empty());
    assert!(!gauge.component_id().is_empty());
    assert!(!crosstab.component_id().is_empty());
    assert!(!pivot.component_id().is_empty());
    assert!(!barcode.component_id().is_empty());
    assert!(!label.component_id().is_empty());
    assert!(!text.component_id().is_empty());
    assert!(!number.component_id().is_empty());
    assert!(!date.component_id().is_empty());
    assert!(!resource.component_id().is_empty());

    // All should serialize to JSON
    assert!(chart.to_data().is_object());
    assert!(sparkline.to_data().is_object());
    assert!(gauge.to_data().is_object());
    assert!(crosstab.to_data().is_object());
    assert!(pivot.to_data().is_object());
    assert!(barcode.to_data().is_object());
    assert!(label.to_data().is_object());
    assert!(text.to_data().is_object());
    assert!(number.to_data().is_object());
    assert!(date.to_data().is_object());
    assert!(resource.to_data().is_object());
}
