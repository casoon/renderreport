//! Core rendering engine
//!
//! The Engine is the main entry point for rendering reports.
//! It manages template packs, themes, and the Typst rendering pipeline.

mod builder;
mod config;
pub mod world;

pub use builder::ReportBuilder;
pub use config::EngineConfig;
pub use world::ReportWorld;

use crate::components::catalog::LayoutHint;
use crate::components::ComponentRegistry;
use crate::pack::{Pack, PackId, PackLoader};
use crate::render::{RenderOutput, RenderRequest};
use crate::theme::Theme;
use crate::Result;

use std::collections::HashMap;
use std::sync::Arc;

use world::FontCache;

/// The main rendering engine
pub struct Engine {
    /// Engine configuration
    config: EngineConfig,
    /// Loaded template packs
    packs: HashMap<PackId, Arc<Pack>>,
    /// Component registry
    components: ComponentRegistry,
    /// Default theme
    default_theme: Theme,
    /// Pack loader
    pack_loader: PackLoader,
    /// Cached font data, built once at construction time
    font_cache: Arc<FontCache>,
}

impl Engine {
    /// Create a new engine with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(EngineConfig::default())
    }

    /// Create a new engine with custom configuration
    pub fn with_config(config: EngineConfig) -> Result<Self> {
        let components = ComponentRegistry::with_standard_components();
        let pack_loader = PackLoader::new(&config.pack_paths);
        let font_cache = Arc::new(FontCache::new(&config));

        Ok(Self {
            config,
            packs: HashMap::new(),
            components,
            default_theme: Theme::default_theme(),
            pack_loader,
            font_cache,
        })
    }

    /// Rebuild the font cache from the current configuration.
    ///
    /// Call this if font paths or system font availability change at runtime.
    pub fn reload_fonts(&mut self) {
        self.font_cache = Arc::new(FontCache::new(&self.config));
    }

    /// Set the default theme
    pub fn set_default_theme(&mut self, theme: Theme) {
        self.default_theme = theme;
    }

    /// Get the default theme
    pub fn default_theme(&self) -> &Theme {
        &self.default_theme
    }

    /// Load a template pack from the configured paths
    pub fn load_pack(&mut self, pack_id: &str) -> Result<()> {
        let pack = self.pack_loader.load(pack_id)?;
        self.packs.insert(PackId::new(pack_id), Arc::new(pack));
        Ok(())
    }

    /// Register an embedded pack
    pub fn register_pack(&mut self, pack: Pack) {
        self.packs.insert(pack.id.clone(), Arc::new(pack));
    }

    /// Get a loaded pack
    pub fn get_pack(&self, pack_id: &PackId) -> Option<&Arc<Pack>> {
        self.packs.get(pack_id)
    }

    /// List loaded packs
    pub fn loaded_packs(&self) -> Vec<&PackId> {
        self.packs.keys().collect()
    }

    /// Get the component registry
    pub fn components(&self) -> &ComponentRegistry {
        &self.components
    }

    /// Get mutable component registry
    pub fn components_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.components
    }

    /// Create a new report builder
    pub fn report(&self, template_id: impl Into<String>) -> ReportBuilder {
        ReportBuilder::new(template_id)
    }

    /// Render a report to PDF
    pub fn render_pdf(&self, request: &RenderRequest) -> Result<Vec<u8>> {
        self.render(request).map(|output| output.into_bytes())
    }

    /// Render a report
    pub fn render(&self, request: &RenderRequest) -> Result<RenderOutput> {
        let theme = self.resolve_theme(request);
        let typst_source = self.generate_typst_source(request, &theme)?;
        let pdf_bytes = self.compile_typst(&typst_source, request)?;
        Ok(RenderOutput::Pdf(pdf_bytes))
    }

    /// Render a report to its intermediate Typst source.
    ///
    /// Returns the full `.typ` source that would be compiled by [`Self::render`] /
    /// [`Self::render_pdf`]. Useful for snapshot testing, lint/format pipelines
    /// (e.g. `typstyle`), and downstream tooling that wants to inspect or
    /// post-process the source before compilation.
    pub fn render_typ(&self, request: &RenderRequest) -> Result<String> {
        let theme = self.resolve_theme(request);
        self.generate_typst_source(request, &theme)
    }

    /// Merge themes: default -> pack -> request
    fn resolve_theme(&self, request: &RenderRequest) -> Theme {
        let mut theme = self.default_theme.clone();

        if let Some(pack_id) = &request.pack_id {
            if let Some(pack) = self.packs.get(pack_id) {
                if let Some(pack_theme) = &pack.default_theme {
                    theme.merge(pack_theme);
                }
            }
        }

        if let Some(request_theme) = &request.theme {
            theme.merge(request_theme);
        }

        theme
    }

    /// Generate Typst source from request
    fn generate_typst_source(&self, request: &RenderRequest, theme: &Theme) -> Result<String> {
        let mut source = String::new();

        // Add theme token definitions
        source.push_str("// Theme Tokens\n");
        source.push_str(&theme.tokens.to_typst_definitions());
        source.push_str("\n\n");

        // Add theme helper functions
        source.push_str("// Theme Helpers\n");
        source.push_str(include_str!("../../templates/theme_helpers.typ"));
        source.push_str("\n\n");

        // Inject metadata as Typst variables (needed by header/footer)
        source.push_str("// Report Metadata\n");
        let title_str = request.title.as_deref().unwrap_or("");
        let date_str = request
            .metadata
            .get("date")
            .map(|s| s.as_str())
            .unwrap_or("");
        let author_str = request
            .metadata
            .get("author")
            .map(|s| s.as_str())
            .unwrap_or("");
        let footer_link_url = request
            .metadata
            .get("footer_link_url")
            .map(|s| s.as_str())
            .unwrap_or("");
        let footer_prefix = request
            .metadata
            .get("footer_prefix")
            .map(|s| s.as_str())
            .unwrap_or("");
        let footer_tagline = request
            .metadata
            .get("footer_tagline")
            .map(|s| s.as_str())
            .unwrap_or("");
        source.push_str(&format!(
            "#let report-title = \"{}\"\n#let report-date = \"{}\"\n#let report-author = \"{}\"\n#let report-footer-link-url = \"{}\"\n#let report-footer-prefix = \"{}\"\n#let report-footer-tagline = \"{}\"\n\n",
            escape_for_typst_string(title_str),
            escape_for_typst_string(date_str),
            escape_for_typst_string(author_str),
            escape_for_typst_string(footer_link_url),
            escape_for_typst_string(footer_prefix),
            escape_for_typst_string(footer_tagline),
        ));

        // Add page setup
        source.push_str(&self.generate_page_setup(theme, &request.page_setup));
        source.push_str("\n\n");

        // Add component functions — only those actually used in this request.
        // When flow-group or grid-component is present their dispatchers
        // reference every component function, so we fall back to including all.
        source.push_str("// Component Functions\n");
        let used_types = collect_used_component_types(&request.components);
        let needs_all = used_types.contains("flow-group") || used_types.contains("grid-component");
        for component_id in self.components.list_components() {
            if needs_all || used_types.contains(component_id.0.as_str()) {
                if let Some(template) = self.components.get_template(component_id) {
                    source.push_str(template);
                    source.push_str("\n\n");
                }
            }
        }

        // Add report content
        source.push_str("// Report Content\n");
        source.push_str(&self.generate_content(request)?);

        Ok(source)
    }

    /// Generate page setup Typst code
    fn generate_page_setup(&self, _theme: &Theme, page_setup: &crate::render::PageSetup) -> String {
        let paper = page_setup.paper.as_deref().unwrap_or("a4");
        let flipped = page_setup.orientation.as_deref() == Some("landscape");

        let margin_block = if let Some(m) = &page_setup.margin {
            format!("  margin: {},\n", m)
        } else {
            "  margin: (\n    top: page-margin-top,\n    bottom: page-margin-bottom,\n    left: page-margin,\n    right: page-margin,\n  ),\n".to_string()
        };

        let header_block = if page_setup.show_header {
            r#"  header: context {
    if counter(page).get().first() > 1 [
      #set text(size: font-size-xs, fill: color-text-muted)
      #grid(
        columns: (1fr, auto),
        gutter: spacing-3,
        [#report-title],
        [#report-date]
      )
      #v(4pt)
      #line(length: 100%, stroke: (paint: color-border, thickness: 0.7pt))
    ]
  },
"#
        } else {
            ""
        };

        let footer_block = if page_setup.show_footer {
            r#"  footer: context {
    if counter(page).get().first() > 1 [
      #v(1pt)
      #line(length: 100%, stroke: (paint: color-border, thickness: 0.5pt))
      #v(3pt)
      #grid(
        columns: (1fr, auto, 1fr),
        gutter: spacing-3,
        [#text(size: font-size-xs, fill: color-text-muted)[
          #if report-footer-prefix != "" { report-footer-prefix + " " }
          #if report-footer-link-url != "" {
            link(report-footer-link-url)[#text(weight: "semibold", fill: color-text-muted)[#report-author]]
          } else {
            text(weight: "semibold")[#report-author]
          }
        ]],
        [#text(size: font-size-xs, fill: color-text-muted)[#counter(page).display("1 / 1", both: true)]],
        align(right)[#text(size: font-size-xs, fill: color-text-muted)[#report-footer-tagline]]
      )
    ]
  },
"#
        } else {
            ""
        };

        format!(
            "#set page(\n  paper: \"{}\",{}\n  fill: color-background,\n{}{}{})\n\n#set text(\n  font: (font-body, \"Arial\", \"Liberation Sans\", \"Noto Sans\"),\n  size: font-size-base,\n  fill: color-text,\n)\n\n#set text(hyphenate: true)\n#set par(justify: false, leading: 0.75em)\n\n#set heading(numbering: none)\n#show heading: set par(justify: false)\n#show heading: set block(sticky: true)\n#show heading.where(level: 1): set text(size: font-size-2xl, weight: \"bold\", fill: color-text)\n#show heading.where(level: 2): set text(size: font-size-xl, weight: \"bold\", fill: color-text)\n#show heading.where(level: 3): set text(size: font-size-lg, weight: \"bold\", fill: color-text)\n#show heading.where(level: 4): set text(size: font-size-base, weight: \"bold\", fill: color-text-muted)\n",
            paper,
            if flipped { "\n  flipped: true," } else { "" },
            margin_block,
            header_block,
            footer_block,
        )
    }

    /// Generate report content
    fn generate_content(&self, request: &RenderRequest) -> Result<String> {
        let mut content = String::new();

        // Title page
        let has_title = request.title.as_ref().is_some_and(|t| !t.is_empty());
        if has_title {
            let title = request.title.as_deref().unwrap_or("");
            let subtitle = request.subtitle.as_deref().unwrap_or("");
            let author = request
                .metadata
                .get("author")
                .map(|s| s.as_str())
                .unwrap_or("");
            let date = request
                .metadata
                .get("date")
                .map(|s| s.as_str())
                .unwrap_or("");

            content.push_str(&format!(
                r#"#block(width: 100%, height: 100%, breakable: false)[
  #v(1fr)
  #block(width: 60pt, height: 4pt, fill: color-primary, radius: 2pt)
  #v(spacing-5)
  #block(width: 100%)[#set par(leading: 0.4em); #text(size: 36pt, weight: "bold", fill: color-text, tracking: -0.02em)[{title}]]
  #v(spacing-3)
  #text(size: 18pt, fill: color-text-muted)[{subtitle}]
  #v(1fr)
  #line(length: 100%, stroke: 0.5pt + color-border)
  #v(spacing-3)
  #text(size: font-size-xs, fill: color-text-muted)[{author}#h(1fr){date}]
]
#pagebreak()

"#,
                title = escape_for_typst_markup(title),
                subtitle = escape_for_typst_markup(subtitle),
                author = escape_for_typst_markup(author),
                date = escape_for_typst_markup(date),
            ));
        }

        // Components
        let components = &request.components;
        let mut i = 0;
        while i < components.len() {
            let component = &components[i];
            let component_type = component
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            use crate::components::catalog::ComponentCatalog;

            let layout_hint = ComponentCatalog::get(component_type)
                .map(|d| d.layout_hint)
                .unwrap_or(LayoutHint::KeepTogether);

            // AlwaysNewPage: emit a page break before the component.
            if layout_hint == LayoutHint::AlwaysNewPage {
                content.push_str("#pagebreak()\n\n");
            }

            // KeepWithNext: wrap this component and the immediately following
            // non-structural sibling in a keep-together block.
            if layout_hint == LayoutHint::KeepWithNext {
                let next = components.get(i + 1);
                let next_type = next
                    .and_then(|n| n.get("type"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let next_hint = ComponentCatalog::get(next_type)
                    .map(|d| d.layout_hint)
                    .unwrap_or(LayoutHint::KeepTogether);
                // A `Breakable` next component (e.g. a long audit table) must be
                // allowed to span pages. Pairing it into a `breakable: false`
                // keep-together block traps it and makes it overflow the page
                // instead of breaking, overwriting content below (#366). So a
                // heading is only kept together with a non-breakable sibling.
                let next_is_content = !next_type.is_empty()
                    && !matches!(
                        next_hint,
                        LayoutHint::AlwaysNewPage
                            | LayoutHint::KeepWithNext
                            | LayoutHint::Breakable
                    );

                if next_is_content {
                    let next = next.expect("next_type non-empty implies next is Some");
                    let cur_data = component
                        .get("data")
                        .cloned()
                        .unwrap_or_else(|| component.clone());
                    let next_data = next.get("data").cloned().unwrap_or_else(|| next.clone());
                    let cur_fn = component_function_name(component_type);
                    let next_fn = component_function_name(next_type);
                    let cur_lit = to_typst_dict(&cur_data);
                    let next_lit = to_typst_dict(&next_data);
                    content.push_str(&format!(
                        "#block(breakable: false)[\n  #{cur_fn}({cur_lit})\n  #v(spacing-3)\n  #{next_fn}({next_lit})\n]\n\n#v(spacing-4)\n\n"
                    ));
                    i += 2;
                    continue;
                }
            }

            let data = component
                .get("data")
                .cloned()
                .unwrap_or_else(|| component.clone());

            // Generate component call
            let fn_name = component_function_name(component_type);
            let data_lit = to_typst_dict(&data);

            // Layout / structural components manage their own spacing.
            let is_structural = matches!(
                layout_hint,
                LayoutHint::AlwaysNewPage | LayoutHint::KeepWithNext
            ) || component_type == "page-break";
            if is_structural {
                content.push_str(&format!("#{}({})\n\n", fn_name, data_lit));
            } else {
                content.push_str(&format!("#{}({})\n\n#v(spacing-4)\n\n", fn_name, data_lit));
            }
            i += 1;
        }

        Ok(content)
    }

    /// Compile Typst source to PDF
    fn compile_typst(&self, source: &str, request: &RenderRequest) -> Result<Vec<u8>> {
        use crate::render::typst_compile;

        typst_compile::compile_to_pdf(source, &self.font_cache, request)
    }
}

