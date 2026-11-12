//! Binds a request template to data.
//!
//! A template is a normal render request (as JSON) that may contain three
//! additions. [`bind`] resolves them in a pure JSON-to-JSON step, before the
//! request is deserialized, so the engine itself is unchanged.
//!
//! - **Placeholders:** `"{{ customer.name }}"` in any string value. A string
//!   that consists of one placeholder only takes the JSON type of the value
//!   (number, array, object); otherwise the value is written into the text.
//!   Path segments are separated by dots, array elements are addressed by
//!   index (`items.0.title`). The filters `upper`, `lower` and `round` can be
//!   chained with `|`: `{{ name | upper }}`.
//! - **Repetition:** `{"for": "finding in findings", "component": {…}}` as an
//!   array element is replaced by one `component` per element of the array.
//! - **Condition:** `{"if": "findings", "component": {…}}` as an array element
//!   is kept only when the path exists and is not null, false, 0, `""`, `[]`
//!   or `{}`.
//!
//! Errors name the location in the template and the data path that is missing.

use serde_json::{Map, Value};

/// A problem found while binding a template to data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindError {
    /// Location in the template, e.g. `components[2].data.title`.
    pub path: String,
    /// What is wrong at that location.
    pub message: String,
}

impl std::fmt::Display for BindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for BindError {}

/// Like [`bind`] for JSON text.
pub fn bind_str(template_json: &str, data_json: &str) -> Result<Value, BindError> {
    let parse = |text: &str, what: &str| {
        serde_json::from_str::<Value>(text).map_err(|e| BindError {
            path: format!("({what})"),
            message: format!("not valid JSON: {e}"),
        })
    };
    bind(
        &parse(template_json, "template")?,
        &parse(data_json, "data")?,
    )
}

/// Resolves placeholders, `for` and `if` in `template` against `data`.
pub fn bind(template: &Value, data: &Value) -> Result<Value, BindError> {
    let mut scope = Scope {
        data,
        locals: Vec::new(),
    };
    bind_value(template, "", &mut scope)
}

/// The data and the loop variables that are visible at one place.
struct Scope<'a> {
    data: &'a Value,
    locals: Vec<(String, &'a Value)>,
}

impl<'a> Scope<'a> {
    /// The value at `path`; the error says where the path breaks off.
    fn lookup(&self, path: &str) -> Result<&'a Value, String> {
        let path = path.trim();
        let mut segments = path.split('.');
        let first = segments.next().unwrap_or_default();
        let mut current = match self.locals.iter().rev().find(|(name, _)| name == first) {
            Some((_, value)) => *value,
            None => self
                .data
                .get(first)
                .ok_or_else(|| format!("data path `{path}` not found: no `{first}` in the data"))?,
        };
        let mut walked = first.to_string();
        for segment in segments {
            current = match current {
                Value::Object(map) => map.get(segment),
                Value::Array(items) => segment.parse::<usize>().ok().and_then(|i| items.get(i)),
                _ => None,
            }
            .ok_or_else(|| {
                format!("data path `{path}` not found: `{walked}` has no `{segment}`")
            })?;
            walked.push('.');
            walked.push_str(segment);
        }
        Ok(current)
    }
}

fn error(path: &str, message: impl Into<String>) -> BindError {
    BindError {
        path: if path.is_empty() { "(root)" } else { path }.into(),
        message: message.into(),
    }
}

fn child(path: &str, key: &str) -> String {
    if path.is_empty() {
        key.into()
    } else {
        format!("{path}.{key}")
    }
}

fn bind_value(value: &Value, path: &str, scope: &mut Scope) -> Result<Value, BindError> {
    match value {
        Value::String(text) => bind_string(text, path, scope),
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for (i, item) in items.iter().enumerate() {
                bind_element(item, &format!("{path}[{i}]"), scope, &mut out)?;
            }
            Ok(Value::Array(out))
        }
        Value::Object(map) => {
            if is_directive(value) {
                return Err(error(
                    path,
                    "`for` and `if` can only be used as an element of an array",
                ));
            }
            let mut out = Map::new();
            for (key, item) in map {
                out.insert(key.clone(), bind_value(item, &child(path, key), scope)?);
            }
            Ok(Value::Object(out))
        }
        other => Ok(other.clone()),
    }
}

/// An object `{"for"|"if": …, "component": …}`.
fn is_directive(value: &Value) -> bool {
    value.as_object().is_some_and(|m| {
        m.contains_key("component") && (m.contains_key("for") || m.contains_key("if"))
    })
}

