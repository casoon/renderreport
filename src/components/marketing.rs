//! Marketing and sales components: heroes, pricing, testimonials and CTAs.

use super::catalog::{ComponentCategory, LayoutHint};
use super::Component;
use crate::component_catalog;
use serde::{Deserialize, Serialize};

component_catalog! {
    id: "hero-summary",
    data: HeroSummary,
    template: include_str!("../../templates/components/hero_summary.typ"),
    description: "Full-width hero summary with title, subtitle, and key metrics",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Hero summary for the first content page after TOC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroSummary {
    pub score: u32,
    pub grade: String,
    pub domain: String,
    pub date: String,
    pub verdict: String,
    pub metrics: Vec<HeroMetric>,
    #[serde(default)]
    pub top_actions: Vec<String>,
    #[serde(default)]
    pub positive_aspects: Vec<String>,
    #[serde(default = "default_hero_good")]
    pub good_threshold: u32,
    #[serde(default = "default_hero_warn")]
    pub warn_threshold: u32,
}

fn default_hero_good() -> u32 {
    70
}

fn default_hero_warn() -> u32 {
    50
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroMetric {
    pub title: String,
    pub value: String,
    #[serde(default)]
    pub accent_color: Option<String>,
}

impl HeroSummary {
    pub fn new(score: u32, grade: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            score,
            grade: grade.into(),
            domain: domain.into(),
            date: String::new(),
            verdict: String::new(),
            metrics: Vec::new(),
            top_actions: Vec::new(),
            positive_aspects: Vec::new(),
            good_threshold: 70,
            warn_threshold: 50,
        }
    }

    pub fn with_date(mut self, date: impl Into<String>) -> Self {
        self.date = date.into();
        self
    }

    pub fn with_verdict(mut self, verdict: impl Into<String>) -> Self {
        self.verdict = verdict.into();
        self
    }

    pub fn add_metric(mut self, metric: HeroMetric) -> Self {
        self.metrics.push(metric);
        self
    }

    pub fn with_top_actions(mut self, actions: Vec<String>) -> Self {
        self.top_actions = actions;
        self
    }

    pub fn with_positive_aspects(mut self, aspects: Vec<String>) -> Self {
        self.positive_aspects = aspects;
        self
    }

    pub fn with_thresholds(mut self, good: u32, warn: u32) -> Self {
        self.good_threshold = good;
        self.warn_threshold = warn;
        self
    }
}

impl Component for HeroSummary {
    fn component_id(&self) -> &'static str {
        "hero-summary"
    }

    fn to_data(&self) -> serde_json::Value {
        let mut data = serde_json::to_value(self).unwrap_or_default();
        if let serde_json::Value::Object(ref mut map) = data {
            let status = if self.score >= self.good_threshold {
                "good"
            } else if self.score >= self.warn_threshold {
                "warning"
            } else {
                "bad"
            };
            map.insert(
                "computed_status".into(),
                serde_json::Value::String(status.into()),
            );
        }
        data
    }
}

// ─── Phase 3: Marketing / Narrative Components ──────────────────────────────

/// Marketing feature/benefit grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureGridItem {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

