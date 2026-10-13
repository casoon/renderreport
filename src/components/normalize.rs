//! Fills in what a component's Rust builder would have computed, for
//! component data that arrives as JSON (CLI, Wasm, a `RenderRequest` built by
//! hand).
//!
//! Templates read fields that `Component::to_data` derives — a score card's
//! `computed_status`, a word search's grid — and that a JSON author cannot be
//! expected to supply. [`normalize_components`] runs each built-in
//! component's data through its struct (serde defaults, then `to_data`), so a
//! request needs only the fields the builder itself requires.

use serde::de::DeserializeOwned;
use serde_json::Value;

use super::Component;

/// Normalizes every component in `components`, including components nested
/// in other components' data (grid items, flow groups, columns).
///
/// Data that does not deserialize into the component's struct — a missing
/// required field, a wrong type, or data that is already the output of
/// `to_data` — is left as it is, so the render behaves as before.
pub fn normalize_components(components: &mut [Value]) {
    for component in components {
        normalize_value(component);
    }
}

/// Walks the JSON tree depth-first, so nested components are normalized
/// before the component that contains them.
fn normalize_value(value: &mut Value) {
    match value {
        Value::Array(items) => items.iter_mut().for_each(normalize_value),
        Value::Object(map) => {
            map.values_mut().for_each(normalize_value);
            let Some(Value::String(id)) = map.get("type") else {
                return;
            };
            let id = id.clone();
            if let Some(Value::Object(data)) = map.get_mut("data") {
                if let Some(Value::Object(computed)) =
                    normalize_data(&id, &Value::Object(data.clone()))
                {
                    // Keys the struct does not know stay; the struct's values win.
                    data.extend(computed);
                }
            }
        }
        _ => {}
    }
}

/// `to_data` of the built-in component `id` deserialized from `data`, or
/// `None` for unknown ids and data that does not fit the struct.
fn normalize_data(id: &str, data: &Value) -> Option<Value> {
    match id {
        "audit-table" => typed::<super::standard::AuditTable>(data),
        #[cfg(feature = "barcodes")]
        "barcode" => typed::<super::barcode::Barcode>(data),
        "before-after" => typed::<super::standard::BeforeAfter>(data),
        "benchmark-table" => typed::<super::standard::BenchmarkTable>(data),
        "benefit-strip" => typed::<super::advanced::BenefitStrip>(data),
        "big-number" => typed::<super::advanced::BigNumber>(data),
        "callout" => typed::<super::standard::Callout>(data),
        "card-dashboard" => typed::<super::standard::CardDashboard>(data),
        "chart" => typed::<super::charts::Chart>(data),
        "checklist-panel" => typed::<super::advanced::ChecklistPanel>(data),
        "columns" => typed::<super::advanced::Columns>(data),
        "comparison-block" => typed::<super::advanced::ComparisonBlock>(data),
        "comparison-cluster" => typed::<super::advanced::ComparisonCluster>(data),
        "cover-page" => typed::<super::standard::CoverPage>(data),
        "crosstab" => typed::<super::crosstab::Crosstab>(data),
        "cta-box" => typed::<super::standard::CTABox>(data),
        "date-field" => typed::<super::text::DateField>(data),
        "device-preview" => typed::<super::advanced::DevicePreview>(data),
        "diagnosis-panel" => typed::<super::advanced::DiagnosisPanel>(data),
        "divider" => typed::<super::advanced::Divider>(data),
        "dominant-issue-spotlight" => typed::<super::advanced::DominantIssueSpotlight>(data),
        "eyebrow" => typed::<super::text::Eyebrow>(data),
        "fact-box" => typed::<super::standard::FactBox>(data),
        "faq-list" => typed::<super::advanced::FaqList>(data),
        "feature-grid" => typed::<super::standard::FeatureGrid>(data),
        "finding" => typed::<super::standard::Finding>(data),
        "flow-group" => typed::<super::advanced::FlowGroup>(data),
        "funnel" => typed::<super::standard::Funnel>(data),
        "gauge" => typed::<super::charts::Gauge>(data),
        "glossary-list" => typed::<super::advanced::GlossaryList>(data),
        "grid-component" => typed::<super::advanced::Grid>(data),
        "hero-summary" => typed::<super::standard::HeroSummary>(data),
        "image" => typed::<super::standard::Image>(data),
        "impact-grid" => typed::<super::advanced::ImpactGrid>(data),
        "key-value-list" => typed::<super::advanced::KeyValueList>(data),
        "label" => typed::<super::text::Label>(data),
        "list" => typed::<super::advanced::List>(data),
        "logo-strip" => typed::<super::advanced::LogoStrip>(data),
        "metric-card" => typed::<super::standard::MetricCard>(data),
        "metric-strip" => typed::<super::advanced::MetricStrip>(data),
        "module-comparison" => typed::<super::standard::ModuleComparison>(data),
        "number-field" => typed::<super::text::NumberField>(data),
        "page-break" => typed::<super::advanced::PageBreak>(data),
        "phase-block" => typed::<super::advanced::PhaseBlock>(data),
        "pivot-table" => typed::<super::crosstab::PivotTable>(data),
        "portfolio-summary" => typed::<super::standard::PortfolioSummary>(data),
        "pricing-card" => typed::<super::advanced::PricingCard>(data),
        "problem-solution" => typed::<super::standard::ProblemSolution>(data),
        "process-flow" => typed::<super::standard::ProcessFlow>(data),
        "product-hero" => typed::<super::standard::ProductHero>(data),
        "progress-bar" => typed::<super::advanced::ProgressBar>(data),
        "pull-quote" => typed::<super::advanced::PullQuote>(data),
        "quote-block" => typed::<super::standard::QuoteBlock>(data),
        "recommendation-card" => typed::<super::advanced::RecommendationCard>(data),
        "resource-field" => typed::<super::text::ResourceField>(data),
        "roadmap-block" => typed::<super::standard::RoadmapBlock>(data),
        "score-band" => typed::<super::standard::ScoreBand>(data),
        "score-card" => typed::<super::standard::ScoreCard>(data),
        "section" => typed::<super::standard::Section>(data),
        "section-header-split" => typed::<super::advanced::SectionHeaderSplit>(data),
        "severity-overview" => typed::<super::standard::SeverityOverview>(data),
        "side-label" => typed::<super::advanced::SideLabel>(data),
        "sparkline" => typed::<super::charts::Sparkline>(data),
        "spotlight-card" => typed::<super::advanced::SpotlightCard>(data),
        "stat" => typed::<super::standard::Stat>(data),
        "stat-pair" => typed::<super::standard::StatPair>(data),
        "status-pill" => typed::<super::standard::StatusPill>(data),
        "step-card-row" => typed::<super::advanced::StepCardRow>(data),
        "summary-box" => typed::<super::standard::SummaryBox>(data),
        "table-of-contents" => typed::<super::advanced::TableOfContents>(data),
        "tag-cloud" => typed::<super::standard::TagCloud>(data),
        "testimonial" => typed::<super::standard::Testimonial>(data),
        "textblock" => typed::<super::text::TextBlock>(data),
        "timeline" => typed::<super::standard::Timeline>(data),
        "trend-tile" => typed::<super::standard::TrendTile>(data),
        "use-case-card" => typed::<super::advanced::UseCaseCard>(data),
        "watermark" => typed::<super::advanced::Watermark>(data),
        "why-it-matters" => typed::<super::standard::WhyItMatters>(data),
        "word-search" => typed::<super::wordsearch::WordSearch>(data),
        "wrong-right-block" => typed::<super::advanced::WrongRightBlock>(data),
        _ => None,
    }
}

