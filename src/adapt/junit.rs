//! JUnit XML: pass/fail figures, the failures as findings, a table of suites.
//!
//! The XML is read by a minimal tag scanner that knows only what a test
//! report contains (`testsuite`, `testcase`, `failure`, `error`, `skipped`
//! and their attributes and text); everything else is skipped.

use serde_json::json;

use super::{callout, component, format_number, plain, section, truncate, Options, Report, Source};

const MAX_FINDINGS: usize = 100;

enum Event {
    Open {
        name: String,
        attrs: Vec<(String, String)>,
        empty: bool,
    },
    Close(String),
    Text(String),
}

#[derive(Default)]
struct Suite {
    name: String,
    attrs: [u64; 4],
    time_attr: f64,
    tests: u64,
    failed: u64,
    skipped: u64,
    time: f64,
}

enum Outcome {
    Pass,
    Failure,
    Error,
    Skipped,
}

struct Case {
    suite: usize,
    name: String,
    classname: String,
    outcome: Outcome,
    message: String,
    detail: String,
}

pub(super) fn build(source: &Source, options: &Options) -> Result<Report, String> {
    let lang = options.lang();
    let events = scan(source.text)?;

    let mut suites: Vec<Suite> = Vec::new();
    let mut open_suites: Vec<usize> = Vec::new();
    let mut cases: Vec<Case> = Vec::new();
    let mut current: Option<usize> = None;
    let mut collecting = false;
    for event in events {
        match event {
            Event::Open { name, attrs, empty } => {
                let attr = |key: &str| {
                    attrs
                        .iter()
                        .find(|(k, _)| k == key)
                        .map(|(_, v)| v.as_str())
                };
                let number = |key: &str| attr(key).and_then(|v| v.trim().parse::<f64>().ok());
                match name.as_str() {
                    "testsuite" => {
                        suites.push(Suite {
                            name: attr("name").unwrap_or("").to_string(),
                            attrs: [
                                number("tests").unwrap_or(0.0) as u64,
                                number("failures").unwrap_or(0.0) as u64,
                                number("errors").unwrap_or(0.0) as u64,
                                number("skipped").unwrap_or(0.0) as u64,
                            ],
                            time_attr: number("time").unwrap_or(0.0),
                            ..Suite::default()
                        });
                        open_suites.push(suites.len() - 1);
                    }
                    "testcase" => {
                        let suite = match open_suites.last() {
                            Some(&s) => s,
                            None => {
                                suites.push(Suite {
                                    name: lang.t("(no suite)", "(ohne Suite)").into(),
                                    ..Suite::default()
                                });
                                open_suites.push(suites.len() - 1);
                                suites.len() - 1
                            }
                        };
                        suites[suite].time += number("time").unwrap_or(0.0);
                        cases.push(Case {
                            suite,
                            name: attr("name").unwrap_or("").to_string(),
                            classname: attr("classname").unwrap_or("").to_string(),
                            outcome: Outcome::Pass,
                            message: String::new(),
                            detail: String::new(),
                        });
                        current = Some(cases.len() - 1);
                    }
                    "failure" | "error" | "skipped" => {
                        if let Some(case) = current.map(|n| &mut cases[n]) {
                            case.outcome = match name.as_str() {
                                "failure" => Outcome::Failure,
                                "error" => Outcome::Error,
                                _ => Outcome::Skipped,
                            };
                            case.message = attr("message").unwrap_or("").to_string();
                            collecting = !empty;
                        }
                    }
                    _ => {}
                }
            }
            Event::Close(name) => match name.as_str() {
                "testsuite" => {
                    open_suites.pop();
                }
                "testcase" => current = None,
                "failure" | "error" | "skipped" => collecting = false,
                _ => {}
            },
            Event::Text(text) => {
                if let (true, Some(case)) = (collecting, current) {
                    cases[case].detail.push_str(&text);
                }
            }
        }
    }
    if suites.is_empty() {
        return Err(
            "junit: no <testsuite> or <testcase> element found (is this a JUnit XML file?)".into(),
        );
    }

    for case in &cases {
        let suite = &mut suites[case.suite];
        suite.tests += 1;
        match case.outcome {
            Outcome::Failure | Outcome::Error => suite.failed += 1,
            Outcome::Skipped => suite.skipped += 1,
            Outcome::Pass => {}
        }
    }
    // A file without test case elements is taken from the suites' attributes.
    for suite in suites.iter_mut().filter(|_| cases.is_empty()) {
        suite.tests = suite.attrs[0];
        suite.failed = suite.attrs[1] + suite.attrs[2];
        suite.skipped = suite.attrs[3];
    }
    for suite in suites.iter_mut().filter(|s| s.time == 0.0) {
        suite.time = suite.time_attr;
    }
    let suites: Vec<&Suite> = suites.iter().filter(|s| s.tests > 0).collect();
    if suites.is_empty() {
        return Err("junit: the file contains no test cases".into());
    }

    let tests: u64 = suites.iter().map(|s| s.tests).sum();
    let failed: u64 = suites.iter().map(|s| s.failed).sum();
    let skipped: u64 = suites.iter().map(|s| s.skipped).sum();
    let passed = tests.saturating_sub(failed + skipped);
    let time: f64 = suites.iter().map(|s| s.time).sum();

    let mut report = Report {
        title: "Test report".into(),
        subtitle: (suites.len() == 1 && !suites[0].name.is_empty()).then(|| suites[0].name.clone()),
        ..Report::default()
    };
    report
        .components
        .push(section(lang.t("Summary", "Zusammenfassung"), 1));
    report.components.push(if failed == 0 {
        callout(
            "success",
            lang.t("All tests passed", "Alle Tests bestanden"),
            &format!("{passed} {} {tests}.", lang.t("of", "von")),
        )
    } else {
        callout(
            "error",
            lang.t("Tests failed", "Tests fehlgeschlagen"),
            &format!(
                "{failed} {} {tests} {}.",
                lang.t("of", "von"),
                lang.t("tests failed", "Tests sind fehlgeschlagen")
            ),
        )
    });
    report.components.push(component(
        "metric-strip",
        json!({ "items": [
            { "label": lang.t("Tests", "Tests"), "value": tests.to_string() },
            { "label": lang.t("Passed", "Bestanden"), "value": passed.to_string() },
            { "label": lang.t("Failed", "Fehlgeschlagen"), "value": failed.to_string() },
            { "label": lang.t("Skipped", "Übersprungen"), "value": skipped.to_string() },
            { "label": lang.t("Duration", "Dauer"), "value": format_number(time), "unit": "s" },
        ] }),
    ));

    let failures: Vec<&Case> = cases
        .iter()
        .filter(|c| matches!(c.outcome, Outcome::Failure | Outcome::Error))
        .collect();
    if !failures.is_empty() {
        report
            .components
            .push(section(lang.t("Failures", "Fehlschläge"), 1));
        let suite_names: Vec<&str> = suites.iter().map(|s| s.name.as_str()).collect();
        for case in failures.iter().take(MAX_FINDINGS) {
            let title = match (case.classname.is_empty(), case.name.is_empty()) {
                (false, false) => format!("{}.{}", case.classname, case.name),
                (true, false) => case.name.clone(),
                _ => case.classname.clone(),
            };
            let detail = plain(case.detail.trim());
            let description = match (case.message.is_empty(), detail.is_empty()) {
                (false, false) if !detail.starts_with(&case.message) => {
                    format!("{}\n{detail}", plain(&case.message))
                }
                (false, _) => plain(&case.message),
                (true, false) => detail,
                (true, true) => lang.t("(no message)", "(keine Meldung)").into(),
            };
            let mut data = json!({
                "title": if title.is_empty() { "(unnamed)".to_string() } else { title },
                "severity": if matches!(case.outcome, Outcome::Error) { "critical" } else { "high" },
                "description": truncate(&description, 600),
            });
            if let Some(name) = suite_names.get(case.suite).filter(|n| !n.is_empty()) {
                data["category"] = (*name).into();
            }
            report.components.push(component("finding", data));
        }
        if failures.len() > MAX_FINDINGS {
            report.components.push(callout(
                "info",
                lang.t("Failures shortened", "Fehlschläge gekürzt"),
                &format!(
                    "{} {MAX_FINDINGS} {} {}.",
                    lang.t("Showing the first", "Gezeigt werden die ersten"),
                    lang.t("of", "von"),
                    failures.len()
                ),
            ));
        }
    }

    report
        .components
        .push(section(lang.t("Suites", "Suites"), 1));
    let rows: Vec<Vec<String>> = suites
        .iter()
        .map(|s| {
            vec![
                s.name.clone(),
                s.tests.to_string(),
                s.tests.saturating_sub(s.failed + s.skipped).to_string(),
                s.failed.to_string(),
                s.skipped.to_string(),
                format_number(s.time),
            ]
        })
        .collect();
    let right = |header: &str| json!({ "header": header, "align": "right" });
    report.components.push(component(
        "table",
        json!({
            "columns": [
                { "header": lang.t("Suite", "Suite") },
                right(lang.t("Tests", "Tests")),
                right(lang.t("Passed", "Bestanden")),
                right(lang.t("Failed", "Fehlgeschlagen")),
                right(lang.t("Skipped", "Übersprungen")),
                right(lang.t("Time (s)", "Zeit (s)")),
            ],
            "rows": rows,
        }),
    ));
    Ok(report)
}

