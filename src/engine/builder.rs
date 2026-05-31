//! Report builder for fluent API

use crate::components::Component;
use crate::pack::PackId;
use crate::render::{PageSetup, RenderRequest};
use crate::theme::Theme;

use std::collections::HashMap;
use std::path::PathBuf;

/// Pending section opened via [`ReportBuilder::section`].
#[derive(Debug, Clone)]
struct SectionContext {
    heading: serde_json::Value,
    items: Vec<serde_json::Value>,
}

/// Fluent builder for creating reports
#[derive(Debug, Clone)]
pub struct ReportBuilder {
    template_id: String,
    pack_id: Option<PackId>,
    title: Option<String>,
    subtitle: Option<String>,
    theme: Option<Theme>,
    components: Vec<serde_json::Value>,
    assets: HashMap<String, PathBuf>,
    metadata: HashMap<String, String>,
    page_setup: PageSetup,
    /// Nested section stack; non-empty while inside `.section()`/`.end_section()` calls.
    section_stack: Vec<SectionContext>,
}

impl ReportBuilder {
    /// Create a new report builder
    pub fn new(template_id: impl Into<String>) -> Self {
        Self {
            template_id: template_id.into(),
            pack_id: None,
            title: None,
            subtitle: None,
            theme: None,
            components: Vec::new(),
            assets: HashMap::new(),
            metadata: HashMap::new(),
            page_setup: PageSetup::default(),
            section_stack: Vec::new(),
        }
    }

    /// Set the template pack to use
    pub fn pack(mut self, pack_id: impl Into<String>) -> Self {
        self.pack_id = Some(PackId::new(pack_id));
        self
    }

    /// Set the report title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the report subtitle
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set the theme
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Set the page layout configuration
    pub fn page_setup(mut self, page_setup: PageSetup) -> Self {
        self.page_setup = page_setup;
        self
    }

    /// Add a component to the current scope (root or active section).
    pub fn add_component(mut self, component: impl Component) -> Self {
        let value = serde_json::json!({
            "type": component.component_id(),
            "data": component.to_data()
        });
        self.push_to_current_scope(value);
        self
    }

    /// Add a raw component (JSON) to the current scope.
    pub fn add_raw_component(mut self, component: serde_json::Value) -> Self {
        self.push_to_current_scope(component);
        self
    }

    /// Add multiple components to the current scope.
    pub fn add_components(mut self, components: impl IntoIterator<Item = impl Component>) -> Self {
        for component in components {
            let value = serde_json::json!({
                "type": component.component_id(),
                "data": component.to_data()
            });
            self.push_to_current_scope(value);
        }
        self
    }

    /// Open a new section. All components added until [`Self::end_section`] are
    /// wrapped together with the section heading in a `flow-group` with
    /// soft keep-together behavior.
    ///
    /// Sections may be nested: calling `section()` again before `end_section()`
    /// creates a child section inside the current one.
    pub fn section(mut self, title: impl Into<String>, level: u8) -> Self {
        let heading = serde_json::json!({
            "type": "section",
            "data": {
                "title": title.into(),
                "level": level.clamp(1, 6),
                "content": []
            }
        });
        self.section_stack.push(SectionContext {
            heading,
            items: Vec::new(),
        });
        self
    }

    /// Close the current section and emit a `flow-group` containing the section
    /// heading followed by all components added since [`Self::section`] was called.
    ///
    /// Panics in debug mode if called without a matching `section()`.
    pub fn end_section(mut self) -> Self {
        let ctx = self
            .section_stack
            .pop()
            .expect("end_section called without a matching section()");

        let mut items = vec![ctx.heading];
        items.extend(ctx.items);

        let flow_group = serde_json::json!({
            "type": "flow-group",
            "data": {
                "items": items,
                "spacing": null,
                "keep_together_if_under": null
            }
        });

        self.push_to_current_scope(flow_group);
        self
    }

    fn push_to_current_scope(&mut self, value: serde_json::Value) {
        if let Some(ctx) = self.section_stack.last_mut() {
            ctx.items.push(value);
        } else {
            self.components.push(value);
        }
    }

    /// Register an asset (image, file, etc.)
    pub fn asset(mut self, name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        self.assets.insert(name.into(), path.into());
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build the render request
    pub fn build(self) -> RenderRequest {
        RenderRequest {
            template_id: self.template_id,
            pack_id: self.pack_id,
            title: self.title,
            subtitle: self.subtitle,
            theme: self.theme,
            components: self.components,
            assets: self.assets,
            metadata: self.metadata,
            page_setup: self.page_setup,
        }
    }
}
