//! The `chartlet` component draws a chartlet specification into the PDF.
#![cfg(feature = "chartlet")]

use lopdf::Document as PdfDocument;
use renderreport::validate::validate_request;
use renderreport::{Engine, RenderRequest};

fn request(chartlet_data: &str) -> String {
    format!(
        r#"{{ "template_id": "default", "title": "Charts", "metadata": {{ "lang": "en" }},
        "components": [ {{ "type": "chartlet", "data": {chartlet_data} }} ] }}"#
    )
}

const SPEC: &str = r#"{ "schemaVersion": 1, "type": "bar", "title": "Failed checks by page",
    "categories": ["Home", "Search", "Checkout"],
    "series": [{ "name": "Failed", "values": [4, 9, 12] }] }"#;

#[test]
fn a_specification_renders_as_a_figure_with_alt_text() {
    let json = request(&format!(
        r#"{{ "spec": {SPEC}, "alt": "Checkout has the most failed checks", "caption": "Failed checks" }}"#
    ));
    assert!(validate_request(&json, &[]).is_empty());

    let engine = Engine::new().unwrap();
    let request: RenderRequest = serde_json::from_str(&json).unwrap();
    let pdf = engine.render_pdf(&request).unwrap();

    let document = PdfDocument::load_mem(&pdf).unwrap();
    // title page + the page with the chart
    assert_eq!(document.get_pages().len(), 2);
    let text = String::from_utf8_lossy(&pdf);
    assert!(text.contains("Checkout has the most failed checks"));
    assert!(text.contains("Figure"));

    // The specification does not travel into the Typst source, the SVG does.
    let typ = engine.render_typ(&request).unwrap();
    assert!(!typ.contains("schemaVersion"));
    assert!(typ.contains("<svg"));
}

#[test]
fn without_alt_the_generated_description_is_used() {
    let engine = Engine::new().unwrap();
    let request: RenderRequest =
        serde_json::from_str(&request(&format!(r#"{{ "spec": {SPEC} }}"#))).unwrap();
    // PDF/UA enforcement fails the render if the image has no alt text.
    engine.render_pdf(&request).unwrap();
}

#[test]
fn an_invalid_specification_is_reported_by_validate() {
    let json = request(r#"{ "spec": { "schemaVersion": 1, "type": "nonsense" } }"#);
    let issues = validate_request(&json, &[]);
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert_eq!(issues[0].path, "components[0].data");
}
