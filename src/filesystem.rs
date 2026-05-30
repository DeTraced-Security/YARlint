//! Filesystem interaction utilities.
//!
//! This module is responsible for discovering candidate YARA files
//! and filtering unsupported filesystem entries before validation
//! and parsing.

pub mod collect;
pub mod filters;

pub use collect::collect_yara_files;
pub use filters::is_yara_file;