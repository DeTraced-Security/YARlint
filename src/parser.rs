//! YARA rule parsing.
//!
//! This module contains functionality for processing validated YARA
//! files and extracting the information required for linting.

pub mod ast_parser;
pub mod lexer;
pub mod span;
pub mod syntax;
pub mod token;

use crate::parser::{
    ast_parser::{AstParser, hex::HexAstParser},
    lexer::{hex::tokenize_hex, yara::tokenize},
    syntax::{
        hex::{HexExprNode, HexNode},
        rule_file::RuleFileNode,
    },
    token::Token,
};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

/// Parses and validates one or more YARA files.
///
/// Each file is opened and read into memory before being passed to the
/// tokenizer. Any I/O or tokenization error will immediately stop processing
/// and be returned to the caller.
///
/// # Arguments
///
/// * `files` (`&Vec<std::path::PathBuf>`) - A collection of paths to YARA
///   files that should be parsed.
///
/// # Returns
///
/// Returns the parsed rule files.
///
/// # Errors
///
/// Returns an error if:
/// - A file cannot be opened
/// - A file cannot be read
/// - Tokenization fails
/// - Parsing fails
pub fn parse_files(files: &Vec<std::path::PathBuf>) -> Result<Vec<RuleFileNode>, String> {
    let mut rule_files = Vec::with_capacity(files.len());

    for file_path in files {
        if let Some((rule_file, _)) = parse_file(file_path)? {
            rule_files.push(rule_file);
        }
    }

    Ok(rule_files)
}

/// Parses YARA files and preserves their lexer output.
///
/// The returned tokens are the same allocation consumed by the AST parser,
/// allowing token-aware cops to run without reading or lexing the file again.
///
/// # Arguments
///
/// * `files` (`&Vec<std::path::PathBuf>`) - Paths to YARA files to parse
///
/// # Returns
///
/// Returns each parsed rule file together with its lexer tokens.
///
/// # Errors
///
/// Returns an error if:
/// - A file cannot be opened
/// - A file cannot be read
/// - Tokenization fails
/// - Parsing fails
pub(crate) fn parse_files_with_tokens(
    files: &Vec<std::path::PathBuf>,
) -> Result<Vec<(RuleFileNode, Vec<Token>)>, String> {
    let mut rule_files = Vec::with_capacity(files.len());

    for file_path in files {
        if let Some(parsed_file) = parse_file(file_path)? {
            rule_files.push(parsed_file);
        }
    }

    Ok(rule_files)
}

/// Parses one YARA file and preserves its lexer tokens.
///
/// Empty token streams are skipped without constructing a syntax tree.
///
/// # Arguments
///
/// * `file_path` (`&Path`) - Path to the YARA file to parse
///
/// # Returns
///
/// Returns the parsed rule file and tokens, or `None` when the file contains
/// no YARA tokens.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be opened
/// - The file cannot be read
/// - Tokenization fails
/// - Parsing fails
fn parse_file(file_path: &Path) -> Result<Option<(RuleFileNode, Vec<Token>)>, String> {
    println!("File name: {}", file_path.display());
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);
    let mut file_source = String::new();

    reader
        .read_to_string(&mut file_source)
        .map_err(|e| e.to_string())?;
    let tokens = tokenize(&file_source)?;

    if tokens.is_empty() {
        println!("Skipping {}: contains no YARA rule", file_path.display());
        return Ok(None);
    }

    let parser = AstParser::new(tokens);
    let parsed_file = AstParser::parse_rule_file_with_tokens(parser)?;
    Ok(Some(parsed_file))
}

/// Parses the contents of a YARA hex string into a [`HexNode`].
///
/// # Arguments
///
/// * `hex_string` (`&str`) - the raw text between a hex string's braces,
///   e.g. `4D 5A ?? [4-6] ( AA | BB )`.
///
/// # Returns
///
/// Returns a [`HexNode`] containing both the parsed structure and the
/// original raw text (kept for future use, e.g. autofix diffing).
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
