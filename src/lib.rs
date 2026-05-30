//! YARlint is a YARA rule linter focused on identifying malformed,
//! unsafe, or non-standard rule files before deployment.
//!
//! The library is organized into the following components:
//!
//! - `cli` - command-line argument definitions and output helpers
//! - `filesystem` - file discovery and filtering
//! - `validation` - encoding, size, and byte-level validation
//! - `parser` - YARA rule parsing
//! - `app` - pipeline orchestration

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub mod cli;
pub mod filesystem;
pub mod parser;
pub mod validation;
pub mod app;