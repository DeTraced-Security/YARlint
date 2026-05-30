//! Encoding validation rules.
//!
//! This module contains validators responsible for ensuring that
//! candidate YARA files use supported text encodings.

/// Validates that a file contains only UTF-8 encoded text.
///
/// ASCII files are considered valid, as ASCII is a strict subset of UTF-8.
///
/// Files containing malformed UTF-8 sequences are rejected before
/// byte-level validation and parsing.
pub fn validate_encoding(file: &[u8]) -> Result<bool, String> {
    match std::str::from_utf8(file) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}