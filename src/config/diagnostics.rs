//! Non-fatal configuration problems, collected rather than thrown.
//!
//! Fixing configuration mistakes one run at a time is miserable, so every
//! problem found while merging and validating is pushed here and the whole set
//! is reported together.

use std::ops::Range;
use std::path::{Path, PathBuf};

/// How serious a diagnostic is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// The value was ignored or replaced with its default; linting continues.
    Warning,
    /// The configuration is wrong in a way the user almost certainly cares
    /// about; linting should not proceed.
    Error,
}

/// One problem found in a configuration file.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// How serious this problem is.
    pub level: Level,

    /// What is wrong, in one sentence.
    pub message: String,

    /// File the problem was found in, when it came from a file.
    pub path: Option<PathBuf>,

    /// Byte range of the offending text within that file.
    pub span: Option<Range<usize>>,

    /// One-line line and column description, pre-rendered while the source
    /// text was still available.
    pub location: Option<String>,

    /// Optional follow-up, typically a "did you mean" suggestion.
    pub help: Option<String>,
}

/// A collected set of configuration problems.
#[derive(Debug, Clone, Default)]
pub struct Diagnostics {
    /// Every problem found so far, in the order they were found.
    entries: Vec<Diagnostic>,
}

impl Diagnostics {
    /// Create an empty set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a problem.
    ///
    /// # Arguments
    ///
    /// * `diagnostic` - the problem to record
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.entries.push(diagnostic);
    }

    /// Every problem found, in the order they were found.
    #[must_use]
    pub fn entries(&self) -> &[Diagnostic] {
        &self.entries
    }

    /// True if no problems were found.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// True if at least one problem is an error rather than a warning.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.entries.iter().any(|d| d.level == Level::Error)
    }
}

/// Builder for a single diagnostic.
///
/// Exists so that call sites read as one expression rather than a struct
/// literal with four `None` fields.
#[derive(Debug)]
pub struct DiagnosticBuilder {
    /// The diagnostic under construction.
    diagnostic: Diagnostic,
}

impl DiagnosticBuilder {
    /// Start building a warning.
    ///
    /// # Arguments
    ///
    /// * `message` - what is wrong, in one sentence
    #[must_use]
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(Level::Warning, message)
    }

    /// Start building an error.
    ///
    /// # Arguments
    ///
    /// * `message` - what is wrong, in one sentence
    #[must_use]
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(Level::Error, message)
    }

    /// Start building a diagnostic at the given level.
    ///
    /// # Arguments
    ///
    /// * `level` - how serious the problem is
    /// * `message` - what is wrong, in one sentence
    #[must_use]
    fn new(level: Level, message: impl Into<String>) -> Self {
        Self {
            diagnostic: Diagnostic {
                level,
                message: message.into(),
                path: None,
                span: None,
                location: None,
                help: None,
            },
        }
    }

    /// Attach the source location, rendering line and column immediately.
    ///
    /// # Arguments
    ///
    /// * `path` - file the problem was found in
    /// * `text` - full text of that file, used to compute line and column
    /// * `span` - byte range of the offending text, if known
    #[must_use]
    pub fn at(mut self, path: &Path, text: &str, span: Option<Range<usize>>) -> Self {
        self.diagnostic.path = Some(path.to_path_buf());
        self.diagnostic.location = span.as_ref().map(|s| {
            let (line, col) = line_col(text, s.start);
            format!("{}:{line}:{col}", path.display())
        });
        self.diagnostic.span = span;
        self
    }

    /// Attach a follow-up suggestion.
    ///
    /// # Arguments
    ///
    /// * `help` - the suggestion text
    #[must_use]
    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.diagnostic.help = Some(help.into());
        self
    }

    /// Attach a "did you mean" suggestion, if a close enough candidate exists.
    ///
    /// # Arguments
    ///
    /// * `input` - what the user actually wrote
    /// * `candidates` - the valid options
    #[must_use]
    pub fn suggest<'a>(self, input: &str, candidates: impl IntoIterator<Item = &'a str>) -> Self {
        match did_you_mean(input, candidates) {
            Some(best) => self.help(format!("did you mean `{best}`?")),
            None => self,
        }
    }

    /// Finish and push into a diagnostic set.
    ///
    /// # Arguments
    ///
    /// * `diagnostics` - the set to push into
    pub fn emit(self, diagnostics: &mut Diagnostics) {
        diagnostics.push(self.diagnostic);
    }
}

