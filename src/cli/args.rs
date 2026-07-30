//! Command-line argument definitions.
//!
//! This module defines the user-facing command-line interface for
//! YARlint and is responsible for parsing and validating runtime
//! configuration supplied by the user.

use clap::Parser;

use crate::linter::cops::selector::CopSelector;

/// Command-line arguments accepted by YARlint.
///
/// These arguments control file discovery and traversal behavior
/// before validation and parsing begin.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
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

    /// Enables verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Restrict linting to only the given categories or cops.
    ///
    /// Accepts a category name (e.g. `Naming`) or a fully qualified
    /// cop name (e.g. `Naming/RuleName`). May be passed multiple
    /// times, as a comma-separated list, or both.
    #[arg(long, value_delimiter = ',')]
    pub only: Vec<CopSelector>,

    /// Exclude the given categories or cops from linting.
    ///
    /// Same format as `--only`. Naming the same specific cop in both
    /// `--only` and `--except` is an error, not a silent tie-break.
    #[arg(long, value_delimiter = ',')]
    pub except: Vec<CopSelector>,
}