/// Convert a JSON value to a Typst literal expression.
///
/// - `null` → `none`
/// - booleans / numbers → their Typst equivalents
/// - strings → `"..."` with proper escape sequences
/// - arrays → `(item, ...)` with trailing comma for single-element arrays
/// - objects → `(key: val, ...)` with string-quoted keys for non-identifiers
pub fn to_typst_dict(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "none".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => {
            let mut out = String::with_capacity(s.len() + 2);
            out.push('"');
            for ch in s.chars() {
                match ch {
                    '\\' => out.push_str("\\\\"),
                    '"' => out.push_str("\\\""),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c => out.push(c),
                }
            }
            out.push('"');
            out
        }
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(to_typst_dict).collect();
            match items.len() {
                0 => "()".to_string(),
                // Single-element: needs trailing comma to be an array, not parenthesized value
                1 => format!("({},)", items[0]),
                _ => format!("({})", items.join(", ")),
            }
        }
        serde_json::Value::Object(obj) => {
            if obj.is_empty() {
                return "(:)".to_string();
            }
            let entries: Vec<String> = obj
                .iter()
                .map(|(k, v)| {
                    if is_typst_ident(k) {
                        format!("{}: {}", k, to_typst_dict(v))
                    } else {
                        let escaped_key = k.replace('\\', "\\\\").replace('"', "\\\"");
                        format!("\"{}\": {}", escaped_key, to_typst_dict(v))
                    }
                })
                .collect();
            format!("({})", entries.join(", "))
        }
    }
}

