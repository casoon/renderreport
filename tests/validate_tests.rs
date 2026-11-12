//! `validate` accepts the shipped example requests and the schema covers
//! every component that has a struct.

use renderreport::validate::validate_request;

#[test]
fn example_requests_are_valid() {
    for entry in std::fs::read_dir("examples/requests").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "json") {
            let json = std::fs::read_to_string(&path).unwrap();
            let issues = validate_request(&json, &[]);
            assert!(issues.is_empty(), "{}: {issues:?}", path.display());
        }
    }
}

#[cfg(feature = "schema")]
#[test]
fn every_component_with_a_validator_has_a_schema() {
    for d in renderreport::ComponentCatalog::all() {
        assert_eq!(
            d.validate.is_some(),
            d.schema.is_some(),
            "component `{}`",
            d.id
        );
    }
}

#[test]
fn an_alias_is_a_known_component_type() {
    let request = r#"{ "template_id": "default", "title": "R", "components": [
        { "type": "table", "data": { "columns": [{ "header": "A" }], "rows": [["x"]] } },
        { "type": "table", "data": { "columns": "wrong", "rows": [] } },
        { "type": "cover-page", "data": { "title": "Report" } }
    ] }"#;
    let issues = validate_request(request, &[]);
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert_eq!(issues[0].path, "components[1].data");
    assert!(issues[0].message.starts_with("table:"), "{issues:?}");

    // A pack that defines the name itself keeps it.
    let issues = validate_request(request, &["table"]);
    assert_eq!(issues.len(), 0, "{issues:?}");
}
