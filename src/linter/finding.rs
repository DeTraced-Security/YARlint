//! Lint findings and severity levels.

/// Severity of a lint finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    /// Informational message.
    Info,

    /// Warning-level issue.
    Warning,

    /// Error-level issue.
    Error,
}

/// A single lint result.
#[derive(Debug, Clone)]
pub struct Finding {
    /// Name of the rule that generated this finding.
    pub rule: &'static str,

    /// Human-readable message.
    pub message: String,

    /// Finding severity.
    pub severity: Severity,
}
