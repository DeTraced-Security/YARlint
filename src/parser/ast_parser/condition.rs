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
/// Returns an error if:
/// - the condition expression contains invalid syntax
/// - the condition expression ends unexpectedly
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
/// Returns an error if:
/// - the expression contains invalid syntax
/// - the expression cannot be parsed
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
///
/// # Arguments
///
/// * `parser` - an `AstParser` positioned at the start of a logical
///   OR expression
///
/// # Returns
///
/// Returns the root [`ExprNode`] of the parsed OR expression
///
/// # Errors
///
/// Returns an error if:
/// - the expression contains invalid syntax
/// - the expression cannot be parsed
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
///
/// # Arguments
///
/// * `parser` - an `AstParser` positioned at the start of a logical
///   AND expression
///
/// # Returns
///
/// Returns the root [`ExprNode`] of the parsed AND expression
///
/// # Errors
///
/// Returns an error if:
/// - the expression contains invalid syntax
/// - the expression cannot be parsed
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
///
/// # Arguments
///
/// * `parser` - an `AstParser` positioned at the start of a
///   comparison expression
///
/// # Returns
///
/// Returns the root [`ExprNode`] of the parsed comparison expression
///
/// # Errors
///
/// Returns an error if:
/// - the expression contains invalid syntax
/// - the expression cannot be parsed
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
/// # Arguments
///
/// * `parser` - an `AstParser` positioned at the start of a primary
///   expression
///
/// # Returns
///
/// Returns an [`ExprNode`] representing the parsed primary expression,
/// or an error if the current token cannot begin a valid expression.
///
/// # Errors
///
/// Returns an error if:
/// - the expression contains invalid syntax
/// - the expression cannot be parsed
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

            if parser.check(&TokenType::Dot) {
                parser.expect(&TokenType::Dot)?;

                let function = parser.expect_identifier()?;

                parser.expect(&TokenType::LParen)?;

                let mut args = Vec::new();

                while !parser.check(&TokenType::RParen) {
                    args.push(parse_expr(parser)?);

                    if parser.check(&TokenType::Comma) {
                        parser.advance();
                    }
                }

                parser.expect(&TokenType::RParen)?;

                return Ok(ExprNode::ModuleFunction {
                    module: ident,
                    function,
                    arguments: args,
                });
            }

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

                return Ok(ExprNode::FunctionCall {
                    name: ident,
                    arguments: args,
                });
            }

            Ok(ExprNode::Identifier(ident))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{
        ast_parser::AstParser,
        lexer::tokenize,
        syntax::{BinaryOperator, ExprNode},
    };

    fn parse_expr_from(source: &str) -> Result<ExprNode, String> {
        let tokens = tokenize(source).unwrap();
        let mut parser = AstParser::new(tokens);
        parse_expr(&mut parser)
    }

    // --- parse_condition ---

    #[test]
    fn parse_condition_returns_err_on_empty_input() {
        let tokens = vec![];
        let mut parser = AstParser::new(tokens);

        assert!(parse_condition(&mut parser).is_err());
    }

    // --- parse_expr / parse_or ---

    #[test]
    fn parse_expr_parses_single_identifier() {
        let result = parse_expr_from("filesize").unwrap();

        assert!(matches!(result, ExprNode::Identifier(s) if s == "filesize"));
    }

    #[test]
    fn parse_expr_parses_or_expression() {
        let result = parse_expr_from("$a or $b").unwrap();

        assert!(matches!(
            result,
            ExprNode::Binary {
                operator: BinaryOperator::Or,
                ..
            }
        ));
    }

    #[test]
    fn parse_expr_parses_chained_or_expressions() {
        // $a or $b or $c should produce a left-associative tree
        let result = parse_expr_from("$a or $b or $c").unwrap();

        assert!(matches!(
            result,
            ExprNode::Binary {
                operator: BinaryOperator::Or,
                ..
            }
        ));
    }

    // --- parse_and ---

    #[test]
    fn parse_expr_parses_and_expression() {
        let result = parse_expr_from("$a and $b").unwrap();

        assert!(matches!(
            result,
            ExprNode::Binary {
                operator: BinaryOperator::And,
                ..
            }
        ));
    }

    #[test]
    fn and_has_higher_precedence_than_or() {
        let result = parse_expr_from("$a or $b and $c").unwrap();

        let ExprNode::Binary {
            operator, right, ..
        } = result
        else {
            // do not worry about llvm-cov
            panic!("expected binary expression");
        };

        assert_eq!(operator, BinaryOperator::Or);
        assert!(matches!(
            *right,
            ExprNode::Binary {
                operator: BinaryOperator::And,
                ..
            }
        ));
    }

    // --- parse_comparison ---

    #[test]
    fn parse_expr_parses_equals_equals() {
        let result = parse_expr_from("filesize == 100").unwrap();

        assert!(matches!(
            result,
            ExprNode::Binary {
                operator: BinaryOperator::Equals,
                ..
            }
        ));
    }

    #[test]
    fn parse_expr_parses_less_than() {
        let result = parse_expr_from("filesize < 100").unwrap();

        assert!(matches!(
            result,
            ExprNode::Binary {
                operator: BinaryOperator::LessThan,
                ..
            }
        ));
    }

    #[test]
    fn parse_expr_parses_greater_than() {
        let result = parse_expr_from("filesize > 100").unwrap();

        assert!(matches!(
            result,
            ExprNode::Binary {
                operator: BinaryOperator::GreaterThan,
                ..
            }
        ));
    }

    #[test]
    fn parse_expr_parses_less_than_or_equal() {
        let result = parse_expr_from("filesize <= 100").unwrap();

        assert!(matches!(
            result,
            ExprNode::Binary {
                operator: BinaryOperator::LessThanEqual,
                ..
            }
        ));
    }

    #[test]
    fn parse_expr_parses_greater_than_or_equal() {
        let result = parse_expr_from("filesize >= 100").unwrap();

        assert!(matches!(
            result,
            ExprNode::Binary {
                operator: BinaryOperator::GreaterThanEqual,
                ..
            }
        ));
    }

    // --- parse_primary: numbers ---

    #[test]
    fn parse_primary_parses_number_literal() {
        let result = parse_expr_from("42").unwrap();

        assert!(matches!(result, ExprNode::Number(n) if n == "42"));
    }

    #[test]
    fn parse_primary_parses_number_of_them() {
        let result = parse_expr_from("2 of them").unwrap();

        assert!(matches!(
            result,
            ExprNode::Of {
                pattern,
                ..
            } if pattern == "them"
        ));
    }

    #[test]
    fn parse_primary_parses_number_of_pattern() {
        let result = parse_expr_from("2 of ($s*)").unwrap();

        assert!(matches!(
            result,
            ExprNode::Of { pattern, .. } if pattern == "$s"
        ));
    }

    // --- parse_primary: string literals ---

    #[test]
    fn parse_primary_parses_string_literal() {
        let result = parse_expr_from(r#""cmd.exe""#).unwrap();

        assert!(matches!(result, ExprNode::StringLiteral(s) if s == "cmd.exe"));
    }

    // --- parse_primary: string identifiers ---

    #[test]
    fn parse_primary_parses_string_identifier() {
        let result = parse_expr_from("$foo").unwrap();

        assert!(matches!(result, ExprNode::Identifier(s) if s == "$foo"));
    }

    // --- parse_primary: identifiers and function calls ---

    #[test]
    fn parse_primary_parses_plain_identifier() {
        let result = parse_expr_from("filesize").unwrap();

        assert!(matches!(result, ExprNode::Identifier(s) if s == "filesize"));
    }

    #[test]
    fn parse_primary_parses_function_call_with_no_args() {
        let result = parse_expr_from("myfunc()").unwrap();

        assert!(matches!(
            result,
            ExprNode::FunctionCall { name, arguments } if name == "myfunc" && arguments.is_empty()
        ));
    }

    #[test]
    fn parse_primary_parses_function_call_with_one_arg() {
        let result = parse_expr_from("uint16(0)").unwrap();

        assert!(matches!(
            result,
            ExprNode::FunctionCall { name, arguments } if name == "uint16" && arguments.len() == 1
        ));
    }

    #[test]
    fn parse_primary_parses_module_function_call() {
        let result = parse_expr_from("pe.imphash()").unwrap();

        assert!(matches!(
            result,
            ExprNode::ModuleFunction { module, function, arguments }
                if module == "pe" && function == "imphash" && arguments.is_empty()
        ));
    }

    #[test]
    fn parse_primary_parses_module_function_call_with_args() {
        let result = parse_expr_from("pe.exports(\"WinMain\")").unwrap();

        assert!(matches!(
            result,
            ExprNode::ModuleFunction { module, function, arguments }
                if module == "pe" && function == "exports" && arguments.len() == 1
        ));
    }

    #[test]
    fn parse_primary_parses_function_call_with_multiple_args() {
        let result = parse_expr_from("myfunc(1, 2)").unwrap();
        assert!(matches!(
            result,
            ExprNode::FunctionCall { arguments, .. } if arguments.len() == 2
        ));
    }

    #[test]
    fn parse_primary_parses_module_function_with_multiple_args() {
        let result = parse_expr_from("pe.func(1, 2)").unwrap();
        assert!(matches!(
            result,
            ExprNode::ModuleFunction { arguments, .. } if arguments.len() == 2
        ));
    }

    // --- parse_primary: grouped expressions ---

    #[test]
    fn parse_primary_parses_parenthesized_expression() {
        let result = parse_expr_from("(filesize < 100)").unwrap();

        // grouping is transparent — returns inner expression directly
        assert!(matches!(
            result,
            ExprNode::Binary {
                operator: BinaryOperator::LessThan,
                ..
            }
        ));
    }

    #[test]
    fn parse_primary_parses_nested_parentheses() {
        let result = parse_expr_from("((filesize < 100))").unwrap();

        assert!(matches!(
            result,
            ExprNode::Binary {
                operator: BinaryOperator::LessThan,
                ..
            }
        ));
    }

    // --- parse_primary: all of ---

    #[test]
    fn parse_primary_parses_all_of_them() {
        let result = parse_expr_from("all of them").unwrap();

        assert!(matches!(result, ExprNode::AllOfThem));
    }

    #[test]
    fn parse_primary_parses_all_of_pattern() {
        let result = parse_expr_from("all of ($s*)").unwrap();

        assert!(matches!(result, ExprNode::AllOf { pattern } if pattern == "$s"));
    }

    // --- error cases ---

    #[test]
    fn parse_primary_returns_err_on_unexpected_token() {
        let result = parse_expr_from("}");

        assert!(result.is_err());
    }

    #[test]
    fn parse_primary_returns_err_on_empty_input() {
        let result = parse_expr_from("");

        assert!(result.is_err());
    }

    #[test]
    fn parse_primary_returns_err_on_unclosed_paren() {
        let result = parse_expr_from("(filesize < 100");

        assert!(result.is_err());
    }
}
