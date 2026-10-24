//! Rendering with packs: templates, components and the strict loader.

use renderreport::engine::EngineConfig;
use renderreport::pack::{Pack, PackLoader, PackManifest};
use renderreport::{Engine, Error, RenderRequest};
use serde_json::json;
use std::path::{Path, PathBuf};

fn packs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("packs")
}

fn engine_with(packs: &[&str]) -> Engine {
    let config = EngineConfig::builder().pack_path(packs_dir()).build();
    let mut engine = Engine::with_config(config).expect("engine");
    for pack in packs {
        engine.load_pack(pack).expect("bundled pack loads");
    }
    engine
}

fn request(value: serde_json::Value) -> RenderRequest {
    serde_json::from_value(value).expect("valid request")
}

fn seo_request(template_id: &str) -> RenderRequest {
    request(json!({
        "template_id": template_id,
        "pack_id": "seo-audit",
        "title": "SEO Audit",
        "metadata": { "url": "https://example.com", "date": "2026-10-23", "auditor": "Casoon" },
        "components": [
            { "type": "section", "data": { "title": "Scores", "level": 1 } },
            { "type": "lighthouse-scores", "data": {
                "performance": 91, "accessibility": 88, "best_practices": 100, "seo": 45 } },
            { "type": "grid-component", "data": { "columns": 1, "items": [
                { "content": { "type": "core-web-vitals",
                               "data": { "lcp": 2.1, "fid": 80, "cls": 0.3 } } }
            ] } }
        ]
    }))
}

#[test]
fn bundled_packs_load() {
    let loader = PackLoader::new(&[packs_dir()]);
    for id in ["standard", "seo-audit"] {
        loader.load(id).unwrap_or_else(|e| panic!("pack {id}: {e}"));
    }
}

#[test]
fn seo_pack_renders_template_and_components() {
    let engine = engine_with(&["seo-audit"]);
    let request = seo_request("seo-report");

    let typ = engine.render_typ(&request).unwrap();
    assert!(typ.contains("#let lighthouse-scores(data)"));
    assert!(typ.contains("comp-type == \"core-web-vitals\" { core-web-vitals(comp-data) }"));
    assert!(typ.contains("#show: body => report("));
    assert!(typ.contains("url: \"https://example.com\""));
    // The template renders the title page, the built-in one is skipped.
    assert!(!typ.contains("#block(width: 100%, height: 100%, breakable: false)"));

    let pdf = engine.render_pdf(&request).unwrap();
    assert!(pdf.starts_with(b"%PDF"));
}

#[test]
fn standard_pack_templates_render() {
    let engine = engine_with(&["standard"]);
    for template in ["default", "minimal"] {
        let request = request(json!({
            "template_id": template,
            "pack_id": "standard",
            "title": "Report",
            "metadata": { "author": "Casoon", "date": "2026-10-23" },
            "components": [{ "type": "callout", "data": { "content": "Hello" } }]
        }));
        let pdf = engine
            .render_pdf(&request)
            .unwrap_or_else(|e| panic!("template {template}: {e}"));
        assert!(pdf.starts_with(b"%PDF"));
    }
}

#[test]
fn default_template_falls_back_to_built_in_layout() {
    let engine = engine_with(&["seo-audit"]);
    let typ = engine.render_typ(&seo_request("default")).unwrap();
    assert!(!typ.contains("#show: body => report("));
    assert!(typ.contains("#block(width: 100%, height: 100%, breakable: false)"));
}

#[test]
fn unknown_template_is_an_error() {
    let engine = engine_with(&["seo-audit"]);
    let err = engine.render_typ(&seo_request("nope")).unwrap_err();
    assert!(err.to_string().contains("no template `nope`"), "{err}");
    assert!(err.to_string().contains("seo-report"), "{err}");
}

#[test]
fn pack_that_is_not_loaded_is_an_error() {
    let engine = engine_with(&[]);
    let err = engine.render_typ(&seo_request("seo-report")).unwrap_err();
    assert!(matches!(err, Error::Pack(_)), "{err}");
}

#[test]
fn pack_component_overrides_built_in() {
    let mut engine = engine_with(&[]);
    let mut pack = Pack::new("override", PackManifest::default());
    pack.components.insert(
        "callout".into(),
        "#let callout(data) = text(fill: red)[OVERRIDE #data.content]".into(),
    );
    engine.register_pack(pack);

    let request = request(json!({
        "template_id": "default",
        "pack_id": "override",
        "components": [{ "type": "callout", "data": { "content": "x" } }]
    }));
    let typ = engine.render_typ(&request).unwrap();
    assert!(typ.contains("OVERRIDE"));
    assert_eq!(typ.matches("#let callout(").count(), 1);
}

/// A pack directory under the target tmp dir, removed again on drop.
struct TempPack(PathBuf);

impl TempPack {
    fn new(name: &str, manifest: &str, files: &[(&str, &str)]) -> Self {
        // One search root per test: tests run in parallel and each removes
        // its root on drop.
        let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("packs-{name}"));
        let dir = root.join(name);
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("pack.toml"), manifest).unwrap();
        for (path, content) in files {
            let path = dir.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, content).unwrap();
        }
        Self(root)
    }
}

impl Drop for TempPack {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

const MANIFEST_HEAD: &str = "[pack]\nname = \"t\"\nversion = \"0.1.0\"\n\n";

#[test]
fn loader_rejects_missing_listed_file() {
    let manifest = format!("{MANIFEST_HEAD}[components.widget]\nfile = \"widget.typ\"\n");
    let tmp = TempPack::new("missing-file", &manifest, &[]);
    let err = PackLoader::new(std::slice::from_ref(&tmp.0))
        .load("missing-file")
        .unwrap_err();
    assert!(err.to_string().contains("components/widget.typ"), "{err}");
}

#[test]
fn loader_rejects_component_without_its_function() {
    let manifest = format!("{MANIFEST_HEAD}[components.widget]\nfile = \"widget.typ\"\n");
    let tmp = TempPack::new(
        "wrong-fn",
        &manifest,
        &[("components/widget.typ", "#let other(data) = []")],
    );
    let err = PackLoader::new(std::slice::from_ref(&tmp.0))
        .load("wrong-fn")
        .unwrap_err();
    assert!(err.to_string().contains("#let widget(data)"), "{err}");
}

#[test]
fn pack_assets_are_reachable_from_templates() {
    let manifest = format!("{MANIFEST_HEAD}[components.logo]\nfile = \"logo.typ\"\n");
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect width="10" height="10"/></svg>"#;
    let tmp = TempPack::new(
        "with-asset",
        &manifest,
        &[
            (
                "components/logo.typ",
                "#let logo(data) = image(\"/logo.svg\", width: 20pt, alt: \"Logo\")",
            ),
            ("assets/logo.svg", svg),
        ],
    );
    let config = EngineConfig::builder().pack_path(tmp.0.clone()).build();
    let mut engine = Engine::with_config(config).unwrap();
    engine.load_pack("with-asset").unwrap();
    let request = request(json!({
        "template_id": "default",
        "pack_id": "with-asset",
        "title": "Assets",
        "components": [{ "type": "logo", "data": {} }]
    }));
    let pdf = engine.render_pdf(&request).unwrap();
    assert!(pdf.starts_with(b"%PDF"));
}
