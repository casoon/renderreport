//! Structural PDF/UA-1 accessibility checks on a rendered report.
//!
//! `Engine::render_pdf` now compiles with `typst_pdf::PdfStandard::Ua_1`
//! enforced (see `src/render/typst_compile.rs`), which already makes Typst
//! itself hard-fail at compile time if the document is missing a title,
//! language, outline, tagging, or image alt text. That compile-time check is
//! necessary but not sufficient proof that the *emitted PDF bytes* actually
//! carry the required structure — this test parses the real output with
//! `lopdf` and asserts on it directly, per the acceptance criterion in
//! auditmysite#573 ("automatisierbaren Export-Check: Strukturbaum vorhanden,
//! Sprache gesetzt, Titel vorhanden, Lesezeichen vorhanden").

use lopdf::Document as PdfDocument;
use renderreport::prelude::*;
use renderreport::RenderRequest;
use std::path::PathBuf;

fn asset_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join(name)
}

fn representative_request(engine: &Engine) -> RenderRequest {
    engine
        .report("default")
        .title("Accessibility Structure Report")
        .metadata("lang", "de")
        .metadata("author", "Casoon")
        .add_component(Section::new("Ergebnisse").with_level(1))
        .add_component(ScoreCard::new("Score", 92))
        .add_component(Callout::info("Struktur-Test für PDF/UA-1."))
        .asset("placeholder.svg", asset_path("placeholder.svg"))
        .add_component(Image::new("placeholder.svg").with_alt("Firmenlogo"))
        .build()
}

#[test]
fn rendered_pdf_has_struct_tree_lang_title_and_outline() {
    let engine = Engine::new().expect("Engine::new failed");
    let request = representative_request(&engine);

    let pdf_bytes = engine
        .render_pdf(&request)
        .expect("render_pdf failed to produce a PDF/UA-1-conformant document");
    assert!(!pdf_bytes.is_empty(), "render_pdf returned empty bytes");

    let doc = PdfDocument::load_mem(&pdf_bytes).expect("failed to parse the generated PDF");
    let catalog = doc.catalog().expect("PDF has no document catalog");

    // (a) Structure tree present → the PDF is tagged.
    assert!(
        catalog.has(b"StructTreeRoot"),
        "catalog is missing /StructTreeRoot — the PDF is not tagged"
    );

    // (b) Document language set at the catalog level.
    let lang = catalog
        .get(b"Lang")
        .and_then(|o| o.as_str())
        .expect("catalog is missing /Lang");
    assert_eq!(
        lang, b"de",
        "document language did not propagate from metadata(\"lang\", \"de\")"
    );

    // (c) Title present in the document info dictionary.
    let info = doc
        .trailer
        .get(b"Info")
        .and_then(|o| o.as_reference())
        .and_then(|id| doc.get_dictionary(id))
        .expect("PDF has no /Info dictionary");
    let title = info
        .get(b"Title")
        .and_then(|o| o.as_str())
        .expect("/Info dictionary is missing /Title");
    assert_eq!(title, b"Accessibility Structure Report");

    // (d) At least one bookmark/outline entry exists.
    assert!(
        catalog.has(b"Outlines"),
        "catalog is missing /Outlines — no PDF bookmarks were produced"
    );
}

/// The same check for a report that uses the `cover-page` component as its
/// front page instead of `RenderRequest.title` — this is the shape real
/// audit reports use (a rich dashboard cover, not the plain built-in title
/// page), and it must derive the same PDF/UA-1 title from the component's
/// own `title` field. See `fallback_cover_page_title` in `src/engine/mod.rs`.
#[test]
fn rendered_pdf_with_cover_page_component_derives_title_from_it() {
    let engine = Engine::new().expect("Engine::new failed");
    let request = engine
        .report("default")
        // Deliberately no `.title(...)` call.
        .metadata("lang", "en")
        .add_component(
            CoverPage::new("Cover Page Report", "example.com", 88, "B+").with_brand("Casoon"),
        )
        .add_component(Section::new("Findings").with_level(1))
        .add_component(ScoreCard::new("Score", 88))
        .build();

    let pdf_bytes = engine
        .render_pdf(&request)
        .expect("render_pdf failed to produce a PDF/UA-1-conformant document");

    let doc = PdfDocument::load_mem(&pdf_bytes).expect("failed to parse the generated PDF");
    let catalog = doc.catalog().expect("PDF has no document catalog");
    assert!(catalog.has(b"StructTreeRoot"));
    assert!(catalog.has(b"Outlines"));

    let info = doc
        .trailer
        .get(b"Info")
        .and_then(|o| o.as_reference())
        .and_then(|id| doc.get_dictionary(id))
        .expect("PDF has no /Info dictionary");
    let title = info
        .get(b"Title")
        .and_then(|o| o.as_str())
        .expect("/Info dictionary is missing /Title");
    assert_eq!(title, b"Cover Page Report");
}
