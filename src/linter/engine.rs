//! Lint engine.

use crate::linter::{context::LintContext, cop::Cop, finding::Finding};

/// Lint execution engine.
pub struct LintEngine {
    /// Cops to be loaded into the lint engine
    cops: Vec<Box<dyn Cop>>,
}

impl LintEngine {
    /// Create an empty engine.
    pub fn new() -> Self {
        Self { cops: Vec::new() }
    }

    /// Register a rule.
    ///
    /// # Arguments
    ///
    /// * `cop` (`C`) - cop to be registered to the engine
    pub fn register<C>(&mut self, cop: C)
    where
        C: Cop + 'static,
    {
        self.cops.push(Box::new(cop));
    }

    /// Run all registered cops.
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - a LintContext containing YARA rules to
    ///   be tested
    ///
    /// # Returns
    ///
    /// Returns a vector containing the any violations found in the rules
    /// checked
    pub fn run(&self, context: &LintContext) -> Vec<Finding> {
        let mut findings = Vec::new();

        for cop in &self.cops {
            cop.check(context, &mut findings);
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