/// Returns true if `s` is a valid Typst identifier.
///
/// Typst keywords that cannot be used as bare dictionary keys.
const TYPST_KEYWORDS: &[&str] = &[
    "none", "auto", "true", "false", "not", "and", "or", "let", "set", "show", "context", "if",
    "else", "for", "in", "while", "break", "continue", "return", "import", "include", "as",
];

/// Returns true if `s` is a valid Typst identifier that can be used as a bare dict key.
///
/// Typst identifiers start with a letter or underscore and continue with
/// letters, digits, underscores, or hyphens — but keywords must be quoted.
fn is_typst_ident(s: &str) -> bool {
    if TYPST_KEYWORDS.contains(&s) {
        return false;
    }
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        }
        _ => false,
    }
}

/// Escape a string for embedding inside a Typst string literal `"..."`.
fn escape_for_typst_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

/// Escape a string for embedding inside a Typst content block `[...]`.
fn escape_for_typst_markup(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '#' | '[' | ']' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            c => out.push(c),
        }
    }
    out
}

/// Collect all component type IDs referenced anywhere in a component list,
/// including deeply nested items inside flow-group and grid-component.
fn collect_used_component_types(
    components: &[serde_json::Value],
) -> std::collections::HashSet<String> {
    let mut used = std::collections::HashSet::new();
    for c in components {
        collect_types_in_value(c, &mut used);
    }
    used
}

