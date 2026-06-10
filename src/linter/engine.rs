//! Lint engine.

use crate::linter::{context::LintContext, finding::Finding, rule::Rule};

/// Lint execution engine.
pub struct LintEngine {
    /// Rules to be loaded into the lint engine
    rules: Vec<Box<dyn Rule>>,
}

impl LintEngine {
    /// Create an empty engine.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Register a rule.
    pub fn register<R>(&mut self, rule: R)
    where
        R: Rule + 'static,
    {
        self.rules.push(Box::new(rule));
    }

    /// Run all registered rules.
    pub fn run(&self, context: &LintContext) -> Vec<Finding> {
        let mut findings = Vec::new();

        for rule in &self.rules {
            rule.check(context, &mut findings);
        }

        findings
    }
}

impl Default for LintEngine {
    fn default() -> Self {
        Self::new()
    }
}
