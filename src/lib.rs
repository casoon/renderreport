//! # RenderReport
//!
//! Data-driven report generation with Typst as embedded render engine.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use renderreport::{Engine, ReportBuilder, components::ScoreCard};
//!
//! let engine = Engine::new()?;
//! let report = ReportBuilder::new("seo-audit")
//!     .title("SEO Audit Report")
//!     .add_component(ScoreCard::new("Overall Score", 85))
//!     .build();
//!
//! let pdf = engine.render_pdf(&report)?;
//! std::fs::write("report.pdf", pdf)?;
//! ```

#[cfg(feature = "cli")]
pub mod adapt;
pub mod components;
pub mod engine;
pub mod pack;
pub mod patterns;
pub mod render;
#[cfg(feature = "schema")]
pub mod schema;
pub mod theme;
pub mod validate;
pub mod vfs;

#[cfg(feature = "wasm")]
pub mod wasm;

mod error;

pub use components::{
    Component, ComponentCatalog, ComponentCategory, ComponentDescriptor, ComponentId,
    ComponentRegistry, LayoutHint,
};
pub use engine::{Engine, ReportBuilder};
pub use error::{Error, Result};
pub use pack::{Pack, PackId, PackLoader, PackManifest};
pub use patterns::{AuditPattern, BasePattern, ExecutivePattern, MarketingPattern};
#[cfg(any(feature = "png", feature = "svg"))]
pub use render::PageFormat;
pub use render::{PageSetup, RenderOutput, RenderRequest};
pub use theme::{Theme, ThemeTokens, TokenValue};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::components::*;
    pub use crate::engine::Engine;
    pub use crate::theme::{Theme, ThemeTokens};
    pub use crate::{Error, Result};
}
