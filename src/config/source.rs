//! Where configuration text comes from.
//!
//! The loader is a trait so that everything downstream of it can be tested
//! against string literals. [`MapLoader`] backs the tests; [`FsLoader`] backs
//! the real program.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::config::error::ConfigError;

/// One configuration file: where it came from, and what was in it.
///
/// The text is kept alongside the path because diagnostics need it to turn byte
/// offsets into line and column numbers.
#[derive(Debug, Clone)]
pub struct ConfigSource {
    /// Path the text was read from.
    pub path: PathBuf,

    /// Full contents of the file.
    pub text: String,
}

/// Finds and reads configuration files.
pub trait SourceLoader {
    /// Locate the configuration file to use, if there is one.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if a candidate location could not be
    /// inspected.
    fn discover(&self) -> Result<Option<PathBuf>, ConfigError>;

    /// Read one configuration file.
    ///
    /// # Arguments
    ///
    /// * `path` - the file to read
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if the file could not be read.
    fn load(&self, path: &Path) -> Result<ConfigSource, ConfigError>;
}

/// Reads configuration from the real filesystem.
///
/// Discovery order is the current working directory first, then
/// `~/.config/detraced/yarlint`.
///
/// This is the one piece of the configuration module that touches I/O. It is
/// a candidate to move into `filesystem/config_file.rs` alongside the discovery
/// logic it delegates to.
#[derive(Debug, Clone, Copy, Default)]
pub struct FsLoader;

impl SourceLoader for FsLoader {
    fn discover(&self) -> Result<Option<PathBuf>, ConfigError> {
        crate::filesystem::config_file::check_for_config_files().map_err(|source| ConfigError::Io {
            path: PathBuf::from("."),
            source,
        })
    }

    fn load(&self, path: &Path) -> Result<ConfigSource, ConfigError> {
        let text = std::fs::read_to_string(path).map_err(|source| ConfigError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        Ok(ConfigSource {
            path: path.to_path_buf(),
            text,
        })
    }
}

/// Reads configuration from an in-memory map.
///
/// Lets the whole pipeline be exercised from string literals, with no temporary
/// directories anywhere.
#[derive(Debug, Clone, Default)]
pub struct MapLoader {
    /// Every file this loader knows about, keyed by path.
    files: BTreeMap<PathBuf, String>,

    /// The path [`SourceLoader::discover`] should return.
    entry: Option<PathBuf>,
}

impl MapLoader {
    /// Create a loader with no files.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a loader holding a single file, which discovery will return.
    ///
    /// # Arguments
    ///
    /// * `path` - the path to pretend the text lives at
    /// * `text` - the file contents
    #[must_use]
    pub fn single(path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        let path = path.into();
        let mut loader = Self::new();
        loader.files.insert(path.clone(), text.into());
        loader.entry = Some(path);
        loader
    }

    /// Add a file without making it the discovery entry point.
    ///
    /// # Arguments
    ///
    /// * `path` - the path to pretend the text lives at
    /// * `text` - the file contents
    #[must_use]
    pub fn with(mut self, path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        self.files.insert(path.into(), text.into());
        self
    }
}

impl SourceLoader for MapLoader {
    fn discover(&self) -> Result<Option<PathBuf>, ConfigError> {
        Ok(self.entry.clone())
    }

    fn load(&self, path: &Path) -> Result<ConfigSource, ConfigError> {
        match self.files.get(path) {
            Some(text) => Ok(ConfigSource {
                path: path.to_path_buf(),
                text: text.clone(),
            }),
            None => Err(ConfigError::Io {
                path: path.to_path_buf(),
                source: std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "no such file in MapLoader",
                ),
            }),
        }
    }
}
