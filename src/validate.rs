//! Checks a render request without rendering it.
//!
//! [`validate_request`] finds what [`Engine::render_pdf`](crate::Engine::render_pdf)
//! would otherwise leave unnoticed: component data that does not fit its
//! struct (normalization then skips the component's derived fields) and
//! component types that no catalog or pack defines (rendered as a grey
//! `[type]` placeholder).

use serde_json::Value;

use crate::components::ComponentCatalog;
use crate::render::RenderRequest;

/// One problem found in a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    /// Location in the request, e.g. `components[2].data.score`.
    pub path: String,
    /// What is wrong at that location.
    pub message: String,
}

impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

/// All problems in `request_json`; empty when the request is valid.
///
/// `extra_components` names component types defined outside the built-in
/// catalog (the loaded pack's components); their data is not checked.
pub fn validate_request(request_json: &str, extra_components: &[&str]) -> Vec<Issue> {
    let value: Value = match serde_json::from_str(request_json) {
        Ok(value) => value,
        Err(e) => {
            return vec![Issue {
                path: "(root)".into(),
                message: format!("not valid JSON: {e}"),
            }]
        }
    };
    validate_value(&value, extra_components)
}

/// Like [`validate_request`] for an already parsed request.
pub fn validate_value(request: &Value, extra_components: &[&str]) -> Vec<Issue> {
    let mut issues = Vec::new();

    // Request shape: required fields, field types, page setup, theme.
    if let Err(e) = serde_path_to_error::deserialize::<_, RenderRequest>(request) {
        let path = e.path().to_string();
        issues.push(Issue {
            path: if path == "." { "(root)".into() } else { path },
            message: e.inner().to_string(),
        });
        return issues;
    }

    if let Some(Value::Array(components)) = request.get("components") {
        for (i, component) in components.iter().enumerate() {
            check_component(
                component,
                &format!("components[{i}]"),
                extra_components,
                &mut issues,
            );
        }
    }
    issues
}

/// Checks one `{type, data}` component and, depth-first, every component
/// nested in its data (grid items, flow groups, columns).
fn check_component(value: &Value, path: &str, extra: &[&str], issues: &mut Vec<Issue>) {
    let Some(map) = value.as_object() else {
        issues.push(Issue {
            path: path.into(),
            message: "a component must be an object with `type` and `data`".into(),
        });
        return;
    };
    let Some(id) = map.get("type").and_then(Value::as_str) else {
        issues.push(Issue {
            path: path.into(),
            message: "missing string field `type`".into(),
        });
        return;
    };
    let Some(data) = map.get("data") else {
        issues.push(Issue {
            path: path.into(),
            message: format!("component `{id}` has no `data`"),
        });
        return;
    };

    // A name the request's pack defines wins over a built-in alias.
    let descriptor = ComponentCatalog::get(id).filter(|d| d.id == id || !extra.contains(&id));
    match descriptor {
        Some(descriptor) => {
            if let Some(validate) = descriptor.validate {
                if let Err(message) = validate(data) {
                    issues.push(Issue {
                        path: format!("{path}.data"),
                        message: format!("{id}: {message}"),
                    });
                }
            }
        }
        None if extra.contains(&id) => {}
        None => issues.push(Issue {
            path: format!("{path}.type"),
            message: format!("unknown component type `{id}` (see `list-components`)"),
        }),
    }

    check_nested(data, &format!("{path}.data"), extra, issues);
}

/// Finds `{type, data}` objects below `value` and checks each of them.
fn check_nested(value: &Value, path: &str, extra: &[&str], issues: &mut Vec<Issue>) {
    match value {
        Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                check_nested(item, &format!("{path}[{i}]"), extra, issues);
            }
        }
        Value::Object(map) => {
            if map.get("type").is_some_and(Value::is_string) && map.contains_key("data") {
                check_component(value, path, extra, issues);
            } else {
                for (key, item) in map {
                    check_nested(item, &format!("{path}.{key}"), extra, issues);
                }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn issues(request: Value) -> Vec<Issue> {
        validate_value(&request, &[])
    }

    #[test]
    fn accepts_a_valid_request() {
        let found = issues(json!({
            "template_id": "default",
            "components": [{ "type": "score-card", "data": { "title": "SEO", "score": 64 } }]
        }));
        assert_eq!(found, vec![]);
    }

    #[test]
    fn names_the_failing_field() {
        let found = issues(json!({
            "template_id": "default",
            "components": [{ "type": "score-card", "data": { "title": "SEO", "score": "high" } }]
        }));
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, "components[0].data");
        assert!(found[0].message.contains("score"), "{}", found[0]);
    }

    #[test]
    fn reports_unknown_types_unless_a_pack_defines_them() {
        let request = json!({
            "template_id": "default",
            "components": [{ "type": "my-widget", "data": {} }]
        });
        assert_eq!(validate_value(&request, &[]).len(), 1);
        assert!(validate_value(&request, &["my-widget"]).is_empty());
    }

    #[test]
    fn checks_nested_components() {
        let found = issues(json!({
            "template_id": "default",
            "components": [{ "type": "grid-component", "data": { "columns": 2, "items": [
                { "content": { "type": "score-card", "data": { "title": "SEO" } } }
            ] } }]
        }));
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path, "components[0].data.items[0].content.data");
    }

    #[test]
    fn rejects_a_request_without_template_id() {
        let found = issues(json!({ "components": [] }));
        assert_eq!(found.len(), 1);
        assert!(found[0].message.contains("template_id"));
    }

    #[test]
    fn rejects_invalid_json() {
        assert_eq!(validate_request("{", &[]).len(), 1);
    }
}
