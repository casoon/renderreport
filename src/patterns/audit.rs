//! Audit pattern for security/compliance reports
//!
//! Structure:
//! 1. Hero (HeroSummary with score/grade)
//! 2. Context (why audit matters, scope)
//! 3. Analysis (findings, impact grid, checkpoints)
//! 4. Solution (recommendations, roadmap)
//! 5. Actions (implementation steps)
//! 6. CTA (contact/next steps)

use super::{BasePattern, component_json};
use serde_json::Value;

pub struct AuditPattern {
    base: BasePattern,
}

impl AuditPattern {
    /// Create new audit pattern
    pub fn new() -> Self {
        Self {
            base: BasePattern::new(),
        }
    }

    /// Set hero summary (score, grade, verdict)
    pub fn with_hero_summary(mut self, data: Value) -> Self {
        self.base.hero = Some(component_json("hero-summary", data));
        self
    }

    /// Add context section items (scope, methodology, affected items)
    pub fn add_context(mut self, items: Vec<Value>) -> Self {
        self.base.context.extend(items);
        self
    }

    /// Add findings
    pub fn add_findings(mut self, findings: Vec<Value>) -> Self {
        self.base.analysis.extend(findings);
        self
    }

    /// Add impact overview
    pub fn add_impact_grid(mut self, data: Value) -> Self {
        self.base.analysis.push(component_json("impact-grid", data));
        self
    }

    /// Add checkpoints/verification
    pub fn add_checklist(mut self, data: Value) -> Self {
        self.base.analysis.push(component_json("checklist-panel", data));
        self
    }

    /// Add roadmap (phased remediation)
    pub fn add_roadmap(mut self, data: Value) -> Self {
        self.base.solution.push(component_json("roadmap-block", data));
        self
    }

    /// Add implementation timeline
    pub fn add_timeline(mut self, data: Value) -> Self {
        self.base.actions.push(component_json("timeline", data));
        self
    }

    /// Set CTA (contact, schedule review, etc.)
    pub fn with_cta(mut self, data: Value) -> Self {
        self.base.cta = Some(component_json("cta-box", data));
        self
    }

    /// Convert to component list for rendering
    pub fn to_components(self) -> Vec<Value> {
        self.base.to_components()
    }
}

impl Default for AuditPattern {
    fn default() -> Self {
        Self::new()
    }
}
