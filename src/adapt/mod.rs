//! Input adapters: build a render request from a file people already have.
//!
//! `renderreport from <format> <file>` reads Markdown, CSV, Lighthouse JSON,
//! SARIF or JUnit XML and produces an ordinary render request (a
//! `serde_json::Value`), which [`crate::validate::validate_value`] accepts and
//! the engine renders. Each adapter is a small, self-contained step from the
//! source to components; none of them touches the engine.
//!
//! Texts are English; [`Lang::De`] switches the adapter's own labels via
//! [`Lang::t`].

use std::path::Path;

use serde_json::{json, Map, Value};

mod csv;
mod junit;
mod lighthouse;
mod markdown;
mod sarif;

/// A supported input format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    Markdown,
    Csv,
    Lighthouse,
    Sarif,
    Junit,
}

/// Language of the labels the adapters add (`en` is the default).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
pub enum Lang {
    #[default]
    En,
    De,
}

impl Lang {
    /// The label table: every adapter label is written once per language.
    pub fn t(self, en: &'static str, de: &'static str) -> &'static str {
        match self {
            Lang::En => en,
            Lang::De => de,
        }
    }

    fn code(self) -> &'static str {
        self.t("en", "de")
    }
}

/// What the command line passes to an adapter.
#[derive(Debug, Clone, Default)]
pub struct Options {
    /// Overrides the title the adapter would choose.
    pub title: Option<String>,
    /// `None`: English labels, and Markdown front matter may set the document language.
    pub lang: Option<Lang>,
}

impl Options {
    fn lang(&self) -> Lang {
        self.lang.unwrap_or_default()
    }
}

/// Reads `path` and builds the render request for `format`.
pub fn from_file(format: Format, path: &Path, options: &Options) -> Result<Value, String> {
    let bytes =
        std::fs::read(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    let text =
        String::from_utf8(bytes).map_err(|_| format!("{} is not UTF-8 text", path.display()))?;
    let source = Source {
        text: &text,
        stem: path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("report"),
        dir: path.parent().unwrap_or(Path::new("")),
    };
    let report = match format {
        Format::Markdown => markdown::build(&source, options),
        Format::Csv => csv::build(&source, options),
        Format::Lighthouse => lighthouse::build(&source, options),
        Format::Sarif => sarif::build(&source, options),
        Format::Junit => junit::build(&source, options),
    }?;
    Ok(report.into_request(options))
}

/// The input file as the adapters see it.
struct Source<'a> {
    text: &'a str,
    /// File name without extension, the fallback title.
    stem: &'a str,
    /// Directory of the file, against which relative image paths resolve.
    dir: &'a Path,
}

/// What an adapter produces; [`Report::into_request`] adds the common parts.
#[derive(Default)]
struct Report {
    title: String,
    subtitle: Option<String>,
    metadata: Map<String, Value>,
    components: Vec<Value>,
    assets: Map<String, Value>,
}

impl Report {
    fn into_request(self, options: &Options) -> Value {
        let mut metadata = self.metadata;
        match options.lang {
            Some(lang) => {
                metadata.insert("lang".into(), lang.code().into());
            }
            None => {
                metadata.entry("lang").or_insert_with(|| "en".into());
            }
        }
        let mut request = json!({
            "template_id": "default",
            "title": options.title.clone().unwrap_or(self.title),
            "metadata": metadata,
            "components": self.components,
            "assets": self.assets,
        });
        if let Some(subtitle) = self.subtitle {
            request["subtitle"] = subtitle.into();
        }
        request
    }
}

fn component(kind: &str, data: Value) -> Value {
    json!({ "type": kind, "data": data })
}

fn section(title: &str, level: u32) -> Value {
    component("section", json!({ "title": title, "level": level }))
}

fn text_block(content: &str) -> Value {
    component("textblock", json!({ "content": content }))
}

fn callout(variant: &str, title: &str, content: &str) -> Value {
    component(
        "callout",
        json!({ "variant": variant, "title": title, "content": content }),
    )
}

/// A grid with the components as cells.
fn grid(columns: usize, cells: Vec<Value>) -> Value {
    let items: Vec<Value> = cells.into_iter().map(|c| json!({ "content": c })).collect();
    component(
        "grid-component",
        json!({ "columns": columns, "items": items }),
    )
}

/// Text without the inline Markup that the text components would print
/// literally: emphasis and code marks go, `[text](url)` becomes `text (url)`.
fn plain(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(i) = rest.find('[') {
        let link = rest[i + 1..].split_once("](").and_then(|(label, tail)| {
            let (url, after) = tail.split_once(')')?;
            (!label.contains('[') && !label.contains('\n')).then_some((label, url, after))
        });
        match link {
            Some((label, url, after)) => {
                out.push_str(&rest[..i]);
                out.push_str(label);
                out.push_str(&format!(" ({url})"));
                rest = after;
            }
            None => {
                out.push_str(&rest[..=i]);
                rest = &rest[i + 1..];
            }
        }
    }
    out.push_str(rest);
    out.replace("**", "").replace("__", "").replace('`', "")
}

/// `1234`, `3.5`, `0.33`: integers without decimals, else at most two.
fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        return format!("{}", n as i64);
    }
    let text = format!("{n:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// The first `max` characters, with an ellipsis when cut.
fn truncate(text: &str, max: usize) -> String {
    match text.char_indices().nth(max) {
        Some((i, _)) => format!("{}…", text[..i].trim_end()),
        None => text.to_string(),
    }
}

/// A required field of a JSON document, with the format named in the error.
fn require<'a>(value: &'a Value, key: &str, format: &str, hint: &str) -> Result<&'a Value, String> {
    value
        .get(key)
        .ok_or_else(|| format!("{format}: missing `{key}` {hint}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_strips_inline_marks_and_links() {
        assert_eq!(
            plain("**Bold** and `code`, see [docs](https://x.y/z)."),
            "Bold and code, see docs (https://x.y/z)."
        );
        assert_eq!(plain("a [b] c"), "a [b] c");
    }

    #[test]
    fn numbers_are_short() {
        assert_eq!(format_number(1234.0), "1234");
        assert_eq!(format_number(3.5), "3.5");
        assert_eq!(format_number(1.0 / 3.0), "0.33");
    }
}
