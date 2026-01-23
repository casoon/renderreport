//! Pack system tests

use renderreport::pack::{Pack, PackManifest};
use std::path::PathBuf;

#[test]
fn test_pack_manifest_creation() {
    let manifest = PackManifest::default();

    assert_eq!(manifest.pack.name, "unnamed");
    assert_eq!(manifest.pack.version, "0.1.0");
}

#[test]
fn test_pack_manifest_from_toml() {
    let toml = r#"
        [pack]
        name = "test-pack"
        version = "1.0.0"
        description = "Test pack"

        [compatibility]
        min_engine_version = "0.1.0"
    "#;

    let manifest = PackManifest::from_toml(toml);
    assert!(manifest.is_ok());

    let manifest = manifest.unwrap();
    assert_eq!(manifest.pack.name, "test-pack");
    assert_eq!(manifest.pack.version, "1.0.0");
    assert_eq!(
        manifest.compatibility.min_engine_version,
        Some("0.1.0".into())
    );
}

#[test]
fn test_pack_manifest_to_toml() {
    let manifest = PackManifest::default();
    let toml = manifest.to_toml();

    assert!(toml.is_ok());
    let toml_str = toml.unwrap();
    assert!(toml_str.contains("name = \"unnamed\""));
    assert!(toml_str.contains("version = \"0.1.0\""));
}

#[test]
fn test_pack_creation() {
    let manifest = PackManifest::default();
    let pack = Pack::new("test-pack", manifest);

    assert_eq!(pack.id.0, "test-pack");
    assert!(pack.templates.is_empty());
    assert!(pack.themes.is_empty());
}

#[test]
fn test_pack_with_templates() {
    let manifest = PackManifest::default();
    let mut pack = Pack::new("test-pack", manifest);

    pack.templates
        .insert("default".into(), "template content".into());

    assert_eq!(
        pack.get_template("default"),
        Some(&"template content".into())
    );
    assert_eq!(pack.list_templates().len(), 1);
}

#[test]
fn test_pack_with_themes() {
    use renderreport::theme::Theme;

    let manifest = PackManifest::default();
    let mut pack = Pack::new("test-pack", manifest);

    let theme = Theme::new("dark", "Dark Theme");
    pack.themes.insert("dark".into(), theme.clone());

    assert_eq!(pack.get_theme("dark").unwrap().id, "dark");
    assert_eq!(pack.list_themes().len(), 1);
}

#[test]
fn test_pack_manifest_with_templates() {
    let toml = r#"
        [pack]
        name = "test"
        version = "1.0.0"

        [templates.default]
        file = "default.typ"
        description = "Default template"

        [templates.minimal]
        file = "minimal.typ"
        description = "Minimal template"
    "#;

    let manifest = PackManifest::from_toml(toml).unwrap();

    assert_eq!(manifest.templates.len(), 2);
    assert!(manifest.templates.contains_key("default"));
    assert!(manifest.templates.contains_key("minimal"));
}

#[test]
fn test_pack_manifest_with_themes() {
    let toml = r#"
        [pack]
        name = "test"
        version = "1.0.0"

        [themes.light]
        file = "light.toml"
        default = true

        [themes.dark]
        file = "dark.toml"
        default = false
    "#;

    let manifest = PackManifest::from_toml(toml).unwrap();

    assert_eq!(manifest.themes.len(), 2);
    assert!(manifest.themes.get("light").unwrap().default);
    assert!(!manifest.themes.get("dark").unwrap().default);
}

#[test]
fn test_pack_manifest_default_theme() {
    let toml = r#"
        [pack]
        name = "test"
        version = "1.0.0"

        [themes.light]
        file = "light.toml"
        default = true

        [themes.dark]
        file = "dark.toml"
    "#;

    let manifest = PackManifest::from_toml(toml).unwrap();

    assert_eq!(manifest.default_theme_name(), Some("light"));
}

#[test]
fn test_pack_manifest_capabilities() {
    let toml = r#"
[pack]
name = "test"
version = "1.0.0"

capabilities = ["charts", "tables", "images"]
    "#;

    let manifest = PackManifest::from_toml(toml).unwrap();

    assert_eq!(manifest.pack.capabilities.len(), 3);
    assert!(manifest.pack.capabilities.contains(&"charts".into()));
}

#[test]
fn test_pack_loader_creation() {
    use renderreport::pack::PackLoader;

    let _loader = PackLoader::new(&[PathBuf::from("./packs")]);

    // Loader should be created successfully
    // Can't test loading without actual pack files
    assert!(true);
}

#[test]
fn test_pack_id_creation() {
    use renderreport::pack::PackId;

    let id = PackId::new("test-pack");
    assert_eq!(id.0, "test-pack");

    let id_from_str: PackId = "another-pack".into();
    assert_eq!(id_from_str.0, "another-pack");
}

#[test]
fn test_pack_id_display() {
    use renderreport::pack::PackId;

    let id = PackId::new("my-pack");
    assert_eq!(format!("{}", id), "my-pack");
}
