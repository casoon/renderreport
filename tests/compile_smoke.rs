//! Smoke tests that compile every registered component template through Typst.
//!
//! All component templates are always included in the generated Typst source,
//! so a single successful render_pdf call validates all of them. Any syntax
//! error in any .typ template (e.g. `#v(...)` inside a code-mode block) will
//! surface here immediately instead of at runtime on the first affected audit.

use renderreport::prelude::*;

#[test]
fn all_templates_compile_to_pdf() {
    let engine = Engine::new().expect("Engine::new failed");

    let request = engine
        .report("default")
        .title("Smoke Test")
        .add_component(Section::new("Heading").with_level(2))
        .add_component(ScoreCard::new("Score", 85))
        .add_component(Callout::info("All templates compiled successfully."))
        .build();

    let pdf = engine
        .render_pdf(&request)
        .expect("Typst compilation failed — check template syntax");

    assert!(!pdf.is_empty(), "render_pdf returned empty bytes");
}
