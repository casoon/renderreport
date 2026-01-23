//! Report builder for fluent API

use crate::components::Component;
use crate::pack::PackId;
use crate::render::RenderRequest;
use crate::theme::Theme;

use std::collections::HashMap;
use std::path::PathBuf;

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

    /// Add a component to the report
    pub fn add_component(mut self, component: impl Component) -> Self {
        self.components.push(serde_json::json!({
            "type": component.component_id(),
            "data": component.to_data()
        }));
        self
    }

    /// Add a raw component (JSON)
    pub fn add_raw_component(mut self, component: serde_json::Value) -> Self {
        self.components.push(component);
        self
    }

    /// Add multiple components
    pub fn add_components(mut self, components: impl IntoIterator<Item = impl Component>) -> Self {
        for component in components {
            self.components.push(serde_json::json!({
                "type": component.component_id(),
                "data": component.to_data()
            }));
        }
        self
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
        }
    }
}
