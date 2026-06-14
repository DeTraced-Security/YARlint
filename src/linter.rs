//! YARlint linting engine.

pub mod context;
pub mod cops;
pub mod engine;
pub mod finding;
pub mod rule;

use engine::LintEngine;

use cops::{
    lint::{empty_string::LintEmptyString, empty_string_block::LintEmptyStringBlock},
    naming::rule_name::NamingRuleName,
};

/// Creates the default lint engine with all built-in cops.
pub fn default_engine() -> LintEngine {
    let mut engine = LintEngine::new();

    engine.register(NamingRuleName);
    engine.register(LintEmptyString);
    engine.register(LintEmptyStringBlock);

    engine
}
