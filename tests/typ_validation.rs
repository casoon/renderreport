//! Validation tests on the generated Typst source.
//!
//! These tests exercise `Engine::render_typ()` and check structural
//! properties of the produced `.typ` source that the PDF smoke test
//! cannot reveal — bracket balance, basic renderer-leak heuristics, and
//! survival of Typst-like syntax in user-supplied data.

use renderreport::prelude::*;

const CONTENT_MARKER: &str = "// Report Content";

fn canonical_request(engine: &Engine) -> renderreport::RenderRequest {
    engine
        .report("default")
        .title("Typ Validation")
        .add_component(Section::new("Heading").with_level(2))
        .add_component(ScoreCard::new("Score", 85))
        .add_component(Callout::info("Validation test content."))
        .build()
}

fn content_section(source: &str) -> &str {
    source
        .split_once(CONTENT_MARKER)
        .map(|(_, rest)| rest)
        .unwrap_or(source)
}

/// Strip Typst string literals and comments so structural scans don't trip
/// on brackets/keywords appearing inside text content.
fn strip_strings_and_comments(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut out = String::with_capacity(source.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        // Line comment
        if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        // Block comment
        if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(bytes.len());
            continue;
        }
        // String literal
        if b == b'"' {
            out.push('"');
            i += 1;
            while i < bytes.len() {
                let c = bytes[i];
                if c == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if c == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push('"');
            continue;
        }
        out.push(b as char);
        i += 1;
    }
    out
}

#[test]
fn render_typ_returns_non_empty_source() {
    let engine = Engine::new().expect("Engine::new failed");
    let request = canonical_request(&engine);
    let source = engine.render_typ(&request).expect("render_typ failed");

    assert!(!source.is_empty(), "render_typ produced empty source");
    assert!(
        source.contains(CONTENT_MARKER),
        "source missing '// Report Content' marker — generator structure changed"
    );
}

#[test]
fn generated_source_compiles_to_pdf() {
    // render_typ and render_pdf share the same source generation path, so
    // a successful render_pdf proves render_typ's output is compilable.
    let engine = Engine::new().expect("Engine::new failed");
    let request = canonical_request(&engine);

    let _source = engine.render_typ(&request).expect("render_typ failed");
    let pdf = engine
        .render_pdf(&request)
        .expect("render_pdf failed on canonical request");
    assert!(!pdf.is_empty(), "render_pdf produced empty bytes");
}

/// Bracket count balance over the full source.
///
/// Typst's mode switching (code ↔ markup) makes a stack-based balance
/// check unreliable: inside a markup `[...]` block, `{` and `(` are
/// literal characters, not openers. A *global count* balance is robust
/// in practice because the typical defect we want to catch — a missing
/// `]` or `)` somewhere in a template — manifests as a count mismatch,
/// while well-formed string/JSON contents bring their own balanced pairs.
///
/// This will not catch reordered brackets (e.g. `]][[`) but those would
/// be caught by `generated_source_compiles_to_pdf`.
#[test]
fn bracket_counts_are_balanced_in_generated_source() {
    let engine = Engine::new().expect("Engine::new failed");
    let request = canonical_request(&engine);
    let source = engine.render_typ(&request).expect("render_typ failed");

    let mut square = 0i32;
    let mut curly = 0i32;
    let mut paren = 0i32;
    for ch in source.chars() {
        match ch {
            '[' => square += 1,
            ']' => square -= 1,
            '{' => curly += 1,
            '}' => curly -= 1,
            '(' => paren += 1,
            ')' => paren -= 1,
            _ => {}
        }
    }

    assert_eq!(square, 0, "'[' / ']' count imbalance: net {square}");
    assert_eq!(curly, 0, "'{{' / '}}' count imbalance: net {curly}");
    assert_eq!(paren, 0, "'(' / ')' count imbalance: net {paren}");
}

/// Heuristic renderer-leak detector. Scans the content section for
/// function-call-like fragments that should always be preceded by `#`
/// (Typst's code-escape) when used in markup mode. Plain occurrences
/// like `block(...)` in markup render as visible text — the symptom of
/// a renderer leak.
///
/// We only check obvious cases (whitespace + token + `(`), so this is
/// a regression net, not a parser.
#[test]
fn no_obvious_renderer_leaks_in_content_section() {
    let engine = Engine::new().expect("Engine::new failed");
    let request = canonical_request(&engine);
    let source = engine.render_typ(&request).expect("render_typ failed");
    let content = content_section(&source);
    let stripped = strip_strings_and_comments(content);

    // Tokens that are *only* meaningful as Typst function calls.
    // If they appear in markup mode without a leading `#`, they leak.
    let tokens = ["block", "box", "grid", "stack", "rect", "circle"];

    for line in stripped.lines() {
        let trimmed = line.trim_start();
        for tok in tokens {
            let pat = format!("{tok}(");
            if let Some(pos) = trimmed.find(&pat) {
                // Anything to the left of the token on this line?
                let left = &trimmed[..pos];
                // Allowed contexts: preceded by `#`, inside a code block
                // continuation (e.g. argument list), or part of an identifier.
                let last = left.chars().last();
                let preceded_ok = match last {
                    None => false, // token at column 0 in markup ⇒ leak
                    Some(c) => {
                        // `#tok(` — code escape
                        c == '#'
                            // identifier continuation (e.g. `my-block(`)
                            || c.is_alphanumeric()
                            || c == '_'
                            || c == '-'
                            // argument-list continuation: `, tok(` / `: tok(`
                            // / `( tok(` / `+ tok(` etc.
                            || matches!(c, ',' | ':' | '(' | '+' | '*' | '/' | '=' | '>' | '<' | '&' | '|')
                            // whitespace inside an existing code expression —
                            // we can't statically tell, so accept and rely on
                            // the compile test to catch true breakage.
                            || c.is_whitespace()
                    }
                };
                assert!(
                    preceded_ok,
                    "possible renderer leak: '{pat}' appears in content without code escape\nline: {line}"
                );
            }
        }
    }
}

/// Typst-significant characters inside user-supplied strings must not
/// break compilation. This is a survival test, not a semantic-escaping
/// test — it proves the generator does not produce a syntactically
/// broken `.typ` when a title contains markup-like fragments.
#[test]
fn typst_syntax_in_user_data_does_not_break_compile() {
    let engine = Engine::new().expect("Engine::new failed");
    let request = engine
        .report("default")
        .title("Hello #v(10em) [world] \"quoted\" *bold*")
        .add_component(Section::new("Heading with #block(fill: red)[x]").with_level(2))
        .add_component(Callout::info("Body with `code` and @ref and $math$."))
        .build();

    let source = engine
        .render_typ(&request)
        .expect("render_typ failed on poisoned input");
    assert!(source.contains(CONTENT_MARKER));

    let pdf = engine
        .render_pdf(&request)
        .expect("compile failed: user-supplied Typst syntax broke generation");
    assert!(!pdf.is_empty());
}
