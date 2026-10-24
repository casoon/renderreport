//! Narrative and storytelling components: findings, timelines and recommendations.

use super::catalog::{ComponentCategory, LayoutHint};
use super::{Component, Severity};
use crate::component_catalog;
use serde::{Deserialize, Serialize};

component_catalog! {
    id: "finding",
    data: Finding,
    template: include_str!("../../templates/components/finding.typ"),
    description: "Audit finding with severity indicator and recommendation block",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Finding component for audit issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Finding title
    pub title: String,
    /// Severity level
    pub severity: Severity,
    /// Detailed description
    pub description: String,
    /// Recommendation for fixing
    #[serde(default)]
    pub recommendation: Option<String>,
    /// Affected URL or resource
    #[serde(default)]
    pub affected: Option<String>,
    /// Category/tag
    #[serde(default)]
    pub category: Option<String>,
    /// Label for the "Affected:" line (default: "Affected:")
    #[serde(default)]
    pub label_affected: Option<String>,
    /// Label for the "Recommendation" section header (default: "Recommendation")
    #[serde(default)]
    pub label_recommendation: Option<String>,
}

impl Finding {
    pub fn new(
        title: impl Into<String>,
        severity: Severity,
        description: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            severity,
            description: description.into(),
            recommendation: None,
            affected: None,
            category: None,
            label_affected: None,
            label_recommendation: None,
        }
    }

    pub fn with_recommendation(mut self, rec: impl Into<String>) -> Self {
        self.recommendation = Some(rec.into());
        self
    }

    pub fn with_affected(mut self, affected: impl Into<String>) -> Self {
        self.affected = Some(affected.into());
        self
    }

    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
}

impl Component for Finding {
    fn component_id(&self) -> &'static str {
        "finding"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "roadmap-block",
    data: RoadmapBlock,
    template: include_str!("../../templates/components/roadmap_block.typ"),
    description: "Project roadmap block with phases and milestones",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::Breakable,
}

/// Roadmap block with categorized columns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadmapBlock {
    pub columns: Vec<RoadmapColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadmapColumn {
    pub title: String,
    #[serde(default)]
    pub accent_color: Option<String>,
    pub items: Vec<ActionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub action: String,
    pub role: String,
    pub priority: String,
    #[serde(default)]
    pub effort: Option<String>,
    pub benefit: String,
}

impl RoadmapBlock {
    pub fn new(columns: Vec<RoadmapColumn>) -> Self {
        Self { columns }
    }
}