/// Splits the XML into open tags, close tags and text.
fn scan(xml: &str) -> Result<Vec<Event>, String> {
    let mut events = Vec::new();
    let mut rest = xml.strip_prefix('\u{feff}').unwrap_or(xml);
    while !rest.is_empty() {
        let Some(start) = rest.find('<') else {
            events.push(Event::Text(decode(rest)));
            break;
        };
        if start > 0 {
            events.push(Event::Text(decode(&rest[..start])));
        }
        rest = &rest[start..];
        let skip = |end: &str, rest: &str| -> Result<usize, String> {
            rest.find(end).map(|n| n + end.len()).ok_or_else(|| {
                format!(
                    "junit: unterminated `{}` in the XML",
                    &rest[..end.len().min(rest.len())]
                )
            })
        };
        if rest.starts_with("<!--") {
            rest = &rest[skip("-->", rest)?..];
        } else if let Some(cdata) = rest.strip_prefix("<![CDATA[") {
            let end = cdata
                .find("]]>")
                .ok_or("junit: unterminated CDATA section")?;
            events.push(Event::Text(cdata[..end].to_string()));
            rest = &cdata[end + 3..];
        } else if rest.starts_with("<?") {
            rest = &rest[skip("?>", rest)?..];
        } else if rest.starts_with("<!") {
            rest = &rest[skip(">", rest)?..];
        } else if let Some(close) = rest.strip_prefix("</") {
            let end = close.find('>').ok_or("junit: unterminated closing tag")?;
            events.push(Event::Close(local_name(close[..end].trim())));
            rest = &close[end + 1..];
        } else {
            let end = tag_end(rest).ok_or_else(|| {
                format!(
                    "junit: unterminated tag `{}`",
                    truncate(&rest[..rest.len().min(40)], 40)
                )
            })?;
            let tag = &rest[1..end];
            let empty = tag.ends_with('/');
            let tag = tag.trim_end_matches('/');
            let name_len = tag.find(|c: char| c.is_whitespace()).unwrap_or(tag.len());
            events.push(Event::Open {
                name: local_name(&tag[..name_len]),
                attrs: attributes(&tag[name_len..]),
                empty,
            });
            if empty {
                events.push(Event::Close(local_name(&tag[..name_len])));
            }
            rest = &rest[end + 1..];
        }
    }
    Ok(events)
}

