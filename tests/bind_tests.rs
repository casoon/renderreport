//! The example template renders with both of its data sets, and errors from
//! binding name the template location and the missing data path.

use renderreport::render::bind::bind_str;
use renderreport::validate::validate_value;
use renderreport::{Engine, RenderRequest};

const DIR: &str = "examples/requests/monthly-report";

fn read(name: &str) -> String {
    std::fs::read_to_string(format!("{DIR}/{name}")).unwrap()
}

#[test]
fn template_is_valid_and_renderable_with_each_data_set() {
    let engine = Engine::new().expect("Engine::new failed");
    let template = read("template.json");
    for name in ["march.json", "april.json"] {
        let bound = bind_str(&template, &read(name)).unwrap_or_else(|e| panic!("{name}: {e}"));
        let issues = validate_value(&bound, &[]);
        assert!(issues.is_empty(), "{name}: {issues:?}");

        let request: RenderRequest = serde_json::from_value(bound).unwrap();
        let pdf = engine
            .render_pdf(&request)
            .unwrap_or_else(|e| panic!("{name}: {e}"));
        assert!(pdf.starts_with(b"%PDF"), "{name}");
    }
}

#[test]
fn data_sets_change_the_components() {
    let template = read("template.json");
    let count = |name: &str| {
        let bound = bind_str(&template, &read(name)).unwrap();
        bound["components"].as_array().unwrap().len()
    };
    // March: section, grid, findings section, 2 findings, note.
    assert_eq!(count("march.json"), 6);
    // April: section, grid only.
    assert_eq!(count("april.json"), 2);
}

#[test]
fn missing_data_is_reported_with_both_paths() {
    let err = bind_str(&read("template.json"), r#"{"client": {"name": "x"}}"#).unwrap_err();
    assert!(err.path.starts_with("components["), "{err}");
    assert!(err.message.contains("`scores`"), "{err}");
}
