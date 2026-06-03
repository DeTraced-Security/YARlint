//! Parsing of YARA condition blocks.
//!
//! This module converts tokens from a `condition:` section into
//! expression trees represented by [`ConditionNode`] and [`ExprNode`].
//! It implements operator precedence and grouping semantics used by
//! YARA condition expressions.

use crate::parser::{
    ast_parser::AstParser,
    syntax::{BinaryOperator, ConditionNode, ExprNode},
    token::{Token, TokenType},
};

/// Parses a YARA condition block into a condition AST node.
///
/// This function serves as the entry point for condition parsing.
/// It parses the root expression contained within the condition
/// section and wraps it in a [`ConditionNode`].
///
/// # Arguments
///
/// * `parser` - The active parser positioned at the beginning of a
///   condition expression.
///
/// # Returns
///
/// Returns a populated [`ConditionNode`] on success.
///
/// # Errors
///
/// Returns an error if the condition expression contains invalid
/// syntax or ends unexpectedly.
pub fn parse_condition(parser: &mut AstParser) -> Result<ConditionNode, String> {
    let expression = parse_expr(parser)?;

    Ok(ConditionNode { expression })
}

/// Parses a YARA expression.
///
/// This function is the entry point for expression parsing and begins
/// parsing at the lowest-precedence operator level. More specific
/// parsing functions are responsible for handling operator precedence
/// and constructing the resulting expression tree.
///
/// # Arguments
///
/// * `parser` - The active parser positioned at the start of an
///   expression.
///
/// # Returns
///
/// Returns the root [`ExprNode`] of the parsed expression tree.
///
/// # Errors
///
/// Returns an error if the expression contains invalid syntax or
/// cannot be parsed.
pub fn parse_expr(parser: &mut AstParser) -> Result<ExprNode, String> {
    parse_or(parser)
}

///// Parses an `all of ($pattern*)` expression.
/////
///// Example:
/////
///// ```yara
///// all of ($x*)
///// ```
/////
///// Produces:
/////
///// ```rust
///// ExprNode::AllOf {
/////     pattern: "$x".to_string()
///// }
///// ```
//fn parse_all_of(parser: &mut AstParser) -> Result<ExprNode, String> {
//    parser.expect_keyword("all")?;
//    parser.expect_keyword("of")?;
//
//    parser.expect(&TokenType::LParen)?;
//
//    let pattern = parser.expect_string_identifier()?;
//
//    parser.expect(&TokenType::Star)?;
//
//    parser.expect(&TokenType::RParen)?;
//
//    Ok(ExprNode::AllOf { pattern })
//}

/// Parses a logical OR expression.
///
/// OR has the lowest precedence level.
///
/// Example:
///
/// ```yara
/// $a or $b
/// ```
///
/// Produces:
///
/// ```text
/// Binary(Or)
/// ├── $a
/// └── $b
/// ```
fn parse_or(parser: &mut AstParser) -> Result<ExprNode, String> {
    let mut left = parse_and(parser)?;

    while parser.peek_keyword("or") {
        parser.advance();

        let right = parse_and(parser)?;

        left = ExprNode::Binary {
            left: Box::new(left),
            operator: BinaryOperator::Or,
            right: Box::new(right),
        };
    }

    Ok(left)
}

/// Parses a logical AND expression.
///
/// AND has higher precedence than OR.
///
/// Example:
///
/// ```yara
/// $a and $b
/// ```
///
/// Produces:
///
/// ```text
/// Binary(And)
/// ├── $a
/// └── $b
/// ```
fn parse_and(parser: &mut AstParser) -> Result<ExprNode, String> {
    let mut left = parse_comparison(parser)?;

    while parser.peek_keyword("and") {
        parser.advance();

        let right = parse_comparison(parser)?;

        left = ExprNode::Binary {
            left: Box::new(left),
            operator: BinaryOperator::And,
            right: Box::new(right),
        };
    }

    Ok(left)
}

