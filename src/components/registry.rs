//! Component registry for managing available components

use std::collections::HashMap;
use std::sync::Arc;

type ValidatorFn = Arc<dyn Fn(&serde_json::Value) -> bool + Send + Sync>;

/// Unique identifier for a component type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub String);

impl ComponentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ComponentId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Registry of available component types
#[derive(Default)]
pub struct ComponentRegistry {
    /// Component templates (Typst code)
    templates: HashMap<ComponentId, String>,
    /// Insertion order for deterministic output
    insertion_order: Vec<ComponentId>,
    /// Component factories for validation
    validators: HashMap<ComponentId, ValidatorFn>,
}

impl ComponentRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a registry with standard components
    pub fn with_standard_components() -> Self {
        let mut registry = Self::new();
        registry.register_standard_components();
        registry
    }

    /// Register standard built-in components from the [`crate::ComponentCatalog`].
    ///
    /// Dispatcher templates (`grid-component`, `flow-group`) are registered
    /// last because their Typst bodies reference all other component functions.
    pub fn register_standard_components(&mut self) {
        use crate::components::catalog::ComponentCatalog;

        // The catalog yields descriptors in `inventory` link order, which differs
        // across platforms (macOS vs Linux). Typst captures `#let` closures at
        // definition time, so a component whose template calls another component's
        // function (e.g. `card-dashboard` calls `metric-card`, the dispatchers call
        // everything) must be registered *after* its dependency. We therefore
        // topologically sort the templates by their cross-references, breaking ties
        // by id, which is both dependency-safe and deterministic across platforms.
        let descs: Vec<&'static crate::components::catalog::ComponentDescriptor> =
            ComponentCatalog::all().collect();
        let ids: Vec<&'static str> = descs.iter().map(|d| d.id).collect();

        // Build edges: dep -> set of components that reference it.
        let references = |template: &str, candidate: &str| -> bool {
            // Match `candidate(` not preceded by an identifier char or hyphen,
            // so `metric-card` does not match inside `big-metric-card`.
            let needle = format!("{candidate}(");
            template.match_indices(&needle).any(|(abs, _)| {
                abs == 0
                    || !matches!(
                        template.as_bytes().get(abs - 1),
                        Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_')
                    )
            })
        };

        let mut indegree: HashMap<&'static str, usize> = ids.iter().map(|id| (*id, 0)).collect();
        let mut dependents: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
        for d in &descs {
            for dep in &ids {
                if *dep != d.id && references(d.template, dep) {
                    dependents.entry(dep).or_default().push(d.id);
                    if let Some(deg) = indegree.get_mut(d.id) {
                        *deg += 1;
                    }
                }
            }
        }

        // Kahn's algorithm with an id-sorted ready set for deterministic output.
        let desc_by_id: HashMap<
            &'static str,
            &'static crate::components::catalog::ComponentDescriptor,
        > = descs.iter().map(|d| (d.id, *d)).collect();
        let mut ready: Vec<&'static str> = indegree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(id, _)| *id)
            .collect();
        ready.sort_unstable_by(|a, b| b.cmp(a));
        let mut emitted = 0usize;
        while let Some(id) = ready.pop() {
            // `ready` is sorted ascending; pop the lexicographically smallest by
            // keeping it sorted descending instead.
            if let Some(desc) = desc_by_id.get(id) {
                self.register(ComponentId::new(desc.id), desc.template.to_string());
                emitted += 1;
            }
            if let Some(children) = dependents.get(id) {
                let mut newly_ready = Vec::new();
                for child in children {
                    if let Some(deg) = indegree.get_mut(child) {
                        *deg -= 1;
                        if *deg == 0 {
                            newly_ready.push(*child);
                        }
                    }
                }
                for c in newly_ready {
                    ready.push(c);
                }
                ready.sort_unstable_by(|a, b| b.cmp(a));
            }
        }

        // Cycle fallback: if cross-references form a cycle (should not happen),
        // emit any stragglers in id order so nothing is silently dropped.
        if emitted < descs.len() {
            let mut remaining: Vec<&'static str> = descs
                .iter()
                .map(|d| d.id)
                .filter(|id| !self.templates.contains_key(&ComponentId::new(*id)))
                .collect();
            remaining.sort_unstable();
            for id in remaining {
                if let Some(desc) = desc_by_id.get(id) {
                    self.register(ComponentId::new(desc.id), desc.template.to_string());
                }
            }
        }
    }

    /// Register a component template
    pub fn register(&mut self, id: ComponentId, template: String) {
        if !self.templates.contains_key(&id) {
            self.insertion_order.push(id.clone());
        }
        self.templates.insert(id, template);
    }

    /// Register a component with validator
    pub fn register_with_validator(
        &mut self,
        id: ComponentId,
        template: String,
        validator: impl Fn(&serde_json::Value) -> bool + Send + Sync + 'static,
    ) {
        self.templates.insert(id.clone(), template);
        self.validators.insert(id, Arc::new(validator));
    }

    /// Get a component template
    pub fn get_template(&self, id: &ComponentId) -> Option<&String> {
        self.templates.get(id)
    }

    /// Check if a component is registered
    pub fn has_component(&self, id: &ComponentId) -> bool {
        self.templates.contains_key(id)
    }

    /// Validate component data
    pub fn validate(&self, id: &ComponentId, data: &serde_json::Value) -> bool {
        self.validators.get(id).map(|v| v(data)).unwrap_or(true)
    }

    /// List all registered component IDs in insertion order
    pub fn list_components(&self) -> Vec<&ComponentId> {
        self.insertion_order.iter().collect()
    }
}

impl std::fmt::Debug for ComponentRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentRegistry")
            .field("templates", &self.templates.keys().collect::<Vec<_>>())
            .field("validators_count", &self.validators.len())
            .finish()
    }
}