/// Binds one array element; a directive adds zero or more values to `out`.
fn bind_element(
    element: &Value,
    path: &str,
    scope: &mut Scope,
    out: &mut Vec<Value>,
) -> Result<(), BindError> {
    if !is_directive(element) {
        out.push(bind_value(element, path, scope)?);
        return Ok(());
    }
    let map = element.as_object().expect("a directive is an object");
    if let Some(key) = map
        .keys()
        .find(|k| !matches!(k.as_str(), "for" | "if" | "component"))
    {
        return Err(error(
            path,
            format!("unexpected key `{key}` next to `for`/`if`"),
        ));
    }
    if map.contains_key("for") && map.contains_key("if") {
        return Err(error(
            path,
            "use either `for` or `if`, not both; nest them instead",
        ));
    }
    let component = &map["component"];
    let component_path = child(path, "component");

    if let Some(condition) = map.get("if") {
        let expr = condition
            .as_str()
            .ok_or_else(|| error(&child(path, "if"), "`if` must be a data path as a string"))?;
        let keep = scope.lookup(expr).is_ok_and(is_truthy);
        if keep {
            bind_element(component, &component_path, scope, out)?;
        }
        return Ok(());
    }

    let for_path = child(path, "for");
    let spec = map["for"]
        .as_str()
        .ok_or_else(|| error(&for_path, "`for` must be a string like \"item in items\""))?;
    let (variable, list_path) = spec
        .split_once(" in ")
        .map(|(v, l)| (v.trim(), l.trim()))
        .filter(|(v, l)| !v.is_empty() && !v.contains(char::is_whitespace) && !l.is_empty())
        .ok_or_else(|| {
            error(
                &for_path,
                format!("`{spec}` is not of the form \"item in items\""),
            )
        })?;
    let list = scope.lookup(list_path).map_err(|m| error(&for_path, m))?;
    let Value::Array(items) = list else {
        return Err(error(&for_path, format!("`{list_path}` is not an array")));
    };
    for item in items {
        scope.locals.push((variable.to_string(), item));
        let result = bind_element(component, &component_path, scope, out);
        scope.locals.pop();
        result?;
    }
    Ok(())
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().is_some_and(|f| f != 0.0),
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    }
}

fn bind_string(text: &str, path: &str, scope: &Scope) -> Result<Value, BindError> {
    if !text.contains("{{") {
        return Ok(Value::String(text.into()));
    }
    // Split into literal text and placeholder expressions.
    let mut parts: Vec<(bool, &str)> = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("{{") {
        if start > 0 {
            parts.push((false, &rest[..start]));
        }
        let after = &rest[start + 2..];
        let end = after
            .find("}}")
            .ok_or_else(|| error(path, format!("unclosed `{{{{` in \"{text}\"")))?;
        parts.push((true, &after[..end]));
        rest = &after[end + 2..];
    }
    if !rest.is_empty() {
        parts.push((false, rest));
    }

    if let [(true, expr)] = parts.as_slice() {
        return evaluate(expr, path, scope);
    }
    let mut out = String::new();
    for (is_expr, part) in parts {
        if !is_expr {
            out.push_str(part);
            continue;
        }
        match evaluate(part, path, scope)? {
            Value::String(s) => out.push_str(&s),
            Value::Number(n) => out.push_str(&n.to_string()),
            Value::Bool(b) => out.push_str(&b.to_string()),
            Value::Null => {}
            _ => {
                return Err(error(
                    path,
                    format!(
                        "`{}` is an array or object and cannot be written into text; \
                         use it as the whole string",
                        part.trim()
                    ),
                ))
            }
        }
    }
    Ok(Value::String(out))
}

/// `path | filter | filter`
fn evaluate(expr: &str, path: &str, scope: &Scope) -> Result<Value, BindError> {
    let mut pieces = expr.split('|');
    let data_path = pieces.next().unwrap_or_default();
    let mut value = scope.lookup(data_path).map_err(|m| error(path, m))?.clone();
    for filter in pieces {
        let filter = filter.trim();
        value = match (filter, &value) {
            ("upper", Value::String(s)) => Value::String(s.to_uppercase()),
            ("lower", Value::String(s)) => Value::String(s.to_lowercase()),
            ("round", Value::Number(n)) => match n.as_f64().map(f64::round) {
                Some(f) if n.is_f64() && f.abs() < 9e15 => Value::from(f as i64),
                _ => value.clone(),
            },
            ("upper" | "lower" | "round", _) => {
                let needs = if filter == "round" {
                    "a number"
                } else {
                    "a string"
                };
                return Err(error(
                    path,
                    format!(
                        "filter `{filter}` needs {needs}, but `{}` is not",
                        data_path.trim()
                    ),
                ));
            }
            _ => {
                return Err(error(
                    path,
                    format!("unknown filter `{filter}` (available: upper, lower, round)"),
                ))
            }
        };
    }
    Ok(value)
}

