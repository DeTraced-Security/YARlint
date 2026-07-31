//! Provides configuration values and checks for entire project

use crate::linter::cops::style::rule_name_case::NameCase;
use std::sync::OnceLock;

/// Verbose setting. Sets it once if true, otherwise set to be false.
static VERBOSE: OnceLock<bool> = OnceLock::new();

/// Sets the verbose via arguments passed form command-line
pub fn init_verbose(v: bool) {
    VERBOSE.set(v).ok();
    if verbose() {
        println!("Verbose is set")
    }
}

/// Returns true if verbose is set, and false if not.
pub fn verbose() -> bool {
    *VERBOSE.get().unwrap_or(&false)
}

/// Returns the configured rule name casing convention.
///
/// Hardcoded until the config file format exists
pub fn rule_name_case() -> NameCase {
    NameCase::PascalCase
}
