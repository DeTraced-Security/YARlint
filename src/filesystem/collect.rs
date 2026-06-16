//! File discovery and collection.
//!
//! This module is responsible for traversing filesystems and
//! collecting candidate YARA files for further processing.

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::filesystem::filters::is_yara_file;

/// Collects YARA files from a file or directory path.
///
/// If `path` refers to a single YARA file, that file is returned.
///
/// If `path` refers to a directory, files are collected from the
/// directory and optionally its subdirectories depending on the
/// value of `recursive`.
///
/// When recursive scanning is enabled, the traversal depth may be
/// limited by `depth`. If no depth is provided, traversal is
/// unbounded.
///
/// # Arguments
///
/// * `path` (`&str`) - file or directory path to scan
/// * `recursive` (`bool`) - recursively traverse subdirectories when scanning
///   a directory
/// * `depth` (`Option<usize>`) - maximum traversal depth when recursive scanning is
///   enabled; ignored if `recursive` is not `true`
///
/// # Errors
///
/// Returns an error if:
/// - the supplied path does not exist
/// - a filesystem traversal error occurs
///
/// Non-YARA files are ignored and do not produce an error
pub fn collect_yara_files(
    path: &str,
    recursive: bool,
    depth: Option<usize>,
) -> Result<Vec<PathBuf>, String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err("Path does not exist".to_string());
    }

    let mut yara_files = Vec::new();

    if path.is_file() {
        if is_yara_file(path) {
            yara_files.push(path.to_path_buf());
        }
        return Ok(yara_files);
    }

    let walker = if recursive {
        match depth {
            Some(d) => WalkDir::new(path).max_depth(d),
            None => WalkDir::new(path),
        }
    } else {
        WalkDir::new(path).max_depth(1)
    };

    for entry in walker {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_file() && is_yara_file(path) {
            yara_files.push(path.to_path_buf());
        }
    }

    Ok(yara_files)
}
