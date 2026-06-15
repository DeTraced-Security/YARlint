//! YARA rule lexing.
//!
//! This module converts YARA source code into a sequence of tokens
//! that can be consumed by the parser.

use std::{iter::Peekable, str::Chars};

use crate::parser::{
    span::Span,
    token::{Token, TokenType},
};

/// Reserved keywords recognized by the YARA language.
///
/// These keywords are used by the lexer to distinguish language
/// constructs from ordinary identifiers.
pub const KEYWORDS: &[&str] = &[
    "all",
    "and",
    "any",
    "ascii",
    "at",
    "base64",
    "base64wide",
    "condition",
    "contains",
    "endswith",
    "false",
    "for",
    "fullword",
    "global",
    "import",
    "icontains",
    "iendswith",
    "iequals",
    "in",
    "include",
    "istartswith",
    "matches",
    "meta",
    "nocase",
    "none",
    "not",
    "of",
    "or",
    "private",
    "rule",
    "startswith",
    "strings",
    "them",
    "true",
    "wide",
    "xor",
    "defined",
];

/// List of identifier-like tokens used by the lexer.
///
/// This is used to differentiate between keywords and identifiers
/// during tokenization and parsing.
pub const IDENTIFIERS: &[&str] = &[
    "entrypoint",
    "filesize",
    "uint8",
    "uint16",
    "uint32",
    "uint8be",
    "uint16be",
    "uint32be",
    "int8",
    "int16",
    "int32",
    "int8be",
    "int16be",
    "int32be",
];

/// Stateful lexer used to tokenize YARA source code.
///
/// The lexer maintains the current input position and provides utility
/// methods for consuming characters while tracking source locations.
///
/// Source locations are tracked using line and column numbers, allowing
/// later parsing stages to produce accurate diagnostics and error messages.
struct Lexer<'a> {
    /// Character iterator over the source text.
    chars: Peekable<Chars<'a>>,

    /// Current line number within the source file.
    ///
    /// Line numbers are one-based.
    line: usize,

    /// Current column number within the source file.
    ///
    /// Column numbers are one-based after the first character on a line
    /// has been consumed.
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer for the provided source text.
    ///
    /// The lexer is initialized at the beginning of the input with the
    /// current position set to line 1, column 0.
    /// 
    /// # Arguments
    /// 
    /// * `source` (`&'a str`) - the source to be lexed
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    /// Returns the current source position.
    ///
    /// This position can be attached to tokens and syntax nodes to aid
    /// in error reporting and diagnostics.
    /// 
    /// # Returns
    /// 
    /// Returns the current source position of the lexer
    fn current_span(&self) -> Span {
        Span {
            line: self.line,
            column: self.column,
        }
    }

    /// Returns the next character without consuming it.
    ///
    /// # Returns
    /// 
    /// Returns the next character of the source or `None` if the end
    /// of the input has been reached
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Consumes and returns the next character from the input stream.
    ///
    /// The lexer's line and column counters are updated automatically.
    /// Newline characters increment the line counter and reset the column
    /// counter to zero.
    ///
    /// # Returns
    /// 
    /// Returns the next character of the source or `None` if the end
    /// of the input has been reached.
    fn next(&mut self) -> Option<char> {
        let ch = self.chars.next()?;

        if ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        Some(ch)
    }
}

