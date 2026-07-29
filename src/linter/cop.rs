//! Lint cop trait.

use crate::linter::{context::LintContext, finding::Finding};

/// The family a cop belongs to. Mirrors the folder structure under
/// `src/linter/cops/` (e.g. `Category::Performance` -> `cops/performance/`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Category {
    /// Lint Category
    Lint,
    /// Logic Category
    Logic,
    /// Naming Category
    Naming,
    /// Performance Category
    Performance,
    /// Style Category
    Style,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Category::Lint => "Lint",
            Category::Logic => "Logic",
            Category::Naming => "Naming",
            Category::Performance => "Performance",
            Category::Style => "Style",
        })
    }
}

impl std::str::FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lint" => Ok(Category::Lint),
            "Logic" => Ok(Category::Logic),
            "Naming" => Ok(Category::Naming),
            "Performance" => Ok(Category::Performance),
            "Style" => Ok(Category::Style),
            _ => Err(()),
        }
    }
}

/// Trait implemented by all lint cops.
pub trait Cop {
    /// The cop's bare name, without its category prefix.
    ///
    /// Examples:
    /// - "CopName" -> found in naming/cop_name.rs, category() == Category::Naming
    /// - "DuplicateString" -> found in lint/duplicate_string.rs, category() == Category::Lint
    fn name(&self) -> &'static str;

    /// The cop's category. Should match the folder the cop's implementation
    /// file lives in under `src/linter/cops/`.
    fn category(&self) -> Category;

    /// The fully qualified id, e.g. "Performance/WideStringWithoutAscii".
    /// Not meant to be overridden.
    fn qualified_name(&self) -> String {
        format!("{}/{}", self.category(), self.name())
    }

    /// Execute the cop
    ///
    /// Should be implemented such that the function checks for the cop's
    /// condition in the `context` and pushes a new Finding describing any
    /// non-compliance to `findings`
    ///
    /// # Arguments
    ///
    /// * `context` (`&LintContext`) - A `LintContext` containing the
    ///   `RuleNode`s to be checked
    /// * `findings` (`&mut Vec<Finding>`) - Vector to push Finding to
    fn check(&self, context: &LintContext, findings: &mut Vec<Finding>);
}
