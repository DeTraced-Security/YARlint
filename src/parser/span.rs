//! Token location tracking.
//!
//! This module contains functionality for tracking the location of a token
//! for reporting.

#[derive(PartialEq, Debug, Clone, Copy)]
/// Method of storing the location of a token
///
/// Stores the token's line and column
pub struct Span {
    /// Line number
    pub line: usize,

    /// Column number
    pub column: usize,
}
