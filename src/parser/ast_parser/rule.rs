//! Parsing of complete YARA rules.
//!
//! This module contains the entry point for AST construction. It
//! coordinates parsing of the `meta`, `strings`, and `condition`
//! sections and produces a complete [`RuleNode`] representation.