/// Index of the `>` that ends the tag at the start of `rest`, quotes respected.
fn tag_end(rest: &str) -> Option<usize> {
    let mut quote = None;
    for (i, c) in rest.char_indices() {
        match (quote, c) {
            (None, '"' | '\'') => quote = Some(c),
            (Some(q), c) if c == q => quote = None,
            (None, '>') => return Some(i),
            _ => {}
        }
    }
    None
}

/// A tag name without its namespace prefix.
fn local_name(name: &str) -> String {
    name.rsplit(':').next().unwrap_or(name).to_string()
}

fn attributes(text: &str) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    let mut rest = text.trim();
    while let Some((key, tail)) = rest.split_once('=') {
        let tail = tail.trim_start();
        let Some(quote) = tail.chars().next().filter(|c| matches!(c, '"' | '\'')) else {
            break;
        };
        let Some(end) = tail[1..].find(quote) else {
            break;
        };
        attrs.push((key.trim().to_string(), decode(&tail[1..1 + end])));
        rest = tail[end + 2..].trim_start();
    }
    attrs
}

/// Resolves the five predefined entities and numeric character references.
fn decode(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let decoded = rest.find(';').and_then(|end| {
            let name = &rest[1..end];
            let c = match name {
                "lt" => '<',
                "gt" => '>',
                "amp" => '&',
                "quot" => '"',
                "apos" => '\'',
                _ => {
                    let digits = name.strip_prefix('#')?;
                    let code = match digits.strip_prefix(['x', 'X']) {
                        Some(hex) => u32::from_str_radix(hex, 16).ok()?,
                        None => digits.parse().ok()?,
                    };
                    char::from_u32(code)?
                }
            };
            Some((c, end + 1))
        });
        match decoded {
            Some((c, len)) => {
                out.push(c);
                rest = &rest[len..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_entities() {
        assert_eq!(
            decode("a &lt; b &amp;&amp; c &#65;&#x42; &bogus;"),
            "a < b && c AB &bogus;"
        );
    }

    #[test]
    fn reads_attributes_with_both_quote_styles() {
        let attrs = attributes(r#" name="a b" time='1.5' message="x &gt; y""#);
        assert_eq!(attrs[0], ("name".into(), "a b".into()));
        assert_eq!(attrs[1], ("time".into(), "1.5".into()));
        assert_eq!(attrs[2], ("message".into(), "x > y".into()));
    }
}
