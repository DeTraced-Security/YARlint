//! Application pipeline orchestration.
//!
//! This module coordinates the major stages of a YARlint scan,
//! including file discovery, validation, and parsing.

use crate::cli::Args;
use crate::cli::output::{print_file_summary, print_valid_file_summary};

use crate::filesystem::collect_yara_files;
use crate::validation::validate_files;

/// Executes the YARlint processing pipeline.
///
/// The pipeline consists of the following stages:
///
/// 1. File collection
/// 2. File validation
/// 3. File parsing
///
/// Progress information is reported to the user between stages.
///
/// # Errors
///
/// Returns an error if any stage of the pipeline encounters a
/// fatal failure that prevents processing from continuing.
pub fn yarlint_pipeline(args: &Args) -> Result<(), String> {
    let files: Vec<std::path::PathBuf> =
        collect_yara_files(&args.path, args.recursive, args.depth)?;

    print_file_summary(files.len());

    let valid_files: Vec<std::path::PathBuf> = validate_files(&files)?;

    print_valid_file_summary(valid_files.len());

    //let yara_rules: Vec<crate::parser::yara_rule::YaraRule> = parse_files(&valid_files)?;

    Ok(())
}
