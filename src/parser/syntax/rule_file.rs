//! Root node representing a YARA File.

use crate::parser::syntax::RuleNode;

/// Root node representing a YARA file.
///
/// A rule file consists of imports, and rules.
#[derive(Debug)]
pub struct RuleFileNode {
    /// Imports the the file
    pub imports: Vec<String>,

    /// YARA rules in the rule file.
    pub rules: Vec<RuleNode>,
}
