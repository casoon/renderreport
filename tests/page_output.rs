//! PNG and SVG page output must match the PDF page for page.
#![cfg(all(feature = "png", feature = "svg"))]

use lopdf::Document as PdfDocument;
use renderreport::{Engine, PageFormat, RenderRequest};

fn health_check() -> RenderRequest {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/requests/health-check.json"
    );
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

#[test]
fn png_and_svg_have_one_image_per_pdf_page() {
    let engine = Engine::new().unwrap();
    let request = health_check();

    let pdf = engine.render_pdf(&request).unwrap();
    let pdf_pages = PdfDocument::load_mem(&pdf).unwrap().get_pages().len();
    assert!(pdf_pages > 0);

    let png = engine
        .render_pages(&request, PageFormat::Png { ppi: 72 })
        .unwrap();
    assert_eq!(png.len(), pdf_pages);
    for page in &png {
        assert!(page.starts_with(b"\x89PNG\r\n\x1a\n"));
    }

    let svg = engine.render_pages(&request, PageFormat::Svg).unwrap();
    assert_eq!(svg.len(), pdf_pages);
    for page in &svg {
        assert!(String::from_utf8_lossy(page)
            .trim_start()
            .starts_with("<svg"));
    }
}

#[test]
fn higher_ppi_gives_a_larger_png() {
    let engine = Engine::new().unwrap();
    let request = health_check();
    let low = engine
        .render_pages(&request, PageFormat::Png { ppi: 36 })
        .unwrap();
    let high = engine
        .render_pages(&request, PageFormat::Png { ppi: 144 })
        .unwrap();
    let width = |png: &[u8]| u32::from_be_bytes(png[16..20].try_into().unwrap());
    assert!(width(&high[0]).abs_diff(width(&low[0]) * 4) <= 4);
}