fn typed<T: Component + DeserializeOwned>(data: &Value) -> Option<Value> {
    serde_json::from_value::<T>(data.clone())
        .ok()
        .map(|c| c.to_data())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn normalized(component: Value) -> Value {
        let mut components = vec![component];
        normalize_components(&mut components);
        components.remove(0)
    }

    #[test]
    fn fills_in_derived_fields() {
        let card =
            normalized(json!({ "type": "score-card", "data": { "title": "SEO", "score": 64 } }));
        assert_eq!(card["data"]["computed_status"], "warning");
        assert_eq!(card["data"]["max_score"], 100);

        let callout = normalized(
            json!({ "type": "callout", "data": { "variant": "warning", "content": "x" } }),
        );
        assert_eq!(callout["data"]["callout_type"], "warning");
    }

    #[test]
    fn normalizes_nested_components() {
        let grid = normalized(json!({
            "type": "grid-component",
            "data": { "columns": 2, "items": [{ "content": {
                "type": "score-card", "data": { "title": "SEO", "score": 95 }
            } }] }
        }));
        assert_eq!(
            grid["data"]["items"][0]["content"]["data"]["computed_status"],
            "good"
        );
    }

    #[test]
    fn keeps_unknown_fields() {
        let card = normalized(json!({
            "type": "score-card", "data": { "title": "SEO", "score": 64, "note": "kept" }
        }));
        assert_eq!(card["data"]["note"], "kept");
    }

    #[test]
    fn leaves_data_alone_that_does_not_fit() {
        let wrong_type =
            json!({ "type": "score-card", "data": { "title": "SEO", "score": "high" } });
        assert_eq!(normalized(wrong_type.clone()), wrong_type);

        let unknown = json!({ "type": "my-widget", "data": { "label": "x" } });
        assert_eq!(normalized(unknown.clone()), unknown);
    }

    #[test]
    fn is_idempotent_on_builder_output() {
        let data = crate::components::WordSearch::builder()
            .words(vec!["CAT", "DOG"])
            .seed(7)
            .build()
            .unwrap()
            .to_data();
        let component = json!({ "type": "word-search", "data": data });
        assert_eq!(normalized(component.clone()), component);
    }
}
