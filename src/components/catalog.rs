//! Static component catalog with inventory-based registration.
//!
//! Each component type self-registers a [`ComponentDescriptor`] via the
//! [`component_catalog!`] macro.  The [`ComponentCatalog`] struct provides
//! query methods over all registered descriptors at runtime.
//!
//! # Example
//!
//! ```rust,ignore
//! component_catalog! {
//!     id: "finding",
//!     template: include_str!("../../templates/components/finding.typ"),
//!     description: "Audit finding card with severity indicator",
//!     category: ComponentCategory::NarrativeStorytelling,
//!     layout_hint: LayoutHint::KeepTogether,
//! }
//! ```

/// Metadata and Typst template for a single component type.
///
/// Instances are registered globally at program startup via
/// `inventory::submit!`.  Use the [`component_catalog!`] macro to create
/// and submit a descriptor.
pub struct ComponentDescriptor {
    /// Stable component type ID (e.g. `"finding"`, `"score-card"`).
    pub id: &'static str,
    /// Raw Typst template source embedded at compile time.
    pub template: &'static str,
    /// Human-readable description for docs/tooling.
    pub description: &'static str,
    /// Broad component category used for grouping in explorers / docs.
    pub category: ComponentCategory,
    /// Layout hint used by the engine to inject keep-together / page-break logic.
    pub layout_hint: LayoutHint,
}

inventory::collect!(ComponentDescriptor);

/// Broad grouping for catalogue organisation and documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentCategory {
    /// Page structure components (sections, flow-group, grid, columns).
    Layout,
    /// Text and editorial components (headings, callouts, textblocks).
    TextEditorial,
    /// Metric and KPI components (score-card, gauge, stat, progress-bar).
    MetricsKpis,
    /// Data and comparison components (tables, charts, crosstab).
    DataComparison,
    /// Narrative and storytelling components (findings, timelines, process flows).
    NarrativeStorytelling,
    /// Infographic and visual components (funnel, impact-grid, device-preview).
    Infographics,
    /// Marketing and sales components (pricing, CTA, testimonial, hero).
    MarketingSales,
    /// Media and asset components (image, barcode, logo-strip, watermark).
    MediaAssets,
}

impl ComponentCategory {
    /// Short label for display in component explorers and docs.
    pub fn label(self) -> &'static str {
        match self {
            Self::Layout => "Layout",
            Self::TextEditorial => "Text & Editorial",
            Self::MetricsKpis => "Metrics & KPIs",
            Self::DataComparison => "Data & Comparison",
            Self::NarrativeStorytelling => "Narrative & Storytelling",
            Self::Infographics => "Infographics",
            Self::MarketingSales => "Marketing & Sales",
            Self::MediaAssets => "Media & Assets",
        }
    }
}

/// Hint to the engine about how a component should interact with pagination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutHint {
    /// Component can break across pages (e.g. long tables).
    Breakable,
    /// Component must not break across pages (default for cards, charts).
    KeepTogether,
    /// Engine emits a page break *before* this component.
    AlwaysNewPage,
    /// Engine keeps this component on the same page as the *following* sibling.
    KeepWithNext,
}

/// Query interface for all registered [`ComponentDescriptor`]s.
pub struct ComponentCatalog;

impl ComponentCatalog {
    /// Iterate all registered component descriptors.
    pub fn all() -> impl Iterator<Item = &'static ComponentDescriptor> {
        inventory::iter::<ComponentDescriptor>()
    }

    /// Look up a descriptor by component ID.
    pub fn get(id: &str) -> Option<&'static ComponentDescriptor> {
        Self::all().find(|d| d.id == id)
    }

    /// All registered component IDs.
    pub fn ids() -> Vec<&'static str> {
        Self::all().map(|d| d.id).collect()
    }
}

/// Register a component with the global [`ComponentCatalog`].
///
/// # Example
///
/// ```rust,ignore
/// component_catalog! {
///     id: "finding",
///     template: include_str!("../../templates/components/finding.typ"),
///     description: "Audit finding card with severity indicator",
///     category: ComponentCategory::NarrativeStorytelling,
///     layout_hint: LayoutHint::KeepTogether,
/// }
/// ```
#[macro_export]
macro_rules! component_catalog {
    (
        id: $id:literal,
        template: $template:expr,
        description: $desc:literal,
        category: $cat:expr,
        layout_hint: $hint:expr $(,)?
    ) => {
        ::inventory::submit! {
            $crate::components::catalog::ComponentDescriptor {
                id: $id,
                template: $template,
                description: $desc,
                category: $cat,
                layout_hint: $hint,
            }
        }
    };
}
