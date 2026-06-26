//! Recursive-descent parsing of YARA hex string contents.
//!
//! Hex strings are not part of the main YARA grammar - they are a small,
//! self-contained grammar of their own (bytes, wildcards, jumps, and
//! alternations). [`HexAstParser`] is therefore a fully separate parser
//! from [`crate::parser::ast_parser::AstParser`], with its own cursor
//! over its own token type, rather than sharing one with the main
//! grammar.

use crate::parser::{
    lexer::hex::tokenize_hex,
    syntax::hex::{HexAtom, HexExprNode, HexNode},
    token::{HexToken, HexTokenType},
};

/// Recursive-descent parser for the contents of a YARA hex string.
///
/// Consumes a stream of [`HexToken`]s produced by [`tokenize_hex`] and
/// produces a sequence of [`HexAtom`]s.
pub(crate) struct HexAstParser {
    /// Complete token stream produced by the hex lexer.
    tokens: Vec<HexToken>,

    /// Current parser position.
    pos: usize,
}

impl HexAstParser {
    /// Creates a new hex parser instance positioned at the start of the
    /// token stream.
    pub(crate) fn new(tokens: Vec<HexToken>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Returns the current token without consuming it.
    pub(crate) fn peek(&self) -> Option<&HexToken> {
        self.tokens.get(self.pos)
    }

    /// Checks whether the current token matches the provided token type.
    fn check(&self, token_type: &HexTokenType) -> bool {
        matches!(
            self.peek(),
            Some(HexToken { token_type: t }) if t == token_type
        )
    }

    /// Consumes and returns the current token.
    fn advance(&mut self) -> Option<&HexToken> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    /// Consumes a token and verifies it matches the expected type.
    fn expect(&mut self, expected: &HexTokenType) -> Result<(), String> {
        match self.peek() {
            Some(t) if &t.token_type == expected => {
                self.advance();
                Ok(())
            }
            Some(t) => Err(format!("Expected {:?}, found {:?}", expected, t.token_type)),
            None => Err("Unexpected EOF in hex string".into()),
        }
    }

    /// Consumes a jump bound number.
    fn expect_number(&mut self) -> Result<u32, String> {
        match self.advance() {
            Some(HexToken {
                token_type: HexTokenType::Number(n),
            }) => Ok(*n),
            Some(token) => Err(format!("Expected jump bound, found {:?}", token.token_type)),
            None => Err("Unexpected EOF in hex string".into()),
        }
    }

    /// Parses a sequence of hex atoms.
    ///
    /// Stops without consuming the terminator when it encounters a
    /// [`HexTokenType::Pipe`], a [`HexTokenType::RParen`], or the end of
    /// the token stream - the caller decides what either means. This is
    /// what lets a single function parse both the top-level hex string
    /// and every branch of an alternation.
    pub(crate) fn parse_sequence(&mut self) -> Result<Vec<HexAtom>, String> {
        let mut atoms = Vec::new();

        loop {
            match self.peek().map(|t| &t.token_type) {
                None | Some(HexTokenType::Pipe) | Some(HexTokenType::RParen) => break,

                Some(HexTokenType::Byte(byte)) => {
                    let byte = *byte;
                    self.advance();
                    atoms.push(HexAtom::Byte(byte));
                }

                Some(HexTokenType::Wildcard) => {
                    self.advance();
                    atoms.push(HexAtom::Wildcard);
                }

                Some(HexTokenType::NibbleWildcard { high, low }) => {
                    let high = *high;
                    let low = *low;
                    self.advance();
                    atoms.push(HexAtom::NibbleWildcard { high, low });
                }

                Some(HexTokenType::LBracket) => {
                    let jump = self.parse_jump()?;
                    atoms.push(jump);
                }

                Some(HexTokenType::LParen) => {
                    let alternation = self.parse_alternation()?;
                    atoms.push(alternation);
                }

                Some(other) => {
                    return Err(format!("Unexpected token in hex string: {:?}", other));
                }
            }
        }

        Ok(atoms)
    }

    /// Parses a jump, e.g. `[4-6]`, `[4]`, `[4-]`, `[-]`.
    fn parse_jump(&mut self) -> Result<HexAtom, String> {
        self.expect(&HexTokenType::LBracket)?;

        let min = if self.check(&HexTokenType::Dash) {
            None
        } else {
            Some(self.expect_number()?)
        };

        let max = if self.check(&HexTokenType::Dash) {
            self.advance();

            if self.check(&HexTokenType::RBracket) {
                None
            } else {
                Some(self.expect_number()?)
            }
        } else {
            // No dash present: an exact jump, e.g. `[4]`. min and max match.
            min
        };

        self.expect(&HexTokenType::RBracket)?;

        Ok(HexAtom::Jump { min, max })
    }

    /// Parses an alternation, e.g. `( AA | BB CC )`.
    fn parse_alternation(&mut self) -> Result<HexAtom, String> {
        self.expect(&HexTokenType::LParen)?;

        let mut branches = vec![self.parse_sequence()?];

        while self.check(&HexTokenType::Pipe) {
            self.advance();
            branches.push(self.parse_sequence()?);
        }

        self.expect(&HexTokenType::RParen)?;

        if branches.len() < 2 {
            return Err("Alternation must have at least two branches".into());
        }

        Ok(HexAtom::Alternation(branches))
    }
}

/// Parses the contents of a YARA hex string into a [`HexNode`].
///
/// # Errors
///
/// Returns an error if the hex string is malformed.
pub fn parse_hex_string(hex_string: &str) -> Result<HexNode, String> {
    let hex_tokens = tokenize_hex(hex_string)?;

    if hex_tokens.is_empty() {
        return Ok(HexNode {
            expression: HexExprNode { atoms: Vec::new() },
            original_string: hex_string.to_string(),
        });
    }

    let mut hex_parser = HexAstParser::new(hex_tokens);
    let atoms = hex_parser.parse_sequence()?;

    if let Some(token) = hex_parser.peek() {
        return Err(format!(
            "Unexpected trailing token in hex string: {:?}",
            token.token_type
        ));
    }

    Ok(HexNode {
        expression: HexExprNode { atoms },
        original_string: hex_string.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- expect ---

    #[test]
    fn expect_wrong_token_type_returns_err() {
        let tokens = vec![HexToken {
            token_type: HexTokenType::Pipe,
        }];
        let mut parser = HexAstParser::new(tokens);

        let result = parser.expect(&HexTokenType::LBracket);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Expected LBracket, found Pipe"
        );
    }

    #[test]
    fn expect_eof_returns_err() {
        let mut parser = HexAstParser::new(Vec::new());

        let result = parser.expect(&HexTokenType::LBracket);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unexpected EOF in hex string");
    }

    // --- expect_number ---

    #[test]
    fn expect_number_wrong_token_type_returns_err() {
        let tokens = vec![HexToken {
            token_type: HexTokenType::Dash,
        }];
        let mut parser = HexAstParser::new(tokens);

        let result = parser.expect_number();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected jump bound, found Dash");
    }

    #[test]
    fn expect_number_eof_returns_err() {
        let mut parser = HexAstParser::new(Vec::new());

        let result = parser.expect_number();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unexpected EOF in hex string");
    }

    // --- parse_sequence ---

    #[test]
    fn parse_sequence_unexpected_token_returns_err() {
        // Dash only makes sense inside a jump; at the top of a sequence
        // it's not a valid atom starter.
        let tokens = vec![HexToken {
            token_type: HexTokenType::Dash,
        }];
        let mut parser = HexAstParser::new(tokens);

        let result = parser.parse_sequence();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unexpected token in hex string: Dash");
    }

    // --- parse_alternation ---

    #[test]
    fn parse_alternation_single_branch_returns_err() {
        let tokens = vec![
            HexToken {
                token_type: HexTokenType::LParen,
            },
            HexToken {
                token_type: HexTokenType::Byte(0xAA),
            },
            HexToken {
                token_type: HexTokenType::RParen,
            },
        ];
        let mut parser = HexAstParser::new(tokens);

        let result = parser.parse_alternation();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Alternation must have at least two branches"
        );
    }

    // --- parse_hex_string ---

    #[test]
    fn parse_hex_string_empty_returns_empty_node() {
        let result = parse_hex_string("");

        assert!(result.is_ok());
        let node = result.unwrap();
        assert!(node.expression.atoms.is_empty());
        assert_eq!(node.original_string, "");
    }

    #[test]
    fn parse_hex_string_single_byte_succeeds() {
        let result = parse_hex_string("aa");

        assert!(result.is_ok());
        let node = result.unwrap();
        assert_eq!(node.expression.atoms, vec![HexAtom::Byte(0xAA)]);
        assert_eq!(node.original_string, "aa");
    }

    #[test]
    fn parse_hex_string_trailing_token_returns_err() {
        // A valid byte sequence followed by an unconsumed RParen with no
        // matching LParen, parse_sequence stops cleanly at the byte,
        // leaving the RParen as an unexpected trailing token.
        let result = parse_hex_string("aa )");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Unexpected trailing token in hex string: RParen"
        );
    }
}