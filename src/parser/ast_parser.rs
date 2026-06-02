pub mod condition;
pub mod expr;
pub mod meta;
pub mod rule;
pub mod strings;

use crate::parser::{
    ast_parser::{condition::parse_condition, meta::parse_meta, strings::parse_strings},
    syntax::{
        self, BinaryOperator, ConditionNode, ExprNode, MetaEntryNode, StringNode, rule::RuleNode,
    },
    token::{Token, TokenType},
};

pub struct AstParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl AstParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        matches!(
            self.peek(),
            Some(Token { token_type: t, .. }) if t == token_type
        )
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_keyword(&self, keyword: &str) -> bool {
        matches!(
            self.peek(),
            Some(Token {
                token_type: TokenType::Keyword(k),
                ..
            }) if k == keyword
        )
    }

    pub fn peek_string_identifier(&self) -> bool {
        matches!(
            self.peek(),
            Some(Token {
                token_type: TokenType::StringIdentifier(_),
                ..
            })
        )
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &TokenType) -> Result<(), String> {
        match self.peek() {
            Some(t) if &t.token_type == expected => {
                self.advance();
                Ok(())
            }
            Some(t) => Err(format!("Expected {:?}, found {:?}", expected, t.token_type)),
            None => Err("Unexpected EOF".into()),
        }
    }

    fn expect_number(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token {
                token_type: TokenType::Number(num),
                ..
            }) => Ok(num.clone()),
            Some(token) => Err(format!("Expected Number, found {:?}", token.token_type)),

            None => Err("Unexpected EOF".into()),
        }
    }

    fn expect_boolean(&mut self) -> Result<bool, String> {
        match self.advance() {
            Some(Token {
                token_type: TokenType::Keyword(bool_val),
                ..
            }) => match bool_val.as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(format!("Expected boolean, found keyword '{}'", bool_val)),
            },

            Some(token) => Err(format!("Expected boolean, found {:?}", token.token_type)),

            None => Err("Unexpected EOF".into()),
        }
    }
    fn expect_identifier(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => Ok(name.clone()),

            Some(token) => Err(format!("Expected identifier, found {:?}", token.token_type)),

            None => Err("Unexpected EOF".into()),
        }
    }

    fn expect_string_literal(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token {
                token_type: TokenType::StringLiteral(name),
                ..
            }) => Ok(name.clone()),
            Some(token) => Err(format!(
                "Expected StringLiteral, found {:?}",
                token.token_type
            )),

            None => Err("Unexpected EOF".into()),
        }
    }

    fn expect_string_identifier(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token {
                token_type: TokenType::StringIdentifier(name),
                ..
            }) => Ok(name.clone()),

            Some(token) => Err(format!(
                "Expected string identifier, found {:?}",
                token.token_type
            )),

            None => Err("Unexpected EOF".into()),
        }
    }

    fn expect_keyword(&mut self, keyword: &str) -> Result<(), String> {
        match self.advance() {
            Some(Token {
                token_type: TokenType::Keyword(found),
                ..
            }) if found == keyword => Ok(()),

            Some(token) => Err(format!(
                "Expected keyword '{}', found {:?}",
                keyword, token.token_type
            )),

            None => Err("Unexpected EOF".into()),
        }
    }

    pub fn expect_any_keyword(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token {
                token_type: TokenType::Keyword(keyword),
                ..
            }) => Ok(keyword.clone()),

            Some(token) => Err(format!("Expected keyword, found {:?}", token.token_type)),

            None => Err("Unexpected EOF".into()),
        }
    }

    fn parse_or(parser: &mut AstParser) -> Result<ExprNode, String> {
        let mut left = Self::parse_and(parser)?;

        while parser.peek_keyword("or") {
            parser.advance();

            let right = Self::parse_and(parser)?;

            left = ExprNode::Binary {
                left: Box::new(left),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(parser: &mut AstParser) -> Result<ExprNode, String> {
        let mut left = Self::parse_comparison(parser)?;

        while parser.peek_keyword("and") {
            parser.advance();

            let right = Self::parse_comparison(parser)?;

            left = ExprNode::Binary {
                left: Box::new(left),
                operator: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(parser: &mut AstParser) -> Result<ExprNode, String> {
        if parser.peek_keyword("all") {
            return Self::parse_all_of(parser);
        }

        match parser.advance() {
            Some(Token {
                token_type: TokenType::StringIdentifier(id),
                ..
            }) => Ok(ExprNode::Identifier(id.clone())),

            _ => Err("Expected expression".into()),
        }
    }

    fn parse_all_of(parser: &mut AstParser) -> Result<ExprNode, String> {
        parser.expect_keyword("all")?;
        parser.expect_keyword("of")?;

        parser.expect(&TokenType::LParen)?;

        let pattern = parser.expect_string_identifier()?;

        parser.expect(&TokenType::Star)?;

        parser.expect(&TokenType::RParen)?;

        Ok(ExprNode::AllOf { pattern })
    }

    fn parse_comparison(parser: &mut AstParser) -> Result<ExprNode, String> {
        let mut left = Self::parse_primary(parser)?;

        loop {
            let op = match parser.peek() {
                Some(Token {
                    token_type: TokenType::EqualsEquals,
                    ..
                }) => BinaryOperator::Equals,

                Some(Token {
                    token_type: TokenType::LThan,
                    ..
                }) => BinaryOperator::LessThan,

                Some(Token {
                    token_type: TokenType::GThan,
                    ..
                }) => BinaryOperator::GreaterThan,

                _ => break,
            };

            parser.advance();

            let right = Self::parse_primary(parser)?;

            left = ExprNode::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
}

pub fn parse_rule(tokens: Vec<Token>) -> Result<RuleNode, String> {
    let mut parser = AstParser::new(tokens);

    parser.expect_keyword("rule")?;
    println!("matched rule keyword");

    let name = parser.expect_identifier()?;
    println!("Parsed rule name: {}", name);
    // TODO: tags

    parser.expect(&TokenType::LBrace)?;
    let mut meta: Vec<MetaEntryNode> = Vec::new();
    let mut strings: Vec<StringNode> = Vec::new();
    let mut condition: ConditionNode = ConditionNode {
        expression: ExprNode::Identifier(String::new()),
    };
    if parser.peek_keyword("meta") {
        parser.expect_keyword("meta")?;
        parser.expect(&TokenType::Colon)?;
        meta = parse_meta(&mut parser)?;
    }
    if parser.peek_keyword("strings") {
        parser.expect_keyword("strings")?;
        parser.expect(&TokenType::Colon)?;
        strings = parse_strings(&mut parser)?;
    }
    if parser.peek_keyword("condition") {
        parser.expect_keyword("condition")?;
        parser.expect(&TokenType::Colon)?;
        condition = condition::parse_condition(&mut parser)?;
    }

    parser.expect(&TokenType::RBrace)?;

    Ok(RuleNode {
        name,
        is_global: false,
        is_private: false,
        tags: Vec::new(),
        meta: meta,
        strings: strings,
        condition: condition,
    })
}
