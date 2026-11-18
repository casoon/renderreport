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

fn page_count_with_soft_break(lines: usize, soft: Option<f64>) -> usize {
    use renderreport::components::advanced::SoftPageBreak;
    let engine = Engine::new().expect("Engine::new failed");
    let mut builder = engine
        .report("default")
        .title("Soft Break Test")
        .metadata("lang", "en")
        .add_component(Section::new("Filler").with_level(1));
    for i in 0..lines {
        builder = builder.add_component(Label::new(format!("Filler line {i}")));
    }
    if let Some(room) = soft {
        builder = builder.add_component(SoftPageBreak::needs(room));
    }
    let request = builder
        .add_component(Section::new("After").with_level(1))
        .add_component(Callout::info("Content after the soft break."))
        .build();

    let pdf = engine.render_pdf(&request).expect("render_pdf failed");
    PdfDocument::load_mem(&pdf)
        .expect("failed to parse the generated PDF")
        .get_pages()
        .len()
}

/// Plenty of room left: the next component follows on the same page, the
/// soft break adds nothing.
#[test]
fn soft_page_break_keeps_the_next_component_when_there_is_room() {
    assert_eq!(
        page_count_with_soft_break(3, Some(0.4)),
        page_count_with_soft_break(3, None)
    );
}

/// Less than the needed room left: the soft break starts a new page, which the
/// plain flow would have continued on the same one.
#[test]
fn soft_page_break_starts_a_new_page_when_little_room_is_left() {
    let found = (3..40).find(|&lines| {
        page_count_with_soft_break(lines, Some(0.4)) == page_count_with_soft_break(lines, None) + 1
    });
    assert!(
        found.is_some(),
        "no filler length separates a soft break from none"
    );
}

/// The soft break never costs more than one page, and a page that ends exactly
/// at the break does not get a blank page after it.
#[test]
fn soft_page_break_never_adds_more_than_one_page() {
    for lines in 3..45 {
        let with = page_count_with_soft_break(lines, Some(0.4));
        let without = page_count_with_soft_break(lines, None);
        assert!(
            with == without || with == without + 1,
            "{lines} lines: {with} pages with the soft break, {without} without"
        );
    }
}
