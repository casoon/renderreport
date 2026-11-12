//! JSON Schema for render requests and component data (feature `schema`).
//!
//! The schemas come from the component structs, so they describe exactly
//! what the structs deserialize. Point an editor at [`request_schema`] to
//! get completion and checking while writing a request by hand.

use serde_json::{json, Map, Value};

use crate::components::catalog::SchemaContext;
use crate::components::ComponentCatalog;
use crate::render::RenderRequest;

/// The schema of `T`, as a `$ref` into the context's definitions.
pub fn schema_of<T: schemars::JsonSchema>(ctx: &mut SchemaContext) -> Value {
    ctx.generator.subschema_for::<T>().to_value()
}

/// Schema of the `data` of component `id`, or `None` for an unknown
/// component and for one without a Rust struct.
pub fn component_schema(id: &str) -> Option<Value> {
    let schema = ComponentCatalog::get(id)?.schema?;
    let mut ctx = SchemaContext::new();
    let data = schema(&mut ctx);
    Some(with_definitions(
        json!({
            "title": id,
            "description": ComponentCatalog::get(id)?.description,
            "allOf": [data],
        }),
        &mut ctx,
    ))
}

/// Schema of a whole render request. Every `components` entry is checked
/// against the schema of its `type`; types outside the catalog (pack
/// components) are accepted with any `data`. `for`/`if` entries of a
/// template (see [`crate::render::bind`]) are allowed next to them.
pub fn request_schema() -> Value {
    let mut ctx = SchemaContext::new();
    let request = ctx.generator.subschema_for::<RenderRequest>().to_value();

    let mut descriptors: Vec<_> = ComponentCatalog::all().collect();
    descriptors.sort_by_key(|d| d.id);

    let mut known = Vec::new();
    let mut variants = Vec::new();
    for d in &descriptors {
        known.push(Value::String(d.id.to_string()));
        let data = match d.schema {
            Some(schema) => schema(&mut ctx),
            None => json!({ "type": "object" }),
        };
        variants.push(json!({
            "type": "object",
            "title": d.id,
            "description": d.description,
            "properties": { "type": { "const": d.id }, "data": data },
            "required": ["type", "data"],
        }));
    }
    variants.push(json!({
        "type": "object",
        "title": "pack component",
        "description": "A component defined by the request's pack.",
        "properties": {
            "type": { "type": "string", "not": { "enum": known } },
            "data": {},
        },
        "required": ["type", "data"],
    }));

    // Template directives, see `render::bind`: they are resolved before the
    // request is read, so they stand next to the components.
    variants.push(json!({
        "type": "object",
        "title": "for",
        "description": "Repeats `component` for each element of a list in the data.",
        "properties": {
            "for": { "type": "string", "pattern": "^\\S+ in \\S+$" },
            "component": {},
        },
        "required": ["for", "component"],
        "additionalProperties": false,
    }));
    variants.push(json!({
        "type": "object",
        "title": "if",
        "description": "Keeps `component` only if the data path exists and is not empty.",
        "properties": {
            "if": { "type": "string" },
            "component": {},
        },
        "required": ["if", "component"],
        "additionalProperties": false,
    }));

    let mut root = with_definitions(json!({ "allOf": [request] }), &mut ctx);
    if let Some(properties) = root
        .pointer_mut("/$defs/RenderRequest/properties")
        .and_then(Value::as_object_mut)
    {
        properties.insert(
            "components".into(),
            json!({
                "description": "Report components, each `{type, data}`, or a `for`/`if` entry of a template.",
                "type": "array",
                "items": { "oneOf": variants },
            }),
        );
    }
    root["title"] = json!("renderreport request");
    root
}

/// Adds the definitions collected in `ctx` and the `$schema` keyword.
fn with_definitions(mut schema: Value, ctx: &mut SchemaContext) -> Value {
    let definitions: Map<String, Value> = ctx.generator.take_definitions(true);
    if let Some(object) = schema.as_object_mut() {
        object.insert(
            "$schema".into(),
            json!("https://json-schema.org/draft/2020-12/schema"),
        );
        object.insert("$defs".into(), Value::Object(definitions));
    }
    schema
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_schema_describes_the_struct() {
        let schema = component_schema("score-card").expect("score-card has a schema");
        let text = schema.to_string();
        assert!(text.contains("\"score\""), "{text}");
    }

    #[test]
    fn request_schema_lists_every_component_with_a_struct() {
        let schema = request_schema();
        let variants = schema
            .pointer("/$defs/RenderRequest/properties/components/items/oneOf")
            .and_then(Value::as_array)
            .expect("components are a oneOf");
        assert_eq!(variants.len(), ComponentCatalog::all().count() + 3);
    }

    #[test]
    fn request_schema_resolves_all_references() {
        let schema = request_schema();
        let defs = schema["$defs"].as_object().expect("$defs");
        let text = schema.to_string();
        for part in text.split("\"$ref\":\"#/$defs/").skip(1) {
            let name = part.split('"').next().unwrap();
            assert!(defs.contains_key(name), "dangling $ref to {name}");
        }
    }
}