impl Component for RoadmapBlock {
    fn component_id(&self) -> &'static str {
        "roadmap-block"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// A step in a process flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStep {
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

component_catalog! {
    id: "process-flow",
    data: ProcessFlow,
    template: include_str!("../../templates/components/process_flow.typ"),
    description: "Horizontal or vertical process flow with numbered steps",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Linear process visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessFlow {
    pub steps: Vec<ProcessStep>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub direction: Option<String>,
}

impl ProcessFlow {
    pub fn new(steps: Vec<ProcessStep>) -> Self {
        Self {
            steps,
            title: None,
            direction: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn vertical(mut self) -> Self {
        self.direction = Some("vertical".into());
        self
    }
}

impl Component for ProcessFlow {
    fn component_id(&self) -> &'static str {
        "process-flow"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// A timeline milestone item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineItem {
    pub date: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

component_catalog! {
    id: "timeline",
    data: Timeline,
    template: include_str!("../../templates/components/timeline.typ"),
    description: "Project timeline with milestones and status indicators",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::Breakable,
}

/// Project phases / milestone timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub items: Vec<TimelineItem>,
    #[serde(default)]
    pub title: Option<String>,
}

impl Timeline {
    pub fn new(items: Vec<TimelineItem>) -> Self {
        Self { items, title: None }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for Timeline {
    fn component_id(&self) -> &'static str {
        "timeline"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "problem-solution",
    data: ProblemSolution,
    template: include_str!("../../templates/components/problem_solution.typ"),
    description: "Two-column problem/solution comparison block",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Two-part problem/solution block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSolution {
    pub problem: String,
    pub solution: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub problem_label: Option<String>,
    #[serde(default)]
    pub solution_label: Option<String>,
    #[serde(default)]
    pub impact: Option<String>,
    #[serde(default)]
    pub effort: Option<String>,
}

impl ProblemSolution {
    pub fn new(problem: impl Into<String>, solution: impl Into<String>) -> Self {
        Self {
            problem: problem.into(),
            solution: solution.into(),
            title: None,
            problem_label: None,
            solution_label: None,
            impact: None,
            effort: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_labels(mut self, problem: impl Into<String>, solution: impl Into<String>) -> Self {
        self.problem_label = Some(problem.into());
        self.solution_label = Some(solution.into());
        self
    }

    pub fn with_impact(mut self, impact: impl Into<String>) -> Self {
        self.impact = Some(impact.into());
        self
    }

    pub fn with_effort(mut self, effort: impl Into<String>) -> Self {
        self.effort = Some(effort.into());
        self
    }
}

impl Component for ProblemSolution {
    fn component_id(&self) -> &'static str {
        "problem-solution"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "before-after",
    data: BeforeAfter,
    template: include_str!("../../templates/components/before_after.typ"),
    description: "Before/after comparison with labeled panels",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Before/after comparison block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeforeAfter {
    pub before: String,
    pub after: String,
    #[serde(default)]
    pub label_before: Option<String>,
    #[serde(default)]
    pub label_after: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

impl BeforeAfter {
    pub fn new(before: impl Into<String>, after: impl Into<String>) -> Self {
        Self {
            before: before.into(),
            after: after.into(),
            label_before: None,
            label_after: None,
            title: None,
            note: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_labels(mut self, before: impl Into<String>, after: impl Into<String>) -> Self {
        self.label_before = Some(before.into());
        self.label_after = Some(after.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

impl Component for BeforeAfter {
    fn component_id(&self) -> &'static str {
        "before-after"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "why-it-matters",
    data: WhyItMatters,
    template: include_str!("../../templates/components/why_it_matters.typ"),
    description: "Highlighted explanation block for key insights",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Context / relevance block for storytelling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhyItMatters {
    pub body: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub evidence: Option<String>,
    #[serde(default)]
    pub urgency: Option<String>,
}

impl WhyItMatters {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            title: None,
            evidence: None,
            urgency: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.evidence = Some(evidence.into());
        self
    }

    pub fn with_urgency(mut self, urgency: impl Into<String>) -> Self {
        self.urgency = Some(urgency.into());
        self
    }
}

impl Component for WhyItMatters {
    fn component_id(&self) -> &'static str {
        "why-it-matters"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── PhaseBlock ──────────────────────────────────────────────────────────────

component_catalog! {
    id: "phase-block",
    data: PhaseBlock,
    template: include_str!("../../templates/components/phase_block.typ"),
    description: "Project phase block with status, owner, and date range",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// A phase block for roadmap/plan displays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseBlock {
    pub phase_number: u8,
    pub phase_label: String,
    pub description: String,
    pub items: Vec<String>,
    #[serde(default)]
    pub accent_color: Option<String>,
    #[serde(default)]
    pub item_count_total: Option<usize>,
}

impl PhaseBlock {
    pub fn new(number: u8, label: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            phase_number: number,
            phase_label: label.into(),
            description: description.into(),
            items: Vec::new(),
            accent_color: None,
            item_count_total: None,
        }
    }

    pub fn with_items(mut self, items: Vec<String>) -> Self {
        self.items = items;
        self
    }

    pub fn with_total(mut self, total: usize) -> Self {
        self.item_count_total = Some(total);
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.accent_color = Some(color.into());
        self
    }
}

impl Component for PhaseBlock {
    fn component_id(&self) -> &'static str {
        "phase-block"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── ChecklistPanel ──────────────────────────────────────────────────────────

/// A single row in a ChecklistPanel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistRow {
    pub label: String,
    pub diagnosis: String,
    #[serde(default)]
    pub status: Option<String>,
}

impl ChecklistRow {
    pub fn new(label: impl Into<String>, diagnosis: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            diagnosis: diagnosis.into(),
            status: None,
        }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

component_catalog! {
    id: "checklist-panel",
    data: ChecklistPanel,
    template: include_str!("../../templates/components/checklist_panel.typ"),
    description: "Checklist panel with pass/fail/warning rows",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::Breakable,
}

/// Card showing label–diagnosis pairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistPanel {
    #[serde(default)]
    pub title: Option<String>,
    pub rows: Vec<ChecklistRow>,
}

impl ChecklistPanel {
    pub fn new(rows: Vec<ChecklistRow>) -> Self {
        Self { title: None, rows }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for ChecklistPanel {
    fn component_id(&self) -> &'static str {
        "checklist-panel"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── ImpactGrid ──────────────────────────────────────────────────────────────

/// A single card inside an ImpactGrid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactGridCard {
    pub label: String,
    pub headline: String,
    pub body: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

impl ImpactGridCard {
    pub fn new(
        label: impl Into<String>,
        headline: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            headline: headline.into(),
            body: body.into(),
            status: None,
            icon: None,
        }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

component_catalog! {
    id: "impact-grid",
    data: ImpactGrid,
    template: include_str!("../../templates/components/impact_grid.typ"),
    description: "Grid of impact cards with title, score, and description",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Three-column impact card layout (user / risk / conversion)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactGrid {
    pub user: ImpactGridCard,
    pub risk: ImpactGridCard,
    pub conversion: ImpactGridCard,
    #[serde(default)]
    pub title: Option<String>,
}

impl ImpactGrid {
    pub fn new(user: ImpactGridCard, risk: ImpactGridCard, conversion: ImpactGridCard) -> Self {
        Self {
            user,
            risk,
            conversion,
            title: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for ImpactGrid {
    fn component_id(&self) -> &'static str {
        "impact-grid"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── SpotlightCard ───────────────────────────────────────────────────────────

/// Visual variant for a spotlight card.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SpotlightCardVariant {
    Critical,
    #[default]
    Info,
    Feature,
    Opportunity,
}

impl SpotlightCardVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::Info => "info",
            Self::Feature => "feature",
            Self::Opportunity => "opportunity",
        }
    }
}

component_catalog! {
    id: "spotlight-card",
    data: SpotlightCard,
    template: include_str!("../../templates/components/spotlight_card.typ"),
    description: "Spotlight card with colored severity stripe and body",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// General-purpose spotlight card with left severity stripe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotlightCard {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub variant: Option<String>,
    #[serde(default)]
    pub eyebrow: Option<String>,
    #[serde(default)]
    pub metric: Option<String>,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
}

impl SpotlightCard {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            variant: None,
            eyebrow: None,
            metric: None,
            detail: None,
            action: None,
        }
    }

    /// Set variant from a string (legacy convenience).
    pub fn with_variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }

    /// Set variant from the typed enum.
    pub fn variant(mut self, variant: SpotlightCardVariant) -> Self {
        self.variant = Some(variant.as_str().to_string());
        self
    }

    pub fn with_eyebrow(mut self, eyebrow: impl Into<String>) -> Self {
        self.eyebrow = Some(eyebrow.into());
        self
    }

    pub fn with_metric(mut self, metric: impl Into<String>) -> Self {
        self.metric = Some(metric.into());
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
}

impl Component for SpotlightCard {
    fn component_id(&self) -> &'static str {
        "spotlight-card"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "recommendation-card",
    data: RecommendationCard,
    template: include_str!("../../templates/components/recommendation_card.typ"),
    description: "Recommendation card with impact, effort, and priority badges",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Recommendation card
/// Lighter than Finding: recommendation + impact + effort + priority
/// For: Audit recommendations, best practices, improvements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationCard {
    /// Recommendation title
    pub title: String,
    /// Brief description
    pub description: String,
    /// Impact level (high | medium | low)
    #[serde(default)]
    pub impact: Option<String>,
    /// Effort required (high | medium | low)
    #[serde(default)]
    pub effort: Option<String>,
    /// Priority (critical | high | medium | low)
    #[serde(default)]
    pub priority: Option<String>,
}

impl RecommendationCard {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            impact: None,
            effort: None,
            priority: None,
        }
    }

    pub fn with_impact(mut self, impact: impl Into<String>) -> Self {
        self.impact = Some(impact.into());
        self
    }

    pub fn with_effort(mut self, effort: impl Into<String>) -> Self {
        self.effort = Some(effort.into());
        self
    }

    pub fn with_priority(mut self, priority: impl Into<String>) -> Self {
        self.priority = Some(priority.into());
        self
    }
}

impl Component for RecommendationCard {
    fn component_id(&self) -> &'static str {
        "recommendation-card"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "step-card-row",
    data: StepCardRow,
    template: include_str!("../../templates/components/step_card_row.typ"),
    description: "Horizontal row of numbered step cards",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Step card row
/// Numbered steps displayed horizontally (2-4 steps)
/// For: "How it works", process overview, methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepCardRow {
    /// Step titles
    pub titles: Vec<String>,
    /// Step descriptions
    pub descriptions: Vec<String>,
    /// Number of columns (2-4, defaults to 3)
    #[serde(default = "default_step_columns")]
    pub columns: usize,
}

fn default_step_columns() -> usize {
    3
}

impl Default for StepCardRow {
    fn default() -> Self {
        Self::new()
    }
}

impl StepCardRow {
    pub fn new() -> Self {
        Self {
            titles: Vec::new(),
            descriptions: Vec::new(),
            columns: default_step_columns(),
        }
    }

    pub fn add_step(mut self, title: impl Into<String>, description: impl Into<String>) -> Self {
        self.titles.push(title.into());
        self.descriptions.push(description.into());
        self
    }

    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = columns.clamp(2, 4);
        self
    }
}

impl Component for StepCardRow {
    fn component_id(&self) -> &'static str {
        "step-card-row"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Diagnosis Panel ─────────────────────────────────────────────────────────

/// A single row in a DiagnosisPanel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosisRow {
    pub label: String,
    pub diagnosis: String,
    #[serde(default)]
    pub status: Option<String>,
}

impl DiagnosisRow {
    pub fn new(label: impl Into<String>, diagnosis: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            diagnosis: diagnosis.into(),
            status: None,
        }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

component_catalog! {
    id: "diagnosis-panel",
    data: DiagnosisPanel,
    template: include_str!("../../templates/components/diagnosis_panel.typ"),
    description: "Diagnosis panel with findings grouped by severity",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::Breakable,
}

/// Card with label–diagnosis rows and optional status indicators
/// For: Technical overview, module diagnosis, quick health checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosisPanel {
    #[serde(default)]
    pub title: Option<String>,
    pub rows: Vec<DiagnosisRow>,
}

impl DiagnosisPanel {
    pub fn new(rows: Vec<DiagnosisRow>) -> Self {
        Self { title: None, rows }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for DiagnosisPanel {
    fn component_id(&self) -> &'static str {
        "diagnosis-panel"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Dominant Issue Spotlight ────────────────────────────────────────────────

component_catalog! {
    id: "dominant-issue-spotlight",
    data: DominantIssueSpotlight,
    template: include_str!("../../templates/components/dominant_issue_spotlight.typ"),
    description: "Full-width spotlight for the most critical finding",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Full-width spotlight for a single dominant issue
/// For: Highlighting the biggest problem in an audit report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DominantIssueSpotlight {
    pub title: String,
    pub severity: String,
    pub body: String,
    pub user_impact: String,
    pub recommendation: String,
    #[serde(default)]
    pub eyebrow: Option<String>,
    #[serde(default)]
    pub affected_count: Option<u32>,
    /// Label for the "user impact" field. Defaults to German ("Nutzer-Wirkung")
    /// in the template when not set — pass this explicitly for non-German reports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_user_impact: Option<String>,
    /// Label for the "recommendation" field. Defaults to German ("Empfehlung")
    /// in the template when not set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_recommendation: Option<String>,
    /// Suffix appended after `affected_count` (e.g. "% Anteil" / "% share").
    /// Defaults to German in the template when not set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_share_suffix: Option<String>,
}

impl DominantIssueSpotlight {
    pub fn new(
        title: impl Into<String>,
        severity: impl Into<String>,
        body: impl Into<String>,
        user_impact: impl Into<String>,
        recommendation: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            severity: severity.into(),
            body: body.into(),
            user_impact: user_impact.into(),
            recommendation: recommendation.into(),
            eyebrow: None,
            affected_count: None,
            label_user_impact: None,
            label_recommendation: None,
            label_share_suffix: None,
        }
    }

    pub fn with_eyebrow(mut self, eyebrow: impl Into<String>) -> Self {
        self.eyebrow = Some(eyebrow.into());
        self
    }

    pub fn with_affected_count(mut self, count: u32) -> Self {
        self.affected_count = Some(count);
        self
    }

    /// Sets the user-impact/recommendation/share-suffix labels — required for
    /// a non-German report, since the template's own defaults are German.
    pub fn with_labels(
        mut self,
        label_user_impact: impl Into<String>,
        label_recommendation: impl Into<String>,
        label_share_suffix: impl Into<String>,
    ) -> Self {
        self.label_user_impact = Some(label_user_impact.into());
        self.label_recommendation = Some(label_recommendation.into());
        self.label_share_suffix = Some(label_share_suffix.into());
        self
    }
}

impl Component for DominantIssueSpotlight {
    fn component_id(&self) -> &'static str {
        "dominant-issue-spotlight"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Wrong/Right Block ───────────────────────────────────────────────────────

component_catalog! {
    id: "wrong-right-block",
    data: WrongRightBlock,
    template: include_str!("../../templates/components/wrong_right_block.typ"),
    description: "Wrong vs. right comparison with annotated examples",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Before/after comparison block (wrong vs. right)
/// For: Code examples, best practice comparisons, fix demonstrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrongRightBlock {
    pub wrong: String,
    pub right: String,
    #[serde(default)]
    pub wrong_label: Option<String>,
    #[serde(default)]
    pub right_label: Option<String>,
    #[serde(default)]
    pub is_code: bool,
    #[serde(default)]
    pub note: Option<String>,
}

impl WrongRightBlock {
    pub fn new(wrong: impl Into<String>, right: impl Into<String>) -> Self {
        Self {
            wrong: wrong.into(),
            right: right.into(),
            wrong_label: None,
            right_label: None,
            is_code: false,
            note: None,
        }
    }

    pub fn code(mut self) -> Self {
        self.is_code = true;
        self
    }

    pub fn with_labels(
        mut self,
        wrong_label: impl Into<String>,
        right_label: impl Into<String>,
    ) -> Self {
        self.wrong_label = Some(wrong_label.into());
        self.right_label = Some(right_label.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

impl Component for WrongRightBlock {
    fn component_id(&self) -> &'static str {
        "wrong-right-block"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
