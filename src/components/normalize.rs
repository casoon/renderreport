//! Fills in what a component's Rust builder would have computed, for
//! component data that arrives as JSON (CLI, Wasm, a `RenderRequest` built by
//! hand).
//!
//! Templates read fields that `Component::to_data` derives — a score card's
//! `computed_status`, a word search's grid — and that a JSON author cannot be
//! expected to supply. [`normalize_components`] runs each built-in
//! component's data through its struct (serde defaults, then `to_data`), so a
//! request needs only the fields the builder itself requires.

use serde::de::DeserializeOwned;
use serde_json::Value;

use super::{Component, ComponentCatalog};

/// Normalizes every component in `components`, including components nested
/// in other components' data (grid items, flow groups, columns).
///
/// Data that does not deserialize into the component's struct — a missing
/// required field, a wrong type, or data that is already the output of
/// `to_data` — is left as it is, so the render behaves as before.
pub fn normalize_components(components: &mut [Value]) {
    for component in components {
        normalize_value(component);
    }
}

/// Walks the JSON tree depth-first, so nested components are normalized
/// before the component that contains them.
fn normalize_value(value: &mut Value) {
    match value {
        Value::Array(items) => items.iter_mut().for_each(normalize_value),
        Value::Object(map) => {
            map.values_mut().for_each(normalize_value);
            let Some(Value::String(id)) = map.get("type") else {
                return;
            };
            let id = id.clone();
            if let Some(Value::Object(data)) = map.get_mut("data") {
                if let Some(Value::Object(computed)) =
                    normalize_data(&id, &Value::Object(data.clone()))
                {
                    // Keys the struct does not know stay; the struct's values win.
                    data.extend(computed);
                }
            }
        }
        _ => {}
    }
}

/// `to_data` of the built-in component `id` deserialized from `data`, or
/// `None` for unknown ids and data that does not fit the struct.
fn normalize_data(id: &str, data: &Value) -> Option<Value> {
    ComponentCatalog::get(id)?.normalize?(data)
}

/// The [`NormalizeFn`](super::catalog::NormalizeFn) for component struct `T`.
pub fn typed<T: Component + DeserializeOwned>(data: &Value) -> Option<Value> {
    serde_json::from_value::<T>(data.clone())
        .ok()
        .map(|c| c.to_data())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn normalized(component: Value) -> Value {
        let mut components = vec![component];
        normalize_components(&mut components);
        components.remove(0)
    }

    #[test]
    fn fills_in_derived_fields() {
        let card =
            normalized(json!({ "type": "score-card", "data": { "title": "SEO", "score": 64 } }));
        assert_eq!(card["data"]["computed_status"], "warning");
        assert_eq!(card["data"]["max_score"], 100);

        let callout = normalized(
            json!({ "type": "callout", "data": { "variant": "warning", "content": "x" } }),
        );
        assert_eq!(callout["data"]["callout_type"], "warning");
    }

    #[test]
    fn normalizes_nested_components() {
        let grid = normalized(json!({
            "type": "grid-component",
            "data": { "columns": 2, "items": [{ "content": {
                "type": "score-card", "data": { "title": "SEO", "score": 95 }
            } }] }
        }));
        assert_eq!(
            grid["data"]["items"][0]["content"]["data"]["computed_status"],
            "good"
        );
    }

    #[test]
    fn keeps_unknown_fields() {
        let card = normalized(json!({
            "type": "score-card", "data": { "title": "SEO", "score": 64, "note": "kept" }
        }));
        assert_eq!(card["data"]["note"], "kept");
    }

    #[test]
    fn leaves_data_alone_that_does_not_fit() {
        let wrong_type =
            json!({ "type": "score-card", "data": { "title": "SEO", "score": "high" } });
        assert_eq!(normalized(wrong_type.clone()), wrong_type);

        let unknown = json!({ "type": "my-widget", "data": { "label": "x" } });
        assert_eq!(normalized(unknown.clone()), unknown);
    }

    #[test]
    fn is_idempotent_on_builder_output() {
        let data = crate::components::WordSearch::builder()
            .words(vec!["CAT", "DOG"])
            .seed(7)
            .build()
            .unwrap()
            .to_data();
        let component = json!({ "type": "word-search", "data": data });
        assert_eq!(normalized(component.clone()), component);
    }
}
