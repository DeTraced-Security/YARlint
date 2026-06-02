pub mod condition;
pub mod expr;
pub mod meta;
pub mod rule;
pub mod strings;

use crate::parser::{
    ast_parser::{condition::parse_condition, meta::parse_meta, strings::parse_strings},
    syntax::{self, ConditionNode, ExprNode, MetaEntryNode, StringNode, rule::RuleNode},
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
    let mut condition: Vec<ConditionNode> = Vec::new();
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
        parser.expect(&TokenType::Colon);
        condition = parse_condition(&mut parser)?;
    }

    parser.expect(&TokenType::RBrace)?;

    Ok(RuleNode {
        name,
        is_global: false,
        is_private: false,
        tags: Vec::new(),
        meta: meta,
        strings: strings,
        condition: syntax::condition::ConditionNode {
            expression: ExprNode::Identifier(String::new()),
        },
    })
}
