//! Report patterns — pre-configured component compositions
//!
//! Patterns define the logical structure and flow for common report types:
//! - AuditPattern: Security/compliance audit with findings, impact, roadmap
//! - MarketingPattern: Product/feature showcase with benefits, CTA
//! - ExecutivePattern: High-level summary with key metrics, top findings, recommendation

use serde_json::Value;

pub mod audit;
pub mod marketing;
pub mod executive;

pub use audit::AuditPattern;
pub use marketing::MarketingPattern;
pub use executive::ExecutivePattern;

/// Base pattern: logical report structure
///
/// Every report follows a narrative flow:
/// 1. Hero/intro (title, context)
/// 2. Context (why this matters)
/// 3. Analysis (what we found)
/// 4. Solution (what to do)
/// 5. Actions (next steps)
/// 6. CTA (call to action)
///
/// Patterns instantiate this with sensible defaults for specific use cases.
pub struct BasePattern {
    /// Hero component (HeroSummary, ProductHero, etc.)
    pub hero: Option<Value>,
    /// Context section — why this matters, background
    pub context: Vec<Value>,
    /// Analysis section — findings, metrics, assessment
    pub analysis: Vec<Value>,
    /// Solution section — recommendations, approach
    pub solution: Vec<Value>,
    /// Actions section — concrete next steps
    pub actions: Vec<Value>,
    /// Call-to-action (CTABox, etc.)
    pub cta: Option<Value>,
}

impl BasePattern {
    /// Create empty pattern
    pub fn new() -> Self {
        Self {
            hero: None,
            context: Vec::new(),
            analysis: Vec::new(),
            solution: Vec::new(),
            actions: Vec::new(),
            cta: None,
        }
    }

    /// Convert pattern to flat component list for rendering
    pub fn to_components(&self) -> Vec<Value> {
        let mut components = Vec::new();

        if let Some(hero) = &self.hero {
            components.push(hero.clone());
        }

        if !self.context.is_empty() {
            components.push(section("Context", 2));
            components.extend(self.context.iter().cloned());
        }

        if !self.analysis.is_empty() {
            components.push(section("Analysis", 2));
            components.extend(self.analysis.iter().cloned());
        }

        if !self.solution.is_empty() {
            components.push(section("Recommended Actions", 2));
            components.extend(self.solution.iter().cloned());
        }

        if !self.actions.is_empty() {
            components.push(section("Implementation", 2));
            components.extend(self.actions.iter().cloned());
        }

        if let Some(cta) = &self.cta {
            components.push(cta.clone());
        }

        components
    }
}

impl Default for BasePattern {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper: create Section component as JSON
fn section(title: &str, level: usize) -> Value {
    serde_json::json!({
        "type": "section",
        "data": {
            "title": title,
            "level": level,
            "content": []
        }
    })
}

/// Helper: wrap component in JSON envelope
pub fn component_json(component_id: &str, data: Value) -> Value {
    serde_json::json!({
        "type": component_id,
        "data": data
    })
}
