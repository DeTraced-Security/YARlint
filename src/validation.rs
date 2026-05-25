use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use crate::validation::{bytes::validate_bytes, encoding::validate_encoding, size::validate_size};

pub mod encoding;
pub mod bytes;
pub mod size;

pub fn validate_files(files: &Vec<PathBuf>) -> Result<Vec<PathBuf>, String> {
    let mut valid_files: Vec<PathBuf> = Vec::new();

    for file_path in files {

        let file = File::open(file_path)
            .map_err(|e| e.to_string())?;

        let mut reader = BufReader::new(file);

        let mut file_bytes: Vec<u8> = Vec::new();

        reader
            .read_to_end(&mut file_bytes)
            .map_err(|e| e.to_string())?;

        if validate_size(file_path)?
            && validate_encoding(&file_bytes)?
            && validate_bytes(&file_bytes)?
        {
            valid_files.push(file_path.clone());
        }

    }

    Ok(valid_files)
}