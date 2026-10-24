//! Text and editorial components: callouts, quotes, lists and headers.

use super::catalog::{ComponentCategory, LayoutHint};
use super::Component;
use crate::component_catalog;
use serde::{Deserialize, Serialize};

/// Visual style variant for a callout admonition.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CalloutVariant {
    #[default]
    Info,
    Warning,
    Error,
    Success,
    Tip,
    Neutral,
}

impl CalloutVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Success => "success",
            Self::Tip => "tip",
            Self::Neutral => "neutral",
        }
    }
}

component_catalog! {
    id: "callout",
    data: Callout,
    template: include_str!("../../templates/components/callout.typ"),
    description: "Highlighted information box with icon and variant styling",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Callout/admonition component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Callout {
    /// Callout content
    pub content: String,
    /// Callout type string used by the Typst template (kept for backward compat)
    #[serde(default = "default_callout_type")]
    pub callout_type: String,
    /// Typed variant; takes precedence over `callout_type` when set
    #[serde(default)]
    pub variant: Option<CalloutVariant>,
    /// Optional title
    #[serde(default)]
    pub title: Option<String>,
    /// Inverted: filled background with white text
    #[serde(default)]
    pub inverted: bool,
}

fn default_callout_type() -> String {
    "info".into()
}

impl Callout {
    fn with_variant(content: impl Into<String>, variant: CalloutVariant) -> Self {
        Self {
            content: content.into(),
            callout_type: variant.as_str().to_string(),
            variant: Some(variant),
            title: None,
            inverted: false,
        }
    }

    pub fn info(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Info)
    }

    pub fn warning(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Warning)
    }

    pub fn error(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Error)
    }

    pub fn success(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Success)
    }

    pub fn tip(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Tip)
    }

    pub fn neutral(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Neutral)
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn inverted(mut self) -> Self {
        self.inverted = true;
        self
    }
}

impl Component for Callout {
    fn component_id(&self) -> &'static str {
        "callout"
    }

    fn to_data(&self) -> serde_json::Value {
        let mut data = serde_json::to_value(self).unwrap_or_default();
        // Ensure callout_type reflects the typed variant when both are set
        if let (Some(v), serde_json::Value::Object(ref mut map)) = (self.variant, &mut data) {
            map.insert(
                "callout_type".into(),
                serde_json::Value::String(v.as_str().to_string()),
            );
        }
        data
    }
}

// ── Primitive Components ──────────────────────────────────────────────

component_catalog! {
    id: "status-pill",
    data: StatusPill,
    template: include_str!("../../templates/components/status_pill.typ"),
    description: "Inline status badge with color coding",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Status pill badge — colored label for status display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPill {
    pub label: String,
    #[serde(default = "default_neutral")]
    pub status: String,
    #[serde(default)]
    pub uppercase: bool,
}

fn default_neutral() -> String {
    "neutral".into()
}

impl StatusPill {
    pub fn good(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: "good".into(),
            uppercase: false,
        }
    }
    pub fn warn(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: "warn".into(),
            uppercase: false,
        }
    }
    pub fn bad(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: "bad".into(),
            uppercase: false,
        }
    }
    pub fn neutral(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: "neutral".into(),
            uppercase: false,
        }
    }
    pub fn info(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: "info".into(),
            uppercase: false,
        }
    }
}

impl Component for StatusPill {
    fn component_id(&self) -> &'static str {
        "status-pill"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "tag-cloud",
    data: TagCloud,
    template: include_str!("../../templates/components/tag_cloud.typ"),
    description: "Tag cloud with weighted font sizes",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Wrapping cloud of status-pill badges (flows inline, auto-wraps)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCloud {
    pub items: Vec<TagCloudItem>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default = "default_tag_gap")]
    pub gap: String,
}

