//! Infographic components: funnels and impact grids.

use super::catalog::{ComponentCategory, LayoutHint};
use super::Component;
use crate::component_catalog;
use serde::{Deserialize, Serialize};

/// A step in a conversion funnel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelStep {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}

component_catalog! {
    id: "funnel",
    data: Funnel,
    template: include_str!("../../templates/components/funnel.typ"),
    description: "Conversion funnel visualization with stage counts",
    category: ComponentCategory::Infographics,
    layout_hint: LayoutHint::KeepTogether,
}

/// Conversion funnel visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Funnel {
    pub steps: Vec<FunnelStep>,
    #[serde(default)]
    pub title: Option<String>,
}

impl Funnel {
    pub fn new(steps: Vec<FunnelStep>) -> Self {
        Self { steps, title: None }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for Funnel {
    fn component_id(&self) -> &'static str {
        "funnel"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "device-preview",
    data: DevicePreview,
    template: include_str!("../../templates/components/device_preview.typ"),
    description: "Device frame (desktop/mobile) with screenshot",
    category: ComponentCategory::Infographics,
    layout_hint: LayoutHint::KeepTogether,
}

/// Device preview component for PDF cover pages.
/// Shows a desktop screenshot (3/4 width) and a mobile screenshot (1/4 width)
/// side by side with Apple-style device frames.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePreview {
    /// Asset key for the desktop screenshot (registered via builder.asset())
    pub desktop_src: String,
    /// Asset key for the mobile screenshot (registered via builder.asset())
    pub mobile_src: String,
    /// Height of the mockup block in points
    #[serde(default = "default_device_preview_height")]
    pub height_pt: f64,
    /// Desktop column ratio 0.0–1.0 (defaults to 0.75 for 3/4 layout)
    #[serde(default = "default_desktop_ratio")]
    pub desktop_ratio: f64,
    /// Alternative description of the desktop screenshot for Assistive
    /// Technology (required for PDF/UA-1 — every image needs either a real
    /// description or an explicit decorative marker). Defaults to a generic
    /// placeholder; callers should supply a real description via
    /// [`Self::with_alt_text`] when one is available (e.g. the audited URL).
    #[serde(default = "default_desktop_alt")]
    pub desktop_alt: String,
    /// Alternative description of the mobile screenshot. See `desktop_alt`.
    #[serde(default = "default_mobile_alt")]
    pub mobile_alt: String,
}

fn default_device_preview_height() -> f64 {
    195.0
}

fn default_desktop_ratio() -> f64 {
    0.70
}

fn default_desktop_alt() -> String {
    "Desktop screenshot preview".to_string()
}

fn default_mobile_alt() -> String {
    "Mobile screenshot preview".to_string()
}

impl DevicePreview {
    pub fn new(desktop_src: impl Into<String>, mobile_src: impl Into<String>) -> Self {
        Self {
            desktop_src: desktop_src.into(),
            mobile_src: mobile_src.into(),
            height_pt: default_device_preview_height(),
            desktop_ratio: default_desktop_ratio(),
            desktop_alt: default_desktop_alt(),
            mobile_alt: default_mobile_alt(),
        }
    }

    pub fn with_height(mut self, height_pt: f64) -> Self {
        self.height_pt = height_pt;
        self
    }

    /// Set real alternative descriptions for the desktop/mobile screenshots
    /// (e.g. naming the audited URL), replacing the generic placeholder.
    pub fn with_alt_text(
        mut self,
        desktop_alt: impl Into<String>,
        mobile_alt: impl Into<String>,
    ) -> Self {
        self.desktop_alt = desktop_alt.into();
        self.mobile_alt = mobile_alt.into();
        self
    }
}

impl Component for DevicePreview {
    fn component_id(&self) -> &'static str {
        "device-preview"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
