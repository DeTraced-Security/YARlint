//! Byte-level safety validation.
//!
//! This module detects potentially unsafe or malformed content,
//! including control characters, null bytes, and Unicode bidi
//! control characters.

/// Validates byte-level safety constraints.
///
/// The validator rejects files containing:
///
/// - Null bytes (`0x00`)
/// - ASCII control characters other than `\n`, `\r`, and `\t`
/// - Unicode bidirectional control characters commonly associated
///   with source-code spoofing and visual obfuscation attacks
///
/// This validation is intended to prevent malformed or potentially
/// deceptive content from reaching the parsing stage.
/// 
/// # Arguments
/// 
/// * `file` (`&[u8]`) - file path of the file to be validated
/// 
/// # Returns
/// 
/// Returns `Ok(true)` if the file passes the validation process, or
/// `Ok(false)` if the file did not pass
pub fn validate_bytes(file: &[u8]) -> Result<bool, String> {
    // 1. null byte + control byte scan
    for &byte in file {
        // null byte
        if byte == 0x00 {
            return Ok(false);
        }

        // control chars except \n \r \t
        if (byte < 0x20) && !matches!(byte, b'\n' | b'\r' | b'\t') {
            return Ok(false);
        }
    }

    // 2. bidi character scan (requires UTF-8)
    let Ok(text) = std::str::from_utf8(file) else {
        // If it's not UTF-8, encoding validator should handle it
        return Ok(false);
    };

    for ch in text.chars() {
        let code = ch as u32;

        // U+202A–U+202E
        if (0x202A..=0x202E).contains(&code) {
            return Ok(false);
        }

        // U+2066–U+2069
        if (0x2066..=0x2069).contains(&code) {
            return Ok(false);
        }
    }

    Ok(true)
}
