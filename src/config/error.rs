//! Fatal configuration failures.
//!
//! These stop the world. Problems *inside* an otherwise readable configuration
//! file are collected as [`Diagnostics`](crate::config::diagnostics::Diagnostics)
//! instead, so that the user sees every mistake at once rather than one per run.

use std::fmt;
use std::ops::Range;
use std::path::PathBuf;

/// A failure that prevents configuration from being loaded at all.
#[derive(Debug)]
pub enum ConfigError {
    /// The configuration file could not be read.
    Io {
        /// Path that could not be read.
        path: PathBuf,
        /// Underlying I/O failure.
        source: std::io::Error,
    },

    /// The configuration file is not valid TOML.
    Parse {
        /// Path that failed to parse.
        path: PathBuf,
        /// Message reported by the TOML parser.
        message: String,
        /// Byte range of the offending text, when the parser supplied one.
        span: Option<Range<usize>>,
    },

    /// A cop rejected its own parameters during construction.
    ///
    /// Used for invariants that span two parameters and therefore cannot be
    /// expressed by a single [`ParamSpec`](crate::config::spec::ParamSpec),
    /// such as a minimum that exceeds its maximum.
    Invalid {
        /// Qualified name of the cop that rejected its parameters.
        cop: String,
        /// Explanation of what is wrong.
        message: String,
    },
}

impl ConfigError {
    /// Construct an [`ConfigError::Invalid`] for a cop that rejected its
    /// parameters.
    ///
    /// # Arguments
    ///
    /// * `cop` - qualified name of the cop
    /// * `message` - explanation of what is wrong
    pub fn invalid(cop: impl Into<String>, message: impl Into<String>) -> Self {
        ConfigError::Invalid {
            cop: cop.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io { path, source } => {
                write!(f, "could not read {}: {source}", path.display())
            }
            ConfigError::Parse { path, message, .. } => {
                write!(f, "could not parse {}: {message}", path.display())
            }
            ConfigError::Invalid { cop, message } => {
                write!(f, "invalid configuration for {cop}: {message}")
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io { source, .. } => Some(source),
            ConfigError::Parse { .. } | ConfigError::Invalid { .. } => None,
        }
    }
}
