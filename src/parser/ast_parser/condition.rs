use crate::parser::{ast_parser::AstParser, syntax::ConditionNode, token::TokenType};

pub fn parse_condition(parser: &mut AstParser) -> Result<Vec<ConditionNode>, String> {
    let mut conditions: Vec<ConditionNode> = Vec::new();
    while !parser.check(&TokenType::RBrace) {
        parser.advance();
    }
    Ok(conditions)
}
