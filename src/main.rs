//! YARlint command-line interface.
//!
//! This binary is responsible for argument parsing, user-facing output,
//! and execution of the linting pipeline.

use clap::Parser;

use yarlint::app::yarlint_pipeline;
use yarlint::cli::Args;
use yarlint::cli::output::{print_error, print_recursive_warning, print_scan_start};
use yarlint::config::init_verbose;

/// Parses CLI arguments and executes the YARlint pipeline.
///
/// # Errors
/// 
/// Returns an error if:
/// - errors are thrown by the linting pipeline
/// 
/// Fatal errors are reported to stderr before the process exits
/// with a non-zero status code.
fn main() {
    let args = Args::parse();

    init_verbose(args.verbose);

    print_scan_start(&args.path);

    if args.recursive {
        print_recursive_warning();
    }

    if let Err(err) = yarlint_pipeline(&args) {
        print_error(&err.to_string());
        std::process::exit(1);
    }
}
