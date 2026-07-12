//! YARlint linting engine.

pub mod context;
pub mod engine;
pub mod finding;
pub mod rule;
pub mod rules;

use engine::LintEngine;

use rules::{
    lint::{
        duplicate_string::LintDuplicateString, empty_string::LintEmptyString,
        empty_strings_block::LintEmptyStringsBlock,
    },
    naming::{
        descriptive_meta::NamingDescriptiveMeta, rule_name::NamingRuleName,
        rule_name_length::NamingRuleNameLength,
    },
    style::{
        meta_keys_order::StyleMetaKeysOrder, missing_required_meta::StyleMissingRequiredMeta,
        string_identifier::StyleStringIdentifier,
    },
};

use crate::linter::rules::lint::duplicate_meta::LintDuplicateMeta;

/// Creates the default lint engine with all built-in cops.
pub fn default_engine() -> LintEngine {
    let mut engine = LintEngine::new();

    engine.register(NamingDescriptiveMeta);
    engine.register(NamingRuleName);
    engine.register(NamingRuleNameLength);
    engine.register(LintEmptyString);
    engine.register(LintEmptyStringsBlock);
    engine.register(LintDuplicateMeta);
    engine.register(LintDuplicateString);
    engine.register(StyleStringIdentifier);
    engine.register(StyleMetaKeysOrder);
    engine.register(StyleMissingRequiredMeta);

    engine
}
