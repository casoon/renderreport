//! Layout guarantees auditmysite relies on: the contents page lists the
//! report's sections, not the cover title or itself, and a table row never
//! splits across a page break (auditmysite plan 108).

use lopdf::Document as PdfDocument;
use renderreport::components::advanced::TableOfContents;
use renderreport::components::*;
use renderreport::prelude::*;

fn page_texts(pdf: &[u8]) -> Vec<String> {
    let doc = PdfDocument::load_mem(pdf).expect("failed to parse the generated PDF");
    let pages: Vec<u32> = doc.get_pages().keys().copied().collect();
    pages
        .iter()
        .map(|page| doc.extract_text(&[*page]).unwrap_or_default())
        .collect()
}

#[test]
fn contents_page_lists_sections_but_not_the_cover_title_or_itself() {
    let engine = Engine::new().expect("Engine::new failed");
    let request = engine
        .report("default")
        .title("Cover Title Xyz")
        .metadata("lang", "en")
        .add_component(
            TableOfContents::new()
                .with_title("Contents Qwv")
                .with_depth(1),
        )
        .add_component(Section::new("Alpha Section").with_level(1))
        .add_component(Callout::info("Body."))
        .build();
    let pdf = engine.render_pdf(&request).expect("render_pdf failed");
    let pages = page_texts(&pdf);
    let toc = pages
        .iter()
        .find(|text| text.contains("Contents Qwv"))
        .expect("contents page");
    // The running header repeats the report title above the contents, so
    // only the entries after the contents heading count.
    let entries = toc.split_once("Contents Qwv").expect("contents heading").1;
    assert!(entries.contains("Alpha Section"), "{toc}");
    assert!(!entries.contains("Contents Qwv"), "{toc}");
    assert!(!entries.contains("Cover Title Xyz"), "{toc}");
}

#[test]
fn table_rows_do_not_split_across_pages() {
    let engine = Engine::new().expect("Engine::new failed");
    let mut table = AuditTable::new(vec![
        TableColumn::new("Key").with_width("20%"),
        TableColumn::new("Text").with_width("80%"),
    ]);
    for row in 0..40 {
        table = table.add_row(vec![
            format!("row{row}"),
            format!(
                "begin{row} {} end{row}",
                "words that wrap over several lines inside one narrow cell ".repeat(12)
            ),
        ]);
    }
    let request = engine
        .report("default")
        .metadata("lang", "en")
        .add_component(Section::new("Table").with_level(1))
        .add_component(table)
        .build();
    let pdf = engine.render_pdf(&request).expect("render_pdf failed");
    let pages = page_texts(&pdf);
    assert!(pages.len() > 1, "fixture must span several pages");
    for row in 0..40 {
        let begin = pages
            .iter()
            .position(|t| t.contains(&format!("begin{row} ")));
        let end = pages.iter().position(|t| t.contains(&format!(" end{row}")));
        assert_eq!(begin, end, "row {row} split across pages");
    }
}

#[test]
fn cover_renders_the_indicator_strip_below_the_module_gauges() {
    let engine = Engine::new().expect("Engine::new failed");
    let cover = CoverPage::new("Report", "example.com", 49, "F")
        .with_module_gauges(vec![CoverModuleGauge::new("Accessibility", 49)])
        .with_indicator_gauges(
            "INDICATORS NOT WEIGHTED",
            vec![
                CoverModuleGauge::new("UX Indicator", 68),
                CoverModuleGauge::new("Journey Indicator", 55),
            ],
        );
    let request = engine
        .report("default")
        .metadata("lang", "en")
        .add_component(cover)
        .build();
    let pdf = engine.render_pdf(&request).expect("render_pdf failed");
    let first = page_texts(&pdf).into_iter().next().expect("cover page");
    let modules = first.find("Accessibility").expect("module gauge");
    let label = first
        .find("INDICATORS NOT WEIGHTED")
        .expect("indicator label");
    assert!(
        modules < label,
        "indicators must follow the modules: {first}"
    );
    assert!(first.contains("UX Indicator") && first.contains("Journey Indicator"));
}
