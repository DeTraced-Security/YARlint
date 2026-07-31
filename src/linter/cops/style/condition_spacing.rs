//! Condition operator spacing checks.
//!
//! This module implements the style cop that validates spacing around
//! operators in YARA condition blocks using lexer token spans.

use crate::{
    linter::{
        context::LintContext,
        cop::{Category, Cop},
        finding::{Finding, Severity},
    },
    parser::token::{Token, TokenType},
};

/// Validates spacing around operators in condition blocks.
///
/// The cop consumes the token stream produced during parsing and compares
/// adjacent token spans to require one character of separation around each
/// supported operator.
pub struct StyleConditionSpacing {
    /// Tokens produced by the lexer for the current YARA file.
    tokens: Vec<Token>,
}

impl StyleConditionSpacing {
    /// Creates a condition spacing cop for a token stream.
    ///
    /// # Arguments
    ///
    /// * `tokens` (`Vec<Token>`) - Tokens produced while parsing the file
    ///
    /// # Returns
    ///
    /// Returns a [`StyleConditionSpacing`] ready to inspect the tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

impl Cop for StyleConditionSpacing {
    /// Returns the name of the cop.
    ///
    /// # Returns
    ///
    /// Returns `"ConditionSpacing"`.
    fn name(&self) -> &'static str {
        "ConditionSpacing"
    }

    /// Returns the category of the cop.
    ///
    /// # Returns
    ///
    /// Returns [`Category::Style`].
    fn category(&self) -> Category {
        Category::Style
    }

    /// Checks operator spacing within condition blocks.
    ///
    /// A warning is emitted once for each operator whose adjacent token spans
    /// do not indicate exactly one character of separation.
    ///
    /// # Arguments
    ///
    /// * `_context` (`&LintContext`) - Parsed file associated with the tokens
    /// * `findings` (`&mut Vec<Finding>`) - Collection receiving violations
    fn check(&self, _context: &LintContext, findings: &mut Vec<Finding>) {
        for operator in find_spacing_offenses(&self.tokens) {
            findings.push(Finding {
                rule: self.name(),
                category: self.category(),
                message: format!("Operator '{operator}' does not have single spacing"),
                severity: Severity::Warning,
            });
        }
    }
}

/// Finds condition operators with invalid spacing.
///
/// Only tokens between a `condition:` section marker and the rule's closing
/// brace are inspected.
///
/// # Arguments
///
/// * `tokens` (`&[Token]`) - Tokens produced while parsing the file
///
/// # Returns
///
/// Returns the display name of each operator with invalid spacing.
fn find_spacing_offenses(tokens: &[Token]) -> Vec<String> {
    let mut in_condition = false;

    tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            if !in_condition {
                if matches!(&token.token_type, TokenType::Keyword(keyword) if keyword == "condition")
                    && matches!(
                        tokens.get(index + 1).map(|token| &token.token_type),
                        Some(TokenType::Colon)
                    )
                {
                    in_condition = true;
                }
                return None;
            }

            if token.token_type == TokenType::RBrace {
                in_condition = false;
                return None;
            }

            let (operator, requires_leading_space) = operator(token)?;
            let leading_is_valid = !requires_leading_space
                || index
                    .checked_sub(1)
                    .and_then(|previous| tokens.get(previous))
                    .is_some_and(|previous| has_single_space_between(previous, token));
            let trailing_is_valid = tokens
                .get(index + 1)
                .is_some_and(|next| has_single_space_between(token, next));

            (!leading_is_valid || !trailing_is_valid).then(|| operator.to_string())
        })
        .collect()
}

/// Identifies supported condition operators.
///
/// The boolean in the returned pair indicates whether the operator requires
/// separation from a preceding token. Prefix operators only require trailing
/// separation.
///
/// # Arguments
///
/// * `token` (`&Token`) - Token to classify
///
/// # Returns
///
/// Returns the operator name and leading-spacing requirement, or `None` for a
/// non-operator token.
fn operator(token: &Token) -> Option<(&str, bool)> {
    match &token.token_type {
        TokenType::Keyword(keyword) => match keyword.as_str() {
            "not" | "defined" => Some((keyword, false)),
            "and" | "or" | "at" | "in" | "of" | "contains" | "icontains" | "startswith"
            | "istartswith" | "endswith" | "iendswith" | "iequals" | "matches" => {
                Some((keyword, true))
            }
            _ => None,
        },
        TokenType::EqualsEquals => Some(("==", true)),
        TokenType::NotEquals => Some(("!=", true)),
        TokenType::GThan => Some((">", true)),
        TokenType::GEThan => Some((">=", true)),
        TokenType::LThan => Some(("<", true)),
        TokenType::LEThan => Some(("<=", true)),
        _ => None,
    }
}

/// Determines whether two tokens have one character between them.
///
/// Tokens on different lines never have single spacing. For tokens on the
/// same line, the check compares the right token's starting column with the
/// left token's ending column.
///
/// # Arguments
///
/// * `left` (`&Token`) - Token before the gap
/// * `right` (`&Token`) - Token after the gap
///
/// # Returns
///
/// Returns `true` when the token spans indicate one separating character.
fn has_single_space_between(left: &Token, right: &Token) -> bool {
    if left.span.line != right.span.line {
        return false;
    }

    let left_end = left.span.column + token_width(&left.token_type);
    right.span.column.checked_sub(left_end) == Some(1)
}

/// Returns the source width of a token.
///
/// # Arguments
///
/// * `token_type` (`&TokenType`) - Token classification and associated value
///
/// # Returns
///
/// Returns the number of source characters occupied by the token.
fn token_width(token_type: &TokenType) -> usize {
    match token_type {
        TokenType::Identifier(value)
        | TokenType::StringIdentifier(value)
        | TokenType::Keyword(value)
        | TokenType::Number(value) => value.chars().count(),
        TokenType::StringLiteral(value) | TokenType::Regex(value) | TokenType::HexString(value) => {
            value.chars().count() + 2
        }
        TokenType::GEThan | TokenType::LEThan | TokenType::EqualsEquals | TokenType::NotEquals => 2,
        _ => 1,
    }
}
