//! File filtering helpers.
//!
//! This module contains predicates used during file collection to
//! identify candidate YARA files and exclude unsupported file types.

use std::path::Path;

/// Determines whether a path has a supported YARA file extension.
///
/// Currently, files ending in `.yar` or `.yara` are considered valid
/// YARA rule files.
pub fn is_yara_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("yar") | Some("yara")
    )
}
