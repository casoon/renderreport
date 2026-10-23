//! Static component catalog with inventory-based registration.
//!
//! Each component type self-registers a [`ComponentDescriptor`] via the
//! [`macro@crate::component_catalog`] macro.  The [`ComponentCatalog`] struct provides
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
/// `inventory::submit!`.  Use the [`macro@crate::component_catalog`] macro to create
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
    /// Typst function name the template defines (usually equal to `id`).
    pub fn_name: &'static str,
    /// Runs JSON data through the component's struct, filling in the fields
    /// its builder derives (see `normalize_components`). `None` for
    /// components without a Rust struct.
    pub normalize: Option<NormalizeFn>,
}

/// Deserializes component data into its struct and returns `to_data()`, or
/// `None` when the data does not fit the struct.
pub type NormalizeFn = fn(&serde_json::Value) -> Option<serde_json::Value>;

#[doc(hidden)]
pub use super::normalize::typed as __normalize;

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
    /// Engine starts this component on a new page (a weak break *before* it,
    /// so an already-empty page is reused instead of left blank).
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

/// Register a component with the global [`crate::ComponentCatalog`].
///
/// Place the call next to the component's struct. `fn_name` is only needed
/// when the Typst function name differs from `id`; `data` names the struct
/// that JSON data is normalized through.
///
/// # Example
///
/// ```rust,ignore
/// component_catalog! {
///     id: "finding",
///     data: Finding,
///     template: include_str!("../../templates/components/finding.typ"),
///     description: "Audit finding card with severity indicator",
///     category: ComponentCategory::NarrativeStorytelling,
///     layout_hint: LayoutHint::KeepTogether,
/// }
/// ```
#[macro_export]
macro_rules! component_catalog {
    (@fn_name $id:literal) => { $id };
    (@fn_name $id:literal, $fn_name:literal) => { $fn_name };
    (@normalize) => { None };
    (@normalize $data:ty) => {
        Some($crate::components::catalog::__normalize::<$data>
            as $crate::components::catalog::NormalizeFn)
    };
    (
        id: $id:literal,
        $(fn_name: $fn_name:literal,)?
        $(data: $data:ty,)?
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
                fn_name: $crate::component_catalog!(@fn_name $id $(, $fn_name)?),
                normalize: $crate::component_catalog!(@normalize $($data)?),
            }
        }
    };
}
