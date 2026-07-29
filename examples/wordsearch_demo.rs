//! WordSearch Component Example
//!
//! Demonstrates parameter-driven word search puzzle generation and solution rendering.
//!
//! Run with: cargo run --example wordsearch_demo

use renderreport::components::WordSearch;
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    let engine = Engine::new()?;

    // 1. Generate Puzzle Page (for players to solve)
    let puzzle = WordSearch::builder()
        .title("Rätselspaß: Software & Open Source")
        .description("Finde alle 10 versteckten Begriffe im Buchstabengitter (waagerecht, senkrecht & diagonal)!")
        .words(vec![
            "RUST",
            "TYPST",
            "RENDER",
            "REPORT",
            "ENGINE",
            "CARGO",
            "STENCIL",
            "LAYOUT",
            "PARSER",
            "VECTOR",
        ])
        .grid_size(14, 14)
        .allow_diagonal(true)
        .allow_reverse(false)
        .seed(42)
        .show_solution(false)
        .columns_word_list(3)
        .build()?;

    let report_puzzle = engine
        .report("default")
        .title("WordSearch Rätselseite")
        .subtitle("Generiert mit renderreport")
        .add_component(puzzle)
        .build();

    std::fs::create_dir_all("examples/output")?;

    let pdf_puzzle = engine.render_pdf(&report_puzzle)?;
    let path_puzzle = "examples/output/wordsearch_puzzle.pdf";
    std::fs::write(path_puzzle, pdf_puzzle)?;
    println!("✓ Generated {}", path_puzzle);

    // 2. Generate Solution Page (with highlighted words)
    let solution = WordSearch::builder()
        .title("Lösungsblatt: Software & Open Source")
        .description("Alle versteckten Wörter sind farbig hervorgehoben.")
        .words(vec![
            "RUST", "TYPST", "RENDER", "REPORT", "ENGINE", "CARGO", "STENCIL", "LAYOUT", "PARSER",
            "VECTOR",
        ])
        .grid_size(14, 14)
        .allow_diagonal(true)
        .allow_reverse(false)
        .seed(42)
        .show_solution(true)
        .columns_word_list(3)
        .build()?;

    let report_solution = engine
        .report("default")
        .title("WordSearch Lösungsblatt")
        .subtitle("Generiert mit renderreport")
        .add_component(solution)
        .build();

    let pdf_solution = engine.render_pdf(&report_solution)?;
    let path_solution = "examples/output/wordsearch_solution.pdf";
    std::fs::write(path_solution, pdf_solution)?;
    println!("✓ Generated {}", path_solution);

    Ok(())
}
