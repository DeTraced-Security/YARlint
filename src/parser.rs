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
/// Returns `Ok(())` if all files were successfully read and tokenized.
///
/// # Errors
///
/// Returns an error if:
/// - A file cannot be opened
/// - A file cannot be read
/// - Tokenization fails
pub fn parse_files(files: &Vec<std::path::PathBuf>) -> Result<Vec<RuleFileNode>, String> {
    let mut rule_files: Vec<RuleFileNode> = Vec::new();
    for file_path in files {
        println!("File name: {}", file_path.display());
        let file = File::open(file_path).map_err(|e| e.to_string())?;

        let mut reader = BufReader::new(file);

        let mut file_source = String::new();

        reader
            .read_to_string(&mut file_source)
            .map_err(|e| e.to_string())?;
        let tokens: Vec<Token> = tokenize(&file_source.to_string())?;
        if tokens.is_empty() {
            println!("Skipping {}: contains no YARA rule", file_path.display());
            continue;
        }
        // Debug
        //for token in &tokens {
        //    println!("{:?}", token);
        //}
        let parser: AstParser = AstParser::new(tokens);

        // Debug
        //println!("About to parse rule");
        let rule_file = AstParser::parse_rule_file(parser)?;
        // Debug
        //println!("Rule parsed successfully");
        rule_files.push(rule_file);
    }
    Ok(rule_files)
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
