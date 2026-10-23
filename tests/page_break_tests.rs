//! Pagination behaviour of `PageBreak`: it must still start a new page, but
//! adjacent breaks must not produce a blank page (auditmysite: an empty report
//! part followed by the appendix rendered an empty page between them).

use lopdf::Document as PdfDocument;
use renderreport::components::advanced::{Divider, PageBreak};
use renderreport::components::text::Label;
use renderreport::prelude::*;

fn page_count(page_breaks: usize) -> usize {
    let engine = Engine::new().expect("Engine::new failed");
    let mut builder = engine
        .report("default")
        .title("Page Break Test")
        .metadata("lang", "en")
        .add_component(Section::new("Before").with_level(1))
        .add_component(Callout::info("Content before the break."));
    for _ in 0..page_breaks {
        builder = builder.add_component(PageBreak::new());
    }
    let request = builder
        .add_component(Section::new("After").with_level(1))
        .add_component(Callout::info("Content after the break."))
        .build();

    let pdf = engine.render_pdf(&request).expect("render_pdf failed");
    PdfDocument::load_mem(&pdf)
        .expect("failed to parse the generated PDF")
        .get_pages()
        .len()
}

#[test]
fn page_break_starts_a_new_page() {
    assert_eq!(page_count(1), page_count(0) + 1);
}

#[test]
fn adjacent_page_breaks_do_not_produce_a_blank_page() {
    assert_eq!(page_count(2), page_count(1));
}

fn page_count_with_filler(lines: usize, invisible_divider: bool) -> usize {
    let engine = Engine::new().expect("Engine::new failed");
    let mut builder = engine
        .report("default")
        .title("Divider Test")
        .metadata("lang", "en")
        .add_component(Section::new("Filler").with_level(1));
    for i in 0..lines {
        builder = builder.add_component(Label::new(format!("Filler line {i}")));
    }
    if invisible_divider {
        builder = builder.add_component(Divider {
            style: "solid".to_string(),
            thickness: "0pt".to_string(),
            color: Some("#ffffff".to_string()),
            spacing_above: "15pt".to_string(),
            spacing_below: "0pt".to_string(),
        });
    }
    let request = builder
        .add_component(PageBreak::new())
        .add_component(Section::new("After").with_level(1))
        .build();

    let pdf = engine.render_pdf(&request).expect("render_pdf failed");
    PdfDocument::load_mem(&pdf)
        .expect("failed to parse the generated PDF")
        .get_pages()
        .len()
}

/// An invisible divider is spacing only. Rendered as a zero-width line it was
/// still content and, at the end of a full page, opened an otherwise blank
/// page before the next break (auditmysite finding cards).
#[test]
fn invisible_divider_at_page_end_does_not_produce_a_blank_page() {
    for lines in 16..30 {
        assert_eq!(
            page_count_with_filler(lines, true),
            page_count_with_filler(lines, false),
            "blank page with {lines} filler lines"
        );
    }
}
