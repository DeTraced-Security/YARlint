//! YARlint linting engine.

pub mod context;
pub mod cops;
pub mod engine;
pub mod finding;
pub mod rule;

use engine::LintEngine;

use cops::{
    lint::{
        duplicate_string::LintDuplicateString, empty_string::LintEmptyString,
        empty_strings_block::LintEmptyStringsBlock,
    },
    naming::rule_name::NamingRuleName,
    style::{
        meta_keys_order::StyleMetaKeysOrder, missing_required_meta::StyleMissingRequiredMeta,
        string_identifier::StyleStringIdentifier,
    },
};

/// Creates the default lint engine with all built-in cops.
pub fn default_engine() -> LintEngine {
    let mut engine = LintEngine::new();

    engine.register(NamingRuleName);
    engine.register(LintEmptyString);
    engine.register(LintEmptyStringsBlock);
    engine.register(LintDuplicateString);
    engine.register(StyleStringIdentifier);
    engine.register(StyleMetaKeysOrder);
    engine.register(StyleMissingRequiredMeta);

    engine
}
