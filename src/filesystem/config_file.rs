//! Locating YARlint's configuration file.
//!
//! Discovery checks the current working directory first, then
//! `~/.config/detraced/yarlint`. The first file found wins; there is no
//! merging across locations and no walking up parent directories.

use std::{
    env::{current_dir, home_dir},
    io,
    path::{Path, PathBuf},
};

/// These are the file names that can be YARlint configuration files. They must be this format.
const CONFIG_FILE_NAMES: &[&str] = &["yarlint.toml", ".yarlint.toml"];

/// Looks for a config file in `dir`, preferring names in CONFIG_FILE_NAMES order.
fn find_in_dir(dir: &Path) -> io::Result<Option<PathBuf>> {
    for name in CONFIG_FILE_NAMES {
        let candidate = dir.join(name);
        if candidate.try_exists()? {
            return Ok(Some(candidate));
        }
    }
    Ok(None)
}

/// Checks for configuration file location, and returns the first one found
pub fn check_for_config_files() -> io::Result<Option<PathBuf>> {
    // A local config in the current working directory wins.
    let cwd = current_dir()?;
    if let Some(path) = find_in_dir(&cwd)? {
        return Ok(Some(path));
    }

    // Otherwise fall back to ~/.config/detraced/yarlint.
    if let Some(home) = home_dir() {
        let user_config = home.join(".config").join("detraced").join("yarlint");
        if let Some(path) = find_in_dir(&user_config)? {
            return Ok(Some(path));
        }
    }

    Ok(None)
}
