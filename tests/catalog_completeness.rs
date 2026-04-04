//! Catalog completeness test
//!
//! Verifies that every component registered in the engine appears in the
//! component catalog source file. This ensures the catalog stays up to date
//! when new components are added.
//!
//! Components must be annotated in the catalog via one of:
//!   - `// @id: component-id` comment (for directly-used components)
//!   - `"type": "component-id"` string (for components embedded in Grid JSON)

use renderreport::ComponentRegistry;

#[test]
fn all_registered_components_appear_in_catalog() {
    let catalog_src = include_str!("../examples/component_catalog.rs");

    let registry = ComponentRegistry::with_standard_components();
    let registered = registry.list_components();

    let missing: Vec<String> = registered
        .iter()
        .filter(|id| {
            let comment_pattern = format!("@id: {id}");
            let json_pattern = format!("\"type\": \"{id}\"");
            !catalog_src.contains(&comment_pattern) && !catalog_src.contains(&json_pattern)
        })
        .map(|id| id.to_string())
        .collect();

    assert!(
        missing.is_empty(),
        "The following registered components are missing from examples/component_catalog.rs:\n  {}\n\nAdd them to the catalog with either:\n  // @id: <component-id>\n  or \"type\": \"<component-id>\" in a Grid JSON block.",
        missing.join("\n  ")
    );
}
