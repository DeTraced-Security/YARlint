//! Output helpers for CLI presentation.
//!
//! This module centralizes user-facing console output to keep
//! presentation concerns separate from collection, validation,
//! and parsing logic.

use std::path::PathBuf;
use colored::Colorize;

/// Displays the path currently being scanned.
pub fn print_scan_start(path: &str) {
    println!("Scanning {}", path);
}

/// Displays a warning indicating that recursive scanning is enabled.
///
/// Recursive scans may traverse large directory trees and increase
/// memory usage and runtime.
pub fn print_recursive_warning() {
    println!("{}", format!("Warning: Recursive scanning can consume large amounts of system memory, and may take a long time to complete. Use with caution").yellow());
}

/// Displays the total number of YARA files discovered during collection.
pub fn print_file_summary(count: usize) {
    println!("{}", format!("Found {} YARA files", count).green());
}

/// Displays the total number of files that passed validation.
pub fn print_valid_file_summary(count: usize) {
    println!("{}", format!("Found {} Valid YARA files", count).green());
}

/// Prints a file path to standard output.
pub fn print_file(path: &PathBuf) {
    println!("{}", path.display());

}

/// Prints an error message to standard error using colored output.
pub fn print_error(err: &str) {
    eprintln!("{}", format!("Error: {}", err).red())

}