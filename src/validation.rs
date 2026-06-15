//! Validation pipeline for YARA files.
//!
//! This module is responsible for filtering malformed, unsafe, or
//! unsupported files before they reach the parsing stage.
//!
//! Validation currently includes:
//! - File size checks
//! - UTF-8 encoding validation
//! - Byte-level safety checks
//!
//! Files that fail validation are excluded from further processing.

use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use crate::validation::{bytes::validate_bytes, encoding::validate_encoding, size::validate_size};

pub mod bytes;
pub mod encoding;
pub mod size;

/// Validates a collection of candidate YARA files.
///
/// Each file is loaded into memory and passed through the configured
/// validation checks. Files that pass all validation stages are returned
/// for parsing.
///
/// Validation currently performs:
/// - File size validation
/// - UTF-8 encoding validation
/// - Byte-level safety validation
/// 
/// # Arguments
/// 
/// * `files` (`&Vec<PathBuf>`) - a vector containing files to be validated
///
/// # Returns
/// 
/// Returns a vector containing only files that passed all validation
/// checks.
///
/// # Errors
///
/// Returns an error if:
/// - a file cannot be opened or read
pub fn validate_files(files: &Vec<PathBuf>) -> Result<Vec<PathBuf>, String> {
    let mut valid_files: Vec<PathBuf> = Vec::new();

    for file_path in files {
        let file = File::open(file_path).map_err(|e| e.to_string())?;

        let mut reader = BufReader::new(file);

        let mut file_bytes: Vec<u8> = Vec::new();

        reader
            .read_to_end(&mut file_bytes)
            .map_err(|e| e.to_string())?;

        if validate_size(file_path)?
            && validate_encoding(&file_bytes)?
            && validate_bytes(&file_bytes)?
        {
            valid_files.push(file_path.clone());
        }
    }

    Ok(valid_files)
}
