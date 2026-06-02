use crate::parser::{
    ast_parser::AstParser,
    syntax::{StringModifier, StringNode},
    token::TokenType,
};

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
