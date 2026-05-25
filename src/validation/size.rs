use std::path::PathBuf;

const GIGABYTE_SIZE: u64 = 1073741824;

pub fn validate_size(file: &PathBuf) -> Result<bool, String> {
    let file_metadata = std::fs::metadata(file)
        .map_err(|e| e.to_string())?;
    if file_metadata.len() > GIGABYTE_SIZE {
        return Ok(false);
    }
    Ok(true)
}