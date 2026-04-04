//! Executive pattern for C-level summaries and board reports
//!
//! Structure:
//! 1. Hero (HeroSummary with high-level metrics)
//! 2. Context (executive summary, key facts)
//! 3. Analysis (top findings, impact, risk assessment)
//! 4. Solution (strategic recommendation, next steps)
//! 5. Actions (ownership, timeline, budget)
//! 6. CTA (schedule meeting, approval)

use super::{BasePattern, component_json};
use serde_json::Value;

pub struct ExecutivePattern {
    base: BasePattern,
}

impl ExecutivePattern {
    /// Create new executive pattern
    pub fn new() -> Self {
        Self {
            base: BasePattern::new(),
        }
    }

    /// Set hero summary (key metrics, status, verdict)
    pub fn with_hero_summary(mut self, data: Value) -> Self {
        self.base.hero = Some(component_json("hero-summary", data));
        self
    }

    /// Add executive summary section
    pub fn add_summary(mut self, items: Vec<Value>) -> Self {
        self.base.context.extend(items);
        self
    }

    /// Add key metrics (big-number, fact-box)
    pub fn add_key_metrics(mut self, items: Vec<Value>) -> Self {
        self.base.analysis.extend(items);
        self
    }

    /// Add top findings (spotlight cards, priority list)
    pub fn add_top_findings(mut self, items: Vec<Value>) -> Self {
        self.base.analysis.extend(items);
        self
    }

    /// Add risk assessment
    pub fn add_risk_overview(mut self, data: Value) -> Self {
        self.base.analysis.push(component_json("impact-grid", data));
        self
    }

    /// Add strategic recommendation
    pub fn add_recommendation(mut self, items: Vec<Value>) -> Self {
        self.base.solution.extend(items);
        self
    }

    /// Add implementation plan
    pub fn add_implementation_plan(mut self, data: Value) -> Self {
        self.base.actions.push(component_json("phase-block", data));
        self
    }

    /// Add timeline
    pub fn add_timeline(mut self, data: Value) -> Self {
        self.base.actions.push(component_json("timeline", data));
        self
    }

    /// Set CTA (schedule meeting, approve, next steps)
    pub fn with_cta(mut self, data: Value) -> Self {
        self.base.cta = Some(component_json("cta-box", data));
        self
    }

    /// Convert to component list for rendering
    pub fn to_components(self) -> Vec<Value> {
        self.base.to_components()
    }
}

impl Default for ExecutivePattern {
    fn default() -> Self {
        Self::new()
    }
}
