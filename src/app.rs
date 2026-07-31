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
use crate::linter::cop::Cop;
use crate::linter::cops::selector::{self, ConflictingSelectorError};
use crate::parser::parse_files_with_tokens;
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

    let parsed_rule_files = parse_files_with_tokens(&valid_files)?;
    let mut yara_rule_files: Vec<RuleFileNode> = Vec::with_capacity(parsed_rule_files.len());

    let selector_engine = linter::default_engine();
    let all_cops: Vec<&dyn Cop> = selector_engine.cops().iter().map(AsRef::as_ref).collect();
    selector::resolve_enabled_cops(&all_cops, &args.only, &args.except).unwrap_or_else(
        |ConflictingSelectorError(name)| {
            eprintln!("'{name}' was passed to both --only and --except");
            std::process::exit(1);
        },
    );

    for (rule_file, tokens) in parsed_rule_files {
        let engine = linter::default_engine_with_tokens(tokens);
        let all_cops: Vec<&dyn Cop> = engine.cops().iter().map(AsRef::as_ref).collect();
        let enabled = selector::resolve_enabled_cops(&all_cops, &args.only, &args.except)
            .unwrap_or_else(|ConflictingSelectorError(name)| {
                eprintln!("'{name}' was passed to both --only and --except");
                std::process::exit(1);
            });

        let context = LintContext { file: &rule_file };
        let findings = engine.run_selected(&context, &enabled);

        for finding in findings {
            println!(
                "[{:?}] {}/{}: {}",
                finding.severity, finding.category, finding.rule, finding.message,
            );
        }

        yara_rule_files.push(rule_file);
    }
    if verbose() {
        print_yara_rule_files(&yara_rule_files);
    }
    Ok(())
}
