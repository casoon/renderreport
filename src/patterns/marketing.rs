//! Marketing pattern for product/feature pages and brochures
//!
//! Structure:
//! 1. Hero (ProductHero, full-page intro)
//! 2. Context (problem statement, why it matters)
//! 3. Analysis (features, benefits, proof)
//! 4. Solution (how it works, differentiation)
//! 5. Actions (testimonials, case studies, use cases)
//! 6. CTA (pricing, demo request, signup)

use super::{BasePattern, component_json};
use serde_json::Value;

pub struct MarketingPattern {
    base: BasePattern,
}

impl MarketingPattern {
    /// Create new marketing pattern
    pub fn new() -> Self {
        Self {
            base: BasePattern::new(),
        }
    }

    /// Set hero (ProductHero, title + highlights + CTA)
    pub fn with_product_hero(mut self, data: Value) -> Self {
        self.base.hero = Some(component_json("product-hero", data));
        self
    }

    /// Add problem statement section
    pub fn add_problem_section(mut self, items: Vec<Value>) -> Self {
        self.base.context.extend(items);
        self
    }

    /// Add feature grid or benefits
    pub fn add_features(mut self, data: Value) -> Self {
        self.base.analysis.push(component_json("feature-grid", data));
        self
    }

    /// Add benefit strip
    pub fn add_benefits(mut self, data: Value) -> Self {
        self.base.analysis.push(component_json("benefit-strip", data));
        self
    }

    /// Add comparison (vs. competitors, old vs. new)
    pub fn add_comparison(mut self, data: Value) -> Self {
        self.base.analysis.push(component_json("comparison-block", data));
        self
    }

    /// Add process flow (how it works)
    pub fn add_process_flow(mut self, data: Value) -> Self {
        self.base.solution.push(component_json("process-flow", data));
        self
    }

    /// Add testimonial or case study
    pub fn add_testimonial(mut self, data: Value) -> Self {
        self.base.actions.push(component_json("testimonial", data));
        self
    }

    /// Add use case card
    pub fn add_use_case(mut self, data: Value) -> Self {
        self.base.actions.push(component_json("use-case-card", data));
        self
    }

    /// Add pull quote
    pub fn add_pull_quote(mut self, data: Value) -> Self {
        self.base.actions.push(component_json("pull-quote", data));
        self
    }

    /// Set CTA (pricing, demo, signup)
    pub fn with_cta(mut self, data: Value) -> Self {
        self.base.cta = Some(component_json("cta-box", data));
        self
    }

    /// Add pricing cards at the end
    pub fn add_pricing_cards(mut self, cards: Vec<Value>) -> Self {
        for card in cards {
            self.base.actions.push(component_json("pricing-card", card));
        }
        self
    }

    /// Convert to component list for rendering
    pub fn to_components(self) -> Vec<Value> {
        self.base.to_components()
    }
}

impl Default for MarketingPattern {
    fn default() -> Self {
        Self::new()
    }
}
