//! Provides configuration values and checks for entire project


use std::sync::OnceLock;

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