/// Convert a byte offset into a one-based line and column.
///
/// The column counts characters rather than bytes, so that it lines up with
/// what an editor shows for non-ASCII text.
///
/// # Arguments
///
/// * `text` - the full source text
/// * `offset` - byte offset into `text`
#[must_use]
pub fn line_col(text: &str, offset: usize) -> (usize, usize) {
    let clamped = offset.min(text.len());
    let mut line = 1usize;
    let mut line_start = 0usize;

    for (index, byte) in text.as_bytes()[..clamped].iter().enumerate() {
        if *byte == b'\n' {
            line += 1;
            line_start = index + 1;
        }
    }

    let column = text[line_start..clamped].chars().count() + 1;
    (line, column)
}

/// Pick the closest candidate to `input`, if one is close enough to be worth
/// suggesting.
///
/// Uses Levenshtein distance with a threshold that scales with the length of
/// the input, so short keys require a near-exact match and long keys tolerate a
/// couple of typos.
///
/// # Arguments
///
/// * `input` - what the user actually wrote
/// * `candidates` - the valid options
#[must_use]
pub fn did_you_mean<'a>(
    input: &str,
    candidates: impl IntoIterator<Item = &'a str>,
) -> Option<&'a str> {
    let threshold = match input.chars().count() {
        0..=3 => 1,
        4..=9 => 2,
        _ => 3,
    };

    let mut best: Option<(usize, &'a str)> = None;
    for candidate in candidates {
        // An abbreviation such as `warn` for `warning` is a long edit distance
        // but an obvious intent, so prefix matches are treated as near misses.
        let distance = if is_prefix_of_either(input, candidate) {
            1
        } else {
            levenshtein(input, candidate)
        };
        if distance > threshold {
            continue;
        }
        match best {
            Some((best_distance, _)) if best_distance <= distance => {}
            _ => best = Some((distance, candidate)),
        }
    }

    best.map(|(_, candidate)| candidate)
}

/// The largest length difference for which a prefix still counts as a typo.
///
/// Without a cap, `Naming` would look like a near miss for
/// `Naming/RuleNameLenght` and beat the cop the user actually meant.
const MAX_PREFIX_GAP: usize = 4;

/// True if either string is a short prefix of the other.
///
/// "Short" means within [`MAX_PREFIX_GAP`] characters, so `warn` counts as an
/// abbreviation of `warning` while a bare category does not count as an
/// abbreviation of a fully qualified cop name.
///
/// # Arguments
///
/// * `a` - first string
/// * `b` - second string
#[must_use]
fn is_prefix_of_either(a: &str, b: &str) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }
    if !a.starts_with(b) && !b.starts_with(a) {
        return false;
    }
    a.chars().count().abs_diff(b.chars().count()) <= MAX_PREFIX_GAP
}

/// Levenshtein edit distance between two strings, counting characters.
///
/// # Arguments
///
/// * `a` - first string
/// * `b` - second string
#[must_use]
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }

    let mut previous: Vec<usize> = (0..=b.len()).collect();
    let mut current: Vec<usize> = vec![0; b.len() + 1];

    for (i, ca) in a.iter().enumerate() {
        current[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let cost = usize::from(ca != cb);
            current[j + 1] = (current[j] + 1)
                .min(previous[j + 1] + 1)
                .min(previous[j] + cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[b.len()]
}