/// Parses CSV text into an array of row objects keyed by the header line.
///
/// Fields may be quoted with `"` (`""` is a quote, line breaks are allowed
/// inside); the separator is a comma. A cell that reads back unchanged as a
/// number becomes a number (`42`, `3.5`), every other cell stays a string, so
/// `007` and `1.50` are kept as written. Blank lines are skipped.
pub fn parse_csv(text: &str) -> Result<Value, String> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut record: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => record.push(std::mem::take(&mut field)),
            '\r' if !quoted => {}
            '\n' if !quoted => {
                record.push(std::mem::take(&mut field));
                records.push(std::mem::take(&mut record));
            }
            other => field.push(other),
        }
    }
    if quoted {
        return Err("unterminated quoted field".into());
    }
    if !field.is_empty() || !record.is_empty() {
        record.push(field);
        records.push(record);
    }
    records.retain(|r| !(r.len() == 1 && r[0].is_empty()));

    let mut records = records.into_iter();
    let header = records.next().ok_or("no header line")?;
    let mut rows = Vec::new();
    for (i, record) in records.enumerate() {
        if record.len() != header.len() {
            return Err(format!(
                "row {} has {} fields, the header has {}",
                i + 1,
                record.len(),
                header.len()
            ));
        }
        let row = header
            .iter()
            .zip(record)
            .map(|(name, cell)| (name.clone(), csv_cell(cell)))
            .collect();
        rows.push(Value::Object(row));
    }
    Ok(Value::Array(rows))
}

