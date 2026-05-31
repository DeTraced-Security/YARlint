//! Root syntax node representing a YARA rule.

use crate::parser::syntax::{condition::ConditionNode, meta::MetaEntryNode, strings::StringNode};

/// Root syntax node representing a parsed YARA rule.
///
/// A rule consists of a name, optional modifiers, optional tags,
/// metadata entries, string definitions, and a condition block.
#[derive(Debug, Clone)]
pub struct RuleNode {
    /// Name of the rule.
    ///
    /// Example:
    /// `MAL_SALATSTEALER_RAT_AUG25`
    pub name: String,

    /// Indicates whether the rule is declared as `global`.
    pub is_global: bool,

    /// Indicates whether the rule is declared as `private`.
    pub is_private: bool,

    /// Tags associated with the rule.
    ///
    /// Example:
    /// ```yara
    /// rule ExampleRule : malware windows {
    /// ...
    /// }
    /// ```
    ///
    /// would produce:
    ///
    /// ```text
    /// ["malware", "windows"]
    /// ```
    pub tags: Vec<String>,

    /// Metadata entries contained within the rule's `meta` section.
    pub meta: Vec<MetaEntryNode>,

    /// String definitions contained within the rule's `strings` section.
    pub strings: Vec<StringNode>,

    /// Condition block for the rule.
    pub condition: ConditionNode,
}
