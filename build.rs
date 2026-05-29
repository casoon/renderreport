//! build.rs — generates Typst dispatcher templates from components.toml.
//!
//! Outputs to $OUT_DIR:
//!   flow_group.typ  — complete flow_group.typ with generated _flow-dispatch
//!   grid.typ        — complete grid.typ with generated _grid-dispatch

use std::{env, fs, path::PathBuf};

#[derive(Debug)]
struct ComponentEntry {
    id: String,
    /// Typst function name (usually == id, except "image" → "report-image")
    fn_name: String,
}

fn main() {
    println!("cargo:rerun-if-changed=components.toml");
    println!("cargo:rerun-if-changed=templates/components/flow_group_body.typ");
    println!("cargo:rerun-if-changed=templates/components/grid_body.typ");

    let manifest_src =
        fs::read_to_string("components.toml").expect("components.toml not found");
    let entries = parse_manifest(&manifest_src);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // Generate flow_group.typ
    let flow_body =
        fs::read_to_string("templates/components/flow_group_body.typ")
            .expect("flow_group_body.typ not found");
    let flow_group_typ = format!(
        "// Flow Group Component\n// AUTO-GENERATED dispatch + static body — do not edit the dispatch section.\n\n{}\n\n{}",
        generate_flow_dispatch(&entries),
        flow_body.trim_start()
    );
    fs::write(out_dir.join("flow_group.typ"), &flow_group_typ)
        .expect("failed to write flow_group.typ");

    // Generate grid.typ
    let grid_body =
        fs::read_to_string("templates/components/grid_body.typ")
            .expect("grid_body.typ not found");
    let grid_typ = format!(
        "// Grid Component\n// AUTO-GENERATED dispatch + static body — do not edit the dispatch section.\n\n{}\n\n{}",
        generate_grid_dispatch(&entries),
        grid_body.trim_start()
    );
    fs::write(out_dir.join("grid.typ"), &grid_typ).expect("failed to write grid.typ");
}

fn parse_manifest(src: &str) -> Vec<ComponentEntry> {
    let mut entries = Vec::new();
    let mut current_id: Option<String> = None;
    let mut current_fn: Option<String> = None;

    for line in src.lines() {
        let line = line.trim();
        if line == "[[component]]" {
            if let Some(id) = current_id.take() {
                let fn_name = current_fn.take().unwrap_or_else(|| id.clone());
                entries.push(ComponentEntry { id, fn_name });
            }
        } else if let Some(rest) = line.strip_prefix("id = ") {
            current_id = Some(rest.trim_matches('"').to_string());
        } else if let Some(rest) = line.strip_prefix("fn = ") {
            current_fn = Some(rest.trim_matches('"').to_string());
        }
    }
    // Last entry
    if let Some(id) = current_id {
        let fn_name = current_fn.unwrap_or_else(|| id.clone());
        entries.push(ComponentEntry { id, fn_name });
    }
    entries
}

fn generate_flow_dispatch(entries: &[ComponentEntry]) -> String {
    let mut s = String::from(
        "#let _flow-dispatch(c) = {\n  if type(c) == dictionary and \"type\" in c and \"data\" in c {\n    let comp-type = c.at(\"type\")\n    let comp-data = c.at(\"data\")\n",
    );
    for (i, e) in entries.iter().enumerate() {
        let kw = if i == 0 { "if" } else { "else if" };
        s.push_str(&format!(
            "    {} comp-type == \"{}\" {{ {}(comp-data) }}\n",
            kw, e.id, e.fn_name
        ));
    }
    s.push_str(
        "    else {\n      text(size: 9pt, fill: gray, \"[\" + comp-type + \"]\")\n    }\n  } else if type(c) == str {\n    text(size: 10pt, c)\n  } else {\n    [#c]\n  }\n}",
    );
    s
}

fn generate_grid_dispatch(entries: &[ComponentEntry]) -> String {
    let mut s = String::from(
        "// Component dispatch for nested rendering\n#let _grid-dispatch(c) = {\n  if type(c) == dictionary and \"type\" in c and \"data\" in c {\n    let comp-type = c.at(\"type\")\n    let comp-data = c.at(\"data\")\n    // Dispatch to known component functions\n",
    );
    for (i, e) in entries.iter().enumerate() {
        let kw = if i == 0 { "if" } else { "else if" };
        s.push_str(&format!(
            "    {} comp-type == \"{}\" {{ {}(comp-data) }}\n",
            kw, e.id, e.fn_name
        ));
    }
    s.push_str(
        "    else {\n      // Fallback for unknown types\n      text(size: 9pt, fill: gray, \"[\" + comp-type + \"]\")\n    }\n  } else if type(c) == dictionary {\n    // Raw data without type wrapper — generic display\n    let inner = c\n    if inner.at(\"title\", default: none) != none {\n      text(weight: \"bold\", size: 10pt, inner.title)\n      if inner.at(\"score\", default: none) != none {\n        v(4pt)\n        text(size: 20pt, weight: \"bold\", str(inner.score))\n        if inner.at(\"max_score\", default: none) != none {\n          text(size: 10pt, fill: gray, \" / \" + str(inner.max_score))\n        }\n      }\n      if inner.at(\"description\", default: none) != none {\n        v(4pt)\n        text(size: 9pt, fill: gray, inner.description)\n      }\n    } else {\n      for (key, val) in inner {\n        if type(val) == str or type(val) == int or type(val) == float {\n          text(size: 9pt)[#text(weight: \"bold\")[#key:] #str(val)]\n          linebreak()\n        }\n      }\n    }\n  } else if type(c) == str {\n    text(size: 10pt, c)\n  } else {\n    [#c]\n  }\n}",
    );
    s
}
