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
    ast_parser::AstParser,
    lexer::tokenize,
    syntax::{RuleNode, rule_file::RuleFileNode},
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
/// * `files` - A collection of paths to YARA files that should be parsed.
///
/// # Returns
///
/// Returns `Ok(())` if all files were successfully read and tokenized.
///
/// Returns `Err(String)` if:
///
/// * A file cannot be opened.
/// * A file cannot be read.
/// * Tokenization fails.
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
        for token in &tokens {
            println!("{:?}", token);
        }
        let parser: AstParser = AstParser::new(tokens);

        println!("About to parse rule");
        let rule_file = AstParser::parse_rule_file(parser)?;
        println!("Rule parsed successfully");
        rule_files.push(rule_file);
    }
    Ok(rule_files)
}
