//! Lexing of the contents of a YARA hex string.
//!
//! This module tokenizes the *interior* of a hex string - the raw text
//! already captured between an unescaped `{` and `}` by the main
//! [`crate::parser::lexer::tokenize`] - turning it into a stream of
//! [`HexToken`]s consumed by
//! [`crate::parser::ast_parser::hex::parse_hex_string`].
//!
//! Hex strings are not part of the main YARA grammar. They are a small,
//! self-contained grammar of their own (bytes, wildcards, jumps, and
//! alternations), which is why they get a dedicated lexer rather than
//! being folded into the main token stream.

use std::{iter::Peekable, str::Chars};

use crate::parser::token::{
    HexToken,
    HexTokenType::{self, Unknown},
};

/// Stateful lexer used to tokenize the contents of a YARA hex string.
///
/// Unlike [`crate::parser::lexer::Lexer`], no position is tracked. Hex
/// string findings are reported at the granularity of the whole string,
/// not the individual atom, so no [`crate::parser::span::Span`] is
/// produced here.
struct HexLexer<'a> {
    /// Remaining characters of the hex string's contents.
    chars: Peekable<Chars<'a>>,
}

impl<'a> HexLexer<'a> {
    /// Creates a new hex lexer over the provided hex string contents.
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
        }
    }

    /// Returns the next character without consuming it.
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Consumes and returns the next character.
    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }
}

/// Returns whether `ch` may appear as one half of a byte, wildcard, or
/// nibble-wildcard atom (a hex digit or `?`).
fn is_hex_atom_char(ch: char) -> bool {
    ch == '?' || ch.is_ascii_hexdigit()
}

/// Parses a single hex nibble character.
///
/// # Returns
///
/// `Ok(Some(value))` for a hex digit, `Ok(None)` for a wildcard (`?`).
///
/// # Errors
///
/// Returns an error if `ch` is neither a valid hex digit nor `?`.
fn parse_nibble(ch: char) -> Result<Option<u8>, String> {
    match ch {
        '?' => Ok(None),
        c if c.is_ascii_hexdigit() => Ok(Some(c.to_digit(16).expect("validated hex digit") as u8)),
        other => Err(format!("Invalid hex digit: '{}'", other)),
    }
}

/// Converts the raw contents of a YARA hex string into a sequence of
/// [`HexToken`]s.
///
/// Digits are only ever interpreted as jump bounds while lexing inside
/// a `[...]` jump - everywhere else, a digit (or `?`) is the start of a
/// byte, wildcard, or nibble-wildcard atom. This context, not the
/// character class alone, is what disambiguates `42` the byte from `42`
/// the jump bound.
///
/// # Arguments
///
/// * `source` (`&str`) - the raw text between a hex string's braces,
///   e.g. `4D 5A ?? [4-6] ( AA | BB )`.
///
/// # Returns
///
/// Returns the sequence of [`HexToken`]s found in `source`.
///
/// # Errors
///
/// Returns an error if:
/// - A byte or nibble-wildcard atom is missing its second character.
/// - A character is not a valid hex digit, `?`, or structural symbol.
/// - A jump bound contains a number too large to fit a `u32`.
/// - A `[` is never closed with a matching `]`.
pub(crate) fn tokenize_hex(source: &str) -> Result<Vec<HexToken>, String> {
    let mut lexer = HexLexer::new(source);
    let mut tokens = Vec::new();
    let mut in_jump = false;

    while let Some(ch) = lexer.advance() {
        match ch {
            c if c.is_whitespace() => {}

            '[' => {
                in_jump = true;
                tokens.push(HexToken {
                    token_type: HexTokenType::LBracket,
                });
            }

            ']' => {
                in_jump = false;
                tokens.push(HexToken {
                    token_type: HexTokenType::RBracket,
                });
            }

            '-' => tokens.push(HexToken {
                token_type: HexTokenType::Dash,
            }),

            '(' => tokens.push(HexToken {
                token_type: HexTokenType::LParen,
            }),

            ')' => tokens.push(HexToken {
                token_type: HexTokenType::RParen,
            }),

            '|' => tokens.push(HexToken {
                token_type: HexTokenType::Pipe,
            }),

            // Inside `[...]`, digits form a jump bound (e.g. the `4` in
            // `[4-6]`), not a byte nibble.
            c if in_jump && c.is_ascii_digit() => {
                let mut num = String::from(c);

                while let Some(next) = lexer.peek() {
                    if next.is_ascii_digit() {
                        num.push(next);
                        lexer.advance();
                    } else {
                        break;
                    }
                }

                let value = num
                    .parse::<u32>()
                    .map_err(|e| format!("Invalid jump bound '{}': {}", num, e))?;

                tokens.push(HexToken {
                    token_type: HexTokenType::Number(value),
                });
            }

            // Outside a jump, a hex digit or `?` is the first half of a
            // byte/wildcard/nibble-wildcard atom - always consumed in pairs.
            c if !in_jump && is_hex_atom_char(c) => {
                let high = parse_nibble(c)?;

                let second = lexer
                    .advance()
                    .ok_or_else(|| "Unexpected EOF inside hex byte".to_string())?;

                if !is_hex_atom_char(second) {
                    return Err(format!(
                        "Invalid second character in hex byte: '{}'",
                        second
                    ));
                }

                let low = parse_nibble(second)?;

                let token_type = match (high, low) {
                    (Some(h), Some(l)) => HexTokenType::Byte((h << 4) | l),
                    (None, None) => HexTokenType::Wildcard,
                    (high, low) => HexTokenType::NibbleWildcard { high, low },
                };

                tokens.push(HexToken { token_type });
            }

            other => tokens.push(HexToken {
                token_type: Unknown(other),
            }),
        }
    }

    if in_jump {
        return Err("Unterminated jump in hex string: missing ']'".to_string());
    }

    Ok(tokens)
}