fn default_tag_gap() -> String {
    "4pt".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCloudItem {
    pub label: String,
    #[serde(default = "default_neutral")]
    pub status: String,
}

impl TagCloud {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            title: None,
            gap: "4pt".into(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_gap(mut self, gap: impl Into<String>) -> Self {
        self.gap = gap.into();
        self
    }

    pub fn add(mut self, label: impl Into<String>, status: impl Into<String>) -> Self {
        self.items.push(TagCloudItem {
            label: label.into(),
            status: status.into(),
        });
        self
    }
}

impl Default for TagCloud {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for TagCloud {
    fn component_id(&self) -> &'static str {
        "tag-cloud"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "fact-box",
    data: FactBox,
    template: include_str!("../../templates/components/fact_box.typ"),
    description: "Fact box with a prominent statistic and supporting text",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Inline info/fact box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactBox {
    pub body: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub variant: Option<String>,
}

impl FactBox {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            label: None,
            variant: None,
        }
    }

    pub fn info(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            label: None,
            variant: Some("info".into()),
        }
    }

    pub fn tip(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            label: None,
            variant: Some("tip".into()),
        }
    }

    pub fn warning(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            label: None,
            variant: Some("warning".into()),
        }
    }

    pub fn stat(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            label: None,
            variant: Some("stat".into()),
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Component for FactBox {
    fn component_id(&self) -> &'static str {
        "fact-box"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "quote-block",
    data: QuoteBlock,
    template: include_str!("../../templates/components/quote_block.typ"),
    description: "Styled block quote with attribution",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Quote / pull quote block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteBlock {
    pub quote: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub emphasis: Option<bool>,
}

impl QuoteBlock {
    pub fn new(quote: impl Into<String>) -> Self {
        Self {
            quote: quote.into(),
            author: None,
            emphasis: None,
        }
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn emphasized(mut self) -> Self {
        self.emphasis = Some(true);
        self
    }
}

impl Component for QuoteBlock {
    fn component_id(&self) -> &'static str {
        "quote-block"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "list",
    data: List,
    template: include_str!("../../templates/components/list.typ"),
    description: "Bullet or numbered list with optional icons",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::Breakable,
}

/// List component that iterates over data records
/// Inspired by JasperReports List Component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    /// List title
    #[serde(default)]
    pub title: Option<String>,
    /// Items to display
    pub items: Vec<ListItem>,
    /// Layout style (vertical, horizontal, grid)
    #[serde(default = "default_list_layout")]
    pub layout: String,
    /// Columns for grid layout
    #[serde(default = "default_columns")]
    pub columns: usize,
    /// Show item numbers/bullets
    #[serde(default)]
    pub numbered: bool,
}

fn default_list_layout() -> String {
    "vertical".into()
}

fn default_columns() -> usize {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    /// Item content
    pub content: String,
    /// Optional icon/indicator
    #[serde(default)]
    pub icon: Option<String>,
    /// Nested items
    #[serde(default)]
    pub children: Vec<ListItem>,
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl List {
    pub fn new() -> Self {
        Self {
            title: None,
            items: Vec::new(),
            layout: "vertical".into(),
            columns: 1,
            numbered: false,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn add_item(mut self, content: impl Into<String>) -> Self {
        self.items.push(ListItem {
            content: content.into(),
            icon: None,
            children: Vec::new(),
        });
        self
    }

    pub fn add_item_with_icon(
        mut self,
        content: impl Into<String>,
        icon: impl Into<String>,
    ) -> Self {
        self.items.push(ListItem {
            content: content.into(),
            icon: Some(icon.into()),
            children: Vec::new(),
        });
        self
    }

    pub fn grid_layout(mut self, columns: usize) -> Self {
        self.layout = "grid".into();
        self.columns = columns;
        self
    }

    pub fn numbered(mut self) -> Self {
        self.numbered = true;
        self
    }
}

impl Component for List {
    fn component_id(&self) -> &'static str {
        "list"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── SideLabel ───────────────────────────────────────────────────────────────

component_catalog! {
    id: "side-label",
    data: SideLabel,
    template: include_str!("../../templates/components/side_label.typ"),
    description: "Vertical side label aligned alongside content",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Two-column layout: left 1/3 heading, right 2/3 content (bullets or text).
///
/// Ideal for structured information blocks where a label identifies a category
/// and associated details are listed alongside it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideLabel {
    /// Section heading shown in the left column
    pub heading: String,
    /// Optional smaller subheading below the main heading
    #[serde(default)]
    pub subheading: Option<String>,
    /// Bullet items shown in the right column (preferred for lists)
    #[serde(default)]
    pub items: Vec<String>,
    /// Plain text shown in the right column when no items are provided
    #[serde(default)]
    pub text: Option<String>,
    /// Whether to render a horizontal divider below this block
    #[serde(default)]
    pub divider: bool,
}

impl SideLabel {
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            heading: heading.into(),
            subheading: None,
            items: Vec::new(),
            text: None,
            divider: false,
        }
    }

    pub fn with_subheading(mut self, subheading: impl Into<String>) -> Self {
        self.subheading = Some(subheading.into());
        self
    }

    pub fn add_item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_divider(mut self) -> Self {
        self.divider = true;
        self
    }
}

impl Component for SideLabel {
    fn component_id(&self) -> &'static str {
        "side-label"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "faq-list",
    data: FaqList,
    template: include_str!("../../templates/components/faq_list.typ"),
    description: "Expandable FAQ list with question/answer pairs",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::Breakable,
}

/// FAQ list component
/// Question-answer pairs for FAQs, knowledge bases, product docs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqList {
    /// FAQ items
    pub items: Vec<FaqItem>,
    /// Optional title
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqItem {
    /// Question
    pub question: String,
    /// Answer
    pub answer: String,
}

impl Default for FaqList {
    fn default() -> Self {
        Self::new()
    }
}

impl FaqList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            title: None,
        }
    }

    pub fn add_item(mut self, question: impl Into<String>, answer: impl Into<String>) -> Self {
        self.items.push(FaqItem {
            question: question.into(),
            answer: answer.into(),
        });
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for FaqList {
    fn component_id(&self) -> &'static str {
        "faq-list"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "pull-quote",
    data: PullQuote,
    template: include_str!("../../templates/components/pull_quote.typ"),
    description: "Large pull quote with decorative quotation marks",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Pull quote component
/// Large, visually prominent single quote (centered, full-width)
/// For: Key statements, expert opinions, marketing headlines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullQuote {
    /// The quote text
    pub quote: String,
    /// Optional attribution (author, source)
    #[serde(default)]
    pub attribution: Option<String>,
}

impl PullQuote {
    pub fn new(quote: impl Into<String>) -> Self {
        Self {
            quote: quote.into(),
            attribution: None,
        }
    }

    pub fn with_attribution(mut self, attribution: impl Into<String>) -> Self {
        self.attribution = Some(attribution.into());
        self
    }
}

impl Component for PullQuote {
    fn component_id(&self) -> &'static str {
        "pull-quote"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "glossary-list",
    data: GlossaryList,
    template: include_str!("../../templates/components/glossary_list.typ"),
    description: "Definition list for glossary terms",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::Breakable,
}

/// Glossary list component
/// Term-definition pairs for reference, appendix, or terminology
/// For: Glossaries, abbreviations, technical terms, standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryList {
    /// Glossary items
    pub items: Vec<GlossaryItem>,
    /// Optional title
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryItem {
    /// Term/abbreviation
    pub term: String,
    /// Definition
    pub definition: String,
}

impl Default for GlossaryList {
    fn default() -> Self {
        Self::new()
    }
}

impl GlossaryList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            title: None,
        }
    }

    pub fn add_item(mut self, term: impl Into<String>, definition: impl Into<String>) -> Self {
        self.items.push(GlossaryItem {
            term: term.into(),
            definition: definition.into(),
        });
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for GlossaryList {
    fn component_id(&self) -> &'static str {
        "glossary-list"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
