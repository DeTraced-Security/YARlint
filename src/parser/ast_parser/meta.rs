use crate::parser::{
    ast_parser::AstParser,
    syntax::{MetaEntryNode, MetaValue},
    token::{Token, TokenType},
};

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
        meta_entries.push(MetaEntryNode {
            key: (key),
            value: (value),
        });
    }
    Ok(meta_entries)
}
