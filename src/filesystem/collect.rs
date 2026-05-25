use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::filesystem::filters::is_yara_file;

pub fn collect_yara_files(path: &str, recursive: bool, depth: Option<usize>,) -> Result<Vec<PathBuf>, String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err("Path does not exist".to_string());
    }
    
    let mut yara_files = Vec::new();

    if path.is_file() {
        if is_yara_file(path) {
            yara_files.push(path.to_path_buf());
        }
        return Ok(yara_files)
    }

    let walker = if recursive {
        match depth {
            Some(d) => WalkDir::new(path).max_depth(d),
            None => WalkDir::new(path),
        }
    } else {
        WalkDir::new(path).max_depth(1)
    };

    for entry in walker {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_file() && is_yara_file(path) {
            yara_files.push(path.to_path_buf());
        }
    }

    Ok(yara_files)

}