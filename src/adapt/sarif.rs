//! SARIF 2.1.0 (CodeQL, Semgrep, clippy, trivy, …): results by level and each
//! one as a finding.

use serde_json::{json, Value};

use super::{callout, component, plain, require, section, Options, Report, Source};

const MAX_FINDINGS: usize = 100;
/// Result levels in report order; a result without one counts as `warning`.
const LEVELS: [&str; 4] = ["error", "warning", "note", "none"];

pub(super) fn build(source: &Source, options: &Options) -> Result<Report, String> {
    let lang = options.lang();
    let doc: Value =
        serde_json::from_str(source.text).map_err(|e| format!("sarif: not valid JSON: {e}"))?;
    let runs = require(&doc, "runs", "sarif", "(is this a SARIF 2.1.0 file?)")?
        .as_array()
        .ok_or("sarif: `runs` is not an array")?;

    let mut tools: Vec<String> = Vec::new();
    let mut version = None;
    // (level index, finding data)
    let mut results: Vec<(usize, Value)> = Vec::new();
    for (n, run) in runs.iter().enumerate() {
        let driver = run
            .pointer("/tool/driver")
            .ok_or_else(|| format!("sarif: missing `tool.driver` in runs[{n}]"))?;
        let tool = driver
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("sarif: missing `tool.driver.name` in runs[{n}]"))?;
        if !tools.iter().any(|t| t == tool) {
            tools.push(tool.to_string());
        }
        version = version.or_else(|| {
            ["semanticVersion", "version"]
                .iter()
                .find_map(|k| driver.get(*k)?.as_str())
                .map(String::from)
        });
        let rules = driver.get("rules").and_then(Value::as_array);
        let rule_level = |id: &str| -> Option<&str> {
            rules?
                .iter()
                .find(|r| r.get("id").and_then(Value::as_str) == Some(id))?
                .pointer("/defaultConfiguration/level")?
                .as_str()
        };
        for result in run
            .get("results")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let rule = result.get("ruleId").and_then(Value::as_str);
            let level = result
                .get("level")
                .and_then(Value::as_str)
                .or_else(|| rule.and_then(rule_level))
                .unwrap_or("warning");
            let level = LEVELS.iter().position(|l| *l == level).unwrap_or(3);
            let message = result
                .pointer("/message/text")
                .and_then(Value::as_str)
                .unwrap_or("");
            let severity = ["critical", "high", "medium", "info"][level];
            let mut data = json!({
                "title": rule.unwrap_or("(no rule)"),
                "severity": severity,
                "description": if message.is_empty() { "(no message)".into() } else { plain(message) },
                "category": tool,
            });
            if let Some(place) = location(result) {
                data["affected"] = place.into();
                data["label_affected"] = lang.t("Location:", "Fundstelle:").into();
            }
            results.push((level, data));
        }
    }

    let mut report = Report {
        title: format!("{} report", tools.join(", ")),
        subtitle: version.map(|v| format!("{} {v}", tools[0])),
        ..Report::default()
    };
    if results.is_empty() {
        report.components.push(callout(
            "success",
            lang.t("No results", "Keine Ergebnisse"),
            lang.t(
                "The analysis reported nothing.",
                "Die Analyse hat nichts gemeldet.",
            ),
        ));
        return Ok(report);
    }

    let count = |level: usize| results.iter().filter(|(l, _)| *l == level).count();
    report
        .components
        .push(section(lang.t("Overview", "Übersicht"), 1));
    report.components.push(component(
        "severity-overview",
        json!({
            "title": lang.t("Results by level", "Ergebnisse nach Stufe"),
            "critical": count(0),
            "serious": count(1),
            "moderate": count(2),
            "minor": count(3),
            "label_critical": lang.t("Error", "Fehler"),
            "label_serious": lang.t("Warning", "Warnung"),
            "label_moderate": lang.t("Note", "Hinweis"),
            "label_minor": lang.t("Other", "Sonstige"),
            "label_total_suffix": lang.t("results", "Ergebnisse"),
        }),
    ));

    report
        .components
        .push(section(lang.t("Results", "Ergebnisse"), 1));
    let total = results.len();
    results.sort_by_key(|(level, _)| *level);
    for (_, data) in results.into_iter().take(MAX_FINDINGS) {
        report.components.push(component("finding", data));
    }
    if total > MAX_FINDINGS {
        report.components.push(callout(
            "info",
            lang.t("Results shortened", "Ergebnisse gekürzt"),
            &format!(
                "{} {MAX_FINDINGS} {} {total}.",
                lang.t("Showing the first", "Gezeigt werden die ersten"),
                lang.t("of", "von"),
            ),
        ));
    }
    Ok(report)
}

/// `file:line` of the first location.
fn location(result: &Value) -> Option<String> {
    let physical = result.pointer("/locations/0/physicalLocation")?;
    let uri = physical.pointer("/artifactLocation/uri")?.as_str()?;
    match physical
        .pointer("/region/startLine")
        .and_then(Value::as_u64)
    {
        Some(line) => Some(format!("{uri}:{line}")),
        None => Some(uri.to_string()),
    }
}
