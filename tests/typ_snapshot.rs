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
            panic!("snapshot {name} did not exist — wrote it now. Re-run the test to confirm.");
        }
        return;
    }

    let expected = std::fs::read_to_string(&path).expect("failed to read snapshot");
    if expected != actual {
        let diff_path = path.with_extension("typ.actual");
        std::fs::write(&diff_path, actual).ok();

        // Surface the first differing lines so a mismatch is debuggable from the
        // CI log alone — otherwise only the `.actual` path is shown and the cause
        // (e.g. a platform-specific formatting difference) stays invisible.
        let exp_lines: Vec<&str> = expected.lines().collect();
        let act_lines: Vec<&str> = actual.lines().collect();
        let mut diff = String::new();
        let mut shown = 0;
        for i in 0..exp_lines.len().max(act_lines.len()) {
            let e = exp_lines.get(i).copied().unwrap_or("<missing>");
            let a = act_lines.get(i).copied().unwrap_or("<missing>");
            if e != a {
                diff.push_str(&format!(
                    "  line {}:\n    expected: {e:?}\n    actual:   {a:?}\n",
                    i + 1
                ));
                shown += 1;
                if shown >= 5 {
                    diff.push_str("  … (further differences omitted)\n");
                    break;
                }
            }
        }

        panic!(
            "snapshot mismatch: {name}\n  expected: {}\n  actual:   {}\n  re-run with UPDATE_SNAPSHOTS=1 to accept\nfirst differences:\n{diff}",
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