component_catalog! {
    id: "feature-grid",
    data: FeatureGrid,
    template: include_str!("../../templates/components/feature_grid.typ"),
    description: "Grid of feature items with icon, title, and description",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::Breakable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureGrid {
    pub items: Vec<FeatureGridItem>,
    #[serde(default)]
    pub columns: Option<usize>,
    #[serde(default)]
    pub title: Option<String>,
}

impl FeatureGrid {
    pub fn new(items: Vec<FeatureGridItem>) -> Self {
        Self {
            items,
            columns: None,
            title: None,
        }
    }

    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = Some(columns);
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for FeatureGrid {
    fn component_id(&self) -> &'static str {
        "feature-grid"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "cta-box",
    data: CTABox,
    template: include_str!("../../templates/components/cta_box.typ"),
    description: "Call-to-action box with headline, body, and button label",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Call to action block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTABox {
    pub headline: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub action_label: Option<String>,
    #[serde(default)]
    pub action_url: Option<String>,
    #[serde(default)]
    pub tone: Option<String>,
}

impl CTABox {
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            headline: headline.into(),
            body: None,
            action_label: None,
            action_url: None,
            tone: None,
        }
    }

    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn with_action(mut self, label: impl Into<String>, url: impl Into<String>) -> Self {
        self.action_label = Some(label.into());
        self.action_url = Some(url.into());
        self
    }

    pub fn with_tone(mut self, tone: impl Into<String>) -> Self {
        self.tone = Some(tone.into());
        self
    }
}

impl Component for CTABox {
    fn component_id(&self) -> &'static str {
        "cta-box"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "testimonial",
    data: Testimonial,
    template: include_str!("../../templates/components/testimonial.typ"),
    description: "Customer testimonial quote with attribution",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Customer quote / testimonial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testimonial {
    pub quote: String,
    pub author: String,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub company: Option<String>,
}

impl Testimonial {
    pub fn new(quote: impl Into<String>, author: impl Into<String>) -> Self {
        Self {
            quote: quote.into(),
            author: author.into(),
            role: None,
            company: None,
        }
    }

    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn with_company(mut self, company: impl Into<String>) -> Self {
        self.company = Some(company.into());
        self
    }
}

impl Component for Testimonial {
    fn component_id(&self) -> &'static str {
        "testimonial"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "product-hero",
    data: ProductHero,
    template: include_str!("../../templates/components/product_hero.typ"),
    description: "Product hero block with image, headline, and description",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Product/Project hero page — for introducing a product or project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductHero {
    pub title: String,
    pub subtitle: String,
    #[serde(default)]
    pub tagline: Option<String>,
    #[serde(default)]
    pub highlights: Vec<String>,
    #[serde(default)]
    pub cta_label: Option<String>,
    #[serde(default)]
    pub cta_url: Option<String>,
}

impl ProductHero {
    pub fn new(title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: subtitle.into(),
            tagline: None,
            highlights: vec![],
            cta_label: None,
            cta_url: None,
        }
    }

    pub fn with_tagline(mut self, tagline: impl Into<String>) -> Self {
        self.tagline = Some(tagline.into());
        self
    }

    pub fn add_highlight(mut self, highlight: impl Into<String>) -> Self {
        self.highlights.push(highlight.into());
        self
    }

    pub fn with_highlights(mut self, highlights: Vec<String>) -> Self {
        self.highlights = highlights;
        self
    }

    pub fn with_cta(mut self, label: impl Into<String>, url: impl Into<String>) -> Self {
        self.cta_label = Some(label.into());
        self.cta_url = Some(url.into());
        self
    }
}

impl Component for ProductHero {
    fn component_id(&self) -> &'static str {
        "product-hero"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "benefit-strip",
    data: BenefitStrip,
    template: include_str!("../../templates/components/benefit_strip.typ"),
    description: "Horizontal strip of benefit items with icons",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Horizontal benefit strip (3–5 points)
/// For: Product features, value proposition, marketing intro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitStrip {
    /// Benefit titles
    pub titles: Vec<String>,
    /// Benefit descriptions
    pub descriptions: Vec<String>,
    /// Icons (optional, one per item)
    #[serde(default)]
    pub icons: Vec<String>,
    /// Number of columns (2-5, defaults to 3)
    #[serde(default = "default_benefit_columns")]
    pub columns: usize,
}

fn default_benefit_columns() -> usize {
    3
}

impl Default for BenefitStrip {
    fn default() -> Self {
        Self::new()
    }
}

impl BenefitStrip {
    pub fn new() -> Self {
        Self {
            titles: Vec::new(),
            descriptions: Vec::new(),
            icons: Vec::new(),
            columns: default_benefit_columns(),
        }
    }

    pub fn add_benefit(mut self, title: impl Into<String>, description: impl Into<String>) -> Self {
        self.titles.push(title.into());
        self.descriptions.push(description.into());
        self
    }

    pub fn add_benefit_with_icon(
        mut self,
        icon: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.titles.push(title.into());
        self.descriptions.push(description.into());
        self.icons.push(icon.into());
        self
    }

    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = columns.clamp(2, 5);
        self
    }
}

impl Component for BenefitStrip {
    fn component_id(&self) -> &'static str {
        "benefit-strip"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "pricing-card",
    data: PricingCard,
    template: include_str!("../../templates/components/pricing_card.typ"),
    description: "Pricing tier card with features and CTA",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Pricing / Plan card
/// For: Service packages, pricing tiers, audit levels, feature bundles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingCard {
    /// Plan name
    pub name: String,
    /// Price / cost (e.g. "$99" or "Free")
    pub price: String,
    /// Optional billing period (e.g. "/month")
    #[serde(default)]
    pub billing_period: Option<String>,
    /// Brief description
    #[serde(default)]
    pub description: Option<String>,
    /// Features included
    #[serde(default)]
    pub features: Vec<String>,
    /// CTA label (e.g. "Choose Plan")
    #[serde(default)]
    pub cta_label: Option<String>,
    /// Highlight this card as recommended
    #[serde(default)]
    pub highlighted: bool,
}

impl PricingCard {
    pub fn new(name: impl Into<String>, price: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            price: price.into(),
            billing_period: None,
            description: None,
            features: Vec::new(),
            cta_label: None,
            highlighted: false,
        }
    }

    pub fn with_billing_period(mut self, period: impl Into<String>) -> Self {
        self.billing_period = Some(period.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn add_feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }

    pub fn with_cta(mut self, label: impl Into<String>) -> Self {
        self.cta_label = Some(label.into());
        self
    }

    pub fn highlighted(mut self) -> Self {
        self.highlighted = true;
        self
    }
}

impl Component for PricingCard {
    fn component_id(&self) -> &'static str {
        "pricing-card"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "use-case-card",
    data: UseCaseCard,
    template: include_str!("../../templates/components/use_case_card.typ"),
    description: "Use case card with scenario title, user, and outcome",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Use-case card component
/// Single use case: context + problem + solution
/// For: Product documentation, case studies, application examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseCaseCard {
    /// Use case title
    pub title: String,
    /// Context / industry / audience
    pub context: String,
    /// The problem
    pub problem: String,
    /// The solution / how we help
    pub solution: String,
    /// Optional outcome / result
    #[serde(default)]
    pub outcome: Option<String>,
}

impl UseCaseCard {
    pub fn new(
        title: impl Into<String>,
        context: impl Into<String>,
        problem: impl Into<String>,
        solution: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            context: context.into(),
            problem: problem.into(),
            solution: solution.into(),
            outcome: None,
        }
    }

    pub fn with_outcome(mut self, outcome: impl Into<String>) -> Self {
        self.outcome = Some(outcome.into());
        self
    }
}

impl Component for UseCaseCard {
    fn component_id(&self) -> &'static str {
        "use-case-card"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
