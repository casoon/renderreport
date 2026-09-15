//! Pagination behaviour of `PageBreak`: it must still start a new page, but
//! adjacent breaks must not produce a blank page (auditmysite: an empty report
//! part followed by the appendix rendered an empty page between them).

use lopdf::Document as PdfDocument;
use renderreport::components::advanced::PageBreak;
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
