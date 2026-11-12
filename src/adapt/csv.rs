//! CSV: figures, per-column statistics, a bar chart and the table itself.

use serde_json::{json, Value};

use super::{component, format_number, section, Options, Report, Source};
use crate::render::bind::parse_csv_table;

/// Numeric columns that get their own statistics.
const MAX_STAT_COLUMNS: usize = 6;
/// Rows in the bar chart.
const CHART_ROWS: usize = 12;

pub(super) fn build(source: &Source, options: &Options) -> Result<Report, String> {
    let lang = options.lang();
    let (header, rows) = parse_csv_table(source.text).map_err(|e| format!("csv: {e}"))?;
    if rows.is_empty() {
        return Err("csv: the file has a header line but no data rows".into());
    }

    let numeric: Vec<bool> = (0..header.len())
        .map(|c| {
            let cells = || rows.iter().map(|r| &r[c]).filter(|v| !is_empty(v));
            cells().next().is_some() && cells().all(Value::is_number)
        })
        .collect();
    let numbers = |c: usize| -> Vec<f64> { rows.iter().filter_map(|r| r[c].as_f64()).collect() };

    let mut report = Report {
        title: source.stem.to_string(),
        ..Report::default()
    };
    report
        .components
        .push(section(lang.t("Overview", "Übersicht"), 1));
    report.components.push(component(
        "metric-strip",
        json!({ "items": [
            { "label": lang.t("Rows", "Zeilen"), "value": rows.len().to_string() },
            { "label": lang.t("Columns", "Spalten"), "value": header.len().to_string() },
        ] }),
    ));

    for c in (0..header.len())
        .filter(|&c| numeric[c])
        .take(MAX_STAT_COLUMNS)
    {
        let values = numbers(c);
        let sum: f64 = values.iter().sum();
        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        report.components.push(section(&header[c], 2));
        report.components.push(component(
            "metric-strip",
            json!({ "items": [
                { "label": lang.t("Sum", "Summe"), "value": format_number(sum) },
                { "label": lang.t("Mean", "Mittelwert"), "value": format_number(sum / values.len() as f64) },
                { "label": "Min", "value": format_number(min) },
                { "label": "Max", "value": format_number(max) },
            ] }),
        ));
    }

    let label_column = (0..header.len()).find(|&c| !numeric[c]);
    let value_column = (0..header.len()).find(|&c| numeric[c]);
    if let (Some(label), Some(value)) = (label_column, value_column) {
        let points: Vec<Value> = rows
            .iter()
            .filter(|r| r[value].is_number())
            .take(CHART_ROWS)
            .map(|r| json!({ "label": cell_text(&r[label]), "value": r[value] }))
            .collect();
        let note = if rows.len() > CHART_ROWS {
            format!(
                " ({} {CHART_ROWS} {})",
                lang.t("first", "erste"),
                lang.t("rows", "Zeilen")
            )
        } else {
            String::new()
        };
        report.components.push(component(
            "chart",
            json!({
                "title": format!("{} {} {}{note}", header[value], lang.t("by", "nach"), header[label]),
                "chart_type": "bar",
                "horizontal": true,
                "show_legend": false,
                "series": [{ "name": header[value], "data": points }],
            }),
        ));
    }

    report.components.push(section(lang.t("Data", "Daten"), 1));
    let columns: Vec<Value> = header
        .iter()
        .zip(&numeric)
        .map(|(name, &numeric)| {
            if numeric {
                json!({ "header": name, "align": "right" })
            } else {
                json!({ "header": name })
            }
        })
        .collect();
    let table_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|r| r.iter().map(cell_text).collect())
        .collect();
    report.components.push(component(
        "table",
        json!({ "columns": columns, "rows": table_rows }),
    ));
    Ok(report)
}

fn is_empty(value: &Value) -> bool {
    value.as_str().is_some_and(str::is_empty)
}

fn cell_text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}
