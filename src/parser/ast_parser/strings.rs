//! Parsing of YARA string definitions.
//!
//! This module parses string declarations and their associated
//! modifiers, producing [`StringNode`] values suitable for AST
//! construction and later analysis.

use crate::parser::{
    ast_parser::AstParser,
    syntax::{StringModifier, StringNode},
    token::TokenType,
};

/// Parses the `strings` section of a YARA rule.
///
/// This function consumes string declarations until the beginning of
/// the `condition` section is encountered. Each string declaration
/// consists of a string identifier, an equals sign, a string literal,
/// and zero or more string modifiers.
///
/// Supported modifiers include:
/// - `wide`
/// - `ascii`
/// - `fullword`
/// - `nocase`
/// - `xor`
/// - `base64`
/// - `base64wide`
///
/// # Arguments
///
/// * `parser` - The active parser positioned at the first string
///   declaration following the `strings:` section header.
///
/// # Returns
///
/// Returns a vector containing all parsed [`StringNode`] values.
///
/// # Errors
///
/// Returns an error if:
/// - A string identifier is missing or malformed.
/// - An equals sign is missing.
/// - A string literal is missing or invalid.
/// - The input ends unexpectedly while parsing string declarations.
pub fn parse_strings(parser: &mut AstParser) -> Result<Vec<StringNode>, String> {
    let mut strings = Vec::new();

    while !parser.peek_keyword("condition") {
        let identifier = parser.expect_string_identifier()?;

        parser.expect(&TokenType::Equals)?;

        let value = parser.expect_string_literal()?;

        let mut modifiers = Vec::new();

        while let Some(token) = parser.peek() {
            match &token.token_type {
                TokenType::Keyword(k) if k == "wide" => {
                    parser.advance();
                    modifiers.push(StringModifier::Wide);
                }

                TokenType::Keyword(k) if k == "ascii" => {
                    parser.advance();
                    modifiers.push(StringModifier::Ascii);
                }

                TokenType::Keyword(k) if k == "fullword" => {
                    parser.advance();
                    modifiers.push(StringModifier::Fullword);
                }

                TokenType::Keyword(k) if k == "nocase" => {
                    parser.advance();
                    modifiers.push(StringModifier::Nocase);
                }

                TokenType::Keyword(k) if k == "xor" => {
                    parser.advance();
                    modifiers.push(StringModifier::Xor);
                }

                TokenType::Keyword(k) if k == "base64" => {
                    parser.advance();
                    modifiers.push(StringModifier::Base64);
                }

                TokenType::Keyword(k) if k == "base64wide" => {
                    parser.advance();
                    modifiers.push(StringModifier::Base64Wide);
                }

                _ => break,
            }
        }

        strings.push(StringNode {
            identifier,
            value,
            modifiers,
        });
    }

    Ok(strings)
}
