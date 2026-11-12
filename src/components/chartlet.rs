//! The `chartlet` component: a chart drawn from a chartlet specification.

use super::catalog::{ComponentCategory, LayoutHint};
use super::Component;
use crate::component_catalog;
use ::chartlet::{ChartSpec, RenderFormat, RenderOptions, Variant};
use serde::{Deserialize, Deserializer, Serialize};

component_catalog! {
    id: "chartlet",
    data: Chartlet,
    template: include_str!("../../templates/components/chartlet.typ"),
    description: "Chart drawn from a chartlet specification (SVG with text alternative)",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::KeepTogether,
}

/// Chartlet component
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Chartlet {
    /// chartlet specification (JSON object, see the chartlet schema)
    #[serde(deserialize_with = "parse_spec")]
    #[cfg_attr(feature = "schema", schemars(with = "serde_json::Value"))]
    pub spec: ChartSpec,
    /// Alt text; without it the description chartlet generates from the data is used
    #[serde(default)]
    pub alt: Option<String>,
    /// Caption
    #[serde(default)]
    pub caption: Option<String>,
    /// Width (e.g., "100%", "50%", "200pt")
    #[serde(default)]
    pub width: Option<String>,
}

/// Parses and validates the specification, so `validate` reports an invalid
/// chart with chartlet's own message.
fn parse_spec<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ChartSpec, D::Error> {
    let value = serde_json::Value::deserialize(deserializer)?;
    ChartSpec::from_json(&value.to_string()).map_err(serde::de::Error::custom)
}

impl Component for Chartlet {
    fn component_id(&self) -> &'static str {
        "chartlet"
    }

    /// Draws the chart (print variant: literal colors, no browser-only rules)
    /// and hands the SVG to the template in `svg`.
    fn to_data(&self) -> serde_json::Value {
        let options = RenderOptions {
            variant: Variant::Print,
            ..Default::default()
        };
        let Ok(output) = ::chartlet::render(&self.spec, RenderFormat::Svg, &options) else {
            return serde_json::Value::Null;
        };
        let alt = self.alt.clone().or_else(|| {
            ::chartlet::text_alternative(&self.spec)
                .ok()
                .map(|text| text.description)
        });
        serde_json::json!({
            // The specification is consumed here; keep it out of the Typst source.
            "spec": null,
            "svg": output.content,
            "alt": alt,
            "caption": self.caption,
            "width": self.width,
        })
    }
}
