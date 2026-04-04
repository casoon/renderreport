# Component API Design — Consistency Guidelines

Alle Komponenten folgen diesen Patterns für konsistente, intuitive APIs.

---

## Standard Builder Pattern

Alle Komponenten nutzen den **Builder Pattern** mit Fluent API:

```rust
MyComponent::new(required_fields)
    .with_optional_field(value)
    .add_item(item)
    .with_title(title)
    // ... mehr options
```

**Constructor:** `new()` mit Pflichtfeldern  
**Setter:** `.with_*()` für optionale Felder  
**Adder:** `.add_*()` zum Anhängen von Items  
**Converter:** `.to_*()` zum Umwandeln in andere Formate

---

## Variant Consistency

Komponenten mit mehreren visuellen Varianten nutzen `.variant()`:

| Component | Variants |
|-----------|----------|
| **SpotlightCard** | `.variant(critical, info, feature, opportunity)` |
| **Finding** | `.severity(critical, high, medium, low)` |
| **PhaseBlock** | `.accent_color("#hex")` |
| **CTABox** | `.tone(primary, urgent, neutral)` |
| **FactBox** | `.variant(info, tip, warning, stat)` |
| **Timeline** | `.status(done, active, planned)` |

**Pattern:**
```rust
SpotlightCard::new(title, body)
    .variant("critical")  // oder severity/tone/accent_color
    .with_eyebrow("Important")
```

---

## Tone Consistency

Komponenten mit **Stimmung/Gewichtung** nutzen `.tone()`:

| Component | Tones |
|-----------|-------|
| **CTABox** | primary, urgent, neutral |
| **Finding** | (implizit via severity) |
| **PhaseBlock** | (implizit via phase_number) |

```rust
CTABox::new(headline)
    .with_tone("primary")  // visually prominent
    .with_body(body)
    .with_action(label, url)
```

---

## Size Consistency

Komponenten mit **Größen-Optionen** nutzen `.with_columns()` oder `.with_size()`:

| Component | Size Option |
|-----------|------------|
| **BenefitStrip** | `.with_columns(2-5)` |
| **FeatureGrid** | `.with_columns(1-4)` |
| **Columns** | `.with_ratio(0.2-0.8)` |
| **LogoStrip** | `.with_columns(2-6)` |

```rust
BenefitStrip::new()
    .add_benefit(title, desc)
    .with_columns(3)  // 3-column layout
```

---

## Title/Label Pattern

Komponenten mit optionalen Überschriften:

```rust
// Variante 1: Explizit
FaqList::new()
    .with_title("Frequently Asked Questions")
    .add_item(q, a)

// Variante 2: Implizit
ProcessFlow::new(steps)
    .with_title("How It Works")  // optional
```

---

## Optional Fields Pattern

```rust
// None-safe builders
RecommendationCard::new(title, description)
    .with_impact("high")      // Option<String>
    .with_effort("low")       // Option<String>
    .with_priority("critical") // Option<String>

// Null-safe rendering in Typst
if data.impact != none [ show impact ]
```

---

## Collection Builders

Für Komponenten mit Items:

```rust
// Plural: FaqList, Timeline, etc.
FaqList::new()
    .add_item(question, answer)
    .add_item(question, answer)
    .with_title("FAQ")

// map/vec: List, FeatureGrid (Arrays)
FeatureGrid::new(vec![
    FeatureGridItem { ... },
    FeatureGridItem { ... },
])
```

---

## Serialization Pattern

Alle Komponenten nutzen `#[derive(Serialize, Deserialize)]`:

```rust
impl Component for MyComponent {
    fn component_id(&self) -> &'static str {
        "my-component"
    }
    
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
```

---

## Registry Pattern

Alle Komponenten müssen in `src/components/registry.rs` registriert sein:

```rust
self.register(
    ComponentId::new("my-component"),
    include_str!("../../templates/components/my_component.typ").to_string(),
);
```

---

## Current API Status ✓

✅ **Fully Consistent:**
- `CTABox` — `.tone()`
- `SpotlightCard` — `.variant()`
- `Finding` — `.severity()`

✅ **Collection Builders:**
- `FaqList.add_item()`
- `Timeline.with_items()`
- `FeatureGrid.new(vec![])`

✅ **Optional Fields:**
- `.with_title()`, `.with_description()`, `.with_outcome()`

---

## API Audit Checklist

Für jede neue Komponente:

- [ ] `new()` mit Pflichtfeldern
- [ ] `.with_*()` für optionale Felder
- [ ] `.add_*()` für Collections
- [ ] Optional: `.variant()` / `.tone()` / `.size()`
- [ ] `#[derive(Serialize, Deserialize)]`
- [ ] Component trait impl + to_data()
- [ ] Registry.rs Registrierung
- [ ] Typst template
- [ ] Catalog example mit `// @id: component-id` annotation

---

## Recommended Future Consistency

### Density API
```rust
Component::new()
    .with_density("compact")  // compact, normal, spacious
```

### Emphasis API
```rust
Component::new()
    .with_emphasis("strong")  // subtle, normal, strong
```

### Layout API
```rust
Component::new()
    .with_layout("vertical")  // für Flow-Komponenten
```

Diese Patterns würden **Cross-Component Konsistenz** erhöhen und das System noch **intuitiver** machen.
