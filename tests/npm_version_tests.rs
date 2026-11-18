//! The npm package is published from the same tag as the crate, so its
//! version must be the crate's (0.8.1 reached crates.io but not npm because
//! `npm/renderreport/package.json` kept the previous version).

#[test]
fn npm_package_version_matches_the_crate() {
    let package: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/npm/renderreport/package.json"
        ))
        .expect("npm/renderreport/package.json is readable"),
    )
    .expect("package.json parses");
    assert_eq!(
        package["version"].as_str(),
        Some(env!("CARGO_PKG_VERSION")),
        "bump npm/renderreport/package.json together with Cargo.toml"
    );
}
