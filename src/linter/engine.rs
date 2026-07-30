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

    /// All registered cops, as borrowed trait objects.
    ///
    /// Used by callers that need to resolve an enabled subset (see
    /// [`crate::linter::cops::selector::resolve_enabled_cops`]) before
    /// running.
    pub fn cops(&self) -> &[Box<dyn Cop>] {
        &self.cops
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
        let all: Vec<&dyn Cop> = self.cops.iter().map(AsRef::as_ref).collect();
        self.run_selected(context, &all)
    }

    /// Run only the given subset of registered cops.
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - a LintContext containing YARA rules to
    ///   be tested
    /// * `enabled` (`&[&dyn Cop]`) - the subset of cops to run, e.g. as
    ///   resolved by [`crate::linter::cops::selector::resolve_enabled_cops`]
    ///
    /// # Returns
    ///
    /// Returns a vector containing the any violations found in the rules
    /// checked
    pub fn run_selected(&self, context: &LintContext, enabled: &[&dyn Cop]) -> Vec<Finding> {
        let mut findings = Vec::new();

        for cop in enabled {
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
