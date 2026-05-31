//! YARA rule lexing.
//!
//! This module converts YARA source code into a sequence of tokens
//! that can be consumed by the parser.

use crate::parser::yara_rule::KEYWORDS;

#[derive(Debug)]

/// A token produced by the lexer.
///
/// Stores the token's type and, in the future, may include
/// source location information such as line and column numbers.
pub struct Token {
    /// The token's classification and associated value.
    pub token_type: TokenType,
    //pub column: usize,
    //pub row: usize,
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
/// # Errors
///
/// Returns an error if the lexer encounters invalid syntax, such as:
/// - Unterminated block comments
/// - Unterminated string literals
/// - Other malformed token sequences
pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            // Skip whitespace
            c if c.is_whitespace() => {}

            // Single-character tokens
            '{' => tokens.push(Token {
                token_type: TokenType::LBrace,
            }),

            '}' => tokens.push(Token {
                token_type: TokenType::RBrace,
            }),

            '=' => tokens.push(Token {
                token_type: TokenType::Equals,
            }),

            ':' => tokens.push(Token {
                token_type: TokenType::Colon,
            }),

            '(' => tokens.push(Token {
                token_type: TokenType::LParen,
            }),

            ')' => tokens.push(Token {
                token_type: TokenType::RParen,
            }),

            '*' => tokens.push(Token {
                token_type: TokenType::Star,
            }),

            '.' => tokens.push(Token {
                token_type: TokenType::Dot,
            }),

            '@' => tokens.push(Token {
                token_type: TokenType::AtSymbol,
            }),

            '-' => tokens.push(Token {
                token_type: TokenType::Minus,
            }),

            '+' => tokens.push(Token {
                token_type: TokenType::Plus,
            }),

            '>' => match chars.peek() {
                Some('=') => {
                    chars.next();

                    tokens.push(Token {
                        token_type: TokenType::GEThan,
                    });
                }

                _ => {
                    tokens.push(Token {
                        token_type: TokenType::GThan,
                    });
                }
            },

            '<' => match chars.peek() {
                Some('=') => {
                    chars.next();

                    tokens.push(Token {
                        token_type: TokenType::LEThan,
                    });
                }

                _ => {
                    tokens.push(Token {
                        token_type: TokenType::LThan,
                    });
                }
            },

            '/' => match chars.peek() {
                Some('/') => {
                    chars.next();

                    for ch in chars.by_ref() {
                        if ch == '\n' {
                            break;
                        }
                    }
                }

                Some('*') => {
                    chars.next();

                    loop {
                        match chars.next() {
                            Some('*') => {
                                if let Some('/') = chars.peek() {
                                    chars.next();
                                    break;
                                }
                            }
                            Some(_) => {}
                            None => {
                                return Err("Unterminated block comment".to_string());
                            }
                        }
                    }
                }

                _ => {
                    tokens.push(Token {
                        token_type: TokenType::FSlash,
                    });
                }
            },

            // String literal
            '"' => {
                let mut value = String::new();
                let mut escaped = false;

                for next in chars.by_ref() {
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
                            break;
                        }

                        _ => {
                            value.push(next);
                        }
                    }
                }

                tokens.push(Token {
                    token_type: TokenType::StringLiteral(value),
                });
            }
            // YARA string identifier ($a, $foo, etc.)
            '$' => {
                let mut ident = String::from("$");

                while let Some(next) = chars.peek() {
                    if next.is_alphanumeric() || *next == '_' {
                        ident.push(*next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token {
                    token_type: TokenType::StringIdentifier(ident),
                });
            }

            // Keywords / identifiers
            c if c.is_alphabetic() || c == '_' => {
                let mut word = String::from(c);

                while let Some(next) = chars.peek() {
                    if next.is_alphanumeric() || *next == '_' {
                        word.push(*next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if KEYWORDS.contains(&word.as_str()) {
                    tokens.push(Token {
                        token_type: TokenType::Keyword(word),
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Identifier(word),
                    });
                }
            }

            c if c.is_ascii_digit() => {
                let mut num = String::from(c);

                while let Some(next) = chars.peek() {
                    if next.is_ascii_alphanumeric() {
                        num.push(*next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token {
                    token_type: TokenType::Number(num),
                });
            }

            _ => {
                tokens.push(Token {
                    token_type: TokenType::Unknown(ch),
                });
            }
        }
    }

    //for token in &tokens {
    //   println!("{:?}", token.token_type);
    //}

    Ok(tokens)
}
