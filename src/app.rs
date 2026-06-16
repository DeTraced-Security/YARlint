//! Application pipeline orchestration.
//!
//! This module coordinates the major stages of a YARlint scan,
//! including file discovery, validation, and parsing.

use crate::cli::Args;
use crate::cli::output::{print_file_summary, print_valid_file_summary, print_yara_rule_files};

use crate::config::verbose;
use crate::filesystem::collect_yara_files;
use crate::linter;
use crate::linter::context::LintContext;
use crate::parser::parse_files;
use crate::parser::syntax::rule_file::RuleFileNode;
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
/// # Arguments
/// * `args` (`&Args`) - The parsed arguments provided by the user
///
/// # Errors
///
/// Returns an error if:
/// - Any stage of the pipeline encounters a fatal failure that
///   prevents processing from continuing.
pub fn yarlint_pipeline(args: &Args) -> Result<(), String> {
    let files: Vec<std::path::PathBuf> =
        collect_yara_files(&args.path, args.recursive, args.depth)?;

    print_file_summary(files.len());

    let valid_files: Vec<std::path::PathBuf> = validate_files(&files)?;

    print_valid_file_summary(valid_files.len());

    let yara_rule_files: Vec<RuleFileNode> = parse_files(&valid_files)?;

    for rule_file in &yara_rule_files {
        let context = LintContext { file: rule_file };

        let engine = linter::default_engine();

        let findings = engine.run(&context);

        for finding in findings {
            println!(
                "[{:?}] {}: {}",
                finding.severity, finding.rule, finding.message,
            );
        }
    }
    if verbose() {
        print_yara_rule_files(&yara_rule_files);
    }
    Ok(())
}
