//! Lighthouse JSON (`lighthouse --output json`): category scores, the Core
//! Web Vitals and the audits that fall short.

use serde_json::{json, Value};

use super::{
    component, grid, plain, require, section, text_block, truncate, Options, Report, Source,
};

/// The metrics shown as cards, in this order.
const VITALS: [&str; 5] = [
    "first-contentful-paint",
    "largest-contentful-paint",
    "total-blocking-time",
    "cumulative-layout-shift",
    "speed-index",
];
/// Audits below this score are findings.
/// Categories in Lighthouse's order; others follow.
const CATEGORY_ORDER: [&str; 4] = ["performance", "accessibility", "best-practices", "seo"];
const PASS_SCORE: f64 = 0.9;
const MAX_FINDINGS: usize = 25;

pub(super) fn build(source: &Source, options: &Options) -> Result<Report, String> {
    let lang = options.lang();
    let doc: Value = serde_json::from_str(source.text)
        .map_err(|e| format!("lighthouse: not valid JSON: {e}"))?;
    let mut categories = require(
        &doc,
        "categories",
        "lighthouse",
        "(is this `lighthouse --output json`?)",
    )?
    .as_object()
    .ok_or("lighthouse: `categories` is not an object")?
    .iter()
    .collect::<Vec<_>>();
    // The JSON map is sorted by key; Lighthouse's own order is the report order.
    categories.sort_by_key(|(id, _)| CATEGORY_ORDER.iter().position(|o| o == id));
    let audits = require(
        &doc,
        "audits",
        "lighthouse",
        "(is this `lighthouse --output json`?)",
    )?
    .as_object()
    .ok_or("lighthouse: `audits` is not an object")?;

    let mut report = Report {
        title: "Lighthouse report".into(),
        subtitle: doc
            .get("finalDisplayedUrl")
            .or_else(|| doc.get("finalUrl"))
            .or_else(|| doc.get("requestedUrl"))
            .and_then(Value::as_str)
            .map(String::from),
        ..Report::default()
    };
    if let Some(date) = doc.get("fetchTime").and_then(Value::as_str) {
        report.metadata.insert(
            "date".into(),
            date.chars().take(10).collect::<String>().into(),
        );
    }

    // Category scores.
    let cards: Vec<Value> = categories
        .iter()
        .filter_map(|(_, c)| {
            let score = c.get("score")?.as_f64()?;
            Some(component(
                "score-card",
                json!({
                    "title": c.get("title").and_then(Value::as_str).unwrap_or("?"),
                    "score": (score * 100.0).round() as u32,
                }),
            ))
        })
        .collect();
    if cards.is_empty() {
        return Err("lighthouse: no category has a score (`categories.*.score`)".into());
    }
    report
        .components
        .push(section(lang.t("Scores", "Bewertungen"), 1));
    let columns = if cards.len() == 4 {
        2
    } else {
        cards.len().min(3)
    };
    report.components.push(grid(columns, cards));

    // Core Web Vitals.
    let vitals: Vec<Value> = VITALS
        .iter()
        .filter_map(|id| {
            let audit = audits.get(*id)?;
            let value = audit
                .get("displayValue")
                .and_then(Value::as_str)
                .map(String::from)
                .or_else(|| {
                    audit
                        .get("numericValue")?
                        .as_f64()
                        .map(super::format_number)
                })?;
            let title = audit.get("title").and_then(Value::as_str).unwrap_or(id);
            let mut data = json!({ "title": title, "value": value });
            if let Some(score) = audit.get("score").and_then(Value::as_f64) {
                data["accent_color"] = accent(score).into();
            }
            Some(component("metric-card", data))
        })
        .collect();
    if !vitals.is_empty() {
        report
            .components
            .push(section(lang.t("Core Web Vitals", "Core Web Vitals"), 1));
        report.components.push(grid(3, vitals));
    }

    // Audits below the pass score, worst first. Only audits that count
    // towards a category score (weight > 0) are findings.
    let mut weighted: Vec<(&str, &str)> = Vec::new();
    for (_, category) in &categories {
        let title = category.get("title").and_then(Value::as_str).unwrap_or("");
        for reference in category
            .get("auditRefs")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let weight = reference
                .get("weight")
                .and_then(Value::as_f64)
                .unwrap_or(0.0);
            if let (true, Some(id)) = (weight > 0.0, reference.get("id").and_then(Value::as_str)) {
                if !weighted.iter().any(|(seen, _)| *seen == id) {
                    weighted.push((id, title));
                }
            }
        }
    }
    let mut failing: Vec<(f64, &str, &Value)> = weighted
        .iter()
        .filter_map(|(id, category)| {
            let audit = audits.get(*id)?;
            let score = audit.get("score")?.as_f64()?;
            (score < PASS_SCORE).then_some((score, *category, audit))
        })
        .collect();
    failing.sort_by(|a, b| a.0.total_cmp(&b.0));
    if !failing.is_empty() {
        report
            .components
            .push(section(lang.t("Findings", "Befunde"), 1));
        let total = failing.len();
        for (score, category, audit) in failing.into_iter().take(MAX_FINDINGS) {
            let title = audit.get("title").and_then(Value::as_str).unwrap_or("?");
            let description = audit
                .get("description")
                .and_then(Value::as_str)
                .map(plain)
                .filter(|d| !d.is_empty())
                .unwrap_or_else(|| title.to_string());
            let mut data = json!({
                "title": title,
                "severity": severity(score),
                "description": truncate(&description, 400),
                "category": category,
            });
            if let Some(value) = audit.get("displayValue").and_then(Value::as_str) {
                data["affected"] = value.into();
                data["label_affected"] = lang.t("Result:", "Ergebnis:").into();
            }
            report.components.push(component("finding", data));
        }
        if total > MAX_FINDINGS {
            report.components.push(text_block(&format!(
                "{} {MAX_FINDINGS} {} {total} {}.",
                lang.t("Showing the", "Gezeigt werden die"),
                lang.t("lowest-scoring of", "schwächsten von"),
                lang.t("failing audits", "nicht bestandenen Audits"),
            )));
        }
    }
    Ok(report)
}

fn severity(score: f64) -> &'static str {
    if score < 0.5 {
        "high"
    } else if score < 0.75 {
        "medium"
    } else {
        "low"
    }
}

/// Lighthouse's own bands: 0.9 and up good, 0.5 and up needs improvement.
fn accent(score: f64) -> &'static str {
    if score >= PASS_SCORE {
        "#0c7a43"
    } else if score >= 0.5 {
        "#b45309"
    } else {
        "#b91c1c"
    }
}
