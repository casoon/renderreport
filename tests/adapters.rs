//! The input adapters: each fixture gives a request that validates, renders
//! and carries the figures of the source; broken input names the format.

#![cfg(feature = "cli")]

use renderreport::adapt::{from_file, Format, Lang, Options};
use renderreport::validate::validate_value;
use renderreport::{Engine, RenderRequest};
use serde_json::Value;

const DIR: &str = "fixtures/adapters";

fn options(lang: Option<Lang>) -> Options {
    Options { title: None, lang }
}

/// Builds the request for a fixture and checks that it validates and renders.
fn adapt(format: Format, file: &str, lang: Option<Lang>) -> Value {
    let request = from_file(format, format!("{DIR}/{file}").as_ref(), &options(lang))
        .unwrap_or_else(|e| panic!("{file}: {e}"));
    let issues = validate_value(&request, &[]);
    assert!(issues.is_empty(), "{file}: {issues:?}");
    let parsed: RenderRequest = serde_json::from_value(request.clone()).unwrap();
    let pdf = Engine::new().unwrap().render_pdf(&parsed).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "{file}");
    request
}

fn types(request: &Value) -> Vec<&str> {
    request["components"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["type"].as_str().unwrap())
        .collect()
}

fn count(request: &Value, kind: &str) -> usize {
    types(request).iter().filter(|t| **t == kind).count()
}

/// Writes `text` to a temporary file and adapts it, expecting an error.
fn error_of(format: Format, name: &str, text: &str) -> String {
    let path = std::env::temp_dir().join(format!("renderreport-adapters-{name}"));
    std::fs::write(&path, text).unwrap();
    let error = from_file(format, &path, &options(None)).unwrap_err();
    std::fs::remove_file(&path).unwrap();
    error
}

#[test]
fn markdown_maps_blocks_to_components() {
    let request = adapt(Format::Markdown, "sample.md", None);
    assert_eq!(request["title"], "Quarterly notes");
    assert_eq!(request["subtitle"], "Engineering update");
    assert_eq!(request["metadata"]["author"], "Web Team");
    assert_eq!(request["metadata"]["date"], "2026-10-01");
    assert_eq!(count(&request, "section"), 4);
    assert_eq!(count(&request, "list"), 2);
    assert_eq!(count(&request, "quote-block"), 1);
    assert_eq!(count(&request, "table"), 1);
    assert_eq!(count(&request, "image"), 1);
    assert_eq!(count(&request, "divider"), 1);

    let list = request["components"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["type"] == "list")
        .unwrap();
    assert_eq!(
        list["data"]["items"][1]["children"][0]["content"],
        "Retries removed"
    );
    let text = request.to_string();
    assert!(text.contains("See the changelog (https://example.com/changelog) for details"));
    assert!(!text.contains("**"));
}

#[test]
fn markdown_errors_say_what_is_wrong() {
    let error = error_of(Format::Markdown, "fence.md", "# A\n\n```\ncode\n");
    assert!(
        error.contains("markdown") && error.contains("never closed"),
        "{error}"
    );
    let error = error_of(Format::Markdown, "image.md", "![x](missing.png)\n");
    assert!(
        error.contains("missing.png") && error.contains("not found"),
        "{error}"
    );
}

#[test]
fn csv_has_figures_chart_and_table() {
    let request = adapt(Format::Csv, "sales.csv", None);
    assert_eq!(request["title"], "sales");
    let text = request.to_string();
    // 12500 + 9800 + 15300.5 + 7200 + 11100
    assert!(text.contains("\"55900.5\""), "{text}");
    assert!(text.contains("\"15300.5\"") && text.contains("\"7200\""));
    assert_eq!(count(&request, "chart"), 1);
    assert_eq!(count(&request, "table"), 1);
    let chart = request["components"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["type"] == "chart")
        .unwrap();
    assert_eq!(chart["data"]["series"][0]["name"], "Revenue");
    assert_eq!(chart["data"]["series"][0]["data"][0]["label"], "North");
}

#[test]
fn csv_chart_is_limited_to_twelve_rows() {
    let mut csv = String::from("Name,Value\n");
    for n in 1..=30 {
        csv.push_str(&format!("item {n},{n}\n"));
    }
    let path = std::env::temp_dir().join("renderreport-adapters-long.csv");
    std::fs::write(&path, csv).unwrap();
    let request = from_file(Format::Csv, &path, &options(None)).unwrap();
    std::fs::remove_file(&path).unwrap();
    let chart = request["components"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["type"] == "chart")
        .unwrap();
    assert_eq!(
        chart["data"]["series"][0]["data"].as_array().unwrap().len(),
        12
    );
}

#[test]
fn csv_errors_say_what_is_wrong() {
    let error = error_of(Format::Csv, "empty.csv", "a,b\n");
    assert!(
        error.contains("csv") && error.contains("no data rows"),
        "{error}"
    );
    let error = error_of(Format::Csv, "ragged.csv", "a,b\n1\n");
    assert!(error.contains("csv") && error.contains("row 1"), "{error}");
}

#[test]
fn lighthouse_has_scores_vitals_and_findings() {
    let request = adapt(Format::Lighthouse, "lighthouse.json", None);
    assert_eq!(request["subtitle"], "https://example.com/");
    assert_eq!(request["metadata"]["date"], "2026-10-20");
    let components = request["components"].as_array().unwrap();
    let scores: Vec<&Value> = components[1]["data"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| &i["content"]["data"]["score"])
        .collect();
    assert_eq!(scores, [72, 91, 100, 85]);
    let text = request.to_string();
    assert!(text.contains("4.1 s") && text.contains("310 ms"));
    // Below 0.9 and counted in a category: LCP, TBT, speed index, contrast,
    // meta description; the weightless `unused-javascript` is left out.
    let findings: Vec<&Value> = components
        .iter()
        .filter(|c| c["type"] == "finding")
        .collect();
    assert_eq!(findings.len(), 5, "{text}");
    assert!(!text.contains("Reduce unused JavaScript"));
    assert_eq!(findings[0]["data"]["severity"], "high");
    assert!(!text.contains("](http"));
}

