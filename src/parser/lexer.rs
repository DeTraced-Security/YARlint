//! YARA rule lexing.
//!
//! This module converts YARA source code into a sequence of tokens
//! that can be consumed by the parser.

use std::{iter::Peekable, str::Chars};

use crate::parser::{
    span::Span,
    token::{Token, TokenType},
    yara_rule::KEYWORDS,
};

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    fn current_span(&self) -> Span {
        Span {
            line: self.line,
            column: self.column,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

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

            '=' => tokens.push(Token {
                token_type: TokenType::Equals,
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

    //for token in &tokens {
    //    println!("{:?}, {}:{}", token.token_type, token.span.line, token.span.column)
    //}

    Ok(tokens)
}
