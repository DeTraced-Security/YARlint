//! YARlint linting engine.

pub mod analysis;
pub mod context;
pub mod cop;
pub mod cops;
pub mod engine;
pub mod finding;

use engine::LintEngine;

use cops::{
    lint::{
        duplicate_meta::LintDuplicateMeta, duplicate_string::LintDuplicateString,
        empty_string::LintEmptyString, empty_strings_block::LintEmptyStringsBlock,
    },
    logic::constant_condition::LogicConstantCondition,
    naming::{
        descriptive_meta::NamingDescriptiveMeta, rule_name::NamingRuleName,
        rule_name_length::NamingRuleNameLength,
    },
    performance::wide_string_without_ascii::PerformanceWideStringWithoutAscii,
    style::{
        condition_spacing::StyleConditionSpacing, meta_keys_order::StyleMetaKeysOrder,
        missing_required_meta::StyleMissingRequiredMeta, rule_name_case::StyleRuleNameCase,
        string_identifier::StyleStringIdentifier,
    },
};

use crate::{config, parser::token::Token};

/// Creates the default lint engine with all built-in cops.
pub fn default_engine() -> LintEngine {
    default_engine_with_tokens(Vec::new())
}

/// Creates the default lint engine for a tokenized YARA file.
///
/// The token stream is moved into cops that require source location data.
///
/// # Arguments
///
/// * `tokens` (`Vec<Token>`) - Tokens produced while parsing the file
///
/// # Returns
///
/// Returns a lint engine containing every built-in cop.
pub fn default_engine_with_tokens(tokens: Vec<Token>) -> LintEngine {
    let mut engine = LintEngine::new();
    engine.register(LintDuplicateMeta);
    engine.register(LintDuplicateString);
    engine.register(LintEmptyString);
    engine.register(LintEmptyStringsBlock);

    engine.register(LogicConstantCondition);

    engine.register(NamingDescriptiveMeta);
    engine.register(NamingRuleNameLength);
    engine.register(NamingRuleName);

    engine.register(PerformanceWideStringWithoutAscii);

    engine.register(StyleConditionSpacing::new(tokens));
    engine.register(StyleMetaKeysOrder);
    engine.register(StyleRuleNameCase::new(config::rule_name_case()));
    engine.register(StyleMissingRequiredMeta);
    engine.register(StyleStringIdentifier);

    engine
}