/// Parses comparison operations.
///
/// Supported operators:
///
/// ```yara
/// ==
/// <
/// >
/// ```
///
/// Example:
///
/// ```yara
/// filesize < 100KB
/// ```
///
/// Produces:
///
/// ```text
/// Binary(LessThan)
/// ├── filesize
/// └── 100KB
/// ```
fn parse_comparison(parser: &mut AstParser) -> Result<ExprNode, String> {
    let mut left = parse_primary(parser)?;

    while let Some(tok) = parser.peek() {
        let op = match &tok.token_type {
            TokenType::EqualsEquals => BinaryOperator::Equals,
            TokenType::LThan => BinaryOperator::LessThan,
            TokenType::GThan => BinaryOperator::GreaterThan,
            TokenType::LEThan => BinaryOperator::LessThanEqual,
            TokenType::GEThan => BinaryOperator::GreaterThanEqual,
            _ => break,
        };

        parser.advance();

        let right = parse_primary(parser)?;

        left = ExprNode::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        };
    }

    Ok(left)
}

/// Parses a primary expression.
///
/// Primary expressions form the leaf nodes of the condition expression tree
/// and represent the most basic units that can appear in a YARA condition.
///
/// Supported expression types include:
///
/// - Numeric literals (`100`, `0x5A4D`, `500KB`)
/// - String literals (`"hello"`)
/// - String identifiers (`$a`, `$text`)
/// - Identifiers (`filesize`, `them`, `pe`)
/// - Function calls (`uint16(0)`, `pe.imphash()`)
/// - Parenthesized expressions (`(filesize < 1MB)`)
/// - Pattern matching constructs (`all of ($a*)`)
///
/// Parenthesized expressions are parsed recursively and returned as their
/// contained expression, allowing grouping and precedence handling at
/// higher parsing levels.
///
/// Returns an [`ExprNode`] representing the parsed primary expression,
/// or an error if the current token cannot begin a valid expression.
fn parse_primary(parser: &mut AstParser) -> Result<ExprNode, String> {
    match parser.peek() {
        Some(Token {
            token_type: TokenType::Number(_),
            ..
        }) => {
            let count = parser.expect_number()?;

            if parser.peek_keyword("of") {
                parser.advance();

                if parser.peek_keyword("them") {
                    parser.advance();

                    return Ok(ExprNode::Of {
                        count: Box::new(ExprNode::Number(count)),
                        pattern: "them".to_string(),
                    });
                }

                parser.expect(&TokenType::LParen)?;

                let pattern = parser.expect_string_identifier()?;

                parser.expect(&TokenType::Star)?;
                parser.expect(&TokenType::RParen)?;

                return Ok(ExprNode::Of {
                    count: Box::new(ExprNode::Number(count)),
                    pattern,
                });
            }

            Ok(ExprNode::Number(count))
        }

        Some(Token {
            token_type: TokenType::StringLiteral(_),
            ..
        }) => {
            let s = parser.expect_string_literal()?;
            Ok(ExprNode::StringLiteral(s))
        }
        Some(Token {
            token_type: TokenType::StringIdentifier(_),
            ..
        }) => {
            let ident = parser.expect_string_identifier()?;
            Ok(ExprNode::Identifier(ident))
        }

        Some(Token {
            token_type: TokenType::Identifier(_),
            ..
        }) => {
            let ident = parser.expect_identifier()?;

            if parser.check(&TokenType::LParen) {
                parser.advance();

                let mut args = Vec::new();

                while !parser.check(&TokenType::RParen) {
                    args.push(parse_expr(parser)?);

                    if parser.check(&TokenType::Comma) {
                        parser.advance();
                    }
                }

                parser.expect(&TokenType::RParen)?;

                Ok(ExprNode::FunctionCall {
                    name: ident,
                    arguments: args,
                })
            } else {
                Ok(ExprNode::Identifier(ident))
            }
        }

        Some(Token {
            token_type: TokenType::LParen,
            ..
        }) => {
            parser.advance();
            let expr = parse_expr(parser)?;
            parser.expect(&TokenType::RParen)?;
            Ok(expr)
        }

        Some(Token {
            token_type: TokenType::Keyword(k),
            ..
        }) if k == "all" => {
            parser.expect_keyword("all")?;
            parser.expect_keyword("of")?;

            if parser.peek_keyword("them") {
                parser.advance();

                return Ok(ExprNode::AllOfThem);
            }
            parser.expect(&TokenType::LParen)?;

            let pattern = parser.expect_string_identifier()?;

            parser.expect(&TokenType::Star)?;
            parser.expect(&TokenType::RParen)?;

            Ok(ExprNode::AllOf { pattern })
        }

        Some(tok) => Err(format!("Unexpected token {:?}", tok.token_type)),

        None => Err("Unexpected EOF".into()),
    }
}
