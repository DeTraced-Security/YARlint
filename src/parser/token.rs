//! Token implementation.
//!
//! This module implements the token for the YARA lexer.

use crate::parser::span::Span;

#[derive(Debug)]

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
#[derive(Debug)]
pub enum TokenType {
    /// A standard identifier such as a rule name, metadata key,
    /// module name, or function name.
    Identifier(String),

    /// A YARA string identifier such as `$a` or `$filename`.
    StringIdentifier(String),

    /// A quoted string literal.
    StringLiteral(String),

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

    /// Assignment or equality operator (`=`).
    Equals,

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
