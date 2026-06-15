//! Lint execution context.

use crate::parser::syntax::rule_file::RuleFileNode;

/// Context passed to lint rules.
pub struct LintContext<'a> {
    /// Parsed YARA file.
    pub file: &'a RuleFileNode,
}
