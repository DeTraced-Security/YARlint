//! YARA rule parsing.
//!
//! This module contains functionality for processing validated YARA
//! files and extracting the information required for linting.

//pub mod ast;
pub mod lexer;
pub mod yara_rule;

use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::parser::{
    //ast::ast_parser,
    lexer::{Token, tokenize},
    //yara_rule::YaraRule,
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
pub fn parse_files(files: &Vec<std::path::PathBuf>) -> Result<(), String> {
    for file_path in files {
        let file = File::open(file_path).map_err(|e| e.to_string())?;

        let mut reader = BufReader::new(file);

        let mut file_source = String::new();

        reader
            .read_to_string(&mut file_source)
            .map_err(|e| e.to_string())?;
        let _tokens: Vec<Token> = tokenize(&file_source.to_string())?;
        //let ast_file: YaraRule = ast_parser(tokens)?;
        //println!("{}", ast_file.display());
    }

    Ok(())
}
