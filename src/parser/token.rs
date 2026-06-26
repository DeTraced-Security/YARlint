//! Token implementation.
//!
//! This module implements the token for the YARA lexer.

use crate::parser::span::Span;

#[derive(PartialEq, Debug)]

/// A token produced by the lexer.
///
/// Stores the token's type and source location in a span.
pub struct Token {
    /// The token's classification and associated value.
    pub token_type: TokenType,

    /// The token's location via line and column number
    pub span: Span,
}

/// Represents a lexical token recognized by the YARA lexer.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    /// A standard identifier such as a rule name, metadata key,
    /// module name, or function name.
    Identifier(String),

    /// A YARA string identifier such as `$a` or `$filename`.
    StringIdentifier(String),

    /// A quoted string literal.
    StringLiteral(String),

    /// A regular expression.
    Regex(String),

    /// A String of hexadecimal bytes
    HexString(String),

    /// A reserved YARA keyword.
    Keyword(String),

    /// A numeric literal.
    ///
    /// This may represent decimal, hexadecimal, or size values
    /// until more specialized token types are introduced.
    Number(String),

    /// Greater-than operator (`>`).
    GThan,

    /// Greater-than-or-equal operator (`>=`).
    GEThan,

    /// Less-than operator (`<`).
    LThan,

    /// Less-than-or-equal operator (`<=`).
    LEThan,

    /// Assignment operator (`=`).
    Equals,

    /// Equality operator (`==`).
    EqualsEquals,

    /// Minus operator (`-`).
    Minus,

    /// Plus operator (`+`).
    Plus,

    /// Colon separator (`:`).
    Colon,

    /// Comma separator (`,`).
    Comma,

    /// Left brace (`{`).
    LBrace,

    /// Right brace (`}`).
    RBrace,

    /// Left parenthesis (`(`).
    LParen,

    /// Right parenthesis (`)`).
    RParen,

    /// Forward slash (`/`).
    FSlash,

    /// Wildcard operator (`*`).
    Star,

    /// Member access operator (`.`).
    Dot,

    /// At symbol (`@`).
    AtSymbol,

    /// An unrecognized character encountered during lexing.
    Unknown(char),
}

/// A token produced while lexing the contents of a YARA hex string.
///
/// Hex tokens are never mixed into the main [`Token`] stream - they are
/// produced by [`crate::parser::lexer::hex::tokenize_hex`] from the raw
/// text a [`TokenType::HexString`] token already captured, and consumed
/// only by [`crate::parser::ast_parser::hex::parse_hex_string`]. No
/// [`Span`] is attached: hex-string findings are reported at the
/// granularity of the whole string, not the individual atom, so there is
/// nothing for a per-token position to be used for today.
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct HexToken {
    /// The token's classification and associated value.
    pub(crate) token_type: HexTokenType,
}

/// Represents a lexical token recognized within a YARA hex string.
#[derive(PartialEq, Debug, Clone)]
pub(crate) enum HexTokenType {
    /// A fully specified byte, e.g. `4D`.
    Byte(u8),

    /// A fully wildcarded byte (`??`).
    Wildcard,

    /// A byte with one nibble wildcarded, e.g. `?D` or `D?`.
    NibbleWildcard {
        /// High nibble, or `None` if wildcarded.
        high: Option<u8>,
        /// Low nibble, or `None` if wildcarded.
        low: Option<u8>,
    },

    /// A jump bound number, e.g. the `4` in `[4-6]`.
    Number(u32),

    /// Dash separating jump bounds (`-`).
    Dash,

    /// Left bracket (`[`), opening a jump.
    LBracket,

    /// Right bracket (`]`), closing a jump.
    RBracket,

    /// Left parenthesis (`(`), opening an alternation.
    LParen,

    /// Right parenthesis (`)`), closing an alternation.
    RParen,

    /// Pipe (`|`) separating alternation branches.
    Pipe,

    /// Unknown token type. It is here so an error can be raised in the AST
    /// parser and not the lexer.
    Unknown(char),
}