#[test]
fn lighthouse_errors_say_what_is_wrong() {
    let error = error_of(Format::Lighthouse, "lh.json", r#"{"audits":{}}"#);
    assert!(
        error.contains("lighthouse") && error.contains("`categories`"),
        "{error}"
    );
    let error = error_of(Format::Lighthouse, "lh-bad.json", "not json");
    assert!(
        error.contains("lighthouse") && error.contains("not valid JSON"),
        "{error}"
    );
}

#[test]
fn sarif_counts_levels_and_lists_findings() {
    let request = adapt(Format::Sarif, "results.sarif", None);
    assert_eq!(request["title"], "ExampleScan report");
    assert_eq!(request["subtitle"], "ExampleScan 1.4.2");
    let overview = request["components"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["type"] == "severity-overview")
        .unwrap();
    // One error, two warnings (one by default), one note.
    assert_eq!(overview["data"]["critical"], 1);
    assert_eq!(overview["data"]["serious"], 2);
    assert_eq!(overview["data"]["moderate"], 1);
    assert_eq!(count(&request, "finding"), 4);
    let text = request.to_string();
    assert!(text.contains("EX001") && text.contains("src/config.rs:42"));
    assert!(text.contains("src/io.rs:131"));
}

#[test]
fn sarif_shortens_long_result_lists() {
    let results: Vec<String> = (0..120)
        .map(|n| format!(r#"{{"ruleId":"R{n}","level":"note","message":{{"text":"m{n}"}}}}"#))
        .collect();
    let sarif = format!(
        r#"{{"version":"2.1.0","runs":[{{"tool":{{"driver":{{"name":"T"}}}},"results":[{}]}}]}}"#,
        results.join(",")
    );
    let path = std::env::temp_dir().join("renderreport-adapters-long.sarif");
    std::fs::write(&path, sarif).unwrap();
    let request = from_file(Format::Sarif, &path, &options(None)).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert!(validate_value(&request, &[]).is_empty());
    assert_eq!(count(&request, "finding"), 100);
    assert!(request.to_string().contains("Showing the first 100 of 120"));
}

#[test]
fn sarif_errors_say_what_is_wrong() {
    let error = error_of(Format::Sarif, "no-runs.sarif", r#"{"version":"2.1.0"}"#);
    assert!(
        error.contains("sarif") && error.contains("`runs`"),
        "{error}"
    );
    let error = error_of(
        Format::Sarif,
        "no-tool.sarif",
        r#"{"runs":[{"results":[]}]}"#,
    );
    assert!(error.contains("tool.driver"), "{error}");
}

#[test]
fn junit_has_figures_failures_and_suites() {
    let request = adapt(Format::Junit, "junit.xml", None);
    let components = request["components"].as_array().unwrap();
    let strip = components
        .iter()
        .find(|c| c["type"] == "metric-strip")
        .unwrap();
    let values: Vec<&str> = strip["data"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["value"].as_str().unwrap())
        .collect();
    // 5 tests: 2 pass, 1 failure + 1 error, 1 skipped; 1.2 + 1.15 seconds.
    assert_eq!(values, ["5", "2", "2", "1", "2.35"]);
    let findings: Vec<&Value> = components
        .iter()
        .filter(|c| c["type"] == "finding")
        .collect();
    assert_eq!(findings.len(), 2);
    assert_eq!(
        findings[0]["data"]["title"],
        "parser.Basic.rejects_bad_escape"
    );
    assert_eq!(findings[0]["data"]["severity"], "high");
    assert!(findings[0]["data"]["description"]
        .as_str()
        .unwrap()
        .contains("expected error \"bad escape\""));
    assert_eq!(findings[1]["data"]["severity"], "critical");
    assert!(findings[1]["data"]["description"]
        .as_str()
        .unwrap()
        .contains("<details>"));
    let table = components.iter().find(|c| c["type"] == "table").unwrap();
    assert_eq!(table["data"]["rows"][0][0], "parser");
    assert_eq!(table["data"]["rows"].as_array().unwrap().len(), 2);
}

#[test]
fn junit_errors_say_what_is_wrong() {
    let error = error_of(Format::Junit, "none.xml", "<report/>");
    assert!(
        error.contains("junit") && error.contains("<testsuite>"),
        "{error}"
    );
    let error = error_of(
        Format::Junit,
        "cut.xml",
        "<testsuite name=\"a\"><testcase name=\"b\"",
    );
    assert!(
        error.contains("junit") && error.contains("unterminated"),
        "{error}"
    );
}

#[test]
fn german_labels_and_default_language() {
    let en = adapt(Format::Junit, "junit.xml", None);
    assert_eq!(en["metadata"]["lang"], "en");
    assert!(en.to_string().contains("Passed"));
    let de = adapt(Format::Junit, "junit.xml", Some(Lang::De));
    assert_eq!(de["metadata"]["lang"], "de");
    assert!(de.to_string().contains("Bestanden"));
    assert!(!de.to_string().contains("\"Passed\""));
}

#[test]
fn title_option_overrides_the_adapter_title() {
    let options = Options {
        title: Some("Custom".into()),
        lang: None,
    };
    let request = from_file(Format::Csv, format!("{DIR}/sales.csv").as_ref(), &options).unwrap();
    assert_eq!(request["title"], "Custom");
}