fn collect_types_in_value(v: &serde_json::Value, used: &mut std::collections::HashSet<String>) {
    match v {
        serde_json::Value::Object(obj) => {
            if let Some(t) = obj.get("type").and_then(|t| t.as_str()) {
                used.insert(t.to_string());
            }
            for val in obj.values() {
                collect_types_in_value(val, used);
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                collect_types_in_value(item, used);
            }
        }
        _ => {}
    }
}

fn component_function_name(component_type: &str) -> &str {
    match component_type {
        // Avoid collision with Typst's built-in `image(...)`.
        "image" => "report-image",
        _ => component_type,
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new().expect("Failed to create default engine")
    }
}

impl std::fmt::Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("config", &self.config)
            .field("packs", &self.packs.keys().collect::<Vec<_>>())
            .field("default_theme", &self.default_theme.id)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_to_typst_dict_primitives() {
        assert_eq!(to_typst_dict(&json!(null)), "none");
        assert_eq!(to_typst_dict(&json!(true)), "true");
        assert_eq!(to_typst_dict(&json!(false)), "false");
        assert_eq!(to_typst_dict(&json!(42)), "42");
        assert_eq!(to_typst_dict(&json!(1.5)), "1.5");
        assert_eq!(to_typst_dict(&json!("hello")), "\"hello\"");
    }

    #[test]
    fn test_to_typst_dict_string_escaping() {
        assert_eq!(to_typst_dict(&json!("a\"b")), r#""a\"b""#);
        assert_eq!(to_typst_dict(&json!("a\nb")), "\"a\\nb\"");
        assert_eq!(to_typst_dict(&json!("a\tb")), "\"a\\tb\"");
        assert_eq!(to_typst_dict(&json!("a\\b")), "\"a\\\\b\"");
    }

    #[test]
    fn test_to_typst_dict_arrays() {
        assert_eq!(to_typst_dict(&json!([])), "()");
        assert_eq!(to_typst_dict(&json!([1])), "(1,)");
        assert_eq!(to_typst_dict(&json!([1, 2])), "(1, 2)");
    }

    #[test]
    fn test_to_typst_dict_objects() {
        assert_eq!(to_typst_dict(&json!({})), "(:)");
        // Note: serde_json preserves insertion order for objects created via json!()
        assert_eq!(to_typst_dict(&json!({"score": 85})), "(score: 85)");
    }

    #[test]
    fn test_to_typst_dict_non_ident_key() {
        assert_eq!(to_typst_dict(&json!({"2xl": "val"})), "(\"2xl\": \"val\")");
    }

    #[test]
    fn test_is_typst_ident() {
        assert!(is_typst_ident("score"));
        assert!(is_typst_ident("max_score"));
        assert!(is_typst_ident("chart-type"));
        assert!(is_typst_ident("_private"));
        assert!(!is_typst_ident("2xl"));
        assert!(!is_typst_ident(""));
        assert!(!is_typst_ident("has space"));
        // Typst keywords must not be bare keys
        assert!(!is_typst_ident("context"));
        assert!(!is_typst_ident("none"));
        assert!(!is_typst_ident("true"));
        assert!(!is_typst_ident("false"));
        assert!(!is_typst_ident("let"));
        assert!(!is_typst_ident("set"));
        assert!(!is_typst_ident("in"));
    }
}
