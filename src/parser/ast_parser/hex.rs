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
