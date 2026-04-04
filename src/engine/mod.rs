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

use crate::components::ComponentRegistry;
use crate::pack::{Pack, PackId, PackLoader};
use crate::render::{RenderOutput, RenderRequest};
use crate::theme::Theme;
use crate::Result;

use std::collections::HashMap;
use std::sync::Arc;

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

        Ok(Self {
            config,
            packs: HashMap::new(),
            components,
            default_theme: Theme::default_theme(),
            pack_loader,
        })
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
        // Merge themes: default -> pack -> request
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

        // Generate Typst source
        let typst_source = self.generate_typst_source(request, &theme)?;

        // Compile with Typst
        let pdf_bytes = self.compile_typst(&typst_source, request)?;

        Ok(RenderOutput::Pdf(pdf_bytes))
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
        source.push_str(&format!(
            "#let report-title = \"{}\"\n#let report-date = \"{}\"\n#let report-author = \"{}\"\n#let report-footer-link-url = \"{}\"\n#let report-footer-prefix = \"{}\"\n\n",
            title_str.replace('"', "\\\""),
            date_str.replace('"', "\\\""),
            author_str.replace('"', "\\\""),
            footer_link_url.replace('"', "\\\""),
            footer_prefix.replace('"', "\\\""),
        ));

        // Add page setup
        source.push_str(&self.generate_page_setup(theme));
        source.push_str("\n\n");

        // Add component functions
        source.push_str("// Component Functions\n");
        for component_id in self.components.list_components() {
            if let Some(template) = self.components.get_template(component_id) {
                source.push_str(template);
                source.push_str("\n\n");
            }
        }

        // Add report content
        source.push_str("// Report Content\n");
        source.push_str(&self.generate_content(request)?);

        Ok(source)
    }

    /// Generate page setup Typst code
    fn generate_page_setup(&self, _theme: &Theme) -> String {
        r#"#set page(
  paper: "a4",
  fill: color-background,
  margin: (
    top: page-margin-top,
    bottom: page-margin-bottom,
    left: page-margin,
    right: page-margin,
  ),
  header: context {
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
  footer: context {
    if counter(page).get().first() > 1 [
      #v(1pt)
      #line(length: 100%, stroke: (paint: color-border, thickness: 0.5pt))
      #v(3pt)
      #grid(
        columns: (1fr, auto),
        gutter: spacing-3,
        [#text(size: font-size-xs, fill: color-text-muted)[
          #if report-footer-prefix != "" { report-footer-prefix + " " }
          #if report-footer-link-url != "" {
            link(report-footer-link-url)[#text(weight: "semibold", fill: color-text-muted)[#report-author]]
          } else {
            text(weight: "semibold")[#report-author]
          }
        ]],
        [#text(size: font-size-xs, fill: color-text-muted)[#counter(page).display("1 / 1", both: true)]]
      )
    ]
  },
)

#set text(
  font: (font-body, "Arial", "Liberation Sans", "Noto Sans"),
  size: font-size-base,
  fill: color-text,
)

#set text(hyphenate: true)
#set par(justify: false, leading: 0.75em)

#set heading(numbering: none)
#show heading: set par(justify: false)
#show heading.where(level: 1): set text(size: font-size-2xl, weight: "bold", fill: color-text)
#show heading.where(level: 2): set text(size: font-size-xl, weight: "bold", fill: color-text)
#show heading.where(level: 3): set text(size: font-size-lg, weight: "bold", fill: color-text)
"#
        .to_string()
    }

    /// Generate report content
    fn generate_content(&self, request: &RenderRequest) -> Result<String> {
        let mut content = String::new();

        // Title page
        let has_title = request.title.as_ref().is_some_and(|t| !t.is_empty());
        if has_title {
            let title = request.title.as_deref().unwrap_or("");
            let subtitle = request.subtitle.as_deref().unwrap_or("");
            let author = request.metadata.get("author").map(|s| s.as_str()).unwrap_or("");
            let date = request.metadata.get("date").map(|s| s.as_str()).unwrap_or("");

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
                title = title.replace('"', "\\\""),
                subtitle = subtitle.replace('"', "\\\""),
                author = author.replace('"', "\\\""),
                date = date.replace('"', "\\\""),
            ));
        }

        // Components
        for component in &request.components {
            let component_type = component
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            let data = component
                .get("data")
                .cloned()
                .unwrap_or_else(|| component.clone());

            // Generate component call
            let fn_name = component_function_name(component_type);
            let escaped_data = serde_json::to_string(&data)
                .unwrap_or_default()
                .replace('\\', "\\\\")
                .replace('"', "\\\"");

            // Section and page-break components manage their own spacing;
            // omitting the gap keeps headings bonded to the following content.
            if matches!(component_type, "section" | "page-break") {
                content.push_str(&format!(
                    "#{}(json.decode(\"{}\"))\n\n",
                    fn_name, escaped_data
                ));
            } else {
                content.push_str(&format!(
                    "#{}(json.decode(\"{}\"))\n\n#v(spacing-4)\n\n",
                    fn_name, escaped_data
                ));
            }
        }

        Ok(content)
    }

    /// Compile Typst source to PDF
    fn compile_typst(&self, source: &str, request: &RenderRequest) -> Result<Vec<u8>> {
        use crate::render::typst_compile;

        typst_compile::compile_to_pdf(source, &self.config, request)
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