/// Converts YARA source text into a sequence of lexical tokens.
///
/// The lexer scans the provided source code and produces a vector of
/// [`Token`] values that can be consumed by the parser.
///
/// Supported token types include:
/// - Identifiers
/// - YARA string identifiers (`$foo`)
/// - String literals
/// - Numeric literals
/// - Keywords
/// - Operators and punctuation
///
/// Comments are ignored and are not included in the output token stream.
///
/// # Arguments
/// 
/// * `source` (`&str`) - the source text to be tokenised
/// 
/// # Errors
///
/// Returns an error if the lexer encounters invalid syntax, such as:
/// - Unterminated block comments
/// - Unterminated string literals
/// - Other malformed token sequences
pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    while let Some(ch) = lexer.next() {
        let span = lexer.current_span();

        match ch {
            c if c.is_whitespace() => {}

            '{' => tokens.push(Token {
                token_type: TokenType::LBrace,
                span,
            }),

            '}' => tokens.push(Token {
                token_type: TokenType::RBrace,
                span,
            }),

            ':' => tokens.push(Token {
                token_type: TokenType::Colon,
                span,
            }),

            '(' => tokens.push(Token {
                token_type: TokenType::LParen,
                span,
            }),

            ')' => tokens.push(Token {
                token_type: TokenType::RParen,
                span,
            }),

            '*' => tokens.push(Token {
                token_type: TokenType::Star,
                span,
            }),

            '.' => tokens.push(Token {
                token_type: TokenType::Dot,
                span,
            }),

            '@' => tokens.push(Token {
                token_type: TokenType::AtSymbol,
                span,
            }),

            '-' => tokens.push(Token {
                token_type: TokenType::Minus,
                span,
            }),

            '+' => tokens.push(Token {
                token_type: TokenType::Plus,
                span,
            }),

            '=' => {
                if matches!(lexer.peek(), Some('=')) {
                    lexer.next();

                    tokens.push(Token {
                        token_type: TokenType::EqualsEquals,
                        span,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Equals,
                        span,
                    })
                }
            }

            '>' => {
                if matches!(lexer.peek(), Some('=')) {
                    lexer.next();

                    tokens.push(Token {
                        token_type: TokenType::GEThan,
                        span,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::GThan,
                        span,
                    });
                }
            }

            '<' => {
                if matches!(lexer.peek(), Some('=')) {
                    lexer.next();

                    tokens.push(Token {
                        token_type: TokenType::LEThan,
                        span,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::LThan,
                        span,
                    });
                }
            }

            '/' => {
                let start_span = span;

                match lexer.peek() {
                    // line comment: //
                    Some('/') => {
                        lexer.next(); // consume second '/'

                        while let Some(ch) = lexer.next() {
                            if ch == '\n' {
                                break;
                            }
                        }
                    }

                    // block comment: /* ... */
                    Some('*') => {
                        lexer.next(); // consume '*'

                        loop {
                            match lexer.next() {
                                Some('*') => {
                                    if matches!(lexer.peek(), Some('/')) {
                                        lexer.next(); // consume '/'
                                        break;
                                    }
                                }

                                Some(_) => {}

                                None => {
                                    return Err(format!(
                                        "Unterminated block comment at {}:{}",
                                        start_span.line, start_span.column
                                    ));
                                }
                            }
                        }
                    }

                    // division operator
                    _ => {
                        tokens.push(Token {
                            token_type: TokenType::FSlash,
                            span,
                        });
                    }
                }
            }

            '"' => {
                let mut value = String::new();
                let mut escaped = false;
                let mut terminated = false;

                while let Some(next) = lexer.next() {
                    if escaped {
                        value.push(next);
                        escaped = false;
                        continue;
                    }

                    match next {
                        '\\' => {
                            escaped = true;
                            value.push('\\');
                        }

                        '"' => {
                            terminated = true;
                            break;
                        }

                        _ => value.push(next),
                    }
                }

                if !terminated {
                    return Err(format!(
                        "Unterminated string literal at {}:{}",
                        span.line, span.column
                    ));
                }

                tokens.push(Token {
                    token_type: TokenType::StringLiteral(value),
                    span,
                });
            }

            '$' => {
                let mut ident = String::from("$");

                while let Some(next) = lexer.peek() {
                    if next.is_alphanumeric() || *next == '_' {
                        ident.push(*next);
                        lexer.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token {
                    token_type: TokenType::StringIdentifier(ident),
                    span,
                });
            }

            c if c.is_alphabetic() || c == '_' => {
                let mut word = String::from(c);

                while let Some(next) = lexer.peek() {
                    if next.is_alphanumeric() || *next == '_' {
                        word.push(*next);
                        lexer.next();
                    } else {
                        break;
                    }
                }

                let token_type = if KEYWORDS.contains(&word.as_str()) {
                    TokenType::Keyword(word)
                } else {
                    TokenType::Identifier(word)
                };

                tokens.push(Token { token_type, span });
            }

            c if c.is_ascii_digit() => {
                let mut num = String::from(c);

                while let Some(next) = lexer.peek() {
                    if next.is_ascii_alphanumeric() {
                        num.push(*next);
                        lexer.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token {
                    token_type: TokenType::Number(num),
                    span,
                });
            }

            _ => {
                tokens.push(Token {
                    token_type: TokenType::Unknown(ch),
                    span,
                });
            }
        }
    }

    // Debug
    //for token in &tokens {
    //    println!(
    //        "{:?}, {}:{}",
    //        token.token_type, token.span.line, token.span.column
    //    )
    //}

    Ok(tokens)
}