fn csv_cell(cell: String) -> Value {
    if let Ok(n) = cell.parse::<i64>() {
        if n.to_string() == cell {
            return Value::from(n);
        }
    }
    if let Ok(f) = cell.parse::<f64>() {
        if f.is_finite() && f.to_string() == cell {
            return Value::from(f);
        }
    }
    Value::String(cell)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn run(template: Value, data: Value) -> Result<Value, BindError> {
        bind(&template, &data)
    }

    #[test]
    fn text_placeholders_are_interpolated() {
        let out = run(
            json!({"title": "Report for {{ client.name }} ({{ year }})"}),
            json!({"client": {"name": "Acme"}, "year": 2026}),
        )
        .unwrap();
        assert_eq!(out["title"], "Report for Acme (2026)");
    }

    #[test]
    fn a_lone_placeholder_keeps_the_json_type() {
        let out = run(
            json!({"score": "{{ s }}", "tags": "{{ t }}", "plain": "{{x}}"}),
            json!({"s": 87, "t": ["a", "b"], "x": true}),
        )
        .unwrap();
        assert_eq!(out["score"], 87);
        assert_eq!(out["tags"], json!(["a", "b"]));
        assert_eq!(out["plain"], true);
    }

    #[test]
    fn indexes_address_array_elements() {
        let out = run(
            json!("{{ rows.1.name }}"),
            json!({"rows": [{"name": "a"}, {"name": "b"}]}),
        );
        assert_eq!(out.unwrap(), "b");
    }

    #[test]
    fn filters_are_applied_in_order() {
        let out = run(
            json!([
                "{{ n | upper }}",
                "{{ f | round }}",
                "Score {{ f | round }}",
                "{{ n | upper | lower }}"
            ]),
            json!({"n": "abc", "f": 2.6}),
        )
        .unwrap();
        assert_eq!(out, json!(["ABC", 3, "Score 3", "abc"]));
    }

    #[test]
    fn filter_errors_name_the_filter() {
        let e = run(json!("{{ n | upper }}"), json!({"n": 1})).unwrap_err();
        assert!(e.message.contains("`upper` needs a string"), "{e}");
        let e = run(json!("{{ n | shout }}"), json!({"n": "a"})).unwrap_err();
        assert!(e.message.contains("unknown filter `shout`"), "{e}");
    }

    #[test]
    fn missing_data_reports_template_and_data_path() {
        let e = run(
            json!({"components": [{"data": {"title": "{{ client.name }}"}}]}),
            json!({"client": {}}),
        )
        .unwrap_err();
        assert_eq!(e.path, "components[0].data.title");
        assert!(
            e.message.contains("`client.name`") && e.message.contains("`client` has no `name`"),
            "{e}"
        );
        let e = run(json!("{{ nope }}"), json!({})).unwrap_err();
        assert_eq!(e.path, "(root)");
        assert!(e.message.contains("no `nope` in the data"), "{e}");
    }

    #[test]
    fn unclosed_placeholder_is_an_error() {
        assert!(run(json!("a {{ b"), json!({})).is_err());
    }

    #[test]
    fn structured_values_cannot_be_written_into_text() {
        let e = run(json!("x {{ l }}"), json!({"l": [1]})).unwrap_err();
        assert!(e.message.contains("cannot be written into text"), "{e}");
    }

    #[test]
    fn for_repeats_the_component() {
        let out = run(
            json!({"components": [
                {"type": "section", "data": {"title": "T"}},
                {"for": "f in findings", "component": {"type": "finding", "data": {"title": "{{ f.title }}", "n": "{{ f.n }}"}}}
            ]}),
            json!({"findings": [{"title": "a", "n": 1}, {"title": "b", "n": 2}]}),
        )
        .unwrap();
        let components = out["components"].as_array().unwrap();
        assert_eq!(components.len(), 3);
        assert_eq!(components[2]["data"], json!({"title": "b", "n": 2}));
    }

    #[test]
    fn for_over_an_empty_array_yields_nothing() {
        let out = run(
            json!([{"for": "x in xs", "component": "{{ x }}"}]),
            json!({"xs": []}),
        )
        .unwrap();
        assert_eq!(out, json!([]));
    }

    #[test]
    fn for_errors() {
        let e = run(
            json!([{"for": "x in xs", "component": 1}]),
            json!({"xs": 3}),
        )
        .unwrap_err();
        assert_eq!(e.path, "[0].for");
        assert!(e.message.contains("not an array"), "{e}");
        let e = run(json!([{"for": "xs", "component": 1}]), json!({})).unwrap_err();
        assert!(e.message.contains("item in items"), "{e}");
        let e = run(json!([{"for": "x in nope", "component": 1}]), json!({})).unwrap_err();
        assert!(e.message.contains("`nope`"), "{e}");
    }

    #[test]
    fn if_keeps_or_drops_the_component() {
        let template = json!([{"if": "v", "component": "kept"}]);
        for (v, kept) in [
            (json!(1), true),
            (json!("x"), true),
            (json!([0]), true),
            (json!(0), false),
            (json!(""), false),
            (json!([]), false),
            (json!({}), false),
            (json!(null), false),
            (json!(false), false),
        ] {
            let out = run(template.clone(), json!({ "v": v })).unwrap();
            assert_eq!(out == json!(["kept"]), kept, "{v}");
        }
        assert_eq!(run(template, json!({})).unwrap(), json!([]));
    }

    #[test]
    fn directives_nest_and_work_inside_component_data() {
        let template = json!({"components": [{"type": "grid-component", "data": {"items": [
            {"for": "g in groups", "component": {"content": [
                {"if": "g.items", "component": {"for": "i in g.items", "component": "{{ g.name }}:{{ i }}"}}
            ]}}
        ]}}]});
        let data = json!({"groups": [{"name": "a", "items": [1, 2]}, {"name": "b", "items": []}]});
        let out = run(template, data).unwrap();
        assert_eq!(
            out["components"][0]["data"]["items"],
            json!([{"content": ["a:1", "a:2"]}, {"content": []}])
        );
    }

    #[test]
    fn loop_variable_shadows_data_and_is_scoped() {
        let out = run(
            json!([{"for": "x in xs", "component": "{{ x }}"}, "{{ x }}"]),
            json!({"xs": ["in"], "x": "out"}),
        )
        .unwrap();
        assert_eq!(out, json!(["in", "out"]));
    }

    #[test]
    fn directives_outside_arrays_and_mixed_keys_are_rejected() {
        let e = run(json!({"a": {"if": "x", "component": 1}}), json!({"x": 1})).unwrap_err();
        assert_eq!(e.path, "a");
        let e = run(
            json!([{"if": "x", "for": "a in b", "component": 1}]),
            json!({}),
        )
        .unwrap_err();
        assert!(e.message.contains("not both"), "{e}");
        let e = run(json!([{"if": "x", "component": 1, "else": 2}]), json!({})).unwrap_err();
        assert!(e.message.contains("`else`"), "{e}");
    }

    #[test]
    fn bind_str_reports_bad_json() {
        assert_eq!(bind_str("{", "{}").unwrap_err().path, "(template)");
        assert_eq!(bind_str("{}", "{").unwrap_err().path, "(data)");
        assert_eq!(bind_str("\"{{ a }}\"", "{\"a\": 1}").unwrap(), json!(1));
    }

    #[test]
    fn csv_becomes_rows_with_numbers_where_exact() {
        let rows = parse_csv(
            "\u{feff}name,score,id\r\n\"Doe, J\",42,007\r\n\"say \"\"hi\"\"\",3.5,\"a\nb\"\r\n\r\n",
        )
        .unwrap();
        assert_eq!(
            rows,
            json!([
                {"name": "Doe, J", "score": 42, "id": "007"},
                {"name": "say \"hi\"", "score": 3.5, "id": "a\nb"}
            ])
        );
    }

    #[test]
    fn csv_errors() {
        assert!(parse_csv("").is_err());
        assert!(parse_csv("a,b\n1\n").unwrap_err().contains("row 1"));
        assert!(parse_csv("a\n\"x\n").is_err());
        assert_eq!(parse_csv("a,b").unwrap(), json!([]));
    }
}
