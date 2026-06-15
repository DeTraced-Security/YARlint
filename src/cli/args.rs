//! Command-line argument definitions.
//!
//! This module defines the user-facing command-line interface for
//! YARlint and is responsible for parsing and validating runtime
//! configuration supplied by the user.

use clap::Parser;

/// Command-line arguments accepted by YARlint.
///
/// These arguments control file discovery and traversal behavior
/// before validation and parsing begin.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File or directory path to scan.
    #[arg(short, long)]
    pub path: String,

    /// Recursively traverse subdirectories when scanning a directory.
    #[arg(short, long)]
    pub recursive: bool,

    /// Maximum traversal depth when recursive scanning is enabled.
    ///
    /// Ignored unless `--recursive` is specified.
    #[arg(short, long)]
    pub depth: Option<usize>,
}
