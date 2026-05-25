pub fn validate_encoding(file: &[u8]) -> Result<bool, String> {
    match std::str::from_utf8(file) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}