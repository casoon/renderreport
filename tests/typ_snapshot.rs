//! Snapshot regression test for the generated Typst source.
//!
//! Compares the output of `Engine::render_typ()` for a canonical
//! request against a committed fixture in `fixtures/snapshots/`. Any
//! drift in theme tokens, page setup, component templates, or content
//! generation will fail this test.
//!
//! Update fixtures with `UPDATE_SNAPSHOTS=1 cargo test --test typ_snapshot`.

use renderreport::prelude::*;
use std::path::PathBuf;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("snapshots")
        .join(name)
}

fn assert_snapshot(name: &str, actual: &str) {
    let path = fixture_path(name);
    let update = std::env::var("UPDATE_SNAPSHOTS").is_ok();

    if update || !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, actual).expect("failed to write snapshot");
        if !update {
            panic!(
                "snapshot {name} did not exist — wrote it now. Re-run the test to confirm."
            );
        }
        return;
    }

    let expected = std::fs::read_to_string(&path).expect("failed to read snapshot");
    if expected != actual {
        let diff_path = path.with_extension("typ.actual");
        std::fs::write(&diff_path, actual).ok();
        panic!(
            "snapshot mismatch: {name}\n  expected: {}\n  actual:   {}\n  re-run with UPDATE_SNAPSHOTS=1 to accept",
            path.display(),
            diff_path.display()
        );
    }
}

#[test]
fn snapshot_minimal_report() {
    let engine = Engine::new().expect("Engine::new failed");
    let request = engine
        .report("default")
        .title("Snapshot Report")
        .add_component(Section::new("Heading").with_level(2))
        .add_component(ScoreCard::new("Score", 85))
        .add_component(Callout::info("Stable canonical content."))
        .build();

    let source = engine.render_typ(&request).expect("render_typ failed");
    assert_snapshot("minimal_report.typ", &source);
}
