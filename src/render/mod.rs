//! Rendering functionality

pub mod bind;
mod request;
pub mod typst_compile;

#[cfg(any(feature = "png", feature = "svg"))]
pub use request::PageFormat;
pub use request::{PageSetup, RenderOutput, RenderRequest};
