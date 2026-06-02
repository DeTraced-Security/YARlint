//! Parsing of YARA meta sections.
//!
//! The `meta` section contains key-value pairs that provide descriptive
//! information about a rule. This module converts metadata tokens into
//! collections of [`MetaEntryNode`] values.

use crate::parser::{
    ast_parser::AstParser,
    syntax::{MetaEntryNode, MetaValue},
    token::{Token, TokenType},
};

/// Parses the `meta` section of a YARA rule.
///
/// This function consumes metadata entries until the beginning of the
/// `strings` section is encountered. Each entry consists of an
/// identifier key, an equals sign, and a value.
///
/// Supported metadata value types include:
/// - String literals
/// - Numeric literals
/// - Boolean values (`true` and `false`)
///
/// # Arguments
///
/// * `parser` - The active parser positioned at the first metadata
///   entry following the `meta:` section header.
///
/// # Returns
///
/// Returns a vector containing all parsed [`MetaEntryNode`] values.
///
/// # Errors
///
/// Returns an error if:
/// - A metadata key is not a valid identifier.
/// - An equals sign is missing.
/// - A metadata value is not a supported type.
/// - The input ends unexpectedly while parsing metadata.
pub fn parse_meta(parser: &mut AstParser) -> Result<Vec<MetaEntryNode>, String> {
    let mut meta_entries: Vec<MetaEntryNode> = Vec::new();

    while !parser.check(&crate::parser::token::TokenType::Keyword(
        "strings".to_string(),
    )) {
        let key: String = parser.expect_identifier()?;
        parser.expect(&TokenType::Equals)?;

        let value: MetaValue = match parser.peek() {
            Some(Token {
                token_type: TokenType::StringLiteral(_),
                ..
            }) => MetaValue::String(parser.expect_string_literal()?),

            Some(Token {
                token_type: TokenType::Number(_),
                ..
            }) => MetaValue::Number(parser.expect_number()?),

            Some(Token {
                token_type: TokenType::Keyword(k),
                ..
            }) if k == "true" || k == "false" => MetaValue::Boolean(parser.expect_boolean()?),

            _ => return Err("Expected meta value".into()),
        };

        meta_entries.push(MetaEntryNode { key, value });
    }

    Ok(meta_entries)
}
