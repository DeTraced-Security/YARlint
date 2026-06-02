use crate::parser::{
    ast_parser::AstParser,
    syntax::{BinaryOperator, ConditionNode, ExprNode},
    token::{Token, TokenType},
};

pub fn parse_condition(parser: &mut AstParser) -> Result<ConditionNode, String> {
    let expression = parse_expr(parser)?;

    Ok(ConditionNode { expression })
}

pub fn parse_expr(parser: &mut AstParser) -> Result<ExprNode, String> {
    parse_or(parser)
}

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

fn parse_comparison(parser: &mut AstParser) -> Result<ExprNode, String> {
    let mut left = parse_primary(parser)?;

    loop {
        let op = match parser.peek() {
            Some(tok) => match &tok.token_type {
                TokenType::EqualsEquals => BinaryOperator::Equals,
                TokenType::LThan => BinaryOperator::LessThan,
                TokenType::GThan => BinaryOperator::GreaterThan,
                TokenType::LEThan => BinaryOperator::LessThanEqual,
                TokenType::GEThan => BinaryOperator::GreaterThanEqual,
                _ => break,
            },
            None => break,
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

fn parse_primary(parser: &mut AstParser) -> Result<ExprNode, String> {
    match parser.peek() {
        Some(Token {
            token_type: TokenType::Number(_),
            ..
        }) => {
            let num = parser.expect_number()?;
            Ok(ExprNode::Number(num))
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
            parser.advance();
            parser.expect_keyword("of")?;
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
