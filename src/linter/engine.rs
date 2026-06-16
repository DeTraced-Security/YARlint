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
    /// 
    /// # Arguments
    /// 
    /// * `rule` (`R`) - rule to be registered to the engine
    pub fn register<R>(&mut self, rule: R)
    where
        R: Rule + 'static,
    {
        self.rules.push(Box::new(rule));
    }

    /// Run all registered rules.
    /// 
    /// # Arguments
    /// 
    /// * `context` (`&LintContext`) - a LintContext containing YARA rules to
    /// be tested
    /// 
    /// # Returns
    /// 
    /// Returns a vector containing the any violations found in the rules
    /// checked
    pub fn run(&self, context: &LintContext) -> Vec<Finding> {
        let mut findings = Vec::new();

        for rule in &self.rules {
            rule.check(context, &mut findings);
        }

        findings
    }
}

impl Default for LintEngine {
    /// Equivalent to [`LintEngine::new`]
    fn default() -> Self {
        Self::new()
    }
}
