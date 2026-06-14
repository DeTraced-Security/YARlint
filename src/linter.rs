//! YARlint linting engine.

pub mod context;
pub mod cops;
pub mod engine;
pub mod finding;
pub mod rule;

use cops::naming::rule_name::NamingRuleName;
use engine::LintEngine;

use crate::linter::cops::lint::empty_string::LintEmptyString;

/// Creates the default lint engine with all built-in cops.
pub fn default_engine() -> LintEngine {
    let mut engine = LintEngine::new();

    engine.register(NamingRuleName);
    engine.register(LintEmptyString);

    engine
}
