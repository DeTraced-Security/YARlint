//! File size validation.
//!
//! This module enforces size limits to prevent excessive resource
//! consumption during validation and parsing.

use std::path::PathBuf;

const GIGABYTE_SIZE: u64 = 1073741824;

/// Validates that a file is within the supported size limit.
///
/// Files larger than one gigabyte are rejected to prevent excessive
/// memory consumption during validation and parsing.
///
/// Returns `Ok(false)` when the file exceeds the maximum allowed size.
pub fn validate_size(file: &PathBuf) -> Result<bool, String> {
    let file_metadata = std::fs::metadata(file)
        .map_err(|e| e.to_string())?;
    if file_metadata.len() > GIGABYTE_SIZE {
        return Ok(false);
    }
    Ok(true)
}