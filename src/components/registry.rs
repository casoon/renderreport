//! Component registry for managing available components

use std::collections::HashMap;
use std::sync::Arc;

type ValidatorFn = Arc<dyn Fn(&serde_json::Value) -> bool + Send + Sync>;

/// Unique identifier for a component type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub String);

impl ComponentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ComponentId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Registry of available component types
#[derive(Default)]
pub struct ComponentRegistry {
    /// Component templates (Typst code)
    templates: HashMap<ComponentId, String>,
    /// Insertion order for deterministic output
    insertion_order: Vec<ComponentId>,
    /// Component factories for validation
    validators: HashMap<ComponentId, ValidatorFn>,
}

impl ComponentRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a registry with standard components
    pub fn with_standard_components() -> Self {
        let mut registry = Self::new();
        registry.register_standard_components();
        registry
    }

    /// Register standard built-in components
    pub fn register_standard_components(&mut self) {
        // Standard Components
        self.register(
            ComponentId::new("score-card"),
            include_str!("../../templates/components/score_card.typ").to_string(),
        );

        self.register(
            ComponentId::new("finding"),
            include_str!("../../templates/components/finding.typ").to_string(),
        );

        self.register(
            ComponentId::new("audit-table"),
            include_str!("../../templates/components/audit_table.typ").to_string(),
        );

        self.register(
            ComponentId::new("section"),
            include_str!("../../templates/components/section.typ").to_string(),
        );

        self.register(
            ComponentId::new("image"),
            include_str!("../../templates/components/image.typ").to_string(),
        );

        self.register(
            ComponentId::new("callout"),
            include_str!("../../templates/components/callout.typ").to_string(),
        );

        self.register(
            ComponentId::new("summary-box"),
            include_str!("../../templates/components/summary_box.typ").to_string(),
        );

        // Advanced Components (inspired by JasperReports, BIRT, Pentaho)
        self.register(
            ComponentId::new("list"),
            include_str!("../../templates/components/list.typ").to_string(),
        );

        self.register(
            ComponentId::new("divider"),
            include_str!("../../templates/components/divider.typ").to_string(),
        );

        self.register(
            ComponentId::new("progress-bar"),
            include_str!("../../templates/components/progress_bar.typ").to_string(),
        );

        self.register(
            ComponentId::new("key-value-list"),
            include_str!("../../templates/components/key_value_list.typ").to_string(),
        );

        self.register(
            ComponentId::new("watermark"),
            include_str!("../../templates/components/watermark.typ").to_string(),
        );

        self.register(
            ComponentId::new("page-break"),
            include_str!("../../templates/components/page_break.typ").to_string(),
        );

        // Chart Components (inspired by JasperReports Charts)
        self.register(
            ComponentId::new("chart"),
            include_str!("../../templates/components/chart.typ").to_string(),
        );

        self.register(
            ComponentId::new("sparkline"),
            include_str!("../../templates/components/sparkline.typ").to_string(),
        );

        self.register(
            ComponentId::new("gauge"),
            include_str!("../../templates/components/gauge.typ").to_string(),
        );

        // Crosstab Components (inspired by JasperReports Crosstab, BIRT Cross Tab)
        self.register(
            ComponentId::new("crosstab"),
            include_str!("../../templates/components/crosstab.typ").to_string(),
        );

        self.register(
            ComponentId::new("pivot-table"),
            include_str!("../../templates/components/pivot_table.typ").to_string(),
        );

        // Barcode Components (inspired by JasperReports Barcode, Pentaho Barcode)
        self.register(
            ComponentId::new("barcode"),
            include_str!("../../templates/components/barcode.typ").to_string(),
        );

        // Text Components (inspired by BIRT, Pentaho Text Fields)
        self.register(
            ComponentId::new("label"),
            include_str!("../../templates/components/label.typ").to_string(),
        );

        self.register(
            ComponentId::new("textblock"),
            include_str!("../../templates/components/text.typ").to_string(),
        );

        self.register(
            ComponentId::new("number-field"),
            include_str!("../../templates/components/number_field.typ").to_string(),
        );

        self.register(
            ComponentId::new("date-field"),
            include_str!("../../templates/components/date_field.typ").to_string(),
        );

        self.register(
            ComponentId::new("resource-field"),
            include_str!("../../templates/components/resource_field.typ").to_string(),
        );

        self.register(
            ComponentId::new("side-label"),
            include_str!("../../templates/components/side_label.typ").to_string(),
        );

        self.register(
            ComponentId::new("table-of-contents"),
            include_str!("../../templates/components/table_of_contents.typ").to_string(),
        );

        // Composite Components (self-contained layouts)
        self.register(
            ComponentId::new("metric-card"),
            include_str!("../../templates/components/metric_card.typ").to_string(),
        );

        self.register(
            ComponentId::new("hero-summary"),
            include_str!("../../templates/components/hero_summary.typ").to_string(),
        );

        self.register(
            ComponentId::new("module-dashboard"),
            include_str!("../../templates/components/module_dashboard.typ").to_string(),
        );

        self.register(
            ComponentId::new("action-roadmap"),
            include_str!("../../templates/components/action_roadmap.typ").to_string(),
        );

        self.register(
            ComponentId::new("severity-overview"),
            include_str!("../../templates/components/severity_overview.typ").to_string(),
        );

        self.register(
            ComponentId::new("cover-page"),
            include_str!("../../templates/components/cover_page.typ").to_string(),
        );

        self.register(
            ComponentId::new("module-comparison"),
            include_str!("../../templates/components/module_comparison.typ").to_string(),
        );

        self.register(
            ComponentId::new("benchmark-summary"),
            include_str!("../../templates/components/benchmark_summary.typ").to_string(),
        );

        self.register(
            ComponentId::new("benchmark-table"),
            include_str!("../../templates/components/benchmark_table.typ").to_string(),
        );

        // ─── Phase 1: Primitives ──────────────────────────────────────────
        self.register(
            ComponentId::new("eyebrow"),
            include_str!("../../templates/components/eyebrow.typ").to_string(),
        );

        self.register(
            ComponentId::new("status-pill"),
            include_str!("../../templates/components/status_pill.typ").to_string(),
        );

        self.register(
            ComponentId::new("stat"),
            include_str!("../../templates/components/stat.typ").to_string(),
        );

        self.register(
            ComponentId::new("stat-pair"),
            include_str!("../../templates/components/stat_pair.typ").to_string(),
        );

        self.register(
            ComponentId::new("score-band"),
            include_str!("../../templates/components/score_band.typ").to_string(),
        );

        self.register(
            ComponentId::new("trend-tile"),
            include_str!("../../templates/components/trend_tile.typ").to_string(),
        );

        // ─── Phase 2: Composite Blocks ────────────────────────────────────
        self.register(
            ComponentId::new("section-header-split"),
            include_str!("../../templates/components/section_header_split.typ").to_string(),
        );

        self.register(
            ComponentId::new("phase-block"),
            include_str!("../../templates/components/phase_block.typ").to_string(),
        );

        self.register(
            ComponentId::new("diagnosis-panel"),
            include_str!("../../templates/components/diagnosis_panel.typ").to_string(),
        );

        self.register(
            ComponentId::new("metric-strip"),
            include_str!("../../templates/components/metric_strip.typ").to_string(),
        );

        self.register(
            ComponentId::new("impact-triad"),
            include_str!("../../templates/components/impact_triad.typ").to_string(),
        );

        self.register(
            ComponentId::new("dominant-issue-spotlight"),
            include_str!("../../templates/components/dominant_issue_spotlight.typ").to_string(),
        );

        self.register(
            ComponentId::new("wrong-right-block"),
            include_str!("../../templates/components/wrong_right_block.typ").to_string(),
        );

        self.register(
            ComponentId::new("comparison-cluster"),
            include_str!("../../templates/components/comparison_cluster.typ").to_string(),
        );

        // Grid component MUST be registered last — its template dispatches
        // to other component functions, so they must already be defined.
        self.register(
            ComponentId::new("grid-component"),
            include_str!("../../templates/components/grid.typ").to_string(),
        );

        // Flow group depends on other component functions, including grid-component.
        self.register(
            ComponentId::new("flow-group"),
            include_str!("../../templates/components/flow_group.typ").to_string(),
        );
    }

    /// Register a component template
    pub fn register(&mut self, id: ComponentId, template: String) {
        if !self.templates.contains_key(&id) {
            self.insertion_order.push(id.clone());
        }
        self.templates.insert(id, template);
    }

    /// Register a component with validator
    pub fn register_with_validator(
        &mut self,
        id: ComponentId,
        template: String,
        validator: impl Fn(&serde_json::Value) -> bool + Send + Sync + 'static,
    ) {
        self.templates.insert(id.clone(), template);
        self.validators.insert(id, Arc::new(validator));
    }

    /// Get a component template
    pub fn get_template(&self, id: &ComponentId) -> Option<&String> {
        self.templates.get(id)
    }

    /// Check if a component is registered
    pub fn has_component(&self, id: &ComponentId) -> bool {
        self.templates.contains_key(id)
    }

    /// Validate component data
    pub fn validate(&self, id: &ComponentId, data: &serde_json::Value) -> bool {
        self.validators.get(id).map(|v| v(data)).unwrap_or(true)
    }

    /// List all registered component IDs in insertion order
    pub fn list_components(&self) -> Vec<&ComponentId> {
        self.insertion_order.iter().collect()
    }
}

impl std::fmt::Debug for ComponentRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentRegistry")
            .field("templates", &self.templates.keys().collect::<Vec<_>>())
            .field("validators_count", &self.validators.len())
            .finish()
    }
}
