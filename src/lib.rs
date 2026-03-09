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

pub mod components;
pub mod engine;
pub mod pack;
pub mod render;
pub mod theme;
pub mod vfs;

mod error;

pub use components::{Component, ComponentId, ComponentRegistry};
pub use engine::Engine;
pub use error::{Error, Result};
pub use pack::{Pack, PackId, PackLoader, PackManifest};
pub use render::{RenderOutput, RenderRequest};
pub use theme::{Theme, ThemeTokens, TokenValue};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::components::*;
    pub use crate::engine::Engine;
    pub use crate::theme::{Theme, ThemeTokens};
    pub use crate::{Error, Result};
}
