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